# 25. `slashing` Moudle
## 목차
0. `slashing` 모듈
1. Staking Tombstone
2. 상태
3. 상태 조회
4. 상태 변환

## 0. `slashing` module
`slashing` 모듈은 규정을 위반한 행위자에게 경제적 불이익을 주어 그들의 행동을 억제하는 역할을 한다. 이러한 경제적 불이익에는 일정량의 지분 소각과 일정 기간 동안 블록에 투표할 수 있는 권한 박탈이 포함된다. 이 모듈은 Cosmos 네트워크의 첫 번째 허브인 Cosmos Hub에서 사용된다.

### State
상태 머신에 등록된 검증자는 언제든지 수에 상관없이 존재할 수 있다. 
- 각 블록에서 상위 `MaxValidator`(`x/staking`에 정의됨) 검증인은 보증금을 예치한 상태로 블록을 제안하고 투표할 수 있다. 
- 보증금을 예치한 검증인이 프로토콜 오류를 저지르면 자신의 지분과 위임자의 지분 일부 또는 전부가 위험에 처하게 된다. 
- 이러한 각 검증인에 대해 검증인의 생존 여부 및 기타 위반 관련 속성이 포함된 `ValidatorSigningInfo` 레코드를 보관한다. 

### Tombstone Cap
Cosmos Hub는 초기에 발생할 수 있는 비악의적인 프로토콜 오류의 영향을 완화하기 위해 각 검증자에 대해 이중 서명 오류에 대해 검증자를 한 번만 슬래시하는 Tombstone Cap을 구현한다. 예를 들어, 암호화 키를 안전하게 관리하는 장치인 HSM(Hardware Security Module)을 잘못 구성하여 여러 개의 오래된 블록에 이중 서명하는 경우, 첫 번째 이중 서명에 대해서만 처벌을 받게 되고 즉시 무덤에 묻히게 된다. 여전히 비용이 많이 들고 피하는 것이 바람직하지만, 툼스톤 캡은 의도치 않은 구성 오류로 인한 경제적 영향을 어느 정도 무디게 한다.

툼스톤 개념은 위반이 발생한 시점과 상태 머신에 도달하는 증거 사이에 지연이 있는 결함에만 적용된다. 
Liveness 결함은 서로 중첩될 수 없기 때문에 따로 상한선을 두지 않는다. 왜냐하면 이는 발생하는 즉시 감지되어 해당 검증자는 즉시 감옥에 갇히게 되기 떄문에 중간에 감옥에서 풀려나지 않는 한 여러 번의 Liveness 결함을 저지를 수가 없기 때문이다. 

### Infraction Timelines
`x/slashing` 모듈이 CometBFT 합의를 통해 제출된 증거를 처리하는 방법을 설명하기 위해 다음 예시를 살펴보자:
- $[$ : timeline start
- $]$ : timeline end
- $C_n$ : infraction n committed
- $D_n$ : infraction n discovered
- $V_b$ : validator bonded
- $V_u$ : validator unbonded

#### 1. 단일 이중 서명 위반
```
[----------C1----D1,Vu-----]
```
한 번의 위반이 발생한 후 나중에 발견되면 해당 검증자는 본드가 해제되고 위반 금액 전액이 삭감된다.

#### 2. 다중 이중 서명 위반
```
[----------C1--C2---C3---D1,D2,D3Vu-----]
```
여러 위반을 저지른 후 나중에 발견되면 검증자는 한 번의 위반으로만 감옥에 갇히고 삭감된다. 해당 검증자 툼스톤 처리되므로 유효성 검사자 집합에 다시 참여할 수 없다.

## 1. Staking Tombstone
현재 `slashing` 모듈의 구현에서는 합의 엔진이 검증인의 합의 결함을 상태 머신에 알리면 검증인은 부분적으로 슬래싱되고 검증인 집합에 다시 참여할 수 없는 기간인 감옥 기간(jail period)에 놓이게 된다. 그러나 합의 결함 및 ABCI의 특성으로 인해 위반이 발생하고 위반 증거가 상태 머신에 도달하기까지 지연이 있을 수 있다 (이것이 UnbondingPeriod가 존재하는 주된 이유 중 하나이다).

현재 시스템 설계에서는 검증자가 합의 결함으로 인해 감옥에 들어가고, 이후 감옥 기간이 지나면 `unjail` 트랜잭션을 보내 스스로 감옥을 해제하고 검증자 집합에 다시 참여할 수 있다. 

### 한 번에 다중 위반하는 경우: 슬래싱 기간(slashing period)
`slashing` 모듈은 증거가 실행되기 전에 여러 위반이 발생하고 검증자가 감옥에 들어가면 누적된 처벌을 하지 않고 최악의 위반 하나에 대해서만 처벌해야 하는 기능을 중요시 생각한다. 예를 들어, 다음과 같은 경우이다:
1. Validator A commits 위반 1 (worth 30% slash)
2. Validator A commits 위반 2 (worth 40% slash)
3. Validator A commits 위반 3 (worth 35% slash)
4. Evidence for 위반 1 reaches state machine (and validator is put in jail)
5. Evidence for 위반 2 reaches state machine
6. Evidence for 위반 3 reaches state machine

이는 검증자의 합의 키가 손상된 경우 해커가 여러 블록에 이중 서명을 할 경우에 발생할 수 있다. 이러한 경우에는 단 한 번만 처벌받도록 하기 위해 가장 높은 위반인 '위반 2'에만 슬래시가 적용된다. 왜냐하면 탈옥은 검증자의 연산자 키로 이루어져야 하기 때문에 검증자는 합의 키를 다시 확보한 다음 연산자 키를 사용하여 준비되었다는 신호를 보낼 기회가 있기 때문이다. 이렇게 최대 위반만 추적하는 이 기간을 슬래싱 기간(slashing period)이라고 부른다.

### 이미 처벌받은 위반을 다시 신고당하는 경우 
검증자가 스스로 `unjail`하여 다시 참여하면 새로운 슬래싱 기간이 시작되며, 감옥 해제 후 새로운 위반을 저지르면 이전 슬래싱 기간의 최악의 위반에 더해 누적적으로 슬래싱이 적용된다. 

위반은 슬래싱 기간을 기준으로 그룹화된다. 그런데 만약 신고자가 슬래싱 해제 기간까지 증거를 제출하여 이미 처벌받은 위반에 대해 중복 처벌을 받을 수 있으므로 이전에 다룬 슬래싱 기간에 대한 증거 제출도 허용해야 한다. 예를 들어 다음과 같은 경우이다:
1. Validator A commits 위반 1 (worth 30% slash)
2. Validator A commits 위반 2 (worth 40% slash)
3. Evidence for 위반 1 reaches state machine (and Validator A is put in jail)
4. Validator A unjails

이제 새로운 슬래싱 기간에 들어갔지만 '위반 2'에 대한 증거가 여전히 들어올 수 있으므로 이전 위반에 대한 문은 계속 열어두어야 한다. 슬래싱 기간이 늘어날수록 각 슬래싱 기간마다 가장 높은 위반 금액을 추적해야 하므로 더 복잡해진다.

슬래싱 기간의 최대값은 `len(UnbondingPeriod) / len(JailPeriod)`입니다. 현재 Gaia의 기본값인 `UnbondingPeriod`와 `JailPeriod`는 각각 3주, 2일이다. 
- 즉, 검증자당 최대 11개의 슬래싱 기간을 동시에 추적할 수 있다. 
- `JailPeriod >= UnbondingPeriod`를 설정하면 단 하나의 슬래싱 기간만 추적하면 된다(즉, 슬래싱 기간을 추적할 필요가 없다).

현재 감옥 기간 구현에서는 검증자가 감옥을 해제하면, 해당 검증자에게 위임된 모든 위임자(본드 해제/재위임하지 않은)가 해당 검증자와 함께 유지된다. 

합의 안전 결함이 매우 심각하다는 점을 고려할 때(liveness 결함보다 훨씬 더 심각히다), 위임자가 검증인에게 "자동 재위임"하지 않도록 하는 것이 현명할 것이다.

#### Proposal: infinite jail
합의 결함을 저지른 검증인의 '징역 시간'을 무한대(즉, 툼스톤 상태)로 설정하는 것이 좋다. 이를 통해 해당 검증인을 검증인 집합에서 쫓아내고 검증인 집합에 재진입할 수 없도록 한다. 이렇게 되면 검증자 본인을 포함한 모든 위임자는 위임을 해제하거나 재위임해야 한다. 검증자 운영자는 원한다면 새로운 운영자 키와 합의 키로 새로운 검증자를 만들 수 있지만 새롭게 다시 위임을 받아야 한다. 

툼스톤 시스템을 구현하고 슬래싱 기간 추적을 제거하면 특히 `staking` 모듈에서 소비하는 `slashing` 모듈에 정의된 모든 hook를 제거할 수 있으므로 슬래싱 모듈이 훨씬 더 간단해진다.

#### Single slashing amount
또 다른 최적화를 할 수 있는 방법은 CometBFT 합의에 대한 모든 ABCI 결함이 동일한 수준에서 삭감된다고 가정하면 "최대 삭감"을 추적할 필요가 없다는 것이다. 일단 ABCI 오류가 발생하면 최대값을 찾기 위해 미래의 잠재적 오류를 비교하는 것에 대해 걱정할 필요가 없다.

현재 유일한 CometBFT ABCI 결함이다: 
- 정당하지 않은 precommit(이중 서명)

현재 가까운 시일 내에 다음 결함을 포함할 계획이다:
- 언본딩 단계에 있을 때 precommit 서명 (light client 분할을 안전하게 하기 위해 필요)

이 결함들은 모두 비잔틴 결함에 기인하는 것이므로 똑같이 삭감하고 싶을 것이며, 따라서 위의 변경 사항을 적용할 수 있다.

## 1. 상태
### 1.Signing Info (Liveness)
모든 블록에는 이전 블록에 대한 검증자의 precommit 집합이 포함되며, 이를 CometBFT에서 제공하는 `LastCommitInfo`라고 한다. `LastCommitInfo`는 총 투표권의 2/3 이상의 precommit을 포함하는 한 유효하다.
```go
type LastCommitInfo struct {  
	Round int32  
	Votes []VoteInfo  
}
```

검증자는 일정 블록 수 동안 `LastCommitInfo`에 포함되지 못하면 자동 감금, 잠재적 슬래시, 보증금 예치 해제 등의 불이익을 받게 된다. 

검증자의 livenss 활동에 대한 정보는 [`ValidatorSigningInfo`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/slashing/v1beta1/slashing.proto#L13-L36)를 통해 추적된다. 
```protobuf
message ValidatorSigningInfo {  
	option (gogoproto.equal) = true;  
	option (gogoproto.goproto_stringer) = false;  
	  
	string address = 1 [(cosmos_proto.scalar) = "cosmos.AddressString"];  
	int64 start_height = 2;  
	int64 index_offset = 3;  
	google.protobuf.Timestamp jailed_until = 4  
	[(gogoproto.stdtime) = true, (gogoproto.nullable) = false, (amino.dont_omitempty) = true];  
	
	bool tombstoned = 5;  
	 
	int64 missed_blocks_counter = 6;  
	}
```

### 2. Params
`slashing` 모듈은 접두사가 `0x00`인 상태로 [`Params`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/slashing/v1beta1/slashing.proto#L37-L59)를 저장하며, 거버넌스 또는 권한이 있는 주소로 업데이트할 수 있다.
```protobuf
message Params {  
	option (amino.name) = "cosmos-sdk/x/slashing/Params";  
	  
	int64 signed_blocks_window = 1;  
	bytes min_signed_per_window = 2 [  
	(gogoproto.customtype) = "github.com/cosmos/cosmos-sdk/types.Dec",  
	(gogoproto.nullable) = false,  
	(amino.dont_omitempty) = true  
	];  
	google.protobuf.Duration downtime_jail_duration = 3  
	[(gogoproto.nullable) = false, (amino.dont_omitempty) = true, (gogoproto.stdduration) = true];  
	bytes slash_fraction_double_sign = 4 [  
	(gogoproto.customtype) = "github.com/cosmos/cosmos-sdk/types.Dec",  
	(gogoproto.nullable) = false,  
	(amino.dont_omitempty) = true  
	];  
	bytes slash_fraction_downtime = 5 [  
	(gogoproto.customtype) = "github.com/cosmos/cosmos-sdk/types.Dec",  
	(gogoproto.nullable) = false,  
	(amino.dont_omitempty) = true  
	];  
}
```

## 3. 상태 조회
`slashing` 모듈에서 어떠한 상태 정보를 조회하는지 간략하게 목록을 살펴보자:
1. params: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/slashing#params-1)
2. signing-info 조회하기: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/slashing#signing-info)
3. signing-info 모두 조회하기: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/slashing#signing-infos)

## 4. 상태 변환
`slashing` 모듈에서 어떠한 상태 변환을 일으키는지 간략하게 목록을 살펴보자:
1. unjail: [tx](https://github.com/cosmos/cosmos-sdk/tree/main/x/slashing#unjail-1)



## Resources
- https://docs.cosmos.network/main/build/modules/staking