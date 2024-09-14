use anchor_lang::prelude::*;

pub const VIZING_AUTHORITY: Pubkey = Pubkey::new_from_array([
    217, 72, 146, 56, 194, 10, 128, 182, 124, 7, 203, 21, 187, 13, 122, 157, 108, 189, 113, 111,
    236, 228, 172, 203, 215, 130, 70, 180, 107, 14, 165, 34,
]);
pub const VIZING_APP_SOL_RECEIVER_SEED: &[u8] = b"Vizing_App_Sol_Receiver_Seed";

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct VizingMessage {
    pub src_chainid: u64,
    pub src_contract: [u8; 32],
    pub value: u64,
    #[max_len(1024)]
    pub signature: Vec<u8>,
}

#[account]
#[derive(InitSpace)]
pub struct VizingSolReceiver {
    pub bump: u8,
}

pub fn assert_vizing_authority(vizing_authority: &AccountInfo) -> Result<()> {
    require!(
        vizing_authority.key == &VIZING_AUTHORITY,
        VizingIError::AccessDenied
    );
    Ok(())
}

#[error_code]
pub enum VizingIError {
    #[msg("Unauthorized: Not Vizing Authority")]
    AccessDenied,
}

#[derive(Accounts)]
pub struct VizingSolReceiverInitialize<'info> {
    #[account(init, payer = payer, space = 8 + VizingSolReceiver::INIT_SPACE, seeds = [VIZING_APP_SOL_RECEIVER_SEED], bump)]
    pub sol_pda_receiver: Account<'info, VizingSolReceiver>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl VizingSolReceiverInitialize<'_> {
    pub fn handler(ctx: Context<Self>) -> Result<()> {
        ctx.accounts.sol_pda_receiver.bump = *ctx.bumps.get("sol_pda_receiver").unwrap();
        Ok(())
    }
}
