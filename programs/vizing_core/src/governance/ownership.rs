use crate::library::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(params: InitVizingPadParams)]
pub struct InitVizingPad<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + VizingPadSettings::INIT_SPACE,
        seeds = [contants::VIZING_PAD_SETTINGS_SEED],
        bump,
        constraint = vizing.owner != contants::SYSTEM_ACCOUNT
            && vizing.fee_receiver != contants::SYSTEM_ACCOUNT
            && vizing.engine_admin != contants::SYSTEM_ACCOUNT
            && vizing.station_admin != contants::SYSTEM_ACCOUNT
            && vizing.trusted_relayer != contants::SYSTEM_ACCOUNT
            && vizing.registered_validator != contants::SYSTEM_ACCOUNT
    )]
    pub vizing: Account<'info, VizingPadSettings>,

    #[account(
        init,
        payer = payer,
        space = 8 + RelayerSettings::INIT_SPACE,
        seeds = [contants::RELAYER_SETTINGS_SEED, params.trusted_relayer.key().as_ref()],
        bump,
    )]
    pub relayer: Account<'info, RelayerSettings>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl InitVizingPad<'_> {
    pub fn execute(ctx: &mut Context<InitVizingPad>, params: InitVizingPadParams) -> Result<()> {
        // init vizing settings
        let (_, bump) = Pubkey::find_program_address(
            &[contants::VIZING_PAD_SETTINGS_SEED],
            &ctx.program_id,
        );
        ctx.accounts.vizing.bump = bump;
        ctx.accounts.vizing.owner = params.owner;
        ctx.accounts.vizing.fee_receiver = params.fee_receiver;
        ctx.accounts.vizing.engine_admin = params.engine_admin;
        ctx.accounts.vizing.station_admin = params.station_admin;
        ctx.accounts.vizing.registered_validator = params.registered_validator;
        ctx.accounts.vizing.is_paused = params.is_paused;


        let (_, bump) = Pubkey::find_program_address(
            &[contants::RELAYER_SETTINGS_SEED],
            &ctx.program_id,
        );

        ctx.accounts.relayer.bump = bump;
        ctx.accounts.relayer.is_enabled = true;
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
    pub trusted_relayer: Pubkey,
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
    pub trusted_relayer: Pubkey,
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
        && vizing.station_admin != contants::SYSTEM_ACCOUNT
        && vizing.registered_validator != contants::SYSTEM_ACCOUNT)]
    pub vizing: Account<'info, VizingPadSettings>,
}

impl ModifySettings<'_> {
    pub fn execute(ctx: &mut Context<ModifySettings>, params: &ModifySettingsParams) -> Result<()> {
        ctx.accounts.vizing.owner = params.owner;
        ctx.accounts.vizing.fee_receiver = params.fee_receiver;
        ctx.accounts.vizing.engine_admin = params.engine_admin;
        ctx.accounts.vizing.station_admin = params.station_admin;
        ctx.accounts.vizing.registered_validator = params.registered_validator;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct ModifySettingsParams {
    pub owner: Pubkey,
    pub fee_receiver: Pubkey,
    pub engine_admin: Pubkey,
    pub station_admin: Pubkey,
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

impl  PauseEngine<'_> {
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

    #[account(mut, has_one = station_admin @ VizingError::NotOwner, seeds = [contants::VIZING_PAD_SETTINGS_SEED], bump = vizing.bump, 
     constraint = vizing.owner != contants::SYSTEM_ACCOUNT
        && vizing.station_admin != contants::SYSTEM_ACCOUNT)]
    pub vizing: Account<'info, VizingPadSettings>,

    #[account(
        mut,
        seeds = [contants::RELAYER_SETTINGS_SEED, _relayer.key().as_ref()],
        bump = relayer.bump,
        constraint = relayer.relayer == _relayer.key()
    )]
    pub relayer: Account<'info, RelayerSettings>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct RelayerSettings {
    // immutable
    pub bump: u8,
    // configurable
    pub relayer: Pubkey,

    pub is_enabled: bool,
}


impl InitialRelayer<'_> {
    pub fn execute(ctx: &mut Context<InitialRelayer>, relayer: Pubkey) -> Result<()> {
        let (_, bump) = Pubkey::find_program_address(
            &[contants::RELAYER_SETTINGS_SEED],
            &ctx.program_id,
        );
        ctx.accounts.relayer.relayer = relayer;
        ctx.accounts.relayer.bump = bump;
        ctx.accounts.relayer.is_enabled = true;
        Ok(())
    }
}

impl GrantRelayer<'_> {
    pub fn grant_relayer(ctx: &mut Context<GrantRelayer>, _relayer: Pubkey) -> Result<()> {
        ctx.accounts.relayer.is_enabled = true;
        Ok(())
    }

    pub fn revoke_relayer(ctx: &mut Context<GrantRelayer>,_relayer: Pubkey) -> Result<()> {
        ctx.accounts.relayer.is_enabled = false;
        Ok(())
    }
    
}

