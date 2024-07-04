# 22. `bank` Moudle
## 목차
0. `bank` 모듈
1. 상태
2. 상태 조회
3. 상태 변환 
4. 실습

## 0. `bank` 모듈
[`bank` 모듈](https://github.com/cosmos/cosmos-sdk/tree/v0.45.4/x/bank)은 계정 간 다중 자산 코인 전송을 처리하고 특정 종류의 계정에서 다르게 작동해야 하는 특수한 경우의 의사 전송을 추적한다. (특히 vesting account에 대한 위임/위임 취소). 사용자 잔액을 변경해야 하는 다른 모듈과의 안전한 상호 작용을 위해 다양한 기능을 갖춘 여러 인터페이스를 노출한다.

또한 `bank` 모듈은 애플리케이션에 사용된 모든 자산의 총 공급량을 추적하고 쿼리 지원을 제공한다. 이 모듈은 Cosmos Hub에서 사용되고 있다. 

## 1. 상태
1. [Balance](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/proto/cosmos/bank/v1beta1/genesis.proto#L27-L39): 계정의 가지고 있는 토큰 잔액 상태를 관리한다.
2. [Denomination metadata](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/proto/cosmos/bank/v1beta1/bank.proto#L61-L96): 토큰 액면가 정보에 대한 상태를 관리한다. 
3. [Supply](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/proto/cosmos/bank/v1beta1/genesis.proto#L27-L39): 토큰의 총 공급량에 대한 상태를 관리한다.


## 2. 상태 조회
### 1. balance 조회
[`balances` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/bank/client/cli/query.go#L41-L102)을 사용하면 주소별로 account 잔액을 조회할 수 있다.
```sh
simd query bank balances [address] [flags]
```

명령 쿼리 예시는 다음과 같다:
```sh
simd query bank balances cosmos1..
```

### 2. denom-metadata 조회 
[`denom-metadata` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/bank/client/cli/query.go#L104-L156)을 사용하면 해당 토큰의 액면가(denominations)에 대한 메타데이터를 쿼리할 수 있다. 사용자는 `--denom` 플래그를 사용하여 단일 액면가에 대한 메타데이터를 쿼리할 수도 있고, 플래그 없이 모든 액면가에 대한 메타데이터를 쿼리할 수도 있다.
```sh
simd query bank denom-metadata [flags]
```

명령 쿼리 예시는 다음과 같다:
```sh
simd query bank denom-metadata --denom stake
```


### 4. total 조회 
[`total` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/bank/client/cli/query.go#L158-L214)을 사용하면 토큰의 총 공급량을 쿼리할 수 있다. 
- `--denom` 플래그: 이에 해당하는 토큰의 총 공급량을 쿼리할 수도 있고, 플래그 없이 모든 토큰의 공급량을 쿼리할 수도 있다.
```sh
simd query bank total [flags]
```

명령 쿼리 예시는 다음과 같다:
```sh
simd query bank total --denom stake
```

## 3. 상태 변환 (transcation)
### 1. send 
[`send` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/bank/client/cli/tx.go#L28-L60)을 사용하면 한 계정에서 다른 계정으로 자금을 송금할 수 있다.
```sh
simd tx bank send [from_key_or_address] [to_address] [amount] [flags]
```

명령 트랜잭션 예시는 다음과 같다:
```sh
simd tx bank send cosmos1.. cosmos1.. 100stake
```

### 2. multi-send 
해당 명령어는 v0.46.0 이상부터 사용 가능하다. 더 나은 UX 환경을 위해 추가된 명령어로 [`multi-send` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.46.0/x/bank/client/cli/tx.go#L73-L145)을 사용하면 한 계정에서 다른 여러 계정으로 자금을 송금할 수 있다.
```sh
simd tx bank multi-send [from_key_or_address] [to_address_1, to_address_2, ...] [amount]
```

명령 트랜잭션 예시는 다음과 같다:
```sh
simd tx bank multi-send cosmos1.. cosmos1..,cosmos1..,cosmos1.. 100stake
```

## 4. 실습
해당 실습에서는 유저 간의 간단한 토큰을 전송하는 기능을 다뤄본다. 

### 1. simd 시작하기 
> 이전에 `simd`를 사용한 적이 있다면 홈 디렉터리에 이미 `.simapp` 디렉터리가 있을 수 있다. 이전 데이터를 유지하려면 디렉터리를 다른 위치에 저장하거나 `--home` 플래그를 사용하여 각 명령에 대해 다른 디렉터리를 지정해야 한다. 이전 데이터를 유지하지 않으려면 이전 디렉터리를 제거해준다.(`rm -rf ~/.simapp`).

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
$ simd config chain-id bank-demo 
```

keyring-backend 이름을 설정해준다:
```sh
$ simd config keyring-backend test 
```

#### Key 설정하기
validator 역할을 하는 Alice와 각자의 토큰을 보유한 Bob, Charles를 생성해준다: 
```sh
$ simd keys add alice && simd keys add bob && simd keys add charles
```

두 key가 잘 생성되었는지 확인해보자:
```sh
$ simd keys list
```

조회 결과는 다음 샘플과 같다:
```sh
- name: alice
  type: local
  address: cosmos1d2dkkkfm2wkwsmwqezpl36de6gs2ed2tmdx984
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AuB2P0+kIUUMJ35stzpHcYwDfSredsYMiL0fldGGEOzD"}'
  mnemonic: ""
- name: bob
  type: local
  address: cosmos1s2v3q8l4hdpz925sjgjpxjp2ftmp5ftygkz47e
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A1Q9xqXVl5Q79b8tElcIQ+uoyx7wbOrKUXQtVrzO8S6Q"}'
  mnemonic: ""
- name: charles
  type: local
  address: cosmos1d2csszycpwwu50rxnm79yucq0m9mc8mxgsrm64
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AyQhF0KhQM+95B86aLWKieLfk3DkfULpnYBWx/Yt9lEb"}'
  mnemonic: ""
```

#### Chain 시작하기
나머지 기본 설정을 추가해준다:
```sh
$ simd init test --chain-id bank-demo   &&
simd add-genesis-account alice 5000000000stake --keyring-backend test &&
simd add-genesis-account bob 10000kudos --keyring-backend test &&
simd add-genesis-account charles 10000bang --keyring-backend test &&
simd gentx alice 1000000stake --chain-id bank-demo  &&
simd collect-gentxs
```

이제 체인을 시작한다:
```sh
$ simd start
```

쿼리나 트랜잭션 명령어를 입력할 때 사용자 주소를 복사하여 붙여넣지 않으려면 shell에 사용자 키를 액세스하여 사용할 수 있는 변수로 미리 설정하는 것이 좋다. 
```sh
$ export ALICE=$(simd keys show alice --address) && export BOB=$(simd keys show bob --address) && export CHARLES=$(simd keys show charles --address)
```


### 2. 토큰 전송하기
#### balance 확인하기
토큰을 전송하기 전에 우선 각 친구들의 잔액부터 확인해보자:
```sh
simd query bank balances $ALICE && simd query bank balances $BOB && simd query bank balances $CHARLES
``` 

조회 결과는 다음 샘플과 같다:
```sh
# alice
balances:
- amount: "4999000000"
  denom: stake
pagination:
  next_key: null
  total: "0"
# bob
balances:
- amount: "10000"
  denom: kudos
pagination:
  next_key: null
  total: "0"
# charles
balances:
- amount: "10000"
  denom: bang
pagination:
  next_key: null
  total: "0"
```


#### token 전송하기 
`send` 명령어를 사용하여 다음과 같이 각각에게 `send` 트랜잭션 명령을 보낸다: 
```sh
$ simd tx bank send $ALICE $BOB 100stake
$ simd tx bank send $BOB $ALICE 1000kudos
$ simd tx bank send $CHARLES $BOB 500bang
```

조회 결과는 다음 샘플과 같다:
```sh
# alice
balances:
- amount: "1000"
  denom: kudos
- amount: "4998999900"
  denom: stake
pagination:
  next_key: null
  total: "0"
# bob
balances:
- amount: "500"
  denom: bang
- amount: "9000"
  denom: kudos
- amount: "100"
  denom: stake
pagination:
  next_key: null
  total: "0"
# charles
balances:
- amount: "9500"
  denom: bang
pagination:
  next_key: null
  total: "0"
```
- Alice는 Bob에게 `100stake`를 전송하고, Bob은 Alice에게 `1000kudos`를 전송하고, Charles는 Bob에게 `500bang`을 전송한 트랜잭션이 모두 성공적으로 이뤄짐을 확인할 수 있다. 

## Resources
- https://docs.cosmos.network/v0.47/build/modules/bank
