use std::fmt::{self, Display};

use serde::Deserialize;

#[derive(Clone, Copy, Debug)]
pub struct CodeExample {
    pub code: &'static str,
    pub lines: &'static str,
    pub file_name: &'static str,
    pub language_name: &'static str,
}

#[derive(Clone, Copy, Default, Deserialize)]
pub enum Language {
    #[default]
    Rust,
    C,
    Typescript,
    #[serde(rename = "TSX")]
    Tsx,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        use Language::*;
        let s = match self {
            Rust => "Rust",
            C => "C",
            Typescript => "Typescript",
            Tsx => "TSX",
        };
        f.write_str(s)
    }
}

impl markup::Render for Language {
    fn render(&self, writer: &mut impl fmt::Write) -> fmt::Result {
        write!(writer, "{}", self)
    }
}

impl Language {
    pub const fn code_example(self) -> CodeExample {
        use Language::*;

        // The code examples are generated in the build.rs script
        match self {
            Rust => CodeExample {
                code: include_str!(concat!(env!("OUT_DIR"), "/examples/rust/highlighted.html")),
                lines: include_str!(concat!(env!("OUT_DIR"), "/examples/rust/lines.txt")),
                file_name: "example/src/main.rs",
                language_name: "rust",
            },
            C => CodeExample {
                code: include_str!(concat!(env!("OUT_DIR"), "/examples/c/highlighted.html")),
                lines: include_str!(concat!(env!("OUT_DIR"), "/examples/c/lines.txt")),
                file_name: "example/src/main.c",
                language_name: "c",
            },
            Typescript => CodeExample {
                code: include_str!(concat!(
                    env!("OUT_DIR"),
                    "/examples/typescript/highlighted.html"
                )),
                lines: include_str!(concat!(env!("OUT_DIR"), "/examples/typescript/lines.txt")),
                file_name: "example/src/index.ts",
                language_name: "typescript",
            },
            Tsx => CodeExample {
                code: include_str!(concat!(env!("OUT_DIR"), "/examples/tsx/highlighted.html")),
                lines: include_str!(concat!(env!("OUT_DIR"), "/examples/tsx/lines.txt")),
                file_name: "example/src/index.tsx",
                language_name: "tsx",
            },
        }
    }
}
