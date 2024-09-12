use anchor_lang::prelude::*;
// use std::mem::size_of;
// account struct for add_and_storeuse bob::cpi::accounts::BobAddOp;

// The program definition for Bob
use bob::program::Bob;

// the account where Bob is storing the sum
// use bob::BobData;
use bob::{self, cpi::accounts::BobAddOp, BobData, SENDER_SEED};

declare_id!("EM1xuughPx6KVN7H4DSC7xe7KuTzmMH52X4FnHF22wzF");

#[program]
pub mod alice {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Alice Initialized");
        Ok(())
    }

    pub fn sender_account_initializer(
        _ctx: Context<VizingSender>,
        sender_id: Pubkey,
    ) -> Result<()> {
        msg!("Sender Account Initialized: {}", sender_id);
        Ok(())
    }

    pub fn ask_bob_to_add(ctx: Context<AliceOp>, a: u64, b: u64) -> Result<()> {
        let (account, bump) = Pubkey::find_program_address(&[SENDER_SEED], &ctx.program_id);
        msg!("account: {}, bump: {}", account, bump);

        let seeds = &[SENDER_SEED, &[bump]];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.bob_program.to_account_info(),
            BobAddOp {
                bob_data_account: ctx.accounts.bob_data_account.to_account_info(),
                message_authority: ctx.accounts.sender_account.to_account_info(),
            },
            signer, //&[&[SENDER_SEED]],
        );

        let res = bob::cpi::add_and_store(cpi_ctx, a, b);

        // return an error if the CPI failed
        if res.is_ok() {
            msg!("CPI to bob succeeded");
            return Ok(());
        } else {
            msg!("CPI to bob failed");
            return err!(Errors::CPIToBobFailed);
        }
    }
}

#[error_code]
pub enum Errors {
    #[msg("cpi to bob failed")]
    CPIToBobFailed,
}

#[derive(Accounts)]
pub struct AliceOp<'info> {
    #[account(mut)]
    pub bob_data_account: Account<'info, BobData>,

    /// CHECK:
    #[account(mut)]
    pub sender_account: AccountInfo<'info>,

    pub bob_program: Program<'info, Bob>,
}

#[derive(Accounts)]
#[instruction(sender_id: Pubkey)]
pub struct VizingSender<'info> {
    /// CHECK: We need this PDA as a signer
    #[account(
        init, payer = signer,
        seeds = [SENDER_SEED],
        bump,
        space = 8
    )]
    pub sender_account: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}
