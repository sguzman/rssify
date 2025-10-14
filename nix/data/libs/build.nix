{pkgs}: let
  buildPkgs = with pkgs;
    [
      systemd
      alsa-lib
      wayland
      libxkbcommon

      openssl
      clang
      llvmPackages.libclang
      pkg-config
      makeWrapper
    ]
    ++ (with xorg; [
      libX11
      libXcursor
      libXext
      libXfixes
      libXi
      libXrandr
    ]);
in
  buildPkgs
