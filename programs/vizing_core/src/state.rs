use anchor_lang::prelude::*;

#[account]
pub struct SaveChainId {
    pub dest_chain_id: Vec<u8>,
}

#[account]
pub struct PowerUser {
    pub admin: Pubkey,
    pub engine_admin: Pubkey,
    pub station_admin: Pubkey,
    pub gas_pool_admin: Pubkey,
    pub trusted_relayers: Vec<Pubkey>,
    pub registered_validators: Vec<Pubkey>,
    pub gas_managers: Vec<Pubkey>,
    pub swap_managers: Vec<Pubkey>,
    pub token_managers: Vec<Pubkey>,
}

#[account]
pub struct GasSystemGlobal {
    pub global_base_price: u64,
    pub default_gas_limit: u64,
    pub amount_in_threshold: u64,
    pub molecular: u64,
    pub denominator: u64,
}

#[account]
#[derive(InitSpace)]
pub struct VaultMes {
    pub current_pragma_id: Pubkey,
}
