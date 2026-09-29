{
  stdenvNoCC,
  names,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "outputs";
  src = source;
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  inherit names;
  passAsFile = [ "names" ];
  checkPhase = ''
    sort "$namesPath" > "$TMPDIR/flake.txt"
    awk -F '|' '/^\| `[a-zA-Z0-9-]*` \| [a-z]* \|/ {
      name = $2
      kind = $3
      gsub(/[` ]/, "", name)
      gsub(/ /, "", kind)
      print name, kind
    }' docs/OUTPUTS.md | sort > "$TMPDIR/docs.txt"
    if ! diff "$TMPDIR/flake.txt" "$TMPDIR/docs.txt"; then
      echo "[OUTPUTS] the flake and docs/OUTPUTS.md do not agree."
      echo "[OUTPUTS] a line marked < is an output nobody recorded."
      echo "[OUTPUTS] a line marked > is a row naming an output that is gone."
      exit 1
    fi
    count=$(wc -l < "$TMPDIR/flake.txt")
    echo "[OUTPUTS] the flake and docs/OUTPUTS.md agree on $count outputs."
  '';
  installPhase = ''
    touch $out
  '';
}
