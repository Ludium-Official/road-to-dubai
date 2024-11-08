# register 비즈니스 로직 개선하기: name 입력 데이터 검증

## 0. register 비즈니스 로직 개선하기: name 입력 데이터 검증
요청된 name의 길이와 잘못된 문자열을 사전 필터링하는 기능을 추가해 볼 것이다. 이렇게 하는 이유는 저장할 때 데이터 무결성을 유지하고, 불필요한 데이터가 저장되는 것을 방지하기 위함이다. 이러한 데이터 관리는 시스템의 안정성과 일관성을 유지하는 데 중요하다.

## 1. 커스텀 에러 추가하기
`src/error.rs` 파일에 name 입력 데이터 검증에 필요한 커스텀 에러를 추가한다:
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
- `NameTooShort`, `NameTooLong`, `InvalidCharacter` 에러를 추가하여 입력된 이름이 유효하지 않을 때 발생하는 에러를 정의한다. 

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
- `validate_name` 함수는 입력된 이름의 길이와 유효성을 검증한다.
- `invalid_char` 함수는 이름에 포함될 수 없는 문자를 검사한다.
- `validate_name` 함수는 이름이 너무 짧거나 너무 길 경우 또는 유효하지 않은 문자가 포함된 경우 적절한 에러를 반환한다. 

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
- `execute_register` 함수는 `register` 메시지를 처리하는 함수이다.
- 함수가 실행될 때, `validate_name` 함수를 호출하여 입력된 이름이 유효한지 검증한다.
- 유효하지 않은 이름이 입력된 경우, 적절한 에러를 반환한다. 

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
- `register_available_name_fails_with_invalid_name`: 입력된 이름이 유효하지 않을 때, register 함수가 적절한 에러를 반환하는지 확인한다. 테스트 케이스에는 이름이 너무 짧거나 너무 긴 경우, 대문자 또는 공백이 포함된 경우에 대해서 모두 커버한다.

테스트를 실행하여 모든 테스트가 정상적으로 통과하는지 확인한다:
```sh
$ cargo test
running 9 tests
...
test tests::test_module::register_available_name_and_query_works_with_fees ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 마무리
위와 같이, 입력된 이름의 유효성을 검증하는 로직을 추가하고 테스트를 통해 검증하였다. 이를 통해 저장할 때 데이터 무결성을 유지하고, 시스템의 안정성과 일관성을 보장할 수 있습니다. 다음으로는 소유한 name을 다른 사람에게 양도할 수 있는 transfer라는 새로운 기능을 구현할 것이다.






