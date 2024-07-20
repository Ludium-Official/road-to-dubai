# 23. Message and Event
## 목차
0. Messages
1. CosmosMsg
2. SubMessages
3. Event

## 0. Messages
메시지는 CosmWasm 스마트 컨트랙트와 상호작용하는 방법이다. 대부분의 컨트랙트에는 메시지를 정의하는 [`msg.rs`](./nameservice/src/msg.rs) 파일이 있다. 세 가지 주요 메시지 유형이 있다:
- 인스턴스화 메시지 (`InstantiateMsg`): 이는 컨트랙트를 초기화할 때 전송된다. 일반적으로 컨트랙트를 올바르게 초기화하는 데 필요한 데이터를 포함하고 있습니다. 이는 대부분의 경우 단순한 구조로 이루어져 있다.
- 실행 메시지 (`ExecuteMsg`) 및 쿼리 메시지 (`QueryMsg`): 둘 다 열거형(enum) 메시지이다. 이들은 각각 실행 및 쿼리에 사용되는 메시지 유형을 나타낸다. 

### 메시지 정의 예시
다음은 nameservice 컨트랙의 간단한 [`execute` 메시지 정의 예시](./nameservice/src/msg.rs)이다:
```rust
#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String },
    Transfer { name: String, to: String },
}
```
- `Register`: 이름을 등록하는 메시지
- `Transfer`: 이름을 다른 주소로 전송하는 메시지

#### `cw_serde`
[`cw_serde`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/schema-derive/src/cw_serde.rs)는 CosmWasm 스마트 컨트랙트 개발에서 주로 사용되는 매크로이다. 이는 Rust의 `serde` 라이브러리를 기반으로 하며, JSON 형식으로 데이터를 인코딩하고 디코딩하는 데 사용된다. 이 매크로를 사용하면 메시지와 데이터 구조를 간편하게 처리할 수 있다. 주요 기능은 다음과 같다: 
- 자동 직렬화 및 역직렬화: `cw_serde` 매크로는 `serde` 라이브러리의 `Serialize` 및 `Deserialize` 트레잇을 자동으로 구현한다. 이를 통해 데이터를 JSON 형식으로 손쉽게 변환할 수 있다.
- 스키마 생성: `schemars` 라이브러리를 사용하여 JSON 스키마를 자동으로 생성한다. 이는 메시지와 데이터 구조의 예상 형식과 타입을 정의하는 데 유용하다.

[nameservice 컨트랙트의 스키마 폴더](./nameservice/schema/)를 보면, `cw_serde`로 생성한 스키마 파일을 볼 수 있다.

### 메시지 핸들링
이 메시지들은 [contract.rs 파일의 execute 함수](./nameservice/src/contract.rs)에서 다음과 같이 처리된다:
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::Register { name } => execute_register(deps, env, info, name),
    ExecuteMsg::Transfer { name, to } => execute_transfer(deps, env, info, name, to),
  }
}
```
- 위의 `execute` 함수는 `ExecuteMsg` 열거형의 각 변형을 매치하여 해당하는 함수를 호출한다. 예를 들어, `ExecuteMsg::Register` 메시지가 수신되면 `execute_register` 함수가 호출된다.

## 1. CosmosMsg
[`CosmosMsg`](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/results/cosmos_msg.rs)는 다양한 메시지 유형을 포함하는 열거형이다. 이는 다른 컨트랙트와 상호작용하거나 블록체인 내에서 특정 작업을 수행하는 데 사용된다.
```rust
pub enum CosmosMsg<T = Empty> {
    Bank(BankMsg),
    Custom(T),
    Staking(StakingMsg),
    Distribution(DistributionMsg),
    Stargate {
        type_url: String,
        value: Binary,
    },
    Any(AnyMsg),
    Ibc(IbcMsg),
    Wasm(WasmMsg),
    Gov(GovMsg),
}
```

### 1. WasmMsg
[WasmMsg](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/results/cosmos_msg.rs#L180-L270)는 CosmosMsg의 한 종류로, 다른 스마트 컨트랙트와의 상호작용을 위한 메시지를 포함한다. 여기에는 다음과 같은 종류들이 있다:
```rust
pub enum WasmMsg {
    Execute {
        contract_addr: String,
        msg: Binary,
        funds: Vec<Coin>,
    },
    Instantiate {
        admin: Option<String>,
        code_id: u64,
        msg: Binary,
        funds: Vec<Coin>,
        label: String,
    },
    Migrate {
        contract_addr: String,
        new_code_id: u64,
        msg: Binary,
    },
    UpdateAdmin {
        contract_addr: String,
        admin: String,
    },
    ClearAdmin { contract_addr: String },
}
```
- `Execute`: 다른 컨트랙트를 실행한다
- `Instantiate`: 새로운 컨트랙트를 인스턴스화한다
- `Migrate`: 기존 컨트랙트를 마이그레이션한다
- `UpdateAdmin`: 컨트랙트의 관리자를 업데이트한다
- `ClearAdmin`: 컨트랙트의 관리자를 제거한다

### 2. BankMsg
`BankMsg`는 네이티브 토큰을 다른 컨트랙트로 전송하거나 소각할 수 있는 메시지를 포함한다. 이는 블록체인 자체에서 처리되므로 진정한 스마트 컨트랙트는 아니지만, 네이티브 토큰을 처리하는 데 유용하다.

### 3. Custom
Custom은 특정 블록체인에서 처리하는 메시지를 추가하기 위해 존재한다. 이는 블록체인에 특화된 메시지 타입을 정의할 수 있게 해준다. 예를 들어, 다른 CosmWasm 기반 블록체인에서만 사용되는 메시지를 추가할 수 있다.


## 2. SubMessages
Submessages는 SDK 모듈이나 다른 Cosmwasm 스마트 컨트랙트와 상호작용할 때 사용된다. `CosmosMsg`는 호출이 성공했는지 여부를 알 수 없지만, SubMessage를 사용하면 호출 결과를 얻을 수 있다. 이는 다음과 같은 경우에 유용하다:
- 새로운 컨트랙트를 인스턴스화하고 컨트랙트 주소를 얻을 때
- 액션을 실행하고 결과가 성공적인지 확인할 때 (예: 특정 토큰이 컨트랙트에 전송되었는지 확인)
- 교차 컨트랙트 호출에서 발생한 오류를 처리하여 트랜잭션을 롤백하지 않을 때

Submessage를 사용하려면 CosmosMsg를 SubMsg 구조체로 감싸서 전송해야 한다. SubMsg 구조체는 다음과 같은 필드를 가지고 있다:
```rust
pub struct SubMsg<T> {
    pub id: u64,                // 응답을 처리할 때 사용되는 reply_id
    pub msg: CosmosMsg<T>,      // 전송할 메시지
    pub gas_limit: Option<u64>, // 서브메시지의 가스 한도
    pub reply_on: ReplyOn,      // 언제 응답을 받을지 결정하는 플래그
}
```

재진입 공격을 방지하기 위해, CosmWasm은 컨트랙트 메모리에 컨텍스트를 저장하는 것을 허용하지 않는다. 컨트랙트 간에 상태를 전파하는 방법은 두 가지가 있다:
1. SubMsg에서 반환된 모든 이벤트는 Reply 메시지에서 읽을 수 있다.
2. [`cw_storage_plus::Item`](./21_state.md#1-item)을 사용하여 임시 상태를 저장하고 Reply 핸들러에서 이를 로드한다.

### 1. Reply 전략
Submessage는 다른 컨트랙트가 응답을 제공하는 다양한 옵션을 제공합니다. 네 가지 Reply 옵션이 있다:
```rust
pub enum ReplyOn {
    /// SubMsg가 처리된 후 항상 콜백 수행
    Always,
    /// SubMsg가 오류를 반환한 경우에만 콜백 수행, 성공 시에는 콜백 없음
    Error,
    /// SubMsg가 성공한 경우에만 콜백 수행, 오류 시에는 콜백 없음
    Success,
    /// 콜백 수행 안 함 - 이는 원래의 CosmosMsg 의미와 같음
    Never,
}
```
- 

### 2. Reply 핸들러 
다른 컨트랙트로부터의 응답을 처리하려면 호출하는 컨트랙트에서 새로운 [entry_point](./22_entrypoint.md) 함수 중 하나인 `reply 함수`를 구현해야 한다. 다음은 새로운 컨트랙트 인스턴스화 처리하는 [Reply 예제 코드](https://github.com/CosmWasm/cw-plus/blob/main/packages/utils/src/parse_reply.rs)이다:
```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // 메시지 데이터를 처리하고 컨트랙트 주소를 저장
    let res = parse_reply_instantiate_data(msg)?;

    // res.contract_address 저장
    Ok(Response::new())
}
```

## 3. Event
대부분의 `entry_point` 함수는 `Result<Response, ContractError>` 유형을 반환한다. 여기서 [`Response` 객체](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/results/response.rs#L63-L87)는 Cosmos SDK에서 이벤트를 래핑하는 역할을 한다. 다음은 해당 객체의 구성 요소이다:
```rust
pub struct Response<T = Empty> {
    pub submessages: Vec<SubMsg<T>>,
    pub messages: Vec<CosmosMsg<T>>,
    pub attributes: Vec<Attribute>,
    pub data: Option<Binary>,
}
```

`Response` 타입은 컨트랙트 `entry_point`(예: `instantiate` 또는 `execute`)의 메시지가 성공될 경우 결과로 반환된다. 함수 본문에서 이를 변경할 수 있도록 mutable로 선언할 수 있지만, 더 일반적인 패턴은 함수 끝에서 이를 구성하고 반환하는 것이다. 아래 예제에서, `Response`는 `Result` 타입의 일부로 반환되므로 `Ok`로 감싸져 있다. 예외는 query로, Cosmos SDK 인터페이스 때문에 `StdResult<Binary>`를 반환한다.

Response의 가장 단순한 사용 예시는 다음과 같다:
```rust
Ok(Response::default())
```

이는 `instantiate` 함수에서 클라이언트에 메시지를 반환하지 않을 때 일반적으로 사용된다. 그러나 대부분의 `execute` 처리 사례에서는 `Response`가 반환된다:
```rust
let res = Response::new()
    .add_attribute("action", "transfer")
    .add_attribute("from", info.sender)
    .add_attribute("to", recipient)
    .add_attribute("amount", amount);
Ok(res)
```
- `Response`를 생성하고 여러 key-value 쌍을 추가한 뒤, 이를 Result 타입으로 감싸서 반환한다. CLI를 통해 컨트랙트를 호출하면 raw_log 응답의 일부로 이를 확인할 수 있다.
- `attirbute`을 추가하는 대신 `.add_event`를 사용하여 래핑되지 않은 이벤트를 추가할 수도 있다. 이러한 이벤트는 다른 클라이언트나 컨트랙트와 상호작용할 수 있다.


## Resources
- https://docs.cosmwasm.com/docs/smart-contracts/message/submessage
