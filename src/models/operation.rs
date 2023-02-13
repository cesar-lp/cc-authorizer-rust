use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tx {
    merchant: String,
    amount: u32,
    time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Account(Account),
    Transaction(Tx),
    Invalid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Account {
    available_limit: u32,
    active_card: bool,
    #[serde(skip)]
    txs: Vec<Tx>,
}

impl Account {
    pub fn new(available_limit: u32, active_card: bool) -> Self {
        Self {
            available_limit,
            active_card,
            txs: vec![],
        }
    }

    pub fn execute_tx(&mut self, tx: Tx) {}
}

#[derive(Debug)]
pub struct AccountResult {
    active_card: bool,
    available_limit: u32,
    violations: Vec<String>,
}

impl AccountResult {
    pub fn new(acc: &Account) -> Self {
        Self {
            available_limit: acc.available_limit,
            active_card: acc.active_card,
            violations: vec![],
        }
    }
}

#[derive(Debug)]
pub struct OperationResult {
    account: AccountResult
}

impl OperationResult {
  pub fn new(account: &Account) -> Self {
    Self {
      account: AccountResult::new(account)
    }
  }
}