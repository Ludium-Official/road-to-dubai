# Improving register business logic: name input data verification

## 0. Improving register business logic: name input data verification
We will add a function to pre-filter the length of the requested name and the incorrect string. The reason for this is to maintain data integrity when stored and to prevent unnecessary data from being stored. This data management is important to maintain the stability and consistency of the system.

## 1. Add custom errors
Add the custom error required to verify the name input data to the file `src/error.rs`:
```rust
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Name has been taken (name {name})")]
    NameTaken { name: String },
    
    #[error("Insufficient funds sent")]
    InsufficientFundsSend {},

    // --- Add!
    #[error("Name too short (length {length} min_length {min_length})")]
    NameTooShort { length: u64, min_length: u64 },

    #[error("Name too long (length {length} min_length {max_length})")]
    NameTooLong { length: u64, max_length: u64 },

    #[error("Invalid character(char {c}")]
    InvalidCharacter { c: char },
    // ------
}
```
- `NameTooShort`, `NameTooLong`, and `InvalidCharacter` errors are added to define the error that occurs when the entered name is not valid.

## 2. Implement validate logic
Write the name verification logic in the file `src/contract.rs`:
```rust
// --- New!
const MIN_NAME_LENGTH: u64 = 3;
const MAX_NAME_LENGTH: u64 = 64;

fn validate_name(name: &str) -> Result<(), ContractError> {
    let length = name.len() as u64;
    if (name.len() as u64) < MIN_NAME_LENGTH {
        Err(ContractError::NameTooShort {
            length,
            min_length: MIN_NAME_LENGTH,
        })
    } else if (name.len() as u64) > MAX_NAME_LENGTH {
        Err(ContractError::NameTooLong {
            length,
            max_length: MAX_NAME_LENGTH,
        })
    } else {
        match name.find(invalid_char) {
            None => Ok(()),
            Some(bytepos_invalid_char_start) => {
                let c = name[bytepos_invalid_char_start..].chars().next().unwrap();
                Err(ContractError::InvalidCharacter { c })
            }
        }
    }
}

fn invalid_char(c: char) -> bool {
    let is_valid =
        c.is_ascii_digit() || c.is_ascii_lowercase() || (c == '.' || c == '-' || c == '_');
    !is_valid
}
```
- The function `validate_name` verifies the length and validity of the entered name.
- The function `invalid_char` examines characters that cannot be included in the name.
- The function `validate_name` returns an appropriate error if the name is too short or too long, or if it contains invalid characters.

## 3. Add to register business logic
Add the `validate_name` call to the `src/contract.rs` file:
```rust 
pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    // --- 추가!
    validate_name(&name)?;
    // ------
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.purchase_price)?;

    let key = name.as_bytes();
    let record = NameRecord { owner: info.sender };

    if (NAME_RESOLVER.may_load(deps.storage, key)?).is_some() {
        return Err(ContractError::NameTaken { name });
    }
    
    NAME_RESOLVER.save(deps.storage, key, &record)?;

    Ok(Response::default())
}
```
- The `execute_register` function is a function that processes the `register` message.
- When the function is executed, call the function `validate_name` to verify that the entered name is valid.
- Returns an appropriate error if an invalid name is entered.

## 4. Business logic test
Add a business logic test to the file `src/tests.rs`:
```rust
#[test]
fn register_available_name_fails_with_invalid_name() {
    let mut deps = mock_dependencies();
    mock_init_no_price(deps.as_mut());
    let info = mock_info("bob_key", &coins(2, "token"));

    // hi is too short
    let msg = ExecuteMsg::Register {
        name: "hi".to_string(),
    };
    match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::NameTooShort { .. }) => {}
        Err(_) => panic!("Unknown error"),
    }

    // 65 chars is too long
    let msg = ExecuteMsg::Register {
        name: "01234567890123456789012345678901234567890123456789012345678901234".to_string(),
    };
    match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::NameTooLong { .. }) => {}
        Err(_) => panic!("Unknown error"),
    }

    // no upper case...
    let msg = ExecuteMsg::Register {
        name: "LOUD".to_string(),
    };
    match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::InvalidCharacter { c }) => assert_eq!(c, 'L'),
        Err(_) => panic!("Unknown error"),
    }
    // ... or spaces
    let msg = ExecuteMsg::Register {
        name: "two words".to_string(),
    };
    match execute(deps.as_mut(), mock_env(), info, msg) {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::InvalidCharacter { .. }) => {}
        Err(_) => panic!("Unknown error"),
    }
}
```
- `register_available_name_fails_with_invalid_name`: When the entered name is invalid, check that the register function returns an appropriate error. The test case covers all cases where the name is too short or too long, and includes capital letters or spaces.

Run the test to ensure that all tests pass through normally:
```sh
$ cargo test
running 9 tests
...
test tests::test_module::register_available_name_and_query_works_with_fees ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Wrap it up
As above, logic was added to verify the validity of the input name and verified through tests. This allows you to maintain data integrity when stored and ensure system stability and consistency. Next, we will implement a new function called transfer that can transfer the name you own to another person.





