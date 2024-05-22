//! A Neovim color scheme generator using a perceptual color space.

mod colorscheme;
mod compiler_neovim;
mod compiler_vim;
mod configuration;
mod de;
mod error;
mod modifiers;

pub use colorscheme::{parse, Colorscheme, Srgb, Theme};
pub use compiler_neovim::compile as compile_neovim;
pub use compiler_vim::compile as compile_vim;
pub use configuration::{Highlight, NamespacedThemeElement};
pub use error::Error;
