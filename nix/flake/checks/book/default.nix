{
  stdenvNoCC,
  mdbook,
  source,
}:
stdenvNoCC.mkDerivation {
  name = "book";
  src = source;
  nativeBuildInputs = [ mdbook ];
  dontConfigure = true;
  dontBuild = true;
  doCheck = true;
  checkPhase = ''
    find . -name book.toml | sort > "$TMPDIR/books.txt"
    count=$(wc -l < "$TMPDIR/books.txt")
    if [ "$count" -eq 0 ]; then
      echo "[BOOK] no book was found, the filter is wrong."
      exit 1
    fi
    echo "[BOOK] mdbook reads $count books."

    while read -r manifest; do
      book=$(dirname "$manifest")
      pages="$book/$(sed -n 's/^src *= *"\(.*\)"/\1/p' "$manifest")"
      echo "[BOOK] $book"

      find "$pages" -name '*.md' | sort > "$TMPDIR/first_listing.txt"

      html=$(mktemp --directory)
      mdbook build "$book" --dest-dir "$html"

      find "$pages" -name '*.md' | sort > "$TMPDIR/second_listing.txt"
      if ! diff "$TMPDIR/first_listing.txt" "$TMPDIR/second_listing.txt"; then
        echo "[BOOK] $book names a page that does not exist, mdbook made it up."
        exit 1
      fi

      grep --only-matching '(\./[^)]*\.md)' "$pages/SUMMARY.md" \
        | tr --delete '()' \
        | sed "s|^\./|$pages/|" \
        | sort > "$TMPDIR/summary_pages.txt"
      find "$pages" -name '*.md' -not -name SUMMARY.md | sort > "$TMPDIR/directory_pages.txt"
      if ! diff "$TMPDIR/summary_pages.txt" "$TMPDIR/directory_pages.txt"; then
        echo "[BOOK] $book leaves a page out of its summary, or names one twice."
        exit 1
      fi
    done < "$TMPDIR/books.txt"
  '';
  installPhase = ''
    touch $out
  '';
}
