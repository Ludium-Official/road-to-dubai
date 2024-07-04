
#! WIP

## 0. module 컴포넌트 
[`x/moduleName` 폴더](https://github.com/cosmos/cosmos-sdk/tree/v0.47.0/x)에 모듈을 정의한다. 
### 1. Interface
Interface는 모듈 간의 통신과 여러 모듈을 일관된 앱으로 구성하는 것을 용이하게 한다.

Module은 나머지 애플리케이션과 통합하기 위해 세 가지 앱 모듈 인터페이스를 구현해야 한다:
- AppModuleBasic: 모듈의 비의존적 요소를 구현합니다.
- AppModule: 애플리케이션에 고유한 모듈의 상호 의존적이고 특수한 요소를 구현합니다.
- AppModuleGenesis: 시작 시 블록체인의 초기 상태를 설정하는 모듈의 상호 의존적인 생성/초기화 요소.

앱 모듈과 앱 모듈 베이직 및 해당 함수는 모듈의 `x/moduleName/module.go` 파일에서 정의한다.


### 2. Protobuf
Protobuf는 메시지 처리를 위한 하나의 `Msg` 서비스와 쿼리를 핸들링하기 위한 하나의 gRPC `Query` 서비스를 제공한다.

각 모듈은 두 개의 프로토부프 서비스를 정의한다:
- `Msg`: 메시지를 처리하기 위해 Protobuf 요청 유형과 one-to-one으로 연결된 RPC 메서드 집합이다.
- `Query`: 쿼리를 처리하기 위한 gRPC 쿼리 서비스이다.

### 3. `Msg` Service
`Msg` 서비스와 관련해서는 다음과 같이 유의하자:
- 가장 좋은 방법은 `tx.proto` 파일에 `Msg` Protobuf 서비스를 정의하는 것이다. 
- 각 모듈은 AppModule 인터페이스의 일부로 `RegisterServices` 메서드를 구현해야 한다. 이를 통해 애플리케이션은 모듈이 처리할 수 있는 메시지와 쿼리를 알 수 있다.
- 서비스 메서드는 스토리지 레이아웃에 대한 지식을 캡슐화하고 상태를 업데이트하는 메서드를 제공하는 keeper를 사용해야 한다.

### 4. gRPC `Query` Service
gRPC `Query` 서비스의 경우 다음 사항에 유의하자:
- 가장 좋은 방법은 query.proto 파일에 쿼리 프로토부프 서비스를 정의하는 것이다.
- 이를 통해 사용자는 gRPC를 사용하여 상태를 쿼리할 수 있다.
- 각 gRPC 엔드포인트는 서비스 메서드에 해당하며, gRPC 쿼리 서비스 내에서 접두사 rpc로 이름이 지정된다.
- app.toml의 grpc.enable 및 grpc.address 필드에서 구성할 수 있다.

Protobuf는 각 모듈에 대한 모든 서비스 메서드를 포함하는 `QueryServer` 인터페이스를 생성한다. 모듈은 각 서비스 메서드의 구체적인 구현을 별도의 파일에 제공함으로써 이 `QueryServer` 인터페이스를 구현합니다. 이러한 구현 메서드는 해당 gRPC 쿼리 엔드포인트의 핸들러이다. 이렇게 여러 파일에 걸쳐 우려 사항을 나누면 Protobuf에 의한 파일 재생성으로부터 안전하게 설정할 수 있다.

gRPC-gateway REST 엔드포인트는 gRPC를 사용하지 않으려는 외부 클라이언트를 지원한다. Cosmos SDK는 각 gRPC 서비스에 대한 gRPC 게이트웨이 REST 엔드포인트를 제공한다.

### 5. CLI commands
각 모듈은 CLI를 위한 명령을 정의한다. 모듈과 관련된 명령은 `client/cli`라는 폴더에 정의되어 있다. CLI는 명령을 트랜잭션과 쿼리라는 두 가지 범주로 나눠진다. 이는 각각 `tx.go` 및 `query.go`에서 정의한 것과 동일하다.

### 6. Keeper
`Keeper`는 모듈의 모든 store에 대한 gatekeeper이다. 
- store에 액세스하려면 모듈의 `Keeper`를 반드시 거쳐야 한다. 
- `Keeper`는 스토어 내 스토리지 레이아웃에 대한 지식을 캡슐화하고 이를 업데이트 및 검사하는 메서드를 포함한다. 
- Web2의 MVC 세계에서 왔다면 `Keeper`를 컨트롤러로 생각하면 도움이 된다. `Keeper`는 상태를 정의하고 업데이트, 검사하는 방법을 제시하는 컨트롤러이다.


`Keeper`는 `keeper.go`에 정의되어 있습니다. `Keeper`의 유형 정의는 일반적으로 멀티스토어에서 모듈의 자체 스토어에 대한 키, 다른 모듈의 키퍼에 대한 참조, 애플리케이션의 코덱에 대한 참조로 구성된다.


## 1. Module Folder Structure
일반적인 Cosmos SDK 모듈은 다음과 같이 구성할 수 있다.

직렬화 가능한 데이터 유형 및 Protobuf 인터페이스:
```
proto
└── {project_name}
    └── {module_name}
        └── {proto_version}
            ├── {module_name}.proto
            ├── event.proto
            ├── genesis.proto
            ├── query.proto
            └── tx.proto
```
- `{module_name}.proto`: 모듈의 일반적인 메시지 유형 정의입니다.
- `event.proto`: 이벤트와 관련된 모듈의 메시지 유형 정의.
- `genesis.proto`: 제네시스 상태와 관련된 모듈의 메시지 유형 정의.
- `query.proto`: 모듈의 쿼리 서비스 및 관련 메시지 유형 정의.
- `tx.proto`: 모듈의 Msg 서비스 및 관련 메시지 유형 정의.

그런 다음 나머지 코드 요소를 입력한다:
```
x/{module_name}
├── client
│   ├── cli
│   │   ├── query.go
│   │   └── tx.go
│   └── testutil
│       ├── cli_test.go
│       └── suite.go
├── exported
│   └── exported.go
├── keeper
│   ├── genesis.go
│   ├── grpc_query.go
│   ├── hooks.go
│   ├── invariants.go
│   ├── keeper.go
│   ├── keys.go
│   ├── msg_server.go
│   └── querier.go
├── module
│   └── module.go
├── simulation
│   ├── decoder.go
│   ├── genesis.go
│   ├── operations.go
│   └── params.go
├── spec
│   ├── 01_concepts.md
│   ├── 02_state.md
│   ├── 03_messages.md
│   └── 04_events.md
├── {module_name}.pb.go
├── abci.go
├── codec.go
├── errors.go
├── events.go
├── events.pb.go
├── expected_keepers.go
├── genesis.go
├── genesis.pb.go
├── keys.go
├── msgs.go
├── params.go
├── query.pb.go
└── tx.pb.go
```
- `client/`: 모듈의 CLI 클라이언트 기능 구현 및 모듈의 통합 테스트 스위트.
- `exported/`: 모듈의 exported type - 일반적으로 인터페이스 유형.
- `keeper/`: 모듈의 Keeper 및 MsgServer 구현.
- `module/`: 모듈의 `AppModule` 및 `AppMoudleBasic` 구현.
- `simulation/`: 모듈의 시뮬레이션 패키지는 블록체인 시뮬레이터 애플리케이션(`simapp`)에서 사용하는 함수를 정의한다.
- `spec/`: 중요한 개념, 상태 저장소 구조, 메시지 및 이벤트 유형 정의를 설명하는 모듈의 사양 문서이다.

root 디렉토리에는 프로토콜 버퍼에서 생성된 타입 정의를 포함하여 messages, events 및 genesis 상태에 대한 유형 정의가 포함되어 있다:
- `abci.go`: 모듈의 `BeginBlocker` 및 `EndBlocker` 구현. 이 파일은 `BeginBlocker` 또는 `EndBlocker`를 정의해야 하는 경우에만 필요하다.
- `codec.go`: 인터페이스 타입에 대한 모듈의 registry 메서드.
- `errors.go`: 모듈의 sentinel 에러.
- `events.go`: 모듈의 이벤트 타입 및 생성자.
- `expected_keepers.go`: 모듈의 예상되는 다른 keeper 인터페이스.
- `genesis.go`: 모듈의 제네시스 상태 메서드 및 헬퍼 함수.
- `keys.go`: 모듈의 store keys 및 관련 helper 함수.
- `msgs.go`: 모듈의 메시지 타입 정의 및 관련 메서드.
- `params.go`: 모듈의 매개변수 타입 정의 및 관련 메서드.
- `*.pb.go`: 각 `*.proto` 파일에 정의된 대로 Protobuf에 의해 생성된 모듈의 타입 정의.

> 모듈이 다른 모듈의 keeper에 의존하는 경우, `/exported` 코드 요소는 keeper를 구현하는 모듈에 대한 직접적인 종속성을 피하기 위해 인터페이스 컨트랙트로 keeper를 수신할 것으로 예상한다. 그러나 이러한 인터페이스 컨트랙트는 keeper를 구현하는 모듈에서 작동하는(또는 특정 타입을 반환하는) 메서드를 정의할 수 있다.

> `/exported` 표준 타입에 정의된 인터페이스 타입은 모듈이 `expected_keepers.go` 파일을 통해 인터페이스 컨트랙트를 수신할 수 있도록 한다. 이 패턴을 사용하면 코드를 DRY로 유지할 수 있으며 가져오기 주기의 혼란도 완화할 수 있다.


### 개발 프로세스
1. 상태 정의 
2. keeper에서 KVStore 구현하기
3. 인터페이스 구현  
   1. 트랜잭션 메시지 서비스 구현
   2. 쿼리 서비스 구현
4. CLI 구현
5. BeginBlocker 및 EndBlocker 구현(선택 사항)
6. InitGenesis / ExportGenesis 구현
7. app.go에서 모듈 연결하기 
8. 단위 테스트/통합 테스트/시뮬레이션 테스트 작성하기
9. API 및 CLI 사용에 대한 문서 작성



### v.0.50.x 이상 업데이트 사항
#### Moudle 관리 방식 
Cosmos SDK v0.50부터는 모듈을 자체 리포지토리에 보관하는 방식을 선호한다. 이렇게 하면 코드의 전반적인 모듈화가 향상되고 타사 재사용 절차가 간소화되기 때문이다. 
- 예를 들어, Checkers라는 모듈은 github.com/alice/checkers 리포지토리에 있으며 전체 Go 애플리케이션에서 그대로 사용하거나, `go.mod` 리디렉션을 사용하여 대체(github.com/alice/checkers => ../checkers-module/)와 같이 로컬에 보관할 수 있다.

v0.50에서도 `x/moduleName` 방법을 사용할 수 있다. 예를 들어, Checkers라는 모듈은 `x/checkers`에 들어간다. 

#### Depinject Module
cosmos-sdk 의존성 주입 프레임워크로 v0.50에 beta 버전으로 도입되었다. 
- https://docs.cosmos.network/v0.50/build/packages/depinject