use color::{Oklab, Oklch, OpaqueColor};
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
        mut color: OpaqueColor<Oklch>,
        color_map: impl Fn(NamespacedColor) -> Option<OpaqueColor<Oklch>>,
    ) -> Result<OpaqueColor<Oklch>, Error> {
        if let Some((mix_with, factor)) = self.mix.as_ref() {
            let color2_lab = {
                let color2 = color_map(*mix_with)
                    .ok_or_else(|| Error::ColorMissing(format!("{}", mix_with)))?;
                color2.convert::<Oklab>()
            };
            let color_lab = color.convert::<Oklab>();
            color = color_lab.lerp_rect(color2_lab, *factor).convert();
        }

        if let Some(saturate) = self.saturate {
            color.components[1] += saturate;
        }

        if let Some(gamma) = self.gamma {
            color.components[0] = color.components[0].powf(gamma);
        }

        if let Some(lighten) = self.lighten {
            if lighten > 0. {
                color.components[0] += (1. - color.components[0]) * lighten;
            } else {
                color.components[0] -= color.components[0] * (1. - lighten);
            }
        }

        if let Some(lighten_multiplier) = self.lighten_multiplier {
            color.components[0] *= lighten_multiplier;
        }

        if let Some(lighten) = self.lighten_absolute {
            color.components[0] += lighten;
        }

        Ok(color)
    }
}
