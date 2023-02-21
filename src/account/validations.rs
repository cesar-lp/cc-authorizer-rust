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

            if (tx.seconds_since(right_tx)) <= 120 && right_tx.seconds_since(left_tx) <= 120 {
                return Some(OperationError::HighFrequencySmallInterval);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::*;

    #[test]
    fn insufficient_limit_tx_amount_valid() {
        let account = Account::new(100, true, vec![]);
        let tx = TX::new(100, "Nike", DateTime::default());

        assert_eq!(InsufficientLimit {}.validate(&account, &tx), None);
    }

    #[test]
    fn insufficient_limit_tx_amount_invalid() {
        let account = Account::new(100, true, vec![]);
        let tx = TX::new(101, "Nike", DateTime::default());

        assert_eq!(
            InsufficientLimit {}.validate(&account, &tx),
            Some(OperationError::InsufficientLimit)
        );
    }

    #[test]
    fn duplicated_tx_different_tx() {
        let account = Account::new(100, true, vec![TX::new(101, "Nike", DateTime::default())]);
        let tx = TX::new(102, "Nike", DateTime::default());

        assert_eq!(DuplicatedTx {}.validate(&account, &tx), None);
    }

    #[test]
    fn duplicated_tx_same_tx() {
        let account = Account::new(100, true, vec![TX::new(101, "Nike", DateTime::default())]);
        let tx = TX::new(101, "Nike", DateTime::default());

        assert_eq!(
            DuplicatedTx {}.validate(&account, &tx),
            Some(OperationError::DuplicatedTx)
        );
    }

    #[test]
    fn high_frequency_small_interval_ok() {
        let account = Account::new(
            1000,
            true,
            vec![
                TX::new(
                    101,
                    "Nike",
                    DateTime::parse_from_rfc3339("2019-02-13T11:00:00.000Z")
                        .unwrap()
                        .into(),
                ),
                TX::new(
                    101,
                    "Coke",
                    DateTime::parse_from_rfc3339("2019-02-13T11:01:00.000Z")
                        .unwrap()
                        .into(),
                ),
                TX::new(
                    101,
                    "Pepsi",
                    DateTime::parse_from_rfc3339("2019-02-13T11:02:01.000Z")
                        .unwrap()
                        .into(),
                ),
            ],
        );
        let tx = TX::new(
            102,
            "RedBull",
            DateTime::parse_from_rfc3339("2019-02-13T11:02:02.000Z")
                .unwrap()
                .into(),
        );

        assert_eq!(HighFrequencySmallInterval {}.validate(&account, &tx), None);
    }

    #[test]
    fn high_frequency_small_interval_error() {
        let account = Account::new(
            1000,
            true,
            vec![
                TX::new(
                    101,
                    "Nike",
                    DateTime::parse_from_rfc3339("2019-02-13T11:00:00.000Z")
                        .unwrap()
                        .into(),
                ),
                TX::new(
                    101,
                    "Coke",
                    DateTime::parse_from_rfc3339("2019-02-13T11:01:00.000Z")
                        .unwrap()
                        .into(),
                ),
                TX::new(
                    101,
                    "Pepsi",
                    DateTime::parse_from_rfc3339("2019-02-13T11:01:59.999Z")
                        .unwrap()
                        .into(),
                ),
            ],
        );
        let tx = TX::new(
            102,
            "RedBull",
            DateTime::parse_from_rfc3339("2019-02-13T11:02:00.000Z")
                .unwrap()
                .into(),
        );

        assert_eq!(
            HighFrequencySmallInterval {}.validate(&account, &tx),
            Some(OperationError::HighFrequencySmallInterval)
        );
    }
}
