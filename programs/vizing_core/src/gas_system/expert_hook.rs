use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer as SolTransfer};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use spl_associated_token_account::get_associated_token_address;

use crate::error::ErrorCode;
use crate::message_monitor_lib::*;
use crate::message_type_lib::*;
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct TokenBase {
    pub key: [u16; 20],
    pub symbol: Vec<u8>,
    pub decimals: u8,
    pub max_price: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct TokenTradeFeeConfig {
    pub key1: [u16; 20],
    pub key2: u64,
    pub molecular: u64,
    pub denominator: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct SymbolConfig {
    pub key: Vec<u8>,
    pub address: [u16; 20],
}

#[account]
pub struct MappingTokenConfig {
    pub token_base_mappings: Vec<TokenBase>,
    pub token_trade_fee_config_mappings: Vec<TokenTradeFeeConfig>,
    pub valid: bool,
}

#[account]
pub struct MappingSymbolConfig {
    pub symbol_config_mappings: Vec<SymbolConfig>,
    pub valid: bool,
}

impl MappingTokenConfig {
    pub fn set_token_base(
        &mut self,
        key: [u16; 20],
        symbol: Vec<u8>,
        decimals: u8,
        max_price: u64,
    ) {
        if let Some(pair) = self
            .token_base_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.symbol = symbol;
            pair.decimals = decimals;
            pair.max_price = max_price;
        } else {
            self.token_base_mappings.push(TokenBase {
                key,
                symbol,
                decimals,
                max_price,
            });
        }
        self.valid = true;
    }

    pub fn set_token_trade_fee_config(
        &mut self,
        key1: [u16; 20],
        key2: u64,
        molecular: u64,
        denominator: u64,
    ) {
        if let Some(pair) = self
            .token_trade_fee_config_mappings
            .iter_mut()
            .find(|pair| pair.key1 == key1)
        {
            pair.key2 = key2;
            pair.molecular = molecular;
            pair.denominator = denominator;
        } else {
            self.token_trade_fee_config_mappings
                .push(TokenTradeFeeConfig {
                    key1,
                    key2,
                    molecular,
                    denominator,
                });
        }
        self.valid = true;
    }

    pub fn mapping_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    pub fn get_token_base(&self, key: [u16; 20]) -> Option<TokenBase> {
        self.token_base_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }

    pub fn get_token_trade_fee_config(
        &self,
        key1: [u16; 20],
        key2: u64,
    ) -> Option<TokenTradeFeeConfig> {
        self.token_trade_fee_config_mappings
            .iter()
            .find(|pair| pair.key1 == key1 && pair.key2 == key2)
            .cloned()
    }
}

impl MappingSymbolConfig {
    pub fn set(&mut self, key: Vec<u8>, address: [u16; 20]) {
        if let Some(pair) = self
            .symbol_config_mappings
            .iter_mut()
            .find(|pair| pair.key == key)
        {
            pair.address = address;
        } else {
            self.symbol_config_mappings
                .push(SymbolConfig { key, address });
        }
        self.valid = true;
    }

    pub fn mapping_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    pub fn get(&self, key: Vec<u8>) -> Option<SymbolConfig> {
        self.symbol_config_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }
}

impl ChangePowerUser<'_>{
    pub fn change_power_user(
        ctx: Context<ChangePowerUser>,
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
}

impl WithdrawSplToken<'_>{
    pub fn withdraw_spl_token(
        ctx: Context<WithdrawSplToken>,
        withdraw_amount: u64,
        this_bump: u8,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = ctx.accounts.user.key();
        require!(user_key == power_user.admin_role, ErrorCode::NonAdmin);
        let save_chain_id = &mut ctx.accounts.save_chain_id;
        let dest_chain_id = &save_chain_id.dest_chain_id;

        let seeds = &[
            b"vizing_vault".as_ref(),
            dest_chain_id.as_ref(),
            &[this_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.source.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.contract_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, withdraw_amount)?;

        Ok(())
    }
}

impl WithdrawSol<'_>{
    pub fn withdraw_sol(ctx: Context<WithdrawSol>, withdraw_amount: u64) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = ctx.accounts.user.key();
        require!(user_key == power_user.admin_role, ErrorCode::NonAdmin);

        let source = ctx.accounts.source.to_account_info();
        let destination = ctx.accounts.destination.to_account_info();

        **source.try_borrow_mut_lamports()? -= withdraw_amount;
        **destination.try_borrow_mut_lamports()? += withdraw_amount;
        Ok(())
    }
}

impl SetTokenInfoBase<'_>{
    pub fn set_token_info_base(
        ctx: Context<SetTokenInfoBase>,
        symbol: Vec<u8>,
        token_address: [u16; 20],
        decimals: u8,
        max_price: u64,
    ) -> Result<()> {
        let token_config = &mut ctx.accounts.token_config;
        let symbol_config = &mut ctx.accounts.symbol_config;
        let symbol_clone: Vec<u8> = symbol.clone();
        token_config.set_token_base(token_address, symbol, decimals, max_price);
        symbol_config.set(symbol_clone, token_address);
        Ok(())
    }
}

impl SetTokenTradeFeeMap<'_>{
    pub fn set_token_trade_fee_map(
        ctx: Context<SetTokenTradeFeeMap>,
        token_address: [u16; 20],
        chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    ) -> Result<()> {
        let token_config = &mut ctx.accounts.token_config;
        require!(
            chain_ids.len() == moleculars.len() && chain_ids.len() == denominators.len(),
            ErrorCode::InvalidLength
        );
        for (i, &current_id) in chain_ids.iter().enumerate() {
            token_config.set_token_trade_fee_config(
                token_address,
                current_id,
                moleculars[i],
                denominators[i]
            );
        }
        Ok(())
    }
}

    pub fn compute_trade_fee(
        global_trade_fee_molecular: u64,
        global_trade_fee_denominator: u64,
        token_fee_config_molecular: u64,
        token_fee_config_denominator: u64,
        dest_chain_id: u64,
        token: [u16; 20],
        expect_amount_receive: u64,
    ) -> Option<u64> {
        let fee;
        let molecular;
        let denominator;
        if (token_fee_config_molecular < 1) {
            molecular = global_trade_fee_molecular;
            denominator = global_trade_fee_denominator;
        } else {
            molecular = token_fee_config_molecular;
            denominator = token_fee_config_denominator;
        }
        fee = expect_amount_receive * molecular / denominator;
        Some(fee)
    }

    /// totalAmount =  expectAmountReceive + expectAmountReceive * (molecular / denominator)
    pub fn compute_total_amont(
        global_trade_fee_molecular: u64,
        global_trade_fee_denominator: u64,
        token_fee_config_molecular: u64,
        token_fee_config_denominator: u64,
        dest_chain_id: u64,
        token: [u16; 20],
        expect_amount_receive: u64,
    ) -> Option<u64> {
        let total_amount;
        let fee = compute_trade_fee(
            global_trade_fee_molecular,
            global_trade_fee_denominator,
            token_fee_config_molecular,
            token_fee_config_denominator,
            dest_chain_id,
            token,
            expect_amount_receive,
        )?;

        total_amount = expect_amount_receive + fee;
        Some(total_amount)
    }

    /// realAmount = ((totalAmount × denominator^5) / (molecular + denominator)) / denominator^4
    pub fn compute_amount_composition(
        global_trade_fee_molecular: u64,
        global_trade_fee_denominator: u64,
        token_fee_config_molecular: u64,
        token_fee_config_denominator: u64,
        dest_chain_id: u64,
        token: [u16; 20],
        total_amount: u64,
    ) -> Option<(u64, u64)> {
        let real_amount;
        let trade_fee;
        let molecular;
        let denominator;
        let one_amount;
        if (token_fee_config_molecular < 1) {
            molecular = global_trade_fee_molecular;
            denominator = global_trade_fee_denominator;
        } else {
            molecular = token_fee_config_molecular;
            denominator = token_fee_config_denominator;
        }
        one_amount = (total_amount * denominator.pow(5)) / (molecular + denominator);
        real_amount = one_amount / denominator.pow(4);
        trade_fee = total_amount - real_amount;
        Some((real_amount, trade_fee))
    }



#[derive(Accounts)]
pub struct ChangePowerUser<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info, SaveChainId>,
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
pub struct WithdrawSplToken<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info, SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"vizing_vault".as_ref()],
        bump
    )]
    pub contract_authority: Account<'info, VaultMes>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info, SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub source: AccountInfo<'info>,
    #[account(mut)]
    pub destination: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetTokenInfoBase<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info, SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        payer = user, 
        space = 8 + 128,
        seeds = [b"init_token_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub token_config: Account<'info, MappingTokenConfig>,
    #[account(
        init, 
        payer = user, 
        space = 8 + 128,
        seeds = [b"init_symbol_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub symbol_config: Account<'info, MappingSymbolConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetTokenTradeFeeMap<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info, SaveChainId>,
    #[account(
        mut,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"init_token_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub token_config: Account<'info, MappingTokenConfig>,
    pub system_program: Program<'info, System>,

}
