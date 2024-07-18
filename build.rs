use std::{env, fs, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Highlight {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub sp: Option<String>,
    /// If set, this highlight group is linked to a different group, taking its colors. The other
    /// attributes will be ignored.
    pub link: Option<String>,
    // GUI styles
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub undercurl: Option<bool>,
    pub underdouble: Option<bool>,
    pub underdotted: Option<bool>,
    pub underdashed: Option<bool>,
    pub strikethrough: Option<bool>,
    pub reverse: Option<bool>,
    pub nocombine: Option<bool>,
}

struct ThemeElement<'s>(&'s str);

impl std::fmt::Display for ThemeElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (namespace, element_name) = self.0.split_once('.').unwrap();
        write!(
            f,
            r#"NamespacedThemeElement {{ theme_namespace: "{namespace}", element_name: "{element_name}", }}"#
        )
    }
}

fn main() -> anyhow::Result<()> {
    use std::io::Write;

    println!("cargo::rerun-if-changed=./default_highlights");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("default_highlights.rs");
    let mut code = fs::File::create(dest_path)?;

    writeln!(
        code,
        "pub static DEFAULT_HIGHLIGHTS: &'static [(&'static str, Highlight<'static>)] = &["
    )?;
    for path in fs::read_dir("./default_highlights")? {
        let path = path?.path();
        if !path.is_file() {
            continue;
        }
        let default_highlights = std::fs::read_to_string(path)?;
        let table: toml::Table = toml::from_str(&default_highlights)?;

        for (highlight_group, value) in table {
            writeln!(code, "(")?;
            writeln!(code, r#""{highlight_group}","#)?;
            writeln!(code, "Highlight {{")?;

            for (key, value) in value
                .as_table()
                .expect("highlight value must be a TOML table")
            {
                match key.as_str() {
                    "fg" | "bg" | "sp" => {
                        let theme_element =
                            ThemeElement(value.as_str().expect("fg, bg and sp values must be str"));
                        writeln!(code, "    {key}: Some({theme_element}),")?;
                    }
                    "link" => {
                        let link = value.as_str().expect("link must be str");
                        writeln!(code, "    link: Some({link:?}),")?;
                    }
                    _ => {
                        let attr_value =
                            value.as_bool().expect("highlight attributes must be bool");
                        writeln!(code, "    {key}: Some({attr_value}),")?;
                    }
                }
            }

            writeln!(code, "    ..Highlight::empty()")?;
            writeln!(code, "}}")?;
            writeln!(code, "),")?;
        }
    }
    writeln!(code, "];")?;

    Ok(())
}
