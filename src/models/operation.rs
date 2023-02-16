use std::vec;

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
    CreateAccount(Account),
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

    pub fn execute_tx(&mut self, tx: TX) {}
}

#[derive(Debug)]
pub struct AccountState {
    active_card: bool,
    available_limit: u32,
    violations: Vec<String>,
}

impl AccountState {
    pub fn new(acc: &Account) -> Self {
        Self {
            available_limit: acc.available_limit,
            active_card: acc.active_card,
            violations: vec![],
        }
    }
}

#[derive(Debug)]
pub struct OperationExecutor<'a> {
    account: Option<&'a Account>,
}

impl <'a> OperationExecutor<'a> {
    pub fn new() -> Self {
        Self {
            account: Option::None,
        }
    }

    pub fn create_account(&mut self, account: &'a Account) -> Result<AccountState, OperationError> {
        if self.account.is_some() {
            return Err(OperationError::AccountAlreadyInitialized);
        } else {
            self.account = Option::Some(account);
            Ok(AccountState::new(&account))
        }
    }

    pub fn register_tx(mut self, tx: TX) -> Result<AccountState, OperationError> {
        todo!()
    }
}
