pub mod channel;
pub mod errors;
pub mod governance;
pub mod library;
use anchor_lang::prelude::*;
use channel::*;
use errors::*;
use governance::*;

declare_id!("VzgnUvjAQD9oXjHLsVGrS67GCMVdzKpwHGj3QanJhk3");

#[program]
pub mod vizing_core {

    use super::*;

    pub fn initialize_vizing_pad(
        mut ctx: Context<InitVizingPad>,
        params: InitVizingPadParams,
    ) -> Result<()> {
        InitVizingPad::execute(&mut ctx, params)
    }

    pub fn modify_settings(
        mut ctx: Context<ModifySettings>,
        params: ModifySettingsParams,
    ) -> Result<()> {
        ModifySettings::execute(&mut ctx, &params)
    }

    pub fn launch(mut ctx: Context<LaunchOp>, params: LaunchParams) -> Result<()> {
        LaunchOp::execute(&mut ctx, params)
    }
}
