use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
  #[error("account-already-initialized")]
  AccountAlreadyInitialized,
  #[error("inactive-card")]
  InactiveCard,
  #[error("insufficient-limit")]
  InsufficientLimitError,
  #[error("high-frequency-small-interval")]
  HighFrequencySmallInterval(String, String),
  #[error("duplicated-transaction")]
  DuplicatedTx
}