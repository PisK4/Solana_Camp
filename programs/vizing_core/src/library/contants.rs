use anchor_lang::prelude::*;
// ******* accounts ********
pub const SYSTEM_ACCOUNT: Pubkey = Pubkey::new_from_array([1u8; 32]);

// ******* seeds ********
pub const VIZING_PAD_SETTINGS_SEED: &[u8] = b"Vizing_Pad_Settings_Seed";
pub const RELAYER_SETTINGS_SEED: &[u8] = b"Relayer_Settings_Seed";
pub const VIZING_AUTHORITY_SEED: &[u8] = b"Vizing_Authority_Seed";

// ******* governance ********
pub const RELAYERS_LENGTH: usize = 3;
