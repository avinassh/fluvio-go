gen:
	cargo build
	mv target/debug/libfluvio_go.dylib src/libfluvio_go.dylib

build: gen
	go build

run: build
	./fluvio-go

go:
	go build
	./fluvio-go

clean:
	cargo clean
	go clean
	rm src/libfluvio_go.*
