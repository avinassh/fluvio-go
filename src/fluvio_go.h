#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct FluvioWrapper FluvioWrapper;

typedef struct TopicProducerWrapper TopicProducerWrapper;

struct FluvioWrapper *fluvio_connect(void);

struct TopicProducerWrapper *fluvio_topic_producer(struct FluvioWrapper *ptr,
                                                   const char *topic_ptr);

void topic_producer_send(struct TopicProducerWrapper *ptr,
                         const char *key_ptr,
                         const char *value_ptr);
