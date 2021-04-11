extern crate libc;

use std::ffi::{CStr, CString};
use std::slice;

use fluvio::{Fluvio, FluvioError, TopicProducer};
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
    if err_ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(err_ptr);
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
pub extern "C" fn fluvio_topic_producer(
    ptr: *mut FluvioWrapper,
    topic_ptr: *const c_char,
    err_ptr: *mut FluvioErrorWrapper,
) -> *mut TopicProducerWrapper {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let topic = unsafe {
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
    ptr: *mut TopicProducerWrapper,
    key: *const u8,
    key_len: size_t,
    value: *const u8,
    value_len: size_t,
    err_ptr: *mut FluvioErrorWrapper,
) {
    let tp = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let key = unsafe {
        assert!(!key.is_null());
        slice::from_raw_parts(key, key_len as usize)
    };
    let value = unsafe {
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
