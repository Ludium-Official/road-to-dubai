# Improving register business logic: fee verification

## 0. Improving register business logic: verifying fees
When initializing the contract, we entered the minimum amount of fee. So we will improve the stability of the smart contract by adding logic to verify that sufficient fees have been paid from the user when the 'register' function is executed. If you verify in advance, you can shorten some unnecessary operations.

## 1. Add 'Insufficient FundsSend' Custom Error
First, add the custom error `InsufficientFundsSend` to the `src/error.rs` file:
```rust
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Name has been taken (name {name})")]
    NameTaken { name: String },

    // --- Add!
    #[error("Insufficient funds sent")]
    InsufficientFundsSend {},
}
```
- Add the `Insufficient FundsSend` error to the `ContactError` enumeration to define the error that occurs when a sufficient fee is not paid.

### 2. Create helper logic
Write a helper logic that checks whether the amount of coins submitted by the user meets the minimum amount set at initialization. If not, return a custom error of `Insufficient FundsSend`:
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
- This function takes the user's submitted coin list (sent) and the minimum required coin as a factor and checks whether the coin has been submitted as much as necessary.
- Returns an `Insufficient FundsSend` error if not enough coins have been submitted.

### 3. Create a helper logic test
Write the test code for the helper logic in the file `src/helpers.rs`:
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
- These tests confirm that the `assert_sent_supervisor_coin` function works correctly in a variety of situations.
- The test case includes when a sufficient fee has been paid and when it has not been paid.

When the test is run, it can be confirmed that it passes normally as follows:
```sh
$ cargo test assert_sent_sufficient_coin_works

running 1 test
test helpers::test::assert_sent_sufficient_coin_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s
```

## 4. Add to register business logic
Now let's add helper logic to register business logic that checks sufficient fees:
```rust
pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    // --- Add!
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
- The `execute_register` function is a function that processes the `register` message.
- When the function is executed, call the `assert_sent_supervisor_coin` function to check whether the fee submitted by the user is sufficient.
- If a sufficient fee is not paid, return the `Insufficient FundsSend` error.

## 5. Business logic test
Write a test when a sufficient fee has not been paid:
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
- `fails_on_register_insufficient_fees`: When the user does not pay a sufficient fee, check whether the register function fails.
- `fails_on_register_wrong_fee_denom`: When a user pays a fee with the wrong coin type, check if the register function fails.

Run the test to ensure that all tests pass through normally:
```sh
$ cargo test
running 8 tests
...
test tests::test_module::fails_on_register_wrong_fee_denom ... ok
test tests::test_module::fails_on_register_insufficient_fees ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Wrap it up
Now, the fee verification function has been successfully added in the business logic of the register function and verified through tests. This has made it possible to increase the stability of smart contracts and reduce unnecessary operations. Next, it will maintain data integrity and ensure system stability and consistency by adding a function to filter name lengths and incorrect strings in advance.




