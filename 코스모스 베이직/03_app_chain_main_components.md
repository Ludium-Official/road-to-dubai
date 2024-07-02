# 03. App chain main Components

### Preview

<!-- TODO : preview 멘트 적을 것 -->

---

### Cosmos-SDK based app chain architecture

저희 다룰 코스모스 앱 체인에 대한 아키텍쳐에 대해서 설명할 차례입니다.

아래 그림은 이미 지난 시간에 보았던 그림이라 눈에 좀 더 익숙해졌을 것입니다. 

```sh
                ^  +-------------------------------+  ^
                |  |                               |  |   Built with Cosmos SDK
                |  |  State-machine = Application  |  |
                |  |                               |  v
                |  +-------------------------------+
                |  |                               |  ^
Blockchain node |  |           Consensus           |  |
                |  |                               |  |
                |  +-------------------------------+  |   CometBFT
                |  |                               |  |
                |  |           Networking          |  |
                |  |                               |  |
                v  +-------------------------------+  v
```

하지만 사실상 위의 구조는 아래와 같이도 표현될 수 있습니다. 그리고 우리가 앱체인이라고 부르는 이유는

아래의 Consensus & Networking Level(=CometBFT or Tendermint Level) 은 최대한 신경쓰지 않고 어플리케이션에 집중해서 개발할 수 있기 때문입니다. 

딱 일반적인 개발에서의 SDK의 성격과 같습니다. 다만, 대부분의 Cosmos SDK에서는 위에서 이미 말씀드렸던 대로 CometBFT(=Tendermint) 를 컨센서스로 사용하기 때문에 부분적으로 개발자들도 DPoS의 개념과 PBFT의 개념에 대해서 숙지할 필요가 있습니다만. 당장에 우리가 앱체인을 개발하는데 필요하진 않기 때문에 생략하고 넘어가도록 하겠습니다. 

이 부분에 대해서 좀 더 관심이 있으신 분들은 이 [링크](https://docs.cometbft.com/v0.37/introduction/what-is-cometbft)를 통해서 미리 한번 보는 것도 좋을 것 같습니다. 

(앞으로는 Tendermint가 아닌 CometBFT로 통일해서 말하도록 하겠습니다)

```sh
              +---------------------+
              |                     |
              |     Application     |       -> Cosmos-SDK
              |                     |
              +--------+---+--------+
                       ^   |
                       |   | ABCI
                       |   v
              +--------+---+--------+
              |                     |
              |                     |
              |       CometBFT      |
              |                     |
              |                     |
              +---------------------+
```

### ABCI

지난 시간과 이번시간을 통해서 우린 코스모스 SDK를 이용한 앱체인 구조에 대해서 많이 알게 되었는데요. 

눈치가 빠르신 개발자분들이라면 아셨겠지만. 우리가 만들 어플리케이션 레벨의 app과 실질적인 컨센서스를 담당하는 cometbft레벨은 ABCI라고 하는 interface로 분리가 되어있습니다. 

따라서, 저번에도 말했듯이 반드시 CometBFT & Cosmos-SDK based application 조합을 구성할 필요는 없으며 쌍으로 된 인터페이스만을 준수한다면 어떤 조합이던 가능합니다. 다만 ethereum과 같이 여러 language로 구성된 client의 부재로 대부분 CometBFT & Cosmos-SDK 조합으로 체인이 개발됩니다.

그렇지만, 재미난 점은 ABCI에 있습니다. 최근 Cosmos Ecocsystem상에서 주목받고 있는 체인들은 대부분 ABCI++라고 하는 기존 ABCI에서 보다 확장된 ABCI extension, ABCI++를 도입하고 있습니다. 

이에 관해서는 과제로 내드릴테니 각자 찾아보고 다음시간에 공유하는 시간을 가지면 좋을 것 같습니다.

> The ABCI also allows developers to swap the consensus engine of their application-specific blockchain. Today, only CometBFT is production-ready, but in the future other consensus engines are expected to emerge.

**과제**

ABCI++

ref; 
1. https://docs.cometbft.com/v0.37/spec/abci/
2. https://skip-protocol-docs.netlify.app/about/faq/
3. https://informal.systems/blog/abci-v2-unlocks-this

ABCI를 관련된 내용을 끝마치기 전에 알고 넘어가야할 내용에 대해서 설명드리겠습니다.

> Note that CometBFT only handles transaction bytes. It has no knowledge of what these bytes mean. All CometBFT does is order these transaction bytes deterministically. CometBFT passes the bytes to the application via the ABCI, and expects a return code to inform it if the messages contained in the transactions were successfully processed or not.

위 영어로 된 내용에서 보셨듯이 이런 형태로 ABCI 인터페이스를 통해서 각 노드 클라이언트가 동작한다고 유추할 수 있습니다.

```sh
                   +---------------------+
                   |                     |
                   |     Application     |       -> Cosmos-SDK
                   |                     |
                   +--------+---+--------+
        (ABCI)              ^   |             (ABCI)
received transaction bytes->|   | <- return codes(success or not)
                            |   v
                   +--------+---+--------+
                   |                     |
                   |                     |
                   |       CometBFT      |
                   |                     |
                   |                     |
                   +---------------------+
```


정말 마지막으로! ABCI의 가장 중요한 메시지는 다음과 같습니다:

- CheckTx: CometBFT가 트랜잭션을 받으면, 애플리케이션에 전달되어 몇 가지 기본 요구 사항을 충족하는지 확인합니다. CheckTx는 풀 노드의 메모풀을 스팸 트랜잭션으로부터 보호하는 데 사용됩니다. AnteHandler라는 특별한 핸들러가 충분한 수수료 확인 및 서명 검증과 같은 일련의 검증 단계를 실행하는 데 사용됩니다. 검증이 유효하면 트랜잭션이 메모풀에 추가되고 피어 노드에 릴레이됩니다. CheckTx에서는 트랜잭션이 블록에 포함되지 않았기 때문에 상태가 수정되지 않습니다.

- DeliverTx: CometBFT가 유효한 블록을 받으면, 블록의 각 트랜잭션은 DeliverTx를 통해 애플리케이션에 전달되어 처리됩니다. 이 단계에서 상태 전환이 발생합니다. AnteHandler가 다시 실행되며, 트랜잭션의 각 메시지에 대한 실제 Msg 서비스 RPC도 실행됩니다.

- BeginBlock/EndBlock: 이 메시지는 블록이 트랜잭션을 포함하든 않든 각 블록의 시작과 끝에서 실행됩니다. 자동으로 로직을 실행하는 데 유용합니다. 그러나 계산적으로 비싼 루프는 블록체인을 느리게 하거나 루프가 무한하면 블록체인을 멈출 수 있으므로 주의해서 사용해야 합니다.

더 자세한 ABCI 메서드에 대한 설명은 CometBFT 문서에서 확인할 수 있고, 간단히 ABCI에 구현된 CheckTx와 DeliverTx를 통해서 interface method가 구성되어있다고만 보시면 됩니다. 그렇다면, 개발자들로서 이러한 인터페이스를 항상 자기의 appchain application레벨(app.go)에서 구현해야한다고 생각할 수도 있겠지만. 다행히도 ABCI 인터페이스를 직접 구현할 필요는 없습니다. Cosmos SDK는 baseapp 형태로 기본 구현을 제공하기 때문입니다. 


<!-- 

생략된 파트 

### Why we need to use Cosmos SDK for building a app-chain？

Cosmos SDK is the most advanced framework built today by defining specific block chains of applications. The following are several reasons for considering the use of Cosmos SDK to construct a decentralized application：

The default consensus engine in Cosmos SDK is Tendermint Core. Tendermint is the most existing （ and the only ） mature BFT consensus engine. It is widely used in the entire industry and is considered to be the golden standard consensus engine for building a pile certification system.
Cosmos SDK is open source, and its design purpose is to facilitate the construction of block chains from combustible modules. With the development of the open source Cosmos SDK module ecosystem, it will become easier and easier to build a complex de-centralized platform with it.
Cosmos SDK was inspired by function-based security and was inspired by years of struggle with block chain state machines. This makes Cosmos SDK a very safe environment for building block chains.
Most importantly, Cosmos SDK has been used to construct many application-specific block chains that have been put into production. Among them, we can quote Cosmos Hub, IRIS Hub, Binance Chain, Terra or Kava）.moreBased on Cosmos SDK construction. -->

---

### Main Components of the Cosmos SDK

Cosmos SDK의 주요 구성 요소는 크게 3가지 정도가 있습니다. 

1. baseapp
2. multistore
3. modules

모든 내용을 이번 아티클에서 깊숙히 배우지는 않을 것이고 위의 맥락과 이어지는 부분만 다루도록 하겠습니다. 

우선! 우린 위에서 이미 comsos-sdk를 이용한 앱 체인 어플리케이션이 application 레벨과 cometbft레벨로 나뉘어진 아키텍쳐를 배웠습니다. 

이제 그럼, 그 아키텍쳐 상에서 트랜잭션이 어떻게 처리되는지 순서를 알아봅시다. 


```sh
                   +---------------------+
                   |                     |
                   |     Application     |       -> Cosmos-SDK
                   |                     |
                   +--------+---+--------+
        (ABCI)              ^   |             (ABCI)
received transaction bytes->|   | <- return codes(success or not)
                            |   v
                   +--------+---+--------+
                   |                     |
                   |                     |
                   |       CometBFT      |
                   |                     |
                   |                     |
                   +---------------------+
```

Cosmos SDK 위에 구축된 애플리케이션에서 CometBFT를 통해 DeliverTx로 전송된 트랜잭션이 처리되는 방식은 다음과 같습니다:

1. CometBFT 합의 엔진에서 받은 `transactions`을 디코딩합니다 (CometBFT는 `[]bytes`만 다룹니다).
2. `transactions`에서 `messages`를 추출하고 기본적인 무결성 검사를 수행합니다.
3. 각 `messages`를 적절한 모듈로 라우팅하여 처리합니다.
4. 상태 변경을 커밋합니다.

자 그럼 이제 하나씩 메인 컴포넌트를 살펴봅시다.

#### 1. baseapp

먼저, baseapp입니다. 
baseapp은 Cosmos SDK 애플리케이션의 표준 구현입니다. 이는 기본 합의 엔진과의 연결을 처리하기 위해 ABCI의 구현을 제공합니다. 일반적으로 Cosmos SDK 애플리케이션은 app.go 파일에서 baseapp을 포함하여 확장합니다.

다음은 Cosmos SDK 데모 애플리케이션인 simapp에서의 예시입니다:

```go
// SimApp extends an ABCI application, but with most of its parameters exported.
// They are exported for convenience in creating helper functions, as object
// capabilities aren't needed for testing.
type SimApp struct {
	*baseapp.BaseApp
	legacyAmino       *codec.LegacyAmino
	appCodec          codec.Codec
	txConfig          client.TxConfig
	interfaceRegistry types.InterfaceRegistry

	// keys to access the substores
	keys  map[string]*storetypes.KVStoreKey
	tkeys map[string]*storetypes.TransientStoreKey

	// keepers
	AccountKeeper         authkeeper.AccountKeeper
	BankKeeper            bankkeeper.Keeper
	StakingKeeper         *stakingkeeper.Keeper
	SlashingKeeper        slashingkeeper.Keeper
	MintKeeper            mintkeeper.Keeper
	DistrKeeper           distrkeeper.Keeper
	GovKeeper             govkeeper.Keeper
	CrisisKeeper          *crisiskeeper.Keeper
	UpgradeKeeper         *upgradekeeper.Keeper
	ParamsKeeper          paramskeeper.Keeper
	AuthzKeeper           authzkeeper.Keeper
	EvidenceKeeper        evidencekeeper.Keeper
	FeeGrantKeeper        feegrantkeeper.Keeper
	GroupKeeper           groupkeeper.Keeper
	NFTKeeper             nftkeeper.Keeper
	ConsensusParamsKeeper consensusparamkeeper.Keeper
	CircuitKeeper         circuitkeeper.Keeper

	// the module manager
	ModuleManager      *module.Manager
	BasicModuleManager module.BasicManager

	// simulation manager
	sm *module.SimulationManager

	// module configurator
	configurator module.Configurator
}
```

#### 2. Multistore

Cosmos SDK에서는 블록체인의 상태를 영구적으로 저장하기 위한 멀티스토어를 제공합니다. 

멀티스토어는 개발자가 원하는 수의 KVStores를 선언할 수 있게 합니다. 이러한 KVStores는 값으로 `[]byte` 타입만을 수용하므로, 저장하기 전에 코덱을 사용하여 커스텀 구조를 마샬링(encoding)해야 합니다.

멀티스토어 상태는 블록과 구분되어있어서, 각각의 모듈에서 관리가 됩니다. 

보다 자세한 내용은 store article에서 다뤄집니다.

#### 3. Modules

Cosmos SDK의 강점은 모듈성에 있습니다. 우리가 위에서 배웠던 아키텍쳐에 따라 Cosmos SDK는 각 메시지를 적절한 모듈로 라우팅하는 역할을 합니다.

Cosmos SDK 애플리케이션인 앱체인들은 여러 모듈을 모아서 `app.go`에서 선언한 뒤에 앱체인이 생성됩니다. 기존에는 한 앱체인에 custom module을 구현하였으나 최근에는 각 모듈들이 하나의 library형태로 분리되어 있기도해서 이식성이 좋습니다.

각 모듈은 기본적으로 Cosmos SDK에서 정의한 interface를 만족하게 되며, 모듈 자체적으로 모듈 상태에 대한 하위 집합의 정의하고 자체 `Messages/Transactions` process 구현하게 됩니다. 

각 모듈은 작은 상태 기계로 볼 수 있습니다. 개발자는 모듈이 처리하는 상태의 하위 집합과 상태를 수정하는 커스텀 메시지 유형을 정의해야 합니다. 일반적으로 각 모듈은 멀티스토어에 자신의 KVStore를 선언하여 정의된 상태의 하위 집합을 영구 저장합니다. 

게다가 일부 모듈의 경우는 다른 서드 파티 모듈에 대해 접근할 필요가 있습니다. 따라서, 모듈 간 상호 작용에 대한 보안 원칙이 필요한데 이때 다른 모듈을 위한 액세스 제어 목록을 유지하는 대신, 각 모듈이 다른 모듈에 전달될 수 있는 특수 객체인 `keeper`라는 것을 구현하고 사용하게 됩니다. 보다 자세한 내용은 모듈파트에서 다루도록 합시다.

마지막으로 기본적으로 SDK에 내장된 모듈들은 Cosmos SDK의 x/ 폴더에 정의됩니다. 

끝으로 우리가 이번 시간에 배운 내용들을 정리한 아키텍쳐를 보는 것으로 아티클을 마치고, 아키텍쳐 이해를 위해 이 [영상](https://youtu.be/1_ottIKPfI4?si=XstKA2YGi2-yYKzF)을 한번씩 보는 것도 추천드립니다.

```sh
                +---------------------------------------------+
                |        CometBFT (Consensus & Networking)    |
                +---------------------|-----------------------+
                                      v 
                                    ABCI Interface            
                                      |
                                      v
                                      +
                                      |
                                      |  Transaction relayed from the full-node's
                                      |  CometBFT engine to the node's application
                                      |  via DeliverTx
                                      |
                                      |
                +---------------------v--------------------------+
                |                 APPLICATION                    |
                |                                                |
                |     Using baseapp's methods: Decode the Tx,    |
                |     extract and route the message(s)           |
                |                                                |
                +---------------------+--------------------------+
                                      |
                                      |
                                      |
                                      +---------------------------+
                                                                  |
                                                                  |
                                                                  |  Message routed to
                                                                  |  the correct module
                                                                  |  to be processed
                                                                  |
                                                                  |
+----------------+  +---------------+  +----------------+  +------v----------+
|                |  |               |  |                |  |                 |
|  AUTH MODULE   |  |  BANK MODULE  |  | STAKING MODULE |  |   GOV MODULE    |
|                |  |               |  |                |  |                 |
|                |  |               |  |                |  | Handles message,|
|                |  |               |  |                |  | Updates state   |
|                |  |               |  |                |  |                 |
+----------------+  +---------------+  +----------------+  +------+----------+
                                                                  |
                                                                  |
                                                                  |
                                                                  | (ABCI Interface)
                                       +--------------------------+
                                       |
                                       | Return result to CometBFT
                                       | (0=Ok, 1=Err)
                                       v
                +---------------------------------------------+
                |        CometBFT (Consensus & Networking)    |
                +---------------------------------------------+                                       
```

#### References 
- https://docs.cosmos.network/v0.50/learn/intro/sdk-app-architecture
- https://docs.cosmos.network/v0.50/learn/intro/sdk-design
