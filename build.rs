use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native={}", env!("NVENCODE_LIB_DIR"));
    println!("cargo:rustc-link-lib=nvidia-encode");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("NvEnc.*")
        .whitelist_type("NV.*")
        .whitelist_type("_NV.*")
        .whitelist_type("PNV.*")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}