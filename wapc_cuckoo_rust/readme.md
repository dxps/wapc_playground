# About This Sample

This is a waPC guest implementation, writtend in Rust and using waPC's guest library for this.

<br/>

## Build

By default, you don't need to run the build since the project already includes it as `build/wapc_cuckoo_rust.wasm` file.

If you change the spec (described in `schema.widl` file) run `make` to:
- generate the `src/generated.rs` file, referred in `src/lib.rs`
- build the WASM module in `target/wasm32-unknown-unknown/release/cuckoo_wapc_rust.wasm`
- have the WASM module copied into `build` folder

<br/>

## Usage

Checkout the waPC host implementations details to see how this WASM module.
