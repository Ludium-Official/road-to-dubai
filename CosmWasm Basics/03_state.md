# Contact Status

## 0. State
The State is a store where smart contracts store and query data. It can be seen as similar to the DB interaction layer (e.g., form) in the existing app architecture. The simplest way to create a state is to create a single item.

For example, in the ['cw20-base'](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base) contract, 'TokenInfo' is created when the contract is instantiated. First, the type 'TokenInfo' is declared in ['state.rs'](https://github.com/CosmWasm/cw-plus/blob/main/contracts/cw20-base/src/state.rs):
```rust
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TokenInfo {
pub name: String,
pub symbol: String,
pub decimals: u8,
pub total_supply: Uint128,
pub mint: Option<MinterData>,
}
```

The 'TokenInfo' repository is then initialized:
```rust
pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
```

In the contract, you can find [code](https://github.com/CosmWasm/cw-plus/blob/main/contracts/cw20-base/src/contract.rs#L120-L128) ) that stores data in the corresponding repository in the 'instantiate' function:
```rust
let data = TokenInfo {
name: msg.name,
symbol: msg.symbol,
decimals: msg.decimals,
total_supply,
mint,
};
TOKEN_INFO.save(deps.storage, & data) ?;
```

## 1. cw-storage-plus
The following is a state store that provides productive abstraction on top of 'cosmwasm_std::Storage':
- `Item`: A typed wrapper surrounding a single database key, providing a helper function that allows you to interact without processing raw bytes.
- `Map`: Various types of objects can be stored under the prefix, and are indexed as simple (`&[u8]`) or complex (e.g., `(&[u8], &[u8]`) primary key).

This corresponds to 'Singleton' and 'Bucket' of cosmwasm_storage, but typing and gas consumption are low due to redesigned API and implementation.


### 1. Item
The usage of ['Item'](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/item.rs) is very simple. By entering the appropriate type and unique database key, an interface for data interaction is provided. When using the existing 'Single', the biggest change is as follows:
- Since the storage is no longer stored inside, a read/write version of the object is not required, and only one type is possible.
- Generating items using 'constfn' can be defined as global compilation time constants, saving gas and typing.

The following example declares 'Item' for the contract's 'Config' data:
```rust
const CONFIG: Item<Config> = Item::new("config");
```

### 2. Map
['Map'](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/map.rs) works like a storage-enabled BTreeMap, and key-value inquiry is possible with the input value. A complex key can be used by supporting a simple binary key ('&[u8]') and tuple. This enables efficient repetition and pageing of items at low gas costs. When using the existing 'Bucket', the biggest change is as follows:
- There is no need to store 'storage' inside, so separate read and write modifications are not required.
- Creating buckets using 'constfn' can save gas and typing.

The following is an example of declaring 'Map' of contract's 'NameRecord' data:
```rust
#[cw_serde]
pub struct NameRecord {
pub owner: Addr,
}

pub const NAME_RESOLVER: Map<&[u8], NameRecord> = Map::new("name_resolver");
```
- The types of [Key](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/keys.rs) that are basically supported include &[u8]', 'Addr', 'String', and '&str'.
- A composite key with a combination of (&str, &str) is also possible.
- 'iterator' can be used to efficiently search and process stored data.

In addition, 'Map' also has a function called [Path](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/path.rs), which represents a path to a specific key and provides a function to repeat all items under a specific prefix in the map, and [Prefix](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/prefix.rs), which increases efficiency by reusing the path.

### 3. IndexedMap
['cw721-base'](https://github.com/public-awesome/cw-nfts/blob/main/packages/cw721/src/state.rs) used by Contract ['IndexedMap'](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/indexed_map.rs) is a structure that enables efficient search and alignment by adding indexes to data stores. This is especially useful in situations where complex queries and quick queries are required. This works similarly to the index of the database, making it easy to search for data according to specific attributes.
- Data can be found quickly depending on specific attributes or keys.
- Because the data is indexed, sorting and filtering operations are performed quickly.
- It can efficiently handle complex queries based on multiple properties.

### 4. Deque
['Deque'](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/deque.rs) stores several items in a given key. This type provides efficient 'FIFO' and 'LIFO' access as well as direct index access.

## Resources
- https://github.com/CosmWasm/cw-storage-plus
- https://docs.cosmwasm.com/docs