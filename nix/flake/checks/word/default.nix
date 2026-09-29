{
  stdenvNoCC,
  ripgrep,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "word";
  src = source;
  nativeBuildInputs = [ ripgrep ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    refusals=0
    exclusion=""

    fail() {
      echo "[WORD] ripgrep ended with status $1, so the scan itself failed."
      exit "$1"
    }

    refuse() {
      scope=$1
      shift
      options=()
      while [ "$1" != "--" ]; do
        options+=("$1")
        shift
      done
      shift
      if rg --hidden --line-number --no-heading \
        "''${options[@]}" \
        --glob '!nix/flake/checks/word/**' \
        --glob '!docs/RUST_STYLE.md' \
        --glob '!crates/language/src/core/languages/**' \
        "$scope" > "$TMPDIR/lines.txt"; then
        if [ -n "$exclusion" ]; then
          if rg --invert-match --regexp "$exclusion" "$TMPDIR/lines.txt" > "$TMPDIR/remainder.txt"; then
            mv "$TMPDIR/remainder.txt" "$TMPDIR/lines.txt"
          else
            status=$?
            if [ "$status" != 1 ]; then
              fail "$status"
            fi
            return 0
          fi
        fi
        printf '[WORD] %s\n' "$@"
        cat "$TMPDIR/lines.txt"
        refusals=1
      else
        status=$?
        if [ "$status" != 1 ]; then
          fail "$status"
        fi
      fi
    }

    refuse . --ignore-case --regexp \
      '(^|[^a-zA-Z])(carr(y|ies|ied|ying)|stands?|standing|stood|lies|lie|lying|holds?|holding|held|sits?|sitting|sat|says?|said|saying|speaks?|spoke|spoken|speaking|aims?|aimed|aiming)([^a-zA-Z]|$)' -- \
      "one of the refused verbs. State where a thing is and what it does." \
      "nothing here speaks. A program requests and responds." \
      "a name states a meaning, a type constrains a value, a test proves a behaviour." \
      "a pointer points. It never aims."

    refuse . --ignore-case --regexp '(^|[^a-zA-Z])noughts?([^a-zA-Z]|$)' -- \
      "nought is no word of this repository. Write zero."

    refuse . --ignore-case --regexp '(^|[^a-zA-Z])corpus([^a-zA-Z]|$)' -- \
      "corpus is no word of this repository. Most sentences are better without it."

    refuse . --regexp '0x[0-9A-F]*[a-f][0-9a-fA-F]*' -- \
      "hexadecimal this repository forms is written in upper case."

    refuse . --pcre2 --glob '*.md' \
      --regexp '(^|[^0-9a-zA-Z_])(?=[0-9a-f]{8}([^0-9a-zA-Z_]|$))[0-9a-f]*[a-f]' -- \
      "an address this repository writes down is written in upper case."

    refuse . --ignore-case \
      --regexp '(^|[^a-zA-Z])shapes?d?([^a-zA-Z]|$)|(^|[^a-zA-Z])shaping([^a-zA-Z]|$)' -- \
      "shape is no word of this repository. A thing laid out is a structure."

    refuse . --ignore-case --regexp '(^|[^a-zA-Z])places?([^a-zA-Z]|$)' -- \
      "place is no word of this repository, nowhere and never." \
      "a value that says where a thing is names that thing. An offset, an address," \
      "a position, a storage, a slot, a directory. A place names none of them."

    refuse . --ignore-case --regexp '(^|[^a-zA-Z])whole([^a-zA-Z]|$)' -- \
      "whole is no word of this repository. Say the thing without it."

    refuse . --ignore-case \
      --regexp '(^|[^a-zA-Z])(an?|the|one|every|each|its|their|no|unbroken|first|last)[\s_]+runs?([^a-zA-Z]|$)|(^|[^a-zA-Z])runs?[\s_]+of([^a-zA-Z]|$)' -- \
      "run is a verb. A process runs and nothing else does."

    refuse . --ignore-case --regexp '(^|[^a-zA-Z])readings?([^a-zA-Z]|$)' -- \
      "reading is the act and never the thing. Name what came out of it."

    refuse . --ignore-case --regexp '(^|[^a-zA-Z])written([^a-zA-Z]|$)' -- \
      "written names nothing, anywhere, in any language, in code and in prose."

    refusedWords="written|given|at|of|over|wrote|number|found|one|other|later|behind|rest"
    participles="taken|waiting|wanted|shown|kept|settled|made|describes|covers|mapped|readable"
    stopgaps="util|utils|helper|helpers|common|misc|shared"
    emptyWords="$refusedWords|$participles|$stopgaps"
    refuse . --glob 'crates/**/*.rs' \
      --regexp "^[[:space:]]*(let|const|static)[[:space:]]+(mut[[:space:]]+)?($emptyWords)\b" \
      --regexp "^[[:space:]]*($emptyWords)[[:space:]]*:" \
      --regexp "[(,][[:space:]]*($emptyWords)[[:space:]]*:" -- \
      "a binding, a field, an argument or a constant named with an empty word." \
      "docs/RUST_STYLE.md refuses these words outright, and a value is a noun for" \
      "what it is, never for what an action left behind."

    exclusion='(\blet\s+|\|\s*)(mut\s+)?([a-z_]*_)?(padding|packing|spelling|string|king|plumbing|tooling|ceiling|findings?)\s*[,:=|]'
    refuse . --glob 'crates/**/*.rs' \
      --regexp '^\s*let\s+(mut\s+)?[a-z_]*ing\s*[:=]' \
      --regexp '\|\s*(mut\s+)?[a-z_]*ing\s*[,|]' -- \
      "a binding named for the act that made it. Name what the value is."
    exclusion=""

    acts='(read|walked|gathered|folded|trimmed|satisfied|finished|annotated|marked|padded|named|kept|opens|kill|resolve|cut|there|here)'
    refuse . --glob 'crates/**/*.rs' \
      --regexp "^\s*let\s+(mut\s+)?$acts\s*[:=]" \
      --regexp "\|\s*(mut\s+)?$acts\s*[,|]" \
      --regexp "(^\s*(pub(\([^)]*\))?\s+)?|[(,]\s*)(mut\s+)?\b$acts\s*:" -- \
      "a binding, an argument or a field named for the act that made it."

    exclusion='(?i)(^|[^a-zA-Z])(shared|extended|seeds?)([^a-zA-Z]|$)'
    refuse . --glob 'crates/**/*.rs' \
      --regexp '^\s*let\s+(mut\s+)?[a-z0-9_]*[a-z]ed\s*[:=]' \
      --regexp '\|\s*(mut\s+)?[a-z0-9_]*[a-z]ed\s*[,|]' \
      --regexp '(^\s*(pub(\([^)]*\))?\s+)?|[(,]\s*)(mut\s+)?[a-z][a-z0-9_]*[a-z]ed\s*:' \
      --regexp '^\s*(pub(\([^)]*\))?\s+)?(const|static)\s+[A-Z][A-Z0-9_]*[A-Z]ED\s*:' -- \
      "a name in the past tense. Name what the value is, not what was done to it."
    exclusion=""

    refuse . --glob 'crates/**/*.rs' \
      --regexp '(^|[^a-zA-Z])(0[xXbBoO][0-9a-fA-F_]+|[0-9][0-9_]*)(u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize)([^a-zA-Z]|$)' -- \
      "a literal wearing its type. Name the value or put the type on the call."

    positions='(first|last|start|end|inside)'
    refuse . --glob 'crates/**/*.rs' \
      --regexp "(^\s*let\s+(mut\s+)?|^\s*(pub(\([^)]*\))?\s+)?|[(,]\s*(mut\s+)?)$positions\s*:" \
      --regexp "^\s*let\s+(mut\s+)?$positions\s*=" \
      --regexp '^\s*(pub(\([^)]*\))?\s+)?fn\s+(first|last|end|inside)\s*[(<]' \
      --regexp '^\s*(pub(\([^)]*\))?\s+)?(const|static)\s+(FIRST|LAST|START|END|INSIDE)\s*:' -- \
      "first, last, start, end and inside name a position and never a thing."

    counts='(two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|thousand|million)'
    refuse . --glob 'crates/**/*.rs' \
      --regexp "^\s*let\s+(mut\s+)?$counts\s*[:=]" \
      --regexp "(^\s*(pub(\([^)]*\))?\s+)?|[(,]\s*)(mut\s+)?$counts\s*:" -- \
      "a number counts. It never states what the value is."

    refuse . --glob 'crates/**' --ignore-case --regexp '(^|[^a-zA-Z])refus(e|es|ed|ing|al|als)([^a-zA-Z]|$)' -- \
      "a refusal in this code is an error. The type is Error and the field is error."

    if [ "$refusals" != 0 ]; then
      exit 1
    fi

    echo "[WORD] every rule of this repository was read, and none of them refused a line."
  '';
  installPhase = ''
    touch $out
  '';
}
