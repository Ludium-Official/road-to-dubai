# 31c. transfer execute 구현하기
## 목차
0. transfer 기능 
1. `ExecuteMsg` 메세지에 `Transfer` 타입 추가하기
2. 커스텀 에러 추가하기
3. transfer 비즈니스 로직 구현하기
4. 비즈니스 로직 테스트

## 사전 지식
- [23_message_and_event](./23_message_and_event.md)

## 0. transfer 기능 
transfer 기능은 등록된 이름을 다른 사용자에게 전송하는 기능이다. 이 기능은 이름의 소유권을 변경하며, 소유권 이전 시 수수료를 지불해야 한다. 

## 1. `ExecuteMsg` 메세지에 `Transfer` 타입 추가하기
msg.rs 파일에 Transfer 타입을 추가한다:
```rust
#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String },
	// --- 추가!
    Transfer { name: String, to: String },
}
```

## 2. 커스텀 에러 추가하기
`src/error.rs` 파일에 `Unauthorized`와 `NameNotExists` 에러를 추가한다:
```rust
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // ...

    // --- 추가!
    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Name does not exist (name {name})")]
    NameNotExists { name: String },
    // ------
}
```

## 3. transfer 비즈니스 로직 구현하기
`src/contract.rs` 파일에 `transfer` 기능을 추가한다:
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
		// --- 추가!
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

## 4. 비즈니스 로직 테스트 
### 1. 테스트 작성하기
`src/tests.rs` 파일에 테스트 코드를 추가한다:
```rust
#[test]
fn transfer_works() {
    let mut deps = mock_dependencies();
    mock_init_no_price(deps.as_mut());
    mock_alice_registers_name(deps.as_mut(), &[]);

    // alice_key 보유자가 'alice'를 bob_key로 전송
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

    // alice_key 보유자가 'alice'를 bob_key로 전송
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

    // frank_key 보유자가 등록되지 않은 'alice42'를 bob_key로 전송
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

    // frank_key 보유자가 자신 보유하고 있지 않은 'alice'를 bob_key로 전송
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

    // alice_key 보유자가 'alice'를 bob_key로 충분하지 않은 금액과 함께 전송
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

### 1. 테스트 실행하기 
테스트를 실행하면 정상적으로 동작하는 것을 확인할 수 있다:
```sh
$ cargo test

...
test tests::test_module::fails_on_transfer_insufficient_fees ... ok
test tests::test_module::fails_on_transfer_non_existent ... ok
test tests::test_module::fails_on_transfer_from_nonowner ... ok
test tests::test_module::transfer_works ... ok
test tests::test_module::transfer_works_with_fees ... ok
```