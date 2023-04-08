use chrono::{DateTime, Utc};
use serde::Serialize;
use std::{fmt::Debug, ops::Sub};

use crate::account::{
    AccountRule, DuplicatedTx, HighFrequencySmallInterval, InsufficientLimit, OperationError,
};

#[derive(Debug, PartialEq)]
pub struct TX {
    pub merchant: String,
    pub amount: u32,
    pub time: DateTime<Utc>,
}

impl TX {
    pub fn new(amount: u32, merchant: &str, time: DateTime<Utc>) -> Self {
        Self {
            amount,
            merchant: merchant.to_string(),
            time,
        }
    }

    pub fn seconds_since(&self, another: &TX) -> i64 {
        self.time.sub(another.time).num_seconds()
    }
}

#[derive(Debug)]
pub struct Account {
    pub available_limit: u32,
    pub active_card: bool,
    pub txs: Vec<TX>,
    rules: Vec<Box<dyn AccountRule>>,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.available_limit == other.available_limit
            && self.active_card == other.active_card
            && self.txs == other.txs
    }
}

impl Account {
    pub fn new(available_limit: u32, active_card: bool, txs: Vec<TX>) -> Self {
        Self {
            available_limit,
            active_card,
            txs,
            rules: vec![
                InsufficientLimit::boxed(),
                HighFrequencySmallInterval::boxed(),
                DuplicatedTx::boxed(),
            ],
        }
    }

    pub fn execute_tx(&mut self, tx: TX) -> Result<AccountState, Vec<OperationError>> {
        let errors: Vec<OperationError> = self
            .rules
            .iter()
            .filter_map(|r| r.validate(self, &tx))
            .collect();

        if !errors.is_empty() {
            return Err(errors);
        }

        self.available_limit -= tx.amount;
        self.txs.push(tx);

        Ok(AccountState::new(
            self.active_card,
            self.available_limit,
            vec![],
        ))
    }

    pub fn get_last_n_txs(&self, window_size: usize) -> (&TX, &TX) {
        let total_txs = self.txs.len();

        let initial_window_tx = self.txs.get(total_txs - window_size).unwrap();
        let end_window_tx = self.txs.get(total_txs - 1).unwrap();

        (initial_window_tx, end_window_tx)
    }

    pub fn to_invalid_state(&self, errors: Vec<OperationError>) -> AccountState {
        AccountState::new(self.active_card, self.available_limit, errors)
    }

    pub fn to_state(&self) -> AccountState {
        AccountState::new(self.active_card, self.available_limit, vec![])
    }

    pub fn is_inactive(&self) -> bool {
        !self.active_card
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct AccountState {
    active_card: bool,
    available_limit: u32,
    violations: Vec<String>,
}

impl AccountState {
    pub fn new(active_card: bool, available_limit: u32, errors: Vec<OperationError>) -> Self {
        Self {
            available_limit,
            active_card,
            violations: errors.iter().map(|e| e.to_string()).collect(),
        }
    }

    pub fn not_initialized() -> Self {
        AccountState::new(false, 0, vec![OperationError::AccountNotInitialized])
    }

    pub fn inactive(available_limit: u32) -> Self {
        AccountState::new(false, available_limit, vec![OperationError::InactiveCard])
    }
}

#[cfg(test)]
mod account {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn create() {
        let account = Account::new(
            100,
            true,
            vec![TX::new(50, "Merchant X", DateTime::default())],
        );

        let expected_account = Account {
            available_limit: 100,
            active_card: true,
            txs: vec![TX::new(50, "Merchant X", DateTime::default())],
            rules: vec![
                InsufficientLimit::boxed(),
                HighFrequencySmallInterval::boxed(),
                DuplicatedTx::boxed(),
            ],
        };

        assert_eq!(account, expected_account);
    }

    #[test]
    fn execute_tx_successfuly() {
        let mut account = Account::new(100, true, vec![]);

        let account_state = account
            .execute_tx(TX::new(50, "Merchant X", DateTime::default()))
            .unwrap();

        let expected_account_state = AccountState::new(true, 50, vec![]);

        assert_eq!(account_state, expected_account_state);
    }

    #[test]
    fn execute_tx_return_errors() {
        let mut account = Account::new(100, true, vec![]);

        let errors = account
            .execute_tx(TX::new(150, "Merchant X", DateTime::default()))
            .unwrap_err();

        let expected_errors = vec![OperationError::InsufficientLimit];

        assert_eq!(errors, expected_errors);
    }

    #[test]
    fn get_last_n_txs() {
        let account = Account::new(
            1000,
            true,
            vec![
                TX::new(100, "Merchant X", DateTime::default()),
                TX::new(50, "Merchant Y", DateTime::default()),
                TX::new(25, "Merchant Z", DateTime::default()),
                TX::new(200, "Merchant ZZ", DateTime::default()),
            ],
        );

        let (initial_tx, last_tx) = account.get_last_n_txs(3);

        assert_eq!(initial_tx, &TX::new(50, "Merchant Y", DateTime::default()));
        assert_eq!(last_tx, &TX::new(200, "Merchant ZZ", DateTime::default()));
    }

    #[test]
    fn map_to_invalid_state() {
        let account = Account::new(100, false, vec![]);

        let expected_state = AccountState::new(false, 100, vec![]);

        assert_eq!(account.to_state(), expected_state);
    }

    #[test]
    fn map_to_state() {
        let account = Account::new(100, false, vec![]);

        let expected_state = AccountState::new(false, 100, vec![]);

        assert_eq!(account.to_state(), expected_state);
    }

    #[test]
    fn is_inactive() {
        let inactive_account = Account::new(100, false, vec![]);
        let active_account = Account::new(100, true, vec![]);

        assert_eq!(inactive_account.is_inactive(), true);
        assert_eq!(active_account.is_inactive(), false);
    }
}

#[cfg(test)]
mod account_state {

    use super::*;

    use pretty_assertions::assert_eq;

    use crate::account::OperationError;

    #[test]
    fn create() {
        let state = AccountState::new(true, 123, vec![]);

        let expected_state = AccountState {
            active_card: true,
            available_limit: 123,
            violations: vec![],
        };

        assert_eq!(state, expected_state);
    }

    #[test]
    fn create_with_errors() {
        let state = AccountState::new(true, 123, vec![OperationError::DuplicatedTx]);

        let expected_state = AccountState {
            active_card: true,
            available_limit: 123,
            violations: vec![String::from("duplicated-tx")],
        };

        assert_eq!(state, expected_state);
    }

    #[test]
    fn create_not_initialized() {
        let state = AccountState::not_initialized();

        let expected_state = AccountState {
            active_card: false,
            available_limit: 0,
            violations: vec![String::from("account-not-initialized")],
        };

        assert_eq!(state, expected_state);
    }

    #[test]
    fn create_inactive() {
        let state = AccountState::inactive(100);

        let expected_state = AccountState {
            active_card: false,
            available_limit: 100,
            violations: vec![String::from("inactive-card")],
        };

        assert_eq!(state, expected_state);
    }
}

#[cfg(test)]
mod tx {

    use super::*;

    use chrono::DateTime;
    use pretty_assertions::assert_eq;

    #[test]
    fn create() {
        let datetime = DateTime::default();

        let tx = TX::new(100, "Merchant X", datetime);

        let expected_tx = TX {
            amount: 100,
            merchant: String::from("Merchant X"),
            time: datetime,
        };

        assert_eq!(tx, expected_tx);
    }

    #[test]
    fn get_seconds_since_another_tx() {
        let tx_one = TX::new(
            100,
            "Merchant X",
            DateTime::parse_from_rfc3339("2019-02-13T11:00:00.000Z")
                .unwrap()
                .into(),
        );

        let tx_two = TX::new(
            100,
            "Merchant X",
            DateTime::parse_from_rfc3339("2019-02-13T11:00:05.000Z")
                .unwrap()
                .into(),
        );

        let seconds = tx_two.seconds_since(&tx_one);

        let expected_seconds = 5;

        assert_eq!(seconds, expected_seconds);
    }
}
