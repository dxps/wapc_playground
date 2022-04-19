# About This Sample

This is a waPC Host (WASM Runtime) done in Rust.<br/>
Alternatives would be to have it done in Go or Node.js. The Node.js version exists in [wapc_host_node](../wapc_host_node) sibling project.

<br/>

## Build

Use the standard `cargo build` to build it. The generated result is `./target/debug/wapc_host_rust`.

<br/>

## Usage

`./target/debug/wapc_host_rust ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm name foo`

<br/>
