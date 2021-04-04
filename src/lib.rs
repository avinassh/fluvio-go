use fluvio::{
    Fluvio as _Fluvio,
};
use fluvio_future::{
    task::run_block_on,
};

extern crate libc;

pub struct Fluvio {
    inner: _Fluvio
}

impl Fluvio {
    fn new() -> Self {
        Fluvio {
            inner: run_block_on(_Fluvio::connect()).expect("Failed to connect to fluvio"),
        }
    }
}

#[no_mangle]
pub extern "C" fn connect() -> *mut Fluvio {
    Box::into_raw(Box::new(Fluvio::new()))
}