//! A Neovim color scheme generator using a perceptual color space.

mod colorscheme;
mod compiler_neovim;
mod compiler_vim;
mod configuration;
mod de;
mod default_highlights;
mod error;
mod gamut_map;
mod modifiers;

pub use colorscheme::{parse, Colorscheme, Rgb8, Theme};
pub use compiler_neovim::compile as compile_neovim;
pub use compiler_vim::compile as compile_vim;
pub use configuration::{Highlight, Kind, NamespacedThemeElement};
pub use default_highlights::DEFAULT_HIGHLIGHTS;
pub use error::Error;
