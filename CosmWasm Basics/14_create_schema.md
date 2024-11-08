# Creating Schema

## 0. schema
Contract is an actor model that interacts through external messages. It is recommended to fill out a specification so that messages can be easily exchanged through schema.

## 1. Add alias
Let's also add an alias for `schema` where we've added alias before:
```
[alias]
wasm = "build --release --target wasm32-unknown-unknown"
schema = "run --example schema" // 추가!
```

## 2. Creating a schmea
Previously, when implementing msg, the `cosmwasm_schema` library was added. It not only helps encoding and decoding, but also makes it easier to create schema as its name implies. Add the following code to `examples/schema.rs`:
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

If you enter the `cargo scheme` command, you can see that files in the form of `json` have been created as follows:
```sh
$ cargo schema

Exported the full API as /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/nameservice.json
Exported /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/raw/instantiate.json
Exported /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/raw/execute.json
Exported /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/raw/query.json
Exported /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/raw/response_to_config.json
Exported /Users/ijong-won/Desktop/ludium/road-to-dubai/코즘와즘 러스트/nameservice/schema/raw/response_to_resolve_record.json
```

Among them, `nameservice.json` is a schema containing all information. For example, some of the specification for `execute` is as follows: 
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

This is an execute message to register and it says that the name value of the String type must be entered. It can be seen that it shows as well as we implemented it:
```rust
pub enum ExecuteMsg {
    Register { name: String },
}
```

## Wrap it up
I tried to create a schema simply like this. The schema is used when deployed to the netron test net. Next, we will try to distribute it directly and find out how to interact with the contract through transactions and queries. 