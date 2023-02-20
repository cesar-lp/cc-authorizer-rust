use std::{fmt::Debug, vec};

use serde::{Deserialize, Serialize};

use super::OperationError;

#[derive(Debug, Serialize, Deserialize)]
pub struct TX {
    merchant: String,
    amount: u32,
    time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileOperation {
    #[serde(rename = "account")]
    CreateAccount(Account),
    #[serde(rename = "transaction")]
    ExecuteTX(TX),
    Invalid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Account {
    available_limit: u32,
    active_card: bool,
    #[serde(skip)]
    txs: Vec<TX>,
}

impl Account {
    pub fn new(available_limit: u32, active_card: bool) -> Self {
        Self {
            available_limit,
            active_card,
            txs: vec![],
        }
    }

    pub fn execute_tx(&mut self, tx: TX) {
        self.available_limit -= tx.amount;
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

#[derive(Debug)]
pub struct OperationExecutor {
    account: Option<Account>,
}

impl OperationExecutor {
    pub fn new() -> Self {
        Self {
            account: Option::None,
        }
    }

    pub fn create_account(&mut self, account: Account) -> AccountState {
        if self.account.is_some() {
            return account.to_invalid_state(vec![OperationError::AccountAlreadyInitialized]);
        }

        let state = account.to_state();

        self.account = Option::Some(account);

        return state;
    }

    pub fn register_tx(&mut self, tx: TX) -> AccountState {
        if self.account.is_none() {
            return AccountState::not_initialized();
        }

        let account = self.account.as_mut().unwrap();

        if account.is_inactive() {
            return AccountState::inactive(account.available_limit);
        }

        account.execute_tx(tx);

        let state = account.to_state();

        return state;
    }
}
