# 12. Wasmer 
> WebAssembly(Wasm)가 런타임 실행되는 환경에 대해 이해하기 위해 Wasm 런타임 구현체 중 하나인 Wasmer에 대해서 알아보고자 한다. 

## 목차
0. Wasm 런타임
1. Wasmer

## 0. Wasm 런타임
Wasm 런타임은 웹어셈블리 코드를 로드하고 실행할 수 있는 환경으로, WASM 바이너리를 실행하는 데 필요한 인프라를 제공한다. 런타임은 WASM 명령어를 실행하기 위한 인터프리터 또는 가상 머신이라고 생각하면 된다. 브라우저가 아닌 환경을 위해 특별히 개발된 다양한 Wasm 런타임이 있다. 이러한 런타임은 점점 더 WASI 인터페이스를 지원하기 시작하여 표준화된 시스템 액세스를 통해 브라우저 외부에서 Wasm 애플리케이션을 실행할 수 있게 되었다. 현재 wasm 런타임으로 Wasmer, Wasmtime, WasmEdge 등 존재한다.

우리는 이 중에 Cosmwasm이 사용하고 있는 Wasmer에 대해서 알아보고자 한다.

## 1. Wasmer
Wasmer는 WebAssembly 런타임 중 하나로, 다양한 환경에서 WebAssembly 바이너리를 실행할 수 있게 해준다. Wasmer는 CLI, 네이티브 애플리케이션, 서버 애플리케이션 등 여러 형태로 사용할 수 있다.

### Wasm이 OS에서 실행되는 과정
WASI와 Wasm 런타임 덕분에 Linux, Windows, macOS와 같은 기존 운영 체제에서 Wasm 모듈을 안전하고 빠르게 실행할 수 있다. 이 프로세스는 브라우저 실행과 대체로 유사하다. WASI는 시스템별 차이를 추상화하는 일관된 인터페이스를 제공하여 서버에서 Wasm 바이너리가 보편적으로 실행될 수 있도록 보장한다.
1. Wasm으로 컴파일: 브라우저 시나리오와 마찬가지로 소스 코드는 Emscripten과 같은 도구를 사용하거나 LLVM의 Wasm 백엔드를 통해 직접 Wasm으로 컴파일된다.
2. 런타임 활용: Wasmer와 같은 Wasm 런타임이 탑재된 서버에 Wasm 바이너리를 배포한다.
3. 실행: 런타임은 Wasm 모듈을 로드하고 WASI의 도움으로 파일 시스템, 네트워크 프로토콜 등을 포함한 서버 리소스와 원활하게 상호 작용한다.



## Resources
- https://github.com/wasmerio/wasmer