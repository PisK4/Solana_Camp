use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("bobQhTJQZZk4p98GrskDGxWZpfd5gnap1Hp5JiXpNek");

pub const SENDER_SEED: &[u8] = b"vizing_message_sender";

#[program]
pub mod bob {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!(
            "Data Account Initialized: {}",
            ctx.accounts.bob_data_account.key()
        );

        Ok(())
    }

    pub fn sender_account_initializer(
        _ctx: Context<VizingSender>,
        sender_id: Pubkey,
    ) -> Result<()> {
        msg!("Sender Account Initialized: {}", sender_id);
        Ok(())
    }

    pub fn add_and_store(ctx: Context<BobAddOp>, a: u64, b: u64) -> Result<()> {
        msg!("Adding start");
        let is_signer_bob = ctx.accounts.bob_data_account.to_account_info().is_signer;
        msg!("is_signer_bob: {:?}", is_signer_bob);
        let is_signer_authority = ctx.accounts.message_authority.to_account_info().is_signer;
        msg!("is_signer_authority: {:?}", is_signer_authority);
        msg!("authority: {:?}", ctx.accounts.message_authority.key());

        let result = a + b;
        ctx.accounts.bob_data_account.result = result;
        Ok(())
    }
}

#[account]
pub struct BobData {
    pub result: u64,
}

#[account]
pub struct SenderData {
    pub sender_id: Pubkey,
}

#[derive(Accounts)]
pub struct BobAddOp<'info> {
    #[account(mut)]
    pub bob_data_account: Account<'info, BobData>,
    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub message_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = size_of::<BobData>() + 8)]
    pub bob_data_account: Account<'info, BobData>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum BobError {
    InvalidInput,
    MessageSigerUnauthorized,
}

#[derive(Accounts)]
#[instruction(sender_id: Pubkey)]
pub struct VizingSender<'info> {
    /// CHECK: We need this PDA as a signer
    #[account(
        init, payer = signer,
        seeds = [SENDER_SEED],
        bump,
        space = size_of::<SenderData>() + 8
    )]
    pub sender_account: Account<'info, SenderData>,

    #[account(mut)]
    pub signer: Signer<'info>,

    system_program: Program<'info, System>,
}
