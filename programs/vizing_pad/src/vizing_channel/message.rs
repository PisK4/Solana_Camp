use crate::governance::*;
use crate::library::*;
use crate::VizingGasSystem;
use anchor_lang::prelude::*;

#[derive(Accounts)]
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

    /// CHECK: We need this account as to receive the fee
    #[account(mut, address = vizing_pad_config.fee_collector @VizingError::FeeCollectorInvalid)]
    pub vizing_pad_fee_collector: AccountInfo<'info>,

    #[account(
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    pub system_program: Program<'info, System>,
}
