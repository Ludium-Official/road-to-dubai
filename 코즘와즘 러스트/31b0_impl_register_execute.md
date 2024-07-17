# 00. register execute 구현하기
## 목차
0. register 기능
1. `ExecuteMsg` 메세지 생성하기 
2. `NameRecord` 상태 추가하기
3. register 비즈니스 로직 구현하기
4. name 중복 커스텀 에러 생성하기
5. `ResolveRecord` 쿼리 추가하기
   1. `QueryMsg` 메세지에 `ResolveRecord` 타입 추가하기
   2. `ResolveRecord` 쿼리 조회 비즈니스 로직 구현하기 
6. 비즈니스 로직 테스트 

## 사전 지식
- [21_state](./21_state.md)
- [23_message_and_event](./23_message_and_event.md)
- [24_query](./24_query.md)

## 0. register 기능 
register 함수는 사용자가 입력한 이름을 등록하는 기능을 한다. 이름은 고유해야 하며, 중복된 이름은 등록할 수 없다.

## 1. `ExecuteMsg` 메세지 생성하기 
`msg.rs` 파일에 `ExecuteMsg`를 추가한다:
```rust
#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String },
}
```
- `ExecuteMsg` 열거형에 `Register` 타입을 추가하여, 사용자가 이름을 등록할 수 있는 메시지를 정의한다.

## 2. `NameRecord` 상태 추가하기
`src/config.rs` 파일에 `NameRecord` 상태를 추가한다:
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
- `NameRecord`는 이름을 소유하는 사용자(owner)의 주소(Addr)를 저장하는 구조체이다.
- `NameRecord`는 상태는 [`Map`](./21_state.md#2-map)을 사용하여 소유자 이름과 소유자 주소를 key-value 형태로 저장한다.

## 3. register 비즈니스 로직 구현하기
`src/contract.rs` 파일에 기본 구조를 작성한다:
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


이제 `execute_register` 함수를 작성해보자: 
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
- 이 함수는 msg에 등록된 name을 인자로 받아 이를 컨트랙트 내부 상태인 `NameRecord`에 등록한다. 
- `Map<Key, Value>` 형태로, `String`으로 입력된 이름을 `byte`로 변환하여 `key`로 사용하고, 그 이름을 등록한 계정 주소를 `value`로 매핑하여 저장한다. 이름은 고유해야 하기 때문에 중복된 이름은 등록할 수 없다.

## 4. name 중복 커스텀 에러 생성하기
name이 중복으로 요청되면 이에 대해 잘못된 요청임을 반환해줘야 한다. 

### 1. 라이브러리 추가하기
우선 `thiserror` 라이브러리를 `Cargo.toml` 파일에 추가한다:
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

### 2. 커스텀 에러 생성하기 
`src/error.rs` 파일에 커스텀 에러를 추가한다:
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
- `ContractError` 열거형에 `NameTaken` 에러를 추가하여, 중복된 이름이 요청될 때 에러를 발생시킨다.

이제 중복된 이름을 등록하려고 할 때 발생하는 에러를 처리하도록 `execute_register` 함수를 업데이트한다: 
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> { // StdError -> ContractError 업데이트!
	match msg {
        // --- 업데이트! 
        ExecuteMsg::Register { name } => execute_register(deps, env, info, name),
    }
}

pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> { // StdError -> ContractError 업데이트!
    let key = name.as_bytes();
    let record = NameRecord { owner: info.sender };

    // --- 추가! 
    if (NAME_RESOLVER.may_load(deps.storage, key)?).is_some() {
        return Err(ContractError::NameTaken { name });
    }
    // ------
    
    NAME_RESOLVER.save(deps.storage, key, &record)?;

    Ok(Response::default())
}
```
- 에러 반환타입을 `ContractError`로 변경해준다.
- 이미 등록된 이름에 대해서 재등록이 불가능하도록 비즈니스 로직을 추가해준다.

## 5. `ResolveRecord` 쿼리 추가하기
이제 `ResolveRecord` 쿼리를 추가하여 등록된 이름을 조회할 수 있도록 한다.

### 1. `QueryMsg` 메세지에 `ResolveRecord` 타입 추가하기
`msg.rs` 파일에 `QueryMsg`와 `ResolveRecordResponse` 구조체를 추가한다:
```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    // --- 추가! 
    #[returns(ResolveRecordResponse)]
    ResolveRecord { name: String },
}

#[cw_serde]
pub struct ResolveRecordResponse {
    pub address: Option<String>,
}
```
- `QueryMsg`에 `ResolveRecord` 타입을 추가하여 이름을 조회할 수 있도록 한다.
- `ResolveRecordResponse` 구조체는 조회 결과를 반환하는 구조체이다.

### 2. `ResolveRecord` 쿼리 조회 비즈니스 로직 구현하기 
`src/contract.rs` 파일에 쿼리 함수를 추가한다:
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

// --- 추가! 
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
- `query_resolver` 함수는 이름을 조회하여 해당 소유자의 주소를 반환한다.

## 6. 비즈니스 로직 테스트 
### 1. 테스트 작성하기
`src/tests.rs` 파일에 테스트 코드를 추가한다:
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
        
        // "alice" 이름 중복 
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
        
        // "alice" 이름 중복 
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
- `register_available_name_and_query_works`: 사용자가 새로운 이름을 등록하고, 등록된 이름을 쿼리하여 소유자를 확인하는 테스트이다.
- `register_available_name_and_query_works_with_fees`: 등록 수수료가 필요한 상황에서 사용자가 새로운 이름을 등록하고, 등록된 이름을 쿼리하여 소유자를 확인하는 테스트이다.
- `fails_on_register_already_taken_name`: 이미 등록된 이름을 다시 등록하려고 할 때, 적절한 에러가 발생하는지 확인하는 테스트이다
- `mock_alice_registers_name`: alice_key로 이름을 등록하는 테스트 helper 함수이다.
- `assert_name_owner`: 등록된 이름의 소유자가 예상한 소유자인지 확인하는 테스트 helper 함수이다.


## 마무리 
nameservice에서 중요한 기능인 register에 대해 구현하고 이를 쿼리하는 기능도 추가하여 테스트까지 완료했다. 그러나 아직 비즈니스 로직에서 부족한 부분이 있다.
- 컨트랙트를 초기화할 때 수수료 최소 금액을 입력하였는데 이에 대한 사전 검증이 필요하다.
- 데이터 관리를 위해 입력된 name에 대한 길이, 문자열에 대한 규칙이 필요하다.

이에 대해서 하나씩 개선해보록 하자.