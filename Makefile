gen:
	cargo build --release
	mv target/release/libfluvio_go.* src/

build: gen
	go build -o fluvio-go example/main.go

run: build
	./fluvio-go

go:
	go build -o fluvio-go example/main.go
	./fluvio-go

clean:
	cargo clean
	go clean
	rm src/libfluvio_go.*
	rm fluvio-go
