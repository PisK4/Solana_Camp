use anchor_lang::prelude::*;

pub mod state;
declare_id!("46rihEC7YsmCNUN85tZfWCV371iRnx9a9HybmL7uDAgq");

#[program]
mod process {
    use super::*;
    
    pub fn init_power_user(ctx: Context<PowerUser>) -> Result<()> {

    }
}
