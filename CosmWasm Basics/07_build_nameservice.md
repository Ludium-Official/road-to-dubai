# Build a Name Service Contract

## 0. Nameservice
Nameservice is a smart contract that registers, queries, and manages names on the blockchain. This document describes the process of building a nameservice contract with Rust.

## 1. Pre-installation
### Installing Rust
First, install Rust. If it's already installed, it doesn't matter if it's omitted. The download command is as follows:
```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Make sure that the rust is installed properly:
```sh
$ rustup --version
# rustup 1.27.1 (54dd3d00f 2024-04-24)

$ cargo --version
# cargo 1.78.0 (54d8815d0 2024-03-26)

$ rustc --version
# rustc 1.78.0 (9b00956e5 2024-04-29)
```

### Install rust wasm
Add Rust WASM Target for WASM build:
```sh
$ rustup target add wasm32-unknown-unknown
```

### Install coswasm-check
Install Cosmwasm check tools:
```sh
$ cargo install cosmwasm-check
```


## 2. Build Projct
```sh
$ cargo new  --lib ./nameservice
$ cd nameservice
```

Before building the project, the folder structure should be as follows:
```sh
src  
├── contract.rs  
├── error.rs  
├── helpers.rs  
├── lib.rs  
├── msg.rs  
├── tests.rs  
└── state.rs
```

State modules on `lib.rs` in the following way. 
```rust
pub mod helpers;
pub mod contract;
mod error;
pub mod msg;
pub mod state;

#[cfg(test)]
mod tests;
```
- `msg.rs `: Defines the type of message that communicates with the outside world.
- `contract.rs `: Define the business logic of EntryPoint and its function.
- `error.rs`: Define custom error types.
- `state.rs`: defines the contract internal state.
- `tests.rs`: defines the overall unit test.
- `helpers.rs`: defines the additional functions required to execute business logic, such as checking the token balance of the user.

## 3. Build a project
### Build wasm artifacts
To build a dynamic library in Cargo, you need to modify the `Cargo.toml` file. Add a `lib` section that sets the appropriate crate type to `cdylib` for the dynamic library:
```rust
[package]
name = "counting-contract"
version = "0.1.0"
edition = "2021"
 
[lib]
crate-type = ["cdylib", "rlib"]
```
- In addition to `cdylib`, we added a `rlib` crate type. It instructs cargo to build two types of outputs, and `rlib` is a standard static Rust library. I don't need it right now, but it helps if I have it, so I leave it here for now.

Now I'm ready to build WASM output by calling a slightly modified build command:
```sh
$ cargo build --target wasm32-unknown-unknown

# /target/wasm32-unknown-unknown/debug/nameservice.wasm
# /target/wasm32-unknown-unknown/debug/deps/nameservice.wasm
```

### Setting up alias
At this point, wasm binaries have been prepared, but there is a simple way to make the build command simpler. Often, this is done by creating the following `.cargo/config` file on smart contract projects:
```
[alias]
wasm = "build --release --target wasm32-unknown-unknown"
```
- Basically, when you build a wasm binary to be distributed, you build it using `--release` for optimization.

Now let's see if the alias command works properly:
```sh
$ cargo wasm

# /target/wasm32-unknown-unknown/release/nameservice.wasm
# /target/wasm32-unknown-unknown/release/deps/nameservice.wasm
# /target/wasm32-unknown-unknown/debug/nameservice.wasm
# /target/wasm32-unknown-unknown/debug/deps/nameservice.wasm
```

Let's check the wasm binary file that was built through the `cosm wasm-check` command that was previously installed: 
```sh
$ cosmwasm-check ./target/wasm32-unknown-unknown/release/nameservice.wasm

Available capabilities: {"cosmwasm_1_1", "iterator", "cosmwasm_1_4", "cosmwasm_2_0", "staking", "stargate", "cosmwasm_1_3", "cosmwasm_1_2"}

./target/wasm32-unknown-unknown/release/nameservice.wasm: failure
Error during static Wasm validation: Wasm contract missing a required marker export: interface_version_*

Passes: 0, failures: 1
```

### Adding cosmwasm-std libraries
You can see that something is happening. This is because `entry_point` was not created in the cosmwasm contract. To do this, you must first add the `cosmwasm-std` dependency to the project. Assuming that the trust version is 1.62 or higher (or if the cargo-edit utility is installed manually), you can use cargo add:
> If the rust version is 1.78, the 1.5.4+ version must be installed
```sh
$ cargo add cosmwasm-std@1.5.4
```

You can then see that the library has been added to the dependencies item as follows:
```toml
[package]
name = "nameservice"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
cosmwasm-std = "1.5.4"
```

This is because there is none and `instantiate` `entry_point` is needed to work. The reason `cosmwasm-check` complains about some version markers is that the same macro creates a marker as an entry point, so adding a macro can solve this problem. 


## Resources
- https://github.com/deus-labs/cw-contracts/tree/main/contracts/nameservice