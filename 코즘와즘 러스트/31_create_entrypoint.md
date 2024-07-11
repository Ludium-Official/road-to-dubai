# 00. Build Nameservice Contract
## 목차
0. EntryPoint
   1. `entry_point`
   2. 함수 인자 
   3. 반환 타입
1. EntryPoint 함수 생성하기
   1. `instantiate` EntryPoint
   2. `execute` EntryPoint
   3. `query` EntryPoint

## 0. EntryPoint
`entry_point`는 스마트 컨트랙트에서 메시지가 처리될 때 Cosmwasm VM이 호출하는 함수이다. 중요한 것은 이러한 `entry_point`의 이름(주요 기능과 유사하게)이 고정되어 있어 VM이 정확히 무엇을 호출해야 하는지 알 수 있다는 것이다. 따라서 `instantiate` 함수는 컨트랙트의 생성자와 같다.  

### 1. `entry_point`
`entry_point` 속성은 보일러플레이트를 생성하는 매크로이다. 바이너리는 Cosmwasm VM에 의해 실행되기 때문에 Rust 타입에 대해 잘 알지 못하며, 실제 `entry_point` 서명은 사용하기가 매우 불편하다. 이 문제를 극복하기 위해 매크로를 생성하여 `entry_point`를 생성하고, 이 `entry_point`는 함수를 호출한다.

모든 `not(feature = "library)`는 기존 `#[entry_point]`와 같은 조건부 어트리뷰션으로, "library" 기능이 설정되지 않은 경우에만 이를 추가한다. 이렇게 하는 이유는 다른 컨트랙트에 대한 종속성으로 컨트랙트를 사용할 수 있도록 하기 위해서이다. 최종 바이너리에는 각 `entry_point`의 복사본이 하나만 포함될 수 있으므로, 이 기능 없이 최상위 레벨의 것만 컴파일되도록 한다. 

### 2. 함수 인자 
이제 함수 인수를 살펴보자:
- `_deps`(`Deps` 또는 `DepsMut`): 스마트 컨트랙트 컨텍스트 외부 세계로 가는 관문이다. 
	- 이는 컨트랙트 상태에 액세스하고 다른 컨트랙트를 쿼리할 수 있으며, 몇 가지 유용한 유틸리티 함수가 포함된 API 객체를 제공한다. 
	- 차이점은 `DepsMut`은 상태를 업데이트할 수 있는 반면, `Deps`는 읽기만 가능하다. 
- `_env`(`Env`): 실행 시점의 블록체인 상태에 대한 정보(높이, 실행 타임스탬프, 실행 컨트랙트 자체에 대한 정보)를 전달한다.
- `_info`(`MessageInfo`): 컨트랙트 호출에 대한 정보로, 메시지를 전송하는 주소와 메시지와 함께 전송되는 자금을 포함한다.
- `_msg`(`Empty`): 실행을 트리거한 메시지를 마지막 인자로 받는다. 지금은 {} JSON을 나타내는 Empty 타입이지만 이는 추후 컨트랙트 개발을 하면서 디코딩이 가능한 타입으로 변경이 가능하다. 

### 3. 반환 타입
`entry_point`의 마지막 부분은 반환 타입이다. 모든 `entry_point`는 문자열로 변환할 수 있는 error와 함께 `Result` 타입을 반환하며, 컨트랙트 실패 시 반환된 error는 그냥 기록된다. 

`cosmwasm_std::Response` 타입은 컨트랙트 실행을 완료하는 데 필요한 모든 것을 보관한다. 여기에는 다음과 같은 정보를 가지고 있다:
- `events` 필드:  여기에는 실행의 결과로 블록체인에 전송되는 모든 이벤트가 포함된다. 이벤트는 문자열인 타입과 문자열 키-값 쌍인 속성 목록으로 구성된 매우 단순한 구조를 가진다.
- `attributes` 필드: 이것은 단지 편의를 위한 것이다. 대부분의 실행은 단일 이벤트만 반환하며, 이벤트를 좀 더 쉽게 조작하기 위해 응답에 직접 `attributes` 집합이 있다. 이 모든 attribute는 하나의 wasm 이벤트로 변환되어 방출된다. 그래서 이는 `event`와 하나로 여겨지기도 한다. 
- `SubMsg` 필드: 이것은 교차 컨트랙트 통신의 단서이다. 이러한 메시지는 처리 후 컨트랙트로 전송된다. 중요한 것은 컨트랙트에 의해 예약된 모든 하위 메시지의 처리가 완료되지 않는 한 전체 실행이 완료되지 않는다.
- `data` 필드: 이것은 쿼리 호출의 결과와 마찬가지로 또 다른 Binary 필드이며, 일반적으로 직렬화된 JSON을 포함한다. 모든 컨트랙트 호출은 어떤 형식으로든 몇 가지 추가 정보를 반환할 수 있다.


## 1. EntryPoint 함수 생성하기
이전에 프로젝트 생성에서 전체적인 프로젝트 구조를 생성하였을 것이다. EntryPoint 함수는 `contract.rs`에서 관리하고 있다.

### 1. `instantiate` EntryPoint
그럼 이제 `src/contract.rs`에 EntryPoint를 만들어보자:
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

간단하게 반환 타입은 `Result<Response, StdError>`를 사용한다. 이제 프로젝트를 컴파일하고 `cosmwasm-check`를 통과했는지 확인해 보자:
```sh
$ cargo wasm
$ cosmwasm-check ./target/wasm32-unknown-unknown/release/namespace.wasm
Available capabilities: {"cosmwasm_1_4", "staking", "stargate", "cosmwasm_1_2", "cosmwasm_1_3", "cosmwasm_2_0", "iterator", "cosmwasm_1_1"}

./target/wasm32-unknown-unknown/release/namespace.wasm: pass

All contracts (1) passed checks!
```

정상적으로 통과한 것을 확인할 수 있다. 이제 차례대로 `execute`와 `query`도 만들어줄 것이다. 


### 2. `execute` EntryPoint
`instantiate` 함수와 같은 위치인 `src/contract.rs`에 `execute` 함수를 추가한다:
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
이는 `instantiate` 함수와 생김새는 비슷하다. 

### 3. `query` EntryPoint
마찬가지로 `src/contract.rs`에 `query` 함수를 추가한다:
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

`query` 함수는 반환 타입이 다른 것을 볼 수 있다. 해당 함수는 항상 직렬화된 응답만 포함하는 Ok 케이스의 Binary 객체를 반환한다.  이를 생성하는 일반적인 방법은 `serde::Serialize`를 구현하는 객체에서 `to_binary` 메서드를 호출하면 된다. 

이제 EntryPoint 함수 생성을 완료했다. 각 함수는 기본적인 구조만을 가지고 있으며, 실제 비즈니스 로직은 이제부터 하나씩 추가해 볼 것이다.



