{
  stdenvNoCC,
  mdbook,
  name,
  directory,
  source,
}:
stdenvNoCC.mkDerivation {
  inherit name;
  src = source;
  nativeBuildInputs = [ mdbook ];
  dontConfigure = true;
  buildPhase = ''
    mdbook build ${directory} --dest-dir "$PWD/rendered" 2>&1 | tee "$TMPDIR/mdbook.log"
    if grep 'ERROR' "$TMPDIR/mdbook.log"; then
      echo "[BOOK] ${name} logged the error above, so a page of it is wrong."
      exit 1
    fi
  '';
  installPhase = ''
    cp --recursive rendered $out
  '';
}
