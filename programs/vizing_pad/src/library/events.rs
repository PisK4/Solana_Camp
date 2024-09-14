use crate::vizing_omni::*;
use anchor_lang::prelude::*;

#[event]
pub struct OAppRegisteredEvent {
    pub oapp: Pubkey,
    pub delegate: Pubkey,
}

#[event]
pub struct SuccessfulLaunchMessage {
    pub erliest_arrival_timestamp: u64,
    pub latest_arrival_timestamp: u64,
    pub relayer: Pubkey,
    pub sender: Pubkey,
    pub src_contract: Pubkey,
    pub value: Uint256,
    pub fee: Uint256,
    pub dest_chainid: u64,
    pub addition_params: Vec<u8>,
    pub message: Message,
}
