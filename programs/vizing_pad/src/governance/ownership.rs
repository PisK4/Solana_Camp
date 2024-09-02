use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VizingPadConfigs {
    // immutable
    pub bump: u8,
    // configurable
    pub owner: Pubkey,
    pub fee_collector: Pubkey,
    pub engine_admin: Pubkey,
    pub station_admin: Pubkey,
    pub gas_pool_admin: Pubkey,
    #[max_len(96)]
    pub trusted_relayers: Vec<Pubkey>,
    pub registered_validator: Pubkey,
    // state
    pub is_paused: bool,
}
