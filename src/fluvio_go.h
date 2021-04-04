#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Fluvio Fluvio;

typedef struct TopicProducer TopicProducer;

struct Fluvio *connect(void);

struct TopicProducer *fluvio_topic_producer(struct Fluvio *ptr);

void fluvio_topic_producer_send(struct TopicProducer *ptr);
