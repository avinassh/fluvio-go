#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct FluvioWrapper FluvioWrapper;

typedef struct OffsetWrapper OffsetWrapper;

typedef struct PartitionConsumerStream PartitionConsumerStream;

typedef struct PartitionConsumerWrapper PartitionConsumerWrapper;

typedef struct TopicProducerWrapper TopicProducerWrapper;

typedef struct FluvioErrorWrapper {
  char *msg;
} FluvioErrorWrapper;

typedef struct RecordWrapper {
  int64_t offset;
  const uint8_t *key;
  size_t key_len;
  const uint8_t *value;
  size_t value_len;
} RecordWrapper;

struct FluvioErrorWrapper *fluvio_error_new(void);

void fluvio_error_free(struct FluvioErrorWrapper *err_ptr);

void record_free(struct RecordWrapper *record_ptr);

struct OffsetWrapper *offset_beginning(void);

struct OffsetWrapper *offset_end(void);

struct OffsetWrapper *offset_from_beginning(uint32_t offset);

struct OffsetWrapper *offset_from_end(uint32_t offset);

struct OffsetWrapper *offset_absolute(int64_t index, struct FluvioErrorWrapper *err_ptr);

void offset_free(struct OffsetWrapper *offset_ptr);

struct FluvioWrapper *fluvio_connect(struct FluvioErrorWrapper *err_ptr);

void fluvio_free(struct FluvioWrapper *fluvio_ptr);

struct TopicProducerWrapper *fluvio_topic_producer(struct FluvioWrapper *fluvio_ptr,
                                                   const char *topic_ptr,
                                                   struct FluvioErrorWrapper *err_ptr);

void topic_producer_send(struct TopicProducerWrapper *topic_ptr,
                         const uint8_t *key,
                         size_t key_len,
                         const uint8_t *value,
                         size_t value_len,
                         struct FluvioErrorWrapper *err_ptr);

void topic_producer_free(struct TopicProducerWrapper *topic_producer_ptr);

struct PartitionConsumerWrapper *fluvio_partition_consumer(struct FluvioWrapper *fluvio_ptr,
                                                           const char *topic_ptr,
                                                           int32_t partition,
                                                           struct FluvioErrorWrapper *err_ptr);

void partition_consumer_free(struct PartitionConsumerWrapper *partition_consumer_ptr);

struct PartitionConsumerStream *partition_consumer_stream(struct PartitionConsumerWrapper *partition_consumer_ptr,
                                                          const struct OffsetWrapper *offset_ptr,
                                                          struct FluvioErrorWrapper *err_ptr);

struct RecordWrapper *partition_consumer_stream_next(struct PartitionConsumerStream *stream_ptr,
                                                     struct FluvioErrorWrapper *err_ptr);

void partition_consumer_stream_free(struct PartitionConsumerStream *stream_ptr);
