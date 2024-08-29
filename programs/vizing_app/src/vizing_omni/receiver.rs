use anchor_lang::prelude::*;

pub const VIZING_AUTHORITY: Pubkey = Pubkey::new_from_array([
    137, 71, 54, 167, 199, 236, 39, 80, 113, 216, 76, 7, 85, 39, 112, 180, 125, 214, 156, 170, 202,
    74, 57, 119, 4, 40, 1, 88, 236, 158, 120, 105,
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
