use anchor_lang::prelude::*;

pub const VIZING_AUTHORITY: Pubkey = Pubkey::new_from_array([
    137, 71, 54, 167, 199, 236, 39, 80, 113, 216, 76, 7, 85, 39, 112, 180, 125, 214, 156, 170, 202,
    74, 57, 119, 4, 40, 1, 88, 236, 158, 120, 105,
]);
pub const VIZING_APP_SOL_RECEIVER_SEED: &[u8] = b"Vizing_App_Sol_Receiver_Seed";

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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct LandingMessage {
    pub mode: u8,
    pub target_program: Pubkey,
    pub execute_gas_limit: u64,
    pub max_fee_per_gas: u64,
    #[max_len(256)]
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
