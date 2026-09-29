{
  craneLib,
  source,
}:
let
  workspace = craneLib.crateNameFromCargoToml { cargoToml = "${source}/Cargo.toml"; };
  cargoArtifacts = craneLib.buildDepsOnly {
    inherit (workspace) version;
    pname = "language";
    src = source;
    strictDeps = true;
  };
in
craneLib.cargoNextest {
  inherit cargoArtifacts;
  inherit (workspace) version;
  pname = "language";
  src = source;
  strictDeps = true;
  env.NEXTEST_SHOW_PROGRESS = "counter";
  cargoNextestExtraArgs = "--profile release --no-tests=fail";
}
