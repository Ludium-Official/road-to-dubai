# 10. Trnasaction and Mempool
> cosmos-sdk v0.47부터 Tendermint에서 [CometBFT로 마이그레이션](https://github.com/cosmos/cosmos-sdk/issues/14870) 했기 때문에, 아티클은 cosmos-sdk v0.47, cometbft v0.38 기반으로 작성되었다. 
## 목차
0. 트랜잭션
1. Mempool
2. 트랜잭션 라이프 사이클 
	1. 트랜잭션 생성
	2. 트랜잭션 Broadcasting
	2. Mempool에 추가하기 
	3. Block에 포함하기 
	4. 상태 변경하기 

## 0. 트랜잭션
블록체인 핵심은 복제된 (결정론적) 상태 머신이라는 것에 있다. 글로벌 참여자들에 의해 운영되는 분산 시스템은 모두가 동일한 상태를 지녀 마치 하나의 시스템처럼 동작해야만 한다. 이러한 상태 전환을 트리거하는 것이 트랜잭션이다. 트랜잭션 내부에는 블록체인 네트워크의 상태를 변경하는 요청이 포함되어 있다. 트랜잭션은 사용자가 발생시키며, 이는 블록체인 네트워크를 통해 전파하여 합의를 이루고나면 검증된 블록에 포함된다. 이러한 상태 머신을 정의하기만 하면 합의 엔진 레이어에 있는 CometBFT가 네트워크를 통해 복제를 처리한다.

```
+--------+                              +--------+
|        |                              |        |
|   S    +----------------------------> |   S'   |
|        |   For each T in B: apply(T)  |        |
+--------+                              +--------+
```

### 트랜잭션 Interfcae
Cosmos-SDK에서 사용하는 트랜잭션 인터페이스는 다음과 같다:
[v0.47.0/tx_msg.go#L39-L46](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/tx_msg.go#L42-L50)
- [`GetMsgs`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/tx_msg.go#L45): 트랜잭션의 wrapping을 해제하고 포함된 목록을 반환한다.
- [`sdk.Msg`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/tx_msg.go#L14-L26): 하나의 트랜잭션에 하나 또는 여러 개의 메시지가 있을 수 있다.
- Tx.[`ValidateBasic`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/tx_msg.go#L49): 트랜잭션이 유효하지 않은지 확인하기 위해 ABCI 메시지의 `CheckTx` 및 `DeliverTx`에서 사용하는 lightweigt, stateless 검사가 포함되어 있다. 예를 들어, [`ValidateBasic`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/tx/types.go#L36) 함수는 거래가 올바른 서명자 수에 의해 서명되었는지, 수수료가 사용자의 최대 금액을 초과하지 않는지 확인한다.
- Msg.[`ValidateBasic`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/tx_msg.go#L20): 메시지에 대한 기본 유효성 검사만 수행하는 sdk.Msg용 [`ValidateBasic`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L526-L540) 함수이다. 
- Msg.[`GetSigners`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/tx_msg.go#L25): 트랜잭션의 모든 메시지는 `GetSigners`에 지정된 주소로 서명해야 한다. 


여기서 헷갈리면 안되는 점은 각 Tx와 Msg에 정의된 ValidateBasic 함수는 이름만 같고 기능은 다르다는 것이다. 동작 예시는 다음과 같다:
1. [`runTx`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L618) 에서 auth 모듈에서 생성된 트랜잭션을 확인할 때 먼저 각 메시지에 대해 `Msg.ValidateBasic`을 실행한다. 
2. 그런 다음 auth 모듈의 [`AnteHandler`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/handler.go#L8)를 실행하여 트랜잭션 자체에 대해 `Tx.ValidateBasic`을 호출한다.
```go
Tx interface {
	GetMsgs() []Msg
	ValidateBasic() error
}

Msg interface {
	proto.Message
	ValidateBasic() error
	GetSigners() []AccAddress
}
```
> 더 자세한 동작은 아래 `2. 트랜잭션 라이프 사이클`에서 알아본다.

개발자가 직접 Tx 객체를 커스텀하는 경우는 거의 없다. 이는 트랜잭션 생성에 사용되는 내부 과정의 일부로 보면 된다. 개발자는 일반적으로 [`TxBuilder`](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/client/tx_config.go#L36-L46) 인터페이스를 사용한다.

### Message 
> 트랜잭션 내부 메시지를 CometBFT 합의 계층과 앱 계층 간의 상호작용을 정의하는 ABCI 메시지와 혼동해서는 안 된다.

트랜잭션 내부에 담긴 메시지는 자신이 속한 모듈의 범위 내에서 상태 전환을 트리거하는 요소라고 보면 된다. 이 설계는 모듈 개발자에게 더 많은 책임을 부여한다: 
1. 모듈 개발자는 Protobuf Msg 서비스에 메서드를 추가하고 `MsgServer`를 정의하여 모듈 메시지를 정의한다. 
2. 각 [sdk.Msg](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx_msg.go#L11-L22)는 각 모듈의 `tx.proto` 파일에 정의된 정확히 하나의 Protobuf Msg 서비스 RPC와 연관된다. 
3. Cosmos SDK 앱 라우터는 모든 `sdk.Msg`를 해당 RPC 서비스에 자동으로 매핑하여 적절한 메소드로 라우팅한다. 
4. Protobuf는 각 모듈의 Msg 서비스에 대한 `MsgServer` 인터페이스를 생성하고 모듈 개발자는 이 인터페이스를 구현한다.

이렇게 하면 애플리케이션 개발자는 상태 전환 로직을 반복적으로 구현할 필요 없이 모듈을 가져다 쉽게 사용할 수 있게 된다. 메시지에는 상태 전환 로직에 대한 정보가 포함되어 있지만, 트랜잭션의 다른 메타데이터와 관련 정보는 [`TxBuilder`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/client/tx_config.go#L33-L50)와 컨텍스트에 저장된다.


## 1. Mempool
트랜잭션은 In-memory Cache로 독립적으로 관리되며, 비트코인 이후 Mempool로 알려지게 되었다. [`Mempool`](https://github.com/cometbft/cometbft/blob/v0.37.0/mempool/mempool.go#L32)의 주요 역할은 다음과 같다:
- 트랜잭션이 수신되면 각 모듈 [`AnteHandler`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/handler.go#L8)에 의해 유효성이 검사되고, 유효한 경우 Mempool에 추가된다. 
- 유효한 트랜잭션은 피어 노드에게 브로드캐스팅한다. [`p2p.switch.BroadcastEnvelope`](https://github.com/cometbft/cometbft/blob/v0.37.0/p2p/switch.go#L265-L294) 함수를 보면 고루틴을 사용하여 피어(검증자 집합)들에게 비동기로 전송한다. 
- 제안자가 새로운 블록에 담을 트랜잭션을 Mempool에서 가져온다.
- 블록이 commit되고 나면 Mempool에 저장된 트랜잭션를 삭제하고 유효성을 재검증한다.
 
v0.47부터 Tendermint에서 [CometBFT로 마이그레이션](https://github.com/cosmos/cosmos-sdk/issues/14870) 하면서 애플리케이션에 자체 [`Mempool`](https://github.com/cosmos/cosmos-sdk/blob/v0.50.7/types/mempool/mempool.go)이 추가되어 이전 버전보다 훨씬 더 세분화된 블록을 구축할 수 있게 되었다. 개발자가 Mempool을 위해 작성할 수 있는 디자인은 무수히 많지만, Cosmos SDK는 간단한 Mempool 구현만 제공하기로 결정했다:
1. [`No-op Mempool`](https://github.com/cosmos/cosmos-sdk/blob/v0.50.7/types/mempool/noop.go):  BaseApp이 Mempool과 상호작용할 때 기본적으로 FIFO 순서로 정렬되는 RequestPrepareProposal에 정의된 CometBFT의 트랜잭션 순서에 의존한다고 가정한다.
2. [`Sender Nonce Mempool`](https://github.com/cosmos/cosmos-sdk/blob/v0.50.7/types/mempool/sender_nonce.go): Nonce 문제를 피하기 위해 Nonce별로 정렬된 목록에 트랜잭션을 보관하는 Mempool이다.
3. [`Priority Nonce Mempool`](https://github.com/cosmos/cosmos-sdk/blob/v0.50.7/types/mempool/priority_nonce.go): 우선 순위와 sender nonce로 부분 정렬된 2차원 세트에 tx를 저장하는 [`Mempool`](https://github.com/cosmos/cosmos-sdk/blob/main/types/mempool/priority_nonce_spec.md)이다. 내부적으로는 우선순위가 지정된 하나의 skiplist와 sender nonce로 정렬된 하나의 skiplist 목록을 사용한다.


## 2. 트랜잭션 라이프사이클 
하나 이상의 유효한 메시지를 포함하는 트랜잭션은 CometBFT에 의해 직렬화되고 확인되며, 절대적인 Finality을 가진다. 따라서, 트랜잭션이 블록에 포함되면 체인 re-org이나 취소의 가능성 없이 최종 확정된다.
- 확인된 bytes 형식의 트랜잭션은 내용을 알기 위해 Cosmos SDK 애플리케이션으로 전달된다. 각 메시지는 [`MsgServiceRouter`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/msg_service_router.go)를 사용하여 `BaseApp`을 통해 적절한 모듈로 라우팅된다. 
- `BaseApp`은 트랜잭션에 포함된 각 메시지를 디코딩한다. 각 모듈에는 수신된 각 메시지를 처리하는 자체 [`MsgService`](https://docs.cosmos.network/v0.47/build/building-modules/msg-services)가 있다.

> MsgService는 직접 제작해도 되지만 권장되는 방식은 Protobuf MsgService를 정의하는 것이다. 각 모듈에는 tx.proto에 정확히 하나의 Protobuf MsgService가 정의되어 있으며 모듈의 각 메시지 유형에 대한 RPC 서비스 메서드가 있다.

### 2-1. 트랜잭션 생성 및 전송
이 프로세스는 합의에 구애받지 않으므로 다양한 합의 엔진과 함께 작동할 수 있다. 트랜잭션 브로드캐스팅 단계는 다음과 같다:
1. 트랜잭션 생성 및 서명: 트랜잭션에 넣을 메시지를 결정한 후 TxBuilder를 사용하여 트랜잭션을 생성한다. 트랜잭션은 진위성과 무결성을 보장하기 위해 클라이언트의 개인 키를 사용하여 서명된다.
2. 브로드캐스팅: 서명된 트랜잭션을 [`BroadcastTx`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/client/tx/tx.go#L62-L126) 함수를 통해 네트워크로 전송한다.
3. 네트워크 전파: 노드가 트랜잭션을 수신하면 네트워크의 다른 노드로 트랜잭션이 전파된다. 이렇게 하면 모든 노드가 트랜잭션의 사본을 갖게 된다.
4. 합의 엔진 작업: 구체적인 브로드캐스팅 방법은 사용되는 합의 엔진에 따라 달라질 수 있다.

#### 트랜잭션 생성하기
[`TxBuilder`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/client/tx_config.go#L33-L50) 인터페이스에는 트랜잭션 생성에 필요한 데이터가 포함되어 있으며, 최종 사용자가 원하는 트랜잭션을 생성하도록 자유롭게 설정할 수 있다. 현재 트랜잭션 서명을 위한 두 가지 서명 모드가 있으므로, TxBuilder의 구현도 두 가지이다:
- 트랜잭션 생성을 위한 [wrapper](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/auth/tx/builder.go#L18-L34)는 `SIGN_MODE_DIRECT` 전용이다.
- [StdTxBuilder](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/auth/migrations/legacytx/stdtx_builder.go#L15-L21)는 `SIGN_MODE_LEGACY_AMINO_JSON` 전용이다. 


[`TxConfig`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/client/tx_config.go#L22-L31)는 트랜잭션 관리를 위한 앱 설정이다. 최종 사용자는 `TxConfig` 인터페이스를 사용하는 것을 선호해야 하므로 `TxBuilder` 모드는 설정 안에 숨겨져 있어야 한다. 해당 설정 값 안에 각 트랜잭션을 `SIGN_MODE_DIRECT` 또는 `SIGN_MODE_LEGACY_AMINO_JSON`으로 서명할지에 대한 정보를 담고 있다는 점이다. 
- `txBuilder := txConfig.NewTxBuilder()`를 호출하고 나서, Setter로 `TxBuilder`로 값을 잘 설정해주면 적절한 서명 모드로 새 `TxBuilder`가 생성된다. 
- `TxConfig`는 바이트 인코딩을 설정 값(`SIGN_MODE_DIRECT` or `SIGN_MODE_LEGACY_AMINO_JSON`)에 맞게 잘 처리한다. 

다음은 `TxEncoder()` 메서드를 사용하여 트랜잭션을 생성하고 인코딩하는 방법에 대한 수도 코드이다:
```go
txBuilder := txConfig.NewTxBuilder()
txBuilder.SetMsgs(...) 
txBuilder.SetMemo(...)
txBuilder.SetFeeAmount(...) // and other setters on txBuilder

bz, err := txConfig.TxEncoder()(txBuilder.GetTx())
// bz are bytes to be broadcasted over the network
```

#### 트랜잭션 브로드캐스팅하기 
앱 개발자는 일반적으로 애플리케이션의 `./cmd` 폴더에서 앱에 대한 진입점을 만든다. 트랜잭션이 생성되면 해당 폴더에 있는 다음 인터페이스 기능들을 통해 브로드캐스트할 수 있다: 
1. CLI 인터페이스
2. gRPC
3. REST 

##### 1. CLI 인터페이스 
트랜잭션을 생성하기 위해서 CLI 인터페이스를 사용하는 것이 가장 간단하다. CLI의 경우, 모듈 개발자는 `subCommand`을 만들어 애플리케이션 최상위 트랜잭션 명령인 `TxCmd`에 하위 명령으로 추가해주면 된다. 

CLI 명령은 실제로 메시지 작성, 트랜잭션 생성, 브로드캐스팅 등 트랜잭션 처리의 모든 단계를 하나의 간단한 명령으로 묶어준다. 사용자가 CLI에서 다음 형식의 명령을 입력하고 `[command]`에 트랜잭션 유형을, `[args]`에 인수를, `[flags]`에 가스 가격 등의 구성을 입력하면 트랜잭션을 생성할 수 있다:
```sh
[appname] tx [command] [args] [flags]
```

CLI 명령의 예시는 다음과 같다: 
```sh
// 자동으로 트랜잭션을 생성하고 계정의 개인 키를 사용하여 서명(sign)하고, 지정된 동료 노드에게 전파한다. 
simd tx bank send $MY_VALIDATOR_ADDRESS $RECIPIENT 1000stake 
```
1. 하나의 Msg(`x/bank`의 `MsgSend`)로 트랜잭션을 생성한다.
	- sdk.Msg 생성 
	- tx.GenerateOrBroadcastTxCLI 호출 
	- 트랜잭션 Factory 생성 (해당 Facotory 타입 메서드를 통해서 txBuilder 생성한다.)
	- msg.ValidateBasic으로 트랜잭션 메시지 검증 
2. `MY_VALIDATOR_ADDRESS` 계정에서 트랜잭션을 전송할지 사용자에게 확인을 요청한다.
3. 생성된 트랜잭션을 계정으로 서명한다.
4. (CLI가 노드의 CometBFT RPC 엔드포인트에 연결하고 있기 때문에) 서명된 트랜잭션을 네트워크에 브로드캐스트한다. 

##### 2. gRPC
gRPC는 Cosmos SDK에서 RPC 레이어의 주요 구성 요소이다. 주로 모듈의 `queryService` 부분에서 많이 사용되고 있다. gRPC를 사용하여 트랜잭션을 브로드캐스트하려면 CLI를 사용하거나 Go를 사용하여 프로그래밍 방식으로 트랜잭션을 생성, 서명 및 인코딩해야 한다. 그런 다음 모듈 외의 기능으로 브로드캐스트 기능을 제공하는 `Tx` gRPC 서비스를 통해 브로드캐스트 할 수 있다.
```sh
grpcurl -plaintext \
    -d '{"tx_bytes":"{{txBytes}}","mode":"BROADCAST_MODE_SYNC"}' \
    localhost:9090 \
    cosmos.tx.v1beta1.Service/BroadcastTx
```

##### 3. REST
REST 또한 gRPC 처럼 트랜잭션을 직접 생성하거나 서명하는 것은 불가능하며, 브로드캐스트만 가능하다. 각 gRPC 메서드에는 gRPC gateway를 사용하여 생성된 해당 REST 엔드포인트가 있다. 따라서 gRPC를 사용하는 대신 HTTP를 사용하여 POST `/cosmos/tx/v1beta1/txs` 엔드포인트에서 동일한 트랜잭션을 브로드캐스트할 수도 있다.
```sh
curl -X POST \
    -H "Content-Type: application/json" \
    -d'{"tx_bytes":"{{txBytes}}","mode":"BROADCAST_MODE_SYNC"}' \
    localhost:1317/cosmos/tx/v1beta1/txs
```

### 2-2. 트랜잭션을 Mempool에 추가하기 
생성된 트랜잭션을 수신하는 것은 풀 노드이다. 
- [comebft] ABCI 메시지 `abci.RequestCheckTx`를 애플리케이션 레이어에 전송한다.
- [app] cometbft로 부터 요청을 전달받은 앱은 [`CheckTx`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L342-L375)를 실행한다.
	1. 트랜잭션이 유효성 검사를 통과하면 각 노드가 갖고 있는 In-memory Cache 풀인 [Mempool에 추가](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L716-L720)한다.
	2. 1번이 정상적으로 진행되었으면 `abci.ResponseCheckTx`를 반환한다. 

#### CheckTx 함수
`CheckTx` 함수에 의해 호출된 `runTx` 함수는 `runTxModeCheck` 모드에서 실행된다. cometbft와 같은 합의 엔진은 트랜잭션을 [`[]byte`](https://github.com/cometbft/cometbft/blob/v0.37.0/abci/types/types.pb.go#L827-L830) 형식으로 핸들링하기 떄문에, 이를 수신받은 앱은 디코딩이 필요하다. 다음은 간략하게 표현한 `CheckTx` 함수 코드 내용이다:
```go
func (app *BaseApp) CheckTx(req abci.RequestCheckTx) abci.ResponseCheckTx {
	var mode runTxMode = runTxModeCheck

	app.runTx(mode, req.Tx)
	
	return abci.ResponseCheckTx{ ... }
}
```

#### CheckTx 함수의 runTx 함수 호출
`CheckTx`함수에 의해 호출된 `runTx` 함수는 `runTxModeCheck` 모드로 실행된다. 해당 모드로 실행된 경우에는 메시지 실행 및 상태 변경을 수행하지는 않고 해당 트랜잭션의 모든 유효성 검사만 실행하고 종료된다. 

##### 유효성 검사 
트랜잭션 검사를 우선적으로 실행하는 이유는 잘못된 트랜잭션을 가능한 한 빨리 식별하고 거부하여 연산 낭비를 방지하기 위해서이다. 유효성 검사는 `Stateless` 검사를 수행한 다음 `Stateful` 검사를 수행한다.
- `Stateless` 검사: 우선적으로 수행하는 이유는 노드가 상태에 액세스할 필요가 없어서 비용이 적게 들기 때문이다. 상태를 보관하고 있지 않은 light client나 오프라인 노드가 쉽게 수행할 수 있다.
	- 주소가 비어 있지 않은지 확인하기 
	- 트랜잭션 필드 음수가 아닌 숫자 강제하기
	- 트랜잭션의 데이터 형식 유효성 검사하기
- `Stateful` 검사: 블록체인에 현재 커밋된 상태와 비교하여 트랜잭션의 유효성 검사를 수핼한다. 이는 상태에 대한 액세스가 필요하므로 계산 집약적이고 비용도 다소 발생한다. 그래도 트랜잭션이 완전히 실행되는 트랜잭션 실행 단계에서 추가 유효성 검사가 수행되기 떄문에 이와 같이 사전 필터링하여 계산 리소스 낭비를 최소화하는 것이 주된 목적이다.
	- 계정에 충분한 자금이 있는지 확인하기 
	- 발신자에게 트랜잭션에 필요한 권한이 있는지 확인하기 
	- 트랜잭션이 상태 충돌을 일으키지 않는지 확인하기 

> 풀 노드 상태 보관: 일반적으로 상태를 여러 버전을 다양한 이유들로 인해 보관하고 있다. 예를 들어, 노드는 트랜잭션을 확인하는 과정에서 상태 변경을 실행하지만 쿼리에 응답하기 위해서는 마지막으로 커밋된 상태의 복사본이 필요하므로 커밋되지 않은 변경이 있는 상태를 사용하여 응답해서는 안된다.

[`AnteHandler`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/handler.go#L8)는 선택 사항이다:
- 캐시된 컨텍스트(ctx)의 사본이 트랜잭션 타입에 지정된 검사를 수행하는 `AnteHandler`에게 제공된다. 이 [접근 방식](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L681-L700)의 장점은 `AnteHandler`가 트랜잭션에 대해 상태 저장 검사를 수행하면서도 마지막으로 커밋된 상태를 수정하지 않는다는 것이다. 실행이 실패하면 원본 상태로 되돌릴 수 있다.
- 예를 들어, [auth 모듈 `AnteHandler`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/auth/ante/ante.go)는 시퀀스 번호를 확인 및 증가시키고, 서명을 확인하고, 수수료 차감(ex. 트랜잭션의 첫 번째 서명자로부터 수수료를 공제)하는 검사들이 이루어진다. 모든 상태 변경은 [`checkState`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L83)를 사용하여 이루어진다.

#### CheckTx 이후 
CheckTx로 트랜잭션 유효성을 검사하고, 유효한 트랜잭션만 자신의 mempool에 추가하고 피어에게 브로드캐스팅한다. 이후 앱은 mempool의 트랜잭션 목록이 블록에 포함될 때까지 대기한다. 정직한 노드로 인해 트랜잭션이 유효하지 않은 것으로 확인되면 이전에 추가한 트랜잭션을 삭제할 수도 있다. 합의에 앞서 노드는 들어오는 트랜잭션을 지속적으로 확인하여 피어와 통신한다. 

### 2-3. Block에 포함하기
블록체인 네트워크에 트랜잭션을 블록에 담아 새롭게 추가하려면 참여한 노드들로부터 합의가 필요하다. 이 과정은 일반적으로 지정된 노드(Proposer라고도 함)가 mempool에 있는 트랜잭션을 블록에 포함하는 것으로 라운드가 시작한다. 해당 블록은 네트워크의 다른 노드(검증자)들에게 제안된다.

### 2-3-a. (제안자) Block 제안하기
합의의 첫 번째 단계는 블록 제안(Proposal)이다. 합의 알고리즘은 검증자 중 한 명의 제안자를 선택하여 블록을 생성하고 제안한다. 트랜잭션이 포함되려면 이 제안자의 Mempool에 있어야 한다.

```
        -----------------------
        |Receive Block Proposal|
        -----------------------
                  |
                  v
```

#### PrepareProposal
블록을 제안하기 전에 [`PrepareProposal`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L256) 함수 호출을 통해 블록에 대한 일괄 최적화가 가능하다. 이는 경험적으로 성능 향상을 위한 핵심 요소임이 입증되었다. 이는 커스텀이 가능하며 블록 제안자는 블록을 제안하기 전에 블록에서 앱 종속적인 작업을 수행할 수 있다:
- [comebft] ABCI 메시지 `abci.RequestPrepareProposal`를 애플리케이션 레이어에 전송한다.
- [app] cometbft로 부터 요청을 전달받은 앱은 `PrepareProposal`를 실행한다.

다음은 default 기능으로 설정된 핸들러 함수 내용이다:
1. `PrepareProposal`에서는 `prepareProposalState` 상태를 사용하고 [`PrepareProposalHandler`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L946)를 호출하여 실행한다.
2. mempool의 `Select() 메서드`를 사용하여 트랜잭션을 반복한다. 
3. 반복문을 통해 [`PrepareProposalVerifyTx`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L864-L881) 함수를 실행하여 각 트랜잭션을 인코딩하고 유효성을 검사하는 `runTx`가 호출되어 `AnteHandler`와 같은 기능을 통해 유효성 검사를 진행한다. 성공하면 제안을 실행하는 동안 생성된 이벤트, 태그 및 데이터를 포함하여 유효한 트랜잭션을 반환한다.


#### ProcessProposal
제안된 블록이 있을 경우 [`ProcessProposal`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L309) 함수를 호출하여 검증자는 제안된 블록에 대해 앱 종속적인 작업을 수행할 수 있다. 즉각적인 블록 실행과 같은 기능을 사용할 수 있으며 앱이 유효하지 않은 블록을 거부할 수도 있다:
- [comebft] ABCI 메시지 `abci.RequestPrepareProposal`를 애플리케이션 레이어에 전송한다.
- [app] cometbft로 부터 요청을 전달받은 앱은 [`ProcessProposal`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L309)를 실행한다.

다음은 default 기능으로 설정된 핸들러 함수 내용이다:
1. `ProcessProposal`에서는 마지막으로 커밋된 상태를 기반으로 `processProposalState`를 사용하고 [`ProcessProposalHandler`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L1004)를 호출하여 유효성 검사를 하고 서명된 제안을 처리한다.
2. `abci.RequestProcessProposal`로 부터 받은 `[]byte` 타입으로 된 트랜잭션 배열을 반복한다.
3. 반복문을 통해 [`ProcessProposalVerifyTx`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L883-L917) 함수를 실행하여 `runTx`가 호출되어 `AnteHandler`가 실행된다. 
4. 이 상태에서 사용되는 `ctx`는 header와 main state의 정보로 구축되며, 최소 가스 가격도 설정된다. 


### 2-3-b. (검증자 노드) Block 검증하기 
Block을 제안받은 검증자 노드는 해당 mempool에 있는 트랜잭션을 수락할지 합의하기 위해 라운드 기반 합의가 진행된다. 올바른 제안자로부터 블록 제안을 받은 모든 풀노드는 [`BeginBlock`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L160), [`DeliverTx`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L382), [`EndBlock`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L219)을 순차적으로 실행한다. 
- 각 라운드는 제안자가 가장 최근 트랜잭션의 블록을 생성하는 것으로 시작하여 합의를 담당하는 투표권을 가진 풀 노드 검증자가 해당 블록을 수락할지, 아니면 `nil` 블록으로 진행할지에 동의하는 것으로 끝난다. 
- 검증자 노드는 이 합의에 도달하기 위해 애플리케이션에 ABCI 요청을 통해 트랜잭션을 확인하면서 CometBFT에 채택된 합의 알고리즘을 실행한다.
- 모든 풀노드가 개별적으로 로컬에서 작동하지만, 결과는 항상 일관되고 명확하다. 이는 메시지로 인한 상태 변화를 예측할 수 있고, 트랜잭션이 제안된 블록에서 구체적으로 순서화되어 있기 때문이다.
```
                  |
                  v
        -----------------------
        | BeginBlock	      |
        -----------------------
                  |
                  v
        -----------------------
        | DeliverTx(tx0)      |
        | DeliverTx(tx1)      |
        | DeliverTx(tx2)      |
        | DeliverTx(tx3)      |
        |			.	      |
        |			.	      |
        |			.	      |
        -----------------------
                  |
                  v
        -----------------------
        | EndBlock	     	  |
        -----------------------
                  |
                  v
        -----------------------
        | Consensus	     	  |
        -----------------------
                  |
                  v
```
EVM에서 개발자는 트랜잭션을 제출한 이후에 오프체인에서 액션을 트리거하여 이벤트를 수행해야 하는 불편함이 있는데, 텐더민트를 사용하는 Cosmos에서는 `BeginBlock`과 `EndBlock`을 사용하여 단순 트랜잭션 실행 전/후에 자동으로 호출되는 이벤트 리스너를 사용하여 개발할 수 있다. 해당 작업은 트랜잭션에 의존하여 트리거되지 않으므로 혼잡이 발생하지 않는다는 장점을 가지고 있다. 

#### BeginBlock
[`BeginBlock`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L160)은 블록이 결정된 후 정확히 한 번 호출된다.
- [comebft] ABCI 메시지 `abci.RequestBeginBlock`를 애플리케이션 레이어에 전송한다.
- [app] cometbft로 부터 요청을 전달받은 앱은 `BeginBlock`를 실행한다.

`DeliverTx` ABCI 메시지를 위해 미리 `deliverState`가 세팅된다. `deliverState`는 root 저장소에서 마지막으로 커밋된 상태를 기반으로 설정된다. 
그리고 앱이 사전에 등록한 `beginBlocker` 함수를 실행한다. 참고로, `Commit`에서는 `deliverState`가 `nil`로 설정된다. 다음은 간략하게 표현한 `BeginBlock` 함수의 수도 코드이다:
```go
func (app *BaseApp) BeginBlock(req abci.RequestBeginBlock) (res abci.ResponseBeginBlock) {
	app.validateHeight(req);
	
	if app.deliverState == nil {
		app.setState(runTxModeDeliver, req.Header)
	} else {
		app.deliverState.ctx = app.deliverState.ctx.
			WithBlockHeader(req.Header).
			WithBlockHeight(req.Header.Height)
	}

	res = app.beginBlocker(app.deliverState.ctx, req)
	
	return res
}
```

#### DeliverTx 
`BeginBlock`이 완료되면 블록 내 각 트랜잭션마다 [`DeliverTx`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L377-L414)가 한 번씩 호출된다. 
- [comebft] ABCI 메시지 `abci.RequestDeliverTx`를 애플리케이션 레이어에 전송한다.
- [app] cometbft로 부터 요청을 전달받은 앱은 `DeliverTx`를 실행한다. 

앱은 유효성을 확인하기 위해 추가 검사를 정의한다(예: Key-Value 저장소에서 키가 아직 존재하지 않는지 확인할 수 있다). 트랜잭션이 `DeliverTx`에서 확인을 통과하지 못하더라도, 이러한 트랜잭션을 기각하는 `CheckTx`와는 달리 해당 트랜잭션은 이미 투표를 거쳤으므로 블록의 일부가 된다. `DeliverTx`에서 반환된 응답은 다음 블록의 헤더에 포함된다. 다음은 간략하게 표현한 `DeliverTx` 함수의 수도 코드이다:
```go
func (app *BaseApp) DeliverTx(req abci.RequestDeliverTx) (res abci.ResponseDeliverTx) {
	app.runTx(runTxModeDeliver, req.Tx)
	
	return abci.ResponseDeliverTx{ ... }
}
```

#### DeliverTx 함수의 runTx 함수 호출
`DeliverTx` 함수에 의해 호출된 `runTx` 함수는 `runTxModeDeliver` 모드로 실행된다. `CheckTx`와 마찬가지로 상태 전환은 `deliverState`에서 발생한다. 
- `AnteHandler`가 정의되어 있으면, 이를 실행하여 트랜잭션의 기본 상태 전환을 수행한다.
	- 트랜잭션의 기본적인 검증을 수행하고, 필요한 상태 전환을 한다.
	- `AnteHandler`의 실행이 완료되면, `MultiStore` 캐시를 커밋하여 상태 전환을 영구적으로 적용한다.
- 트랜잭션 메시지([`runMsgs`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L737))를 실행한다.
	- 메시지 실행이 성공하면 `msCache.Write()`를 호출하여 `deliverState`를 커밋한다.
	- 메시지 실행이 실패하더라도 `AnteHandler`의 상태 전환(가스 소모, 수수료 지불 등)은 이미 커밋되었기 때문에 롤백이 되지않는다 점을 참고하자.


#### EndBlock
`DeliverTx`를 통해 모든 트랜잭션이 처리된 후 [`EndBlock`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L219) 실행된다. 현재 블록의 일부로 다른 트랜잭션이 전달되지 않음을 앱에 알리고 다음 블록에서 사용할 검증자 세트(ValidatorUpdates)와 합의 매개변수(ConsensusParamUpdates)의 변경을 요청한다. 

### 2-4. 상태 (영구적으로) 변경하기 
마지막 단계는 노드가 블록과 상태 변경 사항을 `Commit`하는 것이다. 검증자 노드는 트랜잭션의 유효성을 검사하기 위해 상태 전환을 실행하는 이전 단계를 수행한 다음 블록에 서명하여 이를 확인한다. 검증자가 아닌 풀 노드는 합의에 참여하지 않고, 투표할 수 없지만 상태 변경을 `Commit`할지 여부를 파악하기 위해 투표를 기다린다.
- 충분한 검증자 투표(투표력에 가중치를 둔 2/3 이상의 precommits)를 받으면 풀노드는 블록체인에 추가될 새 블록에 커밋하고 앱 레이어에서 상태 전환을 마무리한다. 
	- 새로운 앱 StateRoot가 생성되어 상태 전환에 대한 머클 증명 역할을 한다.
- 애플리케이션은 `Baseapp`에서 상속된 `Commit` ABCI 메서드를 사용하여 애플리케이션의 내부 상태에 `deliverState`를 기록함으로써 모든 상태 전환을 동기화한다. 
- 상태 변경이 커밋되는 즉시 가장 최근에 커밋된 상태부터 `checkState`가 새로 시작되고, 일관성을 유지하고 변경 사항을 반영하기 위해 `deliverState`는 0으로 재설정된다.

```
                  |
                  v
        -----------------------
        | Commit	     	  |
        -----------------------
```

#### Commit 
[`Commit`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L416-L483)은 블록체인에 추가될 새 블록에 커밋하고 앱 레이어에서 상태 전환을 마무리하고 최종 상태로 간주된다.
- [comebft] 2/3 이상의 precommit을 받으면 풀노드는 앱 레이어에 커밋  요청을 전송한다.
- [app] cometbft로 부터 커밋 요청을 전달받은 앱은 `Commit`를 실행한다. 

`Commit` 함수에 의해 `deliverState`에서 발생한 모든 상태 전환은 최종적으로 root `CommitMultiStore`에 기록된다. 이는 디스크에 영구적으로 기록하고 새로운 앱 상태 root Hash를 생성하게 된다. 마지막으로, `checkState`는 새로 커밋된 상태로 설정되고 `deliverState`는 `nil`로 설정되어 `BeginBlock`에서 재설정된다. 다음은 간략하게 표현한 `Commit` 함수의 수도 코드이다:
```go
func (app *BaseApp) Commit() abci.ResponseCommit {
	header := app.deliverState.ctx.BlockHeader()

	// commit 
	app.deliverState.ms.Write()
	commitID := app.cms.Commit()

	// state reset
	app.setState(runTxModeCheck, header)
	emptyHeader := tmproto.Header{ChainID: app.chainID}
	app.setState(runTxPrepareProposal, emptyHeader)
	app.setState(runTxProcessProposal, emptyHeader)
	app.deliverState = nil

	return abci.ResponseCommit{
		Data:         commitID.Hash,
		RetainHeight: retainHeight,
	}
}
```

예외 경우는 다음과 같다:
- 모든 블록에 동일한 수의 트랜잭션이 있는 것은 아니며, 합의에 따라 블록이 0이 되거나 `nil` 블록이 될 수도 있다. 
- 퍼블릭 블록체인 네트워크에서는 검증자가 비잔틴 또는 악의적일 수 있으며, 이로 인해 블록체인에서 `Tx` 검증이 이루어지지 않을 수도 있다. 
- 가능한 악의적 행동에는 제안자가 블록에서 `Tx`을 제외하여 검열하기로 결정하거나 검증자가 블록에 반대 투표하는 것이 포함된다.

이 시점에서 노드는 `Tx`의 유효성을 검증하고 상태 변경을 실행하여 트랜잭션을 전달하고 해당 변경 사항을 커밋했다. 트랜잭션 자체는 `[]byte` 형식으로 블록에 저장되어 블록체인에 추가된다.


## v0.50.x 이상 업데이트 사항
### 1. Tx Msg
[RFC 001](https://docs.cosmos.network/main/build/rfc/rfc-001-tx-validation)은 모듈에 대한 Message 유효성 검사 프로세스의 간소화를 정의했다. 
- [`GetSigners`](https://github.com/cosmos/cosmos-sdk/issues/11275)가 더이상 사용되지 않아 `sdk.Msg` 인터페이스는 `ValidateBasic` 메서드를 구현할 필요가 없도록 업데이트되었다. 대신 `msgServer`에서 직접 메시지의 유효성을 검사하는 것이 좋다. 다음 `x/staking` 모듈과 같이 `msgServer`내부에서 [`Validate`](https://github.com/cosmos/cosmos-sdk/blob/v0.50.7/x/staking/keeper/msg_server.go#L42-L44)를 수행하면 더 이상 `sdk.Msg`의 `ValidateBasic` 메서드는 더 이상 필요하지 않으므로 제거할 수 있다. 
- `Msg`는 더 이상 `LegacyMsg` 인터페이스를 구현할 필요가 없으며 `GetSignBytes`의 구현을 삭제할 수 있다. 이 변경으로 인해 글로벌 레거시 Amino 코덱 정의와 init()의 등록도 안전하게 제거할 수 있다.

v0.50.7에서 사용하는 트랜잭션 인터페이스는 다음과 같다:
[v0.50.7/tx_msg.go#L51-L57](https://github.com/cosmos/cosmos-sdk/blob/v0.50.7/types/tx_msg.go#L51-L57)

```go
type (
	Msg = proto.Message
	// v0.47.
	LegacyMsg interface {
		Msg
		GetSigners() []AccAddress
	}

	HasMsgs interface {
		GetMsgs() []Msg
	}

	Tx interface {
		HasMsgs
		GetMsgsV2() ([]protov2.Message, error)
	}
)
```

### 2. FinalizeBlock
[`FinalizeBlock`](https://docs.cometbft.com/v0.38/guides/go#133-finalizeblock)은 CometBFT v0.38.0에 도입된 ABCI 메서드이다. 이는 위에서 알아본 [`BeginBlock`, `DeliverTx`, `EndBlock`]을 통합하여 제공한다. 각 함수를 호출하는 대신  `FinalizeBlock` 함수 호출 한 번으로 단축되었다고 보면 된다. 


# Resources
- https://docs.cosmos.network/main/
- https://docs.cometbft.com/
- https://ida.interchain.io/academy/2-cosmos-concepts/3-transactions.html
- https://github.com/cosmos/cosmos-sdk/blob/main/docs/learn/beginner/01-tx-lifecycle.md
- Ethan Buchman, "Tendermint: Byzantine Fault Tolerance in the Age of Blockchains", Juen. 2016, https://atrium.lib.uoguelph.ca/items/5459099e-67aa-4a23-83ae-d3471d8d8336
