extern crate libc;

use std::ffi::{CStr, CString};
use std::pin::Pin;
use std::slice;

use fluvio::consumer::Record;
use fluvio::{ConsumerConfig, Fluvio, FluvioError, Offset, PartitionConsumer, TopicProducer};
use fluvio_future::io::{Stream, StreamExt};
use fluvio_future::task::run_block_on;
use libc::{c_char, size_t};

#[repr(C)]
pub struct FluvioErrorWrapper {
    msg: *mut c_char,
}

impl FluvioErrorWrapper {
    fn new() -> FluvioErrorWrapper {
        FluvioErrorWrapper {
            msg: std::ptr::null_mut(),
        }
    }
}

impl Drop for FluvioErrorWrapper {
    fn drop(&mut self) {
        if !self.msg.is_null() {
            unsafe {
                CString::from_raw(self.msg);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn fluvio_error_new() -> *mut FluvioErrorWrapper {
    Box::into_raw(Box::new(FluvioErrorWrapper::new()))
}

#[no_mangle]
pub extern "C" fn fluvio_error_free(err_ptr: *mut FluvioErrorWrapper) {
    if !err_ptr.is_null() {
        unsafe {
            Box::from_raw(err_ptr);
        }
    }
}

#[repr(C)]
pub struct RecordWrapper {
    offset: i64,
    key: *const u8,
    key_len: size_t,
    value: *const u8,
    value_len: size_t,
}

impl RecordWrapper {
    fn new(record: Record) -> RecordWrapper {
        let value: Box<[u8]> = record.value().into();
        let mut record_wrapper = RecordWrapper {
            offset: record.offset(),
            key: std::ptr::null(),
            key_len: 0,
            value: Box::into_raw(value) as *const u8,
            value_len: record.value().len(),
        };
        if let Some(key) = record.key() {
            record_wrapper.key_len = key.len();
            let key: Box<[u8]> = key.into();
            record_wrapper.key = Box::into_raw(key) as *const u8;
        }
        record_wrapper
    }
}

impl Drop for RecordWrapper {
    fn drop(&mut self) {
        if !self.key.is_null() {
            unsafe {
                Box::from_raw(slice::from_raw_parts_mut(self.key as *mut u8, self.key_len));
            }
        }
        if !self.value.is_null() {
            unsafe {
                Box::from_raw(slice::from_raw_parts_mut(
                    self.value as *mut u8,
                    self.value_len,
                ));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn record_free(record_ptr: *mut RecordWrapper) {
    if record_ptr.is_null() {
        unsafe {
            Box::from_raw(record_ptr);
        }
    }
}

pub struct OffsetWrapper {
    inner: Offset,
}

impl OffsetWrapper {
    fn from_offset(offset: Offset) -> OffsetWrapper {
        return OffsetWrapper { inner: offset };
    }

    fn beginning() -> OffsetWrapper {
        return OffsetWrapper {
            inner: Offset::beginning(),
        };
    }

    fn end() -> OffsetWrapper {
        return OffsetWrapper {
            inner: Offset::end(),
        };
    }

    fn from_beginning(offset: u32) -> OffsetWrapper {
        return OffsetWrapper {
            inner: Offset::from_beginning(offset),
        };
    }

    fn from_end(offset: u32) -> OffsetWrapper {
        return OffsetWrapper {
            inner: Offset::from_end(offset),
        };
    }

    fn absolute(index: i64) -> Result<Offset, FluvioError> {
        Offset::absolute(index)
    }
}

#[no_mangle]
pub extern "C" fn offset_beginning() -> *mut OffsetWrapper {
    Box::into_raw(Box::new(OffsetWrapper::beginning()))
}

#[no_mangle]
pub extern "C" fn offset_end() -> *mut OffsetWrapper {
    Box::into_raw(Box::new(OffsetWrapper::end()))
}

#[no_mangle]
pub extern "C" fn offset_from_beginning(offset: u32) -> *mut OffsetWrapper {
    Box::into_raw(Box::new(OffsetWrapper::from_beginning(offset)))
}

#[no_mangle]
pub extern "C" fn offset_from_end(offset: u32) -> *mut OffsetWrapper {
    Box::into_raw(Box::new(OffsetWrapper::from_end(offset)))
}

#[no_mangle]
pub extern "C" fn offset_absolute(
    index: i64,
    err_ptr: *mut FluvioErrorWrapper,
) -> *mut OffsetWrapper {
    match OffsetWrapper::absolute(index) {
        Ok(offset) => Box::into_raw(Box::new(OffsetWrapper::from_offset(offset))),
        Err(fluvio_error) => {
            let err = unsafe {
                assert!(!err_ptr.is_null());
                &mut *err_ptr
            };
            err.msg = CString::new(fluvio_error.to_string()).unwrap().into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn offset_free(offset_ptr: *mut OffsetWrapper) {
    if !offset_ptr.is_null() {
        unsafe {
            Box::from_raw(offset_ptr);
        }
    }
}

type PartitionConsumerStreamInner = Pin<Box<dyn Stream<Item = Result<Record, FluvioError>> + Send>>;
pub struct PartitionConsumerStream {
    pub inner: PartitionConsumerStreamInner,
}

impl PartitionConsumerStream {
    fn next(&mut self) -> Option<Result<Record, FluvioError>> {
        run_block_on(self.inner.next())
    }
}

pub struct ConsumerConfigWrapper {
    wasm_module: Vec<u8>,
}

impl ConsumerConfigWrapper {
    fn new_config_with_wasm_filter(file: &str) -> Result<ConsumerConfigWrapper, std::io::Error> {
        match std::fs::read(file) {
            Ok(buffer) => Ok(ConsumerConfigWrapper {
                wasm_module: buffer,
            }),
            Err(err) => Err(err),
        }
    }
}

pub struct PartitionConsumerWrapper {
    inner: PartitionConsumer,
}

impl PartitionConsumerWrapper {
    fn new(inner: PartitionConsumer) -> PartitionConsumerWrapper {
        PartitionConsumerWrapper { inner }
    }

    fn stream(&self, offset: &OffsetWrapper) -> Result<PartitionConsumerStream, FluvioError> {
        return self.stream_with_config(offset, None);
    }

    fn stream_with_config(
        &self,
        offset: &OffsetWrapper,
        config_wrapper: Option<&ConsumerConfigWrapper>,
    ) -> Result<PartitionConsumerStream, FluvioError> {
        match config_wrapper {
            Some(config_wrapper) => {
                let config =
                    ConsumerConfig::default().with_wasm_filter(config_wrapper.wasm_module.clone());
                match run_block_on(self.inner.stream_with_config(offset.inner.clone(), config)) {
                    Ok(stream) => Ok(PartitionConsumerStream {
                        inner: Box::pin(stream),
                    }),
                    Err(e) => Err(e),
                }
            }
            None => match run_block_on(self.inner.stream(offset.inner.clone())) {
                Ok(stream) => Ok(PartitionConsumerStream {
                    inner: Box::pin(stream),
                }),
                Err(e) => Err(e),
            },
        }
    }
}

pub struct TopicProducerWrapper {
    inner: TopicProducer,
}

impl TopicProducerWrapper {
    fn new(inner: TopicProducer) -> Self {
        TopicProducerWrapper { inner }
    }

    fn send<K, V>(&self, key: K, value: V) -> Result<(), FluvioError>
    where
        K: Into<Vec<u8>>,
        V: Into<Vec<u8>>,
    {
        run_block_on(self.inner.send(key, value))
    }
}

pub struct FluvioWrapper {
    inner: Fluvio,
}

impl FluvioWrapper {
    fn new(inner: Fluvio) -> Self {
        FluvioWrapper { inner }
    }
    fn connect() -> Result<Fluvio, FluvioError> {
        run_block_on(Fluvio::connect())
    }
    fn topic_producer<S: Into<String>>(&mut self, topic: S) -> Result<TopicProducer, FluvioError> {
        run_block_on(self.inner.topic_producer(topic))
    }
    fn partition_consumer<S: Into<String>>(
        &mut self,
        topic: S,
        partition: i32,
    ) -> Result<PartitionConsumer, FluvioError> {
        run_block_on(self.inner.partition_consumer(topic, partition))
    }
}

#[no_mangle]
pub extern "C" fn fluvio_connect(err_ptr: *mut FluvioErrorWrapper) -> *mut FluvioWrapper {
    match FluvioWrapper::connect() {
        Ok(fluvio_obj) => Box::into_raw(Box::new(FluvioWrapper::new(fluvio_obj))),
        Err(fluvio_error) => {
            let err = unsafe {
                assert!(!err_ptr.is_null());
                &mut *err_ptr
            };
            err.msg = CString::new(fluvio_error.to_string()).unwrap().into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn fluvio_free(fluvio_ptr: *mut FluvioWrapper) {
    if !fluvio_ptr.is_null() {
        unsafe {
            Box::from_raw(fluvio_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn fluvio_topic_producer(
    fluvio_ptr: *mut FluvioWrapper,
    topic_ptr: *const c_char,
    err_ptr: *mut FluvioErrorWrapper,
) -> *mut TopicProducerWrapper {
    let f: &mut FluvioWrapper = unsafe {
        assert!(!fluvio_ptr.is_null());
        &mut *fluvio_ptr
    };
    let topic: &str = unsafe {
        assert!(!topic_ptr.is_null());
        CStr::from_ptr(topic_ptr).to_str().unwrap()
    };
    match f.topic_producer(topic) {
        Ok(topic_producer) => Box::into_raw(Box::new(TopicProducerWrapper::new(topic_producer))),
        Err(fluvio_error) => {
            let err = unsafe {
                assert!(!err_ptr.is_null());
                &mut *err_ptr
            };
            err.msg = CString::new(fluvio_error.to_string()).unwrap().into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn topic_producer_send(
    topic_ptr: *mut TopicProducerWrapper,
    key: *const u8,
    key_len: size_t,
    value: *const u8,
    value_len: size_t,
    err_ptr: *mut FluvioErrorWrapper,
) {
    let tp: &mut TopicProducerWrapper = unsafe {
        assert!(!topic_ptr.is_null());
        &mut *topic_ptr
    };
    let key: &[u8] = unsafe {
        assert!(!key.is_null());
        slice::from_raw_parts(key, key_len as usize)
    };
    let value: &[u8] = unsafe {
        assert!(!value.is_null());
        slice::from_raw_parts(value, value_len as usize)
    };
    if let Err(fluvio_error) = tp.send(key, value) {
        let err = unsafe {
            assert!(!err_ptr.is_null());
            &mut *err_ptr
        };
        err.msg = CString::new(fluvio_error.to_string()).unwrap().into_raw();
    }
}

#[no_mangle]
pub extern "C" fn topic_producer_free(topic_producer_ptr: *mut TopicProducerWrapper) {
    if !topic_producer_ptr.is_null() {
        unsafe {
            Box::from_raw(topic_producer_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn consumer_config_with_wasm_filter(
    file_ptr: *const c_char,
    err_ptr: *mut FluvioErrorWrapper,
) -> *mut ConsumerConfigWrapper {
    let file: &str = unsafe {
        assert!(!file_ptr.is_null());
        CStr::from_ptr(file_ptr).to_str().unwrap()
    };
    match ConsumerConfigWrapper::new_config_with_wasm_filter(file) {
        Ok(config) => Box::into_raw(Box::new(config)),
        Err(read_err) => {
            let err = unsafe {
                assert!(!err_ptr.is_null());
                &mut *err_ptr
            };
            err.msg = CString::new(read_err.to_string()).unwrap().into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn consumer_config_free(consumer_config_ptr: *mut ConsumerConfigWrapper) {
    if !consumer_config_ptr.is_null() {
        unsafe {
            Box::from_raw(consumer_config_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn fluvio_partition_consumer(
    fluvio_ptr: *mut FluvioWrapper,
    topic_ptr: *const c_char,
    partition: i32,
    err_ptr: *mut FluvioErrorWrapper,
) -> *mut PartitionConsumerWrapper {
    let f: &mut FluvioWrapper = unsafe {
        assert!(!fluvio_ptr.is_null());
        &mut *fluvio_ptr
    };
    let topic: &str = unsafe {
        assert!(!topic_ptr.is_null());
        CStr::from_ptr(topic_ptr).to_str().unwrap()
    };
    match f.partition_consumer(topic, partition) {
        Ok(partition_consumer) => {
            Box::into_raw(Box::new(PartitionConsumerWrapper::new(partition_consumer)))
        }
        Err(fluvio_error) => {
            let err = unsafe {
                assert!(!err_ptr.is_null());
                &mut *err_ptr
            };
            err.msg = CString::new(fluvio_error.to_string()).unwrap().into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn partition_consumer_free(partition_consumer_ptr: *mut PartitionConsumerWrapper) {
    if !partition_consumer_ptr.is_null() {
        unsafe {
            Box::from_raw(partition_consumer_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn partition_consumer_stream(
    partition_consumer_ptr: *mut PartitionConsumerWrapper,
    offset_ptr: *const OffsetWrapper,
    err_ptr: *mut FluvioErrorWrapper,
) -> *mut PartitionConsumerStream {
    let partition_consumer: &mut PartitionConsumerWrapper = unsafe {
        assert!(!partition_consumer_ptr.is_null());
        &mut *partition_consumer_ptr
    };
    let offset: &OffsetWrapper = unsafe {
        assert!(!offset_ptr.is_null());
        &*offset_ptr
    };
    match partition_consumer.stream(offset) {
        Ok(stream) => Box::into_raw(Box::new(stream)),
        Err(fluvio_error) => {
            let err = unsafe {
                assert!(!err_ptr.is_null());
                &mut *err_ptr
            };
            err.msg = CString::new(fluvio_error.to_string()).unwrap().into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn partition_consumer_stream_with_config(
    partition_consumer_ptr: *mut PartitionConsumerWrapper,
    offset_ptr: *const OffsetWrapper,
    config_ptr: *const ConsumerConfigWrapper,
    err_ptr: *mut FluvioErrorWrapper,
) -> *mut PartitionConsumerStream {
    let partition_consumer: &mut PartitionConsumerWrapper = unsafe {
        assert!(!partition_consumer_ptr.is_null());
        &mut *partition_consumer_ptr
    };
    let offset: &OffsetWrapper = unsafe {
        assert!(!offset_ptr.is_null());
        &*offset_ptr
    };
    let config: &ConsumerConfigWrapper = unsafe {
        assert!(!config_ptr.is_null());
        &*config_ptr
    };

    match partition_consumer.stream_with_config(offset, Some(config)) {
        Ok(stream) => Box::into_raw(Box::new(stream)),
        Err(fluvio_error) => {
            let err = unsafe {
                assert!(!err_ptr.is_null());
                &mut *err_ptr
            };
            err.msg = CString::new(fluvio_error.to_string()).unwrap().into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn partition_consumer_stream_next(
    stream_ptr: *mut PartitionConsumerStream,
    err_ptr: *mut FluvioErrorWrapper,
) -> *mut RecordWrapper {
    let stream: &mut PartitionConsumerStream = unsafe {
        assert!(!stream_ptr.is_null());
        &mut *stream_ptr
    };
    if let Some(result) = stream.next() {
        match result {
            Ok(record) => Box::into_raw(Box::new(RecordWrapper::new(record))),
            Err(fluvio_error) => {
                let err = unsafe {
                    assert!(!err_ptr.is_null());
                    &mut *err_ptr
                };
                err.msg = CString::new(fluvio_error.to_string()).unwrap().into_raw();
                std::ptr::null_mut()
            }
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn partition_consumer_stream_free(stream_ptr: *mut PartitionConsumerStream) {
    if !stream_ptr.is_null() {
        unsafe {
            Box::from_raw(stream_ptr);
        }
    }
}
