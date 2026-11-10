//! Okhsl's lightness estimate.
//!
//! Okhsl's lightness is Oklab's lightness corrected to more closely match CIELAB's L*: (Oklab's
//! lightness is less accurate for dark colors).
//!
//! This is a port of the lightness estimate ("toe") of Ottosson's reference implementation
//! (<https://bottosson.github.io/misc/ok_color.h>), which is:
//!
//! > Copyright(c) 2021 Björn Ottosson
//! >
//! > Permission is hereby granted, free of charge, to any person obtaining a copy of
//! > this softwareand associated documentation files(the "Software"), to deal in
//! > the Software without restriction, including without limitation the rights to
//! > use, copy, modify, merge, publish, distribute, sublicense, and /or sell copies
//! > of the Software, and to permit persons to whom the Software is furnished to do
//! > so, subject to the following conditions :
//! > The above copyright noticeand this permission notice shall be included in all
//! > copies or substantial portions of the Software.
//! > THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! > IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! > FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
//! > AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! > LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! > OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! > SOFTWARE.

const TOE_K1: f32 = 0.206;
const TOE_K2: f32 = 0.03;
const TOE_K3: f32 = (1. + TOE_K1) / (1. + TOE_K2);

/// Okhsl's lightness estimate `L_r` from Oklab lightness `L`.
///
/// Both range from 0 (black) to 1 (white).
pub fn toe(l: f32) -> f32 {
    0.5 * (TOE_K3 * l - TOE_K1
        + ((TOE_K3 * l - TOE_K1) * (TOE_K3 * l - TOE_K1) + 4. * TOE_K2 * TOE_K3 * l).sqrt())
}

/// The inverse of [`toe`]: Oklab lightness `L` from Okhsl's lightness estimate `L_r`.
pub fn toe_inv(l_r: f32) -> f32 {
    (l_r * l_r + TOE_K1 * l_r) / (TOE_K3 * (l_r + TOE_K2))
}

#[cfg(test)]
mod tests {
    use super::{toe, toe_inv};

    #[test]
    fn toe_roundtrip() {
        for i in 0..=100 {
            let l = i as f32 / 100.;
            assert!((toe_inv(toe(l)) - l).abs() < 1e-5);
        }
    }

    #[test]
    fn toe_keeps_black_and_white() {
        assert!(toe(0.).abs() < 1e-6);
        assert!((toe(1.) - 1.).abs() < 1e-5);
    }
}
