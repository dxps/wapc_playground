# About This Sample

This is a waPC host implementation, written in Go.<br/>

<br/>

## Usage

Considering that the WASM module is already built (see the details [here for Rust sample](../wapc_cuckoo_rust/readme.md) and [here for Go sample](../wapc_hello_tinygo/readme.md)), you can run it as follows:

- `go run main.go ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm check_word_exists '{"name":"foo"}'`<br/>
  It should print `true` to the standard output.
  <br/>

- `go run main.go ../wapc_hello_tinygo/build/wapc_hello_tinygo.wasm sayHello '{"name":"John"}'`<br/>
  It should print `Hello John` to the standard output.

<br/>
