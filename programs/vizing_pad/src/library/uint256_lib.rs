use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub struct Uint256 {
    pub high: u128,
    pub low: u128,
}

impl Uint256 {
    pub fn new(high: u128, low: u128) -> Self {
        Self { high, low }
    }
}
