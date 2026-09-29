{ inputs, ... }:
{
  systems = [ "x86_64-linux" ];

  perSystem =
    { system, ... }:
    let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [ inputs.rust-overlay.overlays.default ];
      };
      inherit (pkgs) callPackages;
      inherit (inputs) self;

      rustToolchain = pkgs.callPackage ./packages/rustToolchain { pinned = ../../rust-toolchain.toml; };
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

      formatter = pkgs.nixfmt-tree;

      packages = callPackages ./packages { inherit self; };

      devShells = callPackages ./devShells { inherit rustToolchain; };

      checks = callPackages ./checks {
        inherit
          craneLib
          packages
          self
          rustToolchain
          ;
        shells = builtins.attrNames devShells;
      };
    in
    {
      inherit
        formatter
        devShells
        packages
        checks
        ;
    };
}
