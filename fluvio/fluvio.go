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

func (t *TopicProducer) Send(key, value string) error {
	keyPtr := C.CString(key)
	defer C.free(unsafe.Pointer(keyPtr))
	valuePtr := C.CString(value)
	defer C.free(unsafe.Pointer(valuePtr))
	errPtr := C.fluvio_error_new()
	defer C.fluvio_error_free(errPtr)
	C.topic_producer_send(t.wrapper, keyPtr, valuePtr, errPtr)
	if errPtr.msg == nil {
		return nil
	}
	return NewFluvioError(C.GoString(errPtr.msg))
}

// func (t *TopicProducer) SendWithErr(key, value string) {
// 	keyPtr := C.CString(key)
// 	defer C.free(unsafe.Pointer(keyPtr))
// 	valuePtr := C.CString(value)
// 	defer C.free(unsafe.Pointer(valuePtr))
//
// 	// by allocating memory for struct
// 	errPtr := (*C.FluvioErrorWrapper)(C.calloc(C.size_t(1), (C.size_t)(unsafe.Sizeof([1]C.FluvioErrorWrapper{}))))
// 	var errPtr C.FluvioErrorWrapper
// 	errPtr = (*C.FluvioErrorWrapper)(unsafe.Pointer(new(C.FluvioErrorWrapper)))
//
// 	errPtr := C.CString("")
// 	fmt.Println(errPtr)
// 	defer C.free(unsafe.Pointer(errPtr))
// 	C.topic_producer_send(t.wrapper, keyPtr, valuePtr, errPtr)
// 	fmt.Println(errPtr)
// 	fmt.Println(C.GoString(errPtr))
// }
