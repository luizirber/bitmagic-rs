let
  sources = import ./nix/sources.nix;
  rustPlatform = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { overlays = [ (import sources.rust-overlay) ]; };
in
  with pkgs;

  pkgs.mkShell {
    nativeBuildInputs = [
      clang_13
    ];

    buildInputs = [
      rustPlatform.rust.cargo
      openssl
      pkg-config

      llvmPackages_13.libclang
      llvmPackages_13.libcxxClang

      cmake
      rust-bindgen

      git
      gdb
      cargo-watch
    ];

    BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${llvmPackages_13.libclang.lib}/lib/clang/${lib.getVersion clang}/include";
    LIBCLANG_PATH = "${llvmPackages_13.libclang.lib}/lib";

    # workaround for https://github.com/NixOS/nixpkgs/blob/48dfc9fa97d762bce28cc8372a2dd3805d14c633/doc/languages-frameworks/python.section.md#python-setuppy-bdist_wheel-cannot-create-whl
    SOURCE_DATE_EPOCH = 315532800; # 1980
  }
