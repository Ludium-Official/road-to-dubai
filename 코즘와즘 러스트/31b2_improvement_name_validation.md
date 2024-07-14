# 31b2. register 비즈니스 로직 개선하기: name 입력 데이터 검증
## 목차
0. register 비즈니스 로직 개선하기: name 입력 데이터 검증
1. 커스텀 에러 추가하기
2. validate 로직 구현하기 
3. register 비즈니스 로직에 추가하기 
4. 비즈니스 로직 테스트 

## 0. register 비즈니스 로직 개선하기: name 입력 데이터 검증
요청된 name의 길이와 잘못된 문자열을 사전 필터링하는 기능을 추가해 볼 것이다. 이렇게 하는 이유는 저장할 때 데이터 무결성을 유지하고, 불필요한 데이터가 저장되는 것을 방지하기 위함이다. 이러한 데이터 관리는 시스템의 안정성과 일관성을 유지하는 데 중요하다.

## 1. 커스텀 에러 추가하기
`src/error.rs` 파일에 `NameTooShort`, `NameTooLong`, `InvalidCharacter` 에러를 추가한다:
```rust
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Name has been taken (name {name})")]
    NameTaken { name: String },
    
    #[error("Insufficient funds sent")]
    InsufficientFundsSend {},

    // --- 추가!
    #[error("Name too short (length {length} min_length {min_length})")]
    NameTooShort { length: u64, min_length: u64 },

    #[error("Name too long (length {length} min_length {max_length})")]
    NameTooLong { length: u64, max_length: u64 },

    #[error("Invalid character(char {c}")]
    InvalidCharacter { c: char },
    // ------
}
```

## 2. validate 로직 구현하기 
`src/contract.rs` 파일에 name 검증 로직을 작성한다:
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

## 3. register 비즈니스 로직에 추가하기 
`src/contract.rs` 파일에 `validate_name` 호출을 추가한다:
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

## 4. 비즈니스 로직 테스트 
`src/tests.rs` 파일에 비즈니스 로직 테스트를 추가한다:
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

테스트를 실행하여 모든 테스트가 정상적으로 통과하는지 확인한다:
```sh
$ cargo test
running 9 tests
...
test tests::test_module::register_available_name_and_query_works_with_fees ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
