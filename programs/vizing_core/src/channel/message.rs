use super::*;
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
    #[account(mut, constraint = fee_collector.key() == vizing.fee_receiver @VizingError::FeeCollectorInvalid)]
    pub fee_collector: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl LaunchOp<'_> {
    pub fn execute(ctx: &mut Context<LaunchOp>, params: LaunchParams) -> Result<()> {
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

        msg!("target_contract: {}", params.message.target_contract);

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
    #[account(signer)]
    pub relayer: AccountInfo<'info>,

    #[account(mut, seeds = [contants::VIZING_PAD_SETTINGS_SEED], bump = vizing.bump,
    constraint = vizing.trusted_relayers.contains(&relayer.key()) @VizingError::NotRelayer)]
    pub vizing: Account<'info, VizingPadSettings>,

    /// CHECK: We need this PDA as a signer
    #[account(
            seeds = [contants::VIZING_AUTHORITY_SEED],
            bump = vizing_authority.bump
        )]
    pub vizing_authority: Account<'info, message::VizingAuthorityParams>,

    /// CHECK: target contract
    #[account(mut, constraint = target_contract.key() == params.message.target_contract @VizingError::TargetContractInvalid)]
    pub target_contract: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl LandingOp<'_> {
    pub fn execute(ctx: &mut Context<LandingOp>, params: LandingParams) -> Result<()> {
        let seeds = &[VIZING_AUTHORITY_SEED, &[ctx.accounts.vizing_authority.bump]];
        let signer = &[&seeds[..]];
        let program_id = ctx.accounts.target_contract.key();
        let accounts = ctx
            .remaining_accounts
            .iter()
            .map(|acc| acc.to_account_metas(None)[0].clone())
            .collect::<Vec<_>>();
        let data = build_landing_ix_data(&params).unwrap();
        let result = invoke_signed(
            &Instruction {
                program_id,
                accounts,
                data,
            },
            ctx.remaining_accounts,
            signer,
        );

        if result.is_ok() {
            return Ok(());
        } else {
            return err!(VizingError::CallingFailed);
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct LandingParams {
    pub message_id: [u8; 32],
    pub erliest_arrival_timestamp: u64,
    pub latest_arrival_timestamp: u64,
    pub src_chainid: u64,
    pub src_tx_hash: [u8; 32],
    pub src_contract: Pubkey,
    pub src_chain_nonce: u32,
    pub sender: Pubkey,
    pub value: u64,
    #[max_len(256)]
    pub addition_params: Vec<u8>,
    pub message: LandingMessage,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct LandingMessage {
    pub mode: u8,
    pub target_contract: Pubkey,
    pub execute_gas_limit: u64,
    pub max_fee_per_gas: u64,
    #[max_len(256)]
    pub signature: Vec<u8>,
}

fn build_landing_ix_data(params: &LandingParams) -> Result<Vec<u8>> {
    let mut data = Vec::with_capacity(LandingParams::INIT_SPACE);
    data.extend(RECEIVE_FROM_VIZING_DISCRIMINATOR);
    params.serialize(&mut data)?;
    Ok(data)
}
