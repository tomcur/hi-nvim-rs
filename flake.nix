{
  description = "(Neo)vim perceptual color scheme compiler";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      rec {
        packages.hi-nvim-rs = pkgs.rustPlatform.buildRustPackage {
          pname = "hi-nvim-rs";
          version = (pkgs.lib.trivial.importTOML ./Cargo.toml).package.version;
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
        packages.hi-nvim-rs-web = pkgs.rustPlatform.buildRustPackage {
          pname = "hi-nvim-rs-web";
          version = (pkgs.lib.trivial.importTOML ./hi-nvim-rs-web/Cargo.toml).package.version;
          src = ./.;
          buildAndTestSubdir = "hi-nvim-rs-web";
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
        packages.default = packages.hi-nvim-rs;
        devShells.default = pkgs.mkShell
          {
            buildInputs = with pkgs; [
              cargo
              clippy
              rust-analyzer
              rustc
              rustfmt
            ];
          };
      }
    );
}
