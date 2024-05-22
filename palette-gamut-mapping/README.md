<div align="center">

# Gamut mapping

**Map between color spaces**

[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)
[![Apache 2.0](https://img.shields.io/badge/license-Apache-blue.svg)](#license)
[![Crates.io](https://img.shields.io/crates/v/palette-gamut-mapping.svg)](https://crates.io/crates/palette-gamut-mapping)
[![Docs](https://docs.rs/palette-gamut-mapping/badge.svg)](https://docs.rs/palette-gamut-mapping)

</div>

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
