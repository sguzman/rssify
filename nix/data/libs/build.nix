{pkgs}: let
  buildPkgs = with pkgs; [
    openssl
    pkg-config
    makeWrapper
  ];
in
  buildPkgs
