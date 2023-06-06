{
  description = "Yarrbar flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ (import rust-overlay) ]; };
      in with pkgs; rec {
        devShell = mkShell rec {
          buildInputs = [
            rust-bin.stable.latest.default
            rust-analyzer

            freetype
            fontconfig
            openssl
            cmake
            dbus

            libxkbcommon
            libGL

            # WINIT_UNIX_BACKEND=wayland
            wayland

            # WINIT_UNIX_BACKEND=x11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11
          ];

          nativeBuildInputs = [
            systemd
            pkg-config
            rustPlatform.bindgenHook
          ];
          
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.getVersion clang}/include";
        };
      });
}
