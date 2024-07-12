use std::borrow::Cow;
use std::env;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use itertools::Itertools;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

/// Tuple of (treesitter higlight, neovim highlight group)
// perhaps Neovim treesitter highlight groups can be used directly (but all would have to be linked
// in the config file)
static HIGHLIGHT_NAMES: &[(&str, &str)] = &[
    ("attribute", "@tag.attribute"),
    ("constant", "Constant"),
    ("comment", "Comment"),
    ("function.builtin", "@function.builtin"),
    ("function", "Function"),
    ("keyword", "Keyword"),
    ("operator", "Operator"),
    ("property", "Property"),
    ("punctuation", "Delimiter"),
    ("string", "@string"),
    ("tag", "@tag"),
    ("type", "Type"),
    ("variable", "Identifier"),
];

/// The known Neovim highlight groups. These will be styled.
// Groups specified in HIGHLIGHT_NAMES will be appended.
static KNOWN_NEOVIM_HIGHLIGHT_GROUPS: &[&str] = &["LineNr", "MsgArea", "StatusLine"];

static COLOR_SCHEMES: &[(&'static str, &'static str)] = &[
    ("highlow", include_str!("../colorschemes/highlow.toml")),
    ("twocolor", include_str!("../colorschemes/twocolor.toml")),
    ("grayscale", include_str!("../colorschemes/grayscale.toml")),
];

fn highlight_html_into_write(
    highlighter: &mut Highlighter,
    config: &HighlightConfiguration,
    code: &[u8],
    mut w: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let highlights = highlighter.highlight(config, code, None, |_| None).unwrap();

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                for &char in code[start..end].iter() {
                    match tree_sitter_highlight::util::html_escape(char) {
                        Some(esc) => w.write_all(esc)?,
                        None => w.write_all(&[char])?,
                    };
                }
                // w.write_all(&code[start..end])?;
            }
            HighlightEvent::HighlightStart(s) => {
                let class = HIGHLIGHT_NAMES[s.0].1;
                write!(w, r#"<span class="{class}">"#)?;
            }
            HighlightEvent::HighlightEnd => {
                write!(w, "</span>")?;
            }
        }
    }

    Ok(())
}

fn lines_into_write(
    mut w: impl std::io::Write,
    lines: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    for line in 1..=lines {
        writeln!(w, "{line}")?;
    }

    Ok(())
}

fn styles_to_write(mut w: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    for &group in KNOWN_NEOVIM_HIGHLIGHT_GROUPS
        .iter()
        .chain(HIGHLIGHT_NAMES.iter().map(|(_, g)| g).unique())
    {
        let group = if group.contains("@") || group.contains(".") {
            // kind of inefficient
            Cow::Owned(group.replace("@", "\\@").replace(".", "\\."))
        } else {
            Cow::Borrowed(group)
        };

        writeln!(
            w,
            r#"
.{group} {{
    color: var(--{group}--fg);
    background-color: var(--{group}--bg);
    font-weight: var(--{group}--font-weight);
    font-style: var(--{group}--font-style);
    text-decoration: var(--{group}--text-decoration);
}}

.preview.inverse .{group} {{
    color: var(--{group}--inverse-fg);
    background-color: var(--{group}--inverse-bg);
}}
"#
        )?;
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut highlighter = Highlighter::new();

    let langs_and_configs = &mut [
        (
            "rust",
            include_str!("./code-examples/rust.rs"),
            HighlightConfiguration::new(
                tree_sitter_rust::language(),
                tree_sitter_rust::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap(),
        ),
        (
            "c",
            include_str!("./code-examples/c.c"),
            HighlightConfiguration::new(
                tree_sitter_c::language(),
                tree_sitter_c::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap(),
        ),
        (
            "typescript",
            include_str!("./code-examples/typescript.ts"),
            {
                let highlights: String = [
                    tree_sitter_javascript::HIGHLIGHT_QUERY,
                    tree_sitter_typescript::HIGHLIGHT_QUERY,
                ]
                .into_iter()
                .collect();
                let locals: String = [
                    tree_sitter_javascript::LOCALS_QUERY,
                    tree_sitter_typescript::LOCALS_QUERY,
                ]
                .into_iter()
                .collect();

                HighlightConfiguration::new(
                    tree_sitter_typescript::language_typescript(),
                    &highlights,
                    "",
                    &locals,
                )
            }
            .unwrap(),
        ),
        (
            "tsx",
            include_str!("./code-examples/typescript.tsx"),
            {
                let highlights: String = [
                    tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
                    tree_sitter_javascript::HIGHLIGHT_QUERY,
                    tree_sitter_typescript::HIGHLIGHT_QUERY,
                ]
                .into_iter()
                .collect();
                let locals: String = [
                    tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
                    tree_sitter_javascript::LOCALS_QUERY,
                    tree_sitter_typescript::LOCALS_QUERY,
                ]
                .into_iter()
                .collect();

                HighlightConfiguration::new(
                    tree_sitter_typescript::language_tsx(),
                    &highlights,
                    "",
                    &locals,
                )
            }
            .unwrap(),
        ),
    ];

    let highlight_names: Vec<_> = HIGHLIGHT_NAMES.iter().map(|(h, _)| h).collect();

    for (_, _, highlight_config) in langs_and_configs.iter_mut() {
        highlight_config.configure(&highlight_names);
    }

    // Write site theme
    {
        static GROUPS: &[&str] = &[
            "Normal",
            "NormalFloat",
            "FloatBorder",
            "Visual",
            "DiagnosticError",
            "DiagnosticInfo",
            "WinSeparator",
        ];
        let config = include_str!("../colorschemes/highlow.toml");
        let hi_nvim_rs_web_styler::Compiled {
            css_variables,
            colorscheme,
            ..
        } = hi_nvim_rs_web_styler::style_groups(
            config,
            GROUPS,
            hi_nvim_rs_web_styler::Target::Neovim,
        )
        .expect("parsed color scheme");

        let mut theme_file = {
            let path: PathBuf = [&out_dir, "site_color_scheme.txt"].into_iter().collect();
            std::fs::File::create(path)?
        };
        theme_file.write_all(css_variables.as_bytes())?;

        let mut svg_file = {
            let path: PathBuf = [&out_dir, "favicon.svg"].into_iter().collect();
            std::fs::File::create(path)?
        };
        write!(
            svg_file,
            r#"
                <svg  version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                    <rect width="25" height="100" style="fill:{one}"/>
                    <rect x="25" width="25" height="100" style="fill:{two}"/>
                    <rect x="50" width="25" height="100" style="fill:{three}"/>
                    <rect x="75" width="25" height="100" style="fill:{four}"/>
                    <rect y="75" width="50" height="25" style="fill:{five}"/>
                    <rect x="50" y="75" width="50" height="25" style="fill:{six}"/>
                </svg>
            "#,
            one = colorscheme
                .get_color(hi_nvim_rs::NamespacedThemeElement {
                    theme_namespace: "ui",
                    element_name: "bg"
                })
                .unwrap(),
            two = colorscheme
                .get_color(hi_nvim_rs::NamespacedThemeElement {
                    theme_namespace: "ui",
                    element_name: "bg_gutter"
                })
                .unwrap(),
            three = colorscheme
                .get_color(hi_nvim_rs::NamespacedThemeElement {
                    theme_namespace: "ui",
                    element_name: "bg_highlight_dim"
                })
                .unwrap(),
            four = colorscheme
                .get_color(hi_nvim_rs::NamespacedThemeElement {
                    theme_namespace: "ui",
                    element_name: "bg_visual"
                })
                .unwrap(),
            five = colorscheme
                .get_color(hi_nvim_rs::NamespacedThemeElement {
                    theme_namespace: "ui",
                    element_name: "fg_very_dim"
                })
                .unwrap(),
            six = colorscheme
                .get_color(hi_nvim_rs::NamespacedThemeElement {
                    theme_namespace: "ui",
                    element_name: "fg"
                })
                .unwrap(),
        )?;
    }

    // Write default color schemes
    {
        let mut path: PathBuf = [out_dir.as_str(), "colorschemes"].into_iter().collect();
        for (name, config) in COLOR_SCHEMES {
            path.push(name);
            std::fs::create_dir_all(&path)?;

            let hi_nvim_rs_web_styler::Compiled {
                compiled_colorscheme,
                css_variables,
                ..
            } = hi_nvim_rs_web_styler::style(config, hi_nvim_rs_web_styler::Target::Neovim)
                .expect("parsed color scheme");

            {
                path.push("color_scheme.txt");
                let mut color_scheme_file = std::fs::File::create(&path)?;
                color_scheme_file.write_all(css_variables.as_bytes())?;
                path.pop();
            }

            {
                path.push("neovim_config.vim");
                let mut neovim_config_file = std::fs::File::create(&path)?;
                neovim_config_file.write_all(compiled_colorscheme.as_bytes())?;
                path.pop();
            }

            path.pop();
        }
    }

    // Write CSS style groups
    {
        let mut styles_file = {
            let path: PathBuf = [&out_dir, "groups.css"].into_iter().collect();
            std::fs::File::create(path)?
        };
        let mut buf = BufWriter::new(&mut styles_file);
        styles_to_write(&mut buf)?;
        buf.flush()?;
    }

    // Write example code with highlight groups, and line numbers
    {
        for (lang_name, code, highlight_config) in langs_and_configs {
            let mut path: PathBuf = [out_dir.as_str(), "examples", lang_name]
                .into_iter()
                .collect();
            std::fs::create_dir_all(&path)?;

            {
                path.push("lines.txt");
                let num_lines = code.chars().filter(|c| *c == '\n').count();
                let mut lines_file = std::fs::File::create(&path)?;
                let mut buf = BufWriter::new(&mut lines_file);
                lines_into_write(&mut buf, num_lines)?;
                buf.flush()?;
                path.pop();
            }

            {
                path.push("highlighted.html");
                let mut highlighted_code_file = { std::fs::File::create(&path)? };
                let mut buf = BufWriter::new(&mut highlighted_code_file);
                highlight_html_into_write(
                    &mut highlighter,
                    &highlight_config,
                    code.as_bytes(),
                    &mut buf,
                )?;
                buf.flush()?;
                path.pop();
            }
        }
    }

    Ok(())
}
