use anchor_lang::prelude::error_code;

#[error_code]
pub enum VizingError {
    #[msg("signer of message invalid")]
    MessageSigerInvalid,
    #[msg("Unauthorized: Not Owner")]
    NotOwner,
    #[msg("Already Initialized")]
    AlreadyInitialized,
}
