package main

import (
	"fmt"
	cgo "github.com/avinassh/fluvio-go/fluvio_cgo"
)

func main() {
	fmt.Println(cgo.FlAddition(5, 10))
}