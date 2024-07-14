# 31a. instantiate 함수 구현하기
## 목차
0. instantiate 함수
1. 라이브러리 추가하기 
2. `InstantiateMsg` 메세지 생성하기
3. `Config` 상태 추가하기
4. instantiate 비즈니스 로직 구현하기
5. `Config` 쿼리 추가하기 
   1. `QueryMsg` 메세지 생성하기
   2. `Config` 쿼리 조회 비즈니스 로직 구현하기 
6. 비즈니스 로직 테스트 

## 사전 지식
- [21_state](./21_state.md)
- [23_message_and_event](./23_message_and_event.md)
- [24_query](./24_query.md)

## 0. instantiate 함수
`instantiate` 함수에서는 `config` 값을 초기한다. 여기서 초기화할 값은 `purchase_price`와 `transfer_price`이다.

## 1. 라이브러리 추가하기 
먼저, `Cargo.toml` 파일에 필요한 라이브러리를 추가한다. `cw-storage-plus`는 컨트랙트 내부 상태를 관리하는 라이브러리이고, `cosmwasm-schema`는 메세지 직렬화 기능과 함께 스키마를 생성해주는 라이브러리이다:
```
[package]
name = "namespace"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
cosmwasm-std = "1.5.4"
cosmwasm-schema = "1.5.4"
cw-storage-plus = "0.13.4"

```

## 2. `InstantiateMsg` 메세지 생성하기
`msg.rs`에 `InstantiateMsg`를 생성한다: 
```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub purchase_price: Option<Coin>,
    pub transfer_price: Option<Coin>,
}
```
여기서 `Coin`은 Cosmwasm의 기본 코인 타입이며, `cw_serde`는 구조체를 쉽게 인코딩 및 디코딩할 수 있도록 도와준다. 그리고 추후 사용할 schema 생성도 쉽게 할 수 있게 해준다.


## 3. Config 상태 추가하기
`src/config.rs` 파일에 `Config` 상태를 추가한다: 
```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub purchase_price: Option<Coin>,
    pub transfer_price: Option<Coin>,
}

pub const CONFIG: Item<Config> = Item::new("config");
```


## 4. instantiate 비즈니스 로직 구현하기
`src/contract.rs` 파일에 `instantiate` 함수를 구현한다:
```rust
use crate::{msg::InstantiateMsg, state::{Config, CONFIG}};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let config = Config {
        purchase_price: msg.purchase_price,
        transfer_price: msg.transfer_price,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}
```

## 5. `Config` 쿼리 추가하기 
### 1. `QueryMsg` 메세지 생성하기
`msg.rs` 파일에 `QueryMsg`와 `ConfigResponse` 구조체를 추가한다:
```rust
// --- New!
use crate::state::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub purchase_price: Option<Coin>,
    pub transfer_price: Option<Coin>,
}

impl From<Config> for ConfigResponse {
    fn from(config: Config) -> ConfigResponse {
        ConfigResponse {
            purchase_price: config.purchase_price,
            transfer_price: config.transfer_price,
        }
    }
}
```

### 2. `Config` 쿼리 조회 비즈니스 로직 구현하기 
`src/contract.rs` 파일에 `query` 함수를 추가한다:
```rust
use cosmwasm_std::to_json_binary;
use crate::msg::ConfigResponse;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
	_env: Env, 
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
		// --- 추가! 
        QueryMsg::Config {} => to_json_binary::<ConfigResponse>(&CONFIG.load(deps.storage)?.into()),
    }
}
```


## 6. 비즈니스 로직 테스트 
컨트랙트를 직접 업로드하여 기능을 테스트하기에는 리소스 측면에서 비효율적이기 떄문에 테스트를 꼼꼼하게 작성하여 구현한 함수들이 정상적으로 동작하는지 확인하는 것이 좋다. 

### 1. 테스트 작성하기 
`src/tests.rs` 파일에 테스트 코드를 작성해보자:
```rust
#[cfg(test)]
mod test_module {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, from_json, Coin, Deps, DepsMut};

    use crate::contract::{execute, instantiate, query};
    use crate::msg::{InstantiateMsg, QueryMsg};
    use crate::state::Config;

    #[test]
    fn proper_init_no_fees() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: None,
                transfer_price: None,
            },
        );
    }

    #[test]
    fn proper_init_with_fees() {
        let mut deps = mock_dependencies();

        mock_init_with_price(deps.as_mut(), coin(3, "token"), coin(4, "token"));

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: Some(coin(3, "token")),
                transfer_price: Some(coin(4, "token")),
            },
        );
    }

    fn assert_config_state(deps: Deps, expected: Config) {
        let res = query(deps, mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_json(&res).unwrap();
        assert_eq!(value, expected);
    }

    fn mock_init_with_price(deps: DepsMut, purchase_price: Coin, transfer_price: Coin) {
        let msg = InstantiateMsg {
            purchase_price: Some(purchase_price),
            transfer_price: Some(transfer_price),
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");

        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: None,
                transfer_price: None,
            },
        );
    }

    fn mock_init_no_price(deps: DepsMut) {
        let msg = InstantiateMsg {
            purchase_price: None,
            transfer_price: None,
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }
}
```

### 2. 테스트 실행하기 
`cargo test`로 테스트 코드를 실행해보자. 그러면 다음과 같이 정상적으로 테스트가 실행된 것을 확인할 수 있다:
```sh
$ cargo test
running 2 tests
test tests::test_module::proper_init_no_fees ... ok
test tests::test_module::proper_init_with_fees ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


이제 instantiate 함수와 query 함수가 정상적으로 동작함을 확인할 수 있다. 다음으로는 nameservice 핵심 로직인 name을 등록하는 기능을 구현헤보자. 

