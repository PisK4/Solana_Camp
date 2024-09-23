use crate::vizing_omni::*;
use anchor_lang::prelude::*;
use crate::library::Uint256;

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
    pub fee: u64,
    pub dest_chainid: u64,
    pub addition_params: Vec<u8>,
    pub message: Message,
}
