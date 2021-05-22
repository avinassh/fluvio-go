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

	topic := "echo"
	topicProducer, err := fluvioClient.TopicProducer(topic)
	if err != nil {
		fmt.Println("error while getting producer", err)
		return
	}
	defer topicProducer.Close()

	// start the producer in a go routine
	go func() {
		for i := 0; i <= 10; i++ {
			err = topicProducer.SendString(fmt.Sprintf("%d", i), fmt.Sprintf("Hello, Fluvio %d!", i))
			if err != nil {
				fmt.Println("error while sending", err)
				return
			}
		}
	}()

	// start the consumer
	partitionConsumer, err := fluvioClient.PartitionConsumer(topic, 0)
	if err != nil {
		fmt.Println("error while getting partition consumer", err)
		return
	}
	defer partitionConsumer.Close()

	stream, err := partitionConsumer.Stream(fluvio.NewOffsetFromBeginning(0))
	if err != nil {
		fmt.Println("error while getting stream on partition consumer", err)
		return
	}
	defer stream.Close()

	for i := 0; i <= 10; i++ {
		r, _ := stream.Next()
		fmt.Printf("Got record: key=%s, value=%s\n", string(r.Key), string(r.Value))
	}
}
