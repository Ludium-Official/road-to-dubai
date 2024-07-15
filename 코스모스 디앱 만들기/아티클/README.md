# Cosmos-SDK dApp 만들기

## Cosmos-SDK 기초

- Cosmos-SDK는 쉽게 PoS(Proof-of-Stake) 기반의 앱체인을 제작할 수 있는 툴킷으로, Cosmos-SDK를 사용해 제작된 체인들을 앱체인이라고 주로 말한다.

- Consensus 레이어는 Tendermint 또는 CombetBFT를 활용하고 개발자는 Application레이어만 신경쓰도록 설계된 SDK 이다.

```
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

- Cosmos-SDK는 기본 모듈을 제공함으로서 아래와 같은 기능을 기본으로 제공한다.(v0.50.x 기준)

  - x/auth : 계정과 관련된 기능
  - x/authz : 다른 계정으로 권한을 위임할 수 있다.(대신 Transaction을 전송 할 수 있다)
  - x/bank : 계정에 토큰 상태를 관리하고, 전송할 수 있다.
  - x/staking, Distribution : dPoS 관련된 관련된 기능을 한다.(위임, 클레임)
  - x/governance : 앱체인 자체적으로 거버넌스를 운영할 수 있는 기능을 제공한다.(제안, 투표, 결과를 통한 상태 변경)
  - x/mint : 인플레이션 관련 토큰을 민팅하는 기능
  - x/feegrant : 수수료 대납 관련
  - x/nft : ERC721과 같은 스펙의
  - 이외에도 체인을 운영하기 위한 Slashing, Genutil, Circuit, Upgrade, Consensus, Evidence, Crisis 등 모듈이 있다.

- 기본 모듈을 아니지만 아래 주요 모듈과 라이브러리들이 존재한다.

  - x/liquidstaking : 적용한 앱체인에 대해서 스테이킹한 토큰을 tokenize하여 활용 가능하다.
  - x/ethermint : EVM을 앱체인에서 사용할 수 있도록 지원하는 모듈(SmartContracts)
  - x/wasm : CosmWasm을 앱체인에서 사용할 수 있도록 지원하는 모듈(SmartContracts)
  - ibc-go : 앱체인간 토큰, 데이터 전송을 가능하게 하는 모듈
  - ibc-apps : packet forward middleware라는 ibc로 특정 데이터를 전송하고 수신한 체인에서 자동으로 처리되는 기능이 구현 가능하다.

- Cosmos-SDK에서 기본 모듈을 제외한 기능을 구현하려면

  - Custom Module을 제작해서 새로운 기능을 활용할 수 있다. 모듈은 체인 업데이트를 통해 보통 적용된다.

  - Smart Contract 관련 기능을 활용해 Cosmwasm 컨트랙트 또는 EVM 컨트랙트를 활용할 수 있다.

- Smart Contract?

  - 블록체인은 State Machine으로 볼 수 있는데, 어떤 상태들이 기록되 있는 것이다. 컨트랙트 코드도 그 중 일부이고, 컨트랙트의 상태도 저장되어 있다. 그 내용들을 EVM 또는 WASM 모듈을 통해 활용하는 것이다.

- Cosmos-SDK는 상태 값을 조회할 수 있도록 REST API / gRPC 방식 모두 제공한다.

- Cosmos-SDK에서 Amino, Direct, Text 방식으로 Message Sign을 제공한다.

## dApp 개발을 위한 기초 지식

- dApp은 Blockchain Network(앱체인)을 실제 유저가 활용하는 애플리케이션(웹 / 모바일 웹 / 모바일 앱)으로, 블록체인에 있는 상태 값을 조회 또는 저장하여 활용하는 어플리케이션으로 볼 수 있다. 모바일 지갑, 익스텐션 지갑, Explorer 등 체인의 정보를 활용하는 것을 모두 dApp으로 볼 수 있다.

- 앱 체인에서 돈을 전송하는 것을 설명하면

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

- CosmJS

  - Cosmos-SDK의 대표 typescript/javascript 라이브러리로 앱체인과 통신합니다.

  - CosmJS 라이브러리에서 하는 일

    - 니모닉/개인키를 활용해 Message를 Sign & Broadcast합니다.

    - Cosmostation, Keplr과 같은 지갑의 도움을 받아 사용자 서명을 요청합니다.(OfflineSigner)

    - 블록체인 네트워크에 State 정보를 Query 할 수 있습니다.

    - Cosmos SDK 모듈에서 발생하는 이벤트를 수신합니다.

- cosmos-kit

  - cosmos-kit은 Cosmos생태계의 지갑 Cosmostation, Keplr, Leap 등 다양한 지갑연결 및 CosmJS를 활용하여 Cosmos-SDK를 활용한 앱체인들과 쉽게 통신할 수 있다.

  - create-cosmos-app을 활용해서 다양한 예제를 제공하고 있다.

- Mintscan

  - 코스모스 생태계의 대표적인 Explorer

  - 메인넷, 테스트넷 등 다양한 체인을 지원하고 있으며 각 체인별 정보와 통계 뿐만 아니라 Cosmos 생태계 정보를 한번에 볼 수 있다.

  - 지갑이 내장되어 있어 x/bank, x/staking, x/governance 등 Cosmos 주요 기능을 바로 활용 가능하다.

- Ignite

  - Cosmos-SDK의 scaffold 기능을 하는 cli

  - 체인 생성, 모듈 생성, 릴레이어 등 생성을 쉽게 한다.

## 주요 생태계 dApp

- Astroport

  - Cosmos 생태계의 대표 DeFi 프로젝트

- Stargaze

  - Cosmos 생태계의 대표 NFT 프로젝트

- Osmosis

  - Cosmos 생태계의 대표 DeFi 프로젝트

- Stride

  - Cosmos 생태계의 주요 Liquid Staking 프로젝트
