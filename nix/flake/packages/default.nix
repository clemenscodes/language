{
  callPackage,
  lib,
  self,
}:
let
  inherit (lib) any hasPrefix removePrefix;
  inherit (lib.sources) cleanSourceWith;
  sourceTree =
    name: paths:
    cleanSourceWith {
      inherit name;
      src = self;
      filter =
        path: _type:
        let
          relativePath = removePrefix "${self}/" (toString path);
        in
        any (prefix: hasPrefix prefix relativePath || hasPrefix relativePath prefix) paths;
    };
in
{
  book = callPackage ./book {
    name = "book";
    directory = ".";
    source = sourceTree "book" [
      "book.toml"
      "docs"
    ];
  };

  language-book = callPackage ./book {
    name = "language-book";
    directory = "crates/language";
    source = sourceTree "language-book" [
      "crates/language/book.toml"
      "crates/language/docs"
      "crates/language/README.md"
    ];
  };
}
