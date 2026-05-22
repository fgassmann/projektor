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
          # This overlay adds the "rust-bin" package to nixpkgs
          (import rust-overlay)
        ];

        # System-specific nixpkgs with rust-overlay applied
        pkgs = import nixpkgs { inherit system overlays; };

        # Use the specific version of the Rust toolchain specified by the toolchain file
        # #.fromRustupToolchainFile ./rust-toolchain.toml;
        localRust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
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
