# bitmagic-sys

Raw (unsafe, low-level) bindings for [BitMagic](http://www.bitmagic.io).
Exposes the BitMagic C API as generated by [bindgen].

[bindgen]: https://crates.io/crates/bindgen

The bindings generation is inspired by how [zstd-rs] handles `bindgen` calls.
There is a copy of the `bindgen`-generated files in `src/bindings.rs`
and by default it is used when the crate is build.
If the optional feature `bindgen` is active,
then it will run `bindgen` during the build process and regenerate the bindings
dynamically. This is useful in places where installing the `bindgen`
dependencies (like `libclang`) is difficult.

[zstd-rs]: https://github.com/gyscos/zstd-rs/