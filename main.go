package main

/*
#cgo LDFLAGS: -L./target/debug -lfluvio_go
#include "./src/fluvio_go.h"
#include <stdlib.h>
*/
import "C"
import "fmt"

func main() {
	fluvio := C.fluvio_connect()
	tp := C.fluvio_topic_producer(fluvio)
	for i := 1; i <= 10; i++ {
        val := fmt.Sprintf("from kall chinni: %d", i)
        fmt.Println("Sending: ", val)
        C.fluvio_topic_producer_send(tp, C.CString(val))
    }
}