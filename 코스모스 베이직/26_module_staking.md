# 25. `staking` Moudle
## 목차
0. `staking` 모듈
1. 상태
2. Validator 상태 및 상태 변환 
2. 상태 조회
3. 상태 변환
4. 실습 
    1. simd 실행하기 
    2. 검증자에게 토큰 위임하기 
    3. 검증자 정보 수정하기
    4. 보상과 주식 지분
    5. 토큰 위임 철회하기

## 0. `staking` module
`staking` 모듈은 Proof of Stake(PoS) 기능을 지원한다. 체인의 기본 스테이킹 토큰 보유자는 검증자가 될 수 있으며, 검증자에게 토큰을 위임하여 궁극적으로 시스템에 대한 효과적인 검증자 집합을 결정할 수 있다. 이 모듈은 Cosmos 네트워크의 첫 번째 허브인 Cosmos Hub에서 사용된다.

## 1. 상태
1. [`Pool`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/staking/v1beta1/staking.proto#L370-L389): 예치된 토큰에 대한 전체적인 정보를 관리한다. 예치된(bonded) 토큰과 예치되지 않은(unbonded) 토큰의 공급을 추적한다.
2. [`LastTotalPower`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/staking/v1beta1/genesis.proto#L16-L22): 이전 endblock 동안 기록된 예치된 토큰의 총량을 저장한다. "Last" 접두사가 붙은 저장소 항목은 endblock 구간에서만 상태 변경이 일어나야 한다.
3. [`UnbondingID`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/staking/v1beta1/staking.proto#L257): 가장 최신에 발생한 예치금 해제(unbonding)에 대한 ID를 저장한다. 즉, 예치된 토큰에 대해서 관련된 작업(검증자의 예치된 토큰 위임 해제, 위임자의 예치된 토큰 위임 해제, 재위임)이 새롭게 발생할 때마다 `UnbondingID`가 증가하여 예치 해제 작업에 대한 고유한 ID를 생성한다. 
4. [`Params`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/staking/v1beta1/staking.proto#L310-L333): `staking` 모듈은 접두사가 `0x51`인 상태로 파라미터를 저장하며, 거버넌스 또는 권한이 있는 주소로 업데이트할 수 있다.
5. [`Validator`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/staking/v1beta1/staking.proto#L82-L138): 검증자는 `Bonded`, `Unbonded`, `Unbonding` 세 가지 상태를 가질 수 있다. 
6. [`Delegation`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/staking/v1beta1/staking.proto#L198-L216): 위임은 `DelegatorAddr`와 `ValidatorAddr`를 결합하여 식별한다. 
7. [`UnbondingDelegation`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/staking/v1beta1/staking.proto#L198-L216): 위임된 토큰은 해제할 수 있지만, 비잔틴 행위가 감지되면 토큰을 슬래싱할 수 있는 일정 기간이 필요하다. 
8. [`Redelegation`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/proto/cosmos/staking/v1beta1/staking.proto#L198-L216): 위임된 토큰은 현재 검증인에서 다른 검증인(대상 검증인)으로 즉시 재위임될 수 있다. 그러나 이 경우 재위임 객체에서 추적해야 하며, 토큰이 현재 검증자가 저지른 비잔틴 오류에 기여한 경우 해당 토큰의 지분이 삭감될 수 있다.

## 2. Validator 상태 및 상태 변환 
### 1. 상태 
#### `Unbonded`
검증자 active 집합에 속하지 않은 상태를 나타낸다. 
- 위임자로부터 토큰을 위임을 받을 수 있다.
- 블록에 서명할 수 없으며 보상을 받을 수 없다. 

#### `Bonded`
validator가 충분한 예치된 토큰을 받으면 `EndBlock` 중에 자동으로 active 집합에 합류하고 상태가 `Bonded` 상태로 업데이트된다. 
- 블록에 서명하고 보상을 받을 수 있다. 
- 추가로 토큰을 위임을 받을 수 있다. 
- liveness에 저해되는 잘못된 행동으로 인해 예치금이 삭감될 수 있다. (slashing)

#### `Unbonding`
검증자가 자의든 타의든 슬래싱, 감금 또는 툼스톤으로 인해 active 집합에 속하지 않게 되면, 위임된 모든 예치금 해제가 시작된다. 이후 모든 위임자는 토큰이 `BondedPool`에서 자신의 계정으로 이동하기 전까지 예치가 해제되는 시간(`UnbondingTime`)을 기다려야 한다. 만약 검증인이 위임 해제하기 전에 예치된 위임자의 토큰으로 부적절한 행위를 한 사건이 발견된다면, 위임 해제를 기다리는 동안에도 여전히 슬래시될 수 있다.

### 2. 상태 변환 
검증자의 상태 변환은 active `ValidaotorSet` 집합의 변경 사항을 `EndBlock`에서 확인하면서 수행된다. 
- 검증자는 `Unbonded`, `Unbonding` 또는 `Bonded` 상태일 수 있다. 
- `Unbonded`, `Unbonding`을 통칭하여 `Not Bonded`라고 한다. 
- 검증자는 (`Bonded` → `Unbonded`)를 제외한 모든 상태 사이를 직접 이동할 수 있다.
- Jail 🔁 Unjail: 검증자가 감옥에 갇히면 CometBFT 집합에서 제거된다. 이 프로세스는 양방향으로 진행될 수 있다. 


## 3. 상태 조회
`staking` 모듈에서 어떠한 상태 정보를 조회하는지 간략하게 목록을 살펴보자:
1. delegation 조회: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#delegation-1)
2. delegation 모두 조회: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#delegations-1)
3. delegations-to: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#delegations-to) 
4. historical-info: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#historical-info)
5. params: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#params-1)
6. pool: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#pool-1)
7. redelegation 조회: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#redelegation-1) 
8. redelegation 모두 조회: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#redelegations-1) 
9. redelegations-from: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#redelegations-from)
10. unbonding-delegation 조회: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#unbonding-delegation)
10. unbonding-delegations: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#unbonding-delegations-1)
11. unbonding-delegations-from: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#unbonding-delegations-from)
12. validator: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#validator-1)
13. validators: [cli](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#validators-1)

## 6. 상태 변환
`staking` 모듈에서 어떠한 상태 변환을 일으키는지 간략하게 목록을 살펴보자:
1. create-validator: [tx](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#create-validator)
2. delegate: [tx](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#delegate-1)
3. edit-validator: [tx](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#edit-validator)
4. redelegate: [tx](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#redelegate)
5. unbond: [tx](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#unbond)
6. cancel unbond: [tx](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#cancel-unbond)
7. rotate cons pubkey: [tx](https://github.com/cosmos/cosmos-sdk/tree/main/x/staking#rotate-cons-pubkey)

## 4. 실습 
이번 실습에서는 검증자(Validator)로 참여하고 위임자(Delegator)로서 토큰을 위임하고 철회하는 과정을 통해 검증자와 위임자의 관계를 이해한다. 


### 1. simd 실행하기 
> 이전에 `simd`를 사용한 적이 있다면 홈 디렉터리에 이미 `.simapp` 디렉터리가 있을 수 있다. 이전 데이터를 유지하려면 디렉터리를 다른 위치에 저장하거나 `--home` 플래그를 사용하여 각 명령에 대해 다른 디렉터리를 지정해야 한다. 이전 데이터를 유지하지 않으려면 이전 디렉터리를 제거해준다.(`rm -rf ~/.simapp && rm -rf ~/.simd-bob`).


cosmos-sdk 레포지토리를 복제하고, 버전은 `v0.45.4`로 변경한다:
```sh
$ git clone https://github.com/cosmos/cosmos-sdk
$ cd cosmos-sdk && git checkout v0.45.4
```

`simd` 바이너리를 빌드한다:
```sh
$ make install
```

빌드가 완료되었으면 `simd`가 제대로 동작하는 버전 체크를 통해 확인한다: 
```sh
$ simd version

0.45.4 # good!
```


#### Chain 설정하기
chain ID를 설정해준다:
```sh
$ simd config chain-id staking-demo
```

keyring-backend 이름을 설정해준다:
```sh
$ simd config keyring-backend test 
```

#### Key 설정하기
validator 역할을 할 alice와 delegator 역할을 할 bob, charles를 생성해준다:
```sh
$ simd keys add alice && simd keys add bob && simd keys add charles
```

```sh
$ simd keys list
```

```sh
- name: alice
  type: local
  address: cosmos1qjfvucv56atm3pam8pxf8kxgnf8mm0syk8w6y3
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AsjlT55xZlxtl9eqk39bXDCoqIV3EWMidzCOBJpDuxD0"}'
  mnemonic: ""
- name: bob
  type: local
  address: cosmos1kztp9mle077sjerhrj49zqlt9te2sv9gsafhs7
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Agr8hi52jrGpMB9h2kzG1SS0+4ScIwdFQBfnBdnuz6lb"}'
  mnemonic: ""
- name: charles
  type: local
  address: cosmos1h8d0cdf55qvmcm3xpmvklh95m053qq4m7xzmzx
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A+FXq8BR0gwCOYxaJLnPAaDOwO81gtTWNBOFXCYK/gRI"}'
  mnemonic: ""
```

#### Chain 시작하기
나머지 기본 설정을 추가해준다:
```sh
$ simd init test --chain-id staking-demo &&
simd add-genesis-account alice 1000000000stake --keyring-backend test &&
simd add-genesis-account bob 500000000stake --keyring-backend test &&
simd add-genesis-account charles 500000000stake --keyring-backend test && 
simd gentx alice 1000000stake --chain-id staking-demo && 
simd collect-gentxs
```

이제 체인을 시작한다:
```sh
$ simd start
```

쿼리나 트랜잭션 명령어를 입력할 때 사용자 주소를 복사하여 붙여넣지 않으려면 shell에 사용자 키를 액세스하여 사용할 수 있는 변수로 미리 설정하는 것이 좋다. 
```sh
$ export ALICE=$(simd keys show alice --address) && 
export ALICE_VAL=$(simd keys show alice --bech val --address) &&
export BOB=$(simd keys show bob --address) && 
export CHARLES=$(simd keys show charles --address)
```

### 2. 검증자에게 토큰 위임하기 
#### Alice 검증자 조회하기 
Alice는 앱을 시작할 때 네트워크 초기화 과정에서 초기 검증자로 설정되었다. 그래서 Alice 검증자 정보에 대해 쿼리해보도록 하자:
```sh
$ simd query staking validator $ALICE_VAL && 
simd query staking delegations-to $ALICE_VAL
```

조회 결과는 다음 샘플과 같다:
```sh
# simd query staking validator $ALICE_VAL
commission:
  commission_rates:
    max_change_rate: "0.010000000000000000"
    max_rate: "0.200000000000000000"
    rate: "0.100000000000000000"
  update_time: "2024-07-04T16:18:19.881014Z"
consensus_pubkey:
  '@type': /cosmos.crypto.ed25519.PubKey
  key: fJxptFMZIv+MPmI1hMHPzK016iwLrk1YhMBvub1NqUQ=
delegator_shares: "1000000.000000000000000000"
description:
  details: ""
  identity: ""
  moniker: test
  security_contact: ""
  website: ""
jailed: false
min_self_delegation: "1"
operator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
status: BOND_STATUS_BONDED
tokens: "1000000"
unbonding_height: "0"
unbonding_time: "1970-01-01T00:00:00Z"

# simd query staking delegations-to $ALICE_VAL
delegation_responses:
- balance:
    amount: "1000000"
    denom: stake
  delegation:
    delegator_address: cosmos1qjfvucv56atm3pam8pxf8kxgnf8mm0syk8w6y3
    shares: "1000000.000000000000000000"
    validator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
pagination:
  next_key: null
  total: "0"
```

#### 토큰 위임하기 
Bob과 Charles가 Alice 검증자 주소로 토큰을 위임한다:
```sh
$ simd tx staking delegate $ALICE_VAL 100000stake --from=bob
$ simd tx staking delegate $ALICE_VAL 200000stake --from=charles
```

위임되고 나서 Alice 검증자 정보를 다시 확인해보도록 하자:
```sh
$ simd query staking validator $ALICE_VAL && 
simd query staking delegations-to $ALICE_VAL
```

그러면 조회 결과는 다음과 같다: 
```sh
# simd query staking validator $ALICE_VAL
commission:
  commission_rates:
    max_change_rate: "0.010000000000000000"
    max_rate: "0.200000000000000000"
    rate: "0.100000000000000000"
  update_time: "2024-07-04T16:18:19.881014Z"
consensus_pubkey:
  '@type': /cosmos.crypto.ed25519.PubKey
  key: fJxptFMZIv+MPmI1hMHPzK016iwLrk1YhMBvub1NqUQ=
delegator_shares: "1300000.000000000000000000"
description:
  details: ""
  identity: ""
  moniker: test
  security_contact: ""
  website: ""
jailed: false
min_self_delegation: "1"
operator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
status: BOND_STATUS_BONDED
tokens: "1300000"
unbonding_height: "0"
unbonding_time: "1970-01-01T00:00:00Z"

# simd query staking delegations-to $ALICE_VAL
delegation_responses:
- balance:
    amount: "1000000"
    denom: stake
  delegation:
    delegator_address: cosmos1qjfvucv56atm3pam8pxf8kxgnf8mm0syk8w6y3
    shares: "1000000.000000000000000000"
    validator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
- balance:
    amount: "100000"
    denom: stake
  delegation:
    delegator_address: cosmos1kztp9mle077sjerhrj49zqlt9te2sv9gsafhs7
    shares: "100000.000000000000000000"
    validator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
- balance:
    amount: "200000"
    denom: stake
  delegation:
    delegator_address: cosmos1h8d0cdf55qvmcm3xpmvklh95m053qq4m7xzmzx
    shares: "200000.000000000000000000"
    validator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
pagination:
  next_key: null
  total: "0"
```

### 3. 검증자 정보 수정하기
Alice 검증자 정보 중애 moniker를 수정해보도록 하자:
```sh
$ simd tx staking edit-validator --moniker="new-alice-validator" --from=alice
```

이를 조회해보면 잘 변경된 것을 확인할 수 있다. 
```sh
$ simd query staking validator $ALICE_VAL | grep moniker
moniker: new-alice-validator # good!
```
- `simd tx staking edit-validator --help` 명령어를 통해 변경할 수 있는 flag들을 확인해 볼 수 있다.

### 4. 보상과 주식 지분
#### 보상 적용에 따른 지분 변화 
검증인은 토큰 수 T를 보유하고 있으며, 발행된 주식 수 S를 보유하고 있다. 
- `T = S + 보상 - 슬래싱된 토큰`이다.
- 각 $위임자_i$는 주식 수 $S_i$를 보유하고 있다. 

위임자는 자신의 지분 비율에 비례하여 $T * S_i / S$에 해당하는 보상을 받을 자격이 있다. 

그리고 위임자가 검증인에게 새로운 토큰을 위임하면, 위임자는 자신의 기여도에 비례하는 수의 지분을 받게 된다. 
- 따라서 위임자 j가 $T_j$ 토큰을 위임하면 $S_j = S * T_j / T$ 주식을 받게 된다. 
- 이제 총 토큰 수는 $T + T_j$이고, 총 지분 수는 $S + S_j$이다. 
- j의 지분 비율은 기여한 총 토큰에서 차지하는 비율과 동일하다: $(S + S_j) / S = (T + T_j) / T$. 즉, 총 토큰 수에 비례하는 지분 비율이다.

여기서는 가상 보상으로 `50000stake` 토큰을 받았다고 가정하고 임의로 Bob에게 수동으로 보상을 추가한 후 토큰 수에 비례하여 지분이 얼마나 증가하는지 관찰해보도록 하자. 
```sh
# 예시: Bob에게 보상으로 50,000 stake 추가
$ simd tx staking delegate $ALICE_VAL 50000stake --from=bob
```


#### Alice 검증자 위임 정보 확인하기 
토큰이 추가되고 나서 Alice 검증자에게 위임된 정보를 다시 확인해보도록 하자:
```sh
$ simd query staking delegations-to $ALICE_VAL
```

조회 결과는 다음 샘플과 같다:
```sh
delegation_responses:
- balance:
    amount: "1000000"
    denom: stake
  delegation:
    delegator_address: cosmos1qjfvucv56atm3pam8pxf8kxgnf8mm0syk8w6y3
    shares: "1000000.000000000000000000"
    validator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
- balance: # Bob!
    amount: "150000" 
    denom: stake
  delegation:
    delegator_address: cosmos1kztp9mle077sjerhrj49zqlt9te2sv9gsafhs7
    shares: "150000.000000000000000000"
    validator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
- balance:
    amount: "200000"
    denom: stake
  delegation:
    delegator_address: cosmos1h8d0cdf55qvmcm3xpmvklh95m053qq4m7xzmzx
    shares: "200000.000000000000000000"
    validator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
pagination:
  next_key: null
  total: "0"
```
- Bob의 주식도 추가된 토큰 양에 비례하여 1:1로 증가한 것을 확인할 수 있다. 
- 이는 초기 위임으로, T = 0이고 S = 0이므로 $T_j / T$가 정의되지 않았기 때문에, $T_j$ 토큰을 위임한 위임자 j는 $S_j = T_j$ 지분을 받는다. 따라서 보상을 받지 않았고 슬래시되지 않은 검증자는 T = S를 갖게 된다.


### 5. 토큰 위임 철회하기
`unbond` 명령을 통해 Bob은 Alice로부터 토큰을 철회할 수도 있다.
```sh
$ simd tx staking unbond $ALICE_VAL 100000stake --from=bob
```

#### Alice 검증자 위임 정보 확인하기 
토큰이 추가되고 나서 Alice 검증자에게 위임된 정보를 다시 확인해보도록 하자:
```sh
$ simd query staking delegations-to $ALICE_VAL
```

조회 결과는 다음 샘플과 같다:
```sh
delegation_responses:
- balance:
    amount: "1000000"
    denom: stake
  delegation:
    delegator_address: cosmos1qjfvucv56atm3pam8pxf8kxgnf8mm0syk8w6y3
    shares: "1000000.000000000000000000"
    validator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
- balance:
    amount: "50000"
    denom: stake
  delegation:
    delegator_address: cosmos1kztp9mle077sjerhrj49zqlt9te2sv9gsafhs7
    shares: "50000.000000000000000000"
    validator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
- balance:
    amount: "200000"
    denom: stake
  delegation:
    delegator_address: cosmos1h8d0cdf55qvmcm3xpmvklh95m053qq4m7xzmzx
    shares: "200000.000000000000000000"
    validator_address: cosmosvaloper1qjfvucv56atm3pam8pxf8kxgnf8mm0synn60gz
pagination:
  next_key: null
  total: "0"
```
- Bob의 토큰 `100000stake` 감소하여 `50000stake`되었고, 그에 따라서 주식 지분도 1:1 비율로 감소한 것을 확인할 수 있다. 


## Resources
- https://docs.cosmos.network/maimodules/staking

