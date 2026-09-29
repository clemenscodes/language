{
  stdenvNoCC,
  findutils,
  gawk,
  ripgrep,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "test";
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

    find . -name '*.rs' -type f | sort > "$TMPDIR/sources.txt"
    sources=$(wc --lines < "$TMPDIR/sources.txt")
    echo "[TEST] this gate reads $sources sources."

    while IFS= read -r source_file; do
      awk '
        /^mod tests \{$/ { inside = 1; next }
        inside && /^    fn / { functions += 1 }
        inside && /^    #\[test\]$/ { tests += 1 }
        END { printf "%d %d\n", functions, tests }
      ' "$source_file" > "$TMPDIR/counts.txt"
      read -r functions tests < "$TMPDIR/counts.txt"
      if [ "$functions" != "$tests" ]; then
        surplus=$((functions - tests))
        echo "[TEST] $source_file keeps $surplus functions beside its tests."
        refusals=1
      fi
    done < "$TMPDIR/sources.txt"

    if [ "$refusals" != 0 ]; then
      echo "[TEST] whatever a test needs, that test states, because a helper hides"
      echo "[TEST] the arrangement the test exists to show."
      exit 1
    fi

    if rg --line-number --no-heading --glob '**/src/main.rs' \
      '(^|[^a-zA-Z])(if|else|match|while|for|unwrap_or|\?;)([^a-zA-Z]|$)' . > "$TMPDIR/decisions.txt"; then
      echo "[TEST] a decision inside a main. No test can call a main, so nothing decides there."
      echo "[TEST] a main parses the command line, delegates, reports and sets the status."
      cat "$TMPDIR/decisions.txt"
      exit 1
    else
      status=$?
      if [ "$status" != 1 ]; then
        echo "[TEST] ripgrep ended with status $status, so the scan itself failed."
        exit "$status"
      fi
    fi

    echo "[TEST] every test module was read, and none of them keeps a function beside its tests."
  '';
  installPhase = ''
    touch $out
  '';
}
