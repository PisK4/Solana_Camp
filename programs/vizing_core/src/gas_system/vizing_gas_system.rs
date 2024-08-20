use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::message_type_lib::*;
use crate::state::*;
use crate::message_monitor_lib::*;

declare_id!("Ga4UfvXHBB4V1FgA5bvvrHT4gg7rraGLG1vshzxndW4i");

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
    pub dapp: [u16; 20], //address
    pub molecular: u64,
    pub denominator: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct DappConfig {
    pub key: u64,
    pub dapp: [u16; 20], //address
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
    pub valid: bool,
}

#[account]
pub struct MappingNativeTokenTradeFeeConfig {
    pub native_token_trade_fee_config_mappings: Vec<NativeTokenTradeFeeConfig>,
    pub valid: bool,
}

#[account]
pub struct MappingAmountInThresholds {
    pub amount_in_thresholds_mappings: Vec<AmountInThresholds>,
    pub valid: bool,
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
        self.valid = true;
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
        dapp: [u16; 20],
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

    pub fn set_dapp_config(&mut self, key: u64, dapp: [u16; 20], value: u64) {
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

    pub fn mapping_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    pub fn get_fee_config(&self, key: u64) -> Option<FeeConfig> {
        // require!(Some(self.valid),ErrorCode::InvalidMapping);
        self.fee_config_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }

    pub fn get_trade_fee(&self, key: u64) -> Option<TradeFee> {
        // require!(Some(self.valid),ErrorCode::InvalidMapping);
        self.trade_fee_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }

    pub fn get_trade_fee_config(&self, key: u64, dapp: [u16; 20]) -> Option<TradeFeeConfig> {
        // require!(Some(self.valid),ErrorCode::InvalidMapping);
        self.trade_fee_config_mappings
            .iter()
            .find(|pair| pair.key == key && pair.dapp == dapp)
            .cloned()
    }

    pub fn get_dapp_config(&mut self, key: u64, dapp: [u16; 20]) -> Option<DappConfig> {
        // require!(Some(self.valid),ErrorCode::InvalidMapping);
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
        self.valid = true;
    }

    pub fn mapping_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    pub fn get_native_token_trade_fee_config(
        &mut self,
        key: u64,
    ) -> Option<NativeTokenTradeFeeConfig> {
        // require!(Some(self.valid),ErrorCode::InvalidMapping);
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

    pub fn mapping_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    pub fn get_amount_in_thresholds(&mut self, key: u64) -> Option<u64> {
        self.amount_in_thresholds_mappings
            .iter()
            .find(|pair| pair.key == key)
            .map(|pair| pair.value)
    }
}

mod vizing_gas_system {
    use super::*;

    pub fn save_chain_id(
        ctx: Context<SaveDestChainId>,
        dest_chain_id: Vec<u8>,
    ) -> Result<()>{
        let save=&mut ctx.accounts.save_chain_id;
        save.dest_chain_id=dest_chain_id;
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

        let g = &mut ctx.accounts.global_trade_fee;
        g.molecular = molecular;
        g.denominator = denominator;
        Ok(())
    }

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
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);

        let gas_system_global = &mut ctx.accounts.gas_system_global;
        gas_system_global.global_base_price = global_base_price;
        gas_system_global.default_gas_limit = default_gas_limit;
        gas_system_global.amount_in_threshold = amount_in_threshold;
        let g = &mut ctx.accounts.global_trade_fee;
        g.molecular = molecular;
        g.denominator = denominator;
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

    pub fn init_native_token_trade_fee_config(
        ctx: Context<InitNativeTokenTradeFeeConfig>,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
        let n = &mut ctx.accounts.native_token_trade_fee_config;
        n.set_native_token_trade_fee_config(0, molecular, denominator);
        Ok(())
    }

    pub fn set_token_fee_config(
        ctx: Context<SetTokenFeeConfig>,
        key: u64,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);

        let g = &mut ctx.accounts.global_trade_fee;
        let n = &mut ctx.accounts.native_token_trade_fee_config;
        g.molecular = molecular;
        g.denominator = denominator;
        n.set_native_token_trade_fee_config(key, molecular, denominator);
        Ok(())
    }

    pub fn batch_set_token_fee_config(
        ctx: Context<BatchSetTokenFeeConfig>,
        dest_chain_id: Vec<u64>,
        molecular: Vec<u64>,
        denominator: Vec<u64>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);
        require!(
            dest_chain_id.len() == molecular.len() && dest_chain_id.len() == denominator.len(),
            ErrorCode::InvalidLength
        );
        let m = &mut ctx.accounts.mapping_fee_config;
        let n = &mut ctx.accounts.native_token_trade_fee_config;

        for (i, &current_id) in dest_chain_id.iter().enumerate() {
            n.set_native_token_trade_fee_config(current_id, molecular[i], denominator[i]);
            m.set_trade_fee(dest_chain_id[i], molecular[i], denominator[i])
        }
        Ok(())
    }

    pub fn batch_set_trade_fee_config_map(
        ctx: Context<BatchSetTradeFeeConfigMap>,
        dapps: Vec<[u16; 20]>,
        dest_chain_id: Vec<u64>,
        molecular: Vec<u64>,
        denominator: Vec<u64>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);
        require!(
            dest_chain_id.len() == molecular.len()
                && dest_chain_id.len() == denominator.len()
                && dest_chain_id.len() == dapps.len(),
            ErrorCode::InvalidLength
        );

        let m = &mut ctx.accounts.mapping_fee_config;
        let n = &mut ctx.accounts.native_token_trade_fee_config;

        for (i, &current_id) in dest_chain_id.iter().enumerate() {
            n.set_native_token_trade_fee_config(current_id, molecular[i], denominator[i]);
            m.set_trade_fee_config(current_id, dapps[i], molecular[i], denominator[i])
        }
        Ok(())
    }

    pub fn batch_set_amount_in_threshold(
        ctx: Context<BatchSetAmountInThreshold>,
        chain_ids: Vec<u64>,
        new_values: Vec<u64>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);
        require!(chain_ids.len() == new_values.len(), ErrorCode::InvalidLength);
        let a = &mut ctx.accounts.amount_in_thresholds;
        for (i, &current_id) in chain_ids.iter().enumerate() {
            a.set_amount_in_thresholds(current_id, new_values[i]);
        }

        Ok(())
    }

    pub fn set_dapp_price_config(
        ctx: Context<SetDappPriceConfig>,
        chain_id: u64,
        dapp: [u16; 20],
        base_price: u64,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);
        let m = &mut ctx.accounts.mapping_fee_config;
        m.set_dapp_config(chain_id, dapp, base_price);

        Ok(())
    }

    pub fn batch_set_dapp_price_config_in_diff_chain(
        ctx: Context<BatchSetDappPriceConfigInDiffChain>,
        chain_id: Vec<u64>,
        dapps: Vec<[u16; 20]>,
        base_prices: Vec<u64>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);
        require!(
            chain_id.len() == dapps.len() && chain_id.len() == base_prices.len(),
            ErrorCode::InvalidLength
        );
        let m = &mut ctx.accounts.mapping_fee_config;
        for (i, &current_id) in chain_id.iter().enumerate() {
            m.set_dapp_config(current_id, dapps[i], base_prices[i]);
        }

        Ok(())
    }

    pub fn batch_set_dapp_price_config_in_same_chain(
        ctx: Context<BatchSetDAppPriceConfigInSameChain>,
        chain_id: u64,
        dapps: Vec<[u16; 20]>,
        base_prices: Vec<u64>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.gas_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonGasManager);
        require!(dapps.len() == base_prices.len(), ErrorCode::InvalidLength);
        let m = &mut ctx.accounts.mapping_fee_config;
        for (i, &price) in base_prices.iter().enumerate() {
            m.set_dapp_config(chain_id, dapps[i], base_prices[i]);
        }
        Ok(())
    }

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
        let if_power_user = power_user.swap_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonSwapManager);
        let m = &mut ctx.accounts.mapping_fee_config;

        let fee_config = m
            .get_fee_config(chain_id)
            .ok_or(ErrorCode::FeeConfigNotFound)?;
        let this_base_price = fee_config.base_price;
        let this_reserve = fee_config.reserve;

        m.set_fee_config(
            chain_id,
            this_base_price,
            this_reserve,
            molecular,
            denominator,
            molecular_decimal,
            denominator_decimal,
        );
        Ok(())
    }

    pub fn batch_set_exchange_rate(
        ctx: Context<BatchSetExchangeRate>,
        chain_id: Vec<u64>,
        molecular: Vec<u64>,
        denominator: Vec<u64>,
        molecular_decimal: Vec<u8>,
        denominator_decimal: Vec<u8>,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key();
        let if_power_user = power_user.swap_manager.contains(user_key);
        require!(if_power_user, ErrorCode::NonSwapManager);
        require!(
            chain_id.len() == molecular.len()
                && chain_id.len() == denominator.len()
                && chain_id.len() == molecular_decimal.len()
                && chain_id.len() == denominator_decimal.len(),
            ErrorCode::InvalidLength
        );
        let m = &mut ctx.accounts.mapping_fee_config;
        for (i, &current_id) in chain_id.iter().enumerate() {
            let fee_config = m
                .get_fee_config(current_id)
                .ok_or(ErrorCode::FeeConfigNotFound)?;
            let this_base_price = fee_config.base_price;
            let this_reserve = fee_config.reserve;
            m.set_fee_config(
                current_id,
                this_base_price,
                this_reserve,
                molecular[i],
                denominator[i],
                molecular_decimal[i],
                denominator_decimal[i],
            );
        }

        Ok(())
    }

    pub fn compute_trade_fee1(
        ctx: Context<ComputeTradeFee1>,
        dest_chain_id: u64,
        amount_out: u64,
    ) -> Option<u64> {
        let global_trade_fee = &mut ctx.accounts.global_trade_fee;
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let fee;
        if let Some(fee_config) = mapping_fee_config.get_trade_fee(dest_chain_id) {
            if fee_config.denominator == 0 {
                fee = amount_out
                    .checked_mul(global_trade_fee.molecular)?
                    .checked_div(global_trade_fee.denominator)?;
                Some(fee)
            } else {
                fee = amount_out
                    .checked_mul(fee_config.molecular)?
                    .checked_div(fee_config.denominator)?;
                Some(fee)
            }
        } else {
            None
        }
    }

    pub fn compute_trade_fee2(
        t_molecular: u64,
        t_denominator: u64,
        g_molecular: u64,
        g_denominator: u64,
        // target_contract: [u16; 20],
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
        message: &[u16]
    ) -> Option<u64> {
        let mut base_price: u64;
        let mut fee: u64;
        let mut this_price: u64=0;
        let mut this_dapp: [u16; 20]=[0; 20];
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
            
            fee=(gas_limit as u64)*this_price;

        }else if(mode==MessageType::NativeTokenSend){
            let (Some((_, gas_limit))) = message_monitor::slice_transfer(message) else { todo!() };
            fee=(gas_limit as u64)*this_price;
        }else{
            fee=base_price*default_gas_limit;
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
                final_fee = fee + trade_fee2;
            } else {
                return None; 
            }
        }
        Some(final_fee)
    }


    pub fn get_dapp_base_price(
        dapp_config_value: u64,
        dest_chain_id: u64,
        chain_base_price: u64,
        dapp: [u16; 20],
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
        ctx: Context<EstimatePrice1>,
        target_contract: [u16; 20],
        dest_chain_id: u64,
    ) -> Option<u64> {
        let m = &mut ctx.accounts.mapping_fee_config;
        let g = &mut ctx.accounts.gas_system_global;
        let dapp_config = m.get_dapp_config(dest_chain_id, target_contract)?;
        let dapp_base_price: u64;
        if (dapp_config.value > 0) {
            dapp_base_price = dapp_config.value;
        } else {
            dapp_base_price = g.global_base_price;
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
        messages: &[&[u16]],
    ) -> Option<u64> {
        let m = &mut ctx.accounts.mapping_fee_config;
        let gas_system_global = &mut ctx.accounts.gas_system_global;
        let global_trade_fee = &mut ctx.accounts.global_trade_fee;
        let a = &mut ctx.accounts.amount_in_thresholds;
        let mut total_trade_fee=0; 

        for (i, &amount) in amount_outs.iter().enumerate() {
            let t=m.get_trade_fee(dest_chain_ids[i])?;
            let this_message: [u16; 20] = match messages[i].try_into() {
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
            total_trade_fee=current_fee+total_trade_fee;
        }
        Some(total_trade_fee)

    }

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
        message: &[u16],
    ) -> Option<u64> {
        let base_price: u64;
        if (fee_config_base_price > 0) {
            base_price = fee_config_base_price;
        } else {
            base_price = g_global_base_price;
        }
        let this_dapp: [u16; 20];
        let fee: u64;
        let mode = MessageType::fetch_msg_mode(&message);

        if (mode == MessageType::StandardActivate || mode == MessageType::ArbitraryActivate) {
            let (Some((dapp, gas_limit, price, _)))=message_monitor::slice_message(message) else { todo!() };

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
            fee=(gas_limit as u64)*price;
        }else if (mode == MessageType::NativeTokenSend) {
            let (Some((_, gas_limit))) = message_monitor::slice_transfer(message) else { todo!() };
            fee=(gas_limit as u64)*base_price;
        }else{
            fee=base_price*g_default_gas_limit;
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
            final_fee = trade_fee2 + amount_in + fee;
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
            if (fee_config_molecular_decimal != fee_config_denominator_decimal) {
                if (fee_config_molecular_decimal > fee_config_denominator_decimal) {
                    amount_out / 10u64
                        .pow((fee_config_molecular_decimal - fee_config_denominator_decimal) as u32)
                } else {
                    amount_out * 10u64
                        .pow((fee_config_denominator_decimal - fee_config_molecular_decimal) as u32)
                }
            } else {
                amount_out
            };
        let amount_In = amount_out * (fee_config_molecular_decimal as u64)
            / (fee_config_denominator_decimal as u64);
        Some(amount_In)
    }

    pub fn exact_input(
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        dest_chain_id: u64,
        amount_in: u64,
    ) -> Option<u64> {
        let this_amount_in =
            if fee_config_molecular_decimal != fee_config_denominator_decimal {
                if fee_config_molecular_decimal > fee_config_denominator_decimal {
                    amount_in * 10u64
                        .pow((fee_config_molecular_decimal - fee_config_denominator_decimal) as u32)
                } else {
                    amount_in / 10u64
                        .pow((fee_config_denominator_decimal - fee_config_molecular_decimal) as u32)
                }
            } else {
                amount_in
            };
        let amount_out = this_amount_in * (fee_config_molecular_decimal as u64)
            / (fee_config_denominator_decimal as u64);
        Some(amount_out)
    }

}

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
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
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
pub struct SetFeeConfig<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitNativeTokenTradeFeeConfig<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        init,
        payer = user, 
        space = 8 + 32,
    )]
    pub native_token_trade_fee_config: Account<'info, MappingNativeTokenTradeFeeConfig>,
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
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
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
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        mut,
        seeds = [b"fee_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
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
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
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
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(
        init,
        payer = user, 
        space = 8 + 32,
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
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
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
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetDAppPriceConfigInSameChain<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
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
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
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
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//get
#[derive(Accounts)]
pub struct ComputeTradeFee1<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        mut,
        seeds = [b"global_trade_fee".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub global_trade_fee: Account<'info, GlobalTradeFee>,
    #[account(mut)]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
}

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

#[derive(Accounts)]
pub struct EstimatePrice1<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(mut)]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(
        mut,
        seeds = [b"gas_global".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub gas_system_global: Account<'info, GasSystemGlobal>,
}

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

// #[derive(Accounts)]
// pub struct EstimateGas<'info> {
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
//     #[account(
//         mut,
//         seeds = [b"global_trade_fee".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
//         bump
//     )]
//     pub global_trade_fee: Account<'info, GlobalTradeFee>,
// }

#[derive(Accounts)]
pub struct BatchEstimateTotalFee<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(mut)]
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
    #[account(mut)]
    pub amount_in_thresholds: Account<'info, MappingAmountInThresholds>,
}

// #[derive(Accounts)]
// pub struct EstimateTotalFee<'info> {
//     #[account(mut)]
//     pub mapping_fee_config: Account<'info, MappingFeeConfig>,
//     #[account(
//         mut,
//         seeds = [b"gas_global".as_ref()],
//         bump
//     )]
//     pub gas_system_global: Account<'info, GasSystemGlobal>,
//     #[account(
//         mut,
//         seeds = [b"global_trade_fee".as_ref()],
//         bump
//     )]
//     pub global_trade_fee: Account<'info, GlobalTradeFee>,
// }

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