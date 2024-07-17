use std::fmt::Write;

use anyhow::Result;

use crate::colorscheme::{Colorscheme, Theme};
use crate::configuration::Highlight;
use crate::error::Error;

/// Check whether a highlight group name is a legal name for Vim.
/// This corresponds to the regexp [a-zA-Z0-9_].
fn vim_legal_name(name: &str) -> bool {
    for char_ in name.chars() {
        if !char_.is_ascii_alphanumeric() && char_ != '_' {
            return false;
        }
    }

    true
}

struct Compiler<'c> {
    colorscheme: &'c Colorscheme<'c>,
    program: String,
    indent: u8,
}

impl<'c> Compiler<'c> {
    fn indent(&mut self) {
        self.indent = self.indent.saturating_add(1);
    }

    fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    fn write_indent(&mut self) {
        self.program += &str::repeat(" ", usize::from(self.indent).saturating_mul(4));
    }

    fn compile_highlight_group(
        &mut self,
        name: &str,
        mut highlight: Highlight<'c>,
        theme: &Theme,
    ) -> Result<()> {
        self.write_indent();

        if let Some(link) = highlight.link {
            if vim_legal_name(link) {
                writeln!(self.program, "hi! link {name} {link}")?;
                return Ok(());
            } else {
                // Can't link because the target won't be output. Instead, statically lift the
                // target's content to the current group.
                highlight = self.colorscheme.highlight(link).unwrap();
            }
        }

        write!(self.program, "hi {name} term=NONE cterm=NONE")?;
        if let Some(fg) = highlight.fg {
            let color = theme.get_color(fg).unwrap();
            write!(self.program, " guifg={color}",)?;
        } else {
            write!(self.program, " guifg=NONE")?;
        }

        if let Some(bg) = highlight.bg {
            let color = theme.get_color(bg).unwrap();
            write!(self.program, " guibg={color}",)?;
        } else {
            write!(self.program, " guibg=NONE")?;
        }

        if let Some(bg) = highlight.sp {
            let color = theme.get_color(bg).unwrap();
            write!(self.program, " guisp={color}",)?;
        } else {
            write!(self.program, " guisp=NONE")?;
        }

        write!(self.program, " gui=")?;
        {
            let mut some = false;
            for (idx, style) in highlight.gui_styles_iter().enumerate() {
                some = true;
                if idx > 0 {
                    write!(self.program, ",")?;
                }
                write!(self.program, "{}", style)?;
            }
            if !some {
                write!(self.program, "NONE")?;
            }
        }

        writeln!(self.program)?;

        Ok(())
    }

    pub fn compile_highlight_groups(&mut self, theme: &Theme) -> Result<()> {
        for (name, highlight) in &self.colorscheme.highlights {
            if !vim_legal_name(name) {
                continue;
            }

            self.compile_highlight_group(name, *highlight, theme)?;
        }

        Ok(())
    }
}

/// Compile a color scheme to a Neovim configuration.
// This should probably be infallible.
pub fn compile(colorscheme: &Colorscheme) -> std::result::Result<String, Error> {
    let mut compiler = Compiler {
        colorscheme,
        program: String::with_capacity(8192),
        indent: 0,
    };

    writeln!(
        compiler.program,
        r#"hi clear
set termguicolors
let g:colors_name = "{name}"
"#,
        name = colorscheme.name
    )
    .map_err(|_| Error::CompilationFailed)?;

    writeln!(compiler.program, r#"if &background == "light""#)
        .map_err(|_| Error::CompilationFailed)?;
    compiler.indent();
    compiler
        .compile_highlight_groups(&colorscheme.light_theme)
        .map_err(|_| Error::CompilationFailed)
        .map_err(|_| Error::CompilationFailed)?;
    writeln!(compiler.program, "else").map_err(|_| Error::CompilationFailed)?;
    compiler
        .compile_highlight_groups(&colorscheme.dark_theme)
        .map_err(|_| Error::CompilationFailed)?;
    write!(compiler.program, "end").map_err(|_| Error::CompilationFailed)?;
    compiler.dedent();

    Ok(compiler.program)
}
