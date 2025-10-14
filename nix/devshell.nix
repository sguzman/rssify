# nix/devshell.nix
{
  perSystem,
  pkgs,
  flake,
  inputs,
  ...
}: let
  shell = import ./dev/build.nix {
    inherit pkgs flake inputs;
  };
in
  perSystem.devshell.mkShell shell
