use anchor_lang::prelude::*;

// use crate::error::errors::ErrorCode;
// use crate::message_type_lib::*;
// use crate::message_monitor_lib::*;
// use crate::state::*;

use crate::library::*;
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct NativeTokenTradeFeeConfig {
    pub key: u64,
    pub molecular: u64,
    pub denominator: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct FeeConfig {
    pub key: u64,
    pub base_price: u64,
    pub reserve: u64,
    pub molecular: u64,
    pub denominator: u64,
    pub molecular_decimal: u8,
    pub denominator_decimal: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct TradeFee {
    pub key: u64,
    pub molecular: u64,
    pub denominator: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct TradeFeeConfig {
    pub key: u64,
    pub dapp: [u8; 32], //address
    pub molecular: u64,
    pub denominator: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct DappConfig {
    pub key: u64,
    pub dapp: [u8; 32], //address
    pub value: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AmountInThresholds {
    pub key: u64,
    pub value: u64,
}

#[account]
pub struct MappingFeeConfig {
    pub fee_config_mappings: Vec<FeeConfig>,
    pub trade_fee_mappings: Vec<TradeFee>,
    pub trade_fee_config_mappings: Vec<TradeFeeConfig>,
    pub dapp_config_mappings: Vec<DappConfig>,
}

#[account]
pub struct MappingNativeTokenTradeFeeConfig {
    pub native_token_trade_fee_config_mappings: Vec<NativeTokenTradeFeeConfig>,
}

#[account]
pub struct MappingAmountInThresholds {
    pub amount_in_thresholds_mappings: Vec<AmountInThresholds>,
}

impl MappingFeeConfig {
    //feeConfig
    pub fn set_fee_config(
        &mut self,
        key: u64,
        base_price: u64,
        reserve: u64,
        molecular: u64,
        denominator: u64,
        molecular_decimal: u8,
        denominator_decimal: u8,
    ) {
        if let Some(pair) = self
            .fee_config_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.base_price = base_price;
            pair.reserve = reserve;
            pair.molecular = molecular;
            pair.denominator = denominator;
            pair.molecular_decimal = molecular_decimal;
            pair.denominator_decimal = denominator_decimal;
        } else {
            self.fee_config_mappings.push(FeeConfig {
                key,
                base_price,
                reserve,
                molecular,
                denominator,
                molecular_decimal,
                denominator_decimal,
            });
        }
    }

    pub fn set_trade_fee(&mut self, key: u64, molecular: u64, denominator: u64) {
        if let Some(pair) = self
            .trade_fee_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.molecular = molecular;
            pair.denominator = denominator;
        } else {
            self.trade_fee_mappings.push(TradeFee {
                key,
                molecular,
                denominator,
            });
        }
    }

    pub fn set_trade_fee_config(
        &mut self,
        key: u64,
        dapp: [u8; 32],
        molecular: u64,
        denominator: u64,
    ) {
        if let Some(pair) = self
            .trade_fee_config_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.dapp = dapp;
            pair.molecular = molecular;
            pair.denominator = denominator;
        } else {
            self.trade_fee_config_mappings.push(TradeFeeConfig {
                key,
                dapp,
                molecular,
                denominator,
            });
        }
    }

    pub fn set_dapp_config(&mut self, key: u64, dapp: [u8; 32], value: u64) {
        if let Some(pair) = self
            .dapp_config_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.dapp = dapp;
            pair.value = value;
        } else {
            self.dapp_config_mappings
                .push(DappConfig { key, dapp, value });
        }
    }

    pub fn get_fee_config(&self, key: u64) -> Option<FeeConfig> {
        self.fee_config_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }

    pub fn get_trade_fee(&self, key: u64) -> Option<TradeFee> {
        self.trade_fee_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }

    pub fn get_trade_fee_config(&self, key: u64, dapp: [u8; 32]) -> Option<TradeFeeConfig> {
        self.trade_fee_config_mappings
            .iter()
            .find(|pair| pair.key == key && pair.dapp == dapp)
            .cloned()
    }

    pub fn get_dapp_config(&mut self, key: u64, dapp: [u8; 32]) -> Option<DappConfig> {
        self.dapp_config_mappings
            .iter()
            .find(|pair| pair.key == key && pair.dapp == dapp)
            .cloned()
    }
}

impl MappingNativeTokenTradeFeeConfig {
    pub fn set_native_token_trade_fee_config(
        &mut self,
        key: u64,
        molecular: u64,
        denominator: u64,
    ) {
        if let Some(pair) = self
            .native_token_trade_fee_config_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.molecular = molecular;
            pair.denominator = denominator;
        } else {
            self.native_token_trade_fee_config_mappings
                .push(NativeTokenTradeFeeConfig {
                    key,
                    molecular,
                    denominator,
                });
        }
    }

    pub fn get_native_token_trade_fee_config(
        &mut self,
        key: u64,
    ) -> Option<NativeTokenTradeFeeConfig> {
        self.native_token_trade_fee_config_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }
}

impl MappingAmountInThresholds {
    pub fn set_amount_in_thresholds(&mut self, key: u64, value: u64) {
        if let Some(pair) = self
            .amount_in_thresholds_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.value = value;
        } else {
            self.amount_in_thresholds_mappings
                .push(AmountInThresholds { key, value });
        }
    }

    pub fn get_amount_in_thresholds(&mut self, key: u64) -> Option<u64> {
        self.amount_in_thresholds_mappings
            .iter()
            .find(|pair| pair.key == key)
            .map(|pair| pair.value)
    }
}

impl SetGasGlobal<'_>{
    pub fn set_gas_global(
        ctx: Context<SetGasGlobal>,
        global_base_price: u64,
        default_gas_limit: u64,
        amount_in_threshold: u64,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_managers.contains(user_key);
        require!(if_power_user, errors::ErrorCode::NonGasManager);

        let gas_system_global = &mut ctx.accounts.gas_system_global;
        gas_system_global.global_base_price = global_base_price;
        gas_system_global.default_gas_limit = default_gas_limit;
        gas_system_global.amount_in_threshold = amount_in_threshold;
        let g = &mut ctx.accounts.global_trade_fee;
        g.molecular = molecular;
        g.denominator = denominator;
        Ok(())
    }
}
    
impl SetFeeConfig<'_>{
    pub fn set_fee_config(
        ctx: Context<SetFeeConfig>,
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
        require!(if_power_user, errors::ErrorCode::NonGasManager);

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

}
    
impl SetTokenFeeConfig<'_>{
    pub fn set_token_fee_config(
        ctx: Context<SetTokenFeeConfig>,
        key: u64,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_managers.contains(user_key);
        require!(if_power_user, errors::ErrorCode::NonGasManager);

        let g = &mut ctx.accounts.global_trade_fee;
        let n = &mut ctx.accounts.native_token_trade_fee_config;
        g.molecular = molecular;
        g.denominator = denominator;
        n.set_native_token_trade_fee_config(key, molecular, denominator);
        Ok(())
    }
}
    
impl SetDappPriceConfig<'_>{
    pub fn set_dapp_price_config(
        ctx: Context<SetDappPriceConfig>,
        chain_id: u64,
        dapp: [u8; 32],
        base_price: u64,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_managers.contains(user_key);
        require!(if_power_user, errors::ErrorCode::NonGasManager);
        let m = &mut ctx.accounts.mapping_fee_config;
        m.set_dapp_config(chain_id, dapp, base_price);
        Ok(())
    }
}
    
impl SetExchangeRate<'_>{
    pub fn set_exchange_rate(
        ctx: Context<SetExchangeRate>,
        chain_id: u64,
        molecular: u64,
        denominator: u64,
        molecular_decimal: u8,
        denominator_decimal: u8,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.swap_managers.contains(user_key);
        require!(if_power_user, errors::ErrorCode::NonSwapManager);
        let m = &mut ctx.accounts.mapping_fee_config;

        if let Some(mut fee_config) = m.get_fee_config(chain_id) {
            fee_config.molecular = molecular;
            fee_config.denominator = denominator;
            fee_config.molecular_decimal = molecular_decimal;
            fee_config.denominator_decimal = denominator_decimal;

            m.set_fee_config(
                chain_id,
                fee_config.base_price,
                fee_config.reserve,
                fee_config.molecular,
                fee_config.denominator,
                fee_config.molecular_decimal,
                fee_config.denominator_decimal,
            );
        } else {
            return err!(errors::ErrorCode::FeeConfigNotFound);
        }
        Ok(())
    }
} 
    
impl BatchSetTokenFeeConfig<'_>{
    pub fn batch_set_token_fee_config(
        ctx: Context<BatchSetTokenFeeConfig>,
        dest_chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_managers.contains(user_key);
        require!(if_power_user, errors::ErrorCode::NonGasManager);
        require!(
            dest_chain_ids.len() == moleculars.len() && dest_chain_ids.len() == denominators.len(),
            errors::ErrorCode::InvalidLength
        );
        let m = &mut ctx.accounts.mapping_fee_config;
        let n = &mut ctx.accounts.native_token_trade_fee_config;

        for (i, &current_id) in dest_chain_ids.iter().enumerate() {
            n.set_native_token_trade_fee_config(current_id, moleculars[i], denominators[i]);
            m.set_trade_fee(current_id, moleculars[i], denominators[i])
        }
        Ok(())
    }
}

impl BatchSetTradeFeeConfigMap<'_>{
    pub fn batch_set_trade_fee_config_map(
        ctx: Context<BatchSetTradeFeeConfigMap>,
        dapps: Vec<[u8; 32]>,
        dest_chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_managers.contains(user_key);
        require!(if_power_user, errors::ErrorCode::NonGasManager);
        require!(
            dest_chain_ids.len() == moleculars.len()
                && dest_chain_ids.len() == denominators.len()
                && dest_chain_ids.len() == dapps.len(),
            errors::ErrorCode::InvalidLength
        );

        let m = &mut ctx.accounts.mapping_fee_config;
        let n = &mut ctx.accounts.native_token_trade_fee_config;

        for (i, &current_id) in dest_chain_ids.iter().enumerate() {
            n.set_native_token_trade_fee_config(current_id, moleculars[i], denominators[i]);
            m.set_trade_fee_config(current_id, dapps[i], moleculars[i], denominators[i])
        }
        Ok(())
    }
}
    
impl BatchSetAmountInThreshold<'_>{
    pub fn batch_set_amount_in_threshold(
        ctx: Context<BatchSetAmountInThreshold>,
        chain_ids: Vec<u64>,
        new_values: Vec<u64>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_managers.contains(user_key);
        require!(if_power_user, errors::ErrorCode::NonGasManager);
        require!(chain_ids.len() == new_values.len(), errors::ErrorCode::InvalidLength);
        let a = &mut ctx.accounts.amount_in_thresholds;
        for (i, &current_id) in chain_ids.iter().enumerate() {
            a.set_amount_in_thresholds(current_id, new_values[i]);
        }
        Ok(())
    }
}
    
impl BatchSetDappPriceConfigInDiffChain<'_>{
    pub fn batch_set_dapp_price_config_in_diff_chain(
        ctx: Context<BatchSetDappPriceConfigInDiffChain>,
        chain_ids: Vec<u64>,
        dapps: Vec<[u8; 32]>,
        base_prices: Vec<u64>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_managers.contains(user_key);
        require!(if_power_user, errors::ErrorCode::NonGasManager);
        require!(
            chain_ids.len() == dapps.len() && chain_ids.len() == base_prices.len(),
            errors::ErrorCode::InvalidLength
        );
        let m = &mut ctx.accounts.mapping_fee_config;
        for (i, &current_id) in chain_ids.iter().enumerate() {
            m.set_dapp_config(current_id, dapps[i], base_prices[i]);
        }

        Ok(())
    }
}
    
impl BatchSetDappPriceConfigInSameChain<'_>{
    pub fn batch_set_dapp_price_config_in_same_chain(
        ctx: Context<BatchSetDappPriceConfigInSameChain>,
        chain_id: u64,
        dapps: Vec<[u8; 32]>,
        base_prices: Vec<u64>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_managers.contains(user_key);
        require!(if_power_user, errors::ErrorCode::NonGasManager);
        require!(dapps.len() == base_prices.len(), errors::ErrorCode::InvalidLength);
        let m = &mut ctx.accounts.mapping_fee_config;
        for (i, &price) in base_prices.iter().enumerate() {
            m.set_dapp_config(chain_id, dapps[i], base_prices[i]);
        }
        Ok(())
    }
}
    
impl BatchSetExchangeRate<'_>{
    pub fn batch_set_exchange_rate(
        ctx: Context<BatchSetExchangeRate>,
        chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
        molecular_decimals: Vec<u8>,
        denominator_decimals: Vec<u8>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.swap_managers.contains(user_key);
        require!(if_power_user, errors::ErrorCode::NonSwapManager);
        require!(
            chain_ids.len() == moleculars.len()
                && chain_ids.len() == denominators.len()
                && chain_ids.len() == molecular_decimals.len()
                && chain_ids.len() == denominator_decimals.len(),
            errors::ErrorCode::InvalidLength
        );
        let m = &mut ctx.accounts.mapping_fee_config;
        for (i, &current_id) in chain_ids.iter().enumerate() {
            let fee_config = m
                .get_fee_config(current_id)
                .ok_or(errors::ErrorCode::FeeConfigNotFound)?;
            let this_base_price = fee_config.base_price;
            let this_reserve = fee_config.reserve;
            m.set_fee_config(
                current_id,
                this_base_price,
                this_reserve,
                moleculars[i],
                denominators[i],
                molecular_decimals[i],
                denominator_decimals[i],
            );
        }
        Ok(())
    }
}
    
    pub fn compute_trade_fee1(
        fee_config_molecular: u64,
        fee_config_denominator: u64,
        global_trade_fee_molecular: u64,
        global_trade_fee_denominator: u64,
        dest_chain_id: u64,
        amount_out: u64,
    ) -> Option<u64> {
        let fee;
        if fee_config_denominator == 0 {
            fee = amount_out
                .checked_mul(global_trade_fee_molecular)?
                .checked_div(global_trade_fee_denominator)?;
        } else {
            fee = amount_out
                .checked_mul(fee_config_molecular)?
                .checked_div(fee_config_denominator)?;
        }
        Some(fee)
    }

    pub fn compute_trade_fee2(
        t_molecular: u64,
        t_denominator: u64,
        g_molecular: u64,
        g_denominator: u64,
        // target_contract: [u8; 32],
        // dest_chain_id: u64,
        amount_out: u64,
    ) -> Option<u64> {
        let fee;
        if(t_denominator>0){
            fee = amount_out
                    .checked_mul(t_molecular)?
                    .checked_div(t_denominator)?;
        }else{
            fee = amount_out
                    .checked_mul(g_molecular)?
                    .checked_div(g_denominator)?;
        }
        Some(fee)
    }

    pub fn estimate_gas(
        global_base_price: u64,
        fee_config_base_price: u64,
        dapp_config_value: u64,
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        trade_fee_config_molecular: u64,
        trade_fee_config_denominator: u64,
        global_trade_fee_molecular: u64,
        global_trade_fee_denominator: u64,
        default_gas_limit: u64,
        amount_out: u64,
        dest_chain_id: u64,
        message: &[u8]
    ) -> Option<u64> {
        let base_price: u64;
        let fee: u64;
        let mut this_price: u64=0;
        let mut this_dapp: [u8; 32]=[0; 32];
        if(fee_config_base_price>0){
            base_price=fee_config_base_price;
        }else{
            base_price=global_base_price;
        }
        let mode = MessageType::fetch_msg_mode(&message);

        if(mode==MessageType::StandardActivate || mode==MessageType::ArbitraryActivate){
            let (Some((dapp, gas_limit, price, _)))=message_monitor::slice_message(message) else { todo!() };
            
            let dapp_base_price = get_dapp_base_price(
                dapp_config_value,
                dest_chain_id,
                base_price,
                dapp
            )?;

            this_dapp=dapp;

            if(price<dapp_base_price){
                this_price=dapp_base_price;
            }else{
                this_price=price;
            }
            
            fee=(gas_limit as u64).checked_mul(this_price)?;

        }else if(mode==MessageType::NativeTokenSend){
            let (Some((_, gas_limit))) = message_monitor::slice_transfer(message) else { todo!() };
            fee=(gas_limit as u64).checked_mul(this_price)?;
        }else{
            fee=base_price.checked_mul(default_gas_limit)?;
        }

        let mut amount_in: u64=amount_out;
        let mut final_fee: u64= fee;
        if(amount_out>0){
            if(fee_config_molecular_decimal != 0){
                amount_in=exact_output(
                    fee_config_molecular_decimal,
                    fee_config_denominator_decimal,
                    dest_chain_id,
                    amount_out
                )?;
            }

            if let Some(trade_fee2) = compute_trade_fee2(
                trade_fee_config_molecular,
                trade_fee_config_denominator,
                global_trade_fee_molecular,
                global_trade_fee_denominator,
                amount_in
            ) {
                final_fee = fee.checked_add(trade_fee2)?;
            } else {
                return None; 
            }
        }
        Some(final_fee)
    }

    // pub fn estimate_gas(
    //     ctx: Context<EstimateGas>,
    //     amount_out: u64,
    //     dest_chain_id: u64,
    //     message: &[u8]
    // ) -> Option<u64> {
    //     let base_price: u64;
    //     let fee: u64;
    //     let mut this_price: u64=0;
    //     let mut this_dapp: [u8; 32]=[0; 32];
    //     let mapping_fee_config=&mut ctx.accounts.mapping_fee_config;
    //     let gas_system_global=&mut ctx.accounts.gas_system_global;
    //     let global_trade_fee=&mut ctx.accounts.global_trade_fee;

    //     let get_fee_config=mapping_fee_config.get_fee_config(dest_chain_id)?;
    //     let get_trade_fee = mapping_fee_config.get_trade_fee(dest_chain_id)?;
        
    //     let fee_config_base_price= get_fee_config.base_price;
    //     let global_base_price= gas_system_global.global_base_price;
    //     let default_gas_limit = gas_system_global.default_gas_limit;
    //     let fee_config_molecular_decimal = get_fee_config.molecular_decimal;
    //     let fee_config_denominator_decimal = get_fee_config.denominator_decimal;
    //     let global_trade_fee_molecular = global_trade_fee.molecular;
    //     let global_trade_fee_denominator = global_trade_fee.denominator;

    //     if(fee_config_base_price>0){
    //         base_price=fee_config_base_price;
    //     }else{
    //         base_price=global_base_price;
    //     }
    //     let mode = MessageType::fetch_msg_mode(&message);

    //     if(mode==MessageType::StandardActivate || mode==MessageType::ArbitraryActivate){
    //         let (Some((dapp, gas_limit, price, _)))=message_monitor::slice_message(message) else { todo!() };
    //         let get_dapp_config = mapping_fee_config.get_dapp_config(dest_chain_id,dapp)?;
    //         let dapp_config_value=get_dapp_config.value;
    //         let dapp_base_price = get_dapp_base_price(
    //             dapp_config_value,
    //             dest_chain_id,
    //             base_price,
    //             dapp
    //         )?;

    //         this_dapp=dapp;

    //         if(price<dapp_base_price){
    //             this_price=dapp_base_price;
    //         }else{
    //             this_price=price;
    //         }
            
    //         fee=(gas_limit as u64).checked_mul(this_price)?;

    //     }else if(mode==MessageType::NativeTokenSend){
    //         let (Some((_, gas_limit))) = message_monitor::slice_transfer(message) else { todo!() };
    //         fee=(gas_limit as u64).checked_mul(this_price)?;
    //     }else{
    //         fee=base_price.checked_mul(default_gas_limit)?;
    //     }

    //     let mut amount_in: u64= amount_out;
    //     let mut final_fee: u64= fee;
    //     if(amount_out>0){
    //         if(fee_config_molecular_decimal != 0){
    //             amount_in=exact_output(
    //                 fee_config_molecular_decimal,
    //                 fee_config_denominator_decimal,
    //                 dest_chain_id,
    //                 amount_out
    //             )?;
    //         }

    //         let get_trade_fee_config = mapping_fee_config.get_trade_fee_config(dest_chain_id, this_dapp)?;
    //         let trade_fee_config_molecular=  get_trade_fee_config.molecular;
    //         let trade_fee_config_denominator=get_trade_fee_config.denominator;

    //         if let Some(trade_fee2) = compute_trade_fee2(
    //             trade_fee_config_molecular,
    //             trade_fee_config_denominator,
    //             global_trade_fee_molecular,
    //             global_trade_fee_denominator,
    //             amount_in
    //         ) {
    //             final_fee = fee.checked_add(trade_fee2)?;
    //         } else {
    //             return None; 
    //         }
    //     }
    //     Some(final_fee)
    // }


    pub fn get_dapp_base_price(
        dapp_config_value: u64,
        dest_chain_id: u64,
        chain_base_price: u64,
        dapp: [u8; 32],
    ) -> Option<u64> {
        let this_dapp_base_price: u64;
        if (dapp_config_value > 0) {
            this_dapp_base_price = dapp_config_value;
        } else {
            this_dapp_base_price = chain_base_price;
        }
        Some(this_dapp_base_price)
    }

    pub fn estimate_price1(
        gas_system_global_base_price: u64,
        dapp_config_value: u64,
        target_contract: [u8; 32],
        dest_chain_id: u64,
    ) -> Option<u64> {
        let dapp_base_price: u64;
        if (dapp_config_value > 0) {
            dapp_base_price = dapp_config_value;
        } else {
            dapp_base_price = gas_system_global_base_price;
        }
        Some(dapp_base_price)
    }

    pub fn estimate_price2(
        gas_system_global_base_price: u64,
        fee_config_base_price: u64,
        dest_chain_id: u64
    ) -> Option<u64> {
        let base_price: u64;
        if (fee_config_base_price > 0) {
            base_price = fee_config_base_price;
        } else {
            base_price = gas_system_global_base_price;
        }
        Some(base_price)
    }

    pub fn batch_estimate_total_fee(
        ctx: Context<BatchEstimateTotalFee>,
        amount_outs: Vec<u64>,
        dest_chain_ids: Vec<u64>,
        messages: &[&[u8]],
    ) -> Option<u64> {
        let m = &mut ctx.accounts.mapping_fee_config;
        let gas_system_global = &mut ctx.accounts.gas_system_global;
        let global_trade_fee = &mut ctx.accounts.global_trade_fee;
        let a = &mut ctx.accounts.amount_in_thresholds;
        let mut total_trade_fee=0; 

        for (i, &amount) in amount_outs.iter().enumerate() {
            let t=m.get_trade_fee(dest_chain_ids[i])?;
            let this_message: [u8; 32] = match messages[i].try_into() {
                Ok(array) => array,
                Err(_) => return None, 
            };
            let d=m.get_dapp_config(dest_chain_ids[i],this_message)?;
            let fee_config = m.get_fee_config(dest_chain_ids[i])?;
            let amount_in_thresholds=a.get_amount_in_thresholds(dest_chain_ids[i])?;
            
            let current_fee=estimate_total_fee(
                amount_in_thresholds,
                t.molecular,
                t.denominator,
                global_trade_fee.molecular,
                global_trade_fee.denominator,
                d.value,
                fee_config.molecular_decimal,
                fee_config.denominator_decimal,
                    fee_config.molecular,
                    gas_system_global.default_gas_limit,
                    gas_system_global.global_base_price,
                    fee_config.base_price,
                    dest_chain_ids[i],
                    amount_outs[i],
                    messages[i]
            )?;
            total_trade_fee=current_fee.checked_add(total_trade_fee)?;
        }
        Some(total_trade_fee)

    }

    // pub fn estimate_total_fee(
    //     token_amount_limit: u64,
    //     t_molecular: u64,
    //     t_denominator: u64,
    //     g_molecular: u64,
    //     g_denominator: u64,
    //     dapp_config_value: u64,
    //     fee_config_molecular_decimal: u8,
    //     fee_config_denominator_decimal: u8,
    //     fee_config_molecular: u64,
    //     g_default_gas_limit: u64,
    //     g_global_base_price: u64,
    //     fee_config_base_price: u64,
    //     dest_chain_id: u64,
    //     amount_out: u64,
    //     message: &[u8],
    // ) -> Option<u64> {
    //     let base_price: u64;
    //     if (fee_config_base_price > 0) {
    //         base_price = fee_config_base_price;
    //     } else {
    //         base_price = g_global_base_price;
    //     }
    //     let this_dapp: [u8; 32];
    //     let fee: u64;
    //     let mode = MessageType::fetch_msg_mode(&message);

    //     if (mode == MessageType::StandardActivate || mode == MessageType::ArbitraryActivate) {
    //         let Some((dapp, gas_limit, price, _))=message_monitor::slice_message(message) else { todo!() };

    //         let dapp_base_price = get_dapp_base_price(
    //             dapp_config_value,
    //             dest_chain_id,
    //             base_price,
    //             dapp
    //         )?;

    //         if (price < dapp_base_price) {
    //             return None; 
    //         }
    //         this_dapp=dapp;
    //         fee=(gas_limit as u64).checked_mul(price)?;
    //     }else if (mode == MessageType::NativeTokenSend) {
    //         let Some((_, gas_limit)) = message_monitor::slice_transfer(message) else { todo!() };
    //         fee=(gas_limit as u64).checked_mul(base_price)?;
    //     }else{
    //         fee=base_price.checked_mul(g_default_gas_limit)?;
    //     }

    //     let mut amount_in: u64=amount_out;
    //     let mut final_fee: u64=fee;
    //     if (amount_out > 0) {
    //         if (fee_config_molecular != 0) {
    //             amount_in = exact_output(
    //                 fee_config_molecular_decimal,
    //                 fee_config_denominator_decimal,
    //                 dest_chain_id,
    //                 amount_out
    //             )?;
    //         }
    //         let trade_fee2 = compute_trade_fee2(
    //             t_molecular,
    //             t_denominator,
    //             g_molecular,
    //             g_denominator,
    //             amount_in
    //         )?;
    //         final_fee = trade_fee2.checked_add(amount_in)?.checked_add(fee)?;
    //     }
    //     if(amount_in>token_amount_limit){
    //         return None;
    //     }

    //     Some(final_fee)
    // }

    pub fn estimate_total_fee(
        token_amount_limit: u64,
        t_molecular: u64,
        t_denominator: u64,
        g_molecular: u64,
        g_denominator: u64,
        dapp_config_value: u64,
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        fee_config_molecular: u64,
        g_default_gas_limit: u64,
        g_global_base_price: u64,
        fee_config_base_price: u64,
        dest_chain_id: u64,
        amount_out: u64,
        message: &[u8],
    ) -> Option<u64> {
        let base_price: u64;
        if (fee_config_base_price > 0) {
            base_price = fee_config_base_price;
        } else {
            base_price = g_global_base_price;
        }
        let this_dapp: [u8; 32];
        let fee: u64;
        let mode = MessageType::fetch_msg_mode(&message);

        if (mode == MessageType::StandardActivate || mode == MessageType::ArbitraryActivate) {
            let Some((dapp, gas_limit, price, _))=message_monitor::slice_message(message) else { todo!() };

            let dapp_base_price = get_dapp_base_price(
                dapp_config_value,
                dest_chain_id,
                base_price,
                dapp
            )?;

            if (price < dapp_base_price) {
                return None; 
            }
            this_dapp=dapp;
            fee=(gas_limit as u64).checked_mul(price)?;
        }else if (mode == MessageType::NativeTokenSend) {
            let Some((_, gas_limit)) = message_monitor::slice_transfer(message) else { todo!() };
            fee=(gas_limit as u64).checked_mul(base_price)?;
        }else{
            fee=base_price.checked_mul(g_default_gas_limit)?;
        }

        let mut amount_in: u64=amount_out;
        let mut final_fee: u64=fee;
        if (amount_out > 0) {
            if (fee_config_molecular != 0) {
                amount_in = exact_output(
                    fee_config_molecular_decimal,
                    fee_config_denominator_decimal,
                    dest_chain_id,
                    amount_out
                )?;
            }
            let trade_fee2 = compute_trade_fee2(
                t_molecular,
                t_denominator,
                g_molecular,
                g_denominator,
                amount_in
            )?;
            final_fee = trade_fee2.checked_add(amount_in)?.checked_add(fee)?;
        }
        if(amount_in>token_amount_limit){
            return None;
        }

        Some(final_fee)
    }

    pub fn exact_output(
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        dest_chain_id: u64,
        amount_out: u64,    
    ) -> Option<u64> {
        let this_amount_out;
            if (fee_config_molecular_decimal != fee_config_denominator_decimal) {
                if (fee_config_molecular_decimal > fee_config_denominator_decimal) {
                    this_amount_out=amount_out.checked_div(10u64
                        .pow((fee_config_molecular_decimal.checked_sub(fee_config_denominator_decimal)?) as u32))?;
                } else {
                    this_amount_out=amount_out.checked_div(10u64
                        .pow((fee_config_denominator_decimal.checked_sub(fee_config_molecular_decimal)?) as u32))?;
                }
            } else {
                this_amount_out=amount_out
            };
        let amount_in = this_amount_out.checked_mul(fee_config_molecular_decimal as u64)?
            .checked_div(fee_config_denominator_decimal as u64)?;
        Some(amount_in)
    }

    pub fn exact_input(
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        dest_chain_id: u64,
        amount_in: u64,
    ) -> Option<u64> {
        let this_amount_in;
            if fee_config_molecular_decimal != fee_config_denominator_decimal {
                if fee_config_molecular_decimal > fee_config_denominator_decimal {
                    this_amount_in=amount_in.checked_mul(10u64
                        .pow((fee_config_molecular_decimal.checked_sub(fee_config_denominator_decimal)?) as u32))?;
                } else {
                    this_amount_in=amount_in.checked_div(10u64
                        .pow((fee_config_denominator_decimal.checked_sub(fee_config_molecular_decimal)?) as u32))?;
                }
            } else {
                this_amount_in=amount_in
            };
        let amount_out = this_amount_in.checked_mul(fee_config_molecular_decimal as u64)?
            .checked_div(fee_config_denominator_decimal as u64)?;
        Some(amount_out)
    }



#[derive(Accounts)]
pub struct SetGasGlobal<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"global_trade_fee".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub global_trade_fee: Account<'info, GlobalTradeFee>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetFeeConfig<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetTokenFeeConfig<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"global_trade_fee".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub global_trade_fee: Account<'info, GlobalTradeFee>,
    #[account(
        mut,
        seeds = [b"native_token_trade_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub native_token_trade_fee_config: Account<'info, MappingNativeTokenTradeFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetTokenFeeConfig<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"native_token_trade_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub native_token_trade_fee_config: Account<'info, MappingNativeTokenTradeFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetTradeFeeConfigMap<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"native_token_trade_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub native_token_trade_fee_config: Account<'info, MappingNativeTokenTradeFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetAmountInThreshold<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"amount_in_thresholds".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub amount_in_thresholds: Account<'info, MappingAmountInThresholds>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetDappPriceConfig<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetDappPriceConfigInDiffChain<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetDappPriceConfigInSameChain<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetExchangeRate<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetExchangeRate<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//get
// #[derive(Accounts)]
// pub struct ComputeTradeFee1<'info> {
//     #[account(mut)]
//     pub save_chain_id: Account<'info,SaveChainId>,
//     #[account(
//         mut,
//         seeds = [b"global_trade_fee".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
//         bump
//     )]
//     pub global_trade_fee: Account<'info, GlobalTradeFee>,
//     #[account(mut)]
//     pub mapping_fee_config: Account<'info, MappingFeeConfig>,
// }

// #[derive(Accounts)]
// pub struct GetDappBasePrice<'info> {
//     #[account(mut)]
//     pub save_chain_id: Account<'info,SaveChainId>,
//     #[account(mut)]
//     pub mapping_fee_config: Account<'info, MappingFeeConfig>,
//     #[account(
//         mut,
//         seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
//         bump
//     )]
//     pub gas_system_global: Account<'info, GasSystemGlobal>,
// }

// #[derive(Accounts)]
// pub struct EstimatePrice1<'info> {
//     #[account(mut)]
//     pub save_chain_id: Account<'info,SaveChainId>,
//     #[account(mut)]
//     pub mapping_fee_config: Account<'info, MappingFeeConfig>,
//     #[account(
//         mut,
//         seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
//         bump
//     )]
//     pub gas_system_global: Account<'info, GasSystemGlobal>,
// }

// #[derive(Accounts)]
// pub struct EstimatePrice2<'info> {
//     #[account(mut)]
//     pub save_chain_id: Account<'info,SaveChainId>,
//     #[account(mut)]
//     pub mapping_fee_config: Account<'info, MappingFeeConfig>,
//     #[account(
//         mut,
//         seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
//         bump
//     )]
//     pub gas_system_global: Account<'info, GasSystemGlobal>,
// }

#[derive(Accounts)]
pub struct EstimateGas<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
    #[account(
        mut,
        seeds = [b"global_trade_fee".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub global_trade_fee: Account<'info, GlobalTradeFee>,
}

#[derive(Accounts)]
pub struct BatchEstimateTotalFee<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
    #[account(
        mut,
        seeds = [b"global_trade_fee".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub global_trade_fee: Account<'info, GlobalTradeFee>,
    #[account(
        mut,
        seeds = [b"amount_in_thresholds".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub amount_in_thresholds: Account<'info, MappingAmountInThresholds>,
}

#[derive(Accounts)]
pub struct EstimateTotalFee<'info> {
    #[account(mut)]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
    #[account(
        mut,
        seeds = [b"global_trade_fee".as_ref()],
        bump
    )]
    pub global_trade_fee: Account<'info, GlobalTradeFee>,
}

// #[derive(Accounts)]
// pub struct ExactOutput<'info> {
//     #[account(mut)]
//     pub mapping_fee_config: Account<'info, MappingFeeConfig>,
// }

// #[derive(Accounts)]
// pub struct ExactInput<'info> {
//     #[account(mut)]
//     pub mapping_fee_config: Account<'info, MappingFeeConfig>,
// }

