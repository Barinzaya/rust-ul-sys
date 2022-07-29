use std::env;
use std::path::PathBuf;

fn main() {
    if let Some(p) = env::var_os("ULTRALIGHT_SDK_ROOT") {
        let ul_path = PathBuf::from(p);

        let lib_path = ul_path.join("lib");
        println!("cargo:rustc-link-search={}", lib_path.display());
    } else {
        println!("cargo:rustc-link-search=/usr/local/lib");
    }

    println!("cargo:rustc-link-lib=dylib=Ultralight");
    println!("cargo:rustc-link-lib=dylib=UltralightCore");
    println!("cargo:rustc-link-lib=dylib=WebCore");
    println!("cargo:rustc-link-lib=dylib=AppCore");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header("wrapper/wrapper.h")
        .impl_debug(true)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate_inline_functions(true)
        .whitelist_var("^UL.*|JS.*|ul.*|WK.*")
        .whitelist_type("^UL.*|JS.*|ul.*|WK.*")
        .whitelist_function("^UL.*|JS.*|ul.*|WK.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
