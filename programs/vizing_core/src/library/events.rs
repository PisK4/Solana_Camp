use anchor_lang::prelude::*;
use crate::vizing_channel::LandingMessage;
use crate::vizing_omni::{AdditionalParams, Message};
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
    pub relayer: [u8; 32],
    pub sender: Pubkey,
    pub src_contract: Pubkey,
    pub value: Uint256,
    pub fee: u64,
    pub dest_chainid: u64,
    pub addition_params: AdditionalParams,
    pub message: Message,
}

#[event]
pub struct SuccessfulLanding {
    pub message_id: [u8; 32],
    pub erliest_arrival_timestamp: u64,
    pub latest_arrival_timestamp: u64,
    pub src_chainid: u64,
    pub src_tx_hash: [u8; 32],
    pub src_contract: [u8; 32],
    pub src_chain_nonce: u32,
    pub sender: [u8; 32],
    pub value: u64,
    pub addition_params: AdditionalParams,
    pub message: LandingMessage,
}
