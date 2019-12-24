extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=libspirv");
    println!("cargo:rerun-if-changed=include/libspirv.h")
}