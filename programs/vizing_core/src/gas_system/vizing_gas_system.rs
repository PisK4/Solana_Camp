use anchor_lang::prelude::*;

use crate::library::*;
use crate::governance::*;
use crate::library::ErrorCode;

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

//24 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone ,InitSpace)]
pub struct NativeTokenTradeFeeConfig {
    pub key: u64,
    pub molecular: u64,
    pub denominator: u64,
}


#[account]
#[derive(InitSpace)]
pub struct VizingGasSystem {
    #[max_len(20)]
    pub gas_system_global_mappings: Vec<GasSystemGlobal>,
    #[max_len(20)]
    pub fee_config_mappings: Vec<FeeConfig>,
    #[max_len(20)]
    pub trade_fee_mappings: Vec<TradeFee>,
    #[max_len(20)]
    pub trade_fee_config_mappings: Vec<TradeFeeAndDappConfig>,
    #[max_len(20)]
    pub native_token_trade_fee_config_mappings: Vec<NativeTokenTradeFeeConfig>,
}

impl VizingGasSystem {
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

//init fee_config and gas_system_global
impl InitFeeConfig<'_>{
    pub fn gas_system_init(
        ctx: &mut Context<InitFeeConfig>,
        params: InitGasSystemParams
    ) -> Result<()> {
        require!(params.denominator>0 ,ErrorCode::ZeroNumber);
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        vizing_gas_system.set_fee_config(
            params.chain_id,
            params.base_price,
            0,
            params.molecular,
            params.denominator,
            params.molecular_decimal,
            params.denominator_decimal,
        );

        vizing_gas_system.set_gas_system_global(
            params.chain_id,
            params.global_base_price,
            params.default_gas_limit,
            params.amount_in_threshold,
            params.global_molecular,
            params.global_denominator,
        );
        
        Ok(())
    }

    // pub fn initialize_fee_config(
    //     ctx: Context<InitFeeConfig>,
    //     key: u64,
    //     base_price: u64,
    //     reserve: u64,
    //     molecular: u64,
    //     denominator: u64,
    //     molecular_decimal: u8,
    //     denominator_decimal: u8,
    // ) -> Result<()> {
    //     let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
    //     vizing_gas_system.set_fee_config(
    //         key,
    //         base_price,
    //         reserve,
    //         molecular,
    //         denominator,
    //         molecular_decimal,
    //         denominator_decimal,
    //     );
    //     Ok(())
    // }
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
        require!(denominator>0,ErrorCode::ZeroNumber);
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        vizing_gas_system.set_gas_system_global(
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
        require!(denominator>0,ErrorCode::ZeroNumber);
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        vizing_gas_system.set_fee_config(
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
        require!(denominator>0,ErrorCode::ZeroNumber);
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let gas_system_global = vizing_gas_system.get_gas_system_global(key);
        vizing_gas_system.set_gas_system_global(
            key,
            gas_system_global.global_base_price,
            gas_system_global.default_gas_limit,
            gas_system_global.amount_in_threshold,
            molecular,
            denominator
        );
        vizing_gas_system.set_trade_fee(key, molecular, denominator);
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
        require!(denominator>0,ErrorCode::ZeroNumber);
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        vizing_gas_system.set_trade_fee_and_dapp_config(chain_id, dapp, molecular, denominator, base_price);
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
        require!(denominator>0,ErrorCode::ZeroNumber);
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        let mut fee_config = vizing_gas_system.get_fee_config(chain_id);

        fee_config.molecular = molecular;
        fee_config.denominator = denominator;
        fee_config.molecular_decimal = molecular_decimal;
        fee_config.denominator_decimal = denominator_decimal;

        vizing_gas_system.set_fee_config(
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
        for &denominator in &denominators {
            require!(denominator > 0, ErrorCode::ZeroNumber);
        }
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;

        for (i, &current_id) in dest_chain_ids.iter().enumerate() {
            vizing_gas_system.set_trade_fee(current_id, moleculars[i], denominators[i]);
            vizing_gas_system.set_native_token_trade_fee_config(current_id, moleculars[i], denominators[i]);
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
        for &denominator in &denominators {
            require!(denominator > 0, ErrorCode::ZeroNumber);
        }

        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;

        for (i, &current_id) in dest_chain_ids.iter().enumerate() {
            vizing_gas_system.set_native_token_trade_fee_config(current_id, moleculars[i], denominators[i]);
            vizing_gas_system.set_trade_fee_and_dapp_config(current_id, dapps[i], moleculars[i], denominators[i], values[i])
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
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        for (i, &current_id) in chain_ids.iter().enumerate() {
            let get_trade_fee_config = vizing_gas_system.get_trade_fee_config(current_id, dapps[i]);
            vizing_gas_system.set_trade_fee_and_dapp_config(current_id, dapps[i],get_trade_fee_config.molecular,get_trade_fee_config.denominator, base_prices[i]);
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
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        for (i, _) in base_prices.iter().enumerate() {
            let get_trade_fee_config = vizing_gas_system.get_trade_fee_config(chain_id, dapps[i]);
            vizing_gas_system.set_trade_fee_and_dapp_config(chain_id, dapps[i],get_trade_fee_config.molecular,get_trade_fee_config.denominator, base_prices[i]);
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
        for &denominator in &denominators {
            require!(denominator > 0, ErrorCode::ZeroNumber);
        }
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        for (i, &current_id) in chain_ids.iter().enumerate() {
            let fee_config = vizing_gas_system.get_fee_config(current_id);
            let this_base_price = fee_config.base_price;
            let this_reserve = fee_config.reserve;
            vizing_gas_system.set_fee_config(
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
        let vizing_gas_system = &mut ctx.accounts.vizing_gas_system;
        vizing_gas_system.remove_trade_fee_config_dapp(
            key,
            dapp
        );
        Ok(())
    }
}

    pub fn compute_trade_fee1(
        trade_fee_molecular: u64,
        trade_fee_denominator: u64,
        gas_system_global_molecular: u64,
        gas_system_global_denominator: u64,
        _dest_chain_id: u64,
        amount_out: Uint256,
    ) -> Option<Uint256> {
        let fee;
        if trade_fee_denominator == 0 {
            if gas_system_global_molecular!=0 && gas_system_global_denominator!=0 {
                let new_gas_system_global_molecular= Uint256::new(0,gas_system_global_molecular as u128);
                let new_gas_system_global_denominator= Uint256::new(0,gas_system_global_denominator as u128);
                fee = amount_out.check_mul(new_gas_system_global_molecular)?.check_div(new_gas_system_global_denominator)?;
            }else{
                fee=Uint256::new(0,0);
            }
        } else {
            if trade_fee_molecular!=0 && trade_fee_denominator!=0 {
                let new_trade_fee_molecular= Uint256::new(0,trade_fee_molecular as u128);
                let new_trade_fee_denominator= Uint256::new(0,trade_fee_denominator as u128);
                fee = amount_out.check_mul(new_trade_fee_molecular)?.check_div(new_trade_fee_denominator)?;
            }else{
                fee=Uint256::new(0,0);
            }
            
        }
        Some(fee)
    }

    pub fn compute_trade_fee2(
        trade_fee_molecular: u64,
        trade_fee_denominator: u64,
        trade_fee_config_molecular: u64,
        trade_fee_config_denominator: u64,
        gas_system_global_molecular: u64,
        gas_system_global_denominator: u64,
        target_contract: [u8; 32],
        dest_chain_id: u64,
        amount_out: Uint256,
    ) -> Option<Uint256> {
        let fee;
        let zero_contract: [u8; 32] = [0; 32];
        if target_contract != zero_contract{
            if trade_fee_config_denominator != 0 {
                let new_trade_fee_config_molecular = Uint256::new(0,trade_fee_config_molecular as u128);
                let new_trade_fee_config_denominator = Uint256::new(0,trade_fee_config_denominator as u128);
                fee = amount_out.check_mul(new_trade_fee_config_molecular)?.check_div(new_trade_fee_config_denominator)?;
            }else{
                fee = compute_trade_fee1(
                    trade_fee_molecular,
                    trade_fee_denominator,
                    gas_system_global_molecular,
                    gas_system_global_denominator,
                    dest_chain_id,
                    amount_out
                )?;
            }
        }else{
            fee = compute_trade_fee1(
                trade_fee_molecular,
                trade_fee_denominator,
                gas_system_global_molecular,
                gas_system_global_denominator,
                dest_chain_id,
                amount_out
            )?;
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
        fee_config_denominator: u64,
        trade_fee_molecular: u64,
        trade_fee_denominator: u64,
        trade_fee_config_molecular: u64,
        trade_fee_config_denominator: u64,
        gas_system_global_molecular: u64,
        gas_system_global_denominator: u64,
        gas_system_global_default_gas_limit: u64,
        amount_out: Uint256,
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
        
        msg!("fee: {:?}",fee);
        let new_fee=Uint256::new(0,fee as u128);
        let mut final_fee=new_fee.clone();
        if amount_out.is_zero()==false {
            let mut output_amount_in=Uint256::new(0,0);
            if fee_config_molecular != 0 {
                output_amount_in=exact_output(
                    fee_config_molecular,
                    fee_config_denominator,
                    fee_config_molecular_decimal,
                    fee_config_denominator_decimal,
                    dest_chain_id,
                    amount_out
                )?;
                msg!("exact_output high: {:?}",output_amount_in.high);
                msg!("exact_output low: {:?}",output_amount_in.low);
            }

            let trade_fee2=compute_trade_fee2(
                trade_fee_molecular,
                trade_fee_denominator,
                trade_fee_config_molecular,
                trade_fee_config_denominator,
                gas_system_global_molecular,
                gas_system_global_denominator,
                this_dapp,
                dest_chain_id,
                output_amount_in
            )?;
            msg!("compute_trade_fee2 high: {:?}",trade_fee2.high);
            msg!("compute_trade_fee2 low: {:?}",trade_fee2.low);

            final_fee = new_fee.check_add(trade_fee2)?;

        }
        let estimate_fee=final_fee.low.try_into().unwrap();
        Some(estimate_fee)
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
        fee_config_base_price: u64,
        gas_system_global_base_price: u64,
        dapp_config_value: u64,
        _target_contract: [u8; 32],
        _dest_chain_id: u64,
    ) -> Option<u64> {
        let dapp_base_price: u64;
        if dapp_config_value > 0 {
            dapp_base_price = dapp_config_value;
        } else {
            if fee_config_base_price > 0{
                dapp_base_price = fee_config_base_price;
            }else {
                dapp_base_price = gas_system_global_base_price;
            }
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
        trade_fee_molecular: u64,
        trade_fee_denominator: u64,
        trade_fee_config_molecular: u64,
        trade_fee_config_denominator: u64,
        gas_system_global_molecular: u64,
        gas_system_global_denominator: u64,
        dapp_config_value: u64,
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        fee_config_molecular: u64,
        fee_config_denominator: u64,
        gas_system_global_default_gas_limit: u64,
        gas_system_global_global_base_price: u64,
        fee_config_base_price: u64,
        dest_chain_id: u64,
        amount_out: Uint256,
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
                msg!("price < dapp_base_price");
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

        msg!("fee: {:?}",fee);
        let mut amount_in = Uint256::new(amount_out.high,amount_out.low);
        let new_fee = Uint256::new(0,fee as u128);
        let mut final_fee = new_fee.clone();
        if amount_out.is_zero()==false {
            if fee_config_molecular != 0 {
                amount_in = exact_output(
                    fee_config_molecular,
                    fee_config_denominator,
                    fee_config_molecular_decimal,
                    fee_config_denominator_decimal,
                    dest_chain_id,
                    amount_out
                )?;
                msg!("exact_output high: {:?}",amount_in.high);
                msg!("exact_output low: {:?}",amount_in.low);
            }
            let trade_fee2 = compute_trade_fee2(
                trade_fee_molecular,
                trade_fee_denominator,
                trade_fee_config_molecular,
                trade_fee_config_denominator,
                gas_system_global_molecular,
                gas_system_global_denominator,
                this_dapp,
                dest_chain_id,
                amount_in
            )?;
            msg!("compute_trade_fee2 high: {:?}",trade_fee2.high);
            msg!("compute_trade_fee2 low: {:?}",trade_fee2.low);
            final_fee = trade_fee2.check_add(amount_in)?.check_add(new_fee)?;
        }
        let limit_amount=Uint256::new(0,token_amount_limit as u128);
        if Uint256::cmp(amount_in,limit_amount)>=1 {
            msg!("Exceed quantity limit");
            return None;
        }
        let estimate_total_fee=final_fee.low.try_into().unwrap();

        Some(estimate_total_fee)
    }

    pub fn exact_output(
        fee_config_molecular: u64,
        fee_config_denominator: u64,
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        _dest_chain_id: u64,
        amount_out: Uint256,    
    ) -> Option<Uint256> {
        let this_amount_out;
        let amount_in;
        msg!("molecular_decimal: {:?}", fee_config_molecular_decimal);
        msg!("denominator_decimal: {:?}", fee_config_denominator_decimal);

        let decimal_diff = (fee_config_molecular_decimal as i32 - fee_config_denominator_decimal as i32).abs();
        if decimal_diff > 18 {
            msg!("Decimal difference is too large, exceeding 18");
            return None;  
        }
        
        if fee_config_molecular_decimal!=0 && fee_config_denominator_decimal!=0{
            let new_fee_config_molecular=Uint256::new(0,fee_config_molecular as u128);
            let new_fee_config_denominator=Uint256::new(0,fee_config_denominator as u128);
            if fee_config_molecular_decimal != fee_config_denominator_decimal {
                if fee_config_molecular_decimal > fee_config_denominator_decimal {
                    let power_value=Uint256::new(0,10u128
                        .pow((fee_config_molecular_decimal-fee_config_denominator_decimal) as u32));
                    this_amount_out=amount_out.check_div(power_value)?;
                } else {
                    let power_value=Uint256::new(0,10u128
                        .pow((fee_config_denominator_decimal-fee_config_molecular_decimal) as u32));
                    this_amount_out=amount_out.check_mul(power_value)?;
                }
            } else {
                this_amount_out=amount_out;
            }
            let new_amount_out=this_amount_out.check_mul(new_fee_config_molecular)?;
            amount_in = new_amount_out.check_div(new_fee_config_denominator)?;
        }else{
            msg!("molecular_decimal or denominator_decimal is 0");
            return None; 
        }
        Some(amount_in)
    }

    pub fn exact_input(
        fee_config_molecular: u64,
        fee_config_denominator: u64,
        fee_config_molecular_decimal: u8,
        fee_config_denominator_decimal: u8,
        _dest_chain_id: u64,
        amount_in: u64,
    ) -> Option<Uint256> {
        let new_amount_in = Uint256::new(0, amount_in as u128);
        let this_amount_in;
        let amount_out;
        msg!("molecular_decimal: {:?}", fee_config_molecular_decimal);
        msg!("denominator_decimal: {:?}", fee_config_denominator_decimal);

        let decimal_diff = (fee_config_molecular_decimal as i32 - fee_config_denominator_decimal as i32).abs();
        if decimal_diff > 18 {
            msg!("Decimal difference is too large, exceeding 18");
            return None;  
        }
        if fee_config_molecular_decimal!=0 && fee_config_denominator_decimal!=0{
            let new_fee_config_molecular=Uint256::new(0,fee_config_molecular as u128);
            let new_fee_config_denominator=Uint256::new(0,fee_config_denominator as u128);

            if fee_config_molecular_decimal != fee_config_denominator_decimal {
                if fee_config_molecular_decimal > fee_config_denominator_decimal {
                    let power_value=Uint256::new(0,10u128
                        .pow((fee_config_molecular_decimal-fee_config_denominator_decimal) as u32));
                    this_amount_in=new_amount_in.check_mul(power_value)?;
                } else {
                    let power_value=Uint256::new(0,10u128
                        .pow((fee_config_denominator_decimal-fee_config_molecular_decimal) as u32));
                    this_amount_in=new_amount_in.check_div(power_value)?;
                }
            } else {
                this_amount_in=new_amount_in;
            }
            let this_mul_amount_in = this_amount_in.check_mul(new_fee_config_denominator)?;
            amount_out = this_mul_amount_in.check_div(new_fee_config_molecular)?;
        }else{
            msg!("molecular_decimal or denominator_decimal is 0");
            return None; 
        }
        Some(amount_out)
    }


//init
#[account]
#[derive(InitSpace)]
pub struct InitGasSystemParams {
    chain_id: u64,
    base_price: u64,
    molecular: u64,
    denominator: u64,
    molecular_decimal: u8,
    denominator_decimal: u8,
    global_base_price: u64,
    default_gas_limit: u64,
    amount_in_threshold: u64,
    global_molecular: u64,
    global_denominator: u64,
}

#[derive(Accounts)]
pub struct InitFeeConfig<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        init,
        payer = payer, 
        space = 8 + VizingGasSystem::INIT_SPACE,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct SetGasGlobal<'info> {
    #[account(seeds = [contants::VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetFeeConfig<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetTokenFeeConfig<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetTokenFeeConfig<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin
        )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetTradeFeeConfigMap<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin
    )]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetDappPriceConfig<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetDappPriceConfigInDiffChain<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetDappPriceConfigInSameChain<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetExchangeRate<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.swap_manager == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BatchSetExchangeRate<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.swap_manager == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct RemoveTradeFeeConfigDapp<'info> {
    #[account(seeds = [VIZING_PAD_CONFIG_SEED], bump = vizing_pad_config.bump
        , constraint = vizing_pad_config.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing_pad_config: Account<'info, VizingPadConfigs>,
    #[account(
        mut,
        seeds = [VIZING_GAS_SYSTEM_SEED, vizing_pad_config.key().as_ref()],
        bump
    )]
    pub vizing_gas_system: Account<'info, VizingGasSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}