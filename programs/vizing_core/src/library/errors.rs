use anchor_lang::prelude::error_code;

#[error_code]
pub enum VizingError {
    #[msg("signer of message invalid")]
    MessageSigerInvalid,
    #[msg("Unauthorized: Not Owner")]
    NotOwner,
    #[msg("Unauthorized: Not Engine Admin")]
    NotEngineAdmin,
    #[msg("Unauthorized: Not Gas Pool Admin")]
    NotGasPoolAdmin,
    #[msg("Unauthorized: Not Station Admin")]
    NotStationAdmin,
    #[msg("Unauthorized: Not Relayer")]
    NotRelayer,
    #[msg("Unauthorized: Relayer Not Active")]
    RelayerNotActivated,
    #[msg("Already Initialized")]
    AlreadyInitialized,
    #[msg("Unauthorized: Fee Collector Invalid")]
    FeeCollectorInvalid,
    #[msg("Vizing Not Activated")]
    VizingNotActivated,
    #[msg("Target Program Invalid")]
    TargetContractInvalid,
    #[msg("target program calling failed")]
    CallingFailed,
    #[msg("Insufficient Balance")]
    InsufficientBalance,
}

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
    #[msg("Not a token manager.")]
    NonTokenManager,
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
    #[msg("Serialization message error.")]
    SerializationError,
    #[msg("Fee config not found.")]
    FeeConfigNotFound,
    #[msg("Trade fee not found.")]
    TradeFeeNotFound,
    #[msg("Trade fee config not found.")]
    TradeFeeConfigNotFound,
    #[msg("Dapp config not found.")]
    DappConfigNotFound,
    #[msg("AmountInThresholds not found.")]
    AmountInThresholdsNotFound,
    #[msg("ComputeTradeFee1 not found.")]
    ComputeTradeFee1NotFound,
    #[msg("ComputeTradeFee2 not found.")]
    ComputeTradeFee2NotFound,
    #[msg("EstimatePrice2 not found.")]
    EstimatePrice2NotFound,
    #[msg("EstimateGas not found.")]
    EstimateGasNotFound,
    #[msg("EstimateTotalFee not found.")]
    EstimateTotalFeeNotFound,
    #[msg("ExactOutput not found.")]
    ExactOutputNotFound,
    #[msg("ExactInputput not found.")]
    ExactInputputNotFound
}
