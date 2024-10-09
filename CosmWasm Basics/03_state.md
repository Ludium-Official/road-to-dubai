# Contract 상태

## 0. State
State(상태)는 스마트 컨트랙트가 데이터를 저장하고 조회하는 저장소이다. 기존 앱 아키텍처에서 DB 상호작용 계층(예, orm)과 비슷하다고 보면 된다. 상태를 작성하는 가장 간단한 방법은 단일 항목을 작성하는 것입니다. 

예를 들어, [`cw20-base`](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base) 컨트랙트에서는 컨트랙트가 인스턴스화될 때 `TokenInfo`가 작성된다. 먼저 [`state.rs`](https://github.com/CosmWasm/cw-plus/blob/main/contracts/cw20-base/src/state.rs)에서 `TokenInfo` 타입이 선언된다:
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

그런 다음 `TokenInfo` 저장소가 초기화된다:
```rust
pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
```

컨트랙트에서 `instantiate` 함수에서 데이터를 해당 저장소에 저장하는 [코드](https://github.com/CosmWasm/cw-plus/blob/main/contracts/cw20-base/src/contract.rs#L120-L128)를 확인할 수 있다:
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
다음은 `cosmwasm_std::Storage` 위에 생산적인 추상화를 제공하는 상태 저장소이다:
- `Item`: 하나의 데이터베이스 키를 둘러싼 타이핑된 wrapper로, 원시 바이트를 처리하지 않고도 상호 작용할 수 있는 helper 기능을 제공한다. 
- `Map`: prefix 아래에 다양한 타입의 객체를 저장할 수 있으며, 단순(`&[u8]`) 또는 복합(예: `(&[u8], &[u8])`) primary key(기본 키)로 인덱스된다. 

이는 cosmwasm_storage의 `Singleton`과 `Bucket`에 해당하지만, 재설계된 API와 구현으로 타이핑과 가스 사용량이 적다.


### 1. Item
[`Item`](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/item.rs)의 사용법은 매우 간단하다. 적절한 타입과 고유한 데이터베이스 키를 입력하면 데이터 상호 작용을 위한 인터페이스가 제공된다. 기존 `Single`을 사용하는 경우 가장 큰 변화는 다음과 같다:
- 더 이상 저장소를 내부에 저장하지 않으므로 객체의 읽기/쓰기 버전이 필요하지 않고 하나의 타입만으로도 가능하다.
- `const fn`을 사용하여 아이템을 생성하면 전역 컴파일 시간 상수로 정의할 수 있으므로 가스와 타이핑을 절약할 수 있다.

다음은 컨트랙트의 `Config` 데이터를 `Item`을 선언하는 예시이다:
```rust
const CONFIG: Item<Config> = Item::new("config");
```

### 2. Map
[`Map`](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/map.rs)은 스토리지 지원 BTreeMap처럼 작동하며, 입력된 값으로 Key-Value 조회가 가능하다. 간단한 바이너리 키(`&[u8]`)와 튜플을 지원하여 복합 키를 사용할 수 있다. 이를 통해 낮은 가스 비용으로 항목에 대한 효율적인 반복 및 페이지 매김이 가능하다. 기존 `Bucket`을 사용하는 경우 가장 큰 변화는 다음과 같다:
- 내부에 `Storage`를 저장할 필요가 없으므로 별도의 읽기 및 쓰기 변형이 필요하지 않다.
- `const fn`을 사용하여 버킷을 생성하면 가스와 타이핑을 절약할 수 있다.

다음은 컨트랙트의 `NameRecord` 데이터를 `Map`을 선언하는 예시이다:
```rust
#[cw_serde]
pub struct NameRecord {
    pub owner: Addr,
}

pub const NAME_RESOLVER: Map<&[u8], NameRecord> = Map::new("name_resolver");
```
- 기본적으로 지원하는 [Key](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/keys.rs) 종류로는 `&[u8]`, `Addr`, `String`, `&str` 등이 있다.
- (&str, &str)의 조합으로 하는 복합 키 또한 가능하다. 
- 저장된 데이터를 효율적으로 검색하고 처리하기 위해 `iterator`를 사용할 수 있다. 

또한 `Map`에는 특정 키에 대한 경로를 나타내며, 경로를 재사용하여 효율성을 높여주는 [`Path`](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/path.rs)와 맵의 특정 prefix 아래의 모든 항목을 반복하는 기능을 제공해주는 [`Prefix`](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/prefix.rs) 기능도 내부적으로 존재한다. 

### 3. IndexedMap
[`cw721-base`](https://github.com/public-awesome/cw-nfts/blob/main/packages/cw721/src/state.rs) 컨트랙트에서 사용하는 [`IndexedMap`](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/indexed_map.rs)은 데이터 저장소에 인덱스를 추가하여 효율적인 검색과 정렬을 가능하게 하는 구조이다. 특히, 복잡한 쿼리와 빠른 조회가 필요한 상황에서 유용하다. 이는 데이터베이스의 인덱스와 유사하게 작동하여, 특정 속성에 따라 데이터를 쉽게 검색할 수 있도록 한다.
- 특정 속성이나 키에 따라 데이터를 빠르게 찾을 수 있다.
- 데이터가 인덱싱되어 있어, 정렬 및 필터링 작업이 빠르게 수행된다.
- 여러 속성을 기반으로 하는 복잡한 쿼리를 효율적으로 처리할 수 있다.

### 4. Deque
[`Deque`](https://github.com/CosmWasm/cw-storage-plus/blob/main/src/deque.rs)는 주어진 키에 여러 항목들을 저장한다. 이 타입은 직접 인덱스 액세스뿐만 아니라 효율적인 `FIFO` 및 `LIFO` 액세스를 제공한다. 


## Resources
- https://github.com/CosmWasm/cw-storage-plus
- https://docs.cosmwasm.com/docs