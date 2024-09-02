pub mod governance;
pub mod library;
pub mod vizing_channel;
pub mod vizing_omni;
use anchor_lang::prelude::*;
use vizing_channel::*;
use vizing_omni::*;

declare_id!("vizngM8xTgmP15xuxpUZHbdec3LBG7bnTe9j1BtaqsE");

#[program]
pub mod vizing_pad {

    use super::*;

    // **********  channel start ************

    pub fn launch(mut _ctx: Context<LaunchOp>, _params: LaunchParams) -> Result<()> {
        Ok(())
    }

    // **********  channel end ************
}
