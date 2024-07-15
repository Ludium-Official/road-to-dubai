# 22. Entrypoint
## 목차
0. Entrypoint
   1. `entry_point` 매크로
   2. 함수 인자 
   3. 반환 타입

## 0. Entrypoint 
Entrypoint(진입점)는 메시지나 쿼리가 컨트랙트에 의해 처리되는 지점을 말한다. 

### 1. `entry_point` 매크로
바이너리 파일을 실행하는 Cosmwasm VM은 Rust 타입을 직접 처리할 수 없기 때문에 [`entry_point` 매크로](https://github.com/CosmWasm/cosmwasm/blob/main/packages/derive/src/lib.rs#L49-L120)를 통해 함수 호출을 처리한다. `entry_point` 매크로에 해당하는 함수 이름 이미 정의되어 있어 VM이 정확히 무엇을 호출해야 하는지 알 수 있다.

다음은 `entry_point` 매크로를 사용하는 예시이다:
```rust
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    // 초기화 로직
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    // 실행 로직
    Ok(Response::new())
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    // 쿼리 로직 
    Ok(Binary::default())
}

#[entry_point]
#[migrate_version(2)]
pub fn migrate(
    deps: DepsMut,
    env: Env,
    msg: MigrateMsg,
) -> StdResult<Response> {
    // 마이그레이션 로직
    Ok(Response::new())
}

#[entry_point]
pub fn reply(
    deps: DepsMut, 
    _env: Env, 
    msg: Reply
) -> StdResult<Response> {
    // reply 로직
    Ok(Response::new())
}
```

`#[cfg_attr(not(feature = "library"), entry_point)]`는 "library" 기능이 설정되지 않은 경우에만 `entry_point` 속성을 추가하는 조건부 어트리뷰션이다. 이는 다른 컨트랙트에 대한 종속성으로 컨트랙트를 사용할 수 있도록 하기 위해 필요하다. 최종 바이너리에는 각 `entry_point`의 복사본이 하나만 포함되도록 한다.

### 2. 함수 인자 
각 `entry_point` 함수는 다음과 같은 인자를 받는다:
- `_deps`([`Deps`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/deps.rs#L25-L30) 또는 [`DepsMut`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/deps.rs#L19-L23)): 스마트 컨트랙트 컨텍스트 외부 세계로 가는 관문이다. 이는 컨트랙트 상태에 액세스하고 다른 컨트랙트를 쿼리할 수 있으며, 몇 가지 유용한 유틸리티 함수가 포함된 API 객체를 제공한다. `DepsMut`은 상태를 업데이트할 수 있는 반면, `Deps`는 읽기만 가능하다.
- `_env`([`Env`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/types.rs#L8-L16)): 실행 시점의 블록체인 상태에 대한 정보(높이, 실행 타임스탬프, 실행 컨트랙트 자체에 대한 정보)를 전달한다.
- `_info`([`MessageInfo`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/types.rs#L83-L106)): 컨트랙트 호출에 대한 정보로, 메시지를 전송하는 주소와 메시지와 함께 전송되는 자금을 포함한다.
- `_msg`([`Empty`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/results/empty.rs#L4-L11)): 실행을 트리거한 메시지를 마지막 인자로 받는다. 지금은 {} JSON을 나타내는 Empty 타입이지만 이는 추후 컨트랙트 개발을 하면서 디코딩이 가능한 타입(`InstantiateMsg`, `ExecuteMsg` 등)으로 변경이 가능하다. 

### 3. 반환 타입
`entry_point`의 마지막 부분은 반환 타입이다. 모든 `entry_point`는 문자열로 변환할 수 있는 에러와 함께 `Result` 타입을 반환하며, 컨트랙트 실패 시 반환된 에러는 그냥 기록된다. 

#### Response 타입
`cosmwasm_std::Response` 타입은 컨트랙트 실행을 완료하는 데 필요한 모든 것을 보관한다. 여기에는 다음과 같은 정보를 가지고 있다:
- `events` 필드:  여기에는 실행의 결과로 블록체인에 전송되는 모든 이벤트가 포함된다. 이벤트는 문자열인 타입과 문자열 키-값 쌍인 속성 목록으로 구성된 매우 단순한 구조를 가진다.
- `attributes` 필드: 이것은 단지 편의를 위한 것이다. 대부분의 실행은 단일 이벤트만 반환하며, 이벤트를 좀 더 쉽게 조작하기 위해 응답에 직접 `attributes` 집합이 있다. 이 모든 attribute는 하나의 wasm 이벤트로 변환되어 방출된다. 그래서 이는 `event`와 하나로 여겨지기도 한다. 
- `SubMsg` 필드: 이것은 교차 컨트랙트 통신의 단서이다. 이러한 메시지는 처리 후 컨트랙트로 전송된다. 중요한 것은 컨트랙트에 의해 예약된 모든 하위 메시지의 처리가 완료되지 않는 한 전체 실행이 완료되지 않는다.
- `data` 필드: 이것은 쿼리 호출의 결과와 마찬가지로 또 다른 Binary 필드이며, 일반적으로 직렬화된 JSON을 포함한다. 모든 컨트랙트 호출은 어떤 형식으로든 몇 가지 추가 정보를 반환할 수 있다.

### Binary 타입 
`query` 함수는 블록체인 상태에 대한 쿼리를 처리하고 결과를 반환하는 역할을 한다. 이 함수는 항상 직렬화된 응답을 반환해야 한다. 이를 위해 `StdResult<Binary>` 타입을 사용한다. 


## Resources
- https://docs.cosmwasm.com/docs/smart-contracts/entry-points/
- https://github.com/CosmWasm/cosmwasm/blob/main/packages/derive/src/lib.rs