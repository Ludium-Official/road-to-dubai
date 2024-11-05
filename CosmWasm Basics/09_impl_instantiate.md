# Implementing the Instantiate Function

## Prior knowledge
- [03_state](./03_state.md)
- [05_message_and_event](./05_message_and_event.md)
- [06_query](./06_query.md)

## 0. INSTANTATE FUNCTION
In the `instantiate` function, the value of `config` is initialized. Here, the values to be initialized are `purchase_price` and `transfer_price`.

## 1. Add a library
First, add the necessary library to the `Cargo.toml` file. `cw-storage-plus` is a library that manages the internal state of the contract, and `cosmwasm-schema` is a library that creates a schema along with a message serialization function:
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

```

## 2. Create `InstantiateMsg` message
Create `InstantiateMsg` on `src/msg.rs`: 
```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub purchase_price: Option<Coin>,
    pub transfer_price: Option<Coin>,
}
```
- Here, `Coin` is Cosmwasm's basic coin type, and [`cw_serde`](./23_message_and_event.md#cw_serde) helps to easily encode and decode the structure.


## 3. Add Config Status
Add `Config` status to file `src/config.rs`: 
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
- The state of `CONFIG` was implemented using [`Item`](./21_state.md#1-item).

## 4. Implementing of instantaneous business logic
Implement the `instantiate` function in the `src/contract.rs` file:
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

## 5. Add `Config` query 
### 1. Create `QueryMsg` message
Add  `QueryMsg`와 `ConfigResponse` structure to `msg.rs` file:
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

### 2. Implemeting a business logic `Config` query  
Add `query` function to `src/contract.rs` file:
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


## 6. Business logic test
Since it is inefficient in terms of resources to test the function by uploading the contract directly, it is recommended to carefully write the test to ensure that the functions implemented work normally.

### 1. Create a test
Try writing the test code in the file `src/tests.rs`:
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
- `proper_init_no_fees`: Check if `InstantiateMsg` handles no price information.
- `proper_init_with_fees`: Check if `InstantiateMsg` handles the case of including price information.
- `assert_config_state`: This is a helper function that checks the query result to see if the state is set as expected.
- `mock_init_with_price`: a helper function that calls the instantiate function including price information in `InstantiateMsg`.
- `mock_init_no_price`: a helper function that calls the instantiate function without price information in `InstantiateMsg`.

### 2. Running a test
Let's run the test code with `cargo test`. Then we can see that the test was executed normally as follows:
```sh
$ cargo test
running 2 tests
test tests::test_module::proper_init_no_fees ... ok
test tests::test_module::proper_init_with_fees ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


Now, it can be confirmed that the instantiate function and the query function operate normally. Next, let's implement the function of registering the name, the core logic of the nameservice.
