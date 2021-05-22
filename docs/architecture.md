# Architecture

`fluvio-go` provides Go binding for fluvio rust client.

1. The fluvio rust library does not have C FFI-ready methods. In `src/lib.rs` we have wrapped each of the fluvio methods for FFI.
2. We use [cbindgen](https://github.com/eqrion/cbindgen) to generate the C bindings, the generated files `fluvio_go.h` and `libfluvio_go` are placed in `src` directory.
3. The Go library files are placed in `fluvio` directory, these reference to generated files from `src` directory by including them in cgo header files

### Quick Overview

```
|- fluvio           // contains all the Go library files
|- src              // all the rust files
    - lib.rs        // the C FFI wrapper over actual fluvio library
    - fluvio_go.h   // auto generated header file for FFI
    - libfluvio_go  // auto generated libray for FFI
|- build.rs         // cbindgen config file which specifies how to generate the FFI files 
```