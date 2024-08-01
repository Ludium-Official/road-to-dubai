# 20. Moudle Basic
> cosmos-sdk v0.47부터 Tendermint에서 [CometBFT로 마이그레이션](https://github.com/cosmos/cosmos-sdk/issues/14870) 했기 때문에, 아티클은 cosmos-sdk v0.47, cometbft v0.38 기반으로 작성되었다. 

## 목차
0. Module 탄생 배경
1. Module 개념 
2. Module 종류

## 0. Module 탄생 배경 
[step1에서도 다룬 일부 배경](./01_empathize_with_app_specific_chain.md)을 기반으로 설명을 이어나가보면, 이더리움의 탄생은 블록체인 기술의 새로운 지평을 열었다. 이더리움의 핵심은 튜링 완전한 EVM(Ethereum Virtual Machine) 설계를 통해 탈중앙화된 블록체인 네트워크 위에 프로그램이 동작할 수 있도록 한 것이다. 이러한 프로그램은 네트워크를 통해 합의되고 검증되어 등록되며, 우리는 이를 스마트 컨트랙트라고 부른다. 스마트 컨트랙트는 탈중앙화의 특성에 맞게 누구나 접근 가능하며 프로그램 코드를 실행할 수 있다는 특징을 가지고 있다. 이러한 특징은 접근성은 블록체인의 투명성과 신뢰성을 높여준다. 그러나 이로 인해 몇 가지 문제점도 존재한다:
- 첫 번째로, 네트워크의 확장성 문제를 들 수 있다. 이더리움 네트워크는 많은 사람들이 참여하면서 단일 스레드로 동작하는 구조 때문에 트랜잭션 처리 속도가 느려졌다. 이는 네트워크가 감당해야 할 부담이 커졌음을 의미한다.
- 두 번째 문제는 유사한 기능을 가진 프로그램들의 반복적인 코드 업로드이다. 이미 존재하는 스마트 컨트랙트를 사용하면 스토리지 권한을 공유하거나 넘겨줘야 한다는 문제가 있어, 개발자들이 동일한 기능을 하는 새로운 스마트 컨트랙트를 반복해서 작성하는 상황이 발생하였다.

그럼에도 불구하고 많은 개발자와 사용자가 이더리움 네트워크에 몰리는 이유는 바로 생태계의 활성화 때문이다. 다양한 앱이 활성화된 네트워크에서 서로 간의 상호작용을 통해 가능한 일이 더 많기 때문이다. 예를 들면 다음과 같다:
- DEX(Decentralized Exchange) 컨트랙트는 Bank 컨트랙트의 돈 입출금 기능을 사용할 수 있다.
- Bank 컨트랙트는 Token 컨트랙트에 요청하여 돈을 전송하도록 할 수 있다.

앱 체인이라고 불리우는 Cosmos SDK는 이러한 문제점을 탈피하고자 하나의 애플리케이션이 하나의 블록체인 위에서 동작하도록 설계되었다. 코스모스 생태계에서는 텐더민트 합의 엔진 덕분에 여러 체인들이 상호 간의 통신이 가능하도록 만들었고, 이는 최종적으로 성능 향상 및 최적화를 가능하게 하였다. 그리고 블록체인에서 기본적으로 필요한 기능들이 구현된 다양한 Module들을 제공하여 체인을 만들 때 바퀴를 재발명하지 않아도 되며, 스토리지 또한 안전하게 관리할 수 있도록 하였다. 이러한 기능들은 다음과 같은 장점을 가지고 있다:
- [object-capabilities](./13_store_and_keepers.md#object-capabilities-모델) 기능을 통해 각 모듈의 스토리지를 안전하게 관리하도록 하여 알지 못하는 모듈이 등장하여도 이로부터 안전을 지킬 수 있다.
- 기존에 제공하는 모듈을 사용하면 auth, bank와 같은 기능들을 스마트 컨트랙트 개발 필요 없이 쉽게 빌드할 수 있다.
- 또한 간단한 문법을 제공하는 Golang으로 만들 수 있다는 장점도 있다.

그렇다고 Cosmos SDK에서 컨트랙트 코드를 빌드할 수 없는 것은 아니다. 이미 스마트 컨트랙트 디앱을 빌드하는 생태계는 거대해졌기 때문에, 이에 익숙한 빌더들을 위해서 rust를 사용하는 [cosmwasm](https://cosmwasm.com/)이나 evm 호환을 제공하는 [ethermint](https://docs.ethermint.zone/)가 존재한다. 이처럼 Cosmos SDK는 이더리움의 한계를 보완하면서도 강력한 블록체인 애플리케이션 생태계를 구축하는 데 중점을 두고 있다. 이를 통해 개발자들은 더 효율적이고 안전한 환경에서 다양한 블록체인 애플리케이션을 개발할 수 있게 되었다.

## 1. Module 개념
합의 엔진(CometBFT)에서 트랜잭션이 릴레이되면 `baseapp`은 트랜잭션에 포함된 메시지를 디코딩하여 이를 적절한 모듈로 메시지를 라우팅한다. 적절한 모듈 메시지 핸들러가 이를 수신하면 상태에 관련된 기능이 실행된다. 코어에서 인프라 관련된 기능을 처리하고 모듈에서 애플리케이션 비즈니스 로직을 구현하는 것으로 볼 수 있다. 이러한 모듈의 핵심 개념은 다음과 같다: 
- 모듈은 더 큰 상태 머신(`baseapp`) 내의 상태 머신이다. 
- 모듈의 핵심 레이아웃은 상태, 상태 조회 및 상태 전환이다. 

모듈은 `KVStore` 저장소를 통해 해당 모듈의 상태를 정의한다. [저장소 관리](./13_store_and_keepers.md#cosmos-sdk의-store-관리)는 `baseapp`에서 초기화할 때 storeKey를 keeper에게 제공하여 저장소에 접근하여 상태를 읽고 쓸 수 있는 권한을 관리한다. 그래서 새로운 기능을 만드려면 Protobuf와 같은 메시지 타입과 해당 상태를 관리하는 로직을 설계해주면 된다. 

이러한 기능을 구현하기 위해서 모듈에는 다음과 같은 기능들을 포함하고 있다:
- 노드와의 상호작용을 하는 [서버 및 인터페이스](./16_grpc_and_rest_and_cometbft_rpc.md)
- [Multistore](./13_store_and_keepers.md#3-multistore)로 불리는 모듈 상태를 관리하는 저장소
- CometBFT와 통신하는 [ABCI](./99c4_cometbft_and_abci_2.md) 구현체 

모듈은 이미 존재하는 다른 모듈과의 상호 작용도 정의한다. Cosmos SDK 앱을 빌드하는 개발자의 대부분의 작업은 애플리케이션에 필요한 커스텀 모듈을 구축하고, 이를 이미 존재하는 모듈과 통합하여 하나의 일관된 애플리케이션으로 구성하는 것이다. 기존 모듈은 Cosmos SDK 자체에서 제공하거나 이미 빌드된 다른 앱 모듈을 사용할 수 있다. 

## 2. Module 종류 
다음은 Cosmos SDK에서 기본으로 제공하는 모듈이다:
- [Auth](./21_module_auth.md): Cosmos SDK 계정 및 트랜잭션 인증(Authentication) 기능 
   - Vesting: Vesting 계정 구현 (v0.51.0 deprecated)
- [Bank](./22_module_bank.md): 토큰 전송 기능
- [Feegrant](./23_module_feegrant.md): 트랜잭션 실행에 대한 수수료 허용량 부여
- [Authz](./24_module_authz.md):  다른 계정을 대신하여 작업을 수행할 수 있는 계정에 대한 권한 부여 기능 
- [Governance](./25_module_gov.md): 온체인 제안 및 투표
- [Staking](./26_module_staking.md): 퍼블릭 블록체인을 위한 Proof-of-Stake 레이어
- [Slashing](./27_module_slashing.md): 검증자 처벌 메커니즘 (PoS)
- [Mint](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/mint/README.md): 새로운 단위의 스테이킹 토큰 생성
- [Params](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/params/README.md): 전 세계에서 사용 가능한 매개변수 저장소
- [Capability](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/capability/README.md): [object-capabilities](./13_store_and_keepers.md#object-capabilities-모델) 구현
- [Crisis](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/crisis/README.md): 위기 상황 특정 상황에서 블록체인을 중단하는 기능 
- [Distribution](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/distribution/README.md): 수수료 분배 및 스테이킹 토큰 제공 분배
- [Evidence](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/evidence/README.md): 이중 서명, 잘못된 행동 등에 대한 증거 처리
- [Upgrade](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/upgrade/README.md): 소프트웨어 업그레이드 관련 기능 
- [NFT](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/nft/README.md): [ADR43](https://docs.cosmos.network/main/architecture/adr-043-nft-module.html)을 기반으로 구현된 NFT 기능
- [Consensus](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/consensus/README.md): Tendermint의 ABCI 합의 매개변수를 수정하기 위한 모듈


## Resource
- https://docs.cosmos.network/main/build/building-modules/intro
- https://github.com/cosmos/cosmos-sdk/tree/v0.47.0/x
- 윤주운, [코스모스 아카데미] 코스모스 SDK에 대하여, Youtube, uploaded by Lunamint, 2018. 10. 12, https://youtu.be/ZD7xk1SfdBw
