{
  description = "Roon TUI - Terminal UI for Roon";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    roon-cli.url = "github:EdgarPost/roon-cli";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, roon-cli }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "roon-tui";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = [ pkgs.makeWrapper ];

          postInstall = ''
            wrapProgram $out/bin/roon-tui \
              --prefix PATH : ${roon-cli.packages.${system}.default}/bin
          '';

          meta = with pkgs.lib; {
            description = "Terminal UI for Roon via roon-cli daemon";
            homepage = "https://github.com/EdgarPost/roon-tui";
            license = licenses.mit;
            maintainers = [ ];
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.cargo
            pkgs.rustfmt
            pkgs.clippy
            pkgs.rust-analyzer
            roon-cli.packages.${system}.default
          ];

          RUST_BACKTRACE = 1;
        };
      }
    );
}
