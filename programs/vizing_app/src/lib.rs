use anchor_lang::prelude::*;

declare_id!("C17xMdoPdgPSYd7oGEjYf5LQ1mg6k6P3eavCBdMfaF1X");

#[program]
pub mod vizing_app {

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn receive_from_vizing(ctx: Context<LandingAppOp>) -> Result<()> {
        msg!(
            "authority from vizing: {}",
            ctx.accounts.vizing_authority.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LandingAppOp<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub vizing_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program<'info, System>,
}
