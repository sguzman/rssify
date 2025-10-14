# nix/packages/*/default.nix
{
  pkgs,
  inputs,
  ...
}: let
  cargoToml = builtins.fromTOML (builtins.readFile (inputs.self + "/Cargo.toml"));
  pname = cargoToml.workspace.metadata.name;
  pkgs' = import inputs.nixpkgs {
    system = pkgs.system;
    overlays = [inputs.rust-overlay.overlays.default];
  };

  rust = import ../../conf/rust.nix {
    inherit inputs;
  };
  toolchain = pkgs'.rust-bin.stable."${rust}".default;
  naersk = pkgs.callPackage inputs.naersk {
    cargo = toolchain;
    rustc = toolchain;
  };

  lib = pkgs.lib;

  libs = import ../../data/libs.nix {
    inherit pkgs inputs;
  };
  runEnvs = import ../../data/env.nix {
    inherit pkgs inputs;
  };
  wrapperArgs =
    lib.concatStringsSep " "
    (lib.mapAttrsToList (
        k: v: "--set ${k} ${lib.escapeShellArg (toString v)}"
      )
      runEnvs);
in
  naersk.buildPackage {
    inherit pname;
    version = cargoToml.workspace.package.version;

    src = ../../..;

    cargoToml = ../../../Cargo.toml;
    cargoLock = ../../../Cargo.lock;

    nativeBuildInputs = libs.build;

    buildInputs = libs.run;

    postInstall = ''
      if [ -d "$out/bin" ]; then
        for b in "$out/bin/"*; do
          [ -f "$b" ] && [ -x "$b" ] || continue
          wrapProgram "$b" ${wrapperArgs}
        done
      fi
    '';
  }
