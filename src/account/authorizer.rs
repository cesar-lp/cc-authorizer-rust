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
