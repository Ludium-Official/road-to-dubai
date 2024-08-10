# Cosmwasm Basic

## 0. Cosmwasm 소개
Cosmos SDK는 앱 개발자들이 자유롭게 모듈 커스텀을 가능하기 위해 간단하고 쉬운 문법을 가지고 있는 Golang으로 작성되었지만, 다양한 프로그래밍 언어를 지원하는 것은 확장성 면에서도 중요하다. 최종적으로는 Cosmos 생태계는 블록체인 인터넷(Interchain)을 활성화하여 Inter-Blockchain Communication Protocol(IBC)로 연결된 수많은 구현 및 기능을 지원하는 더 큰 목표를 바라보고 있다. 

2019년 6월 Cosmos 네트워크에서 구축에 관심이 있는 개발자를 위한 툴링 확장을 위해 개최된 Cosmos HackAtom Berlin에서 CosmWasm의 시작을 알렸다. Cosmos SDK에서 WebAssembly(WASM) 가상 머신(VM)을 활성화하는 프로젝트인 Cosmwasm은 개발자 툴링과 관련하여 인터체인 재단으로부터 보조금을 받게 된 여러 프로젝트 중 하나였다.

Cosmos SDK 애플리케이션 위에서 실행되는 WASM 가상 머신의 첫 번째 구현은 Confio의 Ethan Frey가 설계했다. Cosmos SDK에 WASM을 추가하면 다양한 언어로 작성된 소프트웨어가 블록체인에서 안전하게 실행될 수 있다. 그리고 그 첫 번째 언어로 Rust가 선택되었다. 

### WASM 컨트랙트 첫 번째 언어로 Rust를 선택한 이유 
스마트 컨트랙트 구축할 때 성능에 있어 중요한 고려 사항은 데이터 패킷 크기이다. WASM 코드는 더 많은 범용성을 제공하기 때문에 당연히 스마트 컨트랙트에 특화되어 제작된 EVM 바이트코드보다는 사이즈가 크다. 

Rust에는 GC가 없고 표준 라이브러리를 빌드에서 제외할 수 있으므로 최소화된 단순 에스크로 컨트랙트에는 약 50kB(압축 시 16kB)가 필요하다. Golang이나 Haskell도 대안이 될 수 있지만, 수백 KB의 컨트랙트를 생성할 가능성이 높다.

이러한 고려와 블록체인 생태계에서 Rust 인기로 인해 텐더민트 팀은 이를 Cosmos SDK의 WASM 컨트랙트를 위한 첫 번째 구현 언어로 사용하기로 결정했다.

## 1. Cosmwasm 특징
Cosmwasm은 코스모스 생태계를 위해 구축된 스마트 컨트랙트 플랫폼이다. Cosmwasm은 Cosmos SDK에 플러그인할 수 있는 [모듈](../코스모스%20베이직/20_module_basic.md)로 작성되었다. 즉, 현재 Cosmos SDK를 사용해 블록체인을 구축 중인 누구나 기존 로직을 조정하지 않고도 빠르고 쉽게 Cosmwasm 스마트 컨트랙트 지원을 체인에 추가할 수 있다.

Cosmos 네트워크는 기본적으로 애플리케이션 영역과 합의 엔진 영역으로 나뉘어진다. Cosmwasm은 스마트 컨트랙트를 작성을 통해 애플리케이션 영역에 큰 이점을 가져다줄 수 있다. 그 이유는 다음과 같다:
1. 개발자는 Cosmos SDK와 원활하게 통합되는 모듈을 Rust로 작성할 수 있으므로, 메인넷에서 검증된 Cosmos SDK 모듈과 텐더민트 합의 알고리즘을 활용하면서 Rust 기반의 애플리케이션 로직을 개발할 수 있다. 
2. 체인을 재시작하지 않고 트랜잭션에서 코드를 업로드할 수 있기 때문에 새로운 기능을 훨씬 빠르게 배포할 수 있다. 물론, 핵심 로직을 변경할 때는 Cosmos Hub 업그레이드 절차가 필요하다. 

### Cosmwasm 모듈(`x/wasm`)
Cosmwasm은 또 다른 Cosmos SDK 모듈이므로 다음과 같은 의존성 바이너리 하나만으로도 블록체인에 통합을 시작할 수 있다. 
```go
// go.mod 
require (
    github.com/CosmWasm/wasmd v0.16.0
)
```

[Cosmos Hub](https://github.com/cosmos/gaia/blob/main/app/modules.go#L65)에서는 [wasmd](https://github.com/CosmWasm/wasmd)라는 cosmwasm 샘플 바이너리를 사용하고 있다. 그리고 [Neutron](https://www.neutron.org/)이라는 Cosmwasm 스마트 컨트랙트 플랫폼 체인이 있다. 이를 통해 코스모스 네트워크에 cosmwasm 컨트랙트를 배포하며, 초기화 및 쿼리해서 사용할 수 있다. 


## 2. Cosmos SDK와 Cosmwasm의 상호작용 
CosmWasm 컨트랙트가 Cosmos SDK와 어떻게 상호작용하는지 대략적으로 알아보자. CosmWasm 컨트랙트는 두 가지 주요 작업을 수행한다:
1. `DepsMut`을 받아 블록체인 상태 업데이트하기 (Execution)
2. 데이터를 읽기 전용으로 액세스하여 블록체인 상태 쿼리하기 (Query)

### 1. Execution
#### Cosmos SDK 의 역할 
텐더민트 합의를 이뤄 블록이 커밋되면, 트랜잭션은 차례로 Cosmos SDK에 전달되어 실행된다. Cosmos SDK의 `BaseApp`은 각 트랜잭션을 분리된 컨텍스트에서 처리한다:
- 먼저 모든 서명을 확인하고 가스 요금을 공제한다. 그런 다음 `Gas Meter`를 설정하여 지불된 가스의 양에 따라 실행을 제한한다. ([코스모스 베이직/14.gas fees 참고](../코스모스%20베이직/14_rpc_basic.md))
- 그런 다음 트랜잭션을 실행할 분리된 컨텍스트를 만든다. 이는 코드가 체인의 현재 상태를 읽을 수 있게 하되 (마지막 트랜잭션이 끝난 후), 캐시에만 기록할 수 있게 하여 오류 시 커밋하거나 롤백할 수 있게 한다. ([코스모스 베이직/13.Store and Keepers의 읽기 캐싱 및 쓰기 브랜칭 참고](../코스모스%20베이직/13_store_and_keepers.md))

트랜잭션은 여러 메시지로 구성될 수 있으며 각 메시지는 동일한 컨텍스트와 Gas Limit 내에서 차례로 실행된다. 이는 관계형 데이터베이스 ACID 트랜잭션 방식과 유사하게, 원자성을 매우 중요시 한다. 모든 메시지가 성공하면 컨텍스트는 기본 블록체인 상태에 커밋되고, 하나의 메시지가 실패하면 모든 이후 메시지는 건너뛰고 모든 상태 변경이 되돌려진다. 그렇게 각 트랜잭션은 결과나 오류와 함께 이벤트 로그를 반환한다. 


#### CosmWasm 컨트랙트 실행 (Basic Execution)
`x/wasm`은 트랜잭션의 메시지를 처리하고 스마트 컨트랙트를 업로드, 인스턴스화 및 실행하는 데 사용하는 사용자 정의 Cosmos SDK 모듈이다. 만약 컨트랙트의 `execute`가 실행된다고 하면, 적절히 서명된 `MsgExecuteContract`를 받아 `Keeper.Execute`로 라우팅하고, 적절한 스마트 컨트랙트를 로드하여 이를 실행한다. 이는 트랜잭션의 메시지 실행에 해당되어 성공 또는 실패를 할 수 있다. 만약 실패한다면 블록의 전체 트랜잭션을 롤백하게 된다.

트랜잭션 메세지로 실행되는 `execute` 함수는 Cosmwasm 컨트랙트를 구현할 때 [Entrypoint(진입점)](./22_entrypoint.md)로 제공하고 있다: 
```rust
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> { }
```

`DepsMut`을 통해 상태 읽기 및 쓰기, 모듈의 상태 쿼리를 수행할 수 있다. 작업이 완료되면 Ok(Response) 또는 Err(ContractError)를 반환한다. 메세지가 성공할 경우에는 Response 객체가 구문 분석되고 처리된다. 하지만 오류를 반환하게 되면 이는 문자열로 SDK 모듈에 전달되어 블록 전체 트랜잭션 롤백으로 이어진다. 성공될 경우 [Response 객체가 반환되어 이벤트](./23_message_and_event.md)로 기록된다. 

#### 메시지 디스패치 
교차 컨트랙트 호출이 있는 함수가 실행된다면 메시지 디스패치가 이뤄진다. CosmWasm 컨트랙트는 다른 컨트랙트를 호출하거나 토큰을 이동하기 위해 [`CosmosMsg`](./23_message.md#1-cosmosmsg)를 반환한다. 만약 컨트랙트가 M1, M2 두 개의 메시지를 반환한다고 하면, 이는 `x/wasm`에서 컨트랙트의 권한으로 구문 분석되고 실행된다:
- 성공 시, 이벤트가 방출되고, 반환된 메시지가 처리된다.
- 오류 발생 시, 전체 트랜잭션이 롤백된다. 

CosmosMsg는 깊이 우선으로 실행된다. 예를 들어, 계약 A가 M1 및 M2를 반환하고, 계약 B가 N1 및 N2를 반환하면, 실행 순서는 [M1 -> N1 -> N2 -> M2]가 된다.  

#### 서브 메시지
[`SubMessage`](./23_message.md#2-submessages)를 통해 호출 결과를 얻을 수 있는 기능이다. 서브메시지는 오류 결과를 캡처하여 전체 트랜잭션을 중단하지 않고 오류 메시지를 저장하고 메시지를 실행된 것으로 표시할 수 있다. 

서브메시지가 완료되면 호출자는 결과를 처리할 기회를 얻는다. 이는 서브콜의 원래 ID와 실행 결과를 모두 포함한다. 필요 시 추가 상태를 저장하려면 원래 execute에서 서브메시지를 반환하기 전에 로컬 컨텍스트를 스토어에 저장하고, reply에서 이를 로드해야 한다. 서브메시지 실행 및 응답은 메시지보다 먼저 실행된다. 예를 들어, 계약 A가 서브메시지 S1 및 S2, 메시지 M1을 반환한다. 서브메시지 S1이 메시지 N1을 반환하면, 실행 순서는 [S1 -> N1 -> reply(S1) -> S2 -> reply(S2) -> M1]이 된다.

### 2. Query
실행 중 컨트랙트의 Bank 잔액 조회와 같이 다른 컨트랙트의 정보를 중간에 액세스해야 할 경우가 있다. 이를 위해 읽기 전용 Querier를 사용하여 동기 호출을 실행 중에 수행하는 기능을 제공한다. 쿼리를 수행할 때, 가능한 모든 호출을 나타내는 [`QueryRequest` 구조체](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/query/mod.rs#L43-L71)를 직렬화하고, 이를 FFI를 통해 런타임으로 전달하여 `x/wasm` SDK 모듈에서 해석되어 실행된다. 이는 `CosmosMsg`가 커스텀을 수용하는 것처럼 블록체인별 사용자 정의 쿼리로 확장 가능하다. 또한 원시 protobuf "Stargate" 쿼리를 수행할 수 있는 기능을 제공한다:
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