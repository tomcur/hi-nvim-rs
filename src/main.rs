//! A Neovim color scheme compiler using a perceptual color space.

use clap::{Parser, ValueEnum};
use std::{io::Read, path::PathBuf};

pub(crate) mod colorscheme;
pub(crate) mod compiler_neovim;
pub(crate) mod compiler_vim;
pub(crate) mod configuration;
pub(crate) mod de;
pub mod default_highlights;
pub(crate) mod error;
pub(crate) mod modifiers;

pub(crate) use default_highlights::DEFAULT_HIGHLIGHTS;

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Target {
    Neovim,
    Vim,
}

/// Compile a (Neo)vim color scheme from a hi.nvim.rs theme configuration.
///
/// The color scheme is written to standard output.
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    /// The compilation target. When Vim is chosen as target, highlight groups with unsupported
    /// names are removed. Supported characters in Vim correspond to the regexp [a-zA-Z0-9_].
    /// Neovim adds two characters, supporting regexp [a-zA-Z0-9_\.@]*.
    #[arg(short, long, value_name = "target", value_enum, default_value_t = Target::Neovim)]
    target: Target,

    /// Color scheme input file. Reads from standard input if not set.
    // TODO: perhaps accept multiple files, merging the configurations (allowing for base
    // configurations overridden by more specific configurations)
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = if let Some(file) = cli.file {
        std::fs::read_to_string(file)?
    } else {
        let mut config = String::with_capacity(16_384 /* 16 KiB */);
        std::io::stdin().read_to_string(&mut config).unwrap();
        config
    };

    let colorscheme = colorscheme::parse(&config)?;

    let program = match cli.target {
        Target::Neovim => compiler_neovim::compile(&colorscheme)?,
        Target::Vim => compiler_vim::compile(&colorscheme)?,
    };

    println!("{}", &program);

    Ok(())
}
