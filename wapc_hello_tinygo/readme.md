# About This Sample

This is a waPC Guest (WASM Module) implemented in Go.

## Dev Journey

### Prereqs

Besides the Go setup, two things need to be installed:
- [wapc CLI](https://wapc.io)
- [TinyGo](https://tinygo.org/)
  - At the time of this writing, TinyGo ver. 0.23 supports Go 1.18+.

- This project was created using `wapc new tinygo wapc_hello_tinygo`<br>
- Fetching dependencies using `cd wapc_hello_tinygo/ && go mod tidy`
- Added a bare minimal implementation to the `sayHello` function in `main.go`

<br/>

## Build

Just use `make` and the result can be found in `./buid/wapc_hello_tinygo.wasm`.<br>

<br/>

## Usage

This can be used - same as the other waPC Guest(s) - like this:

- Considering the Go based waPC Host:<br>
  - `cd wapc_host_go`
  - `go run main.go ../wapc_hello_tinygo/build/wapc_hello_tinygo.wasm sayHello '{"name":"John"}'`<br/>
    and it should print `Hello John` to the standard output.
