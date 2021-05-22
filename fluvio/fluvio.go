package fluvio

/*
#cgo LDFLAGS: -L../src -lfluvio_go
#include "../src/fluvio_go.h"
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

type PartitionConsumer struct {
	wrapper *C.PartitionConsumerWrapper
}

type PartitionConsumerStream struct {
	wrapper *C.PartitionConsumerStream
}

func Connect() (*Fluvio, error) {
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

func (f *Fluvio) Close() {
	C.fluvio_free(f.wrapper)
}
