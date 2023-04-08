use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum OperationError {
    #[error("account-already-initialized")]
    AccountAlreadyInitialized,
    #[error("account-not-initialized")]
    AccountNotInitialized,
    #[error("inactive-card")]
    InactiveCard,
    #[error("insufficient-limit")]
    InsufficientLimit,
    #[error("high-frequency-small-interval")]
    HighFrequencySmallInterval,
    #[error("duplicated-tx")]
    DuplicatedTx,
}
