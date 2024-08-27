use crate::vizing_omni::*;
use crate::governance::*;
use crate::library::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct LaunchOp<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub fee_payer: AccountInfo<'info>,

    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub message_authority: AccountInfo<'info>,

    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.is_paused != true @VizingError::VizingNotActivated)]
    pub vizing: Account<'info, VizingPadSettings>,

    /// CHECK: We need this account as to receive the fee
    #[account(mut, address = vizing.fee_receiver @VizingError::FeeCollectorInvalid)]
    pub fee_collector: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl LaunchOp<'_> {
    pub fn vizing_launch(ctx: &mut Context<LaunchOp>, params: LaunchParams) -> Result<()> {
        msg!(
            "message_authority: {}",
            ctx.accounts.message_authority.key()
        );

        msg!(
            "erliest_arrival_timestamp: {}",
            params.erliest_arrival_timestamp
        );
        msg!(
            "latest_arrival_timestamp: {}",
            params.latest_arrival_timestamp
        );
        msg!("value: {}", params.value);

        msg!("dest_chainid: {}", params.dest_chainid);

        msg!("addition_params: {:?}", params.addition_params);

        msg!("mode: {}", params.message.mode);

        msg!("target_program: {}", params.message.target_program);

        msg!("execute_gas_limit: {}", params.message.execute_gas_limit);

        msg!("max_fee_per_gas: {}", params.message.max_fee_per_gas);

        msg!("signature: {:?}", params.message.signature);

        // mock fee
        let fee: u64 = 1000000000;
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.fee_payer.to_account_info(),
                    to: ctx.accounts.fee_collector.to_account_info(),
                },
            ),
            fee,
        )?;

        emit!(SuccessfulLaunchMessage {
            erliest_arrival_timestamp: params.erliest_arrival_timestamp,
            latest_arrival_timestamp: params.latest_arrival_timestamp,
            relayer: params.relayer,
            sender: params.sender,
            src_contract: ctx.accounts.message_authority.key(),
            value: params.value,
            fee: fee,
            dest_chainid: params.dest_chainid,
            addition_params: params.addition_params,
            message: params.message,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(params: LandingParams)]
pub struct LandingOp<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(mut, signer)]
    pub relayer: AccountInfo<'info>,

    #[account(
        seeds = [contants::VIZING_PAD_SETTINGS_SEED], 
        bump = vizing.bump,
        constraint = vizing.trusted_relayers.contains(&relayer.key()) @VizingError::NotRelayer, 
        constraint = vizing.is_paused != true @VizingError::VizingNotActivated
    )]
    pub vizing: Account<'info, VizingPadSettings>,

    /// CHECK: We need this PDA as a signer
    #[account(seeds = [contants::VIZING_AUTHORITY_SEED],bump = vizing_authority.bump)]
    pub vizing_authority: Account<'info, VizingAuthorityParams>,

    /// CHECK: target contract
    #[account(mut, address = params.message.target_program @VizingError::TargetContractInvalid)]
    pub target_program: AccountInfo<'info>,

    #[account(
        seeds = [VIZING_APP_CONFIG_SEED, &vizing_app_configs.vizing_app_program_id.to_bytes()],
        bump = vizing_app_configs.bump,
        constraint = vizing_app_configs.vizing_app_program_id == target_program.key() @VizingError::TargetContractInvalid
    )]
    pub vizing_app_configs: Account<'info, VizingAppConfig>,

    pub system_program: Program<'info, System>,
}

impl LandingOp<'_> {
    #[access_control(landing_check(&ctx))]
    pub fn vizing_landing<'info>(ctx: &mut Context<'_, '_, '_, 'info, LandingOp<'info>>, params: LandingParams) -> Result<()> {
        let balance_before = ctx.accounts.relayer.lamports();

        let account_info = ctx
            .remaining_accounts
            .iter()
            .map(|acc| {
                let mut account = acc.to_account_metas(None)[0].clone();
                account.is_signer = account.pubkey == ctx.accounts.vizing_authority.key();
                account
            })
            .collect::<Vec<_>>();

        let mut traget = ctx.accounts.target_program.to_account_info();
        
        if traget.executable {
            traget = ctx.remaining_accounts[1].to_account_info();

            transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.relayer.to_account_info(),
                        to: traget
                    },
                ),
                params.value,
            )?;

            let ix = Instruction {
                program_id: ctx.accounts.target_program.key(),
                accounts: account_info,
                data: build_landing_ix_data(&params)?,
            };
    
            invoke_signed(
                &ix,
                &[ctx.remaining_accounts].concat(),
                &[&[VIZING_AUTHORITY_SEED, &[ctx.accounts.vizing_authority.bump]]],
            )
            .map_err(|_| VizingError::CallingFailed)?;
    
            require!(
                ctx.accounts.relayer.lamports() <= balance_before + params.value,
                VizingError::InsufficientBalance
            );
    
        }else{
            transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.relayer.to_account_info(),
                        to: traget
                    },
                ),
                params.value,
            )?;
        }

        emit!(SuccessfulLanding {
            message_id: params.message_id,
            erliest_arrival_timestamp: params.erliest_arrival_timestamp,
            latest_arrival_timestamp: params.latest_arrival_timestamp,
            src_chainid: params.src_chainid,
            src_tx_hash: params.src_tx_hash,
            src_contract: params.src_contract,
            src_chain_nonce: params.src_chain_nonce,
            sender: params.sender,
            value: params.value,
            addition_params: params.addition_params,
            message: params.message,
        });

        Ok(())
    }
}

fn landing_check(ctx: &Context<LandingOp>) -> Result<()> {
    let vizing_apps = ctx.accounts.vizing_app_configs.vizing_app_accounts.clone();
    let remaining_accounts = ctx.remaining_accounts.iter().map(|a| a.key).collect::<Vec<_>>();
    for app in vizing_apps {
        if !remaining_accounts.contains(&&app) {
            return Err(VizingError::VizingAppNotInRemainingAccounts.into());
        }
    }
    Ok(())
}

fn build_landing_ix_data(params: &LandingParams) -> Result<Vec<u8>> {
    let mut data = Vec::with_capacity(LandingParams::INIT_SPACE);
    data.extend(RECEIVE_FROM_VIZING_DISCRIMINATOR);
    params.serialize(&mut data)?;
    Ok(data)
}
