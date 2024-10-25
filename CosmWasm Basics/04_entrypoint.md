# Entrypoint

## 0. Entrypoint
Entry point refers to the point at which a message or query is processed by the contract.

### 1.`entry_point` macro
Cosmwasm VMs running binary files cannot directly handle Rust types, so they handle function calls through [`entry_point` macro](https://github.com/CosmWasm/cosmwasm/blob/main/packages/derive/src/lib.rs#L49-L120) ). The function name corresponding to the `entry_point` macro has already been defined, so we know exactly what the VM should call.

The following is an example of using the `entry_point` macro:
```rust
#[entry_point]
pub fn instantiate(
deps: DepsMut,
env: Env,
info: MessageInfo,
msg: InstantiateMsg,
) -> Result<Response, StdError> {
// Initialization logic
Ok(Response::new())
}

#[entry_point]
pub fn execute(
deps: DepsMut,
env: Env,
info: MessageInfo,
msg: ExecuteMsg,
) -> Result<Response, StdError> {
// Execution Logic
Ok(Response::new())
}

#[entry_point]
pub fn query(
deps: Deps,
env: Env,
msg: QueryMsg,
) -> StdResult<Binary> {
// Query logic
Ok(Binary::default())
}

#[entry_point]
#[migrate_version(2)]
pub fn migrate(
deps: DepsMut,
env: Env,
msg: MigrateMsg,
) -> StdResult<Response> {
// Migration logic
Ok(Response::new())
}

#[entry_point]
pub fn reply(
deps: DepsMut,
_env: Env,
msg: Reply
) -> StdResult<Response> {
// reply logic
Ok(Response::new())
}
```

`#[cfg_attr(feature = "library"), entry_point)]` is a conditional attribute that adds the `entry_point` attribute only when the `library` function is not set. This is necessary to allow the contract to be used as a dependency on other contracts. The final binary should contain only one copy of each `entry_point`.

### 2. Functional factors
Each `entry_point` function has the following factors:
- `_deps`([`Deps`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/deps.rs#L25-L30) or [`DepsMut`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/deps.rs#L19-L23)): A gateway to the outside world of a smart contract context. It can access contract states, query other contracts, and provide API objects with several useful utility functions. 'DepsMut' can update states, while 'Deps' can only be read.
- `_env`([`Env`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/types.rs#L8-L16)): Deliver information on the state of the blockchain at the time of execution (height, execution timestamp, and execution contract itself).
- `_info`([`MessageInfo`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/types.rs#L83-L106)) : Information about contract calls, including the address to which the message is sent and the funds sent with the message.
- `_msg`([`Empty`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/results/empty.rs#L4-L11)): The message that triggered the execution is received as the last factor. It is now an Empty type representing {} JSON, but this can be changed to a type that can be decoded (such as 'InstantiateMsg', 'ExecuteMsg', etc.) during contract development in the future.

### 3. Return Type
The last part of `entry_point` is the return type. All `entry_point` returns the `Result` type with an error that can be converted into a string, and the returned error is simply recorded in case of contract failure.

#### Response type
The type `cosmwasm_std::Response` keeps everything you need to complete the contract execution. It has the following information:
- `events` field: This includes all events sent to the blockchain as a result of execution. The event has a very simple structure consisting of a list of attributes, a string type and a string key-value pair.
- `attributes` field: This is for convenience only. Most executions return only a single event, with a set of `attributes` directly in the response to make the event easier to manipulate. All of these attributes are converted into one wasm event and released. So it is sometimes considered as one with `event`.
- `SubMsg` field: This is a clue in cross-contract communication. These messages are sent to the contract after processing. Importantly, the entire execution is not completed unless processing of all sub-messages reserved by the contract is completed.
- `data` field: this is another binary field, just like the result of a query call, and usually contains serialized JSONs. All contract calls can return some additional information in any format.

### Binary type
The `query` function processes a query about the blockchain state and returns a result. This function must always return a serialized response. For this, the type `StdResult<Binary>` is used.


## Resources
- https://docs.cosmwasm.com/docs/smart-contracts/entry-points/
- https://github.com/CosmWasm/cosmwasm/blob/main/packages/derive/src/lib.rs