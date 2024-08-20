use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Non admin role.")]
    NonAdmin,
    #[msg("not a engine.")]
    NonEngine,
    #[msg("Insufficient amount.")]
    InsufficientAmount,
    #[msg("Contract is paused.")]
    ContractPaused,
    #[msg("Not a gas manager.")]
    NonGasManager,
    #[msg("Invalid length.")]
    InvalidLength,
    #[msg("Invalid mapping.")]
    InvalidMapping,
    #[msg("Not a swap manager.")]
    NonSwapManager,
    #[msg("Overflow")]
    Overflow,
    #[msg("Invalid message.")]
    InvalidMessage,
    #[msg("Price too low.")]
    PriceTooLow,
    #[msg("Not a manager.")]
    NotManager,
    #[msg("Invalid mapping.")]
    InValidMappingAccount,
    #[msg("Fee config not found.")]
    FeeConfigNotFound,
    
}
