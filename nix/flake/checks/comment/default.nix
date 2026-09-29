{
  stdenvNoCC,
  ripgrep,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "comment";
  src = source;
  nativeBuildInputs = [ ripgrep ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    if rg --line-number --no-heading --glob '*.rs' '^\s*(//|///|//!)' . > "$TMPDIR/comments.txt"; then
      echo "[COMMENT] a crate contains no comment."
      echo "[COMMENT] a name states what a thing is, a type constrains it, a test proves it."
      echo "[COMMENT] where none of those states the meaning, the design is what changes."
      cat "$TMPDIR/comments.txt"
      exit 1
    else
      status=$?
      if [ "$status" != 1 ]; then
        echo "[COMMENT] ripgrep ended with status $status, so the scan itself failed."
        exit "$status"
      fi
    fi
    sources=$(rg --files --glob '*.rs' . | wc --lines)
    echo "[COMMENT] $sources sources were read, and none of them contains a comment."
  '';
  installPhase = ''
    touch $out
  '';
}
