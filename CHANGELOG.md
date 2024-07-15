# Changelog

## [0.1.5] - 2024-07-15

### Build System and CI

- Temporarily don't publish crates to test workflows
- Allow release-plz to trigger actions

## [0.1.4] - 2024-07-12

### Build System and CI

- (hopeful) fix uploading of compiled color schemes to release

## [0.1.3] - 2024-07-12

### Features

- Add Grayscale color scheme

### Bug Fixes

- *(highlights)* Add String group (@string links to it)

### Documentation

- *(README)* Add configuration explanation
- *(README)* Add Grayscale to color scheme gallery
- *(README)* Set example images' widths to ensure github doesn't squash them
- Use Termsnap for color scheme screenshots

### Refactor

- Make two lines in Rust example code shorter

### Build System and CI

- Add compiled color schemes to release

## [0.1.2] - 2024-06-12

### Bug Fixes

- *(web)* Clear error on success
- *(web)* Improve color scheme preview on small screens
- *(web)* Don't scale down to fit overflow on screen

### Other

- Bump input flakes

## [0.1.1] - 2024-06-10

### Features

- *(highlights)* Add nvim-dap-ui highlights
- *(highlights)* Add WinBar and WinBarNC
- *(web)* Allow specifying listen address and port as cli arguments

### Bug Fixes

- *(web)* Set correct name for default color scheme
- Use correct repo urls
- Typo
- Remove unnecessary file
- Add @variable to default highlight groups

### Documentation

- Improve explanations in highlow.toml
- Fix typo
- Hosted version is now available
- Improve wording
- Fix url typo
- *(hi.nvim.rs)* Add inverse theme generation to feature summary
- *(hi.nvim.rs)* Add screenshots of in-repo color schemes to README
- *(hi.nvim.rs)* Add badges

### Refactor

- Extract palette-gamut-mapping to its own repository

### Build System and CI

- Add buildColorscheme to Nix flake
- Add release-plz git-only note
- Add release-plz
- Add Rust build and test workflow
- Add hi-nvim-rs-web to Nix flake
- Read version from Cargo.toml
- Add package to Nix flake

### Other

- Bump input flakes

