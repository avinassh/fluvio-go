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

func NewFluvio() (*Fluvio, error) {
	errPtr := C.fluvio_error_new()
	defer C.fluvio_error_free(errPtr)
	f := C.fluvio_connect(errPtr)
	if errPtr.msg != nil {
		return nil, NewFluvioError(C.GoString(errPtr.msg))
	}
	return &Fluvio{
		wrapper: f,
	}, nil
}

func (f *Fluvio) TopicProducer(topic string) (*TopicProducer, error) {
	topicPtr := C.CString(topic)
	defer C.free(unsafe.Pointer(topicPtr))
	errPtr := C.fluvio_error_new()
	defer C.fluvio_error_free(errPtr)
	t := C.fluvio_topic_producer(f.wrapper, topicPtr, errPtr)
	if errPtr.msg != nil {
		return nil, NewFluvioError(C.GoString(errPtr.msg))
	}
	return &TopicProducer{
		wrapper: t,
	}, nil
}

func (t *TopicProducer) Send(key, value []byte) error {
	errPtr := C.fluvio_error_new()
	defer C.fluvio_error_free(errPtr)
	C.topic_producer_send(t.wrapper, (*C.uint8_t)(unsafe.Pointer(&key[0])), C.size_t(len(key)),
		(*C.uint8_t)(unsafe.Pointer(&value[0])), C.size_t(len(value)), errPtr)
	if errPtr.msg == nil {
		return nil
	}
	return NewFluvioError(C.GoString(errPtr.msg))
}

func (t *TopicProducer) SendString(key, value string) error {
	return t.Send([]byte(key), []byte(value))
}
