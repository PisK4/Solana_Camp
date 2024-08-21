use anchor_lang::prelude::*;

// use crate::error::ErrorCode;
// use crate::l2_support_lib::*;
// use crate::message_type_lib::*;
// use crate::state::*;
// use crate::message_monitor_lib::*;
// use crate::vizing_gas_system::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ExpertLandingHooks{
    pub key: u64,
    pub address: [u16; 20],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ExpertLaunchHooks{
    pub key: u64,
    pub address: [u16; 20],
}

#[account]
pub struct MappingExpertLandingHooks {
    pub expert_landing_hooks_mappings: Vec<ExpertLandingHooks>,
    pub valid: bool,
}

#[account]
pub struct MappingExpertLaunchHooks {
    pub expert_launch_hooks_mappings: Vec<ExpertLaunchHooks>,
    pub valid: bool,
}

impl MappingExpertLandingHooks{
    pub fn set(
        &mut self,
        key: u64,
        address: [u16; 20]
    ){
        if let Some(pair) = self
            .expert_landing_hooks_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.address = address;
        } else {
            self.expert_landing_hooks_mappings.push(ExpertLandingHooks {
                key,
                address,
            });
        }
        self.valid = true;
    }

    pub fn mapping_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    pub fn get(&self, key: u64)-> Option<ExpertLandingHooks> {
        self.expert_landing_hooks_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }
}

impl MappingExpertLaunchHooks{
    pub fn set(
        &mut self,
        key: u64,
        address: [u16; 20]
    ){
        if let Some(pair) = self
            .expert_launch_hooks_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.address = address;
        } else {
            self.expert_launch_hooks_mappings.push(ExpertLaunchHooks {
                key,
                address,
            });
        }
        self.valid = true;
    }

    pub fn mapping_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    pub fn get(&self, key: u64)-> Option<ExpertLaunchHooks> {
        self.expert_launch_hooks_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }
}

pub mod message_core {
    use super::*;

    pub fn set_expert_landing_hooks(
        ctx: Context<SetExpertLandingHooks>,
        ids: Vec<u64>,
        hooks: Vec<[u16; 20]>,
    ) -> Result<()>{
        let expert_landing_hooks=&mut ctx.accounts.expert_landing_hooks;
        for (i, &id) in ids.iter().enumerate(){
            expert_landing_hooks.set(
                id,
                hooks[i]
            );
        }
        Ok(())
    }

    pub fn set_expert_launch_hooks(
        ctx: Context<SetExpertLaunchHooks>,
        ids: Vec<u64>,
        hooks: Vec<[u16; 20]>,
    ) -> Result<()>{
        let expert_launch_hooks=&mut ctx.accounts.expert_launch_hooks;
        for (i, &id) in ids.iter().enumerate(){
            expert_launch_hooks.set(
                id,
                hooks[i]
            );
        }
        Ok(())
    }

    pub fn launch(
        ctx:Context<Launch>,
        earliestArrivalTimestamp: u64,
        latestArrivalTimestamp: u64,
        relayer: Pubkey,
        sender: Pubkey,
        value: u64,
        dest_chain_id: u64,
        addition_params: &[u16],
        message: &[u16]
    ) -> Result<()>{
        let expert_launch_hooks=&mut ctx.accounts.expert_launch_hooks;

        let mode = MessageType::fetch_msg_mode(&message);
        let expert_handler; 
        if(mode!=MessageType::Default){
            if let Some(get_expert_launch_hooks) = expert_launch_hooks.get(dest_chain_id) {
                expert_handler = get_expert_launch_hooks.address;
            } else {
                return Err(ErrorCode::InvalidMapping.into());
            }
        }
        Ok(())
    }

    

}

#[derive(Accounts)]
pub struct SetExpertLandingHooks<'info>{
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        init,
        payer = user, 
        space = 8 + 32,
        seeds = [b"init_expert_landing_hooks".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub expert_landing_hooks: Account<'info, MappingExpertLandingHooks>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetExpertLaunchHooks<'info>{
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        init,
        payer = user, 
        space = 8 + 32,
        seeds = [b"init_expert_launch_hooks".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub expert_launch_hooks: Account<'info, MappingExpertLaunchHooks>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Launch<'info>{
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_expert_landing_hooks".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub expert_landing_hooks: Account<'info, MappingExpertLandingHooks>,
    #[account(
        mut,
        seeds = [b"init_expert_launch_hooks".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub expert_launch_hooks: Account<'info, MappingExpertLaunchHooks>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}