# waPC Playground

A playground for (study & experiments) using [waPC](https://wapc.io/).

It includes:
- Three waPC Hosts
  - These are the `wapc_host_...` samples
  - Written in Go, Node.js, and Rust
    (3 out of 4 supported host implementations)
- Two waPC Guests
  - Written in Rust ([wapc_cuckoo_rust](./wapc_cuckoo_rust/readme.md)) and Go ([wapc_hello_tinygo](./wapc_hello_tinygo/readme.md))
  - Implemented as WASM modules
  - Using a provided wapc guest library for the respective programming language

All the waPC Host implementations includes a WASM runtime. Through it, these that uses `wapc_cuckoo_rust.wasm` (built out of [wapc_cuckoo_rust](../wapc_cuckoo_rust/readme.md) WASM module, and invoke the functions defined within.

Read their `readme.md` for to know how to use them.
