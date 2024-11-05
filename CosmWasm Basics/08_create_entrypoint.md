# Create EntryPoint

## Prior knowledge
- [04_entrypoint](./04_entrypoint.md)

## 0. Create an EntryPoint function
Previously, the overall project structure would have been created from project creation. The [EntryPoint](./04_entrypoint.md) function is managed by `contract.rs`.

### 1. `instantiate` EntryPoint
Now let's create EntryPoint at `src/contract.rs`:
```rust
use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Empty, StdResult, Response, entry_point
};
 
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: Empty,
) -> Result<Response, StdError> {
	Ok(Response::new())
}
```


Now, in turn, the most basic `entry_point` functions, `execute` and `query`, will also be made.

### 2. `execute` EntryPoint
Add `execute` function to `src/contract.rs`, which is the same location as `instantiat`' function:
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: Empty,
) -> Result<Response, StdError> {
	Ok(Response::new())
}
```
It looks similar to `instantiate` function. 

### 3. `query` EntryPoint
Add `query` function to `src/contract.rs` in the same way:
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    _deps: Deps,
	_env: Env, 
    _msg: Empty,
) -> StdResult<Binary> {
    Ok(Binary::default())
}
```
It can be seen that the `query` function has a different return type. The function always returns a binary object in the Ok case that contains only serialized responses. A general way to generate this is to call the `to_binary` method from an object implementing `serde::serialize`. This will be dealt with later by adding a query message.


## 1. Verifying after contract build
Now let's build the project and see if it has passed contract verification with `cosmwasm-check`:
```sh
$ cargo wasm
$ cosmwasm-check ./target/wasm32-unknown-unknown/release/nameservice.wasm
Available capabilities: {"cosmwasm_1_4", "staking", "stargate", "cosmwasm_1_2", "cosmwasm_1_3", "cosmwasm_2_0", "iterator", "cosmwasm_1_1"}

./target/wasm32-unknown-unknown/release/nameservice.wasm: pass

All contracts (1) passed checks!
```
- When `entry_point` functions are added, it can be confirmed that they pass normally.

## Wrap it up
Now, we have completed the creation of the EntryPoint function. Each function has only a basic structure, and we will update it little by little as we create actual business logic. The function to be implemented is the register function that registers and owns the name on the network and the transfer function that allows the name to be transmitted to others.


