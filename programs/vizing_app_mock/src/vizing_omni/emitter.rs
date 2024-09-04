use anchor_lang::prelude::*;

pub const VIZING_MESSAGE_AUTHORITY_SEED: &[u8] = b"Vizing_Message_Authority_Seed";
pub const VIZING_ERLIEST_ARRIVAL_TIMESTAMP_DEFAULT: u64 = 0;
pub const VIZING_LATEST_ARRIVAL_TIMESTAMP_DEFAULT: u64 = 0;
pub const VIZING_RELAYER_DEFAULT: [u8; 32] = [0; 32];
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
    pub relayer: [u8; 32],
    pub sender: Pubkey,
    pub value: u64,
    pub dest_chainid: u64,
    pub addition_params: AdditionalParams,
    pub message: Message,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Message {
    pub mode: u8,
    pub target_program: Pubkey,
    pub execute_gas_limit: u64,
    pub max_fee_per_gas: u64,
    #[max_len(1024)]
    pub signature: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct AdditionalParams {
    pub mode: u8,
    #[max_len(512)]
    pub signature: Vec<u8>,
}
