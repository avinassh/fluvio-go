package fluvio

/*
#cgo LDFLAGS: -L../src -lfluvio_go
#include "../src/fluvio_go.h"
*/
import "C"
import (
	"unsafe"
)

func (f *Fluvio) PartitionConsumer(topic string, partition int32) (*PartitionConsumer, error) {
	topicPtr := C.CString(topic)
	defer C.free(unsafe.Pointer(topicPtr))
	errPtr := C.fluvio_error_new()
	defer C.fluvio_error_free(errPtr)
	consumer := C.fluvio_partition_consumer(f.wrapper, topicPtr, C.int32_t(partition), errPtr)
	if errPtr.msg != nil {
		return nil, NewFluvioError(C.GoString(errPtr.msg))
	}
	return &PartitionConsumer{wrapper: consumer}, nil
}

func (pc *PartitionConsumer) Stream(offset Offset) (*PartitionConsumerStream, error) {
	errPtr := C.fluvio_error_new()
	defer C.fluvio_error_free(errPtr)
	var offsetPtr *C.OffsetWrapper
	switch o := offset.(type) {
	case *OffsetFromBeginning:
		offsetPtr = C.offset_from_beginning(C.uint32_t(o.value))
		defer C.offset_free(offsetPtr)
	case *OffsetFromEnd:
		offsetPtr = C.offset_from_end(C.uint32_t(o.value))
		defer C.offset_free(offsetPtr)
	case *OffsetAbsolute:
		offsetPtr = C.offset_absolute(C.int64_t(o.value), errPtr)
		if errPtr.msg != nil {
			return nil, NewFluvioError(C.GoString(errPtr.msg))
		}
		defer C.offset_free(offsetPtr)
	default:
		return nil, ErrInvalidOffsetType
	}
	stream := C.partition_consumer_stream(pc.wrapper, offsetPtr, errPtr)
	if errPtr.msg != nil {
		return nil, NewFluvioError(C.GoString(errPtr.msg))
	}
	return &PartitionConsumerStream{wrapper: stream}, nil
}

func (pcs *PartitionConsumerStream) Next() (*Record, error) {
	errPtr := C.fluvio_error_new()
	defer C.fluvio_error_free(errPtr)
	result := C.partition_consumer_stream_next(pcs.wrapper, errPtr)
	if errPtr.msg != nil {
		return nil, NewFluvioError(C.GoString(errPtr.msg))
	}
	if result == nil {
		return nil, ErrNoRecord
	}
	defer C.record_free(result)
	record := &Record{
		Offset: int64(result.offset),
		Value:  C.GoBytes(unsafe.Pointer(result.value), C.int(result.value_len)),
	}
	if C.int(result.key_len) > 0 {
		record.Key = C.GoBytes(unsafe.Pointer(result.key), C.int(result.key_len))
	}
	return record, nil
}

func (pc *PartitionConsumer) Close() {
	C.partition_consumer_free(pc.wrapper)
}

func (pcs *PartitionConsumerStream) Close() {
	C.partition_consumer_stream_free(pcs.wrapper)
}
