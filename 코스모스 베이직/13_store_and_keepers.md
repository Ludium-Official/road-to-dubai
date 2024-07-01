# 13. Store and Keepers
> cosmos-sdk v0.47부터 Tendermint에서 [CometBFT로 마이그레이션](https://github.com/cosmos/cosmos-sdk/issues/14870) 했기 때문에, 아티클은 cosmos-sdk v0.47, cometbft v0.38 기반으로 작성되었다. 

## 목차 
0. Store
1. KVStore
2. KVStore Wrapper
3. Multistore
4. Keeper


## 0. Store
Cosmos SDK는 앱 상태를 유지 관리하기 위해 Store 기능을 제공한다. 다음은 가장 기본적인 [`Store`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/types/store.go#L15-L18) 인터페이스를 나타낸다.
```go
type Store interface {
	GetStoreType() StoreType
	CacheWrapper
}
```
- `GetStoreType()`은 StoreType을 반환하는 Get 메서드이다.
- [`CacheWrapper`]((https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/types/store.go#L263-L287))는 `Write()` 메서드를 통해 Store 읽기 캐싱과 쓰기 브랜칭을 구현하는 간단한 인터페이스이다.

### Store 읽기 캐싱 & 쓰기 브랜칭
읽기 캐싱과 쓰기 브랜칭은 Cosmos SDK에서 보편적으로 사용되며 모든 `StoreType`에서 구현해야 한다. 브랜치는 기본 `Store`에 영향을 주지 않고 전달 및 업데이트할 수 있는 `Store`의 격리된 임시 브랜치를 생성한다. 이는 오류가 발생하면 나중에 쉽게 롤백할 수 있도록 하기 위해 임시 상태 전환을 하는 데 사용된다. 

다음은 `rootmulti.Store`가 구현한 캐싱 스토어를 생성하는 [CacheMultiStore()](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/rootmulti/store.go#L477-L491) 메서드이다:
```go
func (rs *Store) CacheMultiStore() types.CacheMultiStore {
	stores := make(map[types.StoreKey]types.CacheWrapper)
	for k, v := range rs.stores {
		store := types.KVStore(v)
		// Wire the listenkv.Store to allow listeners to observe the writes from the cache store,
		// set same listeners on cache store will observe duplicated writes.
		if rs.ListeningEnabled(k) {
			store = listenkv.NewStore(store, k, rs.listeners[k])
		}
		stores[k] = store
	}
	return cachemulti.NewStore(rs.db, stores, rs.keysByName, rs.traceWriter, rs.getTracingContext())
}
```

다음은 `cachemulti.Store`가 구현한 [Write() 메서드](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachemulti/store.go#L122-L128)이다:
```go
// Write calls Write on each underlying store.
func (cms Store) Write() {
	cms.db.Write()
	for _, store := range cms.stores {
		store.Write()
	}
}
```

### Commit Store
[`CommitStore`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/types/store.go#L29-L33)는 기본 트리 또는 DB에 대한 변경 사항을 커밋할 수 있는 Store를 나타낸다. Cosmos SDK는 `Committer`를 사용하여 기본 `Store` 인터페이스를 확장함으로써 simple store와 commit store를 구분해서 사용한다. 
```go
// Stores of MultiStore must implement CommitStore.
type CommitStore interface {
	Committer
	Store
}
```

[`Committer`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/types/store.go#L20-L27)는 디스크에 변경 사항을 지속하는 메서드를 정의하는 인터페이스이다.
```go
// something that can persist to disk
type Committer interface {
	Commit() CommitID 
	LastCommitID() CommitID

	SetPruning(pruningtypes.PruningOptions)
	GetPruning() pruningtypes.PruningOptions
}
```
- `CommitID`는 상태 트리의 결정론적 커밋이다. 해당 해시 값은 CometBFT와 같은 합의 엔진으로 반환되어 블록 헤더에 저장된다. 


`CommitStore` 인터페이스는 다양한 목적으로 존재하며, 그 중 하나는 어느 객체가 함부로 저장소에 커밋할 수 없도록 하기 위한 것이다. Cosmos SDK의 object-capabilities 모델의 일부로, 오직 `baseapp`만 저장소를 커밋할 수 있어야 한다. 모듈이 일반적으로 `Store`에 액세스하는 [`ctx.KVStore()`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/context.go#L281) 메서드가 `CommitKVStore`가 아닌 Commit 기능이 없는 `KVStore`를 반환하는 이유가 바로 이 때문이다.


## 1. KVStore
### IAVL Store 
`baseapp`에서 사용되는 `KVStore` 및 `CommitKVStore`의 기본 구현은 [`iavl.Store`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0-rc1/store/iavl/store.go#L37-L41)이다.
```go
type Store struct {
	tree   Tree
	logger log.Logger
}
```

iavl store는 다음을 보장하는 자체 밸런스를 조정하는 binary Tree인 IAVL Tree를 기반으로 한다:
- `Get` 및 `Set` 연산은 `O(log n)`이며, 여기서 n은 트리에 있는 요소의 수이다.
- 반복하면 범위 내에서 정렬된 요소를 효율적으로 반환한다.
- 각 트리 버전은 변경 불가능하며 커밋 후에도 (가지치기pruning 설정에 따라) 검색할 수 있다.


### KVStore
[`KVStore`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/types/store.go#L206-L242)는 데이터를 저장하고 검색하는 데 사용되는 간단한 키-값 저장소이다. `KVStore` 인터페이스는 주로 모듈이 `Committer`에 액세스하지 못하도록 제한하는 데 사용된다. 앱의 각 모듈은 자신만의 `KVStore`를 가지고 있으며, 해당 모듈 `Keeper`가 가지고 있는 특정 `key`를 통해서만 액세스할 수 있다. 이러한 `KVStore`들은 글로벌 상태의 하위 집합을 관리하는 데 사용된다. 
```go
type BasicKVStore interface {
	Get(key []byte) []byte
	Has(key []byte) bool
	Set(key, value []byte)
	Delete(key []byte)
}

type KVStore interface {
	Store
	BasicKVStore
	Iterator(start, end []byte) Iterator
	ReverseIterator(start, end []byte) Iterator
}
```
- `KVStore`는 `BasicKVStore` 인터페이스를 통해 `Get` 및 `Set` 메서드와 같은 기본 메서드를 구현한다.
- `Iterator` 객체를 반환하는 `Iterator(start, end)` 메서드는 일반적으로 공통 접두사(Prefix)를 공유하는 키 range를 반복하는 데 사용된다. 
	- 예시: 특정 account의 balance를 조회하기 위해 accountStore를 반복하는 `bank`의 모듈 keeper의 [IterateAccountBalances() 메서드](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/bank/keeper/view.go#L115-L132)


### CommitKVStore
[`CommitKVStore`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/types/store.go#L257-L261)는 `Committer`도 구현하는 `KVStore`이다. 
```go
// CommitKVStore is an interface for MultiStore.
type CommitKVStore interface {
	Committer
	KVStore
}
```

Commit 기능을 허용하는 `baseapp`의 기본 `CommitKVStore`에 [마운트된 스토어는 `CommitKVStore`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/rootmulti/store.go#L64)이다. `KVStore`의 키 리스트(`keys`)는 프록시로 선언되고 앱을 실행할 때 `MultiStore`에 마운트된다. 그리고 해당 키 리스트(`keys`)를 각 스토어를 관리하는 모듈 `keeper`에게도 전달한다. 

## 2. KVStore Wrapper
### CacheKVStore
[`cachekv.Store`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachekv/store.go#L26-L35)는 기본 `KVStore`에 버퍼링된 쓰기/캐시된 읽기 기능을 제공하는 `KVStore Wrapper`이다. 해당 Wrapper는 일반적으로 롤백이 가능한 임시 저장소가 필요할 때마다 사용된다.
```go
type Store struct {
	mtx           sync.Mutex
	cache         map[string]*cValue
	unsortedCache map[string]struct{}
	sortedCache   internal.BTree // always ascending sorted
	parent        types.KVStore
}
```
#### Get 메서드
[`Store.Get()`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachekv/store.go#L52-L68)은 먼저 `Store.cache`에 키와 연결된 값이 있는지 확인한다. 
- 값이 있으면 함수는 값을 반환한다. 
- 그렇지 않은 경우 함수는 `Store.parent.Get()`을 호출하고 결과를 `Store.cache`에 캐시한 다음 반환한다.

#### Set 메서드
[`Store.Set()`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachekv/store.go#L70-L79)은 키-값 쌍을 `Store.cache`로 설정한다. 
- [`cValue`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachekv/store.go#L21-L24)에는 캐시된 값이 기본 값과 다른지 여부를 나타내는 `dirty`라는 bool 필드가 있다. 
- `Store.Set()`이 새 쌍을 캐시할 때 `cValue.dirty`는 참으로 설정되므로 `Store.Write()`가 호출될 때 기본 스토어에 기록할 수 있다.

#### Iterator 메서드
[`Store.Iterator()`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachekv/store.go#L159-L162)는 캐시된 항목과 원본 항목 모두에서 순회해야 한다. [`Store.iterator()`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachekv/store.go#L169-L193)에서는 각각에 대해 두 개의 iterator가 생성되고 병합된다. 

### `GasKv` Store
Cosmos SDK 앱은 가스를 사용하여 리소스 사용량을 추적하고 스팸을 방지한다. [`GasKv.Store`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0-rc1/store/gaskv/store.go#L11-L17)는 저장소를 읽거나 쓸 때마다 자동으로 가스를 소비할 수 있는 `KVStore Wrapper`이다. Cosmos SDK 앱에서 스토리지 사용량을 추적하기 위해 선택한 솔루션이다.


### `TraceKv` Store
[`tracekv.Store`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0-rc1/store/tracekv/store.go#L20-L43)는 기본 `KVStore`에 운영 추적 기능을 제공하는 `KVStore Wrapper`이다. root `Multistore`에서 추적이 활성화된 경우 모든 `KVStore`에 Cosmos SDK에 의해 자동으로 적용된다.

### `Prefix` Store
[`prefix.Store`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0-rc1/store/prefix/store.go#L15-L21)는 기본 `KVStore`에 자동 키 접두사 기능을 제공하는 `KVStore Wrapper`이다.

### `ListenKv` Store
[`listenkv.Store`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0-rc1/store/listenkv/store.go#L11-L18)는 기본 KVStore를 통해 상태 수신 기능을 제공하는 `KVStore Wrapper`이다. 
- 상태 스트리밍 구성 중에 `StoreKey`가 지정된 모든 `KVStore`에 Cosmos SDK에 의해 자동으로 적용된다. 
- 상태 스트리밍 구성에 대한 추가 정보는 store/streaming/README.md에서 확인할 수 있다.

## 3. Multistore
### Multistore Interface
[`Multistore`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/types/store.go#L101-L133) 는 아래 인터페이스를 따르는 일종의 `KVStore`이다: 
```go
type MultiStore interface {
	Store
	CacheMultiStore() CacheMultiStore
	CacheMultiStoreWithVersion(version int64) (CacheMultiStore, error)

	GetStore(StoreKey) Store
	GetKVStore(StoreKey) KVStore

	TracingEnabled() bool

	// ...
}
```
- `TracingEnabled() == true` 추적이 활성화된 경우 `Multistore`를 브랜치하면 모든 기본 `KVStore`를 `TraceKv.Store`에 래핑한다.


각 Cosmos SDK 앱은 Root에서 `Multistore`의 확장형 구조인 저장소를 보유하여 상태를 유지 관리하게 된다. 

### CommitMultiStore
[`CommitMultiStore`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/types/store.go#L141-L203)는 `Multistore` 인터페이스에서 Commit 기능을 가지고 있는 `Committer`를 확장한 저장소이다. 
```go
type CommitMultiStore interface {
	Committer
	MultiStore
	snapshottypes.Snapshotter

	// ...
}
```

`CommitMultiStore` 인터페이스의 구체적인 구현은 [`rootMulti.Store`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/rootmulti/store.go)에서 볼 수 있다. `rootMulti.Store`는 여러 `KVStore`를 마운트할 수 있는 `DB`를 중심으로 구축된 base layer의 `Multistore`로, `baseapp`에서 사용되는 기본 `Multistore`이다.


### CacheMultistore
`rootMulti.Store`를 브랜칭 할 때마다 [`cachemulti.Store`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachemulti/store.go)가 사용된다. 여기서 Store 읽기 캐싱과 쓰기 브랜칭을 주로 사용한다고 볼 수 있다. 
- `cachemulti.Store`는 생성자에서 모든 서브스토어를 브랜치(각 서브스토어에 대해 가상 스토어를 생성)하여 [`Store.stores`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachemulti/store.go#L28)에 보관하고 모든 읽기 쿼리를 캐시한다. 
- [`Store.GetKVStore()`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachemulti/store.go#L163-L170)는 `Store.stores`에서 스토어를 반환한다. 
- [`Store.Write()`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/store/cachemulti/store.go#L122-L128)는 `Store.stores`에 포함된 모든 하위 스토어에서 `CacheWrap.Write()`를 재귀적으로 호출한다.

### Cosmos SDK의 Store 관리 
`baseapp`에서 사용되어 root `Store`라고도 잘 알려진 `CommitMultiStore`는 KVStore의 일종인 `MultiStore`와 `Commiter` 기능을 가지고 있다. 이러한 기능이 통합된 [`CommitKVStore`](./13_store_and_keeper.md#commitkvstore)를 통해 통해 여러 각 모듈의 데이터를 관리한다. `simapp`에 있는 [`NewSimapp 생성자`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/simapp/app.go#L214-L531)를 통해 `CommitMultiStore-`를 어떻게 생성 및 마운트하고 사용하는지 알아보자.

#### 1. 키 리스트(`keys`) 생성
`KVStoreKey`는 각 모듈별로 데이터를 저장할 `CommitKVStore`를 식별하기 위한 키입니다. 이를 생성하는 함수는 다음과 같다.
아래 코드에서 다양한 모듈의 `StoreKey`를 이용해 `KVStoreKey` 객체들을 생성하고 `keys`라는 맵에 저장한다:
```go
keys := sdk.NewKVStoreKeys(
    authtypes.StoreKey, banktypes.StoreKey, ...
)
```

#### 2. 모듈 초기화
각 모듈의 keeper를 초기화할 때, 키 리스트(`keys`)에서 해당 모듈의 `StoreKey`를 꺼내어 초기화한다. 예를 들어, `BankKeeper`는 다음과 같이 초기화된다:
```go
app.BankKeeper = bankkeeper.NewBaseKeeper(
		appCodec,
		keys[banktypes.StoreKey],
		app.AccountKeeper,
		BlockedAddresses(),
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
```

#### 3. CommitMultiStore에 키 마운트
그리고 최종적으로 각 상태를 효율적으로 관리하고, 다양한 모듈이 독립적으로 자신의 데이터를 저장하고 액세스할 수 있도록 하기 위해서 `CommitMultiStore`에 `KVStoreKey`로 생성한 키 리스트(`keys`) 마운트한다. 이를 통해 각 모듈은 자신만의 `CommitKVStore`를 가지게 된다:
```go
app.cms.MountStoreWithDB(keys)

func (rs *Store) MountStoreWithDB(key types.StoreKey) {
	rs.storesParams[key] = newStoreParams(key, db, typ, 0)
	rs.keysByName[key.Name()] = key
}
```

## 4. Keeper
Cosmos SDK 앱은 일반적으로 여러 모듈로 구성된다. 각 모듈은 특정 도메인 로직을 처리하며, 각 모듈의 상태는 `Keeper`를 통해 관리된다.
- 전체 앱 상태: `CommitMultiStore`를 구현한 `rootMulti.Store`를 통해 관리된다. 이는 블록체인의 전체 상태를 나타낸다.
- 하위 집합 상태: 각 모듈은 자신 도메인에 맞는 상태를 다루며, 이는 `Keeper`를 통해 액세스하여 관리된다.


`Keeper`는 말 그대로 모듈의 `Store`에 대한 gatekeeper로, 하위 집합 상태에 대한 액세스를 관리하는 Cosmos SDK 앱의 추상화된 코드이다. 모듈의 상태 데이터에 대한 모든 액세스는 해당 모듈의 `Keeper`를 거쳐야 한다.
- 모듈 내에서 다루는 각 `Store`(일반적으로 `IAVL Store`)에는 액세스 권한을 부여하는 `storeKey`가 제공된다. 
- 모듈의 `Keeper`는 외부에 노출되지 않은 상태로 유지되어야 하는 이 `storeKey`를 통해, 해당 모듈 `Store`에 대한 읽기 및 쓰기 메서드를 정의한다.

예를 들어, BankKeeper의 경우 다음과 같은 역할을 한다:
- [송금(Send)](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/bank/keeper/keeper.go#L38-L40): 계정과 모듈를 대상으로 토큰을 송금하는 기능을 제공한다. 
- [계정 잔액 조회(GetBalance)](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/bank/keeper/view.go#L21-L34): 특정 계정의 잔액을 조회한다. 


### object-capabilities 모델 
`Keeper`를 보면 객체지향 프로그래밍(OOP)에서 주로 사용하는 [Repository 패턴](https://martinfowler.com/eaaCatalog/repository.html)과 비슷한 구조로 `KVStore`의 상태를 관리하며 다른 모듈과 상호작용하는 것을 볼 수 있는데, 이는 Cosmos SDK는 개발자가 원치 않는 모듈 간 상호 작용으로부터 앱을 더 잘 보호할 수 있도록 [object-capabilities](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/docs/docs/core/10-ocap.md) 기반 접근 방식을 채택했기 때문이다. 

object-capabilities은 코드 설계의 모듈성과 코드 구현의 안정적인 캡슐화를 보장한다는 장점이 있다. 이는 앱 상태를 업데이트하는 객체의 동작을 결정하는 코드를 객체 참조 및 연결하는 추상화된 수준에서 분석할 수 있다. 결과적으로 이러한 새로운 모듈이 추가되어도 쉽게 디버깅하고 잘 유지 관리될 수 있게 된다. 


# Resources
- https://docs.cosmos.network/main/
- https://docs.cosmos.network/v0.47/learn/advanced/baseapp#state-updates
- https://en.wikipedia.org/wiki/Object-capability_model
- https://ida.interchain.io/academy/2-cosmos-concepts/7-multistore-keepers.html
- https://github.com/cosmos/cosmos-sdk/blob/main/docs/learn/advanced/04-store.md