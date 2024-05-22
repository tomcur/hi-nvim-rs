/// Convert colors between color spaces with CSS Color Module Level 4's gamut mapping algorithm.
use palette::{
    color_difference::EuclideanDistance,
    convert::FromColorUnclamped,
    num::{Arithmetics, One, PartialCmp, Real, Sqrt, Zero},
    Clamp, Darken, IsWithinBounds, Lighten, Oklab, Oklch,
};

/// Calculate the perceptual error between two colors. This follows the CSS Color Module Level 4
/// DeltaE OK specification.
///
/// This calculates the euclidean distance of the two colors in Oklab space.
///
/// See:
/// https://web.archive.org/web/20230714163953/https://drafts.csswg.org/css-color/#color-difference-OK
#[inline]
fn delta_error_ok<T, C1, C2>(reference: C1, sample: C2) -> T
where
    Oklab<T>: EuclideanDistance<Scalar = T>,
    Oklab<T>: FromColorUnclamped<C1>,
    Oklab<T>: FromColorUnclamped<C2>,
    T: Clone + Arithmetics + Real + Sqrt,
{
    let reference = Oklab::from_color_unclamped(reference);
    let sample = Oklab::from_color_unclamped(sample);
    reference.distance(sample)
}

/// Map a color from a source color space to a target color space by performing a perceptual gamut
/// mapping. This follows the CSS Color Module Level 4 binary search specification.
///
/// The mapping proceeds by first mapping the target to the perceptual Oklch color space. Then the
/// chromacity is reduced until the color (almost) fits in the target color space. The gamut is
/// reduced only as much as is necessary for the *clipped* target to be perceptually
/// indistinguishable from the true color (as measured in Oklab space).
///
/// See:
/// https://web.archive.org/web/20230714163953/https://drafts.csswg.org/css-color/#binsearch
pub fn gamut_map<Source, Target, T>(source: Source) -> Target
where
    Source: Copy,
    Oklch<T>: FromColorUnclamped<Source>,
    Target: Copy
        + Default
        + FromColorUnclamped<Oklch<T>>
        + Clamp
        + IsWithinBounds<Mask = bool>
        + Lighten<Scalar = T>
        + Darken<Scalar = T>,
    Oklab<T>: FromColorUnclamped<Target> + FromColorUnclamped<Oklch<T>>,
    T: Copy + From<f32> + Arithmetics + Sqrt + Real + Zero + One + PartialCmp<Mask = bool>,
{
    // Perhaps there's a nice way to know at compile-time that the target colorspace is boundless,
    // if so we can short-circuit here.

    // `inGamut` from the spec specifies HSL and HWB are checked against sRGB's gamut. Here we just
    // check is_within_bounds. In Palette this (currently) corresponds to sRGB's gamut.

    let origin = Oklch::from_color_unclamped(source);

    if origin.l.lt_eq(&T::zero()) {
        return Target::default().darken(T::one());
    } else if origin.l.gt_eq(&T::one()) {
        return Target::default().lighten(T::one());
    }

    {
        let target = Target::from_color_unclamped(origin);
        if target.is_within_bounds() {
            return target;
        }
    }

    let two = T::one() + T::one();

    let mut min = T::zero();
    let mut min_in_gamut = true;
    let mut max = origin.chroma;
    let mut current = origin;

    let just_noticeable_distance = T::from(0.02);
    let epsilon = T::from(0.000_1);

    loop {
        if !(max - min).gt(&epsilon) {
            // We clamp here as this can be reached in the first iteration of the loop, so
            // `!current_in_gamut` might not be reached. I'm not sure it's necessary.
            break Target::from_color_unclamped(current).clamp();
        }

        current.chroma = (min + max) / two;

        let target = Target::from_color_unclamped(current);
        let current_in_gamut = target.is_within_bounds();

        if min_in_gamut && current_in_gamut {
            min = current.chroma;
            continue;
        }

        if !current_in_gamut {
            let clipped = target.clamp();
            let err = delta_error_ok(clipped, current);
            if err.lt(&just_noticeable_distance) {
                if (just_noticeable_distance - err).lt(&epsilon) {
                    break clipped;
                }

                min_in_gamut = false;
                min = current.chroma;
            } else {
                max = current.chroma;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use palette::{
        color_difference::EuclideanDistance, convert::FromColorUnclamped, Hsl, LinSrgb, Oklab,
        Oklch, Srgb,
    };
    use std::str::FromStr;

    #[test]
    fn test_roundtrip_srgb_is_original() {
        let colors = [
            ("red", LinSrgb::new(1.0, 0.0, 0.0)),
            ("green", LinSrgb::new(0.0, 1.0, 0.0)),
            ("cyan", LinSrgb::new(0.0, 1.0, 1.0)),
            ("magenta", LinSrgb::new(1.0, 0.0, 1.0)),
            ("black", LinSrgb::new(0.0, 0.0, 0.0)),
            ("grey", LinSrgb::new(0.5, 0.5, 0.5)),
            ("yellow", LinSrgb::new(1.0, 1.0, 0.0)),
            ("blue", LinSrgb::new(0.0, 0.0, 1.0)),
            ("white", LinSrgb::new(1.0, 1.0, 1.0)),
        ];

        for (name, color) in colors {
            let mapped: Oklab = super::gamut_map(color);
            let roundtrip: LinSrgb = super::gamut_map(mapped);

            let original_oklab = Oklab::from_color_unclamped(color);
            let roundtrip_oklab = Oklab::from_color_unclamped(roundtrip);

            assert!(
                roundtrip_oklab.distance(original_oklab) < 0.0001,
                "'{}' failed.\n{:?}\n!=\n{:?}",
                name,
                roundtrip,
                color
            );
        }
    }

    #[test]
    fn test_bright_and_dark() {
        let colors = [
            ("bright", Oklch::new(1.01, 0., 0.)),
            ("bright", Oklch::new(1.01, 0.1, 0.)),
            ("bright", Oklch::new(1.01, 0., 100.)),
            ("dark", Oklch::new(-0.01, 0., 0.)),
            ("dark", Oklch::new(-0.01, 0.1, 0.)),
            ("dark", Oklch::new(-0.01, 0., 100.)),
        ];

        for (bright_or_dark, color) in colors {
            let srgb: Srgb = super::gamut_map(color);
            let hsl: Hsl = super::gamut_map(color);

            const EPSILON: f32 = 1e-8;

            assert!(hsl.saturation.abs() < EPSILON);

            match bright_or_dark {
                "bright" => {
                    assert!(srgb.distance(Srgb::new(1., 1., 1.)) < EPSILON);
                    assert!((hsl.lightness - 1.).abs() < EPSILON);
                }
                "dark" => {
                    assert!(srgb.distance(Srgb::new(0., 0., 0.)) < EPSILON);
                    assert!(hsl.lightness.abs() < EPSILON);
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_out_of_gamut() {
        // TODO: find reference test values.

        let color = Oklch::new(0.5, 0.205, 230.);
        let expected: Srgb<f32> = Srgb::<u8>::from_str("#006d91").unwrap().into_format();

        const EPSILON: f32 = 0.05;

        let srgb: Srgb = super::gamut_map(color);
        assert!(
            srgb.distance(expected) < EPSILON,
            "failed.\n{:?}\n!=\n{:?}",
            expected,
            srgb
        );
    }
}
