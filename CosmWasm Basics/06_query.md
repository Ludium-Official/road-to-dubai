# Query

## 0. Query
`Query` is one of the important ways to interact with smart contracts along with messages. Queries can be thought of as a way to read data from a database or query status. It is used not only for external clients (using the CLI) but also while executing contracts. For example, in the previous section, we discussed how to interpret names such as "Alice" or "Bob," which requires you to query other contracts.


First, we address two types of raw queries and custom queries, and examine the meaning of queries through external clients and from internal clients (other contracts).

## 1. Raw Query
The simplest query is raw read access to the key-value repository. If a caller (an external client or other contract) passes a raw binary key used for the contract's storage, it can easily return that raw binary value. The benefit of this approach is that it is very easy to implement and universal. The downside is that the caller is connected to the storage implementation and requires knowledge of the exact contracts that run.

This query is implemented inside the wasmd runtime and bypasses the VM. As a result, support from CosmWasm contracts is not required and all contract states are disclosed. These query_raw functions are exposed to all callers (external and internal).

## 2. Custom Query
Most queries are custom queries, which access the contract's data store in a read-only mode. For example, looking up the balance of a specific address or looking up token information. This has the advantage of not being strongly coupled to the implementation because it can rely on the interface. Each contract can process these custom queries by exposing the query function.

#### Custom Query Usage Example
Messages that can be used for queries are typically defined in the file `msg.rs` or `query.rs`, which depends on how the contract author structures the code. Queries can be performed by external clients (via API or CLI) or internal clients (to other contracts within the contract). Custom queries are defined as items in the [`QueryMsg`](./05_message.md#0-messages) enumeration and are processed by the contract's [query function](./04_entrypoint.md#0-entrypoint).

[The `QueryMsg`](./nameservice/src/msg.rs ) format of the nameservice contract is as follows:
```rust
#[cw_serde]
pub enum QueryMsg {
    ResolveRecord { name: String },
    Config {},
}
```

The contract handles this in [query function](./nameservice/src/contract.rs ):
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ResolveRecord { name } => query_resolver(deps, env, name),
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
    }
}
```
Here, `query_resolver` is another function, and `config_read` is a helper function that accesses the data storage. Custom queries are exposed to the outside through the `query function`. This allows the smart contract to respond to various data inquiry requests.

## 3. External Query
External queries are a common way for web and CLI clients to work with blockchains. They call CometBFT RPC, which is called abci_query of Cosmos SDK. There is an infinite gas limit in the query, which runs on only one node, so the entire blockchain cannot be slowed down. However, to prevent the problem of uploading wasm contracts with infinite loops and using them to DoS attack public RPC nodes that expose queries, we need to define fixed gas limits for query_custom transactions. It does not charge, but is used to limit abuse.

## 4. Internal Query
Interactions between contracts can be easily modeled by sending messages, but there are times when you want to synchronously query other modules without changing their status. For example, interpreting a name as an address or checking the state of KYC in another contract. This can be modeled as a series of messages, but it becomes very complex.

In fact, since this design violates one of the fundamental principles of the actor model, the principle that each contract has exclusive access to its internal state, this can lead to concurrent and re-entry problems.
- To address the concurrency issue, we provide Querier with read-only access to status snapshots immediately before execution of the current CosmWasm message. It is safe to take a snapshot, and both the running contract and the queried contract have read-only access to data before contract execution. The current contract is only written in the cache and flushed upon success.
- Another problem is that the reentry problem is that these queries are called synchronously, so they can be called back as a call contract, which can cause problems. Queries are not a big risk because they only have read-only access and cannot have side effects, but they are still a problem to consider.


## Resources
- https://docs.cosmwasm.com/docs/smart-contracts/message/submessage
- https://docs.cosmwasm.com/docs/architecture/query/