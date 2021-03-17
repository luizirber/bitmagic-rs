use std::env;
use std::path::PathBuf;

fn main() {
    // TODO: deal with
    //   - optimization settings
    //   - wasm

    let dst = cmake::Config::new("BitMagic/lang-maps/")
        .build_target("bm-static")
        .define("BM_NO_STL", "1")
        //.define("CMAKE_EXPORT_COMPILE_COMMANDS", "1")
        //.env("CMAKE_EXPORT_COMPILE_COMMANDS", "1")
        .very_verbose(true)
        .build();
    let mut path_dst = PathBuf::new();
    path_dst.push(dst);
    path_dst.push("build");
    println!("cargo:rustc-link-search=native={}", path_dst.display());
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
