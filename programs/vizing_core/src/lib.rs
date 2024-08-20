pub mod channel;
pub mod governance;
pub mod library;
use anchor_lang::prelude::*;
use channel::*;
use governance::*;

pub mod vizing_gas_system;
pub mod error;
pub mod state;
pub mod message_type_lib;
pub mod l2_support_lib;
pub mod message_monitor_lib;
pub mod message_core;
pub mod expert_hook;

declare_id!("VzgnUvjAQD9oXjHLsVGrS67GCMVdzKpwHGj3QanJhk3");

#[program]
pub mod vizing_core {

    use super::*;

    // **********  channel start ************

    pub fn launch(mut ctx: Context<LaunchOp>, params: LaunchParams) -> Result<()> {
        LaunchOp::execute(&mut ctx, params)
    }

    // **********  channel end ************

    // **********  governance start ************
    pub fn initialize_vizing_pad(
        mut ctx: Context<InitVizingPad>,
        params: InitVizingPadParams,
    ) -> Result<()> {
        InitVizingPad::execute(&mut ctx, params)
    }

    pub fn initial_relayer(mut ctx: Context<InitialRelayer>, relayer: Pubkey) -> Result<()> {
        InitialRelayer::execute(&mut ctx, relayer)
    }

    pub fn modify_settings(
        mut ctx: Context<ModifySettings>,
        params: ModifySettingsParams,
    ) -> Result<()> {
        ModifySettings::execute(&mut ctx, &params)
    }

    pub fn pause_engine(mut ctx: Context<PauseEngine>) -> Result<()> {
        PauseEngine::pause_engine(&mut ctx)
    }

    pub fn unpause_engine(mut ctx: Context<PauseEngine>) -> Result<()> {
        PauseEngine::unpause_engine(&mut ctx)
    }

    pub fn grant_relayer(mut ctx: Context<GrantRelayer>, relayer: Pubkey) -> Result<()> {
        GrantRelayer::grant_relayer(&mut ctx, relayer)
    }

    pub fn revoke_relayer(mut ctx: Context<GrantRelayer>, relayer: Pubkey) -> Result<()> {
        GrantRelayer::revoke_relayer(&mut ctx, relayer)
    }

    pub fn grant_fee_collector(
        mut ctx: Context<GrantFeeCollector>,
        fee_collector: Pubkey,
    ) -> Result<()> {
        GrantFeeCollector::grant_fee_collector(&mut ctx, fee_collector)
    }
    // ***********  governance end ************
}
