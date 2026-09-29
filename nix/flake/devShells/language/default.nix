{
  cargo-nextest,
  deadnix,
  mdbook,
  mkShellNoCC,
  moon,
  nixfmt-tree,
  ripgrep,
  rustToolchain,
  statix,
}:
mkShellNoCC {
  name = "language";

  packages = [
    cargo-nextest
    deadnix
    mdbook
    moon
    nixfmt-tree
    ripgrep
    rustToolchain
    statix
  ];

  RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
}
