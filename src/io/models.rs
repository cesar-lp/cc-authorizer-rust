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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AccountData {
    available_limit: u32,
    active_card: bool,
}

impl AccountData {
    pub fn to_account(self) -> Account {
        Account::new(self.available_limit, self.active_card, vec![])
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

#[cfg(test)]
mod account_data {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn create_account() {
        let account_data = AccountData {
            available_limit: 100,
            active_card: true,
        };
        let expected_account = Account::new(100, true, vec![]);

        assert_eq!(account_data.to_account(), expected_account);
    }
}

#[cfg(test)]
mod tx_data {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn create_tx() {
        let tx_time = DateTime::parse_from_rfc3339("2019-02-13T11:00:00.000Z")
            .unwrap()
            .into();

        let tx_data = TxData {
            amount: 100,
            merchant: String::from("Nike"),
            time: tx_time,
        };
        let expected_tx = TX::new(100, "Nike", tx_time);

        assert_eq!(tx_data.to_tx(), expected_tx);
    }
}
