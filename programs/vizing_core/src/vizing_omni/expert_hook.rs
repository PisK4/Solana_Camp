use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::governance::*;
use crate::library::*;
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct TokenBase {
    pub key: [u8; 32], //token address
    #[max_len(6)]
    pub symbol: String, //token symbol
    pub decimals: u8,
    pub max_price: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct TokenTradeFeeConfig {
    pub key1: [u8; 32], //token address
    pub key2: u64,      //destChainId
    pub molecular: u64,
    pub denominator: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct SymbolConfig {
    #[max_len(6)]
    pub key: String, //token symbol
    pub address: [u8; 32], //token address
}

#[account]
#[derive(InitSpace)]
pub struct MappingTokenConfig {
    #[max_len(90)]
    pub token_base_mappings: Vec<TokenBase>,
    #[max_len(90)]
    pub token_trade_fee_config_mappings: Vec<TokenTradeFeeConfig>,
}

#[account]
#[derive(InitSpace)]
pub struct MappingSymbolConfig {
    #[max_len(25)]
    pub symbol_config_mappings: Vec<SymbolConfig>,
}

impl MappingTokenConfig {
    pub fn set_token_base(&mut self, key: [u8; 32], symbol: String, decimals: u8, max_price: u64) {
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
    }

    pub fn set_token_trade_fee_config(
        &mut self,
        key1: [u8; 32],
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
    }

    pub fn get_token_base(&self, key: [u8; 32]) -> Option<TokenBase> {
        self.token_base_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }

    pub fn get_token_trade_fee_config(
        &self,
        key1: [u8; 32],
        key2: u64,
    ) -> Option<TokenTradeFeeConfig> {
        self.token_trade_fee_config_mappings
            .iter()
            .find(|pair| pair.key1 == key1 && pair.key2 == key2)
            .cloned()
    }
}

impl MappingSymbolConfig {
    pub fn set(&mut self, key: String, address: [u8; 32]) {
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
    }

    pub fn get(&self, key: String) -> Option<SymbolConfig> {
        self.symbol_config_mappings
            .iter()
            .find(|pair| pair.key == key)
            .cloned()
    }
}

//init
impl InitTokenInfoBase<'_> {
    pub fn initialize_token_info_base(
        ctx: Context<InitTokenInfoBase>,
        symbol: String,
        token_address: [u8; 32],
        decimals: u8,
        max_price: u64,
    ) -> Result<()> {
        let token_config = &mut ctx.accounts.token_config;
        let symbol_config = &mut ctx.accounts.symbol_config;
        let symbol_clone = symbol.clone();
        token_config.set_token_base(token_address, symbol, decimals, max_price);
        symbol_config.set(symbol_clone, token_address);
        Ok(())
    }
}

impl WithdrawSplToken<'_> {
    pub fn withdraw_spl_token(
        ctx: Context<WithdrawSplToken>,
        withdraw_amount: u64,
        this_bump: u8,
    ) -> Result<()> {

        let seeds = &[b"vizing_vault".as_ref(), &[this_bump]];
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

impl WithdrawSol<'_> {
    pub fn withdraw_sol(ctx: Context<WithdrawSol>, withdraw_amount: u64) -> Result<()> {
        let source = ctx.accounts.source.to_account_info();
        let destination = ctx.accounts.destination.to_account_info();

        // check source balance
        let source_balance = **source.try_borrow_lamports()?;
        require!(
            source_balance > withdraw_amount,
            errors::ErrorCode::InsufficientAmount
        );

        **source.try_borrow_mut_lamports()? -= withdraw_amount;
        **destination.try_borrow_mut_lamports()? += withdraw_amount;
        Ok(())
    }
}

impl SetTokenInfoBase<'_> {
    pub fn set_token_info_base(
        ctx: Context<SetTokenInfoBase>,
        symbol: String,
        token_address: [u8; 32],
        decimals: u8,
        max_price: u64,
    ) -> Result<()> {
        let token_config = &mut ctx.accounts.token_config;
        let symbol_config = &mut ctx.accounts.symbol_config;
        let symbol_clone = symbol.clone();
        token_config.set_token_base(token_address, symbol, decimals, max_price);
        symbol_config.set(symbol_clone, token_address);
        Ok(())
    }
}

impl SetTokenTradeFeeMap<'_> {
    pub fn set_token_trade_fee_map(
        ctx: Context<SetTokenTradeFeeMap>,
        token_address: [u8; 32],
        chain_ids: Vec<u64>,
        moleculars: Vec<u64>,
        denominators: Vec<u64>,
    ) -> Result<()> {
        let token_config = &mut ctx.accounts.token_config;
        require!(
            chain_ids.len() == moleculars.len() && chain_ids.len() == denominators.len(),
            errors::ErrorCode::InvalidLength
        );
        for (i, &current_id) in chain_ids.iter().enumerate() {
            token_config.set_token_trade_fee_config(
                token_address,
                current_id,
                moleculars[i],
                denominators[i],
            );
        }
        Ok(())
    }
}

pub fn compute_trade_fee(
    gas_system_global_molecular: u64,
    gas_system_global_denominator: u64,
    token_fee_config_molecular: u64,
    token_fee_config_denominator: u64,
    _dest_chain_id: u64,
    _token: [u8; 32],
    expect_amount_receive: u64,
) -> Option<u64> {
    let fee;
    let molecular;
    let denominator;
    if token_fee_config_molecular < 1 {
        molecular = gas_system_global_molecular;
        denominator = gas_system_global_denominator;
    } else {
        molecular = token_fee_config_molecular;
        denominator = token_fee_config_denominator;
    }
    fee = expect_amount_receive
        .checked_mul(molecular)?
        .checked_div(denominator)?;
    Some(fee)
}

/// totalAmount =  expectAmountReceive + expectAmountReceive * (molecular / denominator)
pub fn compute_total_amont(
    gas_system_global_molecular: u64,
    gas_system_global_denominator: u64,
    token_fee_config_molecular: u64,
    token_fee_config_denominator: u64,
    dest_chain_id: u64,
    token: [u8; 32],
    expect_amount_receive: u64,
) -> Option<u64> {
    let total_amount;
    let fee = compute_trade_fee(
        gas_system_global_molecular,
        gas_system_global_denominator,
        token_fee_config_molecular,
        token_fee_config_denominator,
        dest_chain_id,
        token,
        expect_amount_receive,
    )?;

    total_amount = expect_amount_receive.checked_add(fee)?;
    Some(total_amount)
}

/// realAmount = ((totalAmount × denominator^5) / (molecular + denominator)) / denominator^4
pub fn compute_amount_composition(
    gas_system_global_molecular: u64,
    gas_system_global_denominator: u64,
    token_fee_config_molecular: u64,
    token_fee_config_denominator: u64,
    _dest_chain_id: u64,
    _token: [u8; 32],
    total_amount: u64,
) -> Option<(u64, u64)> {
    let real_amount;
    let trade_fee;
    let molecular;
    let denominator;
    let one_amount;
    if token_fee_config_molecular < 1 {
        molecular = gas_system_global_molecular;
        denominator = gas_system_global_denominator;
    } else {
        molecular = token_fee_config_molecular;
        denominator = token_fee_config_denominator;
    }

    let denominator_pow_5 = denominator.checked_pow(5)?;
    let molecular_plus_denominator = molecular.checked_add(denominator)?;
    let numerator = total_amount.checked_mul(denominator_pow_5)?;
    one_amount = numerator.checked_div(molecular_plus_denominator)?;

    real_amount = one_amount.checked_div(denominator.pow(4))?;
    trade_fee = total_amount.checked_sub(real_amount)?;
    Some((real_amount, trade_fee))
}

//init
#[derive(Accounts)]
pub struct InitTokenInfoBase<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info, SaveChainId>,
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadSettings>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        payer = user, 
        space = 8 + MappingTokenConfig::INIT_SPACE,
        seeds = [b"init_token_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub token_config: Account<'info, MappingTokenConfig>,
    #[account(
        init, 
        payer = user, 
        space = 8 + MappingSymbolConfig::INIT_SPACE,
        seeds = [b"init_symbol_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub symbol_config: Account<'info, MappingSymbolConfig>,
    pub system_program: Program<'info, System>,
}

//set

#[derive(Accounts)]
pub struct WithdrawSplToken<'info> {
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.owner == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadSettings>,
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
     #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.owner == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadSettings>,
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
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadSettings>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"init_token_config".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub token_config: Account<'info, MappingTokenConfig>,
    #[account(
        mut,
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
    #[account(seeds = [VIZING_PAD_SETTINGS_SEED], bump = vizing.bump
        , constraint = vizing.gas_pool_admin == user.key() @VizingError::NotGasPoolAdmin)]
    pub vizing: Account<'info, VizingPadSettings>,
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
