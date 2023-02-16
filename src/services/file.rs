use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use crate::models::{AccountState, FileOperation, OperationError, OperationExecutor};

pub fn parse_file(filepath: &str) -> Result<Vec<AccountState>, Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let op_executor = OperationExecutor::new();

    let mut results = vec![];

    for line in reader.lines() {
        let file_line = line?;
        let operation: FileOperation = serde_json::from_str(&file_line).unwrap();

        let result = match operation {
            FileOperation::CreateAccount(acc) => op_executor.create_account(&acc),
            FileOperation::ExecuteTX(tx) => op_executor.register_tx(tx),
            _ => Err(OperationError::InvalidFileOperation),
        };

        results.push(result.unwrap());
    }

    return Ok(results);
}
