//! Default, pre-built color schemes.
use serde::Deserialize;
use std::fmt::{self, Display};

pub struct PrebuiltColorscheme {
    pub name: &'static str,
    pub css_variables: &'static str,
    pub config: &'static str,
    pub neovim_config: &'static str,
}

pub static HIGHLOW: &'static PrebuiltColorscheme = &PrebuiltColorscheme {
    name: "highlow",
    css_variables: include_str!(concat!(
        env!("OUT_DIR"),
        "/colorschemes/highlow/color_scheme.txt"
    )),
    config: include_str!("../../colorschemes/highlow.toml"),
    neovim_config: include_str!(concat!(
        env!("OUT_DIR"),
        "/colorschemes/highlow/neovim_config.vim"
    )),
};

pub static TWOCOLOR: &'static PrebuiltColorscheme = &PrebuiltColorscheme {
    name: "twocolor",
    css_variables: include_str!(concat!(
        env!("OUT_DIR"),
        "/colorschemes/twocolor/color_scheme.txt"
    )),
    config: include_str!("../../colorschemes/twocolor.toml"),
    neovim_config: include_str!(concat!(
        env!("OUT_DIR"),
        "/colorschemes/twocolor/neovim_config.vim"
    )),
};

#[derive(Clone, Copy, Default, Deserialize)]
pub enum Colorscheme {
    #[default]
    Highlow,
    Twocolor,
}

impl Display for Colorscheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Colorscheme::*;
        let s = match self {
            Highlow => "Highlow",
            Twocolor => "Twocolor",
        };
        f.write_str(s)
    }
}

impl markup::Render for Colorscheme {
    fn render(&self, writer: &mut impl fmt::Write) -> fmt::Result {
        write!(writer, "{}", self)
    }
}

impl Colorscheme {
    pub fn colorscheme(self) -> &'static PrebuiltColorscheme {
        use Colorscheme::*;
        match self {
            Highlow => HIGHLOW,
            Twocolor => TWOCOLOR,
        }
    }
}
