{
  description = "A Nix-flake-based Rust development environment";

  # GitHub URLs for the Nix inputs we're using
  inputs = {
    # Simply the greatest package repository on the planet
    nixpkgs.url = "github:NixOS/nixpkgs";
    # A set of helper functions for using flakes
    flake-utils.url = "github:numtide/flake-utils";
    # A utility library for working with Rust
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        localRust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
          targets = [
            "x86_64-unknown-linux-gnu"
            "x86_64-unknown-linux-musl"
          ];
        };
        # Other utilities commonly used in Rust projects (but not in this example project)
        others = with pkgs; [
          cargo-generate
          openssl
          pkg-config
        ];
      in {
        devShells = {
          default = pkgs.mkShell {
            # Packages included in the environment
            buildInputs = [ localRust ] ++ others;

            # Run when the shell is started up
            shellHook = ''
              ${localRust}/bin/cargo --version
              
              # For rust-analyzer 'hover' tooltips to work.
              export RUST_SRC_PATH="${localRust}/lib/rustlib/src/rust/library";
            '';
          };
        };
      });
}
