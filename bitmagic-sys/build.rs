use std::env;

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
        .include("BitMagic/lang-maps/libbm/src")
        .include("BitMagic/src")
        .flag_if_supported("-std=c++14")
        .file("BitMagic/lang-maps/libbm/src/libbm.cpp")
        //.define("BM64ADDR", "1")
        .define("BM_NO_STL", "1");

    parse_features(&mut config);

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

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}

#[cfg(not(feature = "bindgen"))]
fn generate_bindings() {}

fn parse_features(config: &mut cc::Build) {
    if let Ok(features) = env::var("CARGO_CFG_TARGET_FEATURE") {
        // TODO: read CARGO_BUILD_RUSTFLAGS too, so we can set
        // the target cpu? For example, this can be added to
        // ~/.cargo/config in order to use all features for the CPU
        // in the machine where the code is being compiled:
        // ```
        // [target.x86_64-unknown-linux-gnu]
        // rustflags = ["-C", "target-cpu=native"]
        // ```

        if features.contains("avx2") {
            config.define("BMAVX2OPT", "1");
            config.flag_if_supported("-mavx2");
            config.flag_if_supported("-march=skylake");
        } else if features.contains("sse4.2") {
            config.define("BMSSE42OPT", "1");
            config.flag_if_supported("-msse4.2");
            config.flag_if_supported("-march=nehalem");
        }
    } else {
        config.define("BM_SIMD_NO", "1");
    }
}
