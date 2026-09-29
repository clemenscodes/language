{
  stdenvNoCC,
  findutils,
  gawk,
  ripgrep,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "punctuation";
  src = source;
  nativeBuildInputs = [
    findutils
    gawk
    ripgrep
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    refusals=0

    if rg --hidden --line-number --no-heading \
      --regexp '\x{2014}|\x{2013}|\x{2192}|\x{2190}|\x{21C4}|\x{2208}|\x{2026}|\x{2500}' \
      . > "$TMPDIR/characters.txt"; then
      echo "[PUNCTUATION] an em dash, an en dash, an arrow, a set membership, an ellipsis"
      echo "[PUNCTUATION] or a box drawing line. Write a period, a comma, parentheses,"
      echo "[PUNCTUATION] to for a range, or the word the sign replaces."
      cat "$TMPDIR/characters.txt"
      refusals=1
    else
      status=$?
      if [ "$status" != 1 ]; then
        echo "[PUNCTUATION] ripgrep ended with status $status, so the scan itself failed."
        exit "$status"
      fi
    fi

    find . -name '*.md' -type f | sort > "$TMPDIR/pages.txt"
    xargs --delimiter '\n' --no-run-if-empty awk '
      FNR == 1 { fence = 0 }
      /^[[:space:]]*```/ { fence = !fence; next }
      fence { next }
      {
        prose = $0
        gsub(/`[^`]*`/, "", prose)
        if (prose ~ /;/) print FILENAME ":" FNR ": a semicolon: " $0
        if (prose ~ /^[[:space:]]*(-{3,}|\*{3,}|_{3,})[[:space:]]*$/) print FILENAME ":" FNR ": a separator: " $0
      }
    ' < "$TMPDIR/pages.txt" > "$TMPDIR/prose.txt"
    if [ -s "$TMPDIR/prose.txt" ]; then
      echo "[PUNCTUATION] prose has no semicolon and no separator line."
      echo "[PUNCTUATION] use a period, and, or so. A section that needs a separator"
      echo "[PUNCTUATION] is a page of its own. Code inside backticks is not prose."
      cat "$TMPDIR/prose.txt"
      refusals=1
    fi

    if [ "$refusals" != 0 ]; then
      exit 1
    fi

    pages=$(wc -l < "$TMPDIR/pages.txt")
    echo "[PUNCTUATION] every file was read, and $pages pages of prose have no refused sign."
  '';
  installPhase = ''
    touch $out
  '';
}
