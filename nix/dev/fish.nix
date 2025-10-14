{
  pkgs,
  flake,
  inputs,
}: let
  name = import ../conf/name.nix {
    inherit flake;
  };
  libs = import ../data/libs.nix {
    inherit pkgs inputs;
  };
  fmt = import ../funcs/fmtPkg.nix;

  runLibs =
    if libs ? run
    then libs.run
    else [];
  pkgLibs =
    if libs ? pkg
    then libs.pkg
    else [];
  buildLibs =
    if libs ? build
    then libs.build
    else [];
  devLibs =
    if libs ? dev
    then libs.dev
    else [];

  runSummary = builtins.concatStringsSep "\n" (map fmt runLibs);
  pkgSummary = builtins.concatStringsSep "\n" (map fmt pkgLibs);
  buildSummary = builtins.concatStringsSep "\n" (map fmt buildLibs);
  devSummary = builtins.concatStringsSep "\n" (map fmt devLibs);

  runBlock =
    if runLibs == []
    then ""
    else ''
            set_color brwhite; echo "  runtime libs:"; set_color normal
            set_color cyan
      ${runSummary}
            set_color normal
            echo
    '';

  pkgBlock =
    if pkgLibs == []
    then ""
    else ''
            set_color brwhite; echo "  pkg-config libs:"; set_color normal
            set_color magenta
      ${pkgSummary}
            set_color normal
            echo
    '';

  buildBlock =
    if buildLibs == []
    then ""
    else ''
            set_color brwhite; echo "  build libs:"; set_color normal
            set_color yellow
      ${buildSummary}
            set_color normal
            echo
    '';

  devBlock =
    if devLibs == []
    then ""
    else ''
            set_color brwhite; echo "  dev libs:"; set_color normal
            set_color green
      ${devSummary}
            set_color normal
            echo
    '';

  banner = pkgs.writeText "${name}-banner.fish" ''
        function fish_greeting
          set_color -o cyan
          echo "${name} devshell"
          set_color normal

          # Where & who
          set_color brwhite; echo; echo "Project:"; set_color normal
          echo "  • PWD     → "(pwd)
          if command -q git
            if git rev-parse --is-inside-work-tree 2>/dev/null
              echo -n "  • branch  → "
              git rev-parse --abbrev-ref HEAD 2>/dev/null
            end
          end

          # Libraries (auto-generated from Nix)
          set_color brwhite; echo; echo "Libraries (from Nix):"; set_color normal
    ${runBlock}${pkgBlock}${buildBlock}${devBlock}

          # Environment (exported variables)
          set_color brwhite; echo "Environment:"; set_color normal
          # Print exported vars, one per line, NAME bold + VALUE, both brblue.
          # Keep it readable: only typical ALLCAPS names.
          for _line in (env | sort)
            set -l kv (string split -m 1 '=' -- $_line)
            set -l k $kv[1]
            set -l v $kv[2]
            if test -n "$k"
              if string match -rq '^[A-Z0-9_]+$' -- $k
                set_color -o red
                printf "  • %s" $k
                set_color red
                printf " = %s\n" $v
                set_color normal
              end
            end
          end
          echo

          # Dynamic commands menu
          set_color brwhite; echo "Menu (devshell commands):"; set_color normal
          if type -q menu
            menu
          else
            echo "  (menu unavailable)"
          end

          echo
          set_color brwhite; echo "Tip:"; set_color normal
          echo "  • Run 'devhelp' anytime to reprint this banner."
          echo
        end

        # Reprint on demand
        function devhelp
          fish_greeting
        end
  '';
  text = "exec ${pkgs.fish}/bin/fish -C 'source ${banner}'";
in
  text
