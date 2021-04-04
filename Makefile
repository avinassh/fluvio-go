ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

gen:
	rm -rf fluvio_cgo
	mkdir "fluvio_cgo"
	cargo build
	# c-for-go --ccincl --ccdefs cforgo.yml
	c-for-go --ccincl cforgo.yml

build: gen
	go build

run: build
	./fluvio-go