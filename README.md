# Substrate Runtime-Contract Sample

A sample Substrate runtime showing interaction between runtime modules (pallets) and smart contracts (ink!).

## Runtime to Contract Interaction

In the template module (pallet) in the runtime, following funtions are used for contract interaction:

* call_contract
* get_contract_storage

## Contract to Runtime Interaction

In the custom_type contract, the `read_custom_runtime` funtion is used to query the runtime storage for a custom struct.

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

## Build

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

## Run

Start a development chain with:

```bash
./target/release/substrate-runtime-contract-sample --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.
