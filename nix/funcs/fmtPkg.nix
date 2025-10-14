let
  build = p: let
    pn =
      if p ? pname
      then p.pname
      else (p.name or "pkg");
    ver =
      if p ? version
      then p.version
      else "";
    lbl =
      if ver == ""
      then "${pn}"
      else "${pn} ${ver}";
  in ''echo "        • ${lbl}"'';
in
  build
