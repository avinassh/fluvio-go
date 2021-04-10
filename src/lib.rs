extern crate libc;

use fluvio::{Fluvio, TopicProducer};
use fluvio_future::task::run_block_on;
use libc::c_char;
use std::ffi::CStr;

pub struct TopicProducerWrapper {
    inner: TopicProducer,
}

impl TopicProducerWrapper {
    fn send<K, V>(&self, key: K, value: V)
    where
        K: Into<Vec<u8>>,
        V: Into<Vec<u8>>,
    {
        run_block_on(self.inner.send(key, value)).expect("failed to send values");
    }
}

pub struct FluvioWrapper {
    inner: Fluvio,
}

impl FluvioWrapper {
    fn new() -> Self {
        FluvioWrapper {
            inner: run_block_on(Fluvio::connect()).expect("failed to connect to fluvio"),
        }
    }
    fn topic_producer<S: Into<String>>(&mut self, topic: S) -> TopicProducerWrapper {
        TopicProducerWrapper {
            inner: run_block_on(self.inner.topic_producer(topic))
                .expect("failed to get the topic producer"),
        }
    }
}

#[no_mangle]
pub extern "C" fn fluvio_connect() -> *mut FluvioWrapper {
    Box::into_raw(Box::new(FluvioWrapper::new()))
}

#[no_mangle]
pub extern "C" fn fluvio_topic_producer(
    ptr: *mut FluvioWrapper,
    topic_ptr: *const c_char,
) -> *mut TopicProducerWrapper {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let topic = unsafe {
        assert!(!topic_ptr.is_null());
        CStr::from_ptr(topic_ptr).to_str().unwrap()
    };
    Box::into_raw(Box::new(f.topic_producer(topic)))
}

// TODO: change types of key and value to be bytes
#[no_mangle]
pub extern "C" fn topic_producer_send(
    ptr: *mut TopicProducerWrapper,
    key_ptr: *const c_char,
    value_ptr: *const c_char,
) {
    let tp = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let key = unsafe {
        assert!(!key_ptr.is_null());
        CStr::from_ptr(key_ptr).to_str().unwrap()
    };
    let value = unsafe {
        assert!(!value_ptr.is_null());
        CStr::from_ptr(value_ptr).to_str().unwrap()
    };
    tp.send(key, value);
}
