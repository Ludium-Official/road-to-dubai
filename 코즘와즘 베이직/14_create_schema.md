# 31d. Schema 생성하기 

## 0. schema
컨트랙트는 외부 메시지를 통해 상호작용하는 액터 모델이다. schema를 통해 쉽게 메시지를 주고 받을 수 있게끔 명세서를 작성해주는 것이 좋다. 

## 1. alias 추가하기
이전에 alias를 추가한 곳에 `schema`에 대한 alias도 추가해보자:
```
[alias]
wasm = "build --release --target wasm32-unknown-unknown"
schema = "run --example schema" // 추가!
```

## 2. schmea 생성하기
이전에 msg를 구현할 때 `cosmwasm_schema` 라이브러리를 추가했었다. 이는 인코딩 및 디코딩을 도와줄뿐더러 이름 그대로 스키마 생성도 쉽게 할 수 있도록 지원해주고 있다. `examples/schema.rs`에 다음과 같은 코드를 추가해보자:
```rust
use cosmwasm_schema::write_api;
use nameservice::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
```

`cargo schema` 명령어를 입력하면 다음과 같이 `json` 형식의 파일들이 생성된 것을 확인할 수 있다:
```sh
$ cargo schema

Exported the full API as /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/nameservice.json
Exported /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/raw/instantiate.json
Exported /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/raw/execute.json
Exported /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/raw/query.json
Exported /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/raw/response_to_config.json
Exported /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/raw/response_to_resolve_record.json
```

이 중에서 `nameservice.json`은 모든 정보를 포함하고 있는 schema이다. 예시로 `execute`에 대한 명세서 일부를 가져와보면 다음과 같다: 
```json
"execute": {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "ExecuteMsg",
    "oneOf": [
      {
        "type": "object",
        "required": [
          "register"
        ],
        "properties": {
          "register": {
            "type": "object",
            "required": [
              "name"
            ],
            "properties": {
              "name": {
                "type": "string"
              }
            },
            "additionalProperties": false
          }
        },
        "additionalProperties": false
      },
    // ...
```

이는 register에 대한 execute 메세지로 String 타입의 name 값을 필수로 입력해줘야 한다고 나와있다. 이는 우리가 구현한 그대로 잘 보여주고 있는 것을 확인할 수 있다:
```rust
pub enum ExecuteMsg {
    Register { name: String },
}
```

## 마무리 
이렇게 간단하게 schema를 생성해보았다. 해당 스키마는 neutron 테스트넷에 배포할 때 사용된다. 다음에는 직접 배포해보고 트랜잭션과 쿼리를 통해 컨트랙트와 상호작용하는 방법에 대해서 알아본다. 