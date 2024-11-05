# Message and Event

## 0. Messages
Messages are ways to interact with CosmWasm smart contracts. Most contracts have files [`msg.rs `](./nameservice/src/msg.rs) that define messages. There are three main message types:
- Instantiate message (`InstantiateMsg`): This is sent when the contract is initialized. It usually contains the data needed to properly initialize the contract. It consists mostly of a simple structure.
- Execution message (`ExecuteMsg`) and query message (`QueryMsg`): Both are enum messages. These represent the message types used for execution and query, respectively.

### Message Definition Example
The following is a brief example of the 'execute' message definition in the nameservice contract (./nameservice/src/msg.rs ):
```rust
#[cw_serde]
pub enum ExecuteMsg {
Register { name: String },
Transfer { name: String, to: String },
}
```
- 'Register': Message to register a name
- 'Transfer': a message that sends a name to another address

#### `cw_serde`
['cw_serde'](https://github.com/CosmWasm/cosmwasm/blob/main/packages/schema-derive/src/cw_serde.rs) is a macro mainly used in CosmWasm smart contract development. It is based on Rust's 'serde' library and is used to encode and decode data in JSON format. This macro makes it easy to process messages and data structures. Its main functions are as follows:
- Automatic serialization and deserialization: The 'cw_serde' macro automatically implements the 'Serialize' and 'Deserialize' traces of the 'serde' library. Through this, data can be easily converted into JSON format.
- Schema creation: Automatically creates JSON schema using the 'chemars' library. This is useful for defining expected formats and types of message and data structures.

If you look at the schema folder of the nameservice contract (./nameservice/schema/), you can see the schema file created as 'cw_serde'.

### 메시지 정의 예시
다음은 nameservice 컨트랙의 간단한 [`execute` 메시지 정의 예시](./nameservice/src/msg.rs)이다:
```rust
#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String },
    Transfer { name: String, to: String },
}
```
- `Register`: 이름을 등록하는 메시지
- `Transfer`: 이름을 다른 주소로 전송하는 메시지

#### `cw_serde`
[`cw_serde`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/schema-derive/src/cw_serde.rs) is a macro mainly used in CosmWasm smart contract development. It is based on Rust's `serde` library and is used to encode and decode data in JSON format. This macro makes it easy to process messages and data structures. Its main functions are as follows:
- Automatic serialization and deserialization: The `cw_serde` macro automatically implements the `Serialize` and `Deserialize` traces of the `serde` library. Through this, data can be easily converted into JSON format.
- Schema creation: Automatically creates JSON schema using the `chemars` library. This is useful for defining expected formats and types of message and data structures.

If you look at [the schema folder of the nameservice contract](./nameservice/schema/), you can see the schema file created as `cw_serde`.

### Message Handling
These messages are processed in [execute function in file contract.rs](./nameservice/src/contract.rs ):
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
    ExecuteMsg::Transfer { name, to } => execute_transfer(deps, env, info, name, to),
  }
}
```
- The above `execute` function matches each variation of the `ExecuteMsg` enumeration and calls the corresponding function. For example, when the `ExecuteMsg:Register` message is received, the `Execute_register` function is called.

## 1. CosmosMsg
[`CosmosMsg`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/results/cosmos_msg.rs) is an enumeration that includes various message types. It is used to interact with other contracts or to perform certain tasks within the blockchain.
```rust
pub enum CosmosMsg<T = Empty> {
    Bank(BankMsg),
    Custom(T),
    Staking(StakingMsg),
    Distribution(DistributionMsg),
    Stargate {
        type_url: String,
        value: Binary,
    },
    Any(AnyMsg),
    Ibc(IbcMsg),
    Wasm(WasmMsg),
    Gov(GovMsg),
}
```

### 1. WasmMsg
[WasmMsg](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/results/cosmos_msg.rs#L180-L270) is a type of CosmosMsg that contains messages for interaction with other smart contracts. There are the following types:
```rust
pub enum WasmMsg {
    Execute {
        contract_addr: String,
        msg: Binary,
        funds: Vec<Coin>,
    },
    Instantiate {
        admin: Option<String>,
        code_id: u64,
        msg: Binary,
        funds: Vec<Coin>,
        label: String,
    },
    Migrate {
        contract_addr: String,
        new_code_id: u64,
        msg: Binary,
    },
    UpdateAdmin {
        contract_addr: String,
        admin: String,
    },
    ClearAdmin { contract_addr: String },
}
```
- `Execute`: Run a different contract
- `Instantiate`: Instantly instantiate a new contract
- `Migrate`: Migrate existing contracts
- `UpdateAdmin`: Update the contract's administrator
- `ClearAdmin`: Remove the contract's administrator

### 2. BankMsg
`BankMsg` contains a message that can transmit or incinerate the native token to another contract. This is not a true smart contract because it is processed by the blockchain itself, but it is useful for processing native tokens.

### 3. Custom
Custom exists to add messages that are processed by a particular blockchain. This makes it possible to define message types specific to the blockchain. For example, messages used only on other CosmWasm-based blockchains may be added.


## 2. SubMessages
Submessages are used to interact with SDK modules or other Cosmwasm smart contracts. `CosmosMsg` does not know if the call was successful, but SubMessage can be used to obtain the call result. This is useful in the following cases:
- When you instantiate a new contract and obtain a contract address
- When you run an action and verify that the results are successful (for example, make sure that a particular token has been sent to the contract)
- When you do not roll back a transaction by processing an error that occurred on a cross-contract call

To use Submessage, CosmosMsg must be wrapped in a SubMsg structure and transmitted. The SubMsg structure has the following fields:
```rust
pub struct SubMsg<T> {
    pub id: u64,                // reply_id used for processing the response
    pub msg: CosmosMsg<T>,      // Message to send
    pub gas_limit: Option<u64>, // Gas limit for the submessage
    pub reply_on: ReplyOn,      // Flag that determines when the response will be received
}
```

To prevent reentry attacks, CosmWasm does not allow context to be stored in contract memory. There are two ways to propagate states between contracts:
1. All events returned from SubMsg can be read from the Reply message.
2. Use [`cw_storage_plus:Item`] (./03_state.md#1-item) to save the temporary status and load it from the Reply handler.

### 1. Reply Strategy
Submessage offers a variety of options for different contracts to respond to. There are four Reply options:
```rust
pub enum ReplyOn {
    /// Perform Callback after SubMsg is processed
    Always,
    /// Callback if SubMsg returns error, no callback if successful
    Error,
    /// Callback if SubMsg suceeds no call back if error
    Success,
    /// No callback - the same as the original CosmoMsg
    Never,
}
```
- 

### 2. Reply Handler
To process responses from other contracts, you must implement one of the new [entry_point](./04_entrypoint.md) functions on the calling contract. Here is the [Reply example code](https://github.com/CosmWasm/cw-plus/blob/main/packages/utils/src/parse_reply.rs) ) that processes new contracts instantiated:
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // Save contract address after processing the message data
    let res = parse_reply_instantiate_data(msg)?;

    // Save res.contract_address
    Ok(Response::new())
}
```

## 3. Event
Most `entry_point` functions return the type `Result<Response, ContactError>`. Here [`Response` object](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/results/response.rs#L63-L87) is responsible for wrapping events in Cosmos SDK. Here are the components of the object:
```rust
pub struct Response<T = Empty> {
    pub submessages: Vec<SubMsg<T>>,
    pub messages: Vec<CosmosMsg<T>>,
    pub attributes: Vec<Attribute>,
    pub data: Option<Binary>,
}
```

The `Response` type is returned as a result if the message of the contract `entry_point` (e.g., `instantiate` or `execute`) is successful. It can be declared mutable so that it can be changed in the body of the function, but a more common pattern is to configure it at the end of the function and return it. In the example below, `Response` is wrapped in `Ok` because it is returned as part of the `Result` type. The exception is query, which returns `StdResult<Binary>` because of the Cosmos SDK interface.

Here are some of the simplest use examples of Response:
```rust
Ok(Response::default())
```

It is commonly used when the `instantiate` function does not return a message to the client. However, in most cases of `execute` processing, `Response` is returned:
```rust
let res = Response::new()
    .add_attribute("action", "transfer")
    .add_attribute("from", info.sender)
    .add_attribute("to", recipient)
    .add_attribute("amount", amount);
Ok(res)
```
- Create a `Response` and add several key-value pairs, wrap them in a result type and return them. By calling the contract through the CLI, it can be seen as part of the raw_log response.
- Instead of adding `attirbute`, you can also use `.add_event` to add unwrapped events. These events can interact with other clients or contracts.


## Resources
- https://docs.cosmwasm.com/docs/smart-contracts/message/submessage
