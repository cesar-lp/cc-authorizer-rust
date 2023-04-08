# Coding Challenge - Authorizer

Your program is going to receive `json` lines as input through `stdin` and should provide
a `json` line output for each of the inputs through `stdout`. You can imagine this as a stream of events being processed by the authorizer.

## Execution

To run the application execute the following command
```
cargo run -- operation.txt
```

To run tests
```
cargo test
```

## Description

Given a file called operations that contains several lines describing operations in json format:
```
{"account": {"active-card": true, "available-limit": 100}}
{"transaction": {"merchant": "Burger King", "amount": 20, "time": "2019-02-13T10:00:00.000Z"}}
{"transaction": {"merchant": "Habbib's", "amount": 90, "time": "2019-02-13T11:00:00.000Z"}}
{"transaction": {"merchant": "McDonald's", "amount": 30, "time": "2019-02-13T12:00:00.000Z"}}
```

The application should be able to receive the file content through `stdin`, and for each processed operation return an output according to the business rules:
```
{"account": {"active-card": true, "available-limit": 100}, "violations": []}
{"account": {"active-card": true, "available-limit": 80}, "violations": []}
{"account": {"active-card": true, "available-limit": 80}, "violations": ["insufficient-limit"]}
{"account": {"active-card": true, "available-limit": 50}, "violations": []}
```

To run the application, execute the following command

```
cargo run -- *file-location.txt*  
```

## Operations

The program should handle two kinds of operations, deciding on which one to execute based on the line that is being processed:
1. Account creation
2. Transaction authorization for the account

### 1. Account creation

#### Input

Creates the account with the attributes `available-limit` and `active-card`. For simplicity's sake, we will assume that the Authorizer will deal with just one account.

#### Output

The created account's current state with all business logic violations. If no violations happen during operation processing, the field `violations` should return an empty vector `[]`.

#### Business Rules

Once created, the account should not be updated or recreated. If the application receives another account creation operation, it should return the following violation: `account-already-initialized`.

- Creating an account successfully
- Creating an account that violates the Authorizer logic

### 2. Transaction authorization

#### Input

Tries to authorize a transaction for a particular `merchant`, `amount` and `time` given the created account's state and last **authorized transactions**.

#### Output

The account's current state with any business logic violations. If no violations happen during operation processing, the field `violations` should return an empty vector `[]`.

#### Business Rules

You should implement the following rules, keeping in mind that **new rules will appear in the future**:
- No transaction should be accepted without a properly initialized account: `account-not-initialized`
- No transaction should be accepted when the card is not active: `inactive-card`
- The transaction amount should not exceed the available limit: `insufficient-limit`
- There should be no more than 3 transactions within a 2 minutes interval: `high-frequency-small-interval`
- There should be no more than 1 similar transaction (same `amount` and `merchant` ) within a 2 minutes interval: `duplicated-tx`
