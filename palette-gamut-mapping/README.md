# Gamut mapping

Convert [Palette](https://crates.io/crates/palette) colors between color spaces
with [CSS Color Module Level 4's gamut mapping
algorithm](https://www.w3.org/TR/css-color-4/#gamut-mapping).

```rust
use palette::{Oklch, Srgb};
use palette_gamut_mapping::gamut_map;

let color = Oklch::new(0.5, 0.205, 230.);

// roughly equal to #006d91
let srgb: Srgb = gamut_map(color);
```
