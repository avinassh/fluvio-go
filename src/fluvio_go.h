#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct FluvioWrapper FluvioWrapper;

typedef struct TopicProducerWrapper TopicProducerWrapper;

typedef struct FluvioErrorWrapper {
  char *msg;
} FluvioErrorWrapper;

struct FluvioErrorWrapper *fluvio_error_new(void);

void fluvio_error_free(struct FluvioErrorWrapper *err_ptr);

struct FluvioWrapper *fluvio_connect(struct FluvioErrorWrapper *err_ptr);

struct TopicProducerWrapper *fluvio_topic_producer(struct FluvioWrapper *fluvio_ptr,
                                                   const char *topic_ptr,
                                                   struct FluvioErrorWrapper *err_ptr);

void topic_producer_send(struct TopicProducerWrapper *topic_ptr,
                         const uint8_t *key,
                         size_t key_len,
                         const uint8_t *value,
                         size_t value_len,
                         struct FluvioErrorWrapper *err_ptr);
