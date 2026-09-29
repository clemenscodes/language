{
  callPackage,
  craneLib,
  lib,
  packages,
  rustToolchain,
  self,
  shells,
}:
let
  inherit (lib)
    any
    attrNames
    concatStringsSep
    hasPrefix
    hasSuffix
    removePrefix
    ;
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
  nix = cleanSourceWith {
    name = "nix";
    src = self;
    filter = path: type: type == "directory" || hasSuffix ".nix" path;
  };
  gates = {
    statix = callPackage ./statix { source = nix; };

    deadnix = callPackage ./deadnix { source = nix; };

    format = callPackage ./format { source = nix; };

    rustfmt =
      let
        source = sourceTree "rustfmt" [
          "Cargo.toml"
          "crates"
        ];
      in
      callPackage ./rustfmt {
        inherit source rustToolchain;
      };

    comment =
      let
        source = sourceTree "comment" [ "crates" ];
      in
      callPackage ./comment {
        inherit source;
      };

    test =
      let
        source = sourceTree "test" [ "crates" ];
      in
      callPackage ./test {
        inherit source;
      };

    word =
      let
        source = sourceTree "word" [
          "crates"
          "docs"
          "nix"
          "README.md"
        ];
      in
      callPackage ./word {
        inherit source;
      };

    punctuation =
      let
        source = sourceTree "punctuation" [
          ".moon"
          "crates"
          "docs"
          "nix"
          "book.toml"
          "Cargo.toml"
          "CLAUDE.md"
          "flake.nix"
          "moon.yml"
          "README.md"
        ];
      in
      callPackage ./punctuation {
        inherit source;
      };

    book =
      let
        source = sourceTree "book" [
          "book.toml"
          "crates/language/book.toml"
          "crates/language/docs"
          "docs"
        ];
      in
      callPackage ./book {
        inherit source;
      };

    language =
      let
        source = sourceTree "language" [
          ".config"
          "Cargo.lock"
          "Cargo.toml"
          "crates/language/Cargo.toml"
          "crates/language/src"
        ];
      in
      callPackage ./language {
        inherit craneLib source;
      };
  };
  checkNames =
    let
      lines = attrNames gates ++ [
        "outputs"
        "pipeline"
      ];
    in
    concatStringsSep "\n" lines + "\n";
  outputNames =
    let
      lines =
        map (name: "${name} package") (attrNames packages)
        ++ map (name: "${name} check") (
          attrNames gates
          ++ [
            "outputs"
            "pipeline"
          ]
        )
        ++ map (name: "${name} shell") shells;
    in
    concatStringsSep "\n" lines + "\n";
in
gates
// {
  outputs =
    let
      source = sourceTree "outputs" [ "docs/OUTPUTS.md" ];
    in
    callPackage ./outputs {
      names = outputNames;
      inherit source;
    };

  pipeline =
    let
      source = sourceTree "pipeline" [
        ".moon/tasks"
        "moon.yml"
        "nix/flake/checks/default.nix"
      ];
    in
    callPackage ./pipeline {
      names = checkNames;
      inherit source;
    };
}
