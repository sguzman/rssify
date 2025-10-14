{
  pkgs,
  flake,
  inputs,
}: let
  fishBanner = import ./fish.nix {
    inherit pkgs flake inputs;
  };
in {
  fish = fishBanner;
  bash = "";
  zsh = "";
  ion = "";
}
