use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::account::{Account, TX};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileOperation {
    #[serde(rename = "account")]
    CreateAccount(AccountData),
    #[serde(rename = "transaction")]
    ExecuteTX(TxData),
    Invalid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AccountData {
    available_limit: u32,
    active_card: bool,
}

impl AccountData {
    pub fn to_account(self) -> Account {
        Account::new(self.available_limit, self.active_card)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TxData {
    merchant: String,
    amount: u32,
    time: DateTime<Utc>,
}

impl TxData {
    pub fn to_tx(self) -> TX {
        TX {
            amount: self.amount,
            merchant: self.merchant,
            time: self.time,
        }
    }
}
