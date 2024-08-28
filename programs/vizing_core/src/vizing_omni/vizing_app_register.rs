use anchor_lang::prelude::*;

pub const VIZING_APP_CONFIG_SEED: &[u8] = b"Vizing_App_Config_Seed";

#[derive(Accounts)]
#[instruction(params: VizingAppRegisterParams)]
pub struct VizingAppRegister<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + VizingAppConfig::INIT_SPACE,
        seeds = [VIZING_APP_CONFIG_SEED, params.vizing_app_program_id.as_ref()],
        bump
    )]
    pub vizing_app_configs: Account<'info, VizingAppConfig>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct VizingAppConfig {
    #[max_len(160)]
    pub vizing_app_accounts: Vec<Pubkey>,
    pub vizing_app_program_id: Pubkey,
    pub admin: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VizingAppRegisterParams {
    #[max_len(160)]
    vizing_app_accounts: Vec<Pubkey>,
    vizing_app_program_id: Pubkey,
}

impl VizingAppRegister<'_> {
    pub fn register_vizing_app(
        ctx: &mut Context<VizingAppRegister>,
        params: VizingAppRegisterParams,
    ) -> Result<()> {
        ctx.accounts.vizing_app_configs.vizing_app_accounts = params.vizing_app_accounts;
        ctx.accounts.vizing_app_configs.admin = ctx.accounts.admin.key();
        let (_,app_bump) = Pubkey::find_program_address(&[b"vizing_app_configs".as_ref()], ctx.program_id);
        // ctx.accounts.vizing_app_configs.bump = *ctx.bumps.get("vizing_app_configs").unwrap();
        ctx.accounts.vizing_app_configs.bump=app_bump;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(params: VizingAppRegisterParams)]
pub struct VizingAppManagement<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,

    #[account(
        seeds = [VIZING_APP_CONFIG_SEED, vizing_app_configs.vizing_app_program_id.as_ref()],
        bump = vizing_app_configs.bump,
        has_one = admin
    )]
    pub vizing_app_configs: Account<'info, VizingAppConfig>,

    pub system_program: Program<'info, System>,
}

impl VizingAppManagement<'_> {
    pub fn update_vizing_app_accounts(
        ctx: &mut Context<VizingAppManagement>,
        vizing_app_accounts: Vec<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.vizing_app_configs.vizing_app_accounts = vizing_app_accounts;
        Ok(())
    }

    pub fn transfer_ownership(
        ctx: &mut Context<VizingAppManagement>,
        new_admin: Pubkey,
    ) -> Result<()> {
        ctx.accounts.vizing_app_configs.admin = new_admin.key();
        Ok(())
    }
}
