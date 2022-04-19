# waPC Playground

A playground for (study & experiments) using [waPC](https://wapc.io/).

It includes:
- Three different version of waPC host
  - These are the `wapc_host_...` samples
  - Written in Go, Node.js, and Rust
    (3 out of 4 supported host implementations)
- One waPC guest
  - Implemented as a WASM module
  - Using a provided Rust guest library

All the waPC host implementations includes a WAS runtime that uses `wapc_cuckoo_rust.wasm` (built out of [wapc_cuckoo_rust](../wapc_cuckoo_rust/readme.md) WASM module, and invoke the functions defined within.
