{inputs}: let
  cargoToml = builtins.fromTOML (builtins.readFile (inputs.self + "/rust-toolchain.toml"));
  version = cargoToml.toolchain.channel;
in
  version
