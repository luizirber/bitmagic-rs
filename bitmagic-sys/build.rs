fn main() {
    // TODO: deal with
    //   - optimization settings
    //   - wasm

    //let host = env::var("HOST").unwrap();
    //let target = env::var("TARGET").unwrap();

    //let host_and_target_contain = |s| host.contains(s) && target.contains(s);

    let mut config = cc::Build::new();

    config
        .cpp(true) // Switch to C++ library compilation.
        .include("BitMagic/lang-maps/libbm/include")
        .include("BitMagic/src")
        .file("BitMagic/lang-maps/libbm/src/libbm.cpp")
        .define("BM_NO_STL", "1");

    config.compile("bm");

    println!("cargo:rustc-link-lib=static=bm");

    generate_bindings();
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./BitMagic/lang-maps/libbm")
        .header("BitMagic/lang-maps/libbm/include/libbm.h")
        .size_t_is_usize(true)
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}

#[cfg(not(feature = "bindgen"))]
fn generate_bindings() {}
