# Create Cosmos-SDK dApp

## Cosmos-SDK Foundation

Cosmos-SDK is a toolkit that can easily create PoS (Proof-Stake)-based app chains, and chains manufactured using Cosmos-SDK are mainly referred to as app chains.

Cosmos-SDK is an SDK designed to utilize Tendermint or CombetBFT for Consensus layers and to only care about Application layers for developers.

```
                  ^  +-------------------------------+  ^
                  |  |                               |  |   Application Layer
                  |  |  State-machine = Application  |  |   Built with Cosmos SDK
                  |  |                               |  v
                  |  +-------------------------------+
                  |  |                               |  ^
Build with Cosmos |  |           Consensus           |  |
                  |  |                               |  |   Consensus Layer
                  |  +-------------------------------+  |   CometBFT
                  |  |                               |  |
                  |  |           Networking          |  |
                  |  |                               |  |
                  v  +-------------------------------+  v
```

### Cosmos-SDK Module

Cosmos-SDK provides the following functions as a basis by providing a basic module (based on v0.50.x)

- x/auth : Features related to the account

- x/authz : Permission can be delegated to another account (Transaction can be sent instead)

- x/bank : Can manage and transfer token status to account.

- x/staking, Distribution : functions related to dPoS (delegation, claim)

- x/governance: It provides the ability to operate governance on its own by the app chain (change state through suggestions, votes, and results)

- x/minint : ability to mint inflation-related tokens

- x/feegrant : Regarding the payment of fees

- x/nft: NFT support with specifications such as ERC721

- In addition, there are modules such as Slashing, Genutil, Circuit, Upgrade, Consensus, Evidence, and Crisis to operate the chain.

Although it is not a basic module, the main modules and libraries below exist.

- x/liquid staking: It can be used by tokenizing staked tokens for the applied app chain.

- x/etherint : Module that enables EVM to be used on the App Chain (SmartContracts)

- x/wasm : A module that supports CosmWasm to be used on the app chain (SmartContracts)

- ibc-go : inter-app-chain tokens, modules that enable data transmission

- ibc-apps : A function that transmits specific data to ibc called packet forward middleware and automatically processes it in the received chain can be implemented.

### Cosmos-SDK Smart Contract

In the early days, Cosmos-SDK did not have Smart Contract. Therefore, in order to implement new functions, a custom module was created rather than a basic provision module to manage the new state and implement functions accordingly.

To reflect the Custom Module

1. Module Development and Release
2. Chain upgrade with governance (Network participants are upgrading)

Proceed with the above procedure. It can be seen that the above procedure is not public due to high barriers to entry such as chain core developers and Contributions. Therefore, Smart Contract develops.

Cosmos-SDK includes Cosmwasm and Etherint as modules to support Smart Contact.

- Cosmwasm https://cosmwasm.com/

- Built on WebAssembly (Wasm)

- Rust-based

- It provides Cosmsos-SDK Standard so that you can access the app chain module when you were programming Wasm.

- It has many functions specialized in the app chain, such as designating permission.

- Cosmwasm's main chain

- Neutron, Juno, Osmosis, Stargaze ...

- Ethermint https://github.com/evmos/ethermint

- Ethereum Virtual Machine(EVM)과 호환

- EVM Smart Contracts used by other chains can be imported and used immediately.

- It mainly utilizes EVM addresses, and providing an RPC for EVM to Metamask can have the same usability as EVM chains.

- The native function of the app chain can also be used based on the precompiled contact.

- Key Chain Utilizing Ethermint

- Kava, Evmos, Canto ...

### Cosmos-SDK status inquiry

Cosmos-SDK provides both REST API and gRPC methods so that status values can be inquired.

- In projects that do not operate actual nodes, public endpoints are found or API services are found and used.

- At the production level, the stability of the project can be given only when API services are used or nodes are operated directly.

- For the mission, we use public endpoints provided through the hook of the cosmos-kit.

Cosmos-SDK provides Message Sign through Amino, Direct, and Text methods. Each method has a different method of serializing (encoding) signature data.

- Amino : Serialization method used in earlier versions of Cosmos SDK, using structured data

- Direct method: Serialized, efficient/optimized format, recommended with Protobuf

- Text method: human readable serialization (JSON or YAML), inefficient than binary serialization in terms of security and performance

| Method   | Serialization Method    | Main Purpose                               |    Pros/Cons                               |
| ------ | ------------------- | --------------------------------------------- | ------------------------------------------ |
| Amino  | Binary/Test     | Used for early Cosmos SDK 사용                          | Substituted to Protobuf, not recommened for the new application |
| Direct | Protobuf (Birnary) | Used for the recent Cosmos SDK                       | Efficient and optimized binary method   |
| Text   | JSON/YAML (Text)  | Debugging, logging, human readable data | Human redable but less efficient        |

## Basic knowledge for dApp development

- dApp is an application (web/mobile web/mobile app) that uses the Blockchain Network (appchain) by real users, and can be viewed as an application that inquires or stores status values in the blockchain. All of the information on the chain, such as mobile wallets, extension wallets, and explorers, can be viewed as dApps.

- For example, sending tokens from the app chain

  ```
      ^  +-------------------------------+  ^
      |  | Blockchain                    |  |
      |  |                               |  |
      |  |   auth                        |  |  Sing / verify an account.
      |  |   bank                        |  |  Manage token sttus in an account.
      |  |                               |  |
      v  +-------------------------------+  v

      ^  +-------------------------------+  ^
      |  | dApp (UI)                     |  |
      |  |                               |  |
      |  |   address                     |  |  Call the adress through connected wallet.
      |  |   balance                     |  |  Query toekn balance on the chain.
      |  |   send                        |  |  Send token sending message to the connected wallet.
      |  |                               |  |
      v  +-------------------------------+  v

      ^  +-------------------------------+  ^
      |  | Wallet                        |  |
      |  |                               |  |
      |  |   account                     |  |  Manage account (Private key, Mnmonic, etc)
      |  |   balance                     |  |  Query token balacne on the chain.
      |  |   sign                        |  |  Sign and send message requested by dApp by using key.
      |  |                               |  |
      v  +-------------------------------+  v
  ```

## Toolkit for developing dApps supported by Cosmos ecosystem

- CosmJS https://github.com/cosmos/cosmjs

- Communicate with the app chain with Cosmos-SDK's representative typescript/javascript library.

- What you do in CosmJS library

- Sign & Broadcast the Message using the mnemonic/private key.

- Request user signatures with the help of wallets such as Cosmostation and Kipl. (OfflineSigner)

- You can query the state information to the blockchain network.

- Receive events that occur on the Cosmos SDK module.

- cosmos-kit https://cosmology.zone/products/cosmos-kit

- Cosmos-kit can easily communicate with app chains using Cosmos-SDK by utilizing Cosmos-SDK and various wallet connections such as Cosmos Ecosystem Cosmostation, Keplr, and Leap.

- Various examples are provided using create-cosmos-app.

- Mintscan https://mintscan.io

- Representative Explorer of the Cosmos Ecosystem

- It supports various chains such as mainnet and testnet, and can see not only information and statistics for each chain, but also Cosmos ecosystem information at once.

- It has a built-in wallet, so you can immediately utilize Cosmos major functions such as x/bank, x/staking, and x/governance.

- Ignite https://docs.ignite.com/

- cli functioning as a scaffold of Cosmos-SDK

- It facilitates chain generation, module generation, relayer, etc.