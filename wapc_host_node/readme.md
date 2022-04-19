# About This Sample

This sample plays the waPC Host, a host based on Node.js.
It uses the `check_word_exists` function that is exported in a WASM module, built out of [wapc_cuckoo_rust](../wapc_cuckoo_rust/readme.md) sibling sample.

## Usage

Considering that the WASM module is already built (see the details [here](../wapc_cuckoo_rust/readme.md)), you can run it as follows:

- `node wapc.js ../wapc_cuckoo_rust/target/wasm32-unknown-unknown/release/wapc_cuckoo_rust.wasm check_word_exists '{"name":"testme"}'`
  and it should return `Result:false`
- `node wapc.js ../wapc_cuckoo_rust/target/wasm32-unknown-unknown/release/wapc_cuckoo_rust.wasm check_word_exists '{"name":"foo"}'`
  and it should return `Result:true`

