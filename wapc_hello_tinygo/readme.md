# About This Sample

This is a waPC Guest (WASM Module) implemented in Go.

Creation steps:
- This project was simply created using `wapc new tinygo wapc_hello_tinygo`.<br>
  Note that at the time of this writing, TinyGo doesn't support yet Go ver 1.18+.
- `cd wapc_hello_tinygo/ && go mod tidy`
- Added a bare minimal implementation to the `sayHello` function in `main.go`

<br/>

## Build

Just use `make` and the result can be found in `./buid/wapc_hello_tinygo.wasm`.<br>
Again, while writing this Go ver. `1.17.9` was used, as TinyGo doesn't yet support Go 1.18.

<br/>

## Usage

This can be used - same as the other waPC Guest(s) - like this:

- Considering the Go based waPC Host:<br>
  - `cd wapc_host_go`
  - `go run main.go ../wapc_hello_tinygo/build/wapc_hello_tinygo.wasm sayHello '{"name":"John"}'`<br/>
    and it should print `Hello John` to the standard output.
