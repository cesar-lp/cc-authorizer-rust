use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use crate::models::{AccountState, FileOperation, OperationExecutor};

pub fn parse_file(filepath: &str) -> Result<Vec<AccountState>, Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut op_executor = OperationExecutor::new();
    let mut account_states = vec![];

    for line in reader.lines() {
        // let executor = &mut op_executor;
        let file_line = line?;
        let operation: FileOperation = serde_json::from_str(&file_line).unwrap();

        let account_state = match operation {
            FileOperation::CreateAccount(acc) => op_executor.create_account(acc),
            FileOperation::ExecuteTX(tx) => op_executor.register_tx(tx),
            _=> panic!("An invalid operation was found in file to be parsed")
        };

        account_states.push(account_state);
    }

    return Ok(account_states);
}
