# 00. Build Nameservice Contract
## 목차
0. EntryPoint 함수 생성하기
   1. `instantiate` EntryPoint
   2. `execute` EntryPoint
   3. `query` EntryPoint

## 사전 지식
- [22_entrypoint](./22_entrypoint.md)

## 0. EntryPoint 함수 생성하기
이전에 프로젝트 생성에서 전체적인 프로젝트 구조를 생성하였을 것이다. [EntryPoint](./22_entrypoint.md) 함수는 `contract.rs`에서 관리하고 있다.

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
`query` 함수는 반환 타입이 다른 것을 볼 수 있다. 해당 함수는 항상 직렬화된 응답만 포함하는 Ok 케이스의 Binary 객체를 반환한다. 이를 생성하는 일반적인 방법은 `serde::Serialize`를 구현하는 객체에서 `to_binary` 메서드를 호출하면 된다. 

이제 EntryPoint 함수 생성을 완료했다. 각 함수는 기본적인 구조만을 가지고 있으며, 실제 비즈니스 로직은 이제부터 하나씩 추가해보도록 하자.



