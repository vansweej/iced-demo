{
  description = "A flake for building a Rust workspace using buildRustPackage.";

  inputs = {
    rust-overlay.url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.2054";
    flake-utils.url = "https://flakehub.com/f/numtide/flake-utils/0.1.102";
    nixpkgs.follows = "rust-overlay/nixpkgs";
  };

  outputs =
    inputs:
    with inputs;
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustVersion = "latest";
        rustToolchain = pkgs.rust-bin.stable.${rustVersion}.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        common = {
          version = "0.0.1";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.pkg-config ];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };

        dot-analyse-rs = rustPlatform.buildRustPackage (
          common
          // {
            pname = "dot-analyse-rs";
            cargoBuildFlags = "-p dot-analyse-rs";
          }
        );

        recipe-grep = rustPlatform.buildRustPackage (
          common
          // {
            pname = "recipe-grep";
            cargoBuildFlags = "-p recipe_grep";
          }
        );

        recipe-neighbour = rustPlatform.buildRustPackage (
          common
          // {
            pname = "recipe-neighbour";
            cargoBuildFlags = "-p recipe-neighbour";
          }
        );
      in
      rec {
        packages = {
          dot-analyse-rs = dot-analyse-rs;
          recipe-grep = recipe-grep;
          recipe-neighbour = recipe-neighbour;
          all = pkgs.symlinkJoin {
            name = "all";
            paths = [
              dot-analyse-rs
              recipe-grep
              recipe-neighbour
            ];
          };
          default = packages.all;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.cargo-deny
            pkgs.cargo-edit
            pkgs.cargo-tarpaulin
            pkgs.cargo-watch
            pkgs.cargo-outdated
            pkgs.cargo-update
            pkgs.git
            pkgs.openssl
            pkgs.pkg-config
            pkgs.rust-analyzer
          ];

          env = {
            # Required by rust-analyzer
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
        };
      }
    );
}
