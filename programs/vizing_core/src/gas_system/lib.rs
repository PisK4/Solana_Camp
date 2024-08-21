use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("BrBa8Lq9t5kGQkZ1cJaYkyLg17THGjpev6AE8HHMJJ5Q");

pub mod state;
pub mod error;
pub mod message_monitor_lib;
pub mod message_type_lib;
pub mod l2_support_lib;

pub mod vizing_gas_system;
pub mod expert_hook;

use crate::expert_hook::*;
use crate::vizing_gas_system::*;
use crate::state::*;
use crate::error::ErrorCode;

#[program]
mod hello_anchor {
    use super::*;

    //init
    pub fn save_chain_id(
        ctx: Context<SaveDestChainId>,
        dest_chain_id: Vec<u8>,
    ) -> Result<()>{
        let save=&mut ctx.accounts.save_chain_id;
        save.dest_chain_id=dest_chain_id;
        Ok(())
    }
    
    pub fn init_power_user(
        ctx: Context<InitPowerUser>,
        new_admin: Pubkey,
        new_engine_admin: Pubkey,
        new_station_admin: Pubkey,
        new_gas_pool_admin: Pubkey,
        new_trusted_relayer: Pubkey,
        new_registered_validator: Pubkey,
        new_gas_manager: Vec<Pubkey>,
        new_swap_manager: Vec<Pubkey>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        power_user.admin_role = new_admin;
        power_user.engine_admin = new_engine_admin;
        power_user.station_admin = new_station_admin;
        power_user.gas_pool_admin = new_gas_pool_admin;
        power_user.trusted_relayer = new_trusted_relayer;
        power_user.registered_validator = new_registered_validator;
        power_user.gas_manager = new_gas_manager;
        power_user.swap_manager = new_swap_manager;
        Ok(())
    }

    pub fn init_fee_config(
        ctx: Context<InitFeeConfig>,
        key: u64,
        base_price: u64,
        reserve: u64,
        molecular: u64,
        denominator: u64,
        molecular_decimal: u8,
        denominator_decimal: u8,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);

        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        mapping_fee_config.set_fee_config(
            key,
            base_price,
            reserve,
            molecular,
            denominator,
            molecular_decimal,
            denominator_decimal,
        );
        Ok(())
    }

    pub fn init_gas_global(
        ctx: Context<InitGasGlobal>,
        global_base_price: u64,
        default_gas_limit: u64,
        amount_in_threshold: u64,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);

        let gas_system_global = &mut ctx.accounts.gas_system_global;
        gas_system_global.global_base_price = global_base_price;
        gas_system_global.default_gas_limit = default_gas_limit;
        gas_system_global.amount_in_threshold = amount_in_threshold;

        let global_trade_fee = &mut ctx.accounts.global_trade_fee;
        global_trade_fee.molecular = molecular;
        global_trade_fee.denominator = denominator;
        Ok(())
    }

    pub fn init_amount_in_thresholds(
        ctx: Context<AmountInThresholds>,
        key: u64,
        value: u64,
    ) ->Result<()>{
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);

        let a = &mut ctx.accounts.amount_in_thresholds;
        a.set_amount_in_thresholds(
            key,
            value
        );
        Ok(())
    }

    pub fn init_native_token_trade_fee_config(
        ctx: Context<InitNativeTokenTradeFeeConfig>,
        key: u64,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);

        let n = &mut ctx.accounts.native_token_trade_fee_config;
        n.set_native_token_trade_fee_config(key, molecular, denominator);
        Ok(())
    }

    //set
    pub fn set_this_gas_global(
        ctx: Context<SetGasGlobal>,
        global_base_price: u64,
        default_gas_limit: u64,
        amount_in_threshold: u64,
        molecular: u64,
        denominator: u64,
    )-> Result<()> {
        SetGasGlobal::set_gas_global(
            ctx,
            global_base_price,
            default_gas_limit,
            amount_in_threshold,
            molecular,
            denominator
        )
    }


}
// !!!!!need limit SaveDestChainId
#[derive(Accounts)]
pub struct SaveDestChainId<'info> {
    #[account(
        init,
        payer = user, 
        space = 8 + 32,
    )]
    pub save_chain_id: Account<'info, SaveChainId>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitPowerUser<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info, SaveChainId>,
    #[account(
        init, 
        payer = user, 
        space = 8 + 512,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitGasGlobal<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        init,
        payer = user, 
        space = 8 + 64,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
    #[account(
        init,
        payer = user, 
        space = 8 + 32,
        seeds = [b"global_trade_fee".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub global_trade_fee: Account<'info, GlobalTradeFee>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitFeeConfig<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        init,
        payer = user, 
        space = 8 + 128,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AmountInThresholds<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        init,
        payer = user, 
        space = 8 + 32,
        seeds = [b"amount_in_thresholds".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub amount_in_thresholds: Account<'info, MappingAmountInThresholds>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitNativeTokenTradeFeeConfig<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        init,
        payer = user, 
        space = 8 + 32,
        seeds = [b"native_token_trade_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub native_token_trade_fee_config: Account<'info, MappingNativeTokenTradeFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
