use std::{env, path::PathBuf};

fn main() {
    let raylib = cmake::build("raylib");
    let lib = raylib.join("lib");

    println!("cargo:rustc-link-search=native={}", lib.display());
    println!("cargo:rustc-link-lib=static=raylib");

    let include = raylib.join("include");
    let bindings = bindgen::Builder::default()
        .header(include.join("raylib.h").display().to_string())
        .blocklist_item("PI")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings.");
    let bindings_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(bindings_path.join("bindings.rs"))
        .expect("Failed to write bindings file.");
}
