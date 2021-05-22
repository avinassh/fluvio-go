package fluvio

type Offset interface {
	isOffset()
}

type OffsetFromBeginning struct {
	value int64
}

func (_ OffsetFromBeginning) isOffset() {}

type OffsetFromEnd struct {
	value int64
}

func (_ OffsetFromEnd) isOffset() {}

type OffsetAbsolute struct {
	value int64
}

func (_ OffsetAbsolute) isOffset() {}

func NewOffsetFromBeginning(value uint32) Offset {
	return &OffsetFromBeginning{value: int64(value)}
}

func NewOffsetFromEnd(value uint32) Offset {
	return &OffsetFromEnd{value: int64(value)}
}

func NewOffsetAbsolute(value int64) Offset {
	return &OffsetAbsolute{value: value}
}
