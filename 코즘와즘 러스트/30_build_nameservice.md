# 00. Nameservice Contract 빌드하기
## 목차
0. Nameservice
1. 사전 설치
2. 프로젝트 생성하기 
3. 프로젝트 빌드하기

## 0. Nameservice
Nameservice는 블록체인에서 이름을 등록하고, 조회하고, 관리하는 스마트 컨트랙트이다. 이 문서에서는 Rust로 Nameservice 컨트랙트를 빌드하는 과정을 설명한다.

## 1. 사전 설치 
### rust 설치
먼저 Rust를 설치한다. 이미 설치되어있으면 생략해도 무관하다. 다운로드 명령은 다음과 같다:
```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

rust가 정상적으로 설치되었는지 확인한다:
```sh
$ rustup --version
# rustup 1.27.1 (54dd3d00f 2024-04-24)

$ cargo --version
# cargo 1.78.0 (54d8815d0 2024-03-26)

$ rustc --version
# rustc 1.78.0 (9b00956e5 2024-04-29)
```

### rust wasm 설치
WASM 빌드를 위해 Rust WASM 타겟을 추가한다:
```sh
$ rustup target add wasm32-unknown-unknown
```

### coswasm-check 설치
Cosmwasm 체크 도구를 설치한다:
```sh
$ cargo install cosmwasm-check
```


## 2. 프로젝트 생성
```sh
$ cargo new  --lib ./namespace
$ cd namespace
```

프로젝트를 빌드하기 전에, 우선 폴더 구조는 다음과 같아야 한다:
```sh
src  
├── contract.rs  
├── error.rs  
├── helpers.rs  
├── lib.rs  
├── msg.rs  
├── tests.rs  
└── state.rs
```

`lib.rs`에 다음과 같이 모듈들을 선언해준다. 
```rust
pub mod helpers;
pub mod contract;
mod error;
pub mod msg;
pub mod state;

#[cfg(test)]
mod tests;
```
- `msg.rs` : 외부랑 통신하는 메시지 타입에 대해 정의한다.
- `contract.rs`: EntryPoint와 해당 함수의 비즈니스 로직을 정의한다.
- `error.rs`: 커스텀 에러 타입에 대해 정의한다. 
- `state.rs`: 컨트랙트 내부 상태에 대해 정의한다.
- `tests.rs`: 전체적인 유닛 테스트를 정의한다.
- `helpers.rs`: 유저의 토큰 잔액 확인 등 비즈니스 로직을 실행할 때 필요한 부수적인 함수들을 정의한다.

## 3. 프로젝트 빌드하기 
### wasm 아티팩트 빌드하기
Cargo에서 동적 라이브러리를 구축하려면 `Cargo.toml` 파일을 수정해야 한다. 동적 라이브러리에 적합한 crate 유형을 `cdylib`로 설정하는 `lib` 섹션을 추가한다:
```rust
[package]
name = "counting-contract"
version = "0.1.0"
edition = "2021"
 
[lib]
crate-type = ["cdylib", "rlib"]
```
- `cdylib` 외에도 `rlib` 크레이트 유형을 추가했다. 이는 cargo에 두 가지 종류의 출력을 빌드하도록 지시하며, `rlib`는 표준 정적 러스트 라이브러리이다. 지금 당장 필요하지는 않지만 있으면 도움이 되므로 일단 여기 남겨둔다. 

이제 약간 수정된 빌드 명령을 호출하여 WASM 출력을 빌드할 준비가 되었다:
```sh
$ cargo build --target wasm32-unknown-unknown

# /target/wasm32-unknown-unknown/debug/namespace.wasm
# /target/wasm32-unknown-unknown/debug/deps/namespace.wasm
```

### alias 설정하기 
이 시점에서 wasm 바이너리가 준비되었지만 빌드 명령을 더 간결하게 만들 수 있는 간단한 방법이 있다. 스마트 컨트랙트 프로젝트에 다음과 같은 `.cargo/config` 파일을 생성하여 이를 수행하는 경우가 많다:
```
[alias]
wasm = "build --release --target wasm32-unknown-unknown"
```
- 기본적으로 배포할 wasm 바이너리를 빌드할 때에는 최적화를 위해 `--release`를 사용하여 빌드한다. 

이제 alias 명령어가 제대로 작동하는지 확인해 보자:
```sh
$ cargo wasm

# /target/wasm32-unknown-unknown/release/namespace.wasm
# /target/wasm32-unknown-unknown/release/deps/namespace.wasm
# /target/wasm32-unknown-unknown/debug/namespace.wasm
# /target/wasm32-unknown-unknown/debug/deps/namespace.wasm
```

이전에 설치한 `cosmwasm-check` 명령어를 통해서 빌드된 wasm 바이너리 파일을 체크해보자: 
```sh
$ cosmwasm-check ./target/wasm32-unknown-unknown/release/namespace.wasm

Available capabilities: {"cosmwasm_1_1", "iterator", "cosmwasm_1_4", "cosmwasm_2_0", "staking", "stargate", "cosmwasm_1_3", "cosmwasm_1_2"}

./target/wasm32-unknown-unknown/release/namespace.wasm: failure
Error during static Wasm validation: Wasm contract missing a required marker export: interface_version_*

Passes: 0, failures: 1
```

### cosmwasm-std 라이브러리 추가하기 
뭔가 에러가 발생하는 것을 확인할 수 있다. 이는 cosmwasm 컨트랙트에 `entry_point`를 만들어주지 않았기 때문이다. 그러기 위해서는 먼저 프로젝트에 `cosmwasm-std` 종속성을 추가해야 한다. rust 버전이 1.62 이상이라고 가정하면(또는 cargo-edit 유틸리티를 수동으로 설치한 경우) cargo add를 사용할 수 있다:
> rust 버전이 1.78인 경우에는 1.5.4+ 버전을 설치해야 한다.
```sh
$ cargo add cosmwasm-std@1.5.4
```


그러면 다음과 같이 dependecies 항목에 라이브러리가 추가된 것을 확인할 수 있다:
```toml
[package]
name = "namespace"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
cosmwasm-std = "1.5.4"
```

없고 작업을 위해 `instantiate` `entry_point`가 필요하기 때문이다. `cosmwasm-check`가 일부 버전 마커에 대해 불만을 제기하는 이유는 동일한 매크로가 진입점으로 마커를 생성하기 때문이므로 매크로를 추가하면 이 문제를 해결할 수 있다. 


## Resources
- https://github.com/deus-labs/cw-contracts/tree/main/contracts/nameservice