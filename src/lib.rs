extern crate libc;

#[no_mangle]
pub extern "C" fn fl_hello_world() {
    println!("Hello World!");
}

#[no_mangle]
pub extern "C" fn fl_addition(a: u32, b: u32) -> u32 {
    a + b
}