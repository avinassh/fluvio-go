package main

import (
	"fmt"
	"time"

	"github.com/avinassh/fluvio-go/fluvio"
)

func main() {
	f := fluvio.NewFluvio()
	t := f.TopicProducer("hello-go")
	for i := 1; i <= 10; i++ {
	   val := fmt.Sprintf("(from Go) %d (%s)", i, time.Now().String())
	   fmt.Println("Sending: ", val)
	   t.Send(fmt.Sprintf("%d", i), val)
	}
}