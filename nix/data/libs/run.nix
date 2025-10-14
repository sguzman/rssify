{pkgs}: let
  runLibs = with pkgs;
    [
      systemd
      alsa-lib
      libxkbcommon
      wayland
      libGL
      vulkan-loader
    ]
    ++ (with xorg; [
      libXcursor
      libXrandr
      libXi
      libX11
      libXext
      libXfixes
    ]);
in
  runLibs
