extern crate cbindgen;

use std::env;

fn main() {
    // we want this to be run everytime
    println!("cargo:rerun-if-changed=src/");
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/fluvio_go.h");
}