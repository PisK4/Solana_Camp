use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer as SolTransfer};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use spl_associated_token_account::get_associated_token_address;

use crate::error::ErrorCode;
use crate::message_type_lib::*;
use crate::state::*;
use crate::message_monitor_lib::*;

pub mod expert_hook {
    use super::*;

    pub fn init_power_user(
        ctx: Context<InitPowerUser>,
        new_admin: Pubkey,
        new_engine_admin: Pubkey,
        new_station_admin: Pubkey,
        new_gas_pool_admin: Pubkey,
        new_trusted_relayer: Pubkey,
        new_registered_validator: Pubkey,
        new_gas_manager: Pubkey,
        new_swap_manager: Pubkey,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        power_user.admin_role=new_admin;
        power_user.engine_admin=new_engine_admin;
        power_user.station_admin=new_station_admin;
        power_user.gas_pool_admin=new_gas_pool_admin;
        power_user.trusted_relayer=new_trusted_relayer;
        power_user.registered_validator=new_registered_validator;
        power_user.gas_manager=new_gas_manager;
        power_user.swap_manager=new_swap_manager;
        Ok(())

    }

    pub fn change_power_user(
        ctx: Context<ChangePowerUser>,
        new_admin: Pubkey,
        new_engine_admin: Pubkey,
        new_station_admin: Pubkey,
        new_gas_pool_admin: Pubkey,
        new_trusted_relayer: Pubkey,
        new_registered_validator: Pubkey,
        new_gas_manager: Pubkey,
        new_swap_manager: Pubkey,
    ) -> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        power_user.admin_role=new_admin;
        power_user.engine_admin=new_engine_admin;
        power_user.station_admin=new_station_admin;
        power_user.gas_pool_admin=new_gas_pool_admin;
        power_user.trusted_relayer=new_trusted_relayer;
        power_user.registered_validator=new_registered_validator;
        power_user.gas_manager=new_gas_manager;
        power_user.swap_manager=new_swap_manager;
        Ok(())
    }

    pub fn withdraw_spl_token(
        ctx: Context<WithdrawSplToken>,
        withdraw_amount: u64,    
        this_bump: u8    
    )->Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key(); 
        require!(user_key==power_user.admin_role, ErrorCode::NonAdmin);
        let save_chain_id=&mut ctx.acconts.save_chain_id;
        let dest_chain_id=save_chain_id.dest_chain_id;

        let seeds = &[b"vizing_vault".as_ref(),dest_chain_id.as_ref(), &[this_bump]];
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

    pub fn withdraw_sol(
        ctx: Context<WithdrawSol>,
        withdraw_amount: u64,
    )-> Result<()> {
        let power_user = &mut ctx.accounts.power_user;
        let user_key = &mut ctx.accounts.user.key(); 
        require!(user_key==power_user.admin_role, ErrorCode::NonAdmin);

        let source = ctx.accounts.source.to_account_info();
        let destination = ctx.accounts.destination.to_account_info();

        **source.try_borrow_mut_lamports()? -= withdraw_amount;
        **destination.try_borrow_mut_lamports()? += withdraw_amount;
        Ok(())
    }

    
}

#[derive(Accounts)]
pub struct InitPowerUser<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
    #[account(
        init, 
        payer = user, 
        space = 8 + 128,
        seeds = [b"init_power_user".as_ref(),&save_chain_id.dest_chain_id.as_ref()],
        bump
    )]
    pub power_user: Account<'info, PowerUser>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangePowerUser<'info> {
    #[account(mut)]
    pub save_chain_id: Account<'info,SaveChainId>,
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
    pub save_chain_id: Account<'info,SaveChainId>,
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
    pub save_chain_id: Account<'info,SaveChainId>,
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
