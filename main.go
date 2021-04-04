package main

import (
	cgo "github.com/avinassh/fluvio-go/fluvio_cgo"
)

func main() {
	fluvio := cgo.Connect()
	tp := cgo.FluvioTopicProducer([]cgo.Fluvio{*fluvio})
	cgo.FluvioTopicProducerSend([]cgo.TopicProducer{*tp})
}