use anchor_lang::prelude::*;

declare_id!("C17xMdoPdgPSYd7oGEjYf5LQ1mg6k6P3eavCBdMfaF1X");

#[program]
pub mod vizing_app {

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn receive_from_vizing(ctx: Context<LandingAppOp>, params: LandingParams) -> Result<()> {
        msg!(
            "@@authority from vizing: {}",
            ctx.accounts.vizing_authority.key()
        );

        msg!(
            "authority is signer: {}",
            ctx.accounts.vizing_authority.is_signer
        );

        msg!("Hello world from vizing");

        msg!("message: {:?}", params.message);
        msg!("remaining_accounts: {:?}", ctx.remaining_accounts);

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct LandingParams {
    pub message_id: [u8; 32],
    pub erliest_arrival_timestamp: u64,
    pub latest_arrival_timestamp: u64,
    pub src_chainid: u64,
    pub src_tx_hash: [u8; 32],
    pub src_contract: Pubkey,
    pub src_chain_nonce: u32,
    pub sender: Pubkey,
    pub value: u64,
    #[max_len(256)]
    pub addition_params: Vec<u8>,
    pub message: LandingMessage,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug)]
pub struct LandingMessage {
    pub mode: u8,
    pub target_contract: Pubkey,
    pub execute_gas_limit: u64,
    pub max_fee_per_gas: u64,
    #[max_len(256)]
    pub signature: Vec<u8>,
}

#[derive(Accounts)]
pub struct LandingAppOp<'info> {
    /// CHECK: We need signer to claim ownership
    #[account(signer)]
    pub vizing_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program<'info, System>,
}
