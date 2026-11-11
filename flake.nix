{
  description = "(Neo)vim perceptual color scheme compiler";
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
    flake-utils.url = "github:numtide/flake-utils";
    termsnap = {
      url = "github:tomcur/termsnap";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };
  outputs = { nixpkgs, flake-utils, termsnap, ... }:
    let
      inherit (nixpkgs) lib;

      workspaceSrc = extraFiles: lib.fileset.toSource {
        root = ./.;
        fileset = lib.fileset.unions ([
          ./Cargo.toml
          ./Cargo.lock
          ./build.rs
          ./src
          ./default_highlights
          ./hi-nvim-rs-web
          ./hi-nvim-rs-web-styler
        ] ++ extraFiles);
      };

      buildHiNvimRs = pkgs: pkgs.rustPlatform.buildRustPackage {
        pname = "hi-nvim-rs";
        version = (lib.importTOML ./Cargo.toml).package.version;
        src = workspaceSrc [ ];
        cargoLock.lockFile = ./Cargo.lock;
      };

      # Compiles color schemes into a (Neo)vim plugin, with each color scheme
      # at `colors/{name}.vim`. The file name is taken from the `name` in the
      # configuration.
      #
      # Example:
      #
      # ```nix
      # hi-nvim-rs.lib.buildColorschemePlugin {
      #   inherit pkgs;
      #   pname = "my-colorschemes";
      #   colorschemeFiles = [ ./highlow.toml ./verf.toml ];
      # }
      # ```
      buildColorschemePlugin =
        { pkgs
        , pname
        , colorschemeFiles
        , version ? (buildHiNvimRs pkgs).version
        }:
        let
          hi-nvim-rs = buildHiNvimRs pkgs;
          colorschemes = map
            (file: { inherit file; name = (lib.importTOML file).name; })
            colorschemeFiles;
          names = map (c: c.name) colorschemes;
          esc = lib.escapeShellArg;
        in
        assert lib.assertMsg (lib.allUnique names)
          "buildColorschemePlugin: color scheme names must be unique, got: ${toString names}";
        pkgs.vimUtils.buildVimPlugin {
          inherit pname version;
          src = pkgs.runCommand "${pname}-src" { } (''
            mkdir -p "$out/colors" "$out/hi-nvim-rs/neovim" "$out/hi-nvim-rs/vim"
          '' + lib.concatMapStrings
            ({ file, name }: ''
              for target in neovim vim; do
                ${hi-nvim-rs}/bin/hi-nvim-rs --target "$target" ${esc "${file}"} \
                  > "$out/hi-nvim-rs/$target/"${esc "${name}.vim"}
              done
              cat > "$out/colors/"${esc "${name}.vim"} <<'EOF'
              " Load the colorscheme for vim/neovim.
              execute 'source' fnameescape(fnamemodify(resolve(expand('<sfile>:p')), ':h:h') . '/hi-nvim-rs/' . (has('nvim') ? 'neovim' : 'vim') . '/' . ${builtins.toJSON "${name}.vim"})
              EOF
            '')
            colorschemes);
        };

      # Compiles a color scheme to a single file.
      #
      # Example:
      #
      # ```nix
      # hi-nvim-rs.lib.buildColorscheme {
      #   inherit pkgs;
      #   name = "highlow";
      #   colorschemeFile = ./highlow.toml;
      #   target = "neovim";
      # }
      # ```
      buildColorscheme = { pkgs, name, colorschemeFile, target }:
        assert lib.assertOneOf "buildColorscheme: target" target [ "neovim" "vim" ];
        pkgs.runCommand name { } ''
          ${buildHiNvimRs pkgs}/bin/hi-nvim-rs --target ${target} ${lib.escapeShellArg "${colorschemeFile}"} > "$out"
        '';
    in
    {
      lib = { inherit buildColorschemePlugin buildColorscheme; };
    } // flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        hi-nvim-rs = buildHiNvimRs pkgs;

        # Neovim for `scripts/screenshots.sh`.
        screenshotNvim = pkgs.wrapNeovimUnstable pkgs.neovim-unwrapped {
          plugins = [
            (pkgs.vimPlugins.nvim-treesitter.withPlugins (p: [ p.rust ]))
            pkgs.vimPlugins.nvim-lspconfig
            pkgs.vimPlugins.nvim-tree-lua
            pkgs.vimPlugins.trouble-nvim
          ];
        };
      in
      {
        packages = {
          inherit hi-nvim-rs;
          default = hi-nvim-rs;

          hi-nvim-rs-web = pkgs.rustPlatform.buildRustPackage {
            pname = "hi-nvim-rs-web";
            version = (lib.importTOML ./hi-nvim-rs-web/Cargo.toml).package.version;
            src = workspaceSrc [ ./colorschemes ];
            buildAndTestSubdir = "hi-nvim-rs-web";
            cargoLock.lockFile = ./Cargo.lock;
          };

          # The color schemes in ./colorschemes bundled as a (Neo)vim plugin.
          hi-nvim-rs-colorschemes = buildColorschemePlugin {
            inherit pkgs;
            pname = "hi-nvim-rs-colorschemes";
            colorschemeFiles = lib.fileset.toList
              (lib.fileset.fileFilter (file: file.hasExt "toml") ./colorschemes);
          };
        };

        devShells.default = pkgs.mkShell {
          packages = (with pkgs; [
            cargo
            clippy
            rust-analyzer
            rustc
            rustfmt
          ]) ++ [
            # Automated screenshots
            termsnap.packages.${system}.default
          ];

          SCREENSHOT_NVIM = "${screenshotNvim}/bin/nvim";
        };
      }
    );
}
