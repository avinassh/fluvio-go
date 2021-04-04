package main

import (
	"fmt"
	cgo "github.com/avinassh/fluvio-go/fluvio_cgo"
)

func main() {
	gg := cgo.Connect()
	fmt.Println(gg)
}