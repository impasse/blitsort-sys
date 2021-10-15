use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=static=wrapper");

    cc::Build::new()
        .file("wrapper.c")
        .include("./blitsort")
        .opt_level(3)
        .compile("libwrapper.a");    

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Wimplicit-function-declaration")
        .clang_arg("-I./blitsort")
        .allowlist_function("blitsort*")
        .size_t_is_usize(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
