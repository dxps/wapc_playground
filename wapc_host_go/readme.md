# About This Sample

This is a waPC Host (WASM Runtime) done in Go.<br/>

<br/>

## Usage

As prerequisites, have [wasmer](https://wasmer.io/) installed (for example, on Linux/Ubuntu use `curl https://get.wasmer.io -sSfL | sh`).

Run `go run main.go ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm check_word_exists '{"name":"foo"}'` and it should return `true`.

<br/>
