use anchor_lang::prelude::*;

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
    pub target_contract: [u8; 40],
    pub execute_gas_limit: u32,
    pub max_fee_per_gas: u64,
    #[max_len(256)]
    pub signature: Vec<u8>,
}
