pub mod errors;
use anchor_lang::prelude::*;
use errors::VizingError;

// use std::mem::size_of;

declare_id!("VzgnUvjAQD9oXjHLsVGrS67GCMVdzKpwHGj3QanJhk3");

pub const VIZING_PAD_SETTINGS_SEED: &[u8] = b"Vizing_Pad_Settings_Seed";

#[program]
pub mod vizing_core {

    use super::*;

    pub fn initialize_vizing_pad(
        mut ctx: Context<InitVizingPad>,
        params: InitVizingPadParams,
    ) -> Result<()> {
        InitVizingPad::apply(&mut ctx, params)
    }

    pub fn modify_settings(
        mut ctx: Context<ModifySettings>,
        params: ModifySettingsParams,
    ) -> Result<()> {
        ModifySettings::apply(&mut ctx, &params)
    }
}

#[derive(Accounts)]
pub struct InitVizingPad<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + VizingPadSettings::INIT_SPACE,
        seeds = [VIZING_PAD_SETTINGS_SEED],
        bump
    )]
    pub vizing: Account<'info, VizingPadSettings>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl InitVizingPad<'_> {
    pub fn apply(ctx: &mut Context<InitVizingPad>, params: InitVizingPadParams) -> Result<()> {
        // init vizing settings
        let (_, bump) = Pubkey::find_program_address(&[VIZING_PAD_SETTINGS_SEED], &ctx.program_id);
        ctx.accounts.vizing.bump = bump;

        ctx.accounts.vizing.owner = params.owner;
        ctx.accounts.vizing.engine_admin = params.engine_admin;
        ctx.accounts.vizing.station_admin = params.station_admin;
        ctx.accounts.vizing.gas_pool_admin = params.gas_pool_admin;
        ctx.accounts.vizing.trusted_relayer = params.trusted_relayer;
        ctx.accounts.vizing.registered_validator = params.registered_validator;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct InitVizingPadParams {
    pub owner: Pubkey,
    pub engine_admin: Pubkey,
    pub station_admin: Pubkey,
    pub gas_pool_admin: Pubkey,
    pub trusted_relayer: Pubkey,
    pub registered_validator: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct VizingPadSettings {
    // immutable
    pub bump: u8,
    // configurable
    pub owner: Pubkey,
    // #[max_len(64)]
    pub engine_admin: Pubkey,
    // #[max_len(64)]
    pub station_admin: Pubkey,
    // #[max_len(64)]
    pub gas_pool_admin: Pubkey,
    // #[max_len(128)]
    pub trusted_relayer: Pubkey,
    // #[max_len(64)]
    pub registered_validator: Pubkey,
}

#[derive(Accounts)]
pub struct ModifySettings<'info> {
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner @ VizingError::NotOwner, seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump)]
    pub vizing: Account<'info, VizingPadSettings>,
}

impl ModifySettings<'_> {
    pub fn apply(ctx: &mut Context<ModifySettings>, params: &ModifySettingsParams) -> Result<()> {
        ctx.accounts.vizing.owner = params.owner;
        ctx.accounts.vizing.engine_admin = params.engine_admin;
        ctx.accounts.vizing.station_admin = params.station_admin;
        ctx.accounts.vizing.gas_pool_admin = params.gas_pool_admin;
        ctx.accounts.vizing.trusted_relayer = params.trusted_relayer;
        ctx.accounts.vizing.registered_validator = params.registered_validator;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct ModifySettingsParams {
    pub owner: Pubkey,
    pub engine_admin: Pubkey,
    pub station_admin: Pubkey,
    pub gas_pool_admin: Pubkey,
    pub trusted_relayer: Pubkey,
    pub registered_validator: Pubkey,
}
