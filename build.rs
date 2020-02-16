extern crate bindgen;
extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn generate_rust_bindings() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Don't derive Copy and Clone
        .no_copy("Algorithm")
        .rustified_non_exhaustive_enum("*")
        // Whitelist
        .whitelist_type("Logger")
        .whitelist_type("Level")
        // Set clang arguments. Need to force C++ here because header files
        // end in .h instead of .hpp
        .clang_args(vec!["-x", "c++", "-std=c++17"])
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn generate_cpp_bindings() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("No cbindgen.toml found");

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_config(config)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("cpp/bindings.h");
}

fn main() {
    generate_rust_bindings();
    generate_cpp_bindings();
}
