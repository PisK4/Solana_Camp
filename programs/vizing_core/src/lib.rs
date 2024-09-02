pub mod gas_system;
pub mod governance;
pub mod library;
pub mod state;
pub mod vizing_channel;
pub mod vizing_omni;

use anchor_lang::prelude::*;
use gas_system::*;
use governance::*;
use library::*;
use state::*;
use vizing_channel::*;
use vizing_omni::*;

declare_id!("F4syPi7pUoujHYMDGWhHLPJKSqLxi7r6Jff6RLqYiWhp");

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

    //init
    pub fn init_power_user(
        ctx: Context<InitPowerUser>,
        new_admin: Pubkey,
        new_engine_admin: Pubkey,
        new_station_admin: Pubkey,
        new_gas_pool_admin: Pubkey,
        new_trusted_relayers: Vec<Pubkey>,
        new_registered_validators: Vec<Pubkey>,
        new_gas_managers: Vec<Pubkey>,
        new_swap_managers: Vec<Pubkey>,
        new_token_managers: Vec<Pubkey>,
    ) -> Result<()> {
        InitPowerUser::initialize_power_user(
            ctx,
            new_admin,
            new_engine_admin,
            new_station_admin,
            new_gas_pool_admin,
            new_trusted_relayers,
            new_registered_validators,
            new_gas_managers,
            new_swap_managers,
            new_token_managers
        )
    }

    pub fn save_chain_id(ctx: Context<SaveDestChainId>, dest_chain_id: Vec<u8>) -> Result<()> {
        SaveDestChainId::set_chain_id(
            ctx,
            dest_chain_id
        )
    }

    pub fn init_vizing_vault(ctx: Context<InitVizingVault>) -> Result<()> {
        InitVizingVault::initialize_vizing_vault(
            ctx
        )
    }

    pub fn init_gas_global(
        ctx: Context<InitGasGlobal>,
        global_base_price: u64,
        default_gas_limit: u64,
        amount_in_threshold: u64,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
       InitGasGlobal::initialize_gas_global(
            ctx,
            global_base_price,
            default_gas_limit,
            amount_in_threshold,
            molecular,
            denominator
       )
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
        InitFeeConfig::initialize_fee_config(
            ctx,
            key,
            base_price,
            reserve,
            molecular,
            denominator,
            molecular_decimal,
            denominator_decimal
        )
    }

    pub fn init_amount_in_thresholds(
        ctx: Context<InitAmountInThresholds>,
        key: u64,
        value: u64,
    ) -> Result<()> {
        InitAmountInThresholds::initialize_amount_in_thresholds(
            ctx,
            key,
            value
        )
    }

    pub fn init_native_token_trade_fee_config(
        ctx: Context<InitNativeTokenTradeFeeConfig>,
        key: u64,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
        InitNativeTokenTradeFeeConfig::initialize_native_token_trade_fee_config(
            ctx,
            key,
            molecular,
            denominator
        )
    }

    pub fn init_token_info_base(
        ctx: Context<InitTokenInfoBase>,
        symbol: Vec<u8>,
        token_address: [u8; 32],
        decimals: u8,
        max_price: u64,
    ) -> Result<()> {
        InitTokenInfoBase::initialize_token_info_base(
            ctx,
            symbol,
            token_address,
            decimals,
            max_price
        )
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
    ) -> Result<()> {
        SetGasGlobal::set_gas_global(
            ctx,
            global_base_price,
            default_gas_limit,
            amount_in_threshold,
            molecular,
            denominator,
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
    ) -> Result<()> {
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
    ) -> Result<()> {
        SetTokenFeeConfig::set_token_fee_config(ctx, key, molecular, denominator)
    }

    //set_dapp_price_config
    pub fn set_this_dapp_price_config(
        ctx: Context<SetDappPriceConfig>,
        chain_id: u64,
        dapp: [u8; 32],
        base_price: u64,
    ) -> Result<()> {
        SetDappPriceConfig::set_dapp_price_config(ctx, chain_id, dapp, base_price)
    }

    //set_exchange_rate
    pub fn set_this_exchange_rate(
        ctx: Context<SetExchangeRate>,
        chain_id: u64,
        molecular: u64,
        denominator: u64,
        molecular_decimal: u8,
        denominator_decimal: u8,
    ) -> Result<()> {
        SetExchangeRate::set_exchange_rate(
            ctx,
            chain_id,
            molecular,
            denominator,
            molecular_decimal,
            denominator_decimal,
        )
    }

    //batch_set_token_fee_config
    pub fn batch_set_this_token_fee_config(
        ctx: Context<BatchSetTokenFeeConfig>,
        dest_chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    ) -> Result<()> {
        BatchSetTokenFeeConfig::batch_set_token_fee_config(
            ctx,
            dest_chain_ids,
            moleculars,
            denominators,
        )
    }

    //batch_set_trade_fee_config_map
    pub fn batch_set_this_trade_fee_config_map(
        ctx: Context<BatchSetTradeFeeConfigMap>,
        dapps: Vec<[u8; 32]>,
        dest_chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    ) -> Result<()> {
        BatchSetTradeFeeConfigMap::batch_set_trade_fee_config_map(
            ctx,
            dapps,
            dest_chain_ids,
            moleculars,
            denominators,
        )
    }

    //batch_set_amount_in_threshold
    pub fn batch_set_this_amount_in_threshold(
        ctx: Context<BatchSetAmountInThreshold>,
        chain_ids: Vec<u64>,
        new_values: Vec<u64>,
    ) -> Result<()> {
        BatchSetAmountInThreshold::batch_set_amount_in_threshold(ctx, chain_ids, new_values)
    }

    //batch_set_dapp_price_config_in_diff_chain
    pub fn batch_set_this_dapp_price_config_in_diff_chain(
        ctx: Context<BatchSetDappPriceConfigInDiffChain>,
        chain_ids: Vec<u64>,
        dapps: Vec<[u8; 32]>,
        base_prices: Vec<u64>,
    ) -> Result<()> {
        BatchSetDappPriceConfigInDiffChain::batch_set_dapp_price_config_in_diff_chain(
            ctx,
            chain_ids,
            dapps,
            base_prices,
        )
    }

    //batch_set_dapp_price_config_in_same_chain
    pub fn batch_set_this_dapp_price_config_in_same_chain(
        ctx: Context<BatchSetDappPriceConfigInSameChain>,
        chain_id: u64,
        dapps: Vec<[u8; 32]>,
        base_prices: Vec<u64>,
    ) -> Result<()> {
        BatchSetDappPriceConfigInSameChain::batch_set_dapp_price_config_in_same_chain(
            ctx,
            chain_id,
            dapps,
            base_prices,
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
    ) -> Result<()> {
        BatchSetExchangeRate::batch_set_exchange_rate(
            ctx,
            chain_ids,
            moleculars,
            denominators,
            molecular_decimals,
            denominator_decimals,
        )
    }

    //change_power_user
    pub fn change_this_power_user(
        ctx: Context<ChangePowerUser>,
        new_admin: Pubkey,
        new_engine_admin: Pubkey,
        new_station_admin: Pubkey,
        new_gas_pool_admin: Pubkey,
        new_trusted_relayers: Vec<Pubkey>,
        new_registered_validators: Vec<Pubkey>,
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
            new_trusted_relayers,
            new_registered_validators,
            new_gas_managers,
            new_swap_managers,
            new_token_managers,
        )
    }

    // pub fn transfer_sol_valut(ctx: Context<SolTransfer>, amount: u64) ->Result<()>{
    //     SolTransfer::sol_transfer(
    //         ctx,
    //         amount
    //     )
    // }

    //withdraw_spl_token
    pub fn withdraw_vault_spl_token(
        ctx: Context<WithdrawSplToken>,
        withdraw_amount: u64,
        this_bump: u8,
    ) -> Result<()> {
        WithdrawSplToken::withdraw_spl_token(ctx, withdraw_amount, this_bump)
    }

    //withdraw_sol
    pub fn withdraw_vault_sol(ctx: Context<WithdrawSol>, withdraw_amount: u64) -> Result<()> {
        WithdrawSol::withdraw_sol(ctx, withdraw_amount)
    }

    //set_token_info_base
    pub fn set_this_token_info_base(
        ctx: Context<SetTokenInfoBase>,
        symbol: Vec<u8>,
        token_address: [u8; 32],
        decimals: u8,
        max_price: u64,
    ) -> Result<()> {
        SetTokenInfoBase::set_token_info_base(ctx, symbol, token_address, decimals, max_price)
    }

    //set_token_trade_fee_map
    pub fn set_this_token_trade_fee_map(
        ctx: Context<SetTokenTradeFeeMap>,
        token_address: [u8; 32],
        chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    ) -> Result<()> {
        SetTokenTradeFeeMap::set_token_trade_fee_map(
            ctx,
            token_address,
            chain_ids,
            moleculars,
            denominators,
        )
    }

    //get
    /*
        /// @notice Calculate the fee for the native token transfer
        /// @param amount The value we spent in the source chain
    */
    pub fn compute_trade_fee1(
        ctx: Context<ComputeTradeFee1>,
        dest_chain_id: u64,
        amount_out: u64,
    ) -> Result<u64>{
        ComputeTradeFee1::get_compute_trade_fee1(
            ctx,
            dest_chain_id,
            amount_out
        )
    }

    pub fn compute_trade_fee2(
        ctx: Context<ComputeTradeFee2>,
        target_contract: [u8; 32],
        dest_chain_id: u64,
        amount_out: u64,
    ) -> Result<u64> {
        ComputeTradeFee2::get_compute_trade_fee2(
            ctx,
            target_contract,
            dest_chain_id,
            amount_out
        )
    }

    /*
        /// @notice Estimate the gas price we need to encode in message
        /// @param destChainid The chain id of the destination chain
    */
    pub fn estimate_price2(
        ctx: Context<EstimatePrice2>,
        dest_chain_id: u64
    ) -> Result<u64> {
        EstimatePrice2::get_estimate_price2(
            ctx,
            dest_chain_id
        )
    }

    /*
        /// @notice Estimate the gas fee we should pay to vizing
        /// @param amountOut amountOut in the destination chain
        /// @param destChainid The chain id of the destination chain
        /// @param message The message we want to send to the destination chain
    */
    pub fn estimate_gas(
        ctx: Context<EstimateGas>,
        amount_out: u64,
        dest_chain_id: u64,
        message: Message
    ) -> Result<u64> {
        EstimateGas::get_estimate_gas(
            ctx,
            amount_out,
            dest_chain_id,
            message
        )
    }

    /*
        /// @notice Estimate the total fee we should pay to vizing
        /// @param amountOut amountOut in the destination chain
        /// @param destChainid The chain id of the destination chain
        /// @param message The message we want to send to the destination chain
    */
    pub fn estimate_total_fee(
        ctx: Context<EstimateTotalFee>,
        dest_chain_id: u64,
        amount_out: u64,
        message: Message
    ) -> Result<u64> {
        EstimateTotalFee::get_estimate_total_fee(
            ctx,
            dest_chain_id,
            amount_out,
            message
        )
    }

    /*
        /// @notice similar to uniswap Swap Router
        /// @notice Estimate how many native token we should spend to exchange the amountOut in the destChainid
        /// @param destChainid The chain id of the destination chain
        /// @param amountOut The value we want to receive in the destination chain
    */
    pub fn exact_output(
        ctx: Context<ExactOutput>,
        dest_chain_id: u64,
        amount_out: u64,    
    ) -> Result<u64> {
        ExactOutput::get_exact_output(
            ctx,
            dest_chain_id,
            amount_out
        )
    }

    /*
        /// @notice similar to uniswap Swap Router
        /// @notice Estimate how many native token we could get in the destChainid if we input the amountIn
        /// @param destChainid The chain id of the destination chain
        /// @param amountIn The value we spent in the source chain
    */
    pub fn exact_input(
        ctx: Context<ExactInput>,
        dest_chain_id: u64,
        amount_in: u64,
    ) -> Result<u64> {
        ExactInput::get_exact_input(
            ctx,
            dest_chain_id,
            amount_in
        )
    }

}



