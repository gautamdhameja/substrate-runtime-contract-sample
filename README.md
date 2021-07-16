> **This sample is not compatible with the latest release of Substrate because the APIs have evolved quite a bit. The repo is being archived for the same reason. Please do not use this sample. 
> The underlying APIs in Substrate have been replaced by chain extensions and contract-to-runtime interaction is possible with that - https://github.com/paritytech/substrate/pull/9276**

# Substrate Runtime-Contract Sample

A sample Substrate runtime showing interaction between runtime modules (pallets) and smart contracts (ink!).

## Runtime to Contract Interaction

In the template module (pallet) in the runtime, following funtions are used for contract interaction:

* call_contract
* get_contract_storage

## Contract to Runtime Interaction

~~In the custom_type contract, the `read_custom_runtime` funtion is used to query the runtime storage for a custom struct.~~

## Custom types for Polkadot JS

```json
{
  "Foo": {
    "id": "u32",
    "data": "Vec<u8>"
  }
}
```

## Generating Storage Keys

Keys to storage items in substrate runtime are constructed by hashing the raw key.

Refer to the [substrate documentation](https://crates.parity.io/frame_support/macro.decl_storage.html#example) about storage types and key construction.

## Build Node and Runtime

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Initialize your Wasm Build environment:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build --release
```

## Build Contract

Install Cargo Contract:

```bash
cargo install --force --git https://github.com/paritytech/cargo-contract
```

Inside the `contracts/custom_type` sub-directory, run the following commands to build the contract and generate its metadata:

```bash
cargo +nightly contract build
```

```bash
cargo +nightly contract generate-metadata
```

## Run

Start a development chain with:

```bash
./target/release/substrate-runtime-contract-sample --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.

## Deploy the contract

Once the Substrate node is running, [deploy and instantiate the contract](https://substrate.dev/substrate-contracts-workshop/#/0/deploying-your-contract) using the [Polkadot JS portal](https://polkadot.js.org/apps/).
