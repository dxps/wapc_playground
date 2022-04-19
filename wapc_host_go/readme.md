# About This Sample

This is a waPC host implementation, written in Go.<br/>

<br/>

## Usage

As prerequisites, have [wasmer](https://wasmer.io/) installed (for example, on Linux/Ubuntu use `curl https://get.wasmer.io -sSfL | sh`).

Considering that the WASM module is already built (see the details [here](../wapc_cuckoo_rust/readme.md)), you can run it as follows:

`go run main.go ../wapc_cuckoo_rust/build/wapc_cuckoo_rust.wasm check_word_exists '{"name":"foo"}'` and it 

It should print `true` to the standard output.

<br/>
