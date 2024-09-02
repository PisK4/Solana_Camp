use anchor_lang::prelude::*;

pub const VIZING_MESSAGE_AUTHORITY_SEED: &[u8] = b"Vizing_Message_Authority_Seed";
pub const VIZING_ERLIEST_ARRIVAL_TIMESTAMP_DEFAULT: u64 = 0;
pub const VIZING_LATEST_ARRIVAL_TIMESTAMP_DEFAULT: u64 = 0;
pub const VIZING_RELAYER_DEFAULT: Pubkey = Pubkey::new_from_array([
    137, 71, 54, 167, 199, 236, 39, 80, 113, 216, 76, 7, 85, 39, 112, 180, 125, 214, 156, 170, 202,
    74, 57, 119, 4, 40, 1, 88, 236, 158, 120, 105,
]);
pub const VIZING_GASLIMIT_DEFAULT: u64 = 0;

#[derive(Accounts)]
pub struct VizingEmitterInitialize<'info> {
    #[account(init, payer = payer, space = 8 + VizingMessageAuthority::INIT_SPACE, seeds = [VIZING_MESSAGE_AUTHORITY_SEED], bump)]
    pub message_pda_authority: Account<'info, VizingMessageAuthority>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct VizingFeeRouter {
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VizingMessageAuthority {
    pub bump: u8,
}

impl VizingEmitterInitialize<'_> {
    pub fn handler(ctx: Context<Self>) -> Result<()> {
        ctx.accounts.message_pda_authority.bump = *ctx.bumps.get("message_pda_authority").unwrap();
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct LaunchParams {
    pub erliest_arrival_timestamp: u64,
    pub latest_arrival_timestamp: u64,
    pub relayer: Pubkey,
    pub sender: Pubkey,
    pub value: u64,
    pub dest_chainid: u64,
    #[max_len(256)]
    pub addition_params: Vec<u8>,
    pub message: Message,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Message {
    pub mode: u8,
    pub target_program: [u8; 32],
    pub execute_gas_limit: u64,
    pub max_fee_per_gas: u64,
    #[max_len(256)]
    pub signature: Vec<u8>,
}
