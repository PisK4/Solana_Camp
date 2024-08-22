use anchor_lang::prelude::*;

declare_id!("2xiuj4ozxygvkmC1WKJTGZyJXSD8dtbFxWkuJiMLzrTg");

#[program]
pub mod vizing_app_mock {

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn receive_from_vizing(ctx: Context<LandingAppOp>) -> Result<()> {
        msg!(
            "authority from vizing: {}",
            ctx.accounts.vizing_authority.key()
        );

        let a = 10;
        let b = 20;
        let c = a + b;

        msg!("{} + {} = {}", a, b, c);

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
