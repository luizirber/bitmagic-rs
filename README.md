# bitmagic-rs

Experimental Rust bindings for [BitMagic](http://www.bitmagic.io).

[bitmagic-sys] is the low level unsafe bindings exposing the BitMagic C API,
while the top-level [bitmagic] crate exposes a safe Rust API.

[bitmagic-sys]: ./bitmagic-sys/

This is a work in progress, and focused on exposing an API similar to
[fixedbitset 0.3.1](https://docs.rs/fixedbitset/0.3.1) before going for other methods in BitMagic.

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.37.0
