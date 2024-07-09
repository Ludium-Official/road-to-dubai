# 24. `authz` Moudle
## 목차
0. `authz` 모듈
1. 상태
2. 상태 조회
3. 상태 변환 
4. 실습 
    1. simd 실행하기 
    2. Proposal 제출하기 
    3. Grant 부여하기
    4. Transaction 생성하기
    5. Transaction 대신 실행하기 

## 0. `authz` 모듈 
[`authz` 모듈](https://github.com/cosmos/cosmos-sdk/tree/v0.45.4/x/authz)은 한 계정을 대신하여 다른 계정에 작업을 수행할 수 있는 권한을 부여하는 기능을 제공한다. 이 디자인은 [ADR 030](https://docs.cosmos.network/maiarchitecture/adr-030-authz-module)에 정의되어 있다. Cosmos SDK 앱 개발자는 `authz` 모듈을 구현하여 사용자에게 다른 사용자에게 특정 권한을 부여할 수 있는 기능을 제공한다. 예를 들어, 사용자가 다른 사용자가 자신을 대신하여 투표하기를 원할 수 있으므로 다른 사용자에게 자신의 계정에 대한 액세스 권한을 부여하는 대신 다른 사용자가 자신을 대신하여 MsgVote를 실행할 수 있는 권한을 부여할 수 있다. 사용 예시는 다음과 같다: 
- 검증자는 검증자 키를 더 안전하게 유지하기 위해 투표를 위한 별도의 계정을 만들고 싶을 수 있다. 
- DAO에서 개별 계정에 권한을 부여하고 싶을 때 사용하여 다른 구성원의 서명 없이도 메시지를 실행할 수 있다.


## 1. 상태 
1. [Grant](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/proto/cosmos/authz/v1beta1/authz.proto#L13-L39): granter, grantee, authorization type 등 권한 부여에 대한 정보를 다룬다. 
2. [GrantQueue](https://github.com/cosmos/cosmos-sdk/blob/v0.46.0/x/authz/authz.pb.go#L157-L161): 해당 큐는 `v0.46.0` 이상부터 추가되었다. `Grant`가 생성될 때마다 `GrantQueue`에 추가된다. [`BeginBlocker`](https://github.com/cosmos/cosmos-sdk/blob/v0.46.0/x/authz/module/abci.go)에서 `GrantQueue`에 저장된 만료일을 지난 현재 블록 시간으로 접두사 키를 생성하여 만료된 `Grant`를 지속적으로 확인하고, `GrantQueue`에서 일치하는 모든 레코드를 반복하여 `GrantQueue` 및 `Grant`의 저장소에서 삭제한다.

## 2. 상태 조회
### 1. grants 조회 
`grants` 명령을 사용하면 granter-grantee 쌍에 대한 grant 쿼리할 수 있다. 메시지 TypeURL이 설정되어 있으면 해당 메시지 타입에 대한 grant만 쿼리한다.
```sh
simd query authz grants [granter-addr] [grantee-addr] [msg-type-url]? [flags]
```


## 3. 상태 변환
### 1. exec
'exec` 명령을 사용하면 granter가 grantee를 대신하여 트랜잭션을 실행할 수 있다.
```sh
simd tx authz exec [tx-json-file] --from [grantee] [flags]
```

### 2. grant
`grant` 명령을 사용하면 granter가 지정된 사람(grantee)에게 권한을 부여할 수 있다.
```sh
simd tx authz grant <grantee> <authorization_type="send"|"generic"|"delegate"|"unbond"|"redelegate"> --from <granter> [flags]
```

### 3. revoke
`revoke` 명령을 사용하면 granter가 grantee의 권한을 취소할 수 있다.
```sh
simd tx authz revoke [grantee] [msg-type-url] --from=[granter] [flags]
```


## 4. 실습 
해당 실습 Cosmos SDK의 `simapp`을 사용하여 단일 노드 네트워크를 구동하고, 다른 계정에 권한을 부여한 다음, 권한 부여자(granter)를 대신하여 수취인(grantee)으로서 메시지를 실행한다.

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
$ simd config chain-id authz-demo 
```

keyring-backend 이름을 설정해준다:
```sh
$ simd config keyring-backend test 
```

#### Key 설정하기 
granter와 grantee 역할을 맡을 Alice와 Bob을 생성해준다:
```sh
$ simd keys add alice 
$ simd keys add bob 
```

두 key가 잘 생성되었는지 확인해보자:
```sh
$ simd keys list
```

조회 결과는 다음 샘플과 같다:
```sh
- name: alice
  type: local
  address: cosmos123g9hxppa4e50tn25khja3zpeqz49augrznpes
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AvcJre0Gy06nAhyVq3CBWRp4wXZMKVGIhcTTf3IdIIkW"}'
  mnemonic: ""
- name: bob
  type: local
  address: cosmos1ql029f9wez4lw7jr87j05hpjm5c9nqt2y7vrs5
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AktnhrU99tgVmR3xUspWaicbA2TTngCDHlK6JhUCNglQ"}'
  mnemonic: ""
```

#### Chain 설정하기
다음 명령은 `simapp`을 사용하여 체인을 설정한다.

chain ID를 설정해준다:
```sh
$ simd config chain-id authz-demo
```

keyring backend를 설정해준다:
```sh
$ simd config keyring-backend test
```

노드를 초기화한다:
```sh
$ simd init test --chain-id authz-demo
```

genesis 파일에 Alice와 초기 잔액을 추가한다:
```sh
$ simd add-genesis-account alice 5000000000stake --keyring-backend test
```

genesis 파일에 Bob과 초기 잔액을 추가한다:
```sh
$ simd add-genesis-account bob 5000000000stake --keyring-backend test
```

트랜잭션을 생성하여 초기 validator set에 Alice를 추가한다:
```sh
$ simd gentx alice 1000000stake --chain-id authz-demo
```

validator 트랜잭션을 genesis 파일에 추가한다:
```sh
$ simd collect-gentxs
```

이제 체인을 시작한다:
```sh
$ simd start
```

쿼리나 트랜잭션 명령어를 입력할 때 사용자 주소를 복사하여 붙여넣지 않으려면 shell에 사용자 키를 액세스하여 사용할 수 있는 변수로 미리 설정하는 것이 좋다. 
```sh
$ export ALICE=$(simd keys show alice --address)
$ export BOB=$(simd keys show bob --address)
```

### 2. Proposal 제출하기 
거버넌스 제안에 대한 투표 권한을 증명하려면 먼저 거버넌스 제안을 만들어야 한다. 다음 명령은 거버넌스 제안서가 즉시 투표 기간에 들어갈 수 있도록 최소 예치금이 포함된 텍스트 제안서를 만든다.
 > 명령 및 플래그 옵션에 대한 자세한 내용을 보려면 `simd tx gov submit-proposal --help`

#### proposal 생성하기
```sh
$ simd tx gov submit-proposal --title="Test Authorization" --description="Is Bob authorized to vote?" --type="Text" --deposit="10000000stake" --from alice
```

####  proposal 조회하기
```sh
$ simd query gov proposal 1
```

조회 결과는 다음 샘플과 같다:
```sh 
content:
  '@type': /cosmos.gov.v1beta1.TextProposal
  description: Is Bob authorized to vote?
  title: Test Authorization
deposit_end_time: "2024-07-04T08:19:50.144423Z"
final_tally_result:
  abstain: "0"
  "no": "0"
  no_with_veto: "0"
  "yes": "0"
proposal_id: "1"
status: PROPOSAL_STATUS_VOTING_PERIOD
submit_time: "2024-07-02T08:19:50.144423Z"
total_deposit:
- amount: "10000000"
  denom: stake
voting_end_time: "2024-07-04T08:19:50.144423Z"
voting_start_time: "2024-07-02T08:19:50.144423Z"
```

### 3. Grant 부여하기
다음으로, granter은 grantee에게 권한을 부여해야 한다. 
- 여기서 authorization 타입은 "generic"으로, MsgVote와 같은 메시지 타입을 매개변수로 사용하여 grantee가 granter를 대신하여 해당 메시지를 실행할 수 있는 무제한 권한을 주는 것을 말한다. 
- 다른 authorization 타입으로는 "send", "delegate", "unbond", "redelegate"가 있으며, 이 경우 토큰 수에 대한 제한을 granter가 설정할 수 있다. 
- 물론, granter는 이전에 준 권한을 취소할 수 있다. grantee는 그 전까지는 마음대로 권한을 사용할 수 있다. 

#### authorization 생성하기
```sh
$ simd tx authz grant $BOB generic --msg-type /cosmos.gov.v1beta1.MsgVote --from alice
```


#### authorization 조회하기
다음 쿼리를 통해 Alice가 Bob에게 준 권한 목록을 조회해본다:
```sh
$ simd query authz grants $ALICE $BOB /cosmos.gov.v1beta1.MsgVote
```

조회 결과는 다음 샘플과 같다:
```sh
grants:
- authorization:
    '@type': /cosmos.authz.v1beta1.GenericAuthorization
    msg: /cosmos.gov.v1beta1.MsgVote
  expiration: "2025-07-02T08:06:24Z"
pagination: null
```

이제부터 **granter = Alice, grantee = Bob**이 되었다.

### 4. Transaction 생성하기
Bob이 Alice 대신 메시지를 실행하려면 먼저 Alice가 서명되지 않은 트랜잭션을 생성해야 한다.

#### unsigned transaction 생성하기
Alice가 gov 제안에 동의하는 트랜잭션을 생성한다. 
```sh
$ simd tx gov vote 1 yes --from $ALICE --generate-only > tx.json
```

#### transaction 조회하기
```sh
$ cat tx.json
```

조회 결과는 다음 샘플과 같다:
```json
{
    "body": {
        "messages": [
            {
                "@type": "/cosmos.gov.v1beta1.MsgVote",
                "proposal_id": "1",
                "voter": "cosmos1jxd2uhx0j6e59306jq3jfqs7rhs7cnhvey4lqh",
                "option": "VOTE_OPTION_YES"
            }
        ],
        "memo": "",
        "timeout_height": "0",
        "extension_options": [],
        "non_critical_extension_options": []
    },
    "auth_info": {
        "signer_infos": [],
        "fee": {
            "amount": [],
            "gas_limit": "200000",
            "payer": "",
            "granter": ""
        }
    },
    "signatures": []
}
```


### 5. Transaction 대신 실행하기 
Bob은 트랜잭션 `execute` 명령을 사용하여 Alice가 미리 작성해둔 트랜잭션에 서명하고 전송할 수 있다. 트랜잭션의 작성자(`--from` address)는 Bob(grantee)으로 설정해야 한다. 

#### Transaction 서명 및 실행하기 
```sh
$ simd tx authz exec tx.json --from bob
```
```json
{
    "body": {
        "messages": [
            {
                "@type": "/cosmos.authz.v1beta1.MsgExec",
                "grantee": "cosmos1khljzagdncfs03x5g6rf9qp5p93z9qgc3w5dwt",
                "msgs": [
                    {
                        "@type": "/cosmos.gov.v1beta1.MsgVote",
                        "proposal_id": "1",
                        "voter": "cosmos1jxd2uhx0j6e59306jq3jfqs7rhs7cnhvey4lqh",
                        "option": "VOTE_OPTION_YES"
                    }
                ]
            }
        ],
        "memo": "",
        "timeout_height": "0",
        "extension_options": [],
        "non_critical_extension_options": []
    },
    "auth_info": {
        "signer_infos": [],
        "fee": {
            "amount": [],
            "gas_limit": "200000",
            "payer": "",
            "granter": ""
        }
    },
    "signatures": []
}
```

#### vote 조회하기 
```sh
$ simd query gov vote 1 $ALICE
```

조회 결과는 다음 샘플과 같다:
```sh
option: VOTE_OPTION_YES
options:
- option: VOTE_OPTION_YES
  weight: "1.000000000000000000"
proposal_id: "1"
voter: cosmos1jxd2uhx0j6e59306jq3jfqs7rhs7cnhvey4lqh
```

### 6. authorization 취소하기 
Alice(granter)는 Bob(grantee)에게 이미 부여한 권한을 취소할 수 있다 

#### authorization 취소하기 
```sh
$ simd tx authz revoke $BOB /cosmos.gov.v1beta1.MsgVote --from alice
```

#### authorization 조회하기
```sh
$ simd query authz grants $ALICE $BOB /cosmos.gov.v1beta1.MsgVote
```

다음과 같이 조회할 수 없는 에러가 나와야 정상적으로 authorization이 취소된 것이다:
```sh
Error: rpc error: code = NotFound desc = rpc error: code = NotFound desc = no authorization found for /cosmos.gov.v1beta1.MsgVote type: key not found
```


## Resources
- https://docs.cosmos.network/maimodules/authz
- https://docs.cosmos.network/maiarchitecture/adr-030-authz-module
- https://tutorials.cosmos.network/tutorials/8-understand-sdk-modules/1-authz.html