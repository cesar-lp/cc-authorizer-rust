use anyhow::{Context, Result};

use crate::account::{AccountState, Authorizer};
use crate::io::FileOperation;

pub fn parse_file(file_content: String) -> Result<Vec<AccountState>> {
    let mut op_executor = Authorizer::new();
    let mut account_states = vec![];

    for line in file_content.lines() {
        let operation: FileOperation = serde_json::from_str(&line)
            .with_context(|| format!("Invalid file operation '{}'", line))?;

        let account_state = match operation {
            FileOperation::CreateAccount(acc) => op_executor.create_account(acc.to_account()),
            FileOperation::ExecuteTX(tx) => op_executor.register_tx(tx.to_tx()),
        };

        account_states.push(account_state);
    }

    return Ok(account_states);
}

#[cfg(test)]
mod file_parser {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn handle_successful_file_operation() {
        let file_content = String::from(
            "{\"account\": {\"active-card\": true, \"available-limit\": 100}}
            {\"transaction\": {\"merchant\": \"Burger King\", \"amount\": 20, \"time\": \"2019-02-13T10:00:00.000Z\"}}"
        );

        let account_states = parse_file(file_content).unwrap();
        let expected_account_states = vec![
            AccountState::new(true, 100, vec![]),
            AccountState::new(true, 80, vec![]),
        ];

        assert_eq!(account_states, expected_account_states);
    }

    #[test]
    fn handle_invalid_file_operations() {
        let file_content = String::from("{\"invalid_op\": {}}");

        let result = parse_file(file_content);

        assert!(result.is_err());
    }
}
