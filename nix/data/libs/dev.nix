{
  pkgs,
  inputs,
}: let
  rust = import ../../conf/rust.nix {
    inherit inputs;
  };
  pkgs' = import inputs.nixpkgs {
    system = pkgs.system;
    overlays = [inputs.rust-overlay.overlays.default];
  };
  toolchain = pkgs'.rust-bin.stable."${rust}".default;
  buildPkgs = with pkgs'; [
    # Rust toolchain
    toolchain

    # Release + changelog
    cargo-release
    git-cliff

    # Linker/tooling for fast builds
    mold
    clang

    # Common native deps many crates need
    pkg-config
    openssl
    cacert

    # Handy utilities
    jq
    curl

    # Formatting/linting for Nix
    alejandra
    statix
    deadnix
    taplo
    stylua
    fish
    ruff

    # LSP servers
    nixd
    ty
    rust-analyzer
  ];
in
  buildPkgs
