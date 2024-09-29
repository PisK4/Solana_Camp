use anchor_lang::prelude::*;
use vizing_pad::{
    self,
    cpi::accounts::{VizingAppManagement, VizingAppRegister},
    cpi::{register_vizing_app, transfer_vizing_app_admin, update_vizing_app},
};

pub const VIZING_APP_CONFIG_SEED: &[u8] = b"Vizing_App_Config_Seed";
pub const VIZING_APP_AUTHORITY_SEED: &[u8] = b"Vizing_App_Authority_Seed";

#[derive(Accounts)]
pub struct RegisterVizingApp<'info> {
    /// CHECK: 1. admin of the vizing app
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,

    /// CHECK: 2. vizing app config
    pub vizing_app_configs: AccountInfo<'info>,

    /// CHECK: 3. Vizing Pad
    pub vizing_pad_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct VizingAppConfig {
    pub sol_pda_receiver: Pubkey,
    #[max_len(160)]
    pub vizing_app_accounts: Vec<Pubkey>,
    pub vizing_app_program_id: Pubkey,
    pub admin: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VizingAppRegisterParams {
    pub sol_pda_receiver: Pubkey,
    #[max_len(160)]
    pub vizing_app_accounts: Vec<Pubkey>,
    pub vizing_app_program_id: Pubkey,
}

// #[derive(Accounts)]
// #[instruction(params: VizingAppRegisterParams)]
// pub struct VizingAppManagement<'info> {
//     /// CHECK: We need signer to claim ownership
//     #[account(mut, signer)]
//     pub admin: AccountInfo<'info>,

//     #[account(
//         seeds = [VIZING_APP_CONFIG_SEED, &vizing_app_configs.vizing_app_program_id.to_bytes()],
//         bump = vizing_app_configs.bump,
//         has_one = admin
//     )]
//     pub vizing_app_configs: Account<'info, VizingAppConfig>,

//     pub system_program: Program<'info, System>,
// }

pub fn apply_register_vizing_app<'c: 'info, 'info>(
    register_params: VizingAppRegisterParams,
    curr_program_id: &Pubkey,
    vizing_pad_program: &AccountInfo<'info>,
    admin: &AccountInfo<'info>,
    vizing_app_configs: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
) -> Result<()> {
    let params = vizing_pad::vizing_omni::VizingAppRegisterParams {
        sol_pda_receiver: register_params.sol_pda_receiver,
        vizing_app_accounts: register_params.vizing_app_accounts,
        vizing_app_program_id: register_params.vizing_app_program_id,
    };

    let (_, bump_authority) =
        Pubkey::find_program_address(&[VIZING_APP_AUTHORITY_SEED], curr_program_id);

    let seeds = &[VIZING_APP_AUTHORITY_SEED, &[bump_authority]];

    let signer = &[&seeds[..]];

    let ctx = CpiContext::new_with_signer(
        vizing_pad_program.clone(),
        VizingAppRegister {
            admin: admin.clone(),
            vizing_app_configs: vizing_app_configs.clone(),
            system_program: system_program.clone(),
        },
        signer,
    );

    Ok(register_vizing_app(ctx, params)?)
}
