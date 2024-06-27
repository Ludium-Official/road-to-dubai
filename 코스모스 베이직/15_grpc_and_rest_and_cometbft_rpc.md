# 15. gRPC, REST and CometBFTRPC
> cosmos-sdk v0.47부터 Tendermint에서 [CometBFT로 마이그레이션](https://github.com/cosmos/cosmos-sdk/issues/14870) 했기 때문에, 아티클은 cosmos-sdk v0.47, cometbft v0.38 기반으로 작성되었다. 

## 목차 
0. Cosmos SDK의 IPC
1. gRPC 서버 
2. REST 서버 
3. ComettBFTRPC

## 0. Cosmos SDK의 IPC
블록체인 노드에서 노드 간의 통신은 클라이언트-서버 아키텍처의 [REST 및 RPC와 같은 IPC 메커니즘](./14_rpc_basic.md#0-ipcinter-process-communication)을 통해 유지된다. 각 노드는 독립적인 프로세스이지만 상호 간의 효율적인 프로세스 간 통신을 통해 블록체인 네트워크 일관성과 지속적인 상태 동기화를 유지하여 블록체인을 분산 환경에서 신뢰할 수 있는 시스템으로 만든다.

Cosmos SDK 각 노드는 사용자가 노드와 상호 작용할 수 있도록 다음 Endpoint을 공개하고 있다. 
- gRPC 서버(기본 포트: `9090`)
- REST 서버(기본 포트: `1317`)
- Cometbft RPC 엔드포인트(기본 포트: `26657`)

## 1. gRPC 서버 
Cosmos SDK에서 Protobuf는 기본 인코딩 라이브러리로, Protobuf를 기반으로 하는 gRPC와 같은 도구를 사용할 수 있게 해준다. 각 모듈은 상태 쿼리를 정의하는 `gRPC service`(쿼리 서비스)를 사용한다. 트랜잭션 브로드캐스팅에 사용되는 쿼리 서비스와 트랜잭션 서비스는 다음 함수를 통해 [gRPC 서버](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/server/types/app.go#L46-L48)에 연결된다:
```go
// RegisterGRPCServer registers gRPC services directly with the gRPC
// server.
RegisterGRPCServer(grpc.Server)
```

> 트랜잭션을 gRPC를 사용하여 브로드캐스트하는 방법은 [다음 10.Transaction and Mempool#2-1.트랜잭션 생성 및 전송](./10_transaction_and_mempool.md#2-1-트랜잭션-생성-및-전송)을 참고하자.


`grpc.Server`는 모든 gRPC 쿼리 요청과 트랜잭션 브로드캐스트 요청을 처리하는 gRPC 서버이다. 이 서버는 `~/.simapp/config/app.toml`에서 설정할 수 있다:
- `grpc.enable = true|false` 필드는 gRPC 서버를 활성화할지 여부를 정의한다. 기본값은 `true`이다.
- `grpc.address = {string}` 필드는 서버가 바인딩할 ip:port를 정의한다. 기본값은 `localhost:9090`이다.
> `~/.simapp`은 기본적인 Cosmos SDK 노드의 설정 및 데이터베이스가 저장되는 디렉터리이다. 기본적으로 ~/.{app_name}으로 설정되어 있다.

## 2. REST 서버
여러 가지 이유로 gRPC를 사용할 수 없는 경우가 존재한다. (예: 웹 애플리케이션을 구축 중이고 브라우저가 gRPC가 구축된 HTTP2를 지원하지 않는 경우), Cosmos SDK는 [gRPC gateway](./14_rpc_basic.md#3-3-grpc-gateway)를 통해 REST 경로를 제공한다. 모든 경로는 `~/.simapp/config/app.toml`에서 설정할 수 있다:
- `api.enable = true|false` 필드는 REST 서버를 활성화할지 여부를 정의한다. 기본값은 false이다.
- `api.address = {string}` 필드는 서버가 바인딩할 ip:port를 정의한다. 기본값은 `tcp://localhost:1317`이다.

일부 추가 API 구성 옵션은 `~/.simapp/config/app.toml`에 주석과 함께 정의되어 있으므로 해당 파일을 직접 참조하면 된다.

### gRPC-gateway 사용 예시
Protobuf 쿼리 서비스에 정의된 각 gRPC 엔드포인트에 대해 Cosmos SDK는 이에 상응하는 REST를 제공한다. 예를 들어, 잔액 쿼리는 `/cosmos.bank.v1beta1.QueryAllBalances` gRPC 엔드포인트를 통해 수행하거나 `/cosmos/bank/v1beta1/balances/{address}` gRPC 게이트웨이를 통해 수행할 수 있다. 두 REST Endpoint는 동일한 결과를 반환한다. Protobuf 쿼리 서비스에 정의된 각 RPC 메서드에 대해 해당 REST 엔드포인트가 옵션으로 정의된다. 다음은 예시로 든 [잔액 RPC 메서드](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/bank/v1beta1/query.proto#L23-L30)의 옵션 설정 부분이다:
```proto
// AllBalances queries the balance of all coins for a single account.
//
// When called from another module, this query might consume a high amount of
// gas if the pagination field is incorrectly set.
rpc AllBalances(QueryAllBalancesRequest) returns (QueryAllBalancesResponse) {
  option (cosmos.query.v1.module_query_safe) = true;
  option (google.api.http).get               = "/cosmos/bank/v1beta1/balances/{address}";
}
```

앱 개발자의 경우 gRPC-gateway로 설정한 REST 경로를 REST 서버에 연결해야 하며, 이 작업은 ModuleManager에서 [RegisterGRPCGatewayRoutes 함수](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/module/module.go#L56)를 호출하여 수행한다.

## 3. ComettBFTRPC
Cosmos SDK와는 별개로 합의 엔진 CometBFT의 RPC 서버도 공개되어있다. 이 RPC 서버는 `~/.simapp/config/config.toml`의 rpc 테이블 아래에서 매개변수를 조정하여 구성할 수 있으며, 기본 수신 주소는 `tcp://localhost:26657`이다. 
- [CometBFT RPC Endpoint Docs](https://docs.cometbft.com/v0.37/rpc/)

CometBFT RPC Endpoint는 Cosmos SDK와 대체로 연관이 있다: 
- [`/abci_query`](https://docs.cometbft.com/v0.37/rpc/#/ABCI/abci_query): Cosmos SDK 앱 상태를 쿼리한다. `path` 매개변수로 다음 문자열을 보낼 수 있다:
    - 코스모스 bank의 정식 서비스 메서드(예: `/cosmos.bank.v1beta1.Query/AllBalances`). 그런 다음 데이터 필드에는 Protobuf를 사용하여 바이트로 인코딩된 메서드의 요청 매개변수가 포함되어야 한다.
    - `/app/simulate`: 트랜잭션을 시뮬레이션하고 사용된 가스 등 일부 정보를 반환한다.
    - `/app/version`: 애플리케이션의 버전을 반환한다.
    - `/store/{storeName}/key`: 데이터 파라미터에 표시된 키와 관련된 데이터를 명명된 스토어에 직접 쿼리한다.
    - `/store/{storeName}/subspace`: 데이터 매개변수의 값이 접두사로 포함된 키/값 쌍에 대해 네임드 스토어를 직접 쿼리한다.
    - `/p2p/filter/addr/{port}`: 주소 포트별로 노드의 P2P 피어를 필터링한 목록을 반환한다.
    - `/p2p/filter/id/{id}`: 아이디별로 필터링된 노드의 P2P 피어 목록을 반환한다.
- `/broadcast_tx_`{[sync](https://docs.cometbft.com/v0.37/rpc/#/Tx/broadcast_tx_sync), [async](https://docs.cometbft.com/v0.37/rpc/#/Tx/broadcast_tx_async), [commit](https://docs.cometbft.com/v0.37/rpc/#/Tx/broadcast_tx_commit)}: 이 3개의 엔드포인트는 다른 피어에게 트랜잭션을 브로드캐스트한다. 트랜잭션 브로드캐스팅하는 방법으로 알아본 [CLI, gRPC 및 REST](./10_transaction_and_mempool.md#트랜잭션-브로드캐스팅하기) 모두 내부적으로 이 3개의 CometBFT RPC를 사용하고 있다.


# Resources
- https://docs.cosmos.network/v0.47/learn/advanced/grpc_rest
- https://github.com/cosmos/cosmos-sdk/blob/main/docs/learn/advanced/06-grpc_rest.md