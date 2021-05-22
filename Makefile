gen:
	cargo build --release
	mv target/release/libfluvio_go.* src/

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
