use palette::{Darken, FromColor, Lighten, MixAssign, Oklab, Oklch};
use serde::Deserialize;

use crate::configuration::NamespacedColor;
use crate::error::Error;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ColorModifiers<'a> {
    pub saturate: Option<f32>,
    pub gamma: Option<f32>,
    pub lighten: Option<f32>,
    pub lighten_multiplier: Option<f32>,
    pub lighten_absolute: Option<f32>,
    #[serde(borrow)]
    pub mix: Option<(NamespacedColor<'a>, f32)>,
}

impl Default for ColorModifiers<'_> {
    fn default() -> ColorModifiers<'static> {
        ColorModifiers {
            saturate: None,
            gamma: None,
            lighten: None,
            lighten_multiplier: None,
            lighten_absolute: None,
            mix: None,
        }
    }
}

impl ColorModifiers<'_> {
    /// Apply the modifier to the color.
    ///
    /// The `color_map` parameter is a getter function for the theme colors. It is used to
    /// calculate the `blend` calculation.
    pub fn apply(
        &self,
        mut color: Oklch<f32>,
        color_map: impl Fn(NamespacedColor) -> Option<Oklch<f32>>,
    ) -> Result<Oklch<f32>, Error> {
        if let Some((mix_with, factor)) = self.mix.as_ref() {
            let color2_lab = {
                let color2 = color_map(*mix_with)
                    .ok_or_else(|| Error::ColorMissing(format!("{}", mix_with)))?;
                Oklab::from_color(color2)
            };
            let mut color_lab = Oklab::from_color(color);
            color_lab.mix_assign(color2_lab, *factor);

            color = Oklch::from_color(color_lab);
        }

        if let Some(saturate) = self.saturate {
            color.chroma += saturate;
        }

        if let Some(gamma) = self.gamma {
            color.l = color.l.powf(gamma);
        }

        if let Some(lighten) = self.lighten {
            if lighten > 0. {
                color = color.lighten(lighten);
            } else {
                color = color.darken(-lighten);
            }
        }

        if let Some(lighten_multiplier) = self.lighten_multiplier {
            color.l *= lighten_multiplier;
        }

        if let Some(lighten) = self.lighten_absolute {
            color.l += lighten;
        }

        Ok(color)
    }
}
