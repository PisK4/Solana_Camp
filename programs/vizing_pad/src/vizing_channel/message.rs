use crate::governance::*;
use crate::library::*;
use crate::MappingFeeConfig;
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
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,

    pub system_program: Program<'info, System>,
}
