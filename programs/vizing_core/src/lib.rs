pub mod governance;
pub mod library;
pub mod state;
pub mod vizing_channel;
pub mod vizing_omni;

use anchor_lang::prelude::*;
use governance::*;
use state::*;
use vizing_channel::*;
use vizing_omni::*;

declare_id!("VzgnUvjAQD9oXjHLsVGrS67GCMVdzKpwHGj3QanJhk3");

#[program]
pub mod vizing_core {

    use super::*;

    // **********  channel start ************

    pub fn launch(mut ctx: Context<LaunchOp>, params: LaunchParams) -> Result<()> {
        LaunchOp::vizing_launch(&mut ctx, params)
    }

    pub fn landing(mut ctx: Context<LandingOp>, params: LandingParams) -> Result<()> {
        LandingOp::vizing_landing(&mut ctx, params)
    }

    // **********  channel end ************

    // **********  vizing app config start ************

    pub fn register_vizing_app(
        mut ctx: Context<VizingAppRegister>,
        params: VizingAppRegisterParams,
    ) -> Result<()> {
        VizingAppRegister::register_vizing_app(&mut ctx, params)
    }

    pub fn update_vizing_app(
        mut ctx: Context<VizingAppManagement>,
        vizing_app_accounts: Vec<Pubkey>,
    ) -> Result<()> {
        VizingAppManagement::update_vizing_app_accounts(&mut ctx, vizing_app_accounts)
    }

    pub fn transfer_vizing_app_admin(
        mut ctx: Context<VizingAppManagement>,
        new_admin: Pubkey,
    ) -> Result<()> {
        VizingAppManagement::transfer_ownership(&mut ctx, new_admin)
    }

    // **********  vizing app config end ************

    // **********  governance start ************

    pub fn initialize_vizing_pad(
        mut ctx: Context<InitVizingPad>,
        params: InitVizingPadParams,
    ) -> Result<()> {
        InitVizingPad::initialize_vizing_pad(&mut ctx, params)
    }

    pub fn modify_settings(
        mut ctx: Context<ModifySettings>,
        params: OwnerManagementParams,
    ) -> Result<()> {
        ModifySettings::owner_management(&mut ctx, &params)
    }

    pub fn pause_engine(mut ctx: Context<PauseEngine>) -> Result<()> {
        PauseEngine::pause_engine(&mut ctx)
    }

    pub fn unpause_engine(mut ctx: Context<PauseEngine>) -> Result<()> {
        PauseEngine::unpause_engine(&mut ctx)
    }

    pub fn grant_relayer(
        mut ctx: Context<GrantRelayer>,
        new_trusted_relayers: Vec<Pubkey>,
    ) -> Result<()> {
        GrantRelayer::grant_relayer(&mut ctx, new_trusted_relayers)
    }

    pub fn grant_fee_collector(
        mut ctx: Context<GrantFeeCollector>,
        fee_collector: Pubkey,
    ) -> Result<()> {
        GrantFeeCollector::grant_fee_collector(&mut ctx, fee_collector)
    }

    // ***********  governance end ************
}
