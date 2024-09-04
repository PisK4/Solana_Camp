use crate::vizing_omni::*;
use crate::governance::*;
use crate::library::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct LaunchOp<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(mut, signer)]
    pub vizing_app_fee_payer: AccountInfo<'info>,
    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub vizing_app_message_authority: AccountInfo<'info>,

    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.is_paused != true @VizingError::VizingNotActivated)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    /// CHECK: We need this account as to receive the fee
    #[account(mut, address = vizing_pad_config.fee_collector @VizingError::FeeCollectorInvalid)]
    pub vizing_pad_fee_collector: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}


impl LaunchOp<'_> {
    pub fn vizing_launch(ctx: &mut Context<LaunchOp>, params: LaunchParams) -> Result<()> {
        msg!("####Launch in vizing core");

        msg!("vizing_app_fee_payer is signer: {}", ctx.accounts.vizing_app_fee_payer.is_signer);

        msg!("vizing_app_message_authority is signer: {}", ctx.accounts.vizing_app_message_authority.is_signer);
        
        msg!(
            "vizing_app_message_authority: {}",
            ctx.accounts.vizing_app_message_authority.key()
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

        msg!("addition_params mode: {}", params.addition_params.mode);

        msg!("addition_params signature: {:?}", params.addition_params.signature);

        msg!("mode: {}", params.message.mode);

        // msg!("target_program: {}", params.message.target_program);

        msg!("execute_gas_limit: {}", params.message.execute_gas_limit);

        msg!("max_fee_per_gas: {}", params.message.max_fee_per_gas);

        msg!("signature: {:?}", params.message.signature);

        msg!("fee payer: {}", ctx.accounts.vizing_app_fee_payer.key());

        msg!("fee collector: {}", ctx.accounts.vizing_pad_fee_collector.key());

        msg!("vizing_app_fee_payer is writeable: {}", ctx.accounts.vizing_app_fee_payer.is_writable);

        msg!("vizing_pad_fee_collector is writeable: {}", ctx.accounts.vizing_pad_fee_collector.is_writable);


        // mock fee
        let fee: u64 = 1000000000;
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vizing_app_fee_payer.to_account_info(),
                    to: ctx.accounts.vizing_pad_fee_collector.to_account_info(),
                },
            ),
            fee,
        )?;

        emit!(SuccessfulLaunchMessage {
            erliest_arrival_timestamp: params.erliest_arrival_timestamp,
            latest_arrival_timestamp: params.latest_arrival_timestamp,
            relayer: params.relayer,
            sender: params.sender,
            src_contract: ctx.accounts.vizing_app_message_authority.key(),
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
    pub vizing: Account<'info, VizingPadConfigs>,

    /// CHECK: We need this PDA as a signer
    #[account(seeds = [VIZING_AUTHORITY_SEED],bump = vizing_authority.bump)]
    pub vizing_authority: Account<'info, VizingAuthorityParams>,

    /// CHECK: target contract
    #[account(mut, address = params.message.target_program @VizingError::TargetContractInvalid)]
    pub target_program: AccountInfo<'info>,

    #[account(
        seeds = [VIZING_APP_CONFIG_SEED, &vizing_app_configs.vizing_app_program_id.to_bytes()],
        bump = vizing_app_configs.bump,
        constraint = vizing_app_configs.vizing_app_program_id == target_program.key() @VizingError::TargetContractInvalid
    )]
    pub vizing_app_configs: Option<Account<'info, VizingAppConfig>>,

    pub system_program: Program<'info, System>,
}

impl LandingOp<'_> {
    #[access_control(landing_check(&ctx))]
    pub fn vizing_landing<'info>(ctx: &mut Context<'_, '_, '_, 'info, LandingOp<'info>>, params: LandingParams) -> Result<()> {
        let balance_before = ctx.accounts.relayer.lamports();
        let mut traget = ctx.accounts.target_program.to_account_info();
        if traget.executable {
            let account_info = ctx
            .remaining_accounts
            .iter()
            .map(|acc| {
                let mut account = acc.to_account_metas(None)[0].clone();
                account.is_signer = account.pubkey == ctx.accounts.vizing_authority.key();
                account
            })
            .collect::<Vec<_>>();

            if params.value > 0 {
                traget = ctx.remaining_accounts[2].to_account_info();
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

        }else{
            if params.value > 0 {
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
        }

        require!(
            ctx.accounts.relayer.lamports() <= balance_before + params.value,
            VizingError::InsufficientBalance
        );

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
    if let Some(config) = ctx.accounts.vizing_app_configs.clone() {
        let vizing_apps = config.vizing_app_accounts.clone();
        let remaining_accounts = ctx.remaining_accounts.iter().map(|a| a.key).collect::<Vec<_>>();
        for app in vizing_apps {
            if !remaining_accounts.contains(&&app) {
                return Err(VizingError::VizingAppNotInRemainingAccounts.into());
            }
        }
    }
    Ok(())
}

fn build_landing_ix_data(params: &LandingParams) -> Result<Vec<u8>> {
    let vizing_message = VizingMessage {
        src_chainid: params.src_chainid,
        src_contract: params.src_contract,
        value: params.value,
        signature: params.message.signature.clone(),
    };
    let mut data = Vec::with_capacity(VizingMessage::INIT_SPACE);
    data.extend(RECEIVE_FROM_VIZING_DISCRIMINATOR);
    vizing_message.serialize(&mut data)?;
    Ok(data)
}


#[account]
#[derive(InitSpace)]
pub struct LandingParams {
    pub message_id: [u8; 32],
    pub erliest_arrival_timestamp: u64,
    pub latest_arrival_timestamp: u64,
    pub src_chainid: u64,
    pub src_tx_hash: [u8; 32],
    pub src_contract: [u8; 32],
    pub src_chain_nonce: u32,
    pub sender: [u8; 32],
    pub value: u64,
    pub addition_params: AdditionalParams,
    pub message: LandingMessage,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct LandingMessage {
    pub mode: u8,
    pub target_program: Pubkey,
    pub execute_gas_limit: u64,
    pub max_fee_per_gas: u64,
    #[max_len(1024)]
    pub signature: Vec<u8>,
}