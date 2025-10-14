{
  pkgs,
  flake,
}: let
  name = import ../conf/name.nix {
    inherit flake;
  };
  commands = [
    {
      name = "devhelp";
      help = "reprint this banner/help";
      command = "${pkgs.fish}/bin/fish -c devhelp";
    }

    {
      name = "build";
      help = "nix build .#${name}";
      command = "nix build .#${name}";
    }
    {
      name = "run";
      help = "cargo run --";
      command = "cargo run --";
    }

    {
      name = "fmt";
      help = "format Nix + Rust (treefmt: alejandra+rustfmt)";
      command = "nix fmt";
    }
    {
      name = "fmt:nix";
      help = "format Nix only (Alejandra)";
      command = "alejandra .";
    }
    {
      name = "fmt:rust";
      help = "format Rust only (cargo fmt)";
      command = "cargo fmt --all";
    }
    # TOML
    {
      name = "fmt:toml";
      help = "format TOML (taplo)";
      command = "taplo fmt .";
    }
    # Lua
    {
      name = "fmt:lua";
      help = "format Lua (stylua)";
      command = "stylua .";
    }
    # Python
    {
      name = "fmt:python";
      help = "format python (ruff)";
      command = "ruff format .";
    }
    # JS/TS/HTML/CSS
    {
      name = "fmt:web";
      help = "format JS/TS/HTML/CSS (biome)";
      command = "${pkgs.biome}/bin/biome format --write .";
    }
    # Shell
    {
      name = "fmt:sh";
      help = "format shell (shellharden)";
      command = "${pkgs.shellharden}/bin/shellharden -i (fd -e sh -e bash)";
    }
    # Fish
    {
      name = "fmt:fish";
      help = "format fish scripts";
      command = "fd -e fish -X ${pkgs.fish}/bin/fish_indent --write";
    }

    {
      name = "check";
      help = "cargo check (all targets)";
      command = "cargo check --all-targets";
    }
    {
      name = "test";
      help = "cargo test";
      command = "cargo test";
    }

    {
      name = "lint:nix";
      help = "Nix lint: statix + deadnix";
      command = "statix check . && deadnix .";
    }
    {
      name = "fix:nix";
      help = "Auto-fix with statix";
      command = "statix fix .";
    }
  ];
in
  commands
