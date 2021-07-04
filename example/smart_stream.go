package main

import (
	"fmt"

	"github.com/avinassh/fluvio-go/fluvio"
)

func main() {
	fluvioClient, err := fluvio.Connect()
	if err != nil {
		fmt.Println("error while connecting", err)
		return
	}
	defer fluvioClient.Close()

	topic := "hello-smartstreams"
	// file generated from the official example: https://www.fluvio.io/docs/smartstreams/quick-start/#create-a-new-smartstream
	// this filter lets any record of string which contains character `a`
	wasmFile := "example/filter.wasm"

	config, err := fluvioClient.ConsumerConfigWithWasmFilter(wasmFile)
	if err != nil {
		fmt.Println("error while getting consumer config", err)
		return
	}
	defer config.Close()

	// create a consumer, for the same topic, on partition 0
	partitionConsumer, err := fluvioClient.PartitionConsumer(topic, 0)
	if err != nil {
		fmt.Println("error while getting partition consumer", err)
		return
	}
	defer partitionConsumer.Close()

	// create a stream object
	stream, err := partitionConsumer.StreamWithConfig(fluvio.NewOffsetFromBeginning(0), config)
	if err != nil {
		fmt.Println("error while getting stream on partition consumer", err)
		return
	}
	defer stream.Close()

	// loop over streamer to fetch all the records
	for i := 0; i <= 10; i++ {
		r, _ := stream.Next()
		fmt.Printf("Got record: value=%s\n", string(r.Value))
	}
}
