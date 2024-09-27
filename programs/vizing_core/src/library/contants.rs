use anchor_lang::prelude::*;
// ******* accounts ********
pub const SYSTEM_ACCOUNT: Pubkey = Pubkey::new_from_array([1u8; 32]);
pub const RECEIVE_FROM_VIZING_DISCRIMINATOR: [u8; 8] = [70, 242, 190, 65, 53, 148, 145, 116];

// ******* seeds ********
pub const VIZING_PAD_CONFIG_SEED: &[u8] = b"Vizing_Pad_Settings_Seed";
pub const VIZING_SENDER_NONCE_SEED: &[u8] = b"Vizing_Sender_Nonce_Seed";
pub const VIZING_AUTHORITY_SEED: &[u8] = b"Vizing_Authority_Seed";
pub const VIZING_GAS_SYSTEM_SEED: &[u8] = b"init_mapping_fee_config";
pub const VIZING_APP_CONFIG_SEED: &[u8] = b"Vizing_App_Config_Seed";
pub const VIZING_RECORD_SEED: &[u8] = b"init_current_record_message";

// ******* governance ********
pub const RELAYERS_LENGTH: usize = 3;
