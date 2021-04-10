package main

/*
#cgo LDFLAGS: -L./src -lfluvio_go
#include "./src/fluvio_go.h"
#include <stdlib.h>
*/
import "C"
import "fmt"

func main() {
	fluvio := C.fluvio_connect()
	tp := C.fluvio_topic_producer(fluvio, C.CString("chinni"))
	for i := 1; i <= 10; i++ {
        val := fmt.Sprintf("from kall chinni: %d", i+250)
        fmt.Println("Sending: ", val)
        C.topic_producer_send(tp, C.CString(fmt.Sprintf("%d", i)), C.CString(val))
    }
}