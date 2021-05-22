package fluvio

import (
	"strings"
)

var ErrInvalidOffsetType = NewFluvioError("received invalid offset type")
var ErrNoRecord = NewFluvioError("no records received")

type FluvioError struct {
	msg string
}

func (e *FluvioError) Error() string {
	return e.msg
}

func NewFluvioError(msg string) error {
	return &FluvioError{strings.ToLower(msg)}
}
