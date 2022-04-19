package main

import (
	"context"
	"fmt"
	"io/ioutil"
	"os"

	json2msgpack "github.com/izinin/json2msgpack"
	"github.com/vmihailenco/msgpack"

	"github.com/wapc/wapc-go"
	"github.com/wapc/wapc-go/engines/wasmer"
)

func main() {

	wasm := os.Args[1]
	funcname := os.Args[2]
	name := os.Args[3]

	ctx := context.Background()
	code, err := ioutil.ReadFile(wasm)
	if err != nil {
		panic(err)
	}

	engine := wasmer.Engine()

	module, err := engine.New(code, nil)
	if err != nil {
		panic(err)
	}
	module.SetLogger(wapc.Println)
	module.SetWriter(wapc.Print)
	defer module.Close()

	instance, err := module.Instantiate()
	if err != nil {
		panic(err)
	}
	defer instance.Close()

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
