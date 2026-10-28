{
  description = "knop-buzzer";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    # Voeg de officiële Rust overlay toe voor componenten zoals rust-src
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      rust-overlay,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Configureer de exacte Rust toolchain met de vereiste componenten en targets
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
          targets = [ "riscv32imc-unknown-none-elf" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "button-box-env";

          nativeBuildInputs = with pkgs; [
            rustToolchain # Gebruik de uitgebreide toolchain in plaats van losse cargo/rustc
            probe-rs-tools
            pkg-config
            libiconv
          ];

          buildInputs = with pkgs; [
            udev
          ];

          shellHook = ''
            export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
            export LD_LIBRARY_PATH="${pkgs.xorg.libX11}/lib:$LD_LIBRARY_PATH"
          '';
        };
      }
    );
}
