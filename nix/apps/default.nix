# nix/apps/default.nix
{
  flake,
  pkgs,
  ...
}: let
  name = import ../conf/name.nix {
    inherit flake;
  };
  drv = flake.packages.${pkgs.system}.${name};
in {
  type = "app";
  program = "${drv}/bin/${name}";
}
