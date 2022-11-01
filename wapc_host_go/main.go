package main

import (
	"context"
	"fmt"
	"github.com/wapc/wapc-go/engines/wazero"
	"os"

	json2msgpack "github.com/izinin/json2msgpack"
	"github.com/vmihailenco/msgpack"

	"github.com/wapc/wapc-go"
)

func main() {

	wasm := os.Args[1]
	funcname := os.Args[2]
	name := os.Args[3]

	ctx := context.Background()
	guest, err := os.ReadFile(wasm)
	if err != nil {
		panic(err)
	}

	engine := wazero.Engine()

	module, err := engine.New(ctx, nil, guest, &wapc.ModuleConfig{
		Logger: wapc.PrintlnLogger,
		Stdout: os.Stdout,
		Stderr: os.Stderr,
	})
	defer module.Close(ctx)

	instance, err := module.Instantiate(ctx)
	if err != nil {
		panic(err)
	}
	defer instance.Close(ctx)

	b := json2msgpack.EncodeJSON([]byte(name))
	res, err := instance.Invoke(ctx, funcname, b)
	if err != nil {
		panic(err)
	}

	var s string
	if err := msgpack.Unmarshal(res, &s); err != nil {
		panic(err)
	}

	fmt.Println(s)

}
