use std::borrow::Cow;

use hi_nvim_rs::{Colorscheme, Highlight};

pub enum Target {
    Neovim,
    Vim,
}

pub struct Compiled<'a> {
    pub name: &'a str,
    pub css_variables: String,
    pub compiled_colorscheme: String,
    pub colorscheme: Colorscheme<'a>,
}

fn style_group(
    mut w: impl std::fmt::Write,
    group: &str,
    highlight: Highlight<'_>,
    colorscheme: &Colorscheme,
) {
    let group = if group.contains("@") || group.contains(".") {
        // kind of inefficient
        Cow::Owned(group.replace("@", "\\@").replace(".", "\\."))
    } else {
        Cow::Borrowed(group)
    };

    let highlight = if let Some(link) = highlight.link {
        if let Some(highlight) = colorscheme.highlight(link) {
            highlight
        } else {
            return;
        }
    } else {
        highlight
    };

    if let Some(fg) = highlight.fg.and_then(|c| colorscheme.get_color(c)) {
        write!(w, "--{group}--fg: {fg}; ").unwrap();
    }
    if let Some(bg) = highlight.bg.and_then(|c| colorscheme.get_color(c)) {
        write!(w, "--{group}--bg: {bg}; ").unwrap();
    }
    if let Some(fg) = highlight.fg.and_then(|c| colorscheme.get_inverse_color(c)) {
        write!(w, "--{group}--inverse-fg: {fg}; ").unwrap();
    }
    if let Some(bg) = highlight.bg.and_then(|c| colorscheme.get_inverse_color(c)) {
        write!(w, "--{group}--inverse-bg: {bg}; ").unwrap();
    }
    if highlight.bold.unwrap_or(false) {
        write!(w, "--{group}--font-weight: 600; ").unwrap();
    }
    if highlight.italic.unwrap_or(false) {
        write!(w, "--{group}--font-style: italic; ").unwrap();
    }
    if highlight.underline.unwrap_or(false) {
        write!(w, "--{group}--text-decoration: underline; ").unwrap();
    } else if highlight.undercurl.unwrap_or(false) {
        write!(w, "--{group}--text-decoration: underline wavy; ").unwrap();
    } else if highlight.underdouble.unwrap_or(false) {
        write!(w, "--{group}--text-decoration: underline double; ").unwrap();
    } else if highlight.underdotted.unwrap_or(false) {
        write!(w, "--{group}--text-decoration: underline dotted; ").unwrap();
    } else if highlight.underdashed.unwrap_or(false) {
        write!(w, "--{group}--text-decoration: underline dashed; ").unwrap();
    } else if highlight.strikethrough.unwrap_or(false) {
        write!(w, "--{group}--text-decoration: line-through; ").unwrap();
    }
}

/// Parse the color scheme configuration and return a CSS style setting highlight values to
/// variables for the specified highlight groups.
///
/// Panics if a group is specified that is not present in the configuration.
pub fn style_groups<'a>(
    config: &'a str,
    groups: &[&str],
    target: Target,
) -> Result<Compiled<'a>, hi_nvim_rs::Error> {
    let colorscheme = hi_nvim_rs::parse(config)?;
    let compiled_colorscheme = match target {
        Target::Neovim => hi_nvim_rs::compile_neovim(&colorscheme)?,
        Target::Vim => hi_nvim_rs::compile_vim(&colorscheme)?,
    };

    // Pre-allocate some space. A guess.
    let mut css_variables = String::with_capacity((groups.len() * 32).next_power_of_two());

    for &group in groups {
        let highlight = colorscheme.highlight(group).unwrap();
        style_group(&mut css_variables, group, highlight, &colorscheme);
    }

    Ok(Compiled {
        name: colorscheme.name(),
        css_variables,
        compiled_colorscheme,
        colorscheme,
    })
}

/// Parse the color scheme configuration and return a CSS style setting highlight values to
/// variables for all highlight groups.
pub fn style(config: &str, target: Target) -> Result<Compiled, hi_nvim_rs::Error> {
    let colorscheme = hi_nvim_rs::parse(config)?;
    let compiled_colorscheme = match target {
        Target::Neovim => hi_nvim_rs::compile_neovim(&colorscheme)?,
        Target::Vim => hi_nvim_rs::compile_vim(&colorscheme)?,
    };

    // Pre-allocate some space. A guess.
    let mut css_variables = String::with_capacity(4096);

    for (group, highlight) in colorscheme.highlights() {
        style_group(&mut css_variables, group, highlight, &colorscheme);
    }

    Ok(Compiled {
        name: colorscheme.name(),
        css_variables,
        compiled_colorscheme,
        colorscheme,
    })
}
