# 99c4. CometBFT and ABCI 2.0
## 목차
0. CometBFT
1. ABCI 2.0
   1. PrepareProposal 메서드 
   2. ProcessProposal 메서드 
   3. ExtendVote 메서드
   4. VerifyVoteExtension 메서드 
   5. FinalizeBlock 메서드

## 0. CometBFT
CometBFT는 [99c3의 텐더민트 코어](./99c3_tendermint_and_abci.md)를 포크한 프로젝트이다. 그래서 기본적인 기반은 지금까지 알아본 텐더민트 코어와 같다고 봐도 무방하기 때문에, 개념을 이해할 때 CometBFT는 텐더민트 코어와 동일시해도 된다. 
```
              +---------------------+
              |                     |
              |     Application     |
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



> 실제 Cosmos SDK에 [텐더민트 코어에서 CometBFT로 이주](https://github.com/cosmos/cosmos-sdk/issues/14870)된 것은 v0.47 버전부터이다.

## 1. ABCI 2.0
CometBFT에서 주목할 변경점은 ABCI(Application Blockchain Interface)이다. ABCI 2.0(또는 ABCI++)은 Cosmos SDK에서 애플리케이션 개발자에게 더 많은 유연성과 제어를 제공하기 위해 설계되었다. 다음은 이에 대한 ADR 문서 내용들이다: 
- [ADR 60: ABCI 1.0 Integration (Phase I)](https://docs.cosmos.network/v0.50/build/architecture/adr-060-abci-1.0)
- [ADR 64: ABCI 2.0 Integration (Phase II)](https://docs.cosmos.network/v0.50/build/architecture/adr-064-abci-2.0)

ABCI 2.0(또는 ABCI++)는 ABCI의 진화 버전으로, CometBFT와 애플리케이션 간의 상호작용을 위한 인터페이스이다. [99c3_tendermint_core_and_abci](./99c3_tendermint_core_and_abci.md)에서 살펴본 코드베이스를 [cometBFT의 abci/types/application.go](https://github.com/cometbft/cometbft/blob/v0.38.x/abci/types/application.go)의 파일을 보면 대부분 일치하는 것을 볼 수 있지만, 텐더민트 코어에서는 보지 못하였던 메서드가 ABCI 2.0으로 추가된 것을 확인할 수 있다: 
```go
PrepareProposal(ctx context.Context, req *PrepareProposalRequest) (*PrepareProposalResponse, error)

ProcessProposal(ctx context.Context, req *ProcessProposalRequest) (*ProcessProposalResponse, error)

FinalizeBlock(ctx context.Context, req *FinalizeBlockRequest) (*FinalizeBlockResponse, error)

ExtendVote(ctx context.Context, req *ExtendVoteRequest) (*ExtendVoteResponse, error)

VerifyVoteExtension(ctx context.Context, req *VerifyVoteExtensionRequest) (*VerifyVoteExtensionResponse, error)
```

### 1. PrepareProposal 메서드 
`PrepareProposal` ABCI 메서드는 블록 제안자가 다음 블록에 포함할 트랜잭션을 평가하도록 애플리케이션에 요청하는 역할을 한다. 
- 이 메서드가 도입되기 전에는 `CheckTx`가 트랜잭션의 유효성을 평가했으며, 이는 여전히 유효하다. 하지만, 이제 `CheckTx`는 유효한 트랜잭션을 애플리케이션의 mempool 데이터 구조에 추가하는 역할도 맡는다. 
- `PrepareProposal`은 단순 유효성을 평가하는 로직만 수행하므로, 이를 통해 애플리케이션 개발자는 애플리케이션이 자체 mempool을 정의하고 제어하여 정교한 트랜잭션 우선순위 및 필터링 메커니즘을 구현할 수 있는 더 큰 유연성을 가지게 된다. 

### 2. ProcessProposal 메서드 
`ProcessProposal` ABCI 메서드는 `PrepareProposal` 단계에서 선택된 트랜잭션을 포함하는 제안된 블록의 유효성을 보장하는 역할을 한다. 
- 애플리케이션마다 제안된 블록의 유효성을 결정하는 방법은 다를 수 있다. 
- 대부분의 애플리케이션에서는 `AnteHandler` 체인을 호출하는 것으로 충분하지만, 일부 애플리케이션은 특정 트랜잭션이 특정 순서로 포함되도록 하거나 특정 트랜잭션이 반드시 포함되도록 하는 등 추가적인 제어가 필요할 수 있다.

Cosmos SDK는 모든 트랜잭션을 `CheckTx` 흐름, 즉 `AnteHandler`를 사용하여 검증하고, 모든 트랜잭션이 디코딩되지 않는 한 항상 ACCEPT를 반환하는 기본 `ProcessProposal` 구현을 제공한다. 이는 대부분의 애플리케이션에 적합하지만, 추가적인 유효성 검사가 필요한 경우 애플리케이션 개발자가 자체 검증 로직을 구현하여 애플리케이션이 블록의 유효성을 결정하고 처리하는 방법을 더 세밀하게 제어할 수 있도록 한다.


### 3. ExtendVote 메서드
`ExtendVote` ABCI 메서드는 각 검증자가 CometBFT 합의 과정의 pre-commit 단계를 확장할 수 있도록 한다. 구체적으로, 애플리케이션이 사용자 정의 비즈니스 로직을 수행하여 pre-commit 투표에 추가 데이터를 제공할 수 있게 한다. 이 데이터는 "투표 확장(vote extension)"이라고 하며, 투표와 함께 전송되고 다음 높이에서 애플리케이션에서 사용할 수 있게 된다.


동작 방식은 다음과 같다:
1. 데이터 전송: 투표 확장은 투표와 함께 브로드캐스트되고 수신된다.
2. 데이터 수신: 다음 블록의 제안자는 `RequestPrepareProposal.local_last_commit.votes`를 통해 투표 확장을 받는다.
3. 데이터 형식: 애플리케이션이 제공할 투표 확장 정보가 없는 경우, 0바이트 배열을 반환한다.

각 검증자가 자체 투표 확장을 제출하지만, 다음 블록의 제안자만 모든 투표 확장을 받을 수 있다. 모든 투표 확장이 포함되는 것은 아니다. 검증자는 2/3 이상 pre-commit 투표만 기다리면 되기 때문이다.


### 4. VerifyVoteExtension 메서드 
`VerifyVoteExtension` 메서드는 검증자가 수신한 각 pre-commit 메시지에 첨부된 "투표 확장(vote extension)" 데이터를 검증할 수 있게 한다. 유효성 검증에 실패하면 해당 사전 커밋 메시지는 무효로 간주되어 CometBFT에 의해 무시된다. 

CometBFT는 pre-commit 투표를 검증할 때 `VerifyVoteExtension`을 사용한다. 구체적으로, pre-commit 메시지에 대해 CometBFT는 다음과 같이 처리한다:
1. 서명된 투표와 서명된 투표 확장이 포함되지 않은 메시지는 거부한다.
2. 서명된 투표와 서명된 투표 확장의 검증이 실패하면 메시지를 거부한다.
3. 애플리케이션에서 `VerifyVoteExtension`을 거부한 경우 메시지를 거부한다.
4. 그렇지 않은 경우, CometBFT는 pre-commit 메시지를 수락한다.

올바른 검증자가 반복적으로 투표 확장을 검증하지 못하면, 충분한 수의 (+2/3) 검증자가 해당 블록에 대한 pre-commit 투표를 보내더라도 CometBFT는 블록을 확정하지 못할 수 있다. 따라서 `VerifyVoteExtension`은 신중하게 사용해야 한다. Liveness를 위해서 CometBFT는 애플리케이션이 잘못된 투표 확장을 감지한 경우, 이를 `ResponseVerifyVoteExtension`에서 수락하고 실제 애플리케이션 로직 내에서 무시할 것을 권장하고 있다. 


### 5. FinalizeBlock 메서드
`FinalizeBlock` ABCI 메서드는 결정된 블록을 애플리케이션에 전달하여, 블록 내의 트랜잭션을 결정적으로 실행하고 상태를 업데이트하도록 한다. 블록과 트랜잭션 결과에 대한 커밋은 `ResponseFinalizeBlock`의 매개변수를 통해 반환되며, 다음 블록의 헤더에 포함된다. CometBFT는 새로운 블록이 결정될 때 이를 호출한다. 

`FinalizeBlock`은 현재 ABCI 실행 흐름인 `BeginBlock`, 하나 이상의 `DeliverTx`, `EndBlock`을 단일 ABCI 메서드로 캡슐화하여 사용한다. 이를 통해 일관성을 유지하면서도 실행 흐름을 간소화할 수 있게 된다. 


## Resources
- https://docs.cometbft.com/main/spec/abci/
- https://docs.cosmos.network/v0.50/build/architecture/adr-060-abci-1.0
- https://docs.cosmos.network/v0.50/build/architecture/adr-064-abci-2.0