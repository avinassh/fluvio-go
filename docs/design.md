## Design

Rust FFI:
- Every fluvio struct has a wrapper (usually suffixed with `Wrapper`) e.g. `TopicProducer` will have a `TopicProducerWrapper`
- Wrapper struct would have a field called `inner` which points to actual fluvio object
- These "wrapper" objects are exposed in the FFI
- Each of the wrapper structs have a method to free the memory, usually suffixed with `_free` (e.g. `partition_consumer_free`) 

Error Handling:
- Every FFI method takes a pointer to an error struct (`FluvioErrorWrapper`) as the last parameter
- `FluvioErrorWrapper` which wraps over `FluvioError`, represents the stringified error

CGO:
- All cgo related stuff are put in `fluvio` directory. No `C` is exposed outside it
- Every "wrapper" struct from FFI would have a struct here, sans the suffix
- The structs have an internal field called `wrapper` which point to wrapper objects of Rust
