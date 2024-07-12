# 23. `feegrant` 모듈
## 목차
0. `feegrant` 모듈
1. 상태
2. 상태 조회
3. 상태 변환 
4. 실습 
    1. simd 실행하기 
    2. allowance 권한 부여하기
    3. token 전송하기 (수수료 대행)
    4. allowance 권한 취소하기 

## 0. `feegrant` 모듈
web3의 접근성이 어려운 이유 중 하나는 수수료 지불이다. 수수료 비용이 없는 경우에는 투표권 하나 조차 얻기 힘들다. 이러한 불편함을 해소하고자 [ADR 029](https://docs.cosmos.network/maiarchitecture/adr-029-fee-grant-module)에서 [`feegrant` 모듈](https://github.com/cosmos/cosmos-sdk/tree/v0.47.0/x/feegrant)을 제안하였다. `feegrant` 모듈을 사용하면 granter(사용자, 컨트랙트 또는 모듈)가 grantee의 트랜잭션을 블록체인에 브로드캐스트하고자 할 때 수수료를 지불할 수 있고, granter는 토큰에 대한 모든 권한을 보유하며 언제든지 권한을 취소할 수 있다. 

이후에 다뤄볼 `authz`는 다른 사람의 권한을 얻어 트랜잭션을 대신 서명하여 실행해주는 모듈이라면, `feegrant`는 토큰에 대한 권한을 얻어 남의 토큰을 대신 사용할 수 있게 해준다. `feegrant` 모듈의 사용 사례로 자주 논의되는 것은 신규 사용자가 블록체인 또는 스마트 컨트랙트와 상호작용을 시작하기 전에 토큰을 획득할 필요가 없기 때문에 온보딩 경험이 개선된다는 점이다. 두 가지 `FeeAllowance` 타입이 수수료 지급 모듈로 구현된다:
- [`BasicAllowance`](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/proto/cosmos/feegrant/v1beta1/feegrant.proto#L14-L27): grantee는 granter의 계정에서 수수료를 사용한다. 이는 일회성 제한, 만료 또는 제한이 없을 수 있다.
- [`PeriodicAllowance`](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/proto/cosmos/feegrant/v1beta1/feegrant.proto#L29-L54): grantee는 granter의 계정에서 수수료를 사용한다. 이는 주기적으로 한도가 재설정된다.

## 1. 상태
1. [FeeAllowance](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/proto/cosmos/feegrant/v1beta1/feegrant.proto): grantee(수수료 수당 수취인의 계정 주소)와 granter(수수료 수당 부여자의 계정 주소)의 조합을 통해 식별한다.
2. [FeeAllowanceQueue](https://github.com/cosmos/cosmos-sdk/blob/v0.46.0/x/feegrant/migrations/v046/keys.go): 해당 큐는 `v0.46.0` 이상부터 추가되었다. 이는 [`Endblocker`](https://github.com/cosmos/cosmos-sdk/blob/v0.47.0/x/feegrant/module/abci.go)에서 만료된 `FeeAllowance`가 있는지 큐 상태를 확인하고 만료된 경우 이를 제거한다. 

## 2. 상태 조회
### 1. grant 조회하기 
grant 명령을 사용하면 주어진 granter-grantee 쌍에 대한 grant 정보를 쿼리할 수 있다. 
```sh
simd query feegrant grant [granter] [grantee] [flags]
```

### 2. grant 모두 조회하기 
#### Query
`grants`을 사용하면 지정된 grantee에 대한 모든 grant 정보를 쿼리할 수 있다.
```sh
simd query feegrant grants [grantee] [flags]
```

## 3. 상태 변환 
### 1. grant 
`grant` 명령을 통해 사용자는 다른 계정에게 수수료 권한(fee allowance) 부여할 수 있다. 이는 만료일(`--period`), 총 지출 한도(`--spend-limit`) 및/또는 정기적 지출 한도(`--period-limit`)에 대해서 설정할 수 있다. 
```sh
simd tx feegrant grant [granter] [grantee] [flags]
```

총 지출 한도를 포함한 트랜잭션 명령은 다음과 같다:
```sh
simd tx feegrant grant cosmos1.. cosmos1.. --spend-limit 100stake
```

정기적 지출 한도를 포함한 트랜잭션 명령은 다음과 같다:
```sh
simd tx feegrant grant cosmos1.. cosmos1.. --period 3600 --period-limit 10stake
```

### 2. revoke
`revoke` 명령을 통해 사용자는 이미 부여한 수수료 권한(fee allowance)을 취소할 수 있다.
```sh
simd tx feegrant revoke [granter] [grantee] [flags]
```


## 4. 실습 
해당 실습에서는 블록체인에 수수료로 사용할 `stake`라는 기본 토큰과 친구에게 보낼 `kudos`라는 또 다른 토큰을 설정한다.
- 이번에 Alice는 검증자 역할을 하고, Bob은 수수료로 지불할 stake 토큰이 전혀 없더라도 Bob이 Alice에게 `kudos` 토큰을 보낼 수 있는 Baisc 어워드를 받는 수취인(grantee)이 된다.
- Alice는 Bob에게 기본 수당을 지급하는 수여자(granter)가 된다.

### 1. simd 실행하기 
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
$ simd config chain-id feegrant-demo 
```

keyring-backend 이름을 설정해준다:
```sh
$ simd config keyring-backend test 
```

#### Key 설정하기
validator 역할을 하는 Alice와 grantee 역할을 하는 Bob을 생성해준다: 
```sh
$ simd keys add alice && simd keys add bob
```

두 key가 잘 생성되었는지 확인해보자:
```sh
$ simd keys list
```

조회 결과는 다음 샘플과 같다:
```sh
- name: alice
  type: local
  address: cosmos1p66k9e7r0l9ws2utqccm8pa5f2gh2dadr4ax7a
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AqgWlJJn74PZB14uYmCqafvEhBiC3LyJssb5VnmdeWxQ"}'
  mnemonic: ""
- name: bob
  type: local
  address: cosmos1d73pz3m25guayfx7m80g0npsrkw59aec6cl786
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A0qzyyKdJ8jPfJ0BIbaXbaFvsQExYjfx8x2b36RvftcI"}'
  mnemonic: ""
```

#### Chain 시작하기
나머지 기본 설정을 추가해준다:
```sh
$ simd init test --chain-id feegrant-demo  &&
simd add-genesis-account alice 5000000000stake --keyring-backend test &&
simd add-genesis-account bob 2000kudos --keyring-backend test &&
simd gentx alice 1000000stake --chain-id feegrant-demo &&
simd collect-gentxs
```

이제 체인을 시작한다:
```sh
$ simd start
```

쿼리나 트랜잭션 명령어를 입력할 때 사용자 주소를 복사하여 붙여넣지 않으려면 shell에 사용자 키를 액세스하여 사용할 수 있는 변수로 미리 설정하는 것이 좋다. 
```sh
$ export ALICE=$(simd keys show alice --address) && export BOB=$(simd keys show bob --address)
```

### 2. allowance 권한 부여하기
Bob이 Alice에게 `kudis`을 보내려면 먼저 트랜잭션에서 발생할 수 있는 가스 수수료를 Alice가 지불할 수 있도록 Bob에 대한 allowance을 줘야 한다.

#### feegrant 권한 부여하기
`BasicAllowance`은 수취인이 지출 한도(`spend_limit`) 또는 만료(`expiration`)에 도달할 때까지 수수료를 사용할 수 있는 권한이다. 지출 한도가 `100000stake`이고 만료일이 없는 allowance를 준다.
```sh
$ simd tx feegrant grant $ALICE $BOB --spend-limit 100000stake
```

#### allowance 확인하기
```sh
$ simd query feegrant grants $BOB
```

조회 결과는 다음 샘플과 같다:
```sh
allowances:
- allowance:
    '@type': /cosmos.feegrant.v1beta1.BasicAllowance
    expiration: null
    spend_limit:
    - amount: "100000"
      denom: stake
  grantee: cosmos1d73pz3m25guayfx7m80g0npsrkw59aec6cl786
  granter: cosmos1p66k9e7r0l9ws2utqccm8pa5f2gh2dadr4ax7a
pagination:
  next_key: null
  total: "0"
```
- granter는 $ALICE, grantee 주소는 $BOB 임을 확인할 수 있다.

### 3. token 전송하기 (수수료 대행)
#### balance 조회하기
먼저 Alice와 Bob의 잔액을 확인한다. 초기 잔액을 확인하면 나중에 거래가 성공했는지 확인할 수 있는 기준이 된다:
```sh
$ simd query bank balances $ALICE
```
```sh
balances:
- amount: "4999000000"
  denom: stake
pagination:
  next_key: null
  total: "0"
```

```sh
$ simd query bank balances $BOB
```
```sh
balances:
- amount: "2000"
  denom: kudos
pagination:
  next_key: null
  total: "0"
```

#### token 전송하기
tx 명령을 사용하여 전송되는 모든 트랜잭션은 `--fee-account` 플래그를 사용하여 수수료를 지불할 계정을 입력으로 지정할 수 있다. 다음과 같이 Bob이 Alice에게 `kudos` 토큰을 보내고 Alice가 수수료를 지불한다:
```sh
$ simd tx bank send $BOB $ALICE 100kudos --from bob --fee-account $ALICE --fees 500stake
```

#### 다시 balance 조회하기
```sh
$ simd query bank balances $ALICE
```

조회 결과는 다음 샘플과 같다:
```sh
balances:
- amount: "100"
  denom: kudos
- amount: "4998999500"
  denom: stake
pagination:
  next_key: null
  total: "0"
```

Bob이 서명한 트랜잭션으로 인해 Alice의 `500stake`가 줄어든 것을 확인할 수 있다. 그 대신 Bob이 전송한 `100kudos`가 추가되었다. 

```sh
$ simd query bank balances $BOB
```

조회 결과는 다음 샘플과 같다:
```sh
balances:
- amount: "1900"
  denom: kudos
pagination:
  next_key: null
  total: "0"
```
Bob은 Alice에게 보냈기 때문에 `100kudos`가 줄어들었다.

#### allowance 조회하기
```sh
$ simd query feegrant grants $BOB
```

조회 결과는 다음 샘플과 같다:
```sh
allowances:
- allowance:
    '@type': /cosmos.feegrant.v1beta1.BasicAllowance
    expiration: null
    spend_limit:
    - amount: "99500"
      denom: stake
  grantee: cosmos19wugtkh265h7uzqqnh0qj2k02dwszvkdk5p8q6
  granter: cosmos10vd27ql8uu0ut0jumnyyylunlzqd7f3zxelhlt
pagination:
  next_key: null
  total: "0"
```
allowance 양이 `500stake` 줄어들었다. 


### 4. allowance 권한 취소하기 
권한을 부여한 사람(granter)은 `revoke` 명령을 사용하여 수취인(grantee)에게 준 allowance를 취소할 수 있다.

#### allowance 취소하기
```sh
$ simd tx feegrant revoke $ALICE $BOB --from alice
```

#### allowance 조회하기
Bob의 allowance을 확인해보자:
```sh
$ simd query feegrant grants $BOB
```

조회 결과는 다음 샘플과 같다:
```sh
allowances: []
pagination:
  next_key: null
  total: "0"
```
allowances 목록이 비어있는 것을 확인할 수 있다. 

## Resources
- https://docs.cosmos.network/main/build/modules/feegrant
- https://docs.cosmos.network/maiarchitecture/adr-029-fee-grant-module