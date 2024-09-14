use anchor_lang::prelude::*;
// ******* accounts ********
pub const SYSTEM_ACCOUNT: Pubkey = Pubkey::new_from_array([1u8; 32]);
pub const RECEIVE_FROM_VIZING_DISCRIMINATOR: [u8; 8] = [70, 242, 190, 65, 53, 148, 145, 116];

// ******* seeds ********
pub const VIZING_PAD_CONFIG_SEED: &[u8] = b"Vizing_Pad_Settings_Seed";
pub const RELAYER_SETTINGS_SEED: &[u8] = b"Relayer_Settings_Seed";
pub const VIZING_AUTHORITY_SEED: &[u8] = b"Vizing_Authority_Seed";
pub const VIZING_GAS_SYSTEM_SEED: &[u8] = b"init_mapping_fee_config";

// ******* governance ********
pub const RELAYERS_LENGTH: usize = 3;
