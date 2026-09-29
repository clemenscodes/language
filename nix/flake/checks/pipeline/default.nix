{
  stdenvNoCC,
  findutils,
  gawk,
  names,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "pipeline";
  src = source;
  nativeBuildInputs = [
    findutils
    gawk
  ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  inherit names;
  passAsFile = [ "names" ];
  checkPhase = ''
    find . -name moon.yml | sort > "$TMPDIR/projects.txt"

    : > "$TMPDIR/candidates.txt"
    while read -r file; do
      id=$(sed --quiet "s/^id: '\(.*\)'\$/\1/p" "$file")
      awk '
        match($0, /checks\.x86_64-linux\.[a-zA-Z0-9-]+/) {
          print substr($0, RSTART + 20, RLENGTH - 20)
        }
      ' "$file" | sed "s|\$project|$id|" >> "$TMPDIR/candidates.txt"
    done < "$TMPDIR/projects.txt"

    sort --unique "$TMPDIR/candidates.txt" > "$TMPDIR/moonChecks.txt"
    sort "$namesPath" > "$TMPDIR/flakeChecks.txt"

    if ! diff "$TMPDIR/flakeChecks.txt" "$TMPDIR/moonChecks.txt"; then
      echo "[PIPELINE] the flake and moon do not build the same checks."
      echo "[PIPELINE] a line marked < is a check of the flake that no moon task builds."
      echo "[PIPELINE] a line marked > is a moon task naming a check the flake does not have."
      exit 1
    fi

    tab=$(printf '\t')

    : > "$TMPDIR/inputs.txt"
    while read -r file; do
      awk '
        /^  [a-zA-Z][a-zA-Z0-9\/._-]*:$/ { check = "" }
        match($0, /checks\.x86_64-linux\.[a-zA-Z0-9-]+/) {
          check = substr($0, RSTART + 20, RLENGTH - 20)
        }
        /^      - / {
          entry = $0
          gsub(/^      - .|.$/, "", entry)
          sub(/^!/, "", entry)
          sub(/^\//, "", entry)
          sub(/[\/*].*$/, "", entry)
          if (check != "" && entry != "") printf "%s\t%s\n", check, entry
        }
      ' "$file" >> "$TMPDIR/inputs.txt"
    done < "$TMPDIR/projects.txt"
    sort --unique "$TMPDIR/inputs.txt" --output "$TMPDIR/inputs.txt"

    awk '
      /^ *[a-z][a-zA-Z0-9-]* =$/ { name = $1; next }
      name != "" && /sourceTree/ {
        line = $0
        sub(/.*sourceTree[[:space:]]*"[^"]*"/, "", line)
        remainder = line
        while (match(remainder, /"[^"]*"/)) {
          printf "%s\t%s\n", name, substr(remainder, RSTART + 1, RLENGTH - 2)
          remainder = substr(remainder, RSTART + RLENGTH)
        }
        if (line !~ /\]/) list = 1
        next
      }
      list {
        remainder = $0
        while (match(remainder, /"[^"]*"/)) {
          printf "%s\t%s\n", name, substr(remainder, RSTART + 1, RLENGTH - 2)
          remainder = substr(remainder, RSTART + RLENGTH)
        }
        if ($0 ~ /\]/) { list = 0; name = "" }
      }
    ' nix/flake/checks/default.nix | sort --unique > "$TMPDIR/sources.txt"

    : > "$TMPDIR/findings.txt"
    sources=0
    while IFS="$tab" read -r check path; do
      sources=$((sources + 1))
      root=''${path%%/*}
      if ! grep --quiet --line-regexp --fixed-strings "$check$tab$root" "$TMPDIR/inputs.txt"; then
        printf '%s reads %s and its task lists no %s\n' "$check" "$path" "$root" >> "$TMPDIR/findings.txt"
      fi
    done < "$TMPDIR/sources.txt"

    if [ "$sources" = 0 ]; then
      echo "[PIPELINE] no source of a gate was found, the parser is wrong."
      exit 1
    fi

    if [ -s "$TMPDIR/findings.txt" ]; then
      echo "[PIPELINE] a gate reads what its moon task does not list, so moon answers out of a"
      echo "[PIPELINE] cache that never saw the change. Widen the task in the same commit."
      cat "$TMPDIR/findings.txt"
      exit 1
    fi
    echo "[PIPELINE] $sources sources of the gates, and every one of them is a task input."
  '';
  installPhase = ''
    touch $out
  '';
}
