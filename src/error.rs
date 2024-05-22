use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not parse configuration file: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Invalid Kind specified. Valid values are `light` and `dark`")]
    InvalidKind,
    #[error("A color referenced by a theme element is missing: {0}")]
    ColorMissing(String),
    #[error("A theme element referenced by a highlight group is missing: {0}")]
    ThemeElementMissing(String),
    #[error("A link cycle was detected. This highlight group is part of the cycle: {0}")]
    LinkCycle(String),
    #[error("Color scheme compilation failed for an unknown reason")]
    CompilationFailed,
}
