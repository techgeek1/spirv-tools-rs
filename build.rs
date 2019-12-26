use std::env;
use std::path::PathBuf;

fn main() {
    /*
    // Tell cargo to link the spirv library
    println!("cargo:rustc-link-lib=libspirv");

    // Generate the spirv-tools bindings
    let bindings = bindgen::Builder::default()
        .header("spirv-tools/include/spirv-tools/libspirv.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
        */
}