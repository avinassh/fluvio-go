ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

gen:
	cargo build
	mv target/debug/libfluvio_go.dylib src/libfluvio_go.dylib

build: gen
	go build

run: build
	./fluvio-go

clean:
	rm src/libfluvio_go.dylib