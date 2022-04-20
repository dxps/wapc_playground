package main

import (
	"dxps.io/wapc_hello_tinygo/pkg/module"
)

func main() {
	module.Handlers{
		SayHello: sayHello,
	}.Register()
}

func sayHello(name string) (string, error) {
	return "Hello " + name, nil
}
