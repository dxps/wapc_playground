# waPC Playground

A playground for (study and experiments) using [waPC](https://wapc.io/).

It includes:
- Three waPC Hosts
  - These exist in the `wapc_host_...` folders.
  - Written in Go, Node.js, and Rust (3 out of 4 supported host implementations).
- Two waPC Guests
  - Written in Rust ([wapc_cuckoo_rust](./wapc_cuckoo_rust/readme.md)) and Go ([wapc_hello_tinygo](./wapc_hello_tinygo/readme.md)).
  - Implemented as WASM modules.
  - Using a provided wapc guest library for their respective programming languages.
  - Their builds ([this](../wapc_playground/wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm) and [this](./wapc_hello_tinygo/build/wapc_hello_tinygo.wasm)) are included in this repository.

All the waPC Host implementations includes a WASM runtime. Through it, these implementations can invoke the functions that the waPC Guests' are exposing.

Read their `readme.md` files for to know how to use them.
