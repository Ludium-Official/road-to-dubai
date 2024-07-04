# 05. Understand simapp architecture 2
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
