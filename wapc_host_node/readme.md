# About This Sample

This is a waPC host implementation, written in, written in JavaScript and using Node.js.<br/>

## Usage

Considering that the WASM module is already built (see the details [here](../wapc_cuckoo_rust/readme.md)), you can run it as follows:

- for invoking the `check_word_exists` function:
  - `node wapc.js ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm check_word_exists '{"name":"testme"}'`
    and it should print `Result: false` to the standard output.
  - `node wapc.js ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm check_word_exists '{"name":"foo"}'`
    and it should print `Result: true` to the standard output

- for invoking the `handle_input` function:
  - `node wapc.js ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm handle_input '{"inp": { "x":"foo", "y":"bar" }}'`
  and it should print `Result: foo` to the standard output.
