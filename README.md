<div align="center">

# hi.nvim.rs

**(Neo)vim perceptual color scheme compiler**

[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)
[![Apache 2.0](https://img.shields.io/badge/license-Apache-blue.svg)](#license)
[![Crates.io](https://img.shields.io/crates/v/hi-nvim-rs.svg)](https://crates.io/crates/hi-nvim-rs)
[![Docs](https://docs.rs/hi-nvim-rs/badge.svg)](https://docs.rs/hi-nvim-rs)

</div>

Create (Neo)vim color schemes by defining colors and their transformations.
This uses the Oklab color space to make the perceptual effects of
transformations predictable.

A hosted version ~~is~~ will at some point be available
[here](https://hi-nvim-rs.uint.one).

## Feature summary

- Color schemes are compiled to (Neo)vim configurations with no startup overhead
- Specify colors using lightness, chromacity and hue in the Oklch color space
- If you're creating a dark theme, you get the inverse light theme for free, and vice versa
- A curated set of opinionated default Neovim highlight groups is provided,
  requiring only a few theme definitions to get a consistent color scheme
- Theme colors definitions refer to colors with optional transformations (such
  as lightness and chromacity)
- Override or add any highlight group
- Vim is supported as a secondary target

## Examples

| Normal | Inverse |
|---|---|
| ![A screenshot of Neovim using the "highlow" light color scheme](./media/highlow-light.png) | ![A screenshot of Neovim using the "highlow" dark color scheme](./media/highlow-dark.png) |

[_highlow_](./colorschemes/highlow.toml): a color scheme with high contrast between background and foreground,
low color saturation, and low contrast between foreground elements.

| Normal | Inverse |
|---|---|
| ![A screenshot of Neovim using the "twocolor" dark color scheme](./media/twocolor-dark.png) | ![A screenshot of Neovim using the "twocolor" light color scheme](./media/twocolor-light.png) |

[_twocolor_](./colorschemes/twocolor.toml): a color scheme using (mostly) just two hues.

## Getting started

To run the CLI version, run

```shell
$ cargo install hi-nvim-rs
$ hi-nvim-rs --help
$ hi-nvim-rs ./path/to/colorscheme.toml > ~/.config/nvim/colors/a-colorscheme-name.vim
```
