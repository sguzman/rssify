{flake}: let
  cargoToml = builtins.fromTOML (builtins.readFile (flake + "/Cargo.toml"));
in
  cargoToml.package.name
