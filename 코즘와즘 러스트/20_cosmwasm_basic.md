# 20. Cosmwasm Basic
## 목차
0. Cosmwasm

## 0. Cosmwasm
Cosmwasm은 코스모스 생태계를 위해 구축된 Wasm을 사용하는 스마트 컨트랙트 플랫폼이다. Cosmwasm은 Cosmos SDK에 플러그인할 수 있는 [모듈](../코스모스%20베이직/20_module_basic.md)로 작성되었다. 즉, 현재 Cosmos SDK를 사용해 블록체인을 구축 중인 누구나 기존 로직을 조정하지 않고도 빠르고 쉽게 Cosmwasm 스마트 컨트랙트 지원을 체인에 추가할 수 있다.

현재 Cosmwasm에서 가장 많이 사용되는 프로그래밍 언어는 Rust이며, 향후에는 AssemblyScript와 같은 다른 프로그래밍 언어도 사용할 수 있다.

### Cosmwasm 사용하는 방법 
Cosmwasm은 또 다른 Cosmos SDK 모듈이므로 다음과 같은 의존성 바이너리 하나만으로도 블록체인에 통합을 시작할 수 있다. 
```go
// go.mod 
require (
    github.com/CosmWasm/wasmd v0.16.0
)
```

[Cosmos Hub](https://github.com/cosmos/gaia/blob/main/app/modules.go#L65)에서는 [wasmd](https://github.com/CosmWasm/wasmd)라는 cosmwasm 샘플 바이너리를 사용하고 있다. 




## Resources
- https://docs.cosmwasm.com/docs/