{pkgs}: let
  lib = pkgs.lib;
  build = pkgsList:
    lib.makeSearchPath "lib/pkgconfig" (map lib.getDev pkgsList);
in
  build
