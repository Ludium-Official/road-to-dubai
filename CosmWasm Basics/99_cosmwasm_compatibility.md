# Compatibility
## Integration
- https://docs.cosmwasm.com/docs/integration

## 1.x Version Compatibility
v1.0.0-beta says it will provide an easy upgrade path and compatibility with previous versions to all future 1.x versions
- https://medium.com/cosmwasm/its-showtime-3feb474fe183

CosmWasm v1.0.0-beta comes with the code of the entire stack and is compatible with `wasmd`v0.20.0, `cw-plus`v0.10.0, `cosmjs`v0.26.0, and `cosmos-sdk`v0.42.10.

## Cosmwasm
Since Cosmwasm 1.0, the contract-host interface has not changed in an innovative way. Furthermore, the Cosmwasm 2.0 contract remains compatible at the wasm interface level.
- https://github.com/CosmWasm/wasmd

## Cosmwasm 1.4
Contracts using `cosmwasm-std` 1.4.0 can be distributed to all CosmWasm 1.x chains. By activating the cargo function cosmwasm_1_4, you can unlock functions that are only available in the 1.4 chain.
- CosmWasm 1.4 is available in wasmd 0.42 (Cosmos SDK 0.47 compatible).

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



## Rust 1.78 Related Issues
```
Error: rpc error: code = Unknown desc = rpc error: code = Unknown desc = failed to execute message; message index: 0: Error calling the VM: Error executing Wasm: Wasmer runtime error: RuntimeError: Aborted: panicked at /rust/deps/dlmalloc-0.2.6/src/dlmalloc.rs:1198:13:
assertion failed: psize <= size + max_overhead: instantiate wasm contract failed [CosmWasm/wasmd@v0.43.0/x/wasm/keeper/keeper.go:325] With gas wanted: '18446744073709551615' and gas used: '110152' : unknown request
```
- https://github.com/CosmWasm/wasmd/issues/1888
- https://github.com/rustwasm/wasm-pack/issues/1389

`coms-std` should be raised to version 2.0.1+ or 1.5.4+.