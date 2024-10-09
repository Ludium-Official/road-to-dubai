# 33b. transfer 트랜잭션 실행하기
## 목차
0. transfer 트랜잭션 실행하기
1. `ResolveRecord` 쿼리 조회하기 
2. transfer 트랜잭션 실행 결과 제출하기 

## 0. transfer 트랜잭션 실행하기
내가 소유한 name을 다른 사람에게 전송할 수 있다. 이번에는 `Execute`탭에서 `transfer`를 선택한 다음 name을 입력하여 메세지를 작성해보자:
```json
{
  "transfer": {
    "name": "hello",
    "to": "neutron1zppaxke5e4l35jwf3ft8th7pyxfyaqschavvqp"
  }
}
```

여기서도 마찬가지로 초기화할 때 설정한 `transfer_price`에 맞게 최소 금액을 설정해줘야 한다. 전체적인 작성 내용은 다음과 같다:
![](./assets/33b_contract_transfer_execute.png)

이를 실행하고 나면 [성공한 트랜잭션](https://neutron.celat.one/pion-1/txs/BEE3C97CCBE9895FCAA925CC182E5D2889FF84BB19D2EB1DC52ADF431EF9152F)이 발행된다. 그래도 해당 데이터가 제대로 저장되었는지 확인하기 위해서 `ResolveRecord` 쿼리 조회를 해보도록 하자.


## 1. `ResolveRecord` 쿼리 조회하기 
다음과 같이 `ResolveRecord` 쿼리 메시지 형식에 맞게 name을 입력해준다:
```json
{
  "resolve_record": {
    "name": "hello"
  }
}
```

다음 쿼리 결과를 통해 정상적으로 변경이 된 것을 확인할 수 있다:
```json
{
  "data": {
    "address": "neutron1zppaxke5e4l35jwf3ft8th7pyxfyaqschavvqp"
  }
}
```

![](./assets/33b_contract_resolverecord_query.png)

## 2. transfer 트랜잭션 실행 결과 제출하기
다음과 같이 transfer 트랜잭션을 직접 실행한 결과를 제출해야 한다:
- transaction hash: [BEE3C97CCBE9895FCAA925CC182E5D2889FF84BB19D2EB1DC52ADF431EF9152F](https://neutron.celat.one/pion-1/txs/BEE3C97CCBE9895FCAA925CC182E5D2889FF84BB19D2EB1DC52ADF431EF9152F)