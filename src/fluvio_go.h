#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct FluvioWrapper FluvioWrapper;

typedef struct TopicProducerWrapper TopicProducerWrapper;

typedef struct FluvioErrorWrapper {
  char *msg;
} FluvioErrorWrapper;

struct FluvioErrorWrapper *custom_error_new(void);

void custom_error_free(struct FluvioErrorWrapper *err_ptr);

struct FluvioWrapper *fluvio_connect(struct FluvioErrorWrapper *err_ptr);

struct TopicProducerWrapper *fluvio_topic_producer(struct FluvioWrapper *ptr,
                                                   const char *topic_ptr,
                                                   struct FluvioErrorWrapper *err_ptr);

void topic_producer_send(struct TopicProducerWrapper *ptr,
                         const char *key_ptr,
                         const char *value_ptr,
                         struct FluvioErrorWrapper *err_ptr);
