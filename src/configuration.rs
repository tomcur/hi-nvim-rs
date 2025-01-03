//! Deserialization and initial transformation of hi.nvim.rs theme specifications.

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
};

use color::{Oklch, OpaqueColor};
use serde::Deserialize;

use crate::error::Error;
use crate::{de::string_or_struct, modifiers::ColorModifiers};

#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Light,
    Dark,
}

impl TryFrom<&str> for Kind {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        match value {
            "light" => Ok(Kind::Light),
            "dark" => Ok(Kind::Dark),
            _ => Err(Error::InvalidKind),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
struct Lch(f32, f32, f32);

impl From<Lch> for OpaqueColor<Oklch> {
    fn from(Lch(l, chroma, hue): Lch) -> Self {
        OpaqueColor::<Oklch>::new([l, chroma, hue])
    }
}

/// A Neovim highlight group.
///
/// The `fg`, `bg` and `sp` fields code for specific theme colors. If `Link` is set the highlight
/// group is linked to a different group, taking the color attributes from that group. The other
/// fields refer to the highlight `gui` attr-list.
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Highlight<'a> {
    /// A theme color to use as the foreground color (e.g., `syn.function`, `ui.fg_dim`).
    pub fg: Option<NamespacedThemeElement<'a>>,
    /// A theme color to use as the background color (e.g., `diff.removed`, `ui.bg_gutter`).
    pub bg: Option<NamespacedThemeElement<'a>>,
    /// A theme color to use as the special color (e.g., `diagnostics.error`).
    pub sp: Option<NamespacedThemeElement<'a>>,
    /// If set, this highlight group is linked to a different group, taking its colors. The other
    /// attributes will be ignored.
    pub link: Option<&'a str>,
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

impl Highlight<'_> {
    pub const fn empty() -> Self {
        Highlight {
            fg: None,
            bg: None,
            sp: None,
            link: None,
            bold: None,
            italic: None,
            underline: None,
            undercurl: None,
            underdouble: None,
            underdotted: None,
            underdashed: None,
            strikethrough: None,
            reverse: None,
            nocombine: None,
        }
    }

    #[rustfmt::skip]
    pub fn gui_styles_iter(&self) -> impl Iterator<Item = &'static str> {
        self.bold
            .into_iter()
            .map(|_| "bold")
            .chain(self.italic.into_iter().map(|_| "italic"))
            .chain(self.underline.into_iter().map(|_| "underline"))
            .chain(self.undercurl.into_iter().map(|_| "undercurl"))
            .chain(self.underdouble.into_iter().map(|_| "underdouble"))
            .chain(self.underdashed.into_iter().map(|_| "underdashed"))
            .chain(self.underdotted.into_iter().map(|_| "underdotted"))
            .chain(self.strikethrough.into_iter().map(|_| "strikethrough"))
            .chain(self.reverse.into_iter().map(|_| "reverse"))
            .chain(self.nocombine.into_iter().map(|_| "nocombine"))
    }
}

#[derive(Debug, Deserialize)]
struct HuesConfig {
    lightness: f32,
    chroma: f32,
}

/// The top-level configuration document.
#[derive(Debug, Deserialize)]
struct Configuration_<'a> {
    name: &'a str,
    kind: &'a str,
    inverse: Inverse<'a>,
    colors: HashMap<&'a str, Lch>,
    hues: HashMap<&'a str, f32>,
    groups: HashMap<&'a str, HuesConfig>,
    themes: ThemeNamespaces<'a>,
    highlights: BTreeMap<&'a str, Highlight<'a>>,
}

/// Specifier of inverse theme post-processing color modificiations.
#[derive(Debug, Deserialize)]
pub struct Inverse<'a> {
    #[serde(borrow, flatten)]
    pub modifiers: ColorModifiers<'a>,
}

#[derive(Deserialize, Debug)]
pub struct ThemeElementWrap<'a>(
    #[serde(borrow, deserialize_with = "string_or_struct")] pub ThemeElement<'a>,
);

// BTreeMaps to ensure theme content is sorted alphabetically. This helps to generate themes in
// deterministic order.
#[derive(Debug, Deserialize)]
pub struct ThemeElements<'a>(#[serde(borrow)] pub BTreeMap<&'a str, ThemeElementWrap<'a>>);

#[derive(Debug, Deserialize)]
pub struct ThemeNamespaces<'a>(#[serde(borrow)] pub BTreeMap<&'a str, ThemeElements<'a>>);

#[derive(Deserialize, Debug)]
pub struct ThemeElement<'a> {
    #[serde(borrow)]
    pub color: NamespacedColor<'a>,
    #[serde(flatten)]
    pub modifiers: ColorModifiers<'a>,
}

impl<'a> From<&'a str> for ThemeElement<'a> {
    fn from(s: &'a str) -> Self {
        let (namespace, color_name) = if let Some((namespace, color_name)) = s.split_once('.') {
            (ColorNamespace::Group(namespace), color_name)
        } else {
            (ColorNamespace::Colors, s)
        };

        let color = NamespacedColor {
            namespace,
            color_name,
        };

        ThemeElement {
            color,
            modifiers: ColorModifiers::default(),
        }
    }
}

/// The namespaces of colors.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ColorNamespace<'a> {
    Colors,
    Group(&'a str),
}

/// Colors in a color namespace (e.g. `fg.green` is `green` in the `fg` namespace).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamespacedColor<'a> {
    pub namespace: ColorNamespace<'a>,
    pub color_name: &'a str,
}

impl<'a> NamespacedColor<'a> {
    pub fn from_namespace_and_color_name(
        namespace: ColorNamespace<'a>,
        color_name: &'a str,
    ) -> Self {
        NamespacedColor {
            namespace,
            color_name,
        }
    }
}

impl Display for NamespacedColor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let NamespacedColor {
            namespace,
            color_name,
        } = self;

        match namespace {
            ColorNamespace::Colors => {}
            ColorNamespace::Group(group) => {
                f.write_str(group)?;
                f.write_str(".")?;
            }
        }

        f.write_str(color_name)
    }
}

/// An element in a theme namespace (e.g., `syn.type` is `type` in the `syn` namespace). This codes
/// for a specific color.
#[derive(Debug, Clone, Copy)]
pub struct NamespacedThemeElement<'a> {
    pub theme_namespace: &'a str,
    pub element_name: &'a str,
}

impl Display for NamespacedThemeElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let NamespacedThemeElement {
            theme_namespace,
            element_name,
        } = self;
        write!(f, "{theme_namespace}.{element_name}")
    }
}

#[derive(Debug)]
pub struct Configuration<'a> {
    pub name: &'a str,
    pub kind: Kind,
    pub inverse: Inverse<'a>,
    pub colors: HashMap<NamespacedColor<'a>, OpaqueColor<Oklch>>,
    pub themes: ThemeNamespaces<'a>,
    pub highlights: BTreeMap<&'a str, Highlight<'a>>,
}

impl Configuration<'_> {
    pub fn get_color(&self, namespaced_color: NamespacedColor) -> Option<OpaqueColor<Oklch>> {
        self.colors.get(&namespaced_color).copied()
    }
}

pub fn parse<'a>(config_file: &'a str) -> Result<Configuration<'a>, Error> {
    let config: Configuration_ = toml::from_str(config_file)?;

    let colors: HashMap<_, _> = config
        .colors
        .into_iter()
        .map(|(name, color)| {
            (
                NamespacedColor::from_namespace_and_color_name(ColorNamespace::Colors, name),
                color.into(),
            )
        })
        // generate all hue/color group combinations
        .chain(config.hues.iter().flat_map(|(&name, &hue)| {
            config.groups.iter().map(move |(group, group_config)| {
                (
                    NamespacedColor::from_namespace_and_color_name(
                        ColorNamespace::Group(group),
                        name,
                    ),
                    OpaqueColor::<Oklch>::new([group_config.lightness, group_config.chroma, hue]),
                )
            })
        }))
        .collect();

    let highlights = {
        let mut highlights = config.highlights;
        for (name, highlight) in crate::DEFAULT_HIGHLIGHTS {
            highlights.entry(name).or_insert(*highlight);
        }
        highlights
    };

    let config = Configuration {
        name: config.name,
        kind: config.kind.try_into()?,
        inverse: config.inverse,
        colors,
        themes: config.themes,
        highlights,
    };

    Ok(config)
}
