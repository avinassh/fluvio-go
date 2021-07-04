# Fluvio Client for Go

Go binding for Fluvio streaming platform

## Install

Make sure you have already installed [fluvio](https://github.com/infinyon/fluvio), it is up and running.

```shell
# assuming all the dependencies are installed
git clone github.com/avinassh/fluvio-go/fluvio.git
make build
```

## Quick Start

See the [example](example/main.go) file for the complete usage.

### Producer
```go
package main

import "github.com/avinassh/fluvio-go/fluvio"

func main()  {
	// error handling is omitted for brevity
	fluvioClient, _ := fluvio.Connect()
	defer fluvioClient.Close()
	topicProducer, _ := fluvioClient.TopicProducer("echo")
	defer topicProducer.Close()
	topicProducer.Send([]byte("0"), []byte("Hello"))
}
```

### Consumer
```go
package main

import "fmt"
import "github.com/avinassh/fluvio-go/fluvio"

func main()  {
	// error handling is omitted for brevity
	fluvioClient, _ := fluvio.Connect()
	defer fluvioClient.Close()
	partitionConsumer, _ := fluvioClient.PartitionConsumer("echo", 0)
	defer partitionConsumer.Close()
	stream, _ := partitionConsumer.Stream(fluvio.NewOffsetFromBeginning(0))
	defer stream.Close()
	for {
		r, _ := stream.Next()
		fmt.Printf("Got record: key=%s, value=%s\n", string(r.Key), string(r.Value))
	}
}
```

## Smart Streams
### Filter
Check the [example](example/smart_stream.go) for the full usage.
```go
package main

import "fmt"
import "github.com/avinassh/fluvio-go/fluvio"

func main()  {
	// error handling is omitted for brevity
	fluvioClient, _ := fluvio.Connect()
	defer fluvioClient.Close()
	wasmFile := "example/filter.wasm"
	config, _ := fluvioClient.ConsumerConfigWithWasmFilter(wasmFile)
	defer config.Close()
	partitionConsumer, _ := fluvioClient.PartitionConsumer("echo", 0)
	defer partitionConsumer.Close()
	stream, _ := partitionConsumer.StreamWithConfig(fluvio.NewOffsetFromBeginning(0), config)
	defer stream.Close()
	for {
		r, _ := stream.Next()
		fmt.Printf("Got record: key=%s, value=%s\n", string(r.Key), string(r.Value))
	}
}
```

## License
The MIT license. Please check `LICENSE` for more details.

