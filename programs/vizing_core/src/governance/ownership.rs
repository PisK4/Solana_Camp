use anchor_lang::prelude::*;
use crate::library::*;

#[derive(Accounts)]
#[instruction(params: InitVizingPadParams)]
pub struct InitVizingPad<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + VizingPadSettings::INIT_SPACE,
        seeds = [VIZING_PAD_SETTINGS_SEED],
        bump,
    )]
    pub vizing: Account<'info, VizingPadSettings>,

    /// CHECK: We need this PDA as a signer
    #[account(
        init,
        payer = payer,
        space = 8 + VizingAuthorityParams::INIT_SPACE,
        seeds = [VIZING_AUTHORITY_SEED],
        bump,
    )]
    pub vizing_authority: Account<'info, VizingAuthorityParams>,

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
        let (_, bump) =
            Pubkey::find_program_address(&[contants::VIZING_PAD_SETTINGS_SEED], &ctx.program_id);
        ctx.accounts.vizing.bump = bump;
        ctx.accounts.vizing.owner = params.owner;
        ctx.accounts.vizing.fee_receiver = params.fee_receiver;
        ctx.accounts.vizing.engine_admin = params.engine_admin;
        ctx.accounts.vizing.station_admin = params.station_admin;
        ctx.accounts.vizing.gas_pool_admin = params.gas_pool_admin;
        ctx.accounts.vizing.registered_validator = params.registered_validator;
        ctx.accounts.vizing.is_paused = params.is_paused;

        ctx.accounts.vizing.trusted_relayers = params.trusted_relayers;

        let (_, bump) =
            Pubkey::find_program_address(&[contants::VIZING_AUTHORITY_SEED], &ctx.program_id);

        ctx.accounts.vizing_authority.bump = bump;

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct InitVizingPadParams {
    pub owner: Pubkey,
    pub fee_receiver: Pubkey,
    pub engine_admin: Pubkey,
    pub station_admin: Pubkey,
    pub gas_pool_admin: Pubkey,
    #[max_len(96)]
    pub trusted_relayers: Vec<Pubkey>,
    pub registered_validator: Pubkey,
    pub is_paused: bool,
}

#[account]
#[derive(InitSpace)]
pub struct VizingPadSettings {
    // immutable
    pub bump: u8,
    // configurable
    pub owner: Pubkey,
    pub fee_receiver: Pubkey,
    pub engine_admin: Pubkey,
    pub station_admin: Pubkey,
    pub gas_pool_admin: Pubkey,
    #[max_len(96)]
    pub trusted_relayers: Vec<Pubkey>,
    pub registered_validator: Pubkey,
    // state
    pub is_paused: bool,
}

#[derive(Accounts)]
pub struct ModifySettings<'info> {
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner @ VizingError::NotOwner, seeds = [contants::VIZING_PAD_SETTINGS_SEED], bump = vizing.bump, 
     constraint = vizing.owner != contants::SYSTEM_ACCOUNT
        && vizing.fee_receiver != contants::SYSTEM_ACCOUNT
        && vizing.engine_admin != contants::SYSTEM_ACCOUNT
        && vizing.gas_pool_admin != contants::SYSTEM_ACCOUNT
        && vizing.station_admin != contants::SYSTEM_ACCOUNT
        && vizing.registered_validator != contants::SYSTEM_ACCOUNT)]
    pub vizing: Account<'info, VizingPadSettings>,
}

impl ModifySettings<'_> {
    pub fn owner_management(
        ctx: &mut Context<ModifySettings>,
        params: &OwnerManagementParams,
    ) -> Result<()> {
        ctx.accounts.vizing.owner = params.owner;
        ctx.accounts.vizing.fee_receiver = params.fee_receiver;
        ctx.accounts.vizing.engine_admin = params.engine_admin;
        ctx.accounts.vizing.gas_pool_admin = params.gas_pool_admin;
        ctx.accounts.vizing.station_admin = params.station_admin;
        ctx.accounts.vizing.trusted_relayers = params.trusted_relayers.clone();
        ctx.accounts.vizing.registered_validator = params.registered_validator;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct OwnerManagementParams {
    pub owner: Pubkey,
    pub fee_receiver: Pubkey,
    pub engine_admin: Pubkey,
    pub gas_pool_admin: Pubkey,
    pub station_admin: Pubkey,
    #[max_len(96)]
    pub trusted_relayers: Vec<Pubkey>,
    pub registered_validator: Pubkey,
    pub is_paused: bool,
}

#[derive(Accounts)]
pub struct PauseEngine<'info> {
    pub engine_admin: Signer<'info>,
    #[account(mut, has_one = engine_admin @ VizingError::NotEngineAdmin, seeds = [contants::VIZING_PAD_SETTINGS_SEED], bump = vizing.bump, 
     constraint = vizing.owner != contants::SYSTEM_ACCOUNT
        && vizing.fee_receiver != contants::SYSTEM_ACCOUNT)]
    pub vizing: Account<'info, VizingPadSettings>,
}

impl PauseEngine<'_> {
    pub fn pause_engine(ctx: &mut Context<PauseEngine>) -> Result<()> {
        ctx.accounts.vizing.is_paused = true;
        Ok(())
    }

    pub fn unpause_engine(ctx: &mut Context<PauseEngine>) -> Result<()> {
        ctx.accounts.vizing.is_paused = false;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(relayer: Pubkey)]
pub struct InitialRelayer<'info> {
    #[account(mut)]
    pub station_admin: Signer<'info>,

    #[account(mut, has_one = station_admin @ VizingError::NotOwner, seeds = [contants::VIZING_PAD_SETTINGS_SEED], bump = vizing.bump, 
     constraint = vizing.owner != contants::SYSTEM_ACCOUNT
        && vizing.station_admin != contants::SYSTEM_ACCOUNT)]
    pub vizing: Account<'info, VizingPadSettings>,

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

    #[account(mut, has_one = station_admin @ VizingError::NotStationAdmin, seeds = [contants::VIZING_PAD_SETTINGS_SEED], bump = vizing.bump, 
     constraint = vizing.owner != contants::SYSTEM_ACCOUNT
        || vizing.station_admin != contants::SYSTEM_ACCOUNT)]
    pub vizing: Account<'info, VizingPadSettings>,

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
        ctx.accounts.vizing.trusted_relayers = _new_trusted_relayers;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GrantFeeCollector<'info> {
    #[account(mut)]
    pub gas_pool_admin: Signer<'info>,

    #[account(mut, has_one = gas_pool_admin @ VizingError::NotGasPoolAdmin, seeds = [contants::VIZING_PAD_SETTINGS_SEED], bump = vizing.bump, 
     constraint = vizing.owner != contants::SYSTEM_ACCOUNT
        || vizing.gas_pool_admin != contants::SYSTEM_ACCOUNT)]
    pub vizing: Account<'info, VizingPadSettings>,

    pub system_program: Program<'info, System>,
}

impl GrantFeeCollector<'_> {
    pub fn grant_fee_collector(
        ctx: &mut Context<GrantFeeCollector>,
        _fee_collector: Pubkey,
    ) -> Result<()> {
        ctx.accounts.vizing.fee_receiver = _fee_collector;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VizingAuthorityParams {
    pub bump: u8,
}
