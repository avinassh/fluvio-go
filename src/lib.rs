use fluvio::{Fluvio as _Fluvio, TopicProducer as _TopicProducer};
use fluvio_future::task::run_block_on;

extern crate libc;

pub struct TopicProducer {
    inner: _TopicProducer,
}

impl TopicProducer {
    pub extern "C" fn send(&mut self) {
        run_block_on(self.inner.send("some key", "some value")).expect("failed to send values");
    }
}

pub struct Fluvio {
    inner: _Fluvio,
}

impl Fluvio {
    fn new() -> Self {
        Fluvio {
            inner: run_block_on(_Fluvio::connect()).expect("failed to connect to fluvio"),
        }
    }
    fn get_topic_producer(&mut self) -> TopicProducer {
        TopicProducer {
            inner: run_block_on(self.inner.topic_producer("hello-python"))
                .expect("failed to get the topic producer"),
        }
    }
}

#[no_mangle]
pub extern "C" fn connect() -> *mut Fluvio {
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
pub extern "C" fn fluvio_topic_producer_send(ptr: *mut TopicProducer) {
    let tp = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    tp.send();
}
