{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-25.05";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        nf = pkgs.rustPlatform.buildRustPackage {
          pname = "nf";
          version = "1.1";

          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
      in {
        packages.default = nf;

        devshells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            git
            cargo
          ];
        };
      }
    );
}
