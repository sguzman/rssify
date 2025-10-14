{pkgs}: let
  lib = pkgs.lib;
in
  ls:
    lib.mapAttrsToList (name: value: {inherit name value;}) ls
