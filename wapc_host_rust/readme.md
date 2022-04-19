# About This Sample

This is a waPC host implementation, written in Rust.<br/>

<br/>

## Build

Use the standard `cargo build` to build it. The generated result is `./target/debug/wapc_host_rust`.

<br/>

## Usage

Considering that the WASM module is already built (see the details [here](../wapc_cuckoo_rust/readme.md)), you can run it as follows:

`./target/debug/wapc_host_rust ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm name foo`

It should print `response:"true"` to the standard output.

<br/>
