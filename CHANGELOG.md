# Changelog

## [0.3.0] - 2024-12-17

### Other
- Migrate to the Color crate
([f1c188d](https://github.com/tomcur/hi-nvim-rs/commit/f1c188da9877ed197b02fcfd01c94c77cadfa8b9))
- Remove accidental write
([1b22000](https://github.com/tomcur/hi-nvim-rs/commit/1b220006cb1f20a5b899babdcd8a3ec246dced0e))
- Bump input flakes
([267bd2a](https://github.com/tomcur/hi-nvim-rs/commit/267bd2a50bdf1f8708a34cd7c2d3a50cd2b29519))


## [0.2.0] - 2024-07-20

### Breaking
- [**breaking**] generalize `bg` and `fg` to named color groups
([e7f59c4](https://github.com/tomcur/hi-nvim-rs/commit/e7f59c42dd9ef8f28f8318bca74d6c8c30dad0cf))
- [**breaking**] define default pmenu scrollbar/thumb in terms of existing theme elements
([57a1250](https://github.com/tomcur/hi-nvim-rs/commit/57a1250da619fcce9796725d42e39bf4cb005ade))


### Features
- add Verf color scheme
([ff51589](https://github.com/tomcur/hi-nvim-rs/commit/ff51589823a87f7ff302ce43d3333e62d15062da) [2ccb37e](https://github.com/tomcur/hi-nvim-rs/commit/2ccb37e9d86ecad09dcf3129c4d2f8f913520a7e) [8611148](https://github.com/tomcur/hi-nvim-rs/commit/8611148ebd8d4a51a8e55f4b8e533a41eef875de))
- tweak border and status line colors
([a354cc5](https://github.com/tomcur/hi-nvim-rs/commit/a354cc5615aa63670320081fc8f06e10f973e1b0))
- add default highlights for Trouble
([8a73b60](https://github.com/tomcur/hi-nvim-rs/commit/8a73b608b1ce38cf98e385e14dc0f0bd1c68431e))
- tweak Highlow ([3bc4738](https://github.com/tomcur/hi-nvim-rs/commit/3bc473876a4aa9a8a26067cb094ad983b31160e6) [b6c64f9](https://github.com/tomcur/hi-nvim-rs/commit/b6c64f9a8f3fa89e674af63305669611e0c42a51) [3abdc61](https://github.com/tomcur/hi-nvim-rs/commit/3abdc619776712e33cfdacf9c500304fc39c1a01))
  - make dark version darker, define in terms of accented/unaccented groups
  - increase Highlow dark version saturation
  - increase Highlow's fg/bg contrast, decrease fg/fg contrast
- use lighten_absolute in Highlow and Twocolor
([75ac649](https://github.com/tomcur/hi-nvim-rs/commit/75ac649c70b020148d76199e7670e86a6e8d0098))
- tweak Twocolor, use second color for inlines, increase foreground saturation ([0e91bd3](https://github.com/tomcur/hi-nvim-rs/commit/0e91bd34f59aebab1dfc3e01aae5c14df6e0cf64) [da5c977](https://github.com/tomcur/hi-nvim-rs/commit/da5c97730d8be113756fd0c9486bf97516c2ae64))
- set EndOfBuffer default highlight as NonText
([c25c2d2](https://github.com/tomcur/hi-nvim-rs/commit/c25c2d20f99dc77d055a03b8e79f9f441217afb9))
- add @module default highlight
([12f2ae4](https://github.com/tomcur/hi-nvim-rs/commit/12f2ae4136a3f7e4d5c4a94407d3457c6309f08e))
- improve all color schemes' floating window colors
([7fad20a](https://github.com/tomcur/hi-nvim-rs/commit/7fad20a197adee88cdd21030ca1887c30479f158))
- add default Telescope highlights
([5e304b1](https://github.com/tomcur/hi-nvim-rs/commit/5e304b144e08a8f64667141872153d49170869dc))
- improve grayscale color scheme background colors
([1de07ac](https://github.com/tomcur/hi-nvim-rs/commit/1de07acd56d4b0511ea1683ce0da2b43079fb2f3))
- add Spell highlights to default highlight groups
([538893a](https://github.com/tomcur/hi-nvim-rs/commit/538893a88b039605bdca7604a220b6190db38fb6))
- add support for special colors (`guisp`)
([a496554](https://github.com/tomcur/hi-nvim-rs/commit/a4965543e67d4004a8db5db7394f5ecbe23896e3))
- tweak grayscale color scheme
([7382a8a](https://github.com/tomcur/hi-nvim-rs/commit/7382a8ad48e9f83bfc8967ba419096a705b68729))
- *(web)* use Light/Dark previews instead of Normal/Inverse
([08a550e](https://github.com/tomcur/hi-nvim-rs/commit/08a550e9f77fb2a3abe328dac9801d3bf616ea10))


### Bug Fixes
- correct docs (see also: a4965543e67d4004a8db5db7394f5ecbe23896e3)
([bd20f8d](https://github.com/tomcur/hi-nvim-rs/commit/bd20f8dac6a96373214c66dd6482574dd9761876))
- emit underline attribute in color schemes
([9ec4f1a](https://github.com/tomcur/hi-nvim-rs/commit/9ec4f1ad54562548978bf05b3e8fc7df95ed8b42))


### Documentation
- improve preview image
([bf93981](https://github.com/tomcur/hi-nvim-rs/commit/bf93981fda77371b36243796fc0c4fdc569bedbb))
- *(README)* generate and show big preview image
([7e88b5e](https://github.com/tomcur/hi-nvim-rs/commit/7e88b5e0e1dcea2ce6d85c216a5c6fe59848e88e))
- also generate bigger screenshots showing off more features
([42d1a7f](https://github.com/tomcur/hi-nvim-rs/commit/42d1a7f3635ea87510dacc084b77c859b1291911))
- improve screenshot generation script
([0d01703](https://github.com/tomcur/hi-nvim-rs/commit/0d017034c43108d96795690bc506c83affc395be))
- *(CHANGELOG)* add links to generated changelog
([db2f812](https://github.com/tomcur/hi-nvim-rs/commit/db2f812b30faf1b450775c11a7afa7d3c6f7da5b))
- add nvim to dev shell as required for docs
([d14a8d4](https://github.com/tomcur/hi-nvim-rs/commit/d14a8d45df7deef2842ae6aeb4e5dcc5235dffdd))
- improve Termsnap invocation to correctly render italics
([ccdc5d1](https://github.com/tomcur/hi-nvim-rs/commit/ccdc5d1d61806f4cefc6953264afc7d856fb3510))
- use treesitter in color scheme screenshots
([7e9a39b](https://github.com/tomcur/hi-nvim-rs/commit/7e9a39b2cc6aaa20137d35852b2cfa5010948707))


### Refactor
- create combined SVG by nesting, rather than embedding base64
([ecd4b77](https://github.com/tomcur/hi-nvim-rs/commit/ecd4b77dc543896d97c93d56e8d359af99303a7e))
- default highlights for plugins into separate files
([724c9e7](https://github.com/tomcur/hi-nvim-rs/commit/724c9e730ba19617f8946194a2037c1bf09de0ec))
- construct default highlights at build-time
([130b6ef](https://github.com/tomcur/hi-nvim-rs/commit/130b6ef71b73bffca936ab3a000d6c4bf65c225d))
- remove unnecessary clones
([8b5624f](https://github.com/tomcur/hi-nvim-rs/commit/8b5624f9183540ea2af41377bbcade29a02b16d5))


### Build System and CI
- allow release-plz to trigger actions
([94770b2](https://github.com/tomcur/hi-nvim-rs/commit/94770b275fc2c8759c633ed9a6d92da50ff8f742))


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

