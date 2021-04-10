package fluvio

/*
#cgo LDFLAGS: -L../src -lfluvio_go
#include "../src/fluvio_go.h"
#include <stdlib.h>
*/
import "C"
import (
	"unsafe"
)

type Fluvio struct {
	wrapper *C.FluvioWrapper
}

type TopicProducer struct {
	wrapper *C.TopicProducerWrapper
}

func NewFluvio() *Fluvio {
	return &Fluvio{
		wrapper: C.fluvio_connect(),
	}
}

func (f *Fluvio) TopicProducer(topic string) *TopicProducer {
	topicPtr := C.CString(topic)
	defer C.free(unsafe.Pointer(topicPtr))
	return &TopicProducer{
		wrapper: C.fluvio_topic_producer(f.wrapper, topicPtr),
	}
}

func (t *TopicProducer) Send(key, value string) {
	keyPtr := C.CString(key)
	defer C.free(unsafe.Pointer(keyPtr))
	valuePtr := C.CString(value)
	defer C.free(unsafe.Pointer(valuePtr))
	C.topic_producer_send(t.wrapper, keyPtr, valuePtr)
}

