# 10. WebAssembly Basic
> Cosmwasm에 대해 배우기에 앞서 해당 기술의 핵심이 되는 WebAssembly(Wasm)에 대한 이해를 키워나가고자 해당 아티클이 작성되었다.
## 목차
0. WebAssmebly(Wasm)
1. Wasm 특징 

## 0. WebAssembly(Wasm)
WebAssembly(Wasm)는 다양한 소스 언어로부터 이식 가능한 바이너리 실행 파일을 생성할 수 있는 바이너리 명령 형식을 정의하는 개방형 표준이다. 이러한 바이너리는 다양한 환경에서 실행될 수 있으며, 웹에서 기원하여 모든 주요 브라우저에서 지원된다. Wasm을 사용하면 서버, 엣지 등 어디서든 이전보다 더 이식성(portable) 있고 보안(security)이 뛰어난 프로그램을 실행할 수 있다.


## 1. Wasm 특징 
### 1. 리소스 효율성 및 속도
Wasm 응용 프로그램은 최소한의 메모리 공간과 CPU 요구 사항으로 실행되도록 설계되었다. 이는 네이티브 코드와 유사한 속도를 제공하며, VM 부팅이나 컨테이너 시작과 달리 콜드 스타트 시간이 없다.

### 2. 뛰어난 보안
Wasm 런타임은 기본적으로 샌드박스 환경에서 실행되며, 메모리에 안전하게 접근할 수 있다. 기능 기반 모델은 Wasm 애플리케이션이 명시적으로 허용된 항목에만 접근할 수 있도록 한다. 이로 인해 공급망 보안이 강화된다.
- 대부분의 언어는 런타임에서 함수에 주소를 할당한다. 메모리를 바이트 배열 형태로 보면 함수 코드가 제대로 구별되지 않아 안전하지 않다.
- Wasm은 프로그램 메모리를 안전한 영역에 캡슐화하여 프로그램을 실행하는 호스트에 영향을 미치거나 보안을 손상시킬 수 있는 코드를 허용하지 않는다.

### 3. 휴대성
Wasm은 여러 주요 런타임에서 대부분의 CPU 아키텍처(x86, ARM, RISC-V)를 지원하며, Linux, Windows, macOS 및 Non-POSIX 운영 체제에서도 실행될 수 있다.

### 4. 이식성
다양한 프로그래밍 언어를 Wasm으로 컴파일할 수 있으며, 현대적이고 지속적으로 개선되는 툴체인을 사용한다. 컴파일러는 LLVM(Low Level Virtual Machine) 백엔드를 활용하여 LLVM 중간 표현(IR)로 컴파일한 후 Wasm 프로그램을 생성할 수 있다.


## 2. WASI와 Wasm 런타임
WASI와 Wasm 런타임은 Wasm 생태계에서 중요한 역할을 한다. 
- WASI는 WebAssembly 프로그램이 운영 체제와 상호작용할 수 있도록 하여, WebAssembly의 활용 범위를 웹 브라우저 밖으로 확장한다. 
- Wasm 런타임은 이러한 WebAssembly 프로그램이 다양한 환경에서 원활히 실행될 수 있도록 지원한다.

다음 아티클에서는 WASI에 대한 소개와 Cosmwasm이 사용하는 Wasm 런타임인 Wasmer에 대해서 다룰 예정이다.

## Resources
- https://hacks.mozilla.org/category/code-cartoons/a-cartoon-intro-to-webassembly/
- https://rsms.me/wasm-intro