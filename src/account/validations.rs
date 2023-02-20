use std::fmt::Debug;

use crate::account::errors::OperationError;
use crate::account::models::{Account, TX};

pub trait AccountRule: Debug {
    fn validate(&self, account: &Account, tx: &TX) -> Option<OperationError>;
}

#[derive(Debug)]
pub struct DuplicatedTx {}

impl AccountRule for DuplicatedTx {
    fn validate(&self, account: &Account, tx: &TX) -> Option<OperationError> {
        let duplicated_tx = account
            .txs
            .iter()
            .any(|t| t.amount == tx.amount && t.merchant == tx.merchant);

        if duplicated_tx {
            return Some(OperationError::DuplicatedTx);
        }

        None
    }
}

#[derive(Debug)]
pub struct InsufficientLimit {}

impl AccountRule for InsufficientLimit {
    fn validate(&self, account: &Account, tx: &TX) -> Option<OperationError> {
        if account.available_limit < tx.amount {
            return Some(OperationError::InsufficientLimit);
        }

        None
    }
}

#[derive(Debug)]
pub struct HighFrequencySmallInterval {}

impl AccountRule for HighFrequencySmallInterval {
    fn validate(&self, account: &Account, tx: &TX) -> Option<OperationError> {
        let total_txs = account.txs.len();

        if total_txs >= 3 {
            let (left_tx, right_tx) = account.get_last_n_txs(3);

            if (tx.min_since(right_tx)) <= 2 && right_tx.min_since(left_tx) <= 2 {
                return Some(OperationError::HighFrequencySmallInterval);
            }
        }

        None
    }
}
