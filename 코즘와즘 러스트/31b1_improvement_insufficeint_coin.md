# 31b1. register 비즈니스 로직 개선하기: 수수료 검증
## 목차
0. register 비즈니스 로직 개선하기: 수수료 검증
1. `InsufficientFundsSend` 커스텀 에러 추가하기
2. helper 로직 구현하기 
3. helper 로직 테스트 작성하기 
4. register 비즈니스 로직에 추가하기 
5. 비즈니스 로직 테스트 

## 0. register 비즈니스 로직 개선하기: 수수료 검증
컨트랙트를 초기화할 때 수수료 최소 금액을 입력했다. 그래서 우리는 `register` 함수가 실행될 때 사용자로부터 충분한 수수료가 지불되었는지 검증하는 로직을 추가하여 스마트 컨트랙트의 안정성을 향상시킬 것이다. 사전에 미리 검증하면 불필요한 연산을 일부 단축시킬 수 있다. 

## 1. `InsufficientFundsSend` 커스텀 에러 추가하기
먼저, `src/error.rs` 파일에 `InsufficientFundsSend` 커스텀 에러를 추가한다:
```rust
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Name has been taken (name {name})")]
    NameTaken { name: String },

    // --- 추가!
    #[error("Insufficient funds sent")]
    InsufficientFundsSend {},
}
```
- `ContractError` 열거형에 `InsufficientFundsSend` 에러를 추가하여, 충분한 수수료가 지불되지 않았을 때 발생하는 에러를 정의한다.

### 2. helper 로직 작성하기 
사용자가 제출한 코인의 양이 초기화 시 설정된 최소 양에 부합하는지 확인하는 helper 로직을 작성한다. 부합하지 않을 경우 `InsufficientFundsSend` 커스텀 에러를 반환한다:
```rust
use crate::error::ContractError;
use cosmwasm_std::Coin;

pub fn assert_sent_sufficient_coin(
    sent: &[Coin],
    required: Option<Coin>,
) -> Result<(), ContractError> {
    if let Some(required_coin) = required {
        let required_amount = required_coin.amount.u128();
        if required_amount > 0 {
            let sent_sufficient_funds = sent.iter().any(|coin| {
                // check if a given sent coin matches denom
                // and has sufficient amount
                coin.denom == required_coin.denom && coin.amount.u128() >= required_amount
            });

            if sent_sufficient_funds {
                return Ok(());
            } else {
                return Err(ContractError::InsufficientFundsSend {});
            }
        }
    }
    Ok(())
}
```
- 이 함수는 사용자가 제출한 코인 목록(sent)과 필요한 최소 코인(required)을 인자로 받아, 필요한 양만큼 코인이 제출되었는지 확인한다.
- 충분한 코인이 제출되지 않은 경우 `InsufficientFundsSend` 에러를 반환한다.

### 3. helper 로직 테스트 작성하기 
helper 로직에 대한 테스트 코드를 `src/helpers.rs` 파일에 작성한다:
```rust
#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::{coin, coins};

    #[test]
    fn assert_sent_sufficient_coin_works() {
        match assert_sent_sufficient_coin(&[], Some(coin(0, "token"))) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        match assert_sent_sufficient_coin(&[], Some(coin(5, "token"))) {
            Ok(()) => panic!("Should have raised insufficient funds error"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        match assert_sent_sufficient_coin(&coins(10, "smokin"), Some(coin(5, "token"))) {
            Ok(()) => panic!("Should have raised insufficient funds error"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        match assert_sent_sufficient_coin(&coins(10, "token"), Some(coin(5, "token"))) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };

        let sent_coins = vec![coin(2, "smokin"), coin(5, "token"), coin(1, "earth")];
        match assert_sent_sufficient_coin(&sent_coins, Some(coin(5, "token"))) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };
    }
}
```
- 이 테스트들은 `assert_sent_sufficient_coin` 함수가 다양한 상황에서 올바르게 동작하는지 확인한다.
- 테스트 케이스에는 충분한 수수료가 지불된 경우와 지불되지 않은 경우가 포함된다.

테스트 실행하면 다음과 같이 정상적으로 통과하는 것을 확인할 수 있다:
```sh
$ cargo test assert_sent_sufficient_coin_works

running 1 test
test helpers::test::assert_sent_sufficient_coin_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s
```

## 4. register 비즈니스 로직에 추가하기
이제 register 비즈니스 로직에 충분한 수수료 확인하는 helper 로직을 추가해보자:
```rust
pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    // --- 추가!
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.purchase_price)?;
    // ------

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
- 함수가 실행될 때, `assert_sent_sufficient_coin` 함수를 호출하여 사용자가 제출한 수수료가 충분한지 확인한다. 
- 충분한 수수료가 지불되지 않은 경우, `InsufficientFundsSend` 에러를 반환한다.

## 5. 비즈니스 로직 테스트 
충분한 수수료가 지불되지 않았을 때의 테스트를 작성한다:
```rust
 #[test]
    fn fails_on_register_insufficient_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));

        // 금액 미입력
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn fails_on_register_wrong_fee_denom() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));

        // denom이 다른 경우 
        let info = mock_info("alice_key", &coins(2, "earth"));
        let msg = ExecuteMsg::Register {
            name: "alice".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
```
- `fails_on_register_insufficient_fees`: 사용자가 충분한 수수료를 지불하지 않았을 때, register 함수가 실패하는지 확인한다.
- `fails_on_register_wrong_fee_denom`: 사용자가 잘못된 코인 종류로 수수료를 지불했을 때, register 함수가 실패하는지 확인한다.

테스트를 실행하여 모든 테스트가 정상적으로 통과하는지 확인한다:
```sh
$ cargo test
running 8 tests
...
test tests::test_module::fails_on_register_wrong_fee_denom ... ok
test tests::test_module::fails_on_register_insufficient_fees ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


## 마무리
이제 register 함수의 비즈니스 로직에서 수수료 검증 기능을 성공적으로 추가하고 테스트를 통해 검증했다. 이를 통해 스마트 컨트랙트의 안정성을 높이고, 불필요한 연산을 줄일 수 있게 되었다. 다음으로는 이름 길이와 잘못된 문자열을 사전에 필터링하는 기능을 추가하여 데이터 무결성을 유지하고 시스템의 안정성과 일관성을 보장할 것이다.





