# Fluvio Client for Go

Go binding for Fluvio streaming platform

## Install

Make sure you have already installed [fluvio](https://github.com/infinyon/fluvio), it is up and running.

```shell
go get github.com/avinassh/fluvio-go/fluvio
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

## License
The MIT license. Please check `LICENSE` for more details.

