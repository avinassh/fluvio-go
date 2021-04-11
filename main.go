package main

import (
	"fmt"
	"log"
	"time"

	"github.com/avinassh/fluvio-go/fluvio"
)

func fatalIfErr(str string, err error) {
	if err != nil {
		fmt.Println(str, err)
		log.Fatal()
	}
}

func main() {
	f, err := fluvio.NewFluvio()
	fatalIfErr("error while connecting", err)
	t, err := f.TopicProducer("hello-go")
	fatalIfErr("error while getting producer", err)
	for i := 1; i <= 10; i++ {
		val := fmt.Sprintf("(from Go) %d (%s)", i, time.Now().String())
		fmt.Println("Sending: ", val)
		err = t.Send(fmt.Sprintf("%d", i), val)
		fatalIfErr("error while sending", err)
		break
	}
}
