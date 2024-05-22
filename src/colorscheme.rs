#![allow(unused)]

use palette::Oklch;
use palette_gamut_mapping::gamut_map;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{self, Display, Formatter},
};

use crate::configuration::{Highlight, Kind, NamespacedThemeElement};
use crate::error::Error;

/// Non-linear sRGB.
#[derive(Debug, Clone, Copy)]
pub struct Srgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<Oklch<f32>> for Srgb {
    fn from(value: Oklch<f32>) -> Self {
        let rgb: palette::Srgb<f32> = gamut_map(value);
        let rgb: palette::Srgb<u8> = rgb.into_format();
        Srgb {
            red: rgb.red,
            green: rgb.green,
            blue: rgb.blue,
        }
    }
}

impl Display for Srgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Srgb { red, green, blue } = self;
        write!(f, "#{red:02x}{green:02x}{blue:02x}")
    }
}

/// A handle to the colors of a theme.
pub struct Theme<'a> {
    pub(crate) colors: BTreeMap<&'a str, BTreeMap<&'a str, Srgb>>,
}

impl Theme<'_> {
    pub fn get_color(&self, theme_element: NamespacedThemeElement) -> Option<Srgb> {
        self.colors
            .get(theme_element.theme_namespace)?
            .get(theme_element.element_name)
            .copied()
    }
}

/// A Neovim colorscheme. It consists of a light theme and a dark theme. These themes contain
/// colors. The highlights refer to elements in these themes.
pub struct Colorscheme<'a> {
    pub(crate) name: &'a str,
    pub(crate) kind: Kind,
    pub(crate) light_theme: Theme<'a>,
    pub(crate) dark_theme: Theme<'a>,
    pub(crate) highlights: BTreeMap<&'a str, Highlight<'a>>,
}

impl<'a> Colorscheme<'a> {
    /// The name of the colorscheme.
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Whether the colorscheme is specified as a light theme or dark theme. The inverse theme is
    /// generated automatically.
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// The colors of the light theme.
    pub fn light_theme(&self) -> &Theme {
        &self.light_theme
    }

    /// The colors of the dark theme.
    pub fn dark_theme(&self) -> &Theme {
        &self.dark_theme
    }

    pub fn get_color(&self, theme_element: NamespacedThemeElement<'_>) -> Option<Srgb> {
        match self.kind {
            Kind::Light => self.light_theme.get_color(theme_element),
            Kind::Dark => self.dark_theme.get_color(theme_element),
        }
    }

    pub fn get_inverse_color(&self, theme_element: NamespacedThemeElement<'_>) -> Option<Srgb> {
        match self.kind {
            Kind::Light => self.dark_theme.get_color(theme_element),
            Kind::Dark => self.light_theme.get_color(theme_element),
        }
    }

    /// An iterator over the highlight groups.
    pub fn highlights(&self) -> impl Iterator<Item = (&str, Highlight)> {
        self.highlights
            .iter()
            .map(|(&group, highlight)| (group, *highlight))
    }

    /// Get a highlight group by name.
    pub fn highlight(&self, group: &str) -> Option<Highlight<'a>> {
        self.highlights.get(group).copied()
    }
}

/// Reduce all links to a depth of 1 (i.e., after this transformation, all groups that are linked
/// to by a group, do not themselves link to a group).
fn reduce_link_depth<'a>(highlights: &mut BTreeMap<&'a str, Highlight<'a>>) -> Result<(), Error> {
    let mut new_links = HashMap::new();
    let mut link_stack = Vec::new();

    fn find_last_link<'a>(
        mut link: &'a str,
        highlights: &BTreeMap<&'a str, Highlight<'a>>,
        link_stack: &mut Vec<&'a str>,
    ) -> Result<&'a str, Error> {
        loop {
            if link_stack.contains(&link) {
                return Err(Error::LinkCycle(link.to_owned()));
            }
            link_stack.push(link);
            if let Some(linked) = highlights.get(link) {
                if let Some(next_link) = linked.link {
                    link = next_link;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        link_stack.clear();
        Ok(link)
    }

    for (group, highlight) in highlights.iter() {
        if let Some(link) = highlight.link {
            let last = find_last_link(link, highlights, &mut link_stack)?;
            if last != link {
                new_links.insert(*group, last);
            }
        }
    }

    for (group, new_link) in new_links {
        highlights.get_mut(group).unwrap().link = Some(new_link);
    }

    Ok(())
}

/// Parse and transform a `hi.nvim.rs` color scheme TOML configuration into a Neovim color scheme
/// specification.
pub fn parse<'a>(colorscheme_config: &'a str) -> Result<Colorscheme<'a>, Error> {
    let config = {
        let mut config = crate::configuration::parse(&colorscheme_config)?;
        reduce_link_depth(&mut config.highlights);
        config
    };

    let mut theme: BTreeMap<_, BTreeMap<_, _>> = BTreeMap::new();
    let mut inverse_theme: BTreeMap<_, BTreeMap<_, _>> = BTreeMap::new();

    // calculate all theme element colors (for both the normal and inverse themes)
    for (&theme_namespace, elements) in config.themes.0.iter() {
        for (&element_name, element) in elements.0.iter() {
            let color = config
                .get_color(element.0.color)
                .ok_or_else(|| Error::ColorMissing(format!("{}", element.0.color)))?;

            let color = element.0.modifiers.apply(color, |c| config.get_color(c))?;
            let mut inverse_color = color;
            inverse_color.l = 1. - inverse_color.l;

            inverse_color = config
                .inverse
                .modifiers
                .apply(inverse_color, |color| config.get_color(color))?;

            theme
                .entry(theme_namespace)
                .or_default()
                .insert(element_name, Srgb::from(color));
            inverse_theme
                .entry(theme_namespace)
                .or_default()
                .insert(element_name, Srgb::from(inverse_color));
        }
    }

    let (light_theme, dark_theme) = match config.kind {
        Kind::Light => (
            Theme { colors: theme },
            Theme {
                colors: inverse_theme,
            },
        ),
        Kind::Dark => (
            Theme {
                colors: inverse_theme,
            },
            Theme { colors: theme },
        ),
    };

    // check whether all referenced theme elements exist
    for (_, highlight) in &config.highlights {
        if let Some(fg) = highlight.fg {
            if light_theme.get_color(fg).is_none() {
                return Err(Error::ThemeElementMissing(format!("{}", fg)));
            }
        }
        if let Some(bg) = highlight.bg {
            if light_theme.get_color(bg).is_none() {
                return Err(Error::ThemeElementMissing(format!("{}", bg)));
            }
        }
    }

    Ok(Colorscheme {
        name: config.name,
        kind: config.kind,
        light_theme,
        dark_theme,
        highlights: config.highlights,
    })
}
