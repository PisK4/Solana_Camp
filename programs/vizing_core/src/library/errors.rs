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
    #[msg("Vizing App Not In Remaining Accounts")]
    VizingAppNotInRemainingAccounts,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid length.")]
    InvalidLength,
    #[msg("Serialization message error.")]
    SerializationError,
    #[msg("Gas system global not found.")]
    GasSystemGlobalNotFound,
    #[msg("Fee config not found.")]
    FeeConfigNotFound,
    #[msg("Trade fee not found.")]
    TradeFeeNotFound,
    #[msg("Trade fee config not found.")]
    TradeFeeConfigNotFound,
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
    ExactInputputNotFound,
}
