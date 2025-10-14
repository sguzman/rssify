{flake}: let
  cargoToml = builtins.fromTOML (builtins.readFile (flake + "/Cargo.toml"));
  version = cargoToml.toolchain.channel;
in
  version
