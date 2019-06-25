use bindgen;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Fix753 {}
impl bindgen::callbacks::ParseCallbacks for Fix753 {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        Some(original_item_name.trim_start_matches("Fix753_").to_owned())
    }
}

fn main() {
    if let Some(nvencode) = option_env!("NVENCODE_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", nvencode);
    }
    println!("cargo:rustc-link-lib=nvidia-encode");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(Fix753 {}))
        .whitelist_function("NvEnc.*")
        .whitelist_type("NV.*")
        .whitelist_type("_NV.*")
        .whitelist_type("PNV.*")
        .whitelist_var("Fix753.*")
        .whitelist_var("NV.*")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
