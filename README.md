<div align="center">

# hi.nvim.rs

**(Neo)vim perceptual color scheme compiler**

[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)
[![Apache 2.0](https://img.shields.io/badge/license-Apache-blue.svg)](#license)
[![Crates.io](https://img.shields.io/crates/v/hi-nvim-rs.svg)](https://crates.io/crates/hi-nvim-rs)
[![Docs](https://docs.rs/hi-nvim-rs/badge.svg)](https://docs.rs/hi-nvim-rs)

</div>

A tool to create (Neo)vim color schemes by defining colors and their
transformations. It uses the Oklab color space to make the perceptual effects
of transformations predictable.

A hosted version ~~is~~ will at some point be available
[here](https://hi-nvim-rs.uint.one).

## Feature summary

- Color schemes are compiled to (Neo)vim configurations with no startup overhead
- Specify colors using lightness, chromacity and hue in the Oklch color space
- A curated set of opinionated default Neovim highlight groups is provided,
  requiring only a few theme definitions to get a consistent color scheme
- Theme colors definitions refer to colors with optional transformations (such
  as lightness and chromacity)
- Override or add any highlight group
- Vim is supported as a secondary target

## Getting started

To run the CLI version, run

```shell
$ cargo install hi-nvim-rs
$ hi-nvim-rs --help
$ hi-nvim-rs ./path/to/colorscheme.toml > ~/.config/nvim/colors/a-colorscheme-name.vim
```
