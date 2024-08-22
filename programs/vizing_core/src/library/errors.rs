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
<<<<<<< HEAD
    #[msg("Vizing Not Activated")]
    VizingNotActivated,
    #[msg("Target Program Invalid")]
    TargetContractInvalid,
    #[msg("target program calling failed")]
    CallingFailed,
}
=======
    #[msg("Vizing Not Active")]
    VizingNotActive,
}
>>>>>>> 55ad0df065f670e385ba1247c1b2bfdbd863c3a2
