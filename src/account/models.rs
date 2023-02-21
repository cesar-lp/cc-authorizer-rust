use chrono::{DateTime, Utc};
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
                Box::new(InsufficientLimit {}),
                Box::new(HighFrequencySmallInterval {}),
                Box::new(DuplicatedTx {}),
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

#[derive(Debug)]
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
