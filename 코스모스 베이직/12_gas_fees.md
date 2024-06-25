# 12. Gas Fees
> cosmos-sdk v0.47부터 Tendermint에서 [CometBFT로 마이그레이션](https://github.com/cosmos/cosmos-sdk/issues/14870) 했기 때문에, 아티클은 cosmos-sdk v0.47, cometbft v0.38 기반으로 작성되었다. 

## 목차 
0. Gas 수수료
1. Gas Meter
2. AnteHandler

## 0. Gas 수수료 
Cosmos SDK에서 `gas`는 실행 중 리소스 소비를 추적하는 데 사용되는 특수 단위이다. `gas`는 일반적으로 Store에 읽기 및 쓰기가 수행될 때마다 소비되지만, 비용이 많이 드는 연산을 수행해야 하는 경우에도 소비될 수 있다. `gas`는 크게 두 가지 용도로 사용된다:
- 블록이 너무 많은 리소스를 소비하지 않고 최종 완료되었는지 확인한다. 
	- 이는 Cosmos SDK에서 기본적으로 블록 [`GasMeter`](./12_gas_fees.md#1-gas-meter)를 통해 구현된다.
- 최종 사용자의 스팸 및 악용을 방지한다. 
	- 이를 위해 메시지 실행 중에 소비되는 가스에는 일반적으로 가격이 책정되어 수수료가 부과된다`(fee = gas * gasPrices)`. 
	- 수수료는 일반적으로 메시지 발신자가 지불해야 한다. 

스팸을 방지하는 다른 방법(예: 대역폭 체계)이 있을 수 있으므로 Cosmos SDK는 기본적으로 가스 가격 책정을 강제하지 않지만, 대부분의 앱은 `AnteHandler`를 사용하여 스팸을 방지하기 위한 `fee` 메커니즘을 구현한다.

## 1. Gas Meter
Cosmos SDK에서 `gas`는 uint64의 간단한 별칭이며, `GasMeter` 인터페이스를 구현된 객체에 의해 관리된다.
```go
// GasMeter interface to track gas consumption
type GasMeter interface {
	GasConsumed() Gas
	GasConsumedToLimit() Gas
	GasRemaining() Gas
	Limit() Gas
	ConsumeGas(amount Gas, descriptor string)
	RefundGas(amount Gas, descriptor string)
	IsPastLimit() bool
	IsOutOfGas() bool
	String() string
}
```

### Main Gas Meter
`ctx.GasMeter()`는 애플리케이션의 `Main GasMeter`이다. 
- 이는 `setDeliverState`를 통해 `BeginBlock`에서 초기화된 다음 상태 전환으로 이어지는 실행 시퀀스[`BeginBlock` -> `DeliverTx` -> `EndBlock`]에 의해 가스 소비량을 추적한다.
-  각 `DeliverTx`가 시작될 때, `Main GasMeter`는 트랜잭션당 가스 소비량을 추적할 수 있도록 `AnteHandler`에서 0으로 설정되어야 한다.

#### 자동 Gas 소비
대부분의 경우 스토어에 읽기/쓰기가 있을 때마다 자동으로 수행된다. 이 자동 가스 소비 로직은 [`GasKv`](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/store/gaskv/store.go)라는 특수 스토어에서 구현된다.

#### 수동 Gas 소비
수동으로 소비되는 경우는 앱의 각 모듈에서 `BeginBlocker`, `EndBlocker` 또는 `Msg` 서비스를 통해 연산이 수행되는 경우이다.


### Block Gas Meter
[`ctx.BlockGasMeter()`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/types/context.go#L59)는 블록당 gas 소비량을 추적하고 특정 한도를 초과하지 않도록 하는 데 사용되는 gas 추적기이다. 제네시스 단계에서는 초기화 트랜잭션을 수용하기 위해 gas 소비가 무제한임을 [설정](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L81-L82)해준다:
```go
app.deliverState.ctx = app.deliverState.ctx.WithBlockGasMeter(sdk.NewInfiniteGasMeter())
```

다음은 [`BeginBlock`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/abci.go#L188-L193)을 실행할 때 블록의 합의 매개변수에 따라 유한한 한도로 블록 `GasMeter`를 설정하는 메커니즘이다. 이를 통해 과도한 gas 수수료가 부과되는 것을 미리 방지할 수 있다:
```go
gasMeter := app.getBlockGasMeter(app.deliverState.ctx)
app.deliverState.ctx = app.deliverState.ctx.
		WithBlockGasMeter(gasMeter).
		WithHeaderHash(req.Hash).
		WithConsensusParams(app.GetConsensusParams(app.deliverState.ctx))
```

Cosmos SDK 내의 모듈은 실행 중 언제든 `ctx`를 활용하여 블록 gas를 소비할 수 있다. 이 gas 소비는 주로 상태 읽기/쓰기 작업과 트랜잭션 처리 중에 발생한다. `DeliverTx` 함수를 실행하여 트랜잭션을 처리할 때 다음과 같은 작업이 이루어진다: 
- 트랜잭션을 실행하기 전에 `ctx.BlockGasMeter()` 통해 총 gas 사용량을 모니터링하여 가스가 부족한 경우 트랜잭션을 실행하지 않고 바로 반환한다.  
- 트랜잭션이 정상정으로 실행된다면, 블록 `GasMeter`의 현재 값이 한도를 초과하는지 확인한다. 

다음은 `runTx`의 함수 [수도 코드](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/baseapp/baseapp.go#L641-L663)이다:
```go
if mode == runTxModeDeliver && ctx.BlockGasMeter().IsOutOfGas() {
	return gInfo, nil, nil, 0, sdkerrors.Wrap(sdkerrors.ErrOutOfGas, "no block gas left to run tx")
}

ctx.BlockGasMeter().ConsumeGas(
	ctx.GasMeter().GasConsumedToLimit(), "block gas meter",
)
```

## 2. AnteHandler
[`10_transaction_and_mempool`](./10_transaction_and_mempool.md)에서 알아봤다시피, `AnteHandler`는 앱이 `CheckTx`, `BeginBlock`, `DeliverTx` 및 `EndBlock` 와 같은 트랜잭션 관련 요청을 받게 될 때  각 `sdk.Msg`에 대한 Protobuf `Msg` 서비스 함수를 실행하기 전에 `AnteHandler`를 우선적으로 실행하여 유효성 검사를 실행한다. 


# Resources
- https://docs.cosmos.network/main/
- https://github.com/cosmos/cosmos-sdk/blob/main/docs/learn/beginner/04-gas-fees.md