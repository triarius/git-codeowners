{
  description = "A modern Rust project environment using Nix and Crane";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    systems.url = "github:nix-systems/default";
  };

  outputs = { self, nixpkgs, crane, systems, ... }:
    let
      forEachSystem = f: nixpkgs.lib.genAttrs (import systems) (system: f rec {
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
        src = let
          fixturesFilter = path: _type: builtins.match ".*/examples/.*" path != null;
          fixturesOrCargo = path: type: (fixturesFilter path type) || (craneLib.filterCargoSources path type);
        in pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = fixturesOrCargo;
        };
      });
    in
    {
      packages = forEachSystem ({ pkgs, craneLib, src }:
        let
          commonArgs = {
            inherit src;
            strictDeps = true;
            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [
              openssl
            ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            ];
          };

          # Build *only* the dependencies to cache them in the Nix store
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        in
        {
          default = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
          });
        }
      );

      devShells = forEachSystem ({ pkgs, craneLib, src }: {
        default = craneLib.devShell {
          inputsFrom = [ self.packages.${pkgs.system}.default ];
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustfmt
            clippy
          ];
          env = {
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
            RUST_BACKTRACE = "1";
          };
        };
      });
    };
}
