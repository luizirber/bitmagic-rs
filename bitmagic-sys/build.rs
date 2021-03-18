use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn main() {
    // TODO: deal with
    //   - optimization settings
    //   - wasm

    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    let host_and_target_contain = |s| host.contains(s) && target.contains(s);

    // patch BitMagic/lang-maps/CMakeLists.txt
    // -- start of patching
    let cmake_config = Path::new("BitMagic/lang-maps/CMakeLists.txt");

    // Open and read the file entirely
    let mut src = File::open(&cmake_config).unwrap();
    let mut data = String::new();
    src.read_to_string(&mut data).unwrap();
    drop(src); // Close the file early

    // Run the replace operation in memory
    let new_data = data.replace("\nset(CMAKE_BINARY_DIR", "\n#set(CMAKE_BINARY_DIR");

    // Recreate the file and dump the processed contents to it
    let mut dst = File::create(&cmake_config).unwrap();
    dst.write(new_data.as_bytes()).unwrap();
    // -- end of patching

    let mut config = cmake::Config::new("BitMagic/lang-maps/");
    config
        .build_target("bm-static")
        .define("BM_NO_STL", "1")
        //.define("CMAKE_EXPORT_COMPILE_COMMANDS", "1")
        //.env("CMAKE_EXPORT_COMPILE_COMMANDS", "1")
        .very_verbose(true);
    let dst = config.build();

    let mut path_dst = PathBuf::new();
    path_dst.push(dst);
    path_dst.push("build");
    if host_and_target_contain("windows") && host_and_target_contain("msvc") {
        path_dst.push(config.get_profile());
    }
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
