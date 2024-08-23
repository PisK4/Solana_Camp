pub mod vizing_omni;
use anchor_lang::prelude::*;
use vizing_omni::*;

declare_id!("C17xMdoPdgPSYd7oGEjYf5LQ1mg6k6P3eavCBdMfaF1X");

#[program]
pub mod vizing_app {

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn receive_from_vizing(ctx: Context<LandingAppOp>, _params: LandingParams) -> Result<()> {
        msg!(
            "@@authority from vizing: {}",
            ctx.accounts.vizing_authority.key()
        );

        msg!(
            "authority is signer: {}",
            ctx.accounts.vizing_authority.is_signer
        );

        msg!("Hello world from vizing");

        // msg!("message: {:?}", params.message);
        msg!("remaining_accounts: {:?}", ctx.remaining_accounts);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct LandingAppOp<'info> {
    /// CHECK: 1. Vizing Authority account
    #[account(signer)]
    pub vizing_authority: AccountInfo<'info>,
    /// CHECK: 2. Vizing Sponsor account who came with sol
    pub vizing_sponsor: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program<'info, System>,
}
