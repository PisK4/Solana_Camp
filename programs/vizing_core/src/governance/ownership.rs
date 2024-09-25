use anchor_lang::prelude::*;
use crate::library::*;

#[derive(Accounts)]
pub struct InitVizingPad<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + VizingPadConfigs::INIT_SPACE,
        seeds = [VIZING_PAD_CONFIG_SEED],
        bump,
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    /// CHECK: We need this PDA as a signer
    #[account(
        init,
        payer = payer,
        space = 8 + VizingAuthorityParams::INIT_SPACE,
        seeds = [VIZING_AUTHORITY_SEED, vizing_pad_config.key().as_ref()],
        bump,
    )]
    pub vizing_pad_authority: Account<'info, VizingAuthorityParams>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl InitVizingPad<'_> {
    pub fn initialize_vizing_pad(
        ctx: &mut Context<InitVizingPad>,
        params: InitVizingPadParams,
    ) -> Result<()> {
        // init vizing settings
        let (_, vizing_config_bump) =
            Pubkey::find_program_address(&[VIZING_PAD_CONFIG_SEED], &ctx.program_id);

        ctx.accounts.vizing_pad_config.bump = vizing_config_bump;
        ctx.accounts.vizing_pad_config.owner = params.owner;
        ctx.accounts.vizing_pad_config.fee_collector = params.fee_collector;
        ctx.accounts.vizing_pad_config.engine_admin = params.engine_admin;
        ctx.accounts.vizing_pad_config.station_admin = params.station_admin;
        ctx.accounts.vizing_pad_config.gas_pool_admin = params.gas_pool_admin;
        ctx.accounts.vizing_pad_config.swap_manager = params.swap_manager;
        ctx.accounts.vizing_pad_config.registered_validator = params.registered_validator;
        ctx.accounts.vizing_pad_config.trusted_relayers = params.trusted_relayers;

        ctx.accounts.vizing_pad_config.is_paused = params.is_paused;

        let (_, authority_bump) =
        Pubkey::find_program_address(&[VIZING_AUTHORITY_SEED, ctx.accounts.vizing_pad_config.key().as_ref()], &ctx.program_id);
        ctx.accounts.vizing_pad_authority.bump = authority_bump;

        Ok(())
    }
}


#[account]
#[derive(InitSpace)]
pub struct InitVizingPadParams {
    pub owner: Pubkey,
    pub fee_collector: Pubkey,
    pub engine_admin: Pubkey,
    pub station_admin: Pubkey,
    pub gas_pool_admin: Pubkey,
    pub swap_manager: Pubkey,
    #[max_len(96)]
    pub trusted_relayers: Vec<Pubkey>,
    pub registered_validator: Pubkey,
    pub is_paused: bool,
}

#[account]
#[derive(InitSpace)]
pub struct VizingPadConfigs {
    // immutable
    pub bump: u8,
    // configurable
    pub owner: Pubkey,
    pub fee_collector: Pubkey,
    pub engine_admin: Pubkey,
    pub station_admin: Pubkey,
    pub gas_pool_admin: Pubkey,
    pub swap_manager: Pubkey,
    #[max_len(96)]
    pub trusted_relayers: Vec<Pubkey>,
    pub registered_validator: Pubkey,
    // state
    pub is_paused: bool,
}

#[derive(Accounts)]
pub struct OwnerAuthorization<'info> {
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner @ VizingError::NotOwner, seeds = [contants::VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump, 
     constraint = vizing_pad_config.owner != contants::SYSTEM_ACCOUNT
        && vizing_pad_config.fee_collector != contants::SYSTEM_ACCOUNT
        && vizing_pad_config.engine_admin != contants::SYSTEM_ACCOUNT
        && vizing_pad_config.gas_pool_admin != contants::SYSTEM_ACCOUNT
        && vizing_pad_config.station_admin != contants::SYSTEM_ACCOUNT
        && vizing_pad_config.registered_validator != contants::SYSTEM_ACCOUNT)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
}

impl OwnerAuthorization<'_> {
    pub fn owner_management(
        ctx: &mut Context<OwnerAuthorization>,
        params: &OwnerManagementParams,
    ) -> Result<()> {
        ctx.accounts.vizing_pad_config.owner = params.owner;
        ctx.accounts.vizing_pad_config.fee_collector = params.fee_collector;
        ctx.accounts.vizing_pad_config.engine_admin = params.engine_admin;
        ctx.accounts.vizing_pad_config.gas_pool_admin = params.gas_pool_admin;
        ctx.accounts.vizing_pad_config.swap_manager = params.swap_manager;
        ctx.accounts.vizing_pad_config.station_admin = params.station_admin;
        ctx.accounts.vizing_pad_config.trusted_relayers = params.trusted_relayers.clone();
        ctx.accounts.vizing_pad_config.registered_validator = params.registered_validator;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct OwnerManagementParams {
    pub owner: Pubkey,
    pub fee_collector: Pubkey,
    pub engine_admin: Pubkey,
    pub gas_pool_admin: Pubkey,
    pub swap_manager: Pubkey,
    pub station_admin: Pubkey,
    #[max_len(96)]
    pub trusted_relayers: Vec<Pubkey>,
    pub registered_validator: Pubkey,
    pub is_paused: bool,
}

#[derive(Accounts)]
pub struct EngineAdminAuthorization<'info> {
    pub engine_admin: Signer<'info>,
    #[account(mut, has_one = engine_admin @ VizingError::NotEngineAdmin, seeds = [contants::VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump, 
     constraint = vizing_pad_config.owner != contants::SYSTEM_ACCOUNT
        && vizing_pad_config.fee_collector != contants::SYSTEM_ACCOUNT)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
}

impl EngineAdminAuthorization<'_> {
    pub fn pause_engine(ctx: &mut Context<EngineAdminAuthorization>) -> Result<()> {
        ctx.accounts.vizing_pad_config.is_paused = true;
        Ok(())
    }

    pub fn unpause_engine(ctx: &mut Context<EngineAdminAuthorization>) -> Result<()> {
        ctx.accounts.vizing_pad_config.is_paused = false;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(relayer: Pubkey)]
pub struct InitialRelayer<'info> {
    #[account(mut)]
    pub station_admin: Signer<'info>,

    #[account(mut, has_one = station_admin @ VizingError::NotOwner, seeds = [contants::VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump, 
     constraint = vizing_pad_config.owner != contants::SYSTEM_ACCOUNT
        && vizing_pad_config.station_admin != contants::SYSTEM_ACCOUNT)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    #[account(
        init,
        payer = station_admin,
        space = 8 + RelayerSettings::INIT_SPACE,
        seeds = [contants::RELAYER_SETTINGS_SEED, relayer.key().as_ref()],
        bump,
    )]
    pub relayer: Account<'info, RelayerSettings>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_relayer: Pubkey)]
pub struct GrantRelayer<'info> {
    #[account(mut)]
    pub station_admin: Signer<'info>,

    #[account(mut, has_one = station_admin @ VizingError::NotStationAdmin, seeds = [contants::VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump, 
     constraint = vizing_pad_config.owner != contants::SYSTEM_ACCOUNT
        || vizing_pad_config.station_admin != contants::SYSTEM_ACCOUNT)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct RelayerSettings {
    #[max_len(96)]
    pub new_trusted_relayers: Vec<Pubkey>,
}

impl GrantRelayer<'_> {
    pub fn grant_relayer(
        ctx: &mut Context<GrantRelayer>,
        _new_trusted_relayers: Vec<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.vizing_pad_config.trusted_relayers = _new_trusted_relayers;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GasPoolAdminAuthorization<'info> {
    #[account(mut)]
    pub gas_pool_admin: Signer<'info>,

    #[account(mut, has_one = gas_pool_admin @ VizingError::NotGasPoolAdmin, seeds = [contants::VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump, 
     constraint = vizing_pad_config.owner != contants::SYSTEM_ACCOUNT
        || vizing_pad_config.gas_pool_admin != contants::SYSTEM_ACCOUNT)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,

    pub system_program: Program<'info, System>,
}

impl GasPoolAdminAuthorization<'_> {
    pub fn grant_fee_collector(
        ctx: &mut Context<GasPoolAdminAuthorization>,
        _fee_collector: Pubkey,
    ) -> Result<()> {
        ctx.accounts.vizing_pad_config.fee_collector = _fee_collector;
        Ok(())
    }

    pub fn grant_swap_manager(ctx: &mut Context<GasPoolAdminAuthorization>, _swap_manager: Pubkey) -> Result<()> {
        ctx.accounts.vizing_pad_config.swap_manager = _swap_manager;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VizingAuthorityParams {
    pub bump: u8,
}
