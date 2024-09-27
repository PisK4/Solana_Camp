use crate::governance::*;
use crate::library::*;
use crate::vizing_omni::*;
use crate::VizingGasSystem;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(params: LaunchParams)]
pub struct LaunchOp<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(mut, signer)]
    pub vizing_app_fee_payer: AccountInfo<'info>,
    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub vizing_app_message_authority: AccountInfo<'info>,

    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.is_paused != true @VizingError::VizingNotActivated)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    #[account(
        init_if_needed,
        payer = vizing_app_fee_payer,
        space = 8 + SenderNonce::INIT_SPACE,
        seeds = [VIZING_SENDER_NONCE_SEED, params.sender.key().as_ref(), &params.dest_chainid.to_be_bytes()], bump
    )]
    pub sender_nonce: Account<'info, SenderNonce>,

    /// CHECK: We need this account as to receive the fee
    #[account(mut, address = vizing_pad_config.fee_collector @VizingError::FeeCollectorInvalid)]
    pub vizing_pad_fee_collector: AccountInfo<'info>,

    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    pub system_program: Program<'info, System>,
}
