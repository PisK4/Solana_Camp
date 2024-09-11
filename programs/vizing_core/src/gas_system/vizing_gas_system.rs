use anchor_lang::prelude::*;

use crate::library::*;
use crate::governance::*;

//48 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone ,InitSpace)]
pub struct GasSystemGlobal {
    pub key: u64,
    pub global_base_price: u64,
    pub default_gas_limit: u64,
    pub amount_in_threshold: u64,
    pub molecular: u64,
    pub denominator: u64,
}

//24 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone ,InitSpace)]
pub struct NativeTokenTradeFeeConfig {
    pub key: u64,
    pub molecular: u64,
    pub denominator: u64,
}

//42 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone ,InitSpace)]
pub struct FeeConfig {
    pub key: u64,
    pub base_price: u64,
    pub reserve: u64,
    pub molecular: u64,
    pub denominator: u64,
    pub molecular_decimal: u8,
    pub denominator_decimal: u8,
}

//24 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct TradeFee {
    pub key: u64,
    pub molecular: u64,
    pub denominator: u64,
}

//352 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct TradeFeeAndDappConfig {
    pub key: u64,
    #[max_len(10)]
    pub dapps: Vec<[u8; 32]>, //address group
    pub molecular: u64,
    pub denominator: u64,
    pub value: u64,
}


#[account]
#[derive(InitSpace)]
pub struct MappingFeeConfig {
    #[max_len(1)]
    pub gas_system_global_mappings: Vec<GasSystemGlobal>,
    #[max_len(20)]
    pub fee_config_mappings: Vec<FeeConfig>,
    #[max_len(20)]
    pub trade_fee_mappings: Vec<TradeFee>,
    #[max_len(20)]
    pub trade_fee_config_mappings: Vec<TradeFeeAndDappConfig>,
    #[max_len(20)]
    pub native_token_trade_fee_config_mappings: Vec<NativeTokenTradeFeeConfig>,

    // pub bump: u8,
}

impl MappingFeeConfig {
    //gas_system_global
    pub fn set_gas_system_global(
        &mut self,
        key: u64,
        global_base_price: u64,
        default_gas_limit: u64,
        amount_in_threshold: u64,
        molecular: u64,
        denominator: u64,
    ) {
        if let Some(pair) = self
            .gas_system_global_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.global_base_price = global_base_price;
            pair.default_gas_limit = default_gas_limit;
            pair.amount_in_threshold = amount_in_threshold;
            pair.molecular = molecular;
            pair.denominator = denominator;
        } else {
            self.gas_system_global_mappings.push(GasSystemGlobal {
                key,
                global_base_price,
                default_gas_limit,
                amount_in_threshold,
                molecular,
                denominator
            });
        }
    }

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

    pub fn set_trade_fee_and_dapp_config(
        &mut self,
        key: u64,
        dapp: [u8; 32],
        molecular: u64,
        denominator: u64,
        value: u64
    ) {
        if let Some(pair) = self
            .trade_fee_config_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            if !pair.dapps.contains(&dapp) {
                pair.dapps.push(dapp);
            }
            pair.molecular = molecular;
            pair.denominator = denominator;
            pair.value = value;
        } else {
            self.trade_fee_config_mappings.push(TradeFeeAndDappConfig {
                key,
                dapps: vec![dapp],
                molecular,
                denominator,
                value
            });
        }
    }

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

    pub fn remove_trade_fee_config_dapp(&mut self, key: u64, dapp: [u8; 32]) {
        if let Some(pair) = self.trade_fee_config_mappings.iter_mut().find(|pair| pair.key == key) {
            pair.dapps.retain(|item| *item != dapp);
        } else {
            panic!("TradeFeeConfig key not found");
        }
    }

    pub fn get_gas_system_global(
        &mut self,
        key: u64,
    ) -> GasSystemGlobal {
        if let Some(pair) = self.gas_system_global_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
        {
            pair
        } else {
            GasSystemGlobal {
                key: 0,
                global_base_price: 0,
                default_gas_limit: 0,
                amount_in_threshold: 0,
                molecular: 0,
                denominator: 0,
            }
        }
        
    }

    pub fn get_fee_config(&self, key: u64) -> FeeConfig {
        if let Some(pair) = self.fee_config_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
        {
            pair 
        } else {
            FeeConfig {
                key: 0,
                base_price: 0,
                reserve: 0,
                molecular: 0,
                denominator: 0,
                molecular_decimal: 0,
                denominator_decimal: 0,
            }
        }
    }

    pub fn get_trade_fee(&self, key: u64) -> TradeFee {
        if let Some(pair) = self.trade_fee_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
        {
            pair
        }
        else{
            TradeFee {
                key: 0,
                molecular: 0,
                denominator: 0
            }
        }
    }

    pub fn get_trade_fee_config(&self, key: u64, dapp: [u8; 32]) -> TradeFeeAndDappConfig {
        if let Some(pair) = self.trade_fee_config_mappings
            .iter()
            .find(|pair| pair.key == key && pair.dapps.iter().any(|&stored_dapp| stored_dapp == dapp))
            .cloned()
        {
            pair
        }
        else{
            TradeFeeAndDappConfig {
                key: 0,
                dapps: Vec::new(),
                molecular: 0,
                denominator: 0,
                value: 0
            }
        }
    }

    pub fn get_native_token_trade_fee_config(
        &mut self,
        key: u64,
    ) -> NativeTokenTradeFeeConfig {
        if let Some(pair) = self.native_token_trade_fee_config_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
        {
            pair
        }
        else{
            NativeTokenTradeFeeConfig {
                key: 0,
                molecular: 0,
                denominator: 0
            }
        }
    }
}

impl InitFeeConfig<'_>{
    pub fn initialize_fee_config(
        ctx: Context<InitFeeConfig>,
        key: u64,
        base_price: u64,
        reserve: u64,
        molecular: u64,
        denominator: u64,
        molecular_decimal: u8,
        denominator_decimal: u8,
    ) -> Result<()> {
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


//set
impl SetGasGlobal<'_>{
    pub fn set_gas_global(
        ctx: Context<SetGasGlobal>,
        key: u64,
        global_base_price: u64,
        default_gas_limit: u64,
        amount_in_threshold: u64,
        molecular: u64,
        denominator: u64,
    ) -> Result<()> {
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        mapping_fee_config.set_gas_system_global(
            key,
            global_base_price,
            default_gas_limit,
            amount_in_threshold,
            molecular,
            denominator
        );
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
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let gas_system_global = mapping_fee_config.get_gas_system_global(key);
        mapping_fee_config.set_gas_system_global(
            key,
            gas_system_global.global_base_price,
            gas_system_global.default_gas_limit,
            gas_system_global.amount_in_threshold,
            molecular,
            denominator
        );
        
        mapping_fee_config.set_native_token_trade_fee_config(key, molecular, denominator);
        Ok(())
    }
}
    
impl SetDappPriceConfig<'_>{
    pub fn set_dapp_price_config(
        ctx: Context<SetDappPriceConfig>,
        chain_id: u64,
        dapp: [u8; 32],
        molecular: u64,
        denominator: u64,
        base_price: u64,
    ) -> Result<()> {
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        mapping_fee_config.set_trade_fee_and_dapp_config(chain_id, dapp, molecular, denominator, base_price);
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
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        let mut fee_config = mapping_fee_config.get_fee_config(chain_id);

        fee_config.molecular = molecular;
        fee_config.denominator = denominator;
        fee_config.molecular_decimal = molecular_decimal;
        fee_config.denominator_decimal = denominator_decimal;

        mapping_fee_config.set_fee_config(
            chain_id,
            fee_config.base_price,
            fee_config.reserve,
            fee_config.molecular,
            fee_config.denominator,
            fee_config.molecular_decimal,
            fee_config.denominator_decimal,
        );
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
        require!(
            dest_chain_ids.len() == moleculars.len() && dest_chain_ids.len() == denominators.len(),
            errors::ErrorCode::InvalidLength
        );
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;

        for (i, &current_id) in dest_chain_ids.iter().enumerate() {
            mapping_fee_config.set_native_token_trade_fee_config(current_id, moleculars[i], denominators[i]);
            mapping_fee_config.set_trade_fee(current_id, moleculars[i], denominators[i])
        }
        Ok(())
    }
}

impl BatchSetTradeFeeConfigMap<'_>{
    pub fn batch_set_trade_fee_and_dapp_config_map(
        ctx: Context<BatchSetTradeFeeConfigMap>,
        dapps: Vec<[u8; 32]>,
        dest_chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
        values: Vec<u64>
    ) -> Result<()> {
        require!(
            dest_chain_ids.len() == moleculars.len()
                && dest_chain_ids.len() == denominators.len()
                && dest_chain_ids.len() == dapps.len(),
            errors::ErrorCode::InvalidLength
        );

        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;

        for (i, &current_id) in dest_chain_ids.iter().enumerate() {
            mapping_fee_config.set_native_token_trade_fee_config(current_id, moleculars[i], denominators[i]);
            mapping_fee_config.set_trade_fee_and_dapp_config(current_id, dapps[i], moleculars[i], denominators[i], values[i])
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
        require!(
            chain_ids.len() == dapps.len() && chain_ids.len() == base_prices.len(),
            errors::ErrorCode::InvalidLength
        );
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        for (i, &current_id) in chain_ids.iter().enumerate() {
            let get_trade_fee_config = mapping_fee_config.get_trade_fee_config(current_id, dapps[i]);
            mapping_fee_config.set_trade_fee_and_dapp_config(current_id, dapps[i],get_trade_fee_config.molecular,get_trade_fee_config.denominator, base_prices[i]);
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
        require!(dapps.len() == base_prices.len(), errors::ErrorCode::InvalidLength);
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        for (i, _) in base_prices.iter().enumerate() {
            let get_trade_fee_config = mapping_fee_config.get_trade_fee_config(chain_id, dapps[i]);
            mapping_fee_config.set_trade_fee_and_dapp_config(chain_id, dapps[i],get_trade_fee_config.molecular,get_trade_fee_config.denominator, base_prices[i]);
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
        require!(
            chain_ids.len() == moleculars.len()
                && chain_ids.len() == denominators.len()
                && chain_ids.len() == molecular_decimals.len()
                && chain_ids.len() == denominator_decimals.len(),
            errors::ErrorCode::InvalidLength
        );
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        for (i, &current_id) in chain_ids.iter().enumerate() {
            let fee_config = mapping_fee_config.get_fee_config(current_id);
            let this_base_price = fee_config.base_price;
            let this_reserve = fee_config.reserve;
            mapping_fee_config.set_fee_config(
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

impl RemoveTradeFeeConfigDapp<'_>{
    pub fn remove_this_trade_fee_config_dapp(
        ctx: Context<RemoveTradeFeeConfigDapp>,
        key: u64,
        dapp: [u8; 32]
    )-> Result<()> {
        let mapping_fee_config = &mut ctx.accounts.mapping_fee_config;
        mapping_fee_config.remove_trade_fee_config_dapp(
            key,
            dapp
        );
        Ok(())
    }
}

    pub fn compute_trade_fee1(
        fee_config_molecular: u64,
        fee_config_denominator: u64,
        gas_system_global_molecular: u64,
        gas_system_global_denominator: u64,
        _dest_chain_id: u64,
        amount_out: u64,
    ) -> Option<u64> {
        let fee;
        if fee_config_denominator == 0 {
            fee = amount_out
                .checked_mul(gas_system_global_molecular)?
                .checked_div(gas_system_global_denominator)?;
        } else {
            fee = amount_out
                .checked_mul(fee_config_molecular)?
                .checked_div(fee_config_denominator)?;
        }
        Some(fee)
    }

    pub fn compute_trade_fee2(
        trade_fee_config_molecular: u64,
        trade_fee_config_denominator: u64,
        gas_system_global_molecular: u64,
        gas_system_global_denominator: u64,
        _target_contract: [u8; 32],
        _dest_chain_id: u64,
        amount_out: u64,
    ) -> Option<u64> {
        let fee;
        if trade_fee_config_denominator > 0{
            fee = amount_out
                    .checked_mul(trade_fee_config_molecular)?
                    .checked_div(trade_fee_config_denominator)?;
        }else{
            fee = amount_out
                    .checked_mul(gas_system_global_molecular)?
                    .checked_div(gas_system_global_denominator)?;
        }
        Some(fee)
    }

    pub fn estimate_gas(
        gas_system_global_global_base_price: u64,
        fee_config_base_price: u64,
        dapp_config_value: u64,
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        fee_config_molecular: u64,
        trade_fee_config_molecular: u64,
        trade_fee_config_denominator: u64,
        gas_system_global_molecular: u64,
        gas_system_global_denominator: u64,
        gas_system_global_default_gas_limit: u64,
        amount_out: u64,
        dest_chain_id: u64,
        message: &[u8]
    ) -> Option<u64> {
        let base_price: u64;
        let fee: u64;
        let mut this_dapp: [u8; 32]=[0; 32];

        if fee_config_base_price > 0 {
            base_price=fee_config_base_price;
        }else{
            base_price=gas_system_global_global_base_price;
        }
        let mode = MessageType::fetch_msg_mode(&message);

        if mode==MessageType::StandardActivate || mode==MessageType::ArbitraryActivate {
            let Some((_, dapp, gas_limit, price, _))=message_monitor::slice_message(message) else { todo!() };
            let dapp_base_price = get_dapp_base_price(
                dapp_config_value,
                dest_chain_id,
                base_price,
                dapp
            )?;

            msg!("dapp: {:?}", dapp);
            msg!("gas_limit: {:?}", gas_limit);
            msg!("price: {:?}", price);

            this_dapp=dapp;
            let this_price: u64;
            if price<dapp_base_price{
                this_price=dapp_base_price;
            }else{
                this_price=price;
            }
            
            fee=(gas_limit as u64).checked_mul(this_price)?;

        }else if mode==MessageType::NativeTokenSend{
            let Some((_, gas_limit)) = message_monitor::slice_transfer(message) else { todo!() };
            fee=(gas_limit as u64).checked_mul(base_price)?;
        }else{
            fee=base_price.checked_mul(gas_system_global_default_gas_limit)?;
        }

        let mut final_fee: u64= fee;
        if amount_out > 0{
            let mut output_amount_in: u64 = 0;
            if fee_config_molecular != 0 {
                output_amount_in=exact_output(
                    fee_config_molecular_decimal,
                    fee_config_denominator_decimal,
                    dest_chain_id,
                    amount_out
                )?;
            }

            let trade_fee2=compute_trade_fee2(
                trade_fee_config_molecular,
                trade_fee_config_denominator,
                gas_system_global_molecular,
                gas_system_global_denominator,
                this_dapp,
                dest_chain_id,
                output_amount_in
            )?;

            final_fee = fee.checked_add(trade_fee2)?;

        }
        Some(final_fee)
    }

    fn get_dapp_base_price(
        dapp_config_value: u64,
        _dest_chain_id: u64,
        chain_base_price: u64,
        _dapp: [u8; 32],
    ) -> Option<u64> {
        let this_dapp_base_price: u64;
        if dapp_config_value > 0 {
            this_dapp_base_price = dapp_config_value;
        } else {
            this_dapp_base_price = chain_base_price;
        }
        Some(this_dapp_base_price)
    }

    pub fn estimate_price1(
        gas_system_global_base_price: u64,
        dapp_config_value: u64,
        _target_contract: [u8; 32],
        _dest_chain_id: u64,
    ) -> Option<u64> {
        let dapp_base_price: u64;
        if dapp_config_value > 0 {
            dapp_base_price = dapp_config_value;
        } else {
            dapp_base_price = gas_system_global_base_price;
        }
        Some(dapp_base_price)
    }

    pub fn estimate_price2(
        gas_system_global_base_price: u64,
        fee_config_base_price: u64,
        _dest_chain_id: u64
    ) -> Option<u64> {
        let base_price: u64;
        if fee_config_base_price > 0 {
            base_price = fee_config_base_price;
        } else {
            base_price = gas_system_global_base_price;
        }
        Some(base_price)
    }

    pub fn estimate_total_fee(
        token_amount_limit: u64,
        trade_fee_config_molecular: u64,
        trade_fee_config_denominator: u64,
        gas_system_global_molecular: u64,
        gas_system_global_denominator: u64,
        dapp_config_value: u64,
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        fee_config_molecular: u64,
        gas_system_global_default_gas_limit: u64,
        gas_system_global_global_base_price: u64,
        fee_config_base_price: u64,
        dest_chain_id: u64,
        amount_out: u64,
        message: &[u8],
    ) -> Option<u64> {
        let base_price: u64;
        if fee_config_base_price > 0 {
            base_price = fee_config_base_price;
        } else {
            base_price = gas_system_global_global_base_price;
        }
        let mut this_dapp: [u8; 32]=[0; 32];
        let fee: u64;
        let mode = MessageType::fetch_msg_mode(&message);

        if mode == MessageType::StandardActivate || mode == MessageType::ArbitraryActivate {
            let Some((_, dapp, gas_limit, price, _))=message_monitor::slice_message(message) else { todo!() };
            
            msg!("dapp: {:?}", dapp);
            msg!("gas_limit: {:?}", gas_limit);
            msg!("price: {:?}", price);

            let dapp_base_price = get_dapp_base_price(
                dapp_config_value,
                dest_chain_id,
                base_price,
                dapp
            )?;
            msg!("dapp_base_price: {:?}",dapp_base_price);

            if price < dapp_base_price {
                return None; 
            }
            this_dapp=dapp;
            fee=(gas_limit as u64).checked_mul(price)?;
        }else if mode == MessageType::NativeTokenSend {
            let Some((_, gas_limit)) = message_monitor::slice_transfer(message) else { todo!() };
            fee=(gas_limit as u64).checked_mul(base_price)?;
        }else{
            fee=base_price.checked_mul(gas_system_global_default_gas_limit)?;
        }

        let mut amount_in: u64=amount_out;
        let mut final_fee: u64=fee;
        if amount_out > 0 {
            if fee_config_molecular != 0 {
                amount_in = exact_output(
                    fee_config_molecular_decimal,
                    fee_config_denominator_decimal,
                    dest_chain_id,
                    amount_out
                )?;
                msg!("exact_output: {:?}",amount_in);
            }
            let trade_fee2 = compute_trade_fee2(
                trade_fee_config_molecular,
                trade_fee_config_denominator,
                gas_system_global_molecular,
                gas_system_global_denominator,
                this_dapp,
                dest_chain_id,
                amount_in
            )?;
            msg!("compute_trade_fee2: {:?}",trade_fee2);
            final_fee = trade_fee2.checked_add(amount_in)?.checked_add(fee)?;
        }
        if amount_in > token_amount_limit{
            return None;
        }

        Some(final_fee)
    }

    pub fn exact_output(
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        _dest_chain_id: u64,
        amount_out: u64,    
    ) -> Option<u64> {
        let this_amount_out;
            if fee_config_molecular_decimal != fee_config_denominator_decimal {
                if fee_config_molecular_decimal > fee_config_denominator_decimal {
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
        _dest_chain_id: u64,
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


//init
#[derive(Accounts)]
pub struct InitFeeConfig<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.owner == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        init,
        payer = user, 
        space = 8 + MappingFeeConfig::INIT_SPACE,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetGasGlobal<'info> {
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetFeeConfig<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetTokenFeeConfig<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetTokenFeeConfig<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        // , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin
        )]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetTradeFeeConfigMap<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        // , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin
    )]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetDappPriceConfig<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetDappPriceConfigInDiffChain<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetDappPriceConfigInSameChain<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetExchangeRate<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetExchangeRate<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct RemoveTradeFeeConfigDapp<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveDappConfigDapp<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [b"init_mapping_fee_config".as_ref()],
        bump
    )]
    pub mapping_fee_config: Account<'info, MappingFeeConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}