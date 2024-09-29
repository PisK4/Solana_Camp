pub mod gas_system;
pub mod governance;
pub mod library;
pub mod vizing_channel;
pub mod vizing_omni;
use anchor_lang::prelude::*;
use gas_system::*;
use governance::*;
use library::*;
use vizing_channel::*;
use vizing_omni::*;

declare_id!("vizngM8xTgmP15xuxpUZHbdec3LBG7bnTe9j1BtaqsE");

#[program]
pub mod vizing_core {

    use super::*;

    // **********  channel start ************

    pub fn launch(mut ctx: Context<LaunchOp>, params: LaunchParams) -> Result<VizingReceipt> {
        LaunchOp::vizing_launch(&mut ctx, params)
    }

    pub fn landing<'info>(
        mut ctx: Context<'_, '_, '_, 'info, LandingOp<'info>>,
        params: LandingParams,
    ) -> Result<()> {
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
        new_vizing_app_accounts: Vec<Pubkey>,
        new_sol_pda_receiver: Pubkey,
    ) -> Result<()> {
        VizingAppManagement::update_vizing_app_configs(
            &mut ctx,
            new_vizing_app_accounts,
            new_sol_pda_receiver,
        )
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

    pub fn initialize_gas_system(
        mut ctx: Context<InitFeeConfig>,
        params: InitGasSystemParams,
    ) -> Result<()> {
        InitFeeConfig::gas_system_init(&mut ctx, params)
    }

    pub fn modify_settings(
        mut ctx: Context<OwnerAuthorization>,
        params: OwnerManagementParams,
    ) -> Result<()> {
        OwnerAuthorization::owner_management(&mut ctx, &params)
    }

    pub fn pause_engine(mut ctx: Context<EngineAdminAuthorization>) -> Result<()> {
        EngineAdminAuthorization::pause_engine(&mut ctx)
    }

    pub fn unpause_engine(mut ctx: Context<EngineAdminAuthorization>) -> Result<()> {
        EngineAdminAuthorization::unpause_engine(&mut ctx)
    }

    pub fn grant_relayer(
        mut ctx: Context<GrantRelayer>,
        new_trusted_relayers: Vec<Pubkey>,
    ) -> Result<()> {
        GrantRelayer::grant_relayer(&mut ctx, new_trusted_relayers)
    }

    pub fn grant_fee_collector(
        mut ctx: Context<GasPoolAdminAuthorization>,
        fee_collector: Pubkey,
    ) -> Result<()> {
        GasPoolAdminAuthorization::grant_fee_collector(&mut ctx, fee_collector)
    }

    pub fn grant_swap_manager(
        mut ctx: Context<GasPoolAdminAuthorization>,
        swap_manager: Pubkey,
    ) -> Result<()> {
        GasPoolAdminAuthorization::grant_swap_manager(&mut ctx, swap_manager)
    }

    // ***********  governance end ************

    // **********  fee management start ************

    /// @notice owner initialize record_message for dev get data
    pub fn init_record_message(ctx: Context<InitCurrentRecordMessage>) -> Result<()> {
        InitCurrentRecordMessage::init_current_record_message(ctx)
    }

    //set
    //set_gas_global
    /*
    /// @notice gas pool admin set global params in GasSystemGlobal
    /// @param global_base_price global base price
    /// @param default_gas_limit  global default gas limit
    /// @param amount_in_threshold  global amountIn threshold
    /// @param molecular  global molecular
    /// @param denominator  global denominator
     */
    pub fn set_this_gas_global(
        ctx: Context<SetGasGlobal>,
        group_id: u64,
        key: u64,
        global_base_price: u64,
        default_gas_limit: u64,
        amount_in_threshold: u64,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
        SetGasGlobal::set_gas_global(
            ctx,
            group_id,
            key,
            global_base_price,
            default_gas_limit,
            amount_in_threshold,
            molecular,
            denominator,
        )
    }

    //set_fee_config
    /*
    /// @notice gas pool admin set fee config mapping in VizingGasSystem
    /// @param key evm chainId
    /// @param basePrice  base price
    /// @param reserve  reserve
    /// @param molecular  molecular
    /// @param denominator  denominator
    /// @param molecularDecimal  molecular decimal
    /// @param denominatorDecimal  denominator decimal
     */
    pub fn set_this_fee_config(
        ctx: Context<SetFeeConfig>,
        group_id: u64,
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
            group_id,
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
    /*
    /// @notice gas pool admin set fee for native token exchange mapping in VizingGasSystem
    /// @param key evm chainId
    /// @param molecular  evm dapp address
    /// @param denominator  base price
     */
    pub fn set_this_token_fee_config(
        ctx: Context<SetTokenFeeConfig>,
        group_id: u64,
        key: u64,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
        SetTokenFeeConfig::set_token_fee_config(ctx, group_id, key, molecular, denominator)
    }

    //set_dapp_price_config
    /*
    /// @notice gas pool admin set dapp price config in VizingGasSystem
    /// @param chainId evm chainId
    /// @param dapp  evm dapp address
    /// @param basePrice  base price
     */
    pub fn set_this_dapp_price_config(
        ctx: Context<SetDappPriceConfig>,
        group_id: u64,
        chain_id: u64,
        dapp: [u8; 32],
        molecular: u64,
        denominator: u64,
        base_price: u64,
    ) -> Result<()> {
        SetDappPriceConfig::set_dapp_price_config(
            ctx,
            group_id,
            chain_id,
            dapp,
            molecular,
            denominator,
            base_price,
        )
    }

    //set_exchange_rate
    /*
    /// @notice gas pool admin set exchange rate in VizingGasSystem
    /// @param chainId evm chainId
    /// @param molecular  molecular
    /// @param denominator  denominator
    /// @param molecularDecimal  molecular decimal
    /// @param denominatorDecimal  denominator decimal
     */
    pub fn set_this_exchange_rate(
        ctx: Context<SetExchangeRate>,
        group_id: u64,
        chain_id: u64,
        molecular: u64,
        denominator: u64,
        molecular_decimal: u8,
        denominator_decimal: u8,
    ) -> Result<()> {
        SetExchangeRate::set_exchange_rate(
            ctx,
            group_id,
            chain_id,
            molecular,
            denominator,
            molecular_decimal,
            denominator_decimal,
        )
    }

    //batch_set_token_fee_config
    /*
    /// @notice gas pool admin set trade fee config mapping in VizingGasSystem
    /// @param dapps multi dapp address
    /// @param destChainIds  multi evm dest chainId
    /// @param moleculars  molecular group
    /// @param denominators  denominator group
     */
    pub fn batch_set_this_token_fee_config(
        ctx: Context<BatchSetTokenFeeConfig>,
        group_id: u64,
        dest_chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    ) -> Result<()> {
        BatchSetTokenFeeConfig::batch_set_token_fee_config(
            ctx,
            group_id,
            dest_chain_ids,
            moleculars,
            denominators,
        )
    }

    //batch_set_trade_fee_config_map
    /*
    /// @notice gas pool admin set trade fee config mapping in VizingGasSystem
    /// @param dapps multi dapp address
    /// @param destChainIds  multi evm dest chainId
    /// @param moleculars  molecular group
    /// @param denominators  denominator group
     */
    pub fn batch_set_this_trade_fee_config_map(
        ctx: Context<BatchSetTradeFeeConfigMap>,
        group_id: u64,
        dapps: Vec<[u8; 32]>,
        dest_chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
        values: Vec<u64>,
    ) -> Result<()> {
        BatchSetTradeFeeConfigMap::batch_set_trade_fee_and_dapp_config_map(
            ctx,
            group_id,
            dapps,
            dest_chain_ids,
            moleculars,
            denominators,
            values,
        )
    }

    //batch_set_dapp_price_config_in_diff_chain
    /*
    /// @notice gas pool admin batch set dapp price config different chain in VizingGasSystem
    /// @param chainIds multi evm chainId
    /// @param dapps  multi dapp address
    /// @param basePrices  basePrice group
     */
    pub fn batch_set_this_dapp_price_config_in_diff_chain(
        ctx: Context<BatchSetDappPriceConfigInDiffChain>,
        group_id: u64,
        chain_ids: Vec<u64>,
        dapps: Vec<[u8; 32]>,
        base_prices: Vec<u64>,
    ) -> Result<()> {
        BatchSetDappPriceConfigInDiffChain::batch_set_dapp_price_config_in_diff_chain(
            ctx,
            group_id,
            chain_ids,
            dapps,
            base_prices,
        )
    }

    //batch_set_dapp_price_config_in_same_chain
    /*
    /// @notice gas pool admin batch set dapp price config same chain in VizingGasSystem
    /// @param chainId evm chainId
    /// @param dapps  multi dapp address
    /// @param basePrices  basePrice group
     */
    pub fn batch_set_this_dapp_price_config_in_same_chain(
        ctx: Context<BatchSetDappPriceConfigInSameChain>,
        group_id: u64,
        chain_id: u64,
        dapps: Vec<[u8; 32]>,
        base_prices: Vec<u64>,
    ) -> Result<()> {
        BatchSetDappPriceConfigInSameChain::batch_set_dapp_price_config_in_same_chain(
            ctx,
            group_id,
            chain_id,
            dapps,
            base_prices,
        )
    }

    //batch_set_exchange_rate
    /*
    /// @notice gas pool admin batch set exchange rate in VizingGasSystem
    /// @param chainIds multi evm chainId
    /// @param moleculars  molecular group
    /// @param denominators  denominator group
    /// @param molecularDecimals molecular decimals group
    /// @param denominatorDecimals  denominator decimals group
     */
    pub fn batch_set_this_exchange_rate(
        ctx: Context<BatchSetExchangeRate>,
        group_id: u64,
        chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
        molecular_decimals: Vec<u8>,
        denominator_decimals: Vec<u8>,
    ) -> Result<()> {
        BatchSetExchangeRate::batch_set_exchange_rate(
            ctx,
            group_id,
            chain_ids,
            moleculars,
            denominators,
            molecular_decimals,
            denominator_decimals,
        )
    }

    //remove
    pub fn remove_trade_fee_dapp(
        ctx: Context<RemoveTradeFeeConfigDapp>,
        group_id: u64,
        key: u64,
        dapp: [u8; 32],
    ) -> Result<()> {
        RemoveTradeFeeConfigDapp::remove_this_trade_fee_config_dapp(ctx, group_id, key, dapp)
    }

    //get
    /*
    /// @notice Calculate the fee for the native token transfer
    /// @param destChainid The chain id of the destination chain
    /// @param amountOut The value we spent in the source chain
     */
    pub fn compute_trade_fee1(
        ctx: Context<ComputeTradeFee1>,
        dest_chain_id: u64,
        amount_out: Uint256,
    ) -> Result<Uint256> {
        ComputeTradeFee1::get_compute_trade_fee1(ctx, dest_chain_id, amount_out)
    }

    /*
    /// @notice Calculate the fee for the native token transfer
    /// @param targetContract contract address in the destination chain
    /// @param destChainid The chain id of the destination chain
    /// @param amountOut The value we spent in the source chain
     */
    pub fn compute_trade_fee2(
        ctx: Context<ComputeTradeFee2>,
        target_contract: [u8; 32],
        dest_chain_id: u64,
        amount_out: Uint256,
    ) -> Result<Uint256> {
        ComputeTradeFee2::get_compute_trade_fee2(ctx, target_contract, dest_chain_id, amount_out)
    }

    /*
    /// @notice Estimate the gas price we need to encode in message
    /// @param targetContract evm address
    /// @param destChainid The chain id of the destination chain
     */
    pub fn estimate_price1(
        ctx: Context<EstimatePrice1>,
        target_contract: [u8; 32],
        dest_chain_id: u64,
    ) -> Result<u64> {
        EstimatePrice1::get_estimate_price1(ctx, target_contract, dest_chain_id)
    }

    /*
    /// @notice Estimate the gas price we need to encode in message
    /// @param destChainid The chain id of the destination chain
     */
    pub fn estimate_price2(ctx: Context<EstimatePrice2>, dest_chain_id: u64) -> Result<u64> {
        EstimatePrice2::get_estimate_price2(ctx, dest_chain_id)
    }

    /*
    /// @notice Estimate the gas fee we should pay to vizing
    /// @param amountOut amountOut in the destination chain
    /// @param destChainid The chain id of the destination chain
    /// @param message The message we want to send to the destination chain
     */
    pub fn estimate_gas(
        ctx: Context<EstimateGas>,
        amount_out: Uint256,
        dest_chain_id: u64,
        message: Message,
    ) -> Result<u64> {
        EstimateGas::get_estimate_gas(ctx, amount_out, dest_chain_id, message)
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
        amount_out: Uint256,
        message: Message,
    ) -> Result<u64> {
        EstimateTotalFee::get_estimate_total_fee(ctx, dest_chain_id, amount_out, message)
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
        amount_out: Uint256,
    ) -> Result<Uint256> {
        ExactOutput::get_exact_output(ctx, dest_chain_id, amount_out)
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
    ) -> Result<Uint256> {
        ExactInput::get_exact_input(ctx, dest_chain_id, amount_in)
    }

    /*
    /// @notice Estimate the gas price we need to encode in message
    /// @param value The native token that value target address will receive in the destination chain
    /// @param destChainid The chain id of the destination chain
    /// @param additionParams The addition params for the message
    ///        if not in expert mode, set to 0 (`new bytes(0)`)
    /// @param message The message we want to send to the destination chain
     */

    pub fn estimate_vizing_gas_fee1(
        ctx: Context<EstimateVizingGasFee1>,
        value: Uint256,
        dest_chain_id: u64,
        _addition_params: Vec<u8>,
        message: Vec<u8>,
    ) -> Result<u64> {
        EstimateVizingGasFee1::get_estimate_vizing_gas_fee(
            ctx,
            value,
            dest_chain_id,
            _addition_params,
            message,
        )
    }

    pub fn estimate_vizing_gas_fee2(
        ctx: Context<EstimateVizingGasFee2>,
        value: Uint256,
        dest_chain_id: u64,
        _addition_params: Vec<u8>,
        message: Message,
    ) -> Result<u64> {
        EstimateVizingGasFee2::get_estimate_vizing_gas_fee(
            ctx,
            value,
            dest_chain_id,
            _addition_params,
            message,
        )
    }

    // **************** fee management end ****************
}
