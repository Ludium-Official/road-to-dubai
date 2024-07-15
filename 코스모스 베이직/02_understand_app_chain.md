# 02. Understand app chain architecture

### Preview

**Hello World!**, 안녕하세요! 코스모스 베이직 과정 두번째 챕터에 오신 걸 환영합니다!

두번째로 우리가 다룰 주제는 **'앱 체인 아키텍쳐 이해하기'** 입니다.

지난 시간에는 왜 app specific chain이라는 컨셉이 등장했는지 공감하고 이해하는 시간을 가졌습니다.

이어서 저희는 이번 과정에서 그 앱 체인(app specific chain)이 어떻게 구성되어있는지 Cosmos-SDK 기반으로 알아보겠습니다.

---

### What are application-specific blockchains

Cosmos-SDK!? 가 무엇인지 살펴보기 전에 저는 먼저 app specific chain이 어떤 느낌인지 다시 한번 살펴보고자 합니다.

제가 참고한 cosmos sdk official docs에서는 아래와 같이 설명되어있습니다만!

> Application-specific blockchains are blockchains customized to operate a single application. Instead of building a decentralized application on top of an underlying blockchain like Ethereum, developers build their own blockchain from the ground up. This means building a full-node client, a light-client, and all the necessary interfaces (CLI, REST, ...) to interact with the nodes.
> _(https://docs.cosmos.network/v0.50/learn/intro/why-app-specific)_

다소 헷갈릴 수 있게 표현되어있는 저 표현도 저희가 이미 지난 시간에 배웠던 예시와 그림들을 다시 본다면 좀 더 쉽게 이해하실 수 있습니다.

![01_app_based_blockchain](./assets/01_app_based_blockchain.png)

우린 지난시간에 도서관이란 시스템을 예로 앱 체인에 대해서 이야기 해봤었는데요. 이렇게 **도서관과 같이 하나의 어플리케이션만을 위한 블록체인이 바로 application-specific blockchain(app-specific chain, 이하 앱체인)입니다.**

공식문서에서는 single application. 이라고 표현되어있어서 그러면 하나의 프로그램만이 올라가있냐고 착각하실 수도 있는데 하나의 어플리케이션에는 여러 기능들이 존재할 수 있으니 하나의 서비스만을 위한 체인으로 생각하시면 됩니다.

계속해서 도서관이란 시스템을 예로 들어서 도서관에는 회원가입기능, 대출기능, 반납기능 등이 필요하다고 봅시다. (추가적인 기능들이 있겠지만은 생략)

그렇다면, '도서관 앱 체인'에는 다음과 같은 프로그램들이 올라갈 것입니다.

1. 회원가입 프로그램 (Registration Program)
2. 대출 프로그램 (Borrowing Program)
3. 반납 프로그램 (Returning Program)

그럼 위에서 보았던 application based blockchain 아키텍쳐를 예시로 든 '도서관 앱 체인'에 맞게 좀 더 구체적으로 표현해보겠습니다.

![02_library_app_blockchain](./assets/02_library_app_blockchain.png)

---

### Shortcut of application-specific blockchains

다시 한번 우리가 위에서 배운 내용을 짧게 요약한다면, 우리는 이제 이렇게 말할 수 있을 것입니다.

흔히 블록체인을 접하게 되면 가장 먼저 배우게 되는 VM based의 Ethereum(EVM)같이 general purpose를 위한 application blockchains이 아닌
(하나의 블록체인 위에 여러 어플리케이션을 올리려는 목적)

도서관 예시와 같이 이렇게 **하나의 어플리케이션(=서비스)를 위한 블록체인이 바로 application-specific blockchains이라고 이해하시면 되겠습니다.**

---

### Application-specific blockchains benefits

그럼 간단히 offical docs에 나와있는 앱 체인의 장점들에 대해서 알아봅시다.

앱체인에는 크게 4가지 장점(flexibility, performance, security, sovereignty)이 존재합니다.

저는 모든 장점을 자세히 다루지는 않을 것이고 간단히만 훑고 지나갈 것입니다. 당장의 교육과정과 개발과정엔 크게 중요하지 바로 와닿기도 힘들 것 같아서 입니다.

다만, 추후에 좀 더 자세히 앱체인의 특장점에 대해 알고 싶으신 분들은 이 [링크](https://docs.cosmos.network/v0.50/learn/intro/why-app-specific#application-specific-blockchains-benefits)를 통해서 원문을 읽어보시길 바랍니다.

##### Flexibility

cosmos-sdk based가 아닌 다른 앱체인은 다를 수 있지만은! 저희가 앞으로 배울 코스모스 생태계의 앱체인들은 기본적으로 cosmos-sdk로 만들어져있습니다.

아키텍쳐에 대해서는 바로 다음 섹션에서 다룰 것인데,

그걸 다루기 전에 간단히 말씀드리자면 앱 체인은 크게 1. 어플리케이션 영역과 2. 컨센서스 엔진 영역으로 나뉘는데 이 두 파트는 ABCI(Application BlockChain Interface)라고 불리는 인터페이스로 연결되어있습니다.

따라서, 이 인터페이스 표준만 맞춘다면 반드시 cosmos-sdk와 cometbft(전 tendermint)를 써야할 필요는 없습니다. 이를 flexibility라고 표현합니다.

> Application-specific blockchains give maximum flexibility to developers: In Cosmos blockchains, the state-machine is typically connected to the underlying consensus engine via an interface called the ABCI. This interface can be wrapped in any programming language, meaning developers can build their state-machine in the programming language of their choice.

이런 내용들에 대해 더 많이 궁금하시다면, 정말 유명하고 잘하는 paradigm에서 적은 [DAG based의 Narwhal&Bullshark with Cosmos-sdk 아티클](https://www.paradigm.xyz/2022/07/experiment-narwhal-bullshark-cosmos-stack)을 읽어보시는 것도 좋습니다!

##### Performance

퍼포먼스는 흔히들 말하는 TPS(transaction per seconds)를 뜻합니다. 얼마나 많은 트랜잭션들을 적은 시간 내에 효율적으로 처리할 수 있느냐입니다.

앱 체인은 당연히 general purpose체인들인 VM based 체인들과 달리 하나의 어플리케이션을 위해서 컴퓨팅 리소스를 소모하기 때문에 상대적으로 더 퍼포먼스가 좋습니다.

> In order to optimize the performance of the decentralized application, it needs to be constructed as a block chain specific to the application. An application-specific blockchain only operates a single application, so that the application does not compete with others for computation and storage.

##### Security

생략하겠습니다. 궁금하시면 위의 링크를 참고해주세요!

##### Sovereignty

대망의 Sovereignty입니다! 사실상 앱 체인의 가장 큰 장점이자 특징입니다.

일반적으로 general purpose 체인들은 여러 디앱이 하나의 체인 위에 올라가기 떄문에 특정 디앱만을 위한 온체인 레벨의 수정이나 제안을 하기 어려울 수 있습니다.

하지만, 앱 체인은 그 어플리케이션만을 위한 체인으로 디앱 파트인 어플리케이션의 UX를 개선하기 위해서 코어 레벨의 수정도 같이 제안하고 개선해나갈 수 있습니다!

이런 특징을 자기주권성과 같이 부릅니다. 이런 점들을 잘 살린 체인이 Injective랑 Dydx등이 있는데 나중에 같이 알아보면 좋을 것 같습니다.

> One of the major benefits of application-specific blockchains is sovereignty. The fundamental issue here is that the governance of the application and the governance of the network are not aligned. This issue is solved by application-specific blockchains. Because application-specific blockchains specialize to operate a single application, stakeholders of the application have full control over the entire chain. This ensures that the community will not be stuck if a bug is discovered, and that it has the freedom to choose how it is going to evolve.

##### Furthermore

그리고 공식문서에는 언급되지 않은 내용이지만, 간단히 Interchain Security에 대해서도 추가적으로 말씀드릴까 합니다.

위의 내용으로만 본다면 앱 체인은 나름의 방향성과 목적성을 가진 꽤나 괜찮은 블록체인의 방향성일 수 있습니다. 하지만, 위와 같이 여러 주권을 가진 각각의 앱 체인들은 체인마다의 주권을 가지고 있기 때문에 그로 인해 각자의 토큰의 TVL에 따른 시큐리티를 가지게 됩니다.

이로 인해 여러 앱 체인들 중 상대적으로 TVL이 낮은 체인의 경우 보안에 좀 더 취약해질 수 있습니다. PoS 특정상 체인의 시큐리티를 Cash로 지키자는 것이 기본적인 내포된 의미이기 때문이죠.

그래서 Cosmos에는 현재 Interchain Security과 같은 새로운 Soveringn app chain들의 시큐리티를 cosmos 와 같은 큰 TVL의 시큐리티를 상속받아서 운영하는 방식도 논의되고 생겨나고 있습니다. (Stride & Neutron)

---

### Cosmos-SDK based app chain architecture

저희 다룰 코스모스 앱 체인에 대한 아키텍쳐에 대해서 설명할 차례이지만 해당 내용은 다음 아티클로 미루도록 하겠습니다. 

이번 시간에 배웠던 위에서 배웠던 그림과 아래그림을 어떻게 매칭해야할지 많이 고민하는 것으로 시간을 마치려고 합니다. 

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
              |     Tendermint      |
              |                     |
              |                     |
              +---------------------+
```

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

<!-- ![02_app_chain_architecture](./assets/02_app_chain_architecture.png) -->

ref; https://docs.cosmos.network/v0.50/learn/intro/why-app-specific