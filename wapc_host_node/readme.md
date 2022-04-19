# About This Sample

This sample plays the waPC Host, a host based on Node.js.<br/>
Basically, it uses waPC as the WASM runtime for using `wapc_cuckoo_rust.wasm` (built out of [wapc_cuckoo_rust](../wapc_cuckoo_rust/readme.md) sibling sample) WASM module, to invoke the functions defined with it.

## Usage

Considering that the WASM module is already built (see the details [here](../wapc_cuckoo_rust/readme.md)), you can run it as follows:

- for invoking the `check_word_exists` function:
  - `node wapc.js ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm check_word_exists '{"name":"testme"}'`
    and it should return `Result: false`
  - `node wapc.js ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm check_word_exists '{"name":"foo"}'`
    and it should return `Result: true`

- for invoking the `handle_input` function:
  - `node wapc.js ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm handle_input '{"inp": { "x":"foo", "y":"bar" }}'`
  and it should return `Result: foo`
