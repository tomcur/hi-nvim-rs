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

        # The color schemes in ./colorschemes bundled as a (Neo)vim plugin.
        packages.vimPlugin = lib.buildColorschemePlugin {
          pname = "hi-nvim-rs-colorschemes";
          colorschemeFiles = pkgs.lib.mapAttrsToList (file: _: ./colorschemes + "/${file}")
            (pkgs.lib.filterAttrs (file: type: type == "regular" && pkgs.lib.hasSuffix ".toml" file)
              (builtins.readDir ./colorschemes));
        };

        # Compiles color schemes into a (Neo)vim plugin, with each color scheme
        # at `colors/{name}.vim`. The file name is taken from the `name` in the
        # configuration.
        #
        # Example:
        #
        # ```nix
        # lib.buildColorschemePlugin {
        #   pname = "my-colorschemes";
        #   colorschemeFiles = [ ./highlow.toml ./verf.toml ];
        # }
        # ```
        lib.buildColorschemePlugin =
          { pname
          , colorschemeFiles
          , version ? packages.hi-nvim-rs.version
          }:
          let
            names = map (file: (pkgs.lib.importTOML file).name) colorschemeFiles;
            esc = pkgs.lib.escapeShellArg;
          in
          assert pkgs.lib.assertMsg (pkgs.lib.allUnique names)
            "buildColorschemePlugin: color scheme names must be unique, got: ${toString names}";
          pkgs.vimUtils.buildVimPlugin {
            inherit pname version;
            src = pkgs.runCommand "${pname}-src" { } (''
              mkdir -p "$out/colors" "$out/hi-nvim-rs/neovim" "$out/hi-nvim-rs/vim"
            '' + pkgs.lib.concatStrings (pkgs.lib.zipListsWith
              (file: name: ''
                for target in neovim vim; do
                  ${packages.hi-nvim-rs}/bin/hi-nvim-rs --target "$target" ${esc "${file}"} \
                    > "$out/hi-nvim-rs/$target/"${esc "${name}.vim"}
                done
                cat > "$out/colors/"${esc "${name}.vim"} <<'EOF'
                " Load the colorscheme for vim/neovim.
                execute 'source' fnameescape(fnamemodify(resolve(expand('<sfile>:p')), ':h:h') . '/hi-nvim-rs/' . (has('nvim') ? 'neovim' : 'vim') . '/' . ${builtins.toJSON "${name}.vim"})
                EOF
              '')
              colorschemeFiles
              names));
          };

        # Compiles a colorscheme to a single file.
        #
        # Example:
        #
        # ```nix
        # lib.buildColorscheme {
        #   name = "highlow";
        #   colorschemeFile = ./highlow.toml;
        #   target = "neovim";
        # }
        # ```
        lib.buildColorscheme = { name, colorschemeFile, target }:
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
        devShells.default =
          let
            nvim = (pkgs.wrapNeovimUnstable pkgs.neovim-unwrapped {
              plugins = [
                (pkgs.vimPlugins.nvim-treesitter.withPlugins (p: [ p.rust ]))
                pkgs.vimPlugins.nvim-lspconfig
                pkgs.vimPlugins.nvim-tree-lua
                pkgs.vimPlugins.trouble-nvim
              ];
            });
          in
          pkgs.mkShell
            {
              buildInputs = (with pkgs; [
                cargo
                clippy
                rust-analyzer
                rustc
                rustfmt
              ]) ++ [
                # Automated screenshots
                termsnap.packages.${system}.default
              ];

              SCREENSHOT_NVIM = "${nvim}/bin/nvim";
            };
      }
    );
}
