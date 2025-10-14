{
  pkgs,
  flake,
  inputs,
}: let
  dev = import ./shell.nix {
    inherit pkgs flake inputs;
  };
  libs = import ../data/libs.nix {
    inherit pkgs inputs;
  };
  env = import ../data/env.nix {
    inherit pkgs inputs;
  };
  be = import ../funcs/buildEnv.nix {
    inherit pkgs;
  };
  es = be env;
in {
  packages = libs.dev;

  # Source the banner file, then start fish
  devshell.interactive.fish.text = dev.fish;

  motd = "";

  env = es;

  commands = import ../data/cmd.nix {
    inherit pkgs flake;
  };
}
