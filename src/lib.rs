extern crate libc;

use fluvio::{Fluvio as _Fluvio, TopicProducer as _TopicProducer, PartitionConsumer};
use fluvio_future::task::run_block_on;
use libc::c_char;
use std::ffi::CStr;


pub struct TopicProducer {
    inner: _TopicProducer,
}

impl TopicProducer {
    fn send(&mut self, value: &str) {
        run_block_on(self.inner.send("from golang", value)).expect("failed to send values");
    }
}

pub struct Fluvio {
    inner: _Fluvio,
}

impl Fluvio {
    fn new() -> Self {

        let p = PartitionConsumer;

        Fluvio {
            inner: run_block_on(_Fluvio::connect()).expect("failed to connect to fluvio"),
        }
    }
    fn get_topic_producer(&mut self) -> TopicProducer {
        TopicProducer {
            inner: run_block_on(self.inner.topic_producer("chinni"))
                .expect("failed to get the topic producer"),
        }
    }
}

#[no_mangle]
pub extern "C" fn fluvio_connect() -> *mut Fluvio {
    Box::into_raw(Box::new(Fluvio::new()))
}

#[no_mangle]
pub extern "C" fn fluvio_topic_producer(ptr: *mut Fluvio) -> *mut TopicProducer {
    let fl = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    Box::into_raw(Box::new(fl.get_topic_producer()))
}

#[no_mangle]
pub extern "C" fn fluvio_topic_producer_send(ptr: *mut TopicProducer, value_ptr: *const c_char) {
    let tp = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let value = unsafe {
        assert!(!value_ptr.is_null());
        CStr::from_ptr(value_ptr).to_str().unwrap()
    };
    tp.send(value);
}
