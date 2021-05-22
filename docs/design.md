## Design

Rust FFI:
- Every Fluvio struct has a wrapper (usually suffixed), like TopicProducer will have a TopicProducerWrapper
- Wrapper struct would have a field called `inner` which points to actual Fluvio object
- These "wrapper" objects are exposed in the FFI
- Every FFI method takes a pointer to an error struct as the last parameter
- FluvioErrorWrapper which wraps over FluvioError, represents the stringified error

CGO:
- All cgo related stuff are put in `fluvio` directory. No `C` is exposed outside of it
- Every "wrapper" struct from FFI would have a struct here, sans the suffix
- The struct have an internal field called `wrapper` which point to wrapper objects of Rust