{
  description = "(Neo)vim perceptual color scheme compiler";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    termsnap = {
      url = "github:tomcur/termsnap";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { nixpkgs, flake-utils, termsnap, ... }:
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

        # Compiles a colorscheme. Use like
        #
        # ```nix
        # buildColorscheme {
        #   name = "highlow";
        #   colorschemeFile = ./highlow.toml;
        #   target = "neovim";
        # }
        # ```
        buildColorscheme = { name, colorschemeFile, target }:
          assert target == "neovim" || target == "vim";
          pkgs.stdenv.mkDerivation {
            name = name;

            dontUnpack = true;

            buildPhase = ''
              ${packages.hi-nvim-rs}/bin/hi-nvim-rs --target ${target} "${colorschemeFile}" > "${name}.vim"
            '';

            installPhase = ''
              cp "${name}.vim" $out
            '';
          };
        devShells.default = pkgs.mkShell
          {
            buildInputs = (with pkgs; [
              cargo
              clippy
              rust-analyzer
              rustc
              rustfmt
            ]) ++ [
              # Automated screenshots
              (pkgs.wrapNeovimUnstable pkgs.neovim-unwrapped {
                packpathDirs = {
                  myNeovimPackages.start = [
                    pkgs.vimPlugins.nvim-treesitter
                    pkgs.vimPlugins.nvim-treesitter-parsers.rust
                  ];
                };
              })
              termsnap.packages.${system}.default
            ];
          };
      }
    );
}
