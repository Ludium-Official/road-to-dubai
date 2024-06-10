# Cosmos-SDK dApp 만들기

## 기초

- dApp은 Blockchain Network에서 실행되는 애플리케이션 입니다. 쉽게 블록체인에 상태 값을 저장하고 활용하는 어플리케이션으로 볼 수 있습니다.

- Cosmos-SDK는 기본 모듈을 제공함으로서 아래와 같은 기능을 기본으로 제공합니다.(v0.50.x 기준)

  - Authz, Bank, Staking, NFT, Mint, Governance, Auth, Protocolpool, Params, Slashing, Genutil, Circuit, Upgrade, Consensus, Evidence, Distribution, Crisis, Feegrant
  - 위 부분에서 dApp을 만드는데 특히 필요한 Authz, Bank, Staking, NFT, Mint, Governance를 살펴본다.

- Cosmos-SDK에서 지원하는 사인 방식인 amino, direct, text 방식에 대해 알아봅니다.

- 각 단계별 Create Message, Sign, Broadcast에 어떻게 동작하는지 확인합니다.

## dApp 개발을 위한 기초 지식

- dApp 개발이란 네트워크와 데이터를 넣거나, 네트워크에 데이터를 조회하는 앱을 말한다.
- 사인을 위해선 지갑이 필요하다.
  - cosmos 기반의 주요 지갑은 Cosmostation(Mobile, Extension), Keplr(Mobile, Extension), Leap(Mobile, Extension)
- 체인 정보 조회
  - smart contract 정보 조회 및 호출
  - rest, grpc 방식으로 조회
- 결과 확인을 위해 Explorer를 활용한다.
  - Mintscan 또는 각 체인에서 제공하는 Explorer

## Cosmos 생태계에서 지원을 하는 dApp 개발을 위한 툴 킷

- CosmJS

  - Cosmos-sdk의 대표 typescript/javascript 라이브러리로 Cosmos 체인과 통신합니다.

  - CosmJS 라이브러리에서 하는 일

    - 니모닉/개인키를 활용해 Message를 Sign & Broadcast합니다.
    - Cosmostation, Keplr과 같은 지갑의 도움을 받아 사용자 서명을 요청합니다.(OfflineSigner)
    - 블록체인 네트워크에 State 정보를 Query 할 수 있습니다.
    - Cosmos SDK 모듈에서 발생하는 이벤트를 수신합니다.

- Cosmos-Kit

  - cosmos kit은 cosmos생태계의 지갑 cosmostation, keplr, leap 등 다양한 지갑연결 및 cosmos-sdk의 모듈들을 쉽게 호출할 수 있는 라이브러리
  - CosmJS를 여러 지갑들을 활용해서 쉽게 이용할 수 있게 만든 라이브러리다.
