use crate::models::{Operation, OperationResult};

pub fn validate_operation(operations: Vec<Operation>) -> Vec<OperationResult> {
    let mut results: Vec<OperationResult> = vec![];

    for operation in operations.iter() {
        match operation {
            Operation::Account(acc) => {
                results.push(OperationResult::new(acc));
            }
            _ => {}
        }
    }

    return results;
}
