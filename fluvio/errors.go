package fluvio

import "strings"

type FluvioError struct {
	msg string
}

func (e *FluvioError) Error() string {
	return e.msg
}

func NewFluvioError(msg string) error {
	return &FluvioError{strings.ToLower(msg)}
}
