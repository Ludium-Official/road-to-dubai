# Implement transfer execute

## Prior knowledge
- [05_message_and_event](./05_message_and_event.md)

## 0. transfer function
The transfer function is the function of transferring a registered name to another user. This function changes ownership of the name and requires payment of a fee for transfer of ownership.

## 1. Add `Transfer` type to `ExecuteMsg` message
Add Transfer type to file `src/msg.rs`:
```rust
#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String },
	// --- Add!
    Transfer { name: String, to: String },
}
```
- The `Transfer` type is added to the `ExecuteMsg` enumeration to define the message that a user can send a name.

## 2. Add Custom Errors
Pre-defined custom errors for transfer functionality in file `src/error.rs`:
```rust
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // ...

    // --- Add!
    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Name does not exist (name {name})")]
    NameNotExists { name: String },
    // ------
}
```
- `Unauthorized` and `NameNotExists` errors are added to the `ContractError` enumeration to add errors that may occur during transmission.

## 3. Implement transfer business logic
Add `transfer` function to file `src/contract.rs`:
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { name } => execute_register(deps, env, info, name),
		// --- Add!
        ExecuteMsg::Transfer { name, to } => execute_transfer(deps, env, info, name, to),
    }
}

// --- New!
pub fn execute_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    to: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.transfer_price)?;

    let new_owner = deps.api.addr_validate(&to)?;
    let key = name.as_bytes();
    NAME_RESOLVER.update(deps.storage, key, |record| {
        if let Some(mut record) = record {
            if info.sender != record.owner {
                return Err(ContractError::Unauthorized {});
            }

            record.owner = new_owner.clone();
            Ok(record)
        } else {
            Err(ContractError::NameNotExists { name: name.clone() })
        }
    })?;
    Ok(Response::default())
}
```
- The `execute_transfer` function implements the ability to transfer names to other users.
- Check the set transmission fee and return the error if sufficient fee is not paid.
- Verify that the current owner of the name is the user who made the transfer request, or return an `Unauthorized` error.
- Returns `NameNotExists` error if the name does not exist.

## 4. Business logic test
### 1. Create a test
Add the test code to the file `src/tests.rs`:
```rust
#[test]
fn transfer_works() {
    let mut deps = mock_dependencies();
    mock_init_no_price(deps.as_mut());
    mock_alice_registers_name(deps.as_mut(), &[]);

    // Onwer of alice_key sends 'alice' to bob_key
    let info = mock_info("alice_key", &[]);
    let msg = ExecuteMsg::Transfer {
        name: "alice".to_string(),
        to: "bob_key".to_string(),
    };

    let _res = execute(deps.as_mut(), mock_env(), info, msg)
        .expect("contract successfully handles Transfer message");
    
    // then
    assert_name_owner(deps.as_ref(), "alice", "bob_key");
}

#[test]
fn transfer_works_with_fees() {
    let mut deps = mock_dependencies();
    mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));
    mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

    // Onwer of alice_key sends 'alice' to bob_key
    let info = mock_info("alice_key", &[coin(1, "earth"), coin(2, "token")]);
    let msg = ExecuteMsg::Transfer {
        name: "alice".to_string(),
        to: "bob_key".to_string(),
    };

    let _res = execute(deps.as_mut(), mock_env(), info, msg)
        .expect("contract successfully handles Transfer message");
    
    // then
    assert_name_owner(deps.as_ref(), "alice", "bob_key");
}

#[test]
fn fails_on_transfer_non_existent() {
    let mut deps = mock_dependencies();
    mock_init_no_price(deps.as_mut());
    mock_alice_registers_name(deps.as_mut(), &[]);

    // Onwer of frank_key sends unregistered 'alice42' to bob_key
    let info = mock_info("frank_key", &coins(2, "token"));
    let msg = ExecuteMsg::Transfer {
        name: "alice42".to_string(),
        to: "bob_key".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg);

    // then
    match res {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::NameNotExists { name }) => assert_eq!(name, "alice42"),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    assert_name_owner(deps.as_ref(), "alice", "alice_key");
}

#[test]
fn fails_on_transfer_from_nonowner() {
    let mut deps = mock_dependencies();
    mock_init_no_price(deps.as_mut());
    mock_alice_registers_name(deps.as_mut(), &[]);

    // Onwer of frank_key sends unowned 'alice' to bob_key
    let info = mock_info("frank_key", &coins(2, "token"));
    let msg = ExecuteMsg::Transfer {
        name: "alice".to_string(),
        to: "bob_key".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg);

    // then
    match res {
        Ok(_) => panic!("Must return error"),
        Err(ContractError::Unauthorized { .. }) => {}
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
    assert_name_owner(deps.as_ref(), "alice", "alice_key");
}

#[test]
fn fails_on_transfer_insufficient_fees() {
    let mut deps = mock_dependencies();
    mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(5, "token"));
    mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

    // Onwer of alice_key sends 'alice' to bob_key without the sufficient fund
    let info = mock_info("alice_key", &[coin(1, "earth"), coin(2, "token")]);
    let msg = ExecuteMsg::Transfer {
        name: "alice".to_string(),
        to: "bob_key".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg);

    // then
    match res {
        Ok(_) => panic!("register call should fail with insufficient fees"),
        Err(ContractError::InsufficientFundsSend {}) => {}
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
    assert_name_owner(deps.as_ref(), "alice", "alice_key");
}
```
- `transfer_works`: a test that verifies that the registered name is transmitted normally to another user. `alice_key` transmits the name `alice` to `bob_key` and checks whether ownership has been changed properly.
- `transfer_works_with_fees`: a test that checks whether the registered name is transmitted normally to another user in the case of a fee. `alice_key` transmits the name `alice` to `bob_key` and checks whether ownership has been changed properly.
- `fails_on_transfer_non_existent`: This is a test to see if an appropriate error occurs when attempting to send an unregistered name. Check if the `NameNotExists` error occurs when trying to transmit a name `alice42` that does not exist in `frank_key`.
- `fails_on_transfer_from_nonowner`: A test that checks whether an appropriate error occurs when a user other than the owner tries to transmit a name. Check whether an `unauthorized` error occurs when trying to transmit a name `alice` that is not owned by `frank_key` to `bob_key`.
- `fails_on_transfer_insefficient_fees`: This is a test to see if an appropriate error occurs when a name is sent when a sufficient fee has not been paid. If `alice_key` attempts to send the name `alice` to `bob_key` with an insufficient fee, it checks whether the `Insufficient FundsSend` error occurs.

```sh
$ cargo test

...
test tests::test_module::fails_on_transfer_insufficient_fees ... ok
test tests::test_module::fails_on_transfer_non_existent ... ok
test tests::test_module::fails_on_transfer_from_nonowner ... ok
test tests::test_module::transfer_works ... ok
test tests::test_module::transfer_works_with_fees ... ok
```

## Wrap it up
As above, the transfer function was implemented and verified through tests. Through this, ownership of the name can be safely transmitted to other users.

Now, all the core functions of namservice have been implemented. Now, we will proceed with how to create a schema and distribute it over the network.





