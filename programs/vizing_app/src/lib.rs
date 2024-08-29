pub mod vizing_omni;
use anchor_lang::prelude::*;
use vizing_omni::*;

declare_id!("C17xMdoPdgPSYd7oGEjYf5LQ1mg6k6P3eavCBdMfaF1X");

#[program]
pub mod vizing_app {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.sol_pda_receiver.bump = *ctx.bumps.get("sol_pda_receiver").unwrap();
        Ok(())
    }

    #[access_control(assert_vizing_authority(&ctx.accounts.vizing_authority))]
    pub fn receive_from_vizing(ctx: Context<LandingAppOp>, _params: VizingMessage) -> Result<()> {
        msg!(
            "@@authority from vizing: {}",
            ctx.accounts.vizing_authority.key()
        );

        msg!(
            "authority is signer: {}",
            ctx.accounts.vizing_authority.is_signer
        );

        msg!("Hello world from vizing");

        msg!("remaining_accounts: {:?}", ctx.remaining_accounts);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct LandingAppOp<'info> {
    /// CHECK: 1. Vizing Authority account
    #[account(signer)]
    pub vizing_authority: AccountInfo<'info>,

    #[account(seeds = [VIZING_APP_SOL_RECEIVER_SEED], bump = sol_pda_receiver.bump)]
    pub sol_pda_receiver: Account<'info, VizingSolReceiver>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + VizingSolReceiver::INIT_SPACE, seeds = [VIZING_APP_SOL_RECEIVER_SEED], bump)]
    pub sol_pda_receiver: Account<'info, VizingSolReceiver>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
