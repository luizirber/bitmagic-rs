use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    // TODO: deal with
    //   - optimization settings
    //   - wasm

    let dst = Config::new("BitMagic/lang-maps/")
        .build_target("bm-static")
        .define("BM_NO_STL", "1")
        .very_verbose(true)
        .build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=bm-static");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I./BitMagic/lang-maps/libbm")
        .header("BitMagic/lang-maps/libbm/include/libbm.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}
