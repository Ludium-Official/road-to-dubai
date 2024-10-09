# Cosmos-SDK dApp 만들기

## Cosmos-SDK 기초

Cosmos-SDK는 쉽게 PoS(Proof-of-Stake) 기반의 앱체인을 제작할 수 있는 툴킷으로, Cosmos-SDK를 사용해 제작된 체인들을 앱체인이라고 주로 말한다.

Cosmos-SDK는 Consensus 레이어는 Tendermint 또는 CombetBFT를 활용하고 개발자는 Application레이어만 신경쓰도록 설계된 SDK 이다.

```
                  ^  +-------------------------------+  ^
                  |  |                               |  |   Application Layer
                  |  |  State-machine = Application  |  |   Built with Cosmos SDK
                  |  |                               |  v
                  |  +-------------------------------+
                  |  |                               |  ^
Build with Cosmos |  |           Consensus           |  |
                  |  |                               |  |   Consensus Layer
                  |  +-------------------------------+  |   CometBFT
                  |  |                               |  |
                  |  |           Networking          |  |
                  |  |                               |  |
                  v  +-------------------------------+  v
```

### Cosmos-SDK 모듈

Cosmos-SDK는 기본 모듈을 제공함으로서 아래와 같은 기능을 기본으로 제공한다.(v0.50.x 기준)

- x/auth : 계정과 관련된 기능

- x/authz : 다른 계정으로 권한을 위임할 수 있다.(대신 Transaction을 전송 할 수 있다)

- x/bank : 계정에 토큰 상태를 관리하고, 전송할 수 있다.

- x/staking, Distribution : dPoS 관련된 관련된 기능을 한다.(위임, 클레임)

- x/governance : 앱체인 자체적으로 거버넌스를 운영할 수 있는 기능을 제공한다.(제안, 투표, 결과를 통한 상태 변경)

- x/mint : 인플레이션 관련 토큰을 민팅하는 기능

- x/feegrant : 수수료 대납 관련

- x/nft : ERC721과 같은 스펙의 NFT 지원

- 이외에도 체인을 운영하기 위한 Slashing, Genutil, Circuit, Upgrade, Consensus, Evidence, Crisis 등 모듈이 있다.

기본 모듈을 아니지만 아래 주요 모듈과 라이브러리들이 존재한다.

- x/liquidstaking : 적용한 앱체인에 대해서 스테이킹한 토큰을 tokenize하여 활용 가능하다.

- x/ethermint : EVM을 앱체인에서 사용할 수 있도록 지원하는 모듈(SmartContracts)

- x/wasm : CosmWasm을 앱체인에서 사용할 수 있도록 지원하는 모듈(SmartContracts)

- ibc-go : 앱체인간 토큰, 데이터 전송을 가능하게 하는 모듈

- ibc-apps : packet forward middleware라는 ibc로 특정 데이터를 전송하고 수신한 체인에서 자동으로 처리되는 기능이 구현 가능하다.

### Cosmos-SDK Smart Contract

초창기 Cosmos-SDK에는 Smart Contract가 없었다. 따라서 새로운 기능을 구현하기 위해서는 기본 제공 모듈이 아닌 Custom Module을 만들어 새로운 State를 관리하고 그에 맞는 기능을 구현하였다.

Custom Module을 반영하기 위해선

1. 모듈 개발 및 릴리즈
2. 거버넌스를 통한 체인 업그레이드(Validator(네트워크 참여자)가 업그레이드를 진행)

위 절차를 진행한다. 위 절차는 체인 코어 개발자, Contribution 등 진입장벽이 높아 Public 하지 않다고 볼 수 있다. 따라서 Smart Contract가 발전하게 된다.

Cosmos-SDK서는 Smart Contract를 지원하기 위한 모듈로는 Cosmwasm, Ethermint이 있다.

- Cosmwasm https://cosmwasm.com/

  - WebAssembly(Wasm) 기반으로 구축

  - Rust 기반

  - Cosmsos-SDK Standard를 제공하여 Wasm 프로그래밍 할 때 앱체인 모듈에 접근할 수 있다.

  - Permission 지정 등 앱체인에 특화된 기능을 많이 가지고 있다.

  - Cosmwasm을 활요하는 주요 체인

    - Neutron, Juno, Osmosis, Stargaze ...

- Ethermint https://github.com/evmos/ethermint

  - Ethereum Virtual Machine(EVM)과 호환

  - 다른 체인에서 활용하는 EVM Smart Contract를 가져와 바로 활용할 수 있다.

  - 주로 EVM 주소를 활용하며, Metamask에 EVM용 RPC를 제공하면 EVM 체인과 같은 사용성을 가질 수 있다.

  - Precompile해 둔 contract를 바탕으로 앱체인의 네이티브 기능도 사용할 수 있다.

  - Ethermint를 활용하는 주요 체인

    - Kava, Evmos, Canto ...

### Cosmos-SDK 상태 조회

Cosmos-SDK는 상태 값을 조회할 수 있도록 REST API / gRPC 방식 모두 제공한다.

- 실제 노드를 운영하지 않는 프로젝트에선, Public Endpoint를 찾거나, API 서비스를 찾아 사용하게 된다.

- Production 레벨에서는 API 서비스 이용 또는 직접 노드 운영해야 프로젝트의 안정성을 줄 수 있다.

- 미션을 위해서는 cosmos-kit의 훅을 통해 제공되는 public endpoint를 사용한다.

Cosmos-SDK에서 Amino, Direct, Text 방식으로 Message Sign을 제공한다. 각 방식은 서명 데이터를 직렬화(인코딩)하는 방법이 다르다.

- Amino : Cosmos SDK의 초기 버전에서 사용된 직렬화 방식, 구조화된 데이터를 사용

- Direct 방식 : Protobuf를 통해 직렬화, 효율적 / 최적화된 형식, 권장

- Text 방식 : Human Readable한 형식으로 직렬화(JSON 또는 YAML), 보안성과 성능 측면에서 바이너리 직렬화 방식보다는 비효율

| 방식   | 직렬화 방식         | 주요 용도                                     | 장단점                                     |
| ------ | ------------------- | --------------------------------------------- | ------------------------------------------ |
| Amino  | 바이너리/텍스트     | 초기 Cosmos SDK 사용                          | Protobuf로 대체 중, 새로운 앱에서는 비권장 |
| Direct | Protobuf (바이너리) | 최신 Cosmos SDK 사용                          | 효율적이고 최적화된 바이너리 형식          |
| Text   | JSON/YAML (텍스트)  | 디버깅, 로깅, 사람이 읽을 수 있는 데이터 확인 | 사람이 읽기 쉽지만 덜 효율적               |

## dApp 개발을 위한 기초 지식

- dApp은 Blockchain Network(앱체인)을 실제 유저가 활용하는 애플리케이션(웹 / 모바일 웹 / 모바일 앱)으로, 블록체인에 있는 상태 값을 조회 또는 저장하여 활용하는 어플리케이션으로 볼 수 있다. 모바일 지갑, 익스텐션 지갑, Explorer 등 체인의 정보를 활용하는 것을 모두 dApp으로 볼 수 있다.

- 앱 체인에서 토큰 전송을 예를 들면

  ```
      ^  +-------------------------------+  ^
      |  | Blockchain                    |  |
      |  |                               |  |
      |  |   auth                        |  |  계정의 사인 / 검증을 한다.
      |  |   bank                        |  |  계정의 토큰 보유 상태를 관리한다.
      |  |                               |  |
      v  +-------------------------------+  v

      ^  +-------------------------------+  ^
      |  | dApp (UI)                     |  |
      |  |                               |  |
      |  |   address                     |  |  연결된 지갑을 통해 현재 주소를 가져온다.
      |  |   balance                     |  |  체인에 있는 토큰 상태를 조회한다.
      |  |   send                        |  |  연결된 지갑에 토큰 전송 메시지를 전달한다.
      |  |                               |  |
      v  +-------------------------------+  v

      ^  +-------------------------------+  ^
      |  | Wallet                        |  |
      |  |                               |  |
      |  |   account                     |  |  계정을 관리한다(개인키 니모닉 등)
      |  |   balance                     |  |  체인에 있는 토큰 상태를 조회한다.
      |  |   sign                        |  |  dApp에서 요청받은 메시지를 키를 이용해 사인 및 전송한다.
      |  |                               |  |
      v  +-------------------------------+  v
  ```

## Cosmos 생태계에서 지원을 하는 dApp 개발을 위한 툴 킷

- CosmJS https://github.com/cosmos/cosmjs

  - Cosmos-SDK의 대표 typescript/javascript 라이브러리로 앱체인과 통신합니다.

  - CosmJS 라이브러리에서 하는 일

    - 니모닉/개인키를 활용해 Message를 Sign & Broadcast합니다.

    - Cosmostation, Keplr과 같은 지갑의 도움을 받아 사용자 서명을 요청합니다.(OfflineSigner)

    - 블록체인 네트워크에 State 정보를 Query 할 수 있습니다.

    - Cosmos SDK 모듈에서 발생하는 이벤트를 수신합니다.

- cosmos-kit https://cosmology.zone/products/cosmos-kit

  - cosmos-kit은 Cosmos생태계의 지갑 Cosmostation, Keplr, Leap 등 다양한 지갑연결 및 CosmJS를 활용하여 Cosmos-SDK를 활용한 앱체인들과 쉽게 통신할 수 있다.

  - create-cosmos-app을 활용해서 다양한 예제를 제공하고 있다.

- Mintscan https://mintscan.io

  - 코스모스 생태계의 대표적인 Explorer

  - 메인넷, 테스트넷 등 다양한 체인을 지원하고 있으며 각 체인별 정보와 통계 뿐만 아니라 Cosmos 생태계 정보를 한번에 볼 수 있다.

  - 지갑이 내장되어 있어 x/bank, x/staking, x/governance 등 Cosmos 주요 기능을 바로 활용 가능하다.

- Ignite https://docs.ignite.com/

  - Cosmos-SDK의 scaffold 기능을 하는 cli

  - 체인 생성, 모듈 생성, 릴레이어 등 생성을 쉽게 한다.
