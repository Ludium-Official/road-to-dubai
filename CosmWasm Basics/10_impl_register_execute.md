# Implement register execute

## Prior knowledge
- [03_state](./03_state.md)
- [05_message_and_event](./05_message_and_event.md)
- [06_query](./06_query.md)

## 0. register function
The register function registers the name entered by the user. The name must be unique and duplicate names cannot be registered.

## 1. Generating `ExecuteMsg` message
Add `ExecuteMsg` to the `msg.rs` file:
```rust
#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String },
}
```
- The `Register` type is added to the `ExecuteMsg` enumeration to define a message that a user can register a name.

## 2. Add `NameRecord` status
Add the `NameRecord` status to the `src/config.rs` file:
```rust
use cosmwasm_std::Addr;
use cw_storage_plus::Map;


#[cw_serde]
pub struct NameRecord {
    pub owner: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const NAME_RESOLVER: Map<&[u8], NameRecord> = Map::new("name_resolver");
```
- `NameRecord` is a structure that stores the address (Addr) of the owner of the name.
- `NameRecord` uses [`Map`](./21_state.md#2-map) to store the owner name and owner address in key-value form.

## 3. Implement register business logic
Create the default structure in the file `src/contract.rs`:
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { name } => todo!()
    }
}
```


Now let's write the `execute_register` function: 
```rust 
pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, StdError> {
    let key = name.as_bytes();
    let record = NameRecord { owner: info.sender };

    NAME_RESOLVER.save(deps.storage, key, &record)?;

    Ok(Response::default())
}
```
- This function takes the name registered in msg as a factor and registers it in the contract's internal state, `NameRecord`.
- In the form of `Map<Key, Value>`, the name entered with `String` is converted into `byte` and used as `key`, and the account address to which the name is registered is mapped and stored as `value`. Duplicate names cannot be registered because the name must be unique.

## 4. Name Create duplicate custom errors
If the name is repeatedly requested, you must return that it is an invalid request.

### 1. Add a library
First, add the `thiserror` library to the `Cargo.toml` file:
```
[package]
name = "nameservice"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
cosmwasm-std = "1.5.4"
cosmwasm-schema = "1.5.4"
cw-storage-plus = "0.13.4"
thiserror = "1.0.31" # 추가
``` 

### 2. Create a cusotom error 
Add a custom error to `src/error.rs` file:
```rust
// --- New!
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Name has been taken (name {name})")]
    NameTaken { name: String },
}
```
- `NameTaken` error is added to the `ContactError` enumeration to generate an error when a duplicate name is requested.

Now update the function `execute_register` to handle errors that occur when trying to register duplicate names: 
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> { // StdError -> ContractError Update!
	match msg {
        // --- Update! 
        ExecuteMsg::Register { name } => execute_register(deps, env, info, name),
    }
}

pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> { // StdError -> ContractError Update!
    let key = name.as_bytes();
    let record = NameRecord { owner: info.sender };

    // --- Add! 
    if (NAME_RESOLVER.may_load(deps.storage, key)?).is_some() {
        return Err(ContractError::NameTaken { name });
    }
    // ------
    
    NAME_RESOLVER.save(deps.storage, key, &record)?;

    Ok(Response::default())
}
```
- Change the error return type to `ContactError`.
- It adds business logic to prevent re-registration of already registered names.

## 5. Add `ResolveRecord` query
Now add a `ResolveRecord` query so that the registered name can be viewed.

### 1. Add `Resolve Record` type to `QueryMsg` message
Add `QueryMsg` and `ResolveRecordResponse` structures to the `msg.rs` file:
```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    // --- Add! 
    #[returns(ResolveRecordResponse)]
    ResolveRecord { name: String },
}

#[cw_serde]
pub struct ResolveRecordResponse {
    pub address: Option<String>,
}
```
- Add the `Resolve Record` type to `QueryMsg` so that the name can be searched.
- The `Resolve Record Response` structure is a structure that returns the inquiry result.

### 2. Implementing 'Resolve Record' query inquiry business logic
Add a query function to the file `src/contract.rs`:
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
	env: Env, // _env -> env 업데이트!
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary::<ConfigResponse>(&CONFIG.load(deps.storage)?.into()),
        // --- 추가! 
        QueryMsg::ResolveRecord { name } => query_resolver(deps, env, name),
    }
}

// --- Add! 
fn query_resolver(deps: Deps, _env: Env, name: String) -> StdResult<Binary> {
    let key = name.as_bytes();

    let address = match NAME_RESOLVER.may_load(deps.storage, key)? {
        Some(record) => Some(String::from(&record.owner)),
        None => None,
    };
    let resp = ResolveRecordResponse { address };

    to_json_binary(&resp)
}

```
- The `query_resolver` function queries the name and returns the address of the owner.

## 6. Business logic test
### 1. Create a test
Add the test code to the file `src/tests.rs`:
```rust
    #[test]
    fn register_available_name_and_query_works() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);

        // then
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }

    #[test]
    fn register_available_name_and_query_works_with_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));
        mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

        let info = mock_info("bob_key", &coins(5, "token"));
        let msg = ExecuteMsg::Register {
            name: "bob".to_string(),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Register message");

        // then
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
        assert_name_owner(deps.as_ref(), "bob", "bob_key");
    }

    #[test]
    fn fails_on_register_already_taken_name() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);
        
        // "alice" name redundant 
        let info = mock_info("bob_key", &coins(2, "token"));
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTaken { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }
        
        // "alice" name redundant
        let info = mock_info("alice_key", &coins(2, "token"));
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTaken { .. }) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    fn mock_alice_registers_name(deps: DepsMut, sent: &[Coin]) { 
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };
        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract successfully handles Register message");
    }

    fn assert_name_owner(deps: Deps, name: &str, owner: &str) {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::ResolveRecord {
                name: name.to_string(),
            },
        )
        .unwrap();

        let value: ResolveRecordResponse = from_json(&res).unwrap();
        assert_eq!(Some(owner.to_string()), value.address);
    }
```
- `register_available_name_and_query_works`: a test in which a user registers a new name and queries the registered name to identify the owner.
- `register_available_name_and_query_works_with_fees`: a test in which a user registers a new name and queries the registered name to identify the owner when a registration fee is required.
- `fails_on_register_already_taken_name`: a test to see if an appropriate error occurs when attempting to re-register an already registered name
- `mock_alice_registers_name`: It is a test helper function that registers a name with ice_key.
- `assert_name_owner`: A test helper function that verifies that the owner of the registered name is the expected owner.


## Wrap it up
It was implemented for register, which is an important function in nameservice, and added a function to query it to complete the test. However, there are still deficiencies in business logic.
- When the contract was initialized, the minimum amount of fee was entered, which requires prior verification.
- For data management, rules are required for length and string for the entered name.

Let's improve one by one on this.