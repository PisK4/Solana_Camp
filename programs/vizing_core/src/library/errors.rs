use anchor_lang::prelude::error_code;

#[error_code]
pub enum VizingError {
    #[msg("signer of message invalid")]
    MessageSigerInvalid,
    #[msg("Unauthorized: Not Owner")]
    NotOwner,
    #[msg("Unauthorized: Not Engine Admin")]
    NotEngineAdmin,
    #[msg("Already Initialized")]
    AlreadyInitialized,
    #[msg("Unauthorized: Fee Collector Invalid")]
    FeeCollectorInvalid,
    #[msg("Vizing Not Active")]
    VizingNotActive,
}
