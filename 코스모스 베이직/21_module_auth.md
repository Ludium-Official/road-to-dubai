# 21. `auth` Moudle
## 목차
0. `auth` 모듈
1. 상태
2. 상태 조회
3. 상태 변환 
4. 실습
    1. simd 실행하기
    2. 트랜잭션 생성 및 서명하기
    3. 트랜잭션 브로드캐스팅하기

## 0. `auth` 모듈
[`auth` 모듈](https://github.com/cosmos/cosmos-sdk/tree/v0.45.4/x/auth)은 Cosmos Hub에서도 사용하는 가장 기본적인 모듈이다. 기본 트랜잭션과 계정 타입을 지정하는 역할을 담당한다. 여기에는 모든 기본 트랜잭션 유효성 검사(서명, 논스, 보조 필드)가 수행되는 미들웨어(anteHandler)가 포함되어 있으며, 다른 모듈이 계정을 읽고, 쓰고, 수정할 수 있도록 계정 키퍼를 노출합니다.

`auth` 모듈은 `authz`와는 다르다. 차이점은 다음과 같다:
- `auth`: Cosmos SDK 앱에 대한 account 및 트랜잭션 인증이며 기본 트랜잭션 및 account 타입을 지정하는 기능을 제공한다. 
- `authz`: account가 다른 account을 대신하여 작업을 수행할 수 있는 권한 부여로, granter가 grantee에게 권한을 부여하여 grantee가 granter를 대신하여 메시지를 실행할 수 있도록 허용한다.

## 1. 상태
1. [accounts](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/proto/cosmos/auth/v1beta1/auth.proto#L10-L25): 계정에는 pubKey, address, 리플레이 보호를 위한 account number/sequence number 등 고유하게 식별되는 외부 사용자의 인증 정보가 포함되어 있다. 
2. [vesting account](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/proto/cosmos/vesting/v1beta1/vesting.proto#L10-L33): 이는 [v0.51 이후 `x/accounts`를 위해 deprcated](https://docs.cosmos.network/main/build/modules/auth/vesting) 된다.

## 2. 상태 조회
### 1. account 조회
[`account` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/client/cli/query.go#L82-L112)을 사용하면 주소로 account을 쿼리할 수 있다.
```sh
simd query auth account [address] [flags]
```

### 2. account 모두 조회하기 
[`accounts` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/client/cli/query.go#L114-L144)을 사용하면 사용 가능한 모든 account을 쿼리할 수 있습니다.
```sh
simd query auth accounts [flags]
```

### 3. params 조회하기 
[`params` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/client/cli/query.go#L51-L80)을 사용하면 현재 `auth` 매개변수를 쿼리할 수 있다.
```sh
simd query auth params [flags]
```
> 해당 자료는 모든 조회 요청을 나타내지 않는다. grpc, rest 등 더 자세한 내용은 해당 모듈 [README](https://github.com/cosmos/cosmos-sdk/tree/main/x/auth#client)를 참고하자.

## 3. 상태 변환 
### 1. sign
[`sign` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/client/cli/tx_sign.go#L158-L192)을 사용하면 오프라인에서 생성된 트랜잭션에 서명할 수 있다.
```sh
simd tx sign [file]
```

명령 트랜잭션 예시는 다음과 같다:
```sh
simd tx sign tx.json --from $ALICE > tx.signed.json
```

### 2. sign-batch
[`sign-batch` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/client/cli/tx_sign.go#L24-L58)을 사용하면 오프라인에서 생성된 여러 트랜잭션에 서명할 수 있다. 트랜잭션은 한 줄당 하나의 tx가 있는 하나의 파일 또는 여러 파일에 포함될 수 있다.
```sh
simd tx sign-batch [file]
```

명령 트랜잭션 예시는 다음과 같다:
```sh
simd tx sign-batch txs.json --from $ALICE > tx.signed.json
# or 
simd tx sign-batch tx1.json tx2.json tx3.json --from $ALICE > tx.signed.json
```

### 3. multi-sign
[`multi-sign` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/client/cli/tx_multisign.go#L33-L71)을 사용하면 다중 서명 계정에서 오프라인으로 생성된 트랜잭션에 서명할 수 있다.
```sh
simd tx multisign [file] [name] [[signature]...]
```

명령 트랜잭션 예시는 다음과 같다:
```sh
simd tx multisign transaction.json k1k2k3 k1sig.json k2sig.json k3sig.json
```

### 4. multi-sign-batch
[`multi-sign-batch` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/client/cli/tx_multisign.go#L202-L234)는 `sign-batch`의 다중 서명 계정 버전이다. 단, `multi-sign-batch` 명령은 모든 트랜잭션이 하나의 파일에 있어야 하고 `--append` 플래그가 존재하지 않는다는 차이점이 있다.
```sh
simd multisign-batch [file] [name] [[signature-file]...]
```

### 5. validate-signatures
[`validate-signatures` 명령](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/client/cli/validate_sigs.go#L16-L56)을 사용하면 서명된 트랜잭션의 서명을 확인할 수 있다.
```sh
simd tx validate-signatures [file]
```

명령 트랜잭션 예시는 다음과 같다:
```sh
simd tx validate-signatures tx.signed.json

# Signers:
#  0: cosmos1l6vsqhh7rnwsyr2kyz3jjg3qduaz8gwgyl8275
#
# Signatures:
#   0: cosmos1l6vsqhh7rnwsyr2kyz3jjg3qduaz8gwgyl8275        
```   

### 6. broadcast
`broadcast` 명령을 사용하면 서명된 트랜잭션을 네트워크에 브로드캐스트할 수 있다.

명령 트랜잭션 예시는 다음과 같다:
```sh
simd tx broadcast tx.signed.json
```

## 4. 실습 
### 4-1. simd 실행하기
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
$ simd config chain-id auth-demo 
```

keyring-backend 이름을 설정해준다:
```sh
$ simd config keyring-backend test 
```

#### Key 설정하기
validator 역할을 하는 Alice와 그 친구 Bob을 생성해준다: 
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
  address: cosmos1jyuue3asacd6temmncd8kt9wxqamjypqgsdtyd
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A91xVaCh42lfDYUokCxygfwyrRS5ceb+oj1OL1p2S7Xn"}'
  mnemonic: ""
- name: bob
  type: local
  address: cosmos19jd7dgxha4uy4t0y5ytck3mc27672m83wy5r4k
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AtcJwjRRbOxWMZvBx5Uey98kMbmkncGBOIyruq+EQoxo"}'
  mnemonic: ""
```

#### Chain 시작하기
나머지 기본 설정을 추가해준다:
```sh
$ simd init test --chain-id auth-demo   &&
simd add-genesis-account alice 1000500stake --keyring-backend test &&
simd gentx alice 1000000stake --chain-id auth-demo  &&
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

### 4-2. 트랜잭션 생성 및 서명하기
### 트랜잭션 생성하기
Alice가 Bob에게 `100stake`를 보내는 트랜잭션([tx.Tx](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/tx.pb.go#L31-L42))을 `tx.json` 파일로 직접 만든다: 
- 트랜잭션 구조는 message 배열을 담는 [body](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/tx.pb.go#L247-L272)와 서명 정보를 담는 [auth_info](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/tx.pb.go#L342-L355)로 나눠진다. 
```sh
echo '{
    "body": {
        "messages": [
        {
            "@type": "/cosmos.bank.v1beta1.MsgSend",
            "from_address": "'"$ALICE"'",
            "to_address": "'"$BOB"'",
            "amount": [
                {
                    "denom": "stake",
                    "amount": "100"
                }
            ]
        }
        ],
        "memo": ""
    },
    "auth_info": {
        "signer_infos": [],
        "fee": {
            "amount": [
                {
                    "denom": "stake",
                    "amount": "1"
                }
            ],
            "gas_limit": "200000",
            "payer": "",
            "granter": ""
        }
    },
    "signatures": []
}' > tx.json
```
> `simd tx bank send $ALICE $BOB 100stake --generate-only > tx.json` 이런 식으로 생성해도 된다.


트랜잭션 파일이 잘 생성되었는지 확인해본다:
```sh
$ cat tx.json
```

조회 결과는 다음 샘플과 같다:
```json
{
    "body": {
        "messages": [
        {
            "@type": "/cosmos.bank.v1beta1.MsgSend",
            "from_address": "cosmos1jyuue3asacd6temmncd8kt9wxqamjypqgsdtyd",
            "to_address": "cosmos19jd7dgxha4uy4t0y5ytck3mc27672m83wy5r4k",
            "amount": [
                {
                    "denom": "stake",
                    "amount": "100"
                }
            ]
        }
        ],
        "memo": ""
    },
    "auth_info": {
        "signer_infos": [],
        "fee": {
            "amount": [
                {
                    "denom": "stake",
                    "amount": "1"
                }
            ],
            "gas_limit": "200000",
            "payer": "",
            "granter": ""
        }
    },
    "signatures": []
}
```

### 트랜잭션 서명하기 
이제 `sign` 명령어를 사용하여 `tx.json`에 $Alice의 서명 정보를 추가한다. 
```sh
simd tx sign tx.json --keyring-backend test --chain-id auth-demo --from $ALICE > tx.signed.json
```
> 만약 서명 정보가 일치하지 않는 경우 [`Error: tx intended signer does not match the given signer`](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/errors/errors.go#L90-L91) 에러가 발생한다.


해당 트랜잭션을 브로드캐스팅하기 전에 잔액을 먼저 확인한다:
```sh
simd query bank balances $ALICE && simd query bank balances $BOB
```

조회 결과는 다음 샘플과 같다:
```sh
# Alice
- amount: "500"
  denom: stake
pagination:
  next_key: null
  total: "0"

# Bob
balances: []
pagination:
  next_key: null
  total: "0"
```

### 4-3. 트랜잭션 브로드캐스팅하기
```sh
simd tx broadcast tx.signed.json
```


트랜잭션이 브로드캐스팅 된 후 상태가 제대로 변경되었는지 확인해본다:
```sh
simd query bank balances $ALICE && simd query bank balances $BOB
```

조회 결과는 다음 샘플과 같다:
```sh
# Alice
balances:
- amount: "399"
  denom: stake
pagination:
  next_key: null
  total: "0"

# Bob
balances:
- amount: "100"
  denom: stake
pagination:
  next_key: null
  total: "0"
```
- Alice의 잔액이 수수료를 포함해서 `101stake` 차감되고, Bob의 잔액이 `100stake` 추가 된 것을 확인할 수 있다. 


## Resources
- https://docs.cosmos.network/v0.47/build/modules/auth