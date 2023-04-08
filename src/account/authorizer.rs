use std::fmt::Debug;

use crate::account::{Account, AccountState, OperationError, TX};

#[derive(Debug)]
pub struct Authorizer {
    account: Option<Account>,
}

impl Authorizer {
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

        let result = account.execute_tx(tx);

        match result {
            Ok(account_state) => account_state,
            Err(errors) => AccountState::new(account.active_card, account.available_limit, errors),
        }
    }
}

#[cfg(test)]
mod authorizer {

    use chrono::DateTime;

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn create_account() {
        let mut authorizer = Authorizer::new();

        let state = authorizer.create_account(Account::new(1000, true, vec![]));

        let expected_state = AccountState::new(true, 1000, vec![]);

        assert_eq!(state, expected_state);
    }

    #[test]
    fn create_duplicated_account() {
        let mut authorizer = Authorizer {
            account: Some(Account::new(1000, true, vec![])),
        };

        let state = authorizer.create_account(Account::new(1000, true, vec![]));

        let expected_state =
            AccountState::new(true, 1000, vec![OperationError::AccountAlreadyInitialized]);

        assert_eq!(state, expected_state);
    }

    #[test]
    fn execute_tx_on_uninitialized_account() {
        let mut authorizer = Authorizer::new();

        let state = authorizer.register_tx(TX::new(500, "Merchant X", DateTime::default()));

        let expected_state = AccountState::not_initialized();

        assert_eq!(state, expected_state);
    }

    #[test]
    fn execute_tx_on_inactive_account() {
        let mut authorizer = Authorizer {
            account: Some(Account::new(1000, false, vec![])),
        };

        let state = authorizer.register_tx(TX::new(500, "Merchant X", DateTime::default()));

        let expected_state = AccountState::inactive(1000);

        assert_eq!(state, expected_state);
    }
}
