{
  pkgs,
  inputs,
}: let
  libs = {
    dev = import ./libs/dev.nix {
      inherit pkgs inputs;
    };
    run = import ./libs/run.nix {
      inherit pkgs;
    };
    build = import ./libs/build.nix {
      inherit pkgs;
    };
  };
in
  libs
