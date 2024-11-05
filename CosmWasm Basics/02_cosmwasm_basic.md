# Cosmwasm Basic

## 0. Introduction to CosmWasm
Cosmos SDK is written in Golang, which has simple and easy grammar to allow app developers to freely customize modules, but supporting various programming languages is also important in terms of scalability. Finally, the Cosmos ecosystem is looking at the bigger goal of supporting numerous implementations and functions connected by Inter-Blockchain Communication Protocol (IBC) by activating blockchain Internet.

It marked the beginning of CosmWasm at the Cosmos HackAtom Berlin, held in June 2019 to extend tooling for developers interested in building on the Cosmos network. Cosmwasm, a project to activate WebAssembly (WASM) virtual machines (VMs) in the Cosmos SDK, was one of several projects that received grants from the Interchain Foundation regarding developer tooling.

The first implementation of the WASM virtual machine running on top of the Cosmos SDK application was designed by Ethan Frey of Confio. Adding WASM to the Cosmos SDK can safely execute software written in various languages on the blockchain. And Rust was selected as the first language.

### Why did you choose Rust as your first language for WASM Contract
An important consideration in performance when building a smart contract is the size of the data packet. Because WASM code provides more versatility, it is of course larger than EVM byte code specialized in smart contracts.

Rust does not have a GC and standard libraries can be excluded from the build, so a minimal simple escrow contract requires about 50kB (16kB when compressed). Golang or Haskell could be an alternative, but it is likely to create hundreds of KB contracts.

Due to this consideration and Rust popularity in the blockchain ecosystem, the TenderMint team decided to use it as the first implementation language for WASM contracts in Cosmos SDK.

## 1. Cosmwasm Features
Cosmwasm is a smart contract platform built for the Cosmos ecosystem. Cosmwasm is written as a [module] (../Cosm%20 basic/20_module_basic.md) that can be plugged into the Cosmos SDK. In other words, anyone who is currently building a blockchain using the Cosmos SDK can quickly and easily add Cosmwasm smart contract support to the chain without having to adjust the existing logic.

Cosmos networks are basically divided into application areas and consensus engine areas. Cosmwasm can bring great advantages to application areas by creating smart contracts. The reasons are as follows:
1. Developers can write modules that integrate smoothly with the Cosmos SDK as Rust, so they can develop Rust-based application logic while utilizing the Cosmos SDK module verified on the mainnet and the Tender Mint consensus algorithm.
2. New features can be deployed much faster because you can upload code from transactions without restarting the chain. Of course, changing the core logic requires a Cosmos Hub upgrade procedure.

### Cosmwasm Module(`x/wasm`)
Cosmwasm is another Cosmos SDK module, so the following dependency binary alone can initiate integration into the blockchain.
```go
// go.mod
require (
github.com/CosmWasm/wasmd v0.16.0
)
```

[Cosmos Hub](https://github.com/cosmos/gaia/blob/main/app/modules.go#L65) uses a cosmwasm sample binary called [wasmd](https://github.com/CosmWasm/wasmd). And there is a Cosmwasm smart contract platform chain called [Neutron](https://www.neutron.org/). Through this, the cosmwasm contract is distributed to the Cosm network, which can be initialized and queried and used.


## 2. Cosmos SDK and Cosmwasm Interaction
Let's get a rough idea of how the CosmWasm contract interacts with the Cosmos SDK. The CosmWasm contract performs two main tasks:
1. Update blockchain status by receiving 'DepsMut' (Execution)
2. Query blockchain status with read-only access to data (Query)

### 1. Execution
#### Role of Cosmos SDK
When a tendermint agreement is reached and the block is committed, the transactions are sequentially delivered to the Cosmos SDK and executed. The 'BaseApp' of the Cosmos SDK processes each transaction in a separate context:
- First, check all signatures and deduct gas charges. Then, set 'Gas Meter' to limit execution depending on the amount of gas paid. (See [Cosmos Basic/14.gas features](../Cosmos%20Basic/14_rpc_basic.md))
- It then creates a separate context to run the transaction. This allows the code to read the current state of the chain (after the last transaction has ended), but only write to the cache, allowing it to commit or roll back in the event of an error. ([Refer to Cosmos Basic/13.Store and Keepers' Read Caching and Write Branching](../Cosmos%20Basic/13_store_and_keepers.md))

Transactions can consist of multiple messages, and each message is executed in the same context and within Gas Limit in turn. Similar to the relational database ACID transaction method, atomicity is very important.

#### CosmWasm Contract Run (Basic Execution)
`x/wasm` is a user-defined Cosmos SDK module that processes messages in transactions and uses them to upload, instantiate, and execute smart contracts. If the contract's `execute` is executed, it receives an appropriately signed `MsgExecuteContract` and routes it to 'Keeper.Execute' and loads and executes an appropriate smart contract. This corresponds to the execution of a message in the transaction, which can lead to success or failure. If it fails, the entire transaction of the block will be rolled back.

The 'execute' function executed as a transaction message is provided as [Entrypoint](./22_entrypoint.md) when implementing the Cosmwasm contract:
```rust
pub fn execute(
deps: DepsMut,
env: Env,
info: MessageInfo,
msg: ExecuteMsg,
) -> Result<Response, ContractError> { }
```

It is possible to read and write the state and query the state of the module through `DepsMut`. It returns Ok (Response) or Err (ContractEr) when the task is completed. If the message is successful, the Response object is parsed and processed. However, if an error is returned, it is delivered to the SDK module as a string, leading to a rollback of the entire transaction of the block. If successful, the [Response object is returned and recorded as an event](./23_message_and_event.md).

#### Message Dispatch
If a function with a cross-contract call is executed, a message dispatch is made. CosmWasm Contract returns [`CosmosMsg`](./05_message.md#1-cosmmsg) to call other contracts or move tokens. If a contract returns two messages, M1 and M2, it is parsed and executed as the contract's authority in `x/wasm`:
- Upon success, the event is released and the returned message is processed.
- In the event of an error, the entire transaction is rolled back.

CosmosMsg is executed in depth priority. For example, if contract A returns M1 and M2, and contract B returns N1 and N2, the execution order is [M1 -> N1 -> N2 -> M2.

#### Sub-Message
['SubMessage'](./05_message.md#2-submessages) is a function that allows you to obtain a call result. A sub-message can capture an error result, store an error message without interrupting the entire transaction, and mark the message as executed.

When a sub-message is completed, the caller gets an opportunity to process the result. It contains both the original ID of the sub-call and the execution result. To save the additional state if necessary, you must save the local context in the store and load it from reply before returning the sub-message from the original execute. Sub-message execution and response are executed before the message. For example, contract A returns sub-messages S1 and S2, and message M1. If sub-message S1 returns a message N1, the execution sequence becomes [S1->N1 -> S2 -> reply(S2) -> M1].

### 2. Query
In some cases, information from other contracts needs to be accessed in the middle, such as a contract's Bank balance inquiry during execution. For this, it provides the function of performing synchronization calls during execution using a read-only querier. When performing a query, the [`QueryRequest` structure](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/query/mod.rs#L43-L71), which represents all possible calls, is serialized and delivered to the runtime through FFI to be interpreted and executed in the `x/wasm` SDK module. It can be extended to custom queries for each blockchain, just as `CosmosMsg` accepts customization. It also provides the ability to perform a raw protobuf "Stargate" query:
```rust
pub enum QueryRequest<C: CustomQuery> {
Bank(BankQuery),
Custom(C),
Staking(StakingQuery),
Distribution(DistributionQuery),
Stargate {
path: String,
data: Binary,
},
Ibc(IbcQuery),
Wasm(WasmQuery),
Grpc(GrpcQuery),
}
```

## Resources
- https://blog.cosmos.network/announcing-the-launch-of-cosmwasm-cc426ab88e12
- https://docs.cosmwasm.com/docs/