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

        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          mesa
          xorg.libX11
          xorg.libXrandr
          xorg.libXcursor
          xorg.libXi
          xorg.libXinerama
          vulkan-loader
        ];

        iced-demo = rustPlatform.buildRustPackage {
          pname = "iced-demo";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = with pkgs; [ 
            libGL 
            libxkbcommon 
            wayland 
            mesa
            xorg.libX11
            xorg.libXrandr
            xorg.libXcursor
            xorg.libXi
            xorg.libXinerama
            vulkan-loader
          ];
          
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      in
      rec {
        packages = {
          iced-demo = iced-demo;
          default = iced-demo;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
            pkg-config
            openssl
            libGL
            libxkbcommon
            wayland
            mesa
            xorg.libX11
            xorg.libXrandr
            xorg.libXcursor
            xorg.libXi
            xorg.libXinerama
            vulkan-loader
          ];

          RUST_LOG = "debug";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          LD_LIBRARY_PATH = libPath;
        };
      }
    );
}
