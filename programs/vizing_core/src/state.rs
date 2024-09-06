use anchor_lang::prelude::*;

#[account]
pub struct SaveChainId {
    pub dest_chain_id: Vec<u8>,
}

#[account]
#[derive(InitSpace)]
pub struct VaultMes {
    pub current_pragma_id: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct CurrentRecordMessage {
    pub compute_trade_fee1: u64,
    pub compute_trade_fee2: u64,
    pub estimate_price2: u64,
    pub estimate_gas: u64,
    pub estimate_total_fee: u64,
    pub exact_output: u64,
    pub exact_input: u64,
    pub init_state: bool
}
