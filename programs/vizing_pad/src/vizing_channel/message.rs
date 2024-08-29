use crate::governance::*;
use crate::library::*;
use crate::vizing_omni::*;
use anchor_lang::prelude::*;
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
