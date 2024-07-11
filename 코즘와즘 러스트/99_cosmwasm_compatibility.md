# 99. Compatibility
## 1.x 버전 호환성 
v1.0.0-beta에서 모든 향후 1.x 버전으로의 이전 버전과의 호환성과 쉬운 업그레이드 경로 제공하겠다고 함
- https://medium.com/cosmwasm/its-showtime-3feb474fe183

CosmWasm v1.0.0-beta는 전체 스택의 코드와 함께 제공되며 `wasmd`v0.20.0, `cw-plus`v0.10.0, `cosmjs`v0.26.0 및 `cosmos-sdk`v0.42.10과 호환된다. 

## Cosmwasm
Cosmwasm 1.0 이후 컨트랙트-호스트 인터페이스는 획기적인 방식으로 변경되지 않았다. 또한 Cosmwasm 2.0 컨트랙트는 wasm 인터페이스 수준에서 호환성을 유지한다.
- https://github.com/CosmWasm/wasmd

## Cosmwasm 1.4 
`cosmwasm-std` 1.4.0을 사용하는 계약은 모든 CosmWasm 1.x 체인에 배포할 수 있다. cargo 기능 cosmwasm_1_4를 활성화하면 1.4 체인에서만 사용할 수 있는 기능을 잠금 해제할 수 있다.
- CosmWasm 1.4는 wasmd 0.42 (Cosmos SDK 0.47 호환) 에서 사용할 수 있다. 

## Cosmwasm 1.5 
CosmWasm 1.5 will be embedded in the upcoming wasmd 0.44. 
- https://medium.com/cosmwasm/cosmwasm-1-5-946fd3024f1d

## Cosmwasm 2.0
CosmWasm 2.0 will be shipped as part of wasmd 0.51
- Existing contracts compiled with cosmwasm-std ^1.0.0 continue to run as before on chains with wasmvm 2.0. 
- Contracts compiled with cosmwasm-std ^2.0.0 do run on chains with wasmvm 1.x or 2.0. This way, it does not matter in which order you upgrade the chain or contracts.
- https://medium.com/cosmwasm/cosmwasm-2-0-bbb94126ce6f

## wasmd와 CosmosSDK
- wasmd v0.3x - Cosmos SDK v0.45
- wasmd v0.4x - Cosmos SDK v0.47.5
- wasmd v0.50 - Cosmos SDK v0.50.1
- https://github.com/CosmWasm/wasmd/blob/main/INTEGRATION.md



## rust 1.78 관련 이슈
```
Error: rpc error: code = Unknown desc = rpc error: code = Unknown desc = failed to execute message; message index: 0: Error calling the VM: Error executing Wasm: Wasmer runtime error: RuntimeError: Aborted: panicked at /rust/deps/dlmalloc-0.2.6/src/dlmalloc.rs:1198:13:
assertion failed: psize <= size + max_overhead: instantiate wasm contract failed [CosmWasm/wasmd@v0.43.0/x/wasm/keeper/keeper.go:325] With gas wanted: '18446744073709551615' and gas used: '110152' : unknown request
```
- https://github.com/CosmWasm/wasmd/issues/1888
- https://github.com/rustwasm/wasm-pack/issues/1389

`comswams-std`를 2.0.1+ 또는 1.5.4+ 버전으로 올려줘야 한다. 