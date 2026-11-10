//! Generate light themes from dark themes and vice-versa.
//!
//! This flips the perceived lightness of every color. Hue and chroma are kept unchanged. Lightness
//! is measured with Okhsl's lightness estimate [`toe`].
//!
//! A color with lightness estimate `L_r` becomes `1 - L_r`. If the inverse theme's background
//! lightness is set explicitly, colors keep their lightness difference to the background instead.

use color::{Oklch, OpaqueColor};

use crate::okhsl::{toe, toe_inv};

/// Inversion of theme colors.
#[derive(Clone, Copy, Debug)]
pub struct Inversion {
    /// The lightness estimate (see [`toe`]) of the main background.
    bg: f32,
    /// The lightness estimate of the inverse theme's main background.
    inverse_bg: f32,
}

impl Inversion {
    /// `bg` is the Oklab lightness of the main background, and `inverse_bg` that of the inverse
    /// theme.
    ///
    /// If `inverse_bg` is `None`, `1 - L_r` is used (see [`toe`]).
    pub fn new(bg: f32, inverse_bg: Option<f32>) -> Self {
        let bg = toe(bg.clamp(0., 1.));
        let inverse_bg = match inverse_bg {
            Some(inverse_bg) => toe(inverse_bg.clamp(0., 1.)),
            None => 1. - bg,
        };
        Inversion { bg, inverse_bg }
    }

    /// Invert Oklab lightness `l`.
    pub fn lightness(&self, l: f32) -> f32 {
        let l = toe(l.clamp(0., 1.));
        // Keep the lightness difference to the background, but on the other side of it.
        toe_inv((self.inverse_bg + self.bg - l).clamp(0., 1.))
    }

    /// Invert a color.
    pub fn invert(&self, color: OpaqueColor<Oklch>) -> OpaqueColor<Oklch> {
        let [l, c, h] = color.components;
        OpaqueColor::new([self.lightness(l), c.max(0.), h])
    }
}
