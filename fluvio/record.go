package fluvio

type Record struct {
	Offset int64
	Key    []byte
	Value  []byte
}
