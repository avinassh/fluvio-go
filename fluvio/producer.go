package fluvio

/*
#cgo LDFLAGS: -L../src -lfluvio_go
#include "../src/fluvio_go.h"
*/
import "C"
import "unsafe"

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

func (t *TopicProducer) Close() {
	C.topic_producer_free(t.wrapper)
}
