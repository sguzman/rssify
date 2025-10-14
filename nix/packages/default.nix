# nix/packages/default.nix
{
  flake,
  pkgs,
  inputs,
  ...
}: let
  name = import ../conf/name.nix {
    inherit flake;
  };
  path = ./. + "/${name}";
in
  pkgs.callPackage path {
    inherit inputs pkgs;
  }
