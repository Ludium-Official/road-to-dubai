# 05. Understand simapp architecture 2

<!-- 우린 이제 쉽게 가동시켜본 simulation appchain의 내부 코드 구조에 대해서 공부해볼거야

이미 한번 띄워본 체인에 대해서 공부를 하는거니까 좀 더 몰입해서 할 수 있을 . 것같아!

아래 구조는 다음 과정에서도 또 할건데 .. 간단히 설명 -->

```bash
.
├── README.md
├── app.go
├── app_test.go
├── config.go
├── encoding.go
├── export.go
├── genesis.go
├── genesis_account.go
├── genesis_account_test.go
├── helpers
│   └── test_helpers.go
├── params
│   ├── amino.go
│   ├── doc.go
│   ├── encoding.go
│   ├── params.go
│   ├── proto.go
│   └── weights.go
├── sim_bench_test.go
├── sim_test.go
├── simd
│   ├── cmd
│   │   ├── cmd_test.go
│   │   ├── genaccounts.go
│   │   ├── genaccounts_test.go
│   │   ├── root.go
│   │   ├── testnet.go
│   │   └── testnet_test.go
│   └── main.go
├── state.go
├── test_helpers.go
├── types.go
├── utils.go
└── utils_test.go
```

그 중에 app.go란 파일에 있는 아래 코드를 보면 우리가 제네시스에서 사용했던 각 모듈들이 app.go라고 하는 app chain의 기본 구조체에서 참조하고 있다는 걸 알 수 있어

```go
...
  "github.com/cosmos/cosmos-sdk/x/auth"
  "github.com/cosmos/cosmos-sdk/x/auth/ante"
  authrest "github.com/cosmos/cosmos-sdk/x/auth/client/rest"
  authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
  authsims "github.com/cosmos/cosmos-sdk/x/auth/simulation"
  authtx "github.com/cosmos/cosmos-sdk/x/auth/tx"
  authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
  "github.com/cosmos/cosmos-sdk/x/auth/vesting"
  vestingtypes "github.com/cosmos/cosmos-sdk/x/auth/vesting/types"
  "github.com/cosmos/cosmos-sdk/x/authz"
  authzkeeper "github.com/cosmos/cosmos-sdk/x/authz/keeper"
  authzmodule "github.com/cosmos/cosmos-sdk/x/authz/module"
  "github.com/cosmos/cosmos-sdk/x/bank"
  bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
  banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
  "github.com/cosmos/cosmos-sdk/x/capability"
  capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
  capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
...
```

The modules in the /cosmos-sdk/x/ folder are maintained by several organisations working on the Interchain Stack. To understand a module, the best way is to have a look at the respective spec folder. For example, look at the cosmos-sdk/x/bank/spec/01_state.md (opens new window)to understand the state of the bank module which you used in this section.

이렇게 하면 제대로 안돌아감! 왜냐하면 어카운트 키퍼가 제일 먼저있지 않아서

<!-- ![alt text](image-19.png) -->
