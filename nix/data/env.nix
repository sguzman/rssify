{
  pkgs,
  inputs,
}: let
  rust = import ../conf/rust.nix {
    inherit inputs;
  };
  pkgs' = import inputs.nixpkgs {
    system = pkgs.system;
    overlays = [inputs.rust-overlay.overlays.default];
  };
  toolchain = pkgs'.rust-bin.stable."${rust}".default;
  libs = import ./libs.nix {
    inherit pkgs inputs;
  };
  mkPkg = import ../funcs/mkPkgConfigPath.nix {
    inherit pkgs;
  };
  envs = {
    BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.llvmPackages.clang}/lib/clang/${pkgs.llvmPackages.clang.version}/include";
    CC = "${pkgs.clang}/bin/clang";
    CURL_CA_BUNDLE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
    CXX = "${pkgs.clang}/bin/clang++";
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs.run;
    LIBCLANG_PATH = "${pkgs.lib.getLib pkgs.llvmPackages.libclang}/lib";
    NIX_SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
    OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    OPENSSL_NO_VENDOR = "1";
    PKG_CONFIG_PATH = mkPkg libs.run;
    RUSTFLAGS = "-C link-arg=-Wl,-rpath,${pkgs.openssl.out}/lib";
    RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust";
    SCCACHE_DISABLE = "0";
    SHELL = "${pkgs.fish}/bin/fish";
    SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
    TESSDATA_PREFIX = "${pkgs.tesseract}/share/tessdata";
    WINIT_UNIX_BACKEND = "wayland";
  };
in
  envs
