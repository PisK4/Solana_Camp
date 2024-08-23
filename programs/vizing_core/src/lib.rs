pub mod channel;
pub mod governance;
pub mod library;
use channel::*;
use governance::*;

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

// pub mod state;
// pub mod error;
// pub mod message_monitor_lib;
// pub mod message_type_lib;
// pub mod l2_support_lib;

// pub mod vizing_gas_system;
// pub mod expert_hook;

// use crate::expert_hook::*;
// use crate::vizing_gas_system::*;
// use crate::state::*;
// use crate::error::ErrorCode;

declare_id!("VzgnUvjAQD9oXjHLsVGrS67GCMVdzKpwHGj3QanJhk3");

#[program]
pub mod vizing_core {

    use super::*;

    // **********  channel start ************

    pub fn launch(mut ctx: Context<LaunchOp>, params: LaunchParams) -> Result<()> {
        LaunchOp::execute(&mut ctx, params)
    }

    pub fn landing(mut ctx: Context<LandingOp>, params: LandingParams) -> Result<()> {
        LandingOp::execute(&mut ctx, params)
    }

    // **********  channel end ************

    // **********  governance start ************

    pub fn initialize_vizing_pad(
        mut ctx: Context<InitVizingPad>,
        params: InitVizingPadParams,
    ) -> Result<()> {
        InitVizingPad::execute(&mut ctx, params)
    }

    pub fn modify_settings(
        mut ctx: Context<ModifySettings>,
        params: OwnerManagementParams,
    ) -> Result<()> {
        ModifySettings::execute(&mut ctx, &params)
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
        new_gas_managers: Vec<Pubkey>,
        new_swap_managers: Vec<Pubkey>,
        new_token_managers: Vec<Pubkey>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let vizing_vault = &mut ctx.accounts.vizing_vault;
        power_user.admin_role = new_admin;
        power_user.engine_admin = new_engine_admin;
        power_user.station_admin = new_station_admin;
        power_user.gas_pool_admin = new_gas_pool_admin;
        power_user.trusted_relayer = new_trusted_relayer;
        power_user.registered_validator = new_registered_validator;
        power_user.gas_managers = new_gas_managers;
        power_user.swap_managers = new_swap_managers;
        power_user.token_managers = new_token_managers;

        vizing_vault.current_pragma_id=*ctx.program_id;
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
        let if_power_user = power_user.gas_managers.contains(user_key);
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
        let if_power_user = power_user.gas_managers.contains(user_key);
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
        let if_power_user = power_user.gas_managers.contains(user_key);
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
        let if_power_user = power_user.gas_managers.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);

        let n = &mut ctx.accounts.native_token_trade_fee_config;
        n.set_native_token_trade_fee_config(key, molecular, denominator);
        Ok(())
    }

    //set
    //set_gas_global
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

    //set_fee_config    
    pub fn set_this_fee_config(
        ctx: Context<SetFeeConfig>,
        key: u64,
        base_price: u64,
        reserve: u64,
        molecular: u64,
        denominator: u64,
        molecular_decimal: u8,
        denominator_decimal: u8,
    )-> Result<()> {
        SetFeeConfig::set_fee_config(
            ctx,
            key,
            base_price,
            reserve,
            molecular,
            denominator,
            molecular_decimal,
            denominator_decimal,
        )
    }

    //set_token_fee_config
    pub fn set_this_token_fee_config(
        ctx: Context<SetTokenFeeConfig>,
        key: u64,
        molecular: u64,
        denominator: u64,
    )-> Result<()> {
        SetTokenFeeConfig::set_token_fee_config(
            ctx,
            key,
            molecular,
            denominator
        )
    }

    
    //set_dapp_price_config
    pub fn set_this_dapp_price_config(
        ctx: Context<SetDappPriceConfig>,
        chain_id: u64,
        dapp: [u16; 20],
        base_price: u64,
    )-> Result<()> {
        SetDappPriceConfig::set_dapp_price_config(
            ctx,
            chain_id,
            dapp,
            base_price
        )
    }

    //set_exchange_rate
    pub fn set_this_exchange_rate(
        ctx: Context<SetExchangeRate>,
        chain_id: u64,
        molecular: u64,
        denominator: u64,
        molecular_decimal: u8,
        denominator_decimal: u8,
    )-> Result<()> {
        SetExchangeRate::set_exchange_rate(
            ctx,
            chain_id,
            molecular,
            denominator,
            molecular_decimal,
            denominator_decimal
        )
    }

    //batch_set_token_fee_config
    pub fn batch_set_this_token_fee_config(
        ctx: Context<BatchSetTokenFeeConfig>,
        dest_chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    )-> Result<()> {
        BatchSetTokenFeeConfig::batch_set_token_fee_config(
            ctx,
            dest_chain_ids,
            moleculars,
            denominators
        )
    }

    //batch_set_trade_fee_config_map
    pub fn batch_set_this_trade_fee_config_map(
        ctx: Context<BatchSetTradeFeeConfigMap>,
        dapps: Vec<[u16; 20]>,
        dest_chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    )-> Result<()> {
        BatchSetTradeFeeConfigMap::batch_set_trade_fee_config_map(
            ctx,
            dapps,
            dest_chain_ids,
            moleculars,
            denominators
        )
    }

    //batch_set_amount_in_threshold
    pub fn batch_set_this_amount_in_threshold(
        ctx: Context<BatchSetAmountInThreshold>,
        chain_ids: Vec<u64>,
        new_values: Vec<u64>,
    )-> Result<()> {
        BatchSetAmountInThreshold::batch_set_amount_in_threshold(
            ctx,
            chain_ids,
            new_values
        )
    }

    //batch_set_dapp_price_config_in_diff_chain
    pub fn batch_set_this_dapp_price_config_in_diff_chain(
        ctx: Context<BatchSetDappPriceConfigInDiffChain>,
        chain_ids: Vec<u64>,
        dapps: Vec<[u16; 20]>,
        base_prices: Vec<u64>,
    )-> Result<()> {
        BatchSetDappPriceConfigInDiffChain::batch_set_dapp_price_config_in_diff_chain(
            ctx,
            chain_ids,
            dapps,
            base_prices
        )
    }

    //batch_set_dapp_price_config_in_same_chain
    pub fn batch_set_this_dapp_price_config_in_same_chain(
        ctx: Context<BatchSetDappPriceConfigInSameChain>,
        chain_id: u64,
        dapps: Vec<[u16; 20]>,
        base_prices: Vec<u64>,
    )-> Result<()> {
        BatchSetDappPriceConfigInSameChain::batch_set_dapp_price_config_in_same_chain(
            ctx,
            chain_id,
            dapps,
        base_prices
        )
    }

    //batch_set_exchange_rate
    pub fn batch_set_this_exchange_rate(
        ctx: Context<BatchSetExchangeRate>,
        chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
        molecular_decimals: Vec<u8>,
        denominator_decimals: Vec<u8>,
    )-> Result<()> {
        BatchSetExchangeRate::batch_set_exchange_rate(
            ctx,
            chain_ids,
            moleculars,
            denominators,
            molecular_decimals,
            denominator_decimals
        )
    }

    //change_power_user
    pub fn change_this_power_user(
        ctx: Context<ChangePowerUser>,
        new_admin: Pubkey,
        new_engine_admin: Pubkey,
        new_station_admin: Pubkey,
        new_gas_pool_admin: Pubkey,
        new_trusted_relayer: Pubkey,
        new_registered_validator: Pubkey,
        new_gas_managers: Vec<Pubkey>,
        new_swap_managers: Vec<Pubkey>,
        new_token_managers: Vec<Pubkey>,
    ) -> Result<()> {
        ChangePowerUser::change_power_user(
            ctx,
            new_admin,
            new_engine_admin,
            new_station_admin,
            new_gas_pool_admin,
            new_trusted_relayer,
            new_registered_validator,
            new_gas_managers,
            new_swap_managers,
            new_token_managers,
        )
    }
    
    //user=>vizing_vault
    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        );

        transfer(cpi_context, amount)?;
        Ok(())
    }

    //withdraw_spl_token
    pub fn withdraw_vault_spl_token(
        ctx: Context<WithdrawSplToken>,
        withdraw_amount: u64,
        this_bump: u8,
    )-> Result<()> {
        WithdrawSplToken::withdraw_spl_token(
            ctx,
            withdraw_amount,
            this_bump
        )
    }

    //withdraw_sol
    pub fn withdraw_vault_sol(
        ctx: Context<WithdrawSol>, 
        withdraw_amount: u64,
    )-> Result<()> {
        WithdrawSol::withdraw_sol(
            ctx,
            withdraw_amount
        )
    }

    //set_token_info_base
    pub fn set_this_token_info_base(
        ctx: Context<SetTokenInfoBase>,
        symbol: Vec<u8>,
        token_address: [u16; 20],
        decimals: u8,
        max_price: u64,
    )-> Result<()> {
        SetTokenInfoBase::set_token_info_base(
            ctx,
            symbol,
            token_address,
            decimals,
            max_price
        )
    }

    //set_token_trade_fee_map
    pub fn set_this_token_trade_fee_map(
        ctx: Context<SetTokenTradeFeeMap>,
        token_address: [u16; 20],
        chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    )-> Result<()> {
        SetTokenTradeFeeMap::set_token_trade_fee_map(
            ctx,
            token_address,
            chain_ids,
            moleculars,
            denominators
        )
    }
}


// !!!!!need limit SaveDestChainId
#[derive(Accounts)]
pub struct SaveDestChainId<'info> {
    #[account(
        init,
        payer = user, 
        space = 8 + 128,
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
    #[account(
        init, 
        payer = user, 
        space = 8 + 256,
        seeds = [b"vizing_vault".as_ref()],
        bump
    )]
    pub vizing_vault: Account<'info, VaultMes>,
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
        space = 8 + 128,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
    #[account(
        init,
        payer = user, 
        space = 8 + 128,
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
        space = 8 + 256,
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
        space = 8 + 128,
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
        space = 8 + 128,
        seeds = [b"native_token_trade_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub native_token_trade_fee_config: Account<'info, MappingNativeTokenTradeFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    recipient: AccountInfo<'info>,
    system_program: Program<'info, System>,
}
