# 05. Understand simapp architecture 1

지난 시간에 앱 체인의 메인 컴포넌트에 대해 배웠습니다. 이번 시간에는 이 메인 컴포넌트들이 어떻게 구현되어 있는지 알아보겠습니다. Cosmos SDK로 구현된 앱 체인은 여러 레이어로 구성되어 있으며, 개발자들이 주로 신경 쓰는 부분은 맨 위의 state-machine 레벨입니다. 그렇다고 컨센서스와 같은 파트들이 전혀 안쓰이는 것은 아니고 대부분 인터페이스화가 되어있습니다. 

```sh
                ^  +-------------------------------+  ^
                |  |                               |  |
                |  |  State-machine = Application  |  |
                |  |                               |  |   Built with Cosmos SDK
                |  |            ^      +           |  |
                |  +----------- | ABCI | ----------+  v
                |  |            +      v           |  ^
                |  |                               |  |
Blockchain Node |  |           Consensus           |  |
                |  |                               |  |
                |  +-------------------------------+  |   CometBFT
                |  |                               |  |
                |  |           Networking          |  |
                |  |                               |  |
                v  +-------------------------------+  v
```

**참고사항**

> 블록체인 풀 노드는 일반적으로 -d로 끝나는 바이너리로 나타납니다 (예: appd 또는 gaiad). 이 바이너리는 ./cmd/appd/에 있는 main.go 파일을 실행하여 빌드됩니다. 이 작업은 일반적으로 Makefile을 통해 이루어집니다.

우리는 이전 시간에 simapp의 데몬 이름인 simd에서 start라는 명령어로 운영을 해보았습니다. 이제 simapp의 핵심 레벨로 들어가 봅시다.

## simapp 구조

먼저 simapp의 구조를 살펴보겠습니다.

<!-- 그리고 이제 제일 중요한 core application file인 app.go를 살펴보자
simapp 은 baseapp를 instance와 시켜주는 제일 중요함 이거만 잘 이해해도 많은 게 커버가 됨. -->


```bash
.
├── README.md
├── app.go
├── app_test.go
├── config.go
├── encoding.go
├── export.go
├── genesis.go
├── genesis_account.go
├── genesis_account_test.go
├── helpers
│   └── test_helpers.go
├── params
│   ├── amino.go
│   ├── doc.go
│   ├── encoding.go
│   ├── params.go
│   ├── proto.go
│   └── weights.go
├── sim_bench_test.go
├── sim_test.go
├── simd
│   ├── cmd
│   │   ├── cmd_test.go
│   │   ├── genaccounts.go
│   │   ├── genaccounts_test.go
│   │   ├── root.go
│   │   ├── testnet.go
│   │   └── testnet_test.go
│   └── main.go
├── state.go
├── test_helpers.go
├── types.go
├── utils.go
└── utils_test.go
```

<!-- 그 중에 app.go란 파일에 있는 아래 코드를 보면 우리가 제네시스에서 사용했던 각 모듈들이 app.go라고 하는 app chain의 기본 구조체에서 참조하고 있다는 걸 알 수 있어 -->

## app.go 파일
app.go 파일을 보면 제네시스에서 사용했던 각 모듈들이 이 파일에서 참조된다는 것을 알 수 있습니다. start 명령어를 실행하면 app.go 파일에 정의된 state-machine 구조체가 인스턴스화 됩니다.

NewSimapp과 같은 초기화 메서드로 구조체를 생성합니다. 생성된 state machine은 데이터베이스에서 최신 state와 높이를 추출하여 초기화되고, 다른 피어와의 핸드셰이킹을 통해 싱크를 합니다.

<!-- ## Review from previous.. node client

`start`란 커맨드를 실행시키면 `app.go` 라는 파일에 정의된 state-machine struct가 instance화 됩니다.
 
`NewSimapp`과 같은 initiate method로 구조체를 create한다는 말입니다. 

이제부터 우리가 배울 코어 어플리케이션 파일인 app.go에는 아래와 같은 구성요소들이 존재함.

그리고 그렇게 생성된 state machine은

데이터 디렉토리에 위치하는 데이터베이스에서 최신 state와 높이를 추출하고 

Initialize the state-machine with the latest known state, extracted from the db stored in the ~/.app/data folder. At this point, the state-machine is at height appBlockHeight.


다른 피어와의 핸드쉐이킹을 통해서 현재의 앱의 블록높이보다 더 높은 스테이트를 전파받아서 싱크를 하게 되는 것입니다. 
Create and start a new CometBFT instance. Among other things, the node performs a handshake with its peers. It gets the latest blockHeight from them and replays blocks to sync to this height if it is greater than the local appBlockHeight. The node starts from genesis and CometBFT sends an InitChain message via the ABCI to the app, which triggers the InitChainer.
 -->

## Core application file

app.go 파일은 주로 애플리케이션의 타입 정의와 초기화 함수를 포함합니다.

애플리케이션 타입 정의
BaseApp 참조: baseapp를 확장한 애플리케이션 정의. baseapp은 대부분의 핵심 로직을 실현.
스토리지 키 리스트: 각 모듈은 자신의 상태를 저장하기 위해 여러 스토리지를 사용.
모듈 유지관리자 리스트: 각 모듈은 자신의 keeper를 정의.
appCodec 참조: 데이터를 시리얼라이즈하고 디시리얼라이즈하는데 사용.
legacyAmino 인코더 참조: 전통적인 Amino 인코더.
모듈 관리자와 기본 모듈 관리자: 모듈을 관리하고, 필요한 서비스와 쿼리 서비스 등을 등록.



ref; https://docs.cosmos.network/v0.47/learn/beginner/overview-app

<!-- 
1. Citation to baseapp: The self-defined application defined in app.go is the expansion of baseapp. When a transaction is forwarded to the application by Tendermint, the application uses the baseapp method to direct it to the appropriate module. baseapp realized most of the core logic of the application, including all ABCI methods and routing logic.

2. List of storage keys: storage that contains the entire state is realized in Cosmos SDK as a multi-storage （ or storage ）. Each module uses one or more of the multiple storage to save their state part. These stores can be accessed by specific keys declared in the application type. These keys and maintainers are the core of Cosmos SDK's target capability model.

3. Module Maintainer List: Each module defines an abstract called keeper, which handles the reading and writing of the memory of this module. The keeper method of one module can call （ if authorized, ） from other modules, which is why they are declared in the type of application and exported to other modules as an interface so that the latter can only access the authorized function.
4. Citation to appCodec: AppCodec of the application is used to sequence and counterorder data structures in order to store them, because storage can only perpetuate [] bytes. The default encoder is the protocol buffer.

5. Citation to legacyAmino encoder: Some parts of Cosmos SDK have not been migrated to appCodec above, and Amino is still used for hard coding. Other parts explicitly use Amino to achieve backward compatibility. For these reasons, the application still holds references to traditional Amino coders. Please note that the Amino encoder will be deleted from the upcoming SDK.

6. Citation to module managers and basic module managers: Module managers are the objects of a module list containing applications. It facilitates operations related to these modules, such as the Msg service and gRPC query service that register them, or the execution order between modules for various functions such as InitChainer, BeginBlocker, and EndBlocker.
 -->

그럼 위에 구성요소가 담긴 core application simapp의 app.go 를 살펴보겠습니다. 

```go
// SimApp extends an ABCI application, but with most of its parameters exported.
// They are exported for convenience in creating helper functions, as object
// capabilities aren't needed for testing.
type SimApp structure {
	*baseapp.BaseApp
	legacyAmino *codec.LegacyAmino
	appCodec codec.Codec
	interfaceRegistry types.InterfaceRegistry

	invCheckPeriod uint

	// keys to access the substores
	keys map[string]*storetypes.KVStoreKey
	tkeys map[string]*storetypes.TransientStoreKey
	memKeys map[string]*storetypes.MemoryStoreKey

	// keeps
	AccountKeeper authkeeper.AccountKeeper
	BankKeeper bankkeeper.Keeper
	CapabilityKeeper *capabilitykeeper.Keeper
	StakingKeeper stakingkeeper.Keeper
	SlashingKeeper slashingkeeper.Keeper
	MintKeeper mintkeeper.Keeper
	DistrKeeper distrkeeper.Keeper
	GovKeeper govkeeper.Keeper
	CrisisKeeper crisiskeeper.Keeper
	UpgradeKeeper upgradekeeper.Keeper
	ParamsKeeper paramskeeper.Keeper
	AuthzKeeper authzkeeper.Keeper
	EvidenceKeeper evidencekeeper.Keeper
	FeeGrantKeeper feegrantkeeper.Keeper
	GroupKeeper groupkeeper.Keeper
	NFTKeeper nftkeeper.Keeper

	// the module manager
	mm *module.Manager

	// simulation manager
	sm *module.SimulationManager

	// module configurator
	configurator module. Configurator
}
```



## Appchain Initiate Method (NewSimApp)
위에서 정의한 건 baseapp를 확장한 우리가 배울 simapp이란 블록체인 application의 struct 이고.

아래의 AppCreator func을 통해서 실제 struct를 이제 instance시킬 수 있는 function을 만들어 볼 예정

위에서 우리가 예시로 보았던 simapp이라고 하는 구조체의 instance를 초기화 시켜줄 new 함수입니다. 해당 함수는 cosmos-sdk에서 정의한 AppCreater라는 function signature를 준수해야합니다. 


### AppCreator Signature

This function constructs a new application with the type defined in the previous section. It must meet the AppCreator signature to be used in the start order of the application guardian program order.

```go
// AppCreator is a function that allows us to lazily initialize an
// application using various configurations.
AppCreator func(log.Logger, dbm.DB, io.Writer, AppOptions) Application
```

일단 이거 자체는 간단해 보이는데 이제 실제로 간단히 (나름? )

위에서 정의한 simapp을 initiate 해주는 NewApp(AppCreator)를 다 만들고 보면 좀 복잡함..

원래는 훨씬 더 복잡한데 일단 생략하고 넘어가겠습니다. 

```go
// NewSimApp returns a reference to an initialized SimApp.
func NewSimApp(
	logger log.Logger, db dbm.DB, traceStore io.Writer, loadLatest bool, skipUpgradeHeights map[int64]bool,
	homePath string, invCheckPeriod uint, encodingConfig simappparams.EncodingConfig,
	appOpts servertypes.AppOptions, baseAppOptions ...func(*baseapp.BaseApp),
) *SimApp {
	appCodec:= encodingConfig.Codec
	legacyAmino:= encodingConfig.Amino
	interfaceRegistry:= encodingConfig.InterfaceRegistry

	bApp:= baseapp.NewBaseApp(appName, logger, db, encodingConfig.TxConfig.TxDecoder(), baseAppOptions..)
	bApp. SetCommitMultiStoreTracer(traceStore)
	bApp.SetVersion(version.Version)
	bApp.SetInterfaceRegistry(interfaceRegistry)

	keys:= sdk.NewKVStoreKeys(
		authtypes.StoreKey, banktypes.StoreKey, stakingtypes.StoreKey,
		minttypes.StoreKey, districts.StoreKey, slashingtypes.StoreKey,
		govtypes.StoreKey, paramstypes.StoreKey, upgradetypes.StoreKey, feegrant.StoreKey,
		evidencepes.StoreKey, capabilitytypes.StoreKey,
		authzkeeper.StoreKey, nftkeeper.StoreKey, group.StoreKey,
	)
	tkeys:= sdk.NewTransientStoreKeys(paramstypes.TStoreKey)
	// NOTE: The testingkey is just mounted for testing purposes. Actual applications should
	// not include this key.
	memKeys:= sdk.NewMemoryStoreKeys(capabilitytypes.MemStoreKey, "testingkey")

	// configure state listening capabilities using AppOptions
	// we are doing nothing with the returned streaming Services and waitGroup in this case
	if _, _, err:= streaming.LoadStreamingServices(bApp, appOpts, appCodec, keys); err != nil {
		tmos.Exit(err.Error())
	}

	app:= &SimApp{
		BaseApp: bApp,
		legacyAmino: legacyAmino,
		appCodec: appCodec,
		interfaceRegistry: interfaceRegistry,
		invCheckPeriod: invCheckPeriod,
		keys: keys,
		tkeys: tkeys,
		memKeys: memKeys,
	}
	// set the BaseApp's parameter store

	// ..
	// ..
	// .. skipped some code bases
	// ..
	// 꽤 많은 코드들이 생략되었음.

	return app
}
```

위에서 본 코드들은 결국 아래와 같은 것들을 오퍼레이션 해주는 것임.

새 인코더를 예로 들어, 기본 관리 애플리케이션의 각 모듈에 대한 코드 인코더를 사용합니다.
baseapp 예시, 인코더 및 적절한 저장소 키를 참조하는 새 애플리케이션의 예시입니다.
애플리케이션을 사용하는 각 모듈의 NewKeeper 함수는 위에서 예시로 든 애플리케이션 유형에 정의된 모든 keeper를 사용합니다. NewKeeper가 다른 모듈의 keeper를 인용할 필요가 있기 때문에, 유지 관리자는 올바른 순서로 예시되어야 한다는 점에 유의해야 합니다.
애플리케이션의 각 모듈을 사용하는 AppModule 객체 예시 애플리케이션의 모듈 관리자입니다.
모듈 관리자를 사용하여 애플리케이션의 Msg 서비스, gRPC 쿼리 서비스, 전통적인 Msg 라우팅 및 레거시 쿼리 라우팅을 초기화합니다. Tendermint가 ABCI를 통해 애플리케이션으로 트랜잭션을 전달할 때, 이는 여기에서 정의된 라우팅을 사용하여 적절한 모듈의 Msg 서비스로 라우팅됩니다. 유사하게, 애플리케이션이 gRPC 쿼리 요청을 받으면 이는 gRPC 라우팅을 사용하여 해당 모듈의 gRPC 쿼리 서비스로 라우팅됩니다. Cosmos SDK는 여전히 전통적인 Msgs와 전통적인 Tendermint 쿼리를 지원하며, 이는 각각 전통적인 Msg 라우팅과 전통적인 쿼리 라우팅을 사용합니다.

모듈 관리자를 사용하여 애플리케이션의 모듈 변수를 등록합니다. 불변량은 각 블록 끝에서 평가됩니다. 불변량 검사 과정은 InvariantsRegistry라는 특별한 모듈을 통해 이루어집니다. 불변량 값은 이 모듈에 정의된 예상 값과 같아야 합니다. 이 값이 예상 값과 다르면 불변 등록 양식에 정의된 특별한 로직(일반적으로 체인 중지)이 실행됩니다. 이는 중요한 오류가 감지되지 않고 장기적으로 복구하기 어려운 영향을 미치는 것을 방지하는 데 매우 유용합니다.
모듈 관리자를 사용하여 애플리케이션의 각 모듈의 InitGenesis, BeginBlocker 및 EndBlocker 함수 간의 실행 순서를 설정합니다. 모든 모듈이 이러한 기능을 구현하지는 않았다는 점에 유의하세요.
애플리케이션의 나머지 매개변수를 설정합니다:


InitChainer: 애플리케이션을 처음 시작할 때 초기화하는 데 사용됩니다.
BeginBlocker, EndBlocker: 각 블록의 시작과 끝에서 호출됩니다.
anteHandler: 수수료 처리 및 서명 검증에 사용됩니다.
스토어 설치.
애플리케이션으로 반환합니다.


그리고 리턴된 이 어플리케이션을 실행시키면 지난 시간에 배운 run a node를 할 수 있습니다. (노드가 재시작될 때 ~/.app/data 폴더에서 실제 상태가 가져와지거나, 노드가 처음 활성화될 때 생성 파일에서 생성됩니다.)



## InitChainer

그 다음으로 배울 건 initChainer라는 녀석임.

```go
// InitChainer application update at chain initialization
func (app *SimApp) InitChainer(ctx sdk.Context, req abci.RequestInitChain) abci.ResponseInitChain {
	var genesisState GenesisState
	if err:= json.Unmarshal(req.AppStateBytes, &genesisState); err != nil {
		panic(err)
	}
	app.UpgradeKeeper.SetModuleVersionMap(ctx, app.mm.GetVersionMap())
	return app.mm.InitGenesis(ctx, app.appCodec, genesisState)
}
```

이 녀석이 해주는 역할은 결국에 0번째 블록을 실행시켜서 첫번째 블록은 block1을 위한 스테이트를 init해주는 걸 뜻함.

우리가 지난 시간에 gentx를 만들어서 genesis에 넣었던게 기억날지 모르겠는데
만약 그 과정에서 staking을 안했다면 initChainer로부터 에러를 받아.

그리고 만약 제네시스에 올바르지 않은 signature txs가 있더라도 마찬가지

InitChainer is a function that initializes the status of the application from a genesis file （ or the token balance of the genesis account ）.

When the application receives InitChain news from the Tendermint engine, it will be called, which occurs when the node starts at appBlockHeight ==0（ or genesis）. The application must be set up as InitChainer by SetInitChainer in its constructor.

Generally speaking, InitChainer is mainly composed of the InitGenesis function of each module of the application. This is done by calling the InitGenesis function of the module manager, and the module manager will call the InitGenesis function of each module it contains. Please note that the SetOrderInitGenesis method of the module manager must be used to set the order of the InitGenesis function of the call module in the module manager. This is done in the tectonic function of the application, SetOrderInitGenesis must call before SetInitChainer.

## BeginBlocker and EndBlocker

원래는 모듈 파트에서 다룰 예정이지만, 
다음으로 배울 파트는 beginblocker & endblocker야 ethereum이나 다른체인에도 비슷한 logic이나 component가 있는지 정확히 모르겠는데.

이 파트가 매력적이어서 코스모스로 넘어온 체인들도 꽤 있어 내가 알기로는 injective와 같은 체인이 이 endblocker를 통해서 재밌는 로직을 넣어서

dex application을 더 최적화 시켰어. (https://youtu.be/p3rK4rBmy9U?si=vrXHiVzsTx-de-Dt)

결국 dapp들이 이런 부분들 때문에 l2나 다른 ecocsytem의 general purpose vm application에서 soverign application으로 전환하는 니즈가 있는 것 같애

(dydx도 마이그레이션 함)

```go
// BeginBlocker application updates every begin block
func (app *SimApp) BeginBlocker(ctx sdk.Context, req abci.RequestBeginBlock) abci.ResponseBeginBlock {
	return app.mm.BeginBlock(ctx, req)
}

// EndBlocker application updates every end block
func (app *SimApp) EndBlocker(ctx sdk.Context, req abci.RequestEndBlock) abci.ResponseEndBlock {
	return app.mm.EndBlock(ctx, req)
}
```

Cosmos SDK provides developers with the possibility to implement automatic execution codes as part of their applications. This is achieved through two functions called BeginBlocker and EndBlocker. When the applications received news from BeginBlock and EndBlock from the Tendermint engine, they were called, which happened at the beginning and end of each block. The application must be set up BeginBlocker and EndBlocker in its constructor by SetBeginBlocker and SetEndBlocker methods.

Generally speaking, the BeginBlocker and EndBlocker functions are mostly composed of BeginBlock and EndBlock functions for each module of the application. This is done by calling the BeginBlock and EndBlock functions of the module manager, and the module manager will call the BeginBlock and EndBlock functions of each module it contains. Please note that the order of calling the BeginBlock and EndBlock functions of the module must be set in the module manager using the SetOrderBeginBlockers and SetOrderEndBlockers methods, respectively. This is done through the module manager in the tectonic function of the application. The SetOrder BeginBlockers and SetOrderEndBlockers methods must be called before the functions of SetBeginBlocker and SetEndBlocker.

## Registered encoder

코덱은 말 그대로 코덱인데

InterfaceRegistry. InterfaceRegistry is used by Protobuf encoder to handle the interface of using google.protobuf.Any for coding and decoding （ We also say “ unpacking ”）. Any can be considered as a structure containing the specific type of name （ and value） of the type_url（ to realize the interface. InterfaceRegistry provides a registration interface and implementation mechanism that can be safely unpacked from Any. Each module of the application implements the RegisterInterfaces method, which can be used to register the module's own interface and implementation.

In order to understand the details in more detail, Cosmos SDK uses the implementation of the Protobuf specification, called gogoprotobuf. In the default case, Any realized by gogo protobuf uses a global type registration to decode the value in Any into a specific Go type. This introduces a loophole. Relying on any malicious module in the tree can register a type in the global plotobuf registration form and cause it to be loaded and decrypted by a transaction cited in the typo of type_url.

You can read more about Any in ADR-19.

https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-019-protobuf-state-encoding.md

Architecture Decision Records 를 ADR이라고 하는데 다른 프로토콜에서의 EIP 같은 거라고 보면됨.

https://docs.cosmos.network/main/build/architecture
(앞으로도 굳이 다 설명하지 않고 필요한게 있을 때 그때 같이 엮어서 이렇게 볼 예정)

다시 원래 코덱 얘기로 돌아와서

- Codec: The default coder used by the entire Cosmos SDK. It consists of BinaryCodec for coding and decoding status and JSONCodec for exporting data to users （ for example ） in CLI. In the default case, SDK uses Protobuf as a encoder.

- TxConfig: TxConfig defines an interface where the client can use it to generate a specific transaction type defined by the application. Currently, SDK handles two types of transactions: SIGN_MODE_DIRECT（ uses Protobuf binary as an online code ） and SIGN_MODE_LEGACY_AMINO_JSON（ relies on Amino）.

```go
// EncodingConfig specifies the concrete encoding types to use for a given app.
// This is provided for compatibility between protobuf and amino implementations.
type EncodingConfig structure {
	InterfaceRegistry types. InterfaceRegistry
	Codec codec. Codec
	TxConfig client. TxConfig
	Amino *codec.LegacyAmino
}
```




---

끝으로 baseapp .. 


# Baseapp

BaseApp is the core basic type that realizes the Cosmos SDK application, namely：

- The application block link is used for communication between the status machine and the bottom consensus engine （ such as Tendermint）.

- Service router, directing messages and query routing to appropriate modules.

- Different states, because the state machine can update different fluctuation states based on the ABCI news received.

BaseApp's goal is to provide the basic layer of the Cosmos SDK application, and developers can easily expand to establish their own self-defined applications. Usually, developers will create a custom type for their applications, as follows：

```go
type App struct {
  // reference to a BaseApp
  *baseapp.BaseApp

  // list of application store keys

  // list of application keepers

  // module manager
}

```

Using BaseApp expansion app, the former can access all BaseApp methods. This allows developers to use the modules they want to form their self-defined applications without having to care about the hard work of achieving ABCI, service routers, and state management logic.

```go
// BaseApp reflects the ABCI application implementation.
type BaseApp struct { // nolint: maligned
	// initialized on creation
	logger            log.Logger
	name              string               // application name from abci.Info
	db                dbm.DB               // common DB backend
	cms               sdk.CommitMultiStore // Main (uncached) state
	storeLoader       StoreLoader          // function to handle store loading, may be overridden with SetStoreLoader()
	router            sdk.Router           // handle any kind of message
	queryRouter       sdk.QueryRouter      // router for redirecting query calls
	grpcQueryRouter   *GRPCQueryRouter     // router for redirecting gRPC query calls
	msgServiceRouter  *MsgServiceRouter    // router for redirecting Msg service messages
	interfaceRegistry types.InterfaceRegistry
	txDecoder         sdk.TxDecoder // unmarshal []byte into sdk.Tx

	anteHandler    sdk.AnteHandler  // ante handler for fee and auth
	initChainer    sdk.InitChainer  // initialize state with validators and state blob
	beginBlocker   sdk.BeginBlocker // logic to run before any txs
	endBlocker     sdk.EndBlocker   // logic to run after all txs, and to determine valset changes
	addrPeerFilter sdk.PeerFilter   // filter peers by address and port
	idPeerFilter   sdk.PeerFilter   // filter peers by node ID
	fauxMerkleMode bool             // if true, IAVL MountStores uses MountStoresDB for simulation speed.

	// manages snapshots, i.e. dumps of app state at certain intervals
	snapshotManager    *snapshots.Manager
	snapshotInterval   uint64 // block interval between state sync snapshots
	snapshotKeepRecent uint32 // recent state sync snapshots to keep

	// volatile states:
	//
	// checkState is set on InitChain and reset on Commit
	// deliverState is set on InitChain and BeginBlock and set to nil on Commit
	checkState   *state // for CheckTx
	deliverState *state // for DeliverTx

	// an inter-block write-through cache provided to the context during deliverState
	interBlockCache sdk.MultiStorePersistentCache

	// absent validators from begin block
	voteInfos []abci.VoteInfo

	// paramStore is used to query for ABCI consensus parameters from an
	// application parameter store.
	paramStore ParamStore

	// The minimum gas prices a validator is willing to accept for processing a
	// transaction. This is mainly used for DoS and spam prevention.
	minGasPrices sdk.DecCoins

	// initialHeight is the initial height at which we start the baseapp
	initialHeight int64

	// flag for sealing options and parameters to a BaseApp
	sealed bool

	// block height at which to halt the chain and gracefully shutdown
	haltHeight uint64

	// minimum block time (in Unix seconds) at which to halt the chain and gracefully shutdown
	haltTime uint64

	// minRetainBlocks defines the minimum block height offset from the current
	// block being committed, such that all blocks past this offset are pruned
	// from Tendermint. It is used as part of the process of determining the
	// ResponseCommit.RetainHeight value during ABCI Commit. A value of 0 indicates
	// that no blocks should be pruned.
	//
	// Note: Tendermint block pruning is dependant on this parameter in conunction
	// with the unbonding (safety threshold) period, state pruning and state sync
	// snapshot parameters to determine the correct minimum value of
	// ResponseCommit.RetainHeight.
	minRetainBlocks uint64

	// application's version string
	appVersion string

	// recovery handler for app.runTx method
	runTxRecoveryMiddleware recoveryMiddleware

	// trace set will return full stack traces for errors in ABCI Log field
	trace bool

	// indexEvents defines the set of events in the form {eventType}.{attributeKey},
	// which informs Tendermint what to index. If empty, all events will be indexed.
	indexEvents map[string]struct{}
}
```

#### References
https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy
https://docs.cosmos.network/v0.45/core/baseapp.html
