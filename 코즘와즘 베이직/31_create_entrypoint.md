# 31. EntryPoint 생성하기
## 목차
0. EntryPoint 함수 생성하기
   1. `instantiate` EntryPoint
   2. `execute` EntryPoint
   3. `query` EntryPoint
## 1. 컨트랙트 빌드 후 검증하기

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


이제 차례대로 가장 기본적은 `entry_point` 함수인 `execute`와 `query`도 만들어줄 것이다.

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
`query` 함수는 반환 타입이 다른 것을 볼 수 있다. 해당 함수는 항상 직렬화된 응답만 포함하는 Ok 케이스의 Binary 객체를 반환한다. 이를 생성하는 일반적인 방법은 `serde::Serialize`를 구현하는 객체에서 `to_binary` 메서드를 호출하면 된다. 이는 추후에 쿼리 메시지를 추가하면서 다뤄본다.


## 1. 컨트랙트 빌드 후 검증하기
이제 프로젝트를 빌드하고 `cosmwasm-check`를 통해 컨트랙트 검증에 통과했는지 확인해 보자:
```sh
$ cargo wasm
$ cosmwasm-check ./target/wasm32-unknown-unknown/release/nameservice.wasm
Available capabilities: {"cosmwasm_1_4", "staking", "stargate", "cosmwasm_1_2", "cosmwasm_1_3", "cosmwasm_2_0", "iterator", "cosmwasm_1_1"}

./target/wasm32-unknown-unknown/release/nameservice.wasm: pass

All contracts (1) passed checks!
```
- `entry_point` 함수들을 추가하니 정상적으로 통과하는 것을 확인할 수 있다.

## 마무리 
이제 EntryPoint 함수 생성을 완료했다. 각 함수는 기본적인 구조만을 가지고 있으며, 실제 비즈니스 로직을 만들면서 조금씩 업데이트해나갈 것이다. 구현할 기능은 name을 네트워크에 등록하여 소유하는 register 기능과 소유한 name을 다른 사람에게 전송할 수 있는 transfer 기능이다.



