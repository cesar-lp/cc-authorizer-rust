use std::{fs::File, io::{BufReader, BufRead, Error}};

use crate::models::Operation;

pub fn parse_file(filepath: &str) -> Result<Vec<Operation>, Error> {
    // let file = File::open("create_accounts.txt")?;
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut operations: Vec<Operation> = vec![];

    for line in reader.lines() {
        let file_line = line?;
        let operation: Operation = serde_json::from_str(&file_line).unwrap();

        match operation {
            Operation::Account(acc) => operations.push(Operation::Account(acc)),
            Operation::Transaction(tx) => operations.push(Operation::Transaction(tx)),
            _ => {
                println!("Invalid operation received");
            }
        }
    }

    return Ok(operations);
}
