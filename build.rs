fn main() {
    println!("cargo:rustc-link-lib=bass");
    println!("cargo:rustc-link-lib=bass_fx");
    println!("cargo:rerun-if-changed=src/wrapper.h");
    bindgen::Builder::default()
        .header("src/bass_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bass_bindings.rs")
        .expect("Couldn't write bindings!");
}
