//! `kcu_shade`: KColorUtils' shade, an offset of a colour's luma and chroma in KDE's HCY space.

use super::kde_color_common::{Rgba, enter, read, shade_q};

/// Shades `c` as `KColorUtils::shade` does: adds `luma` to its HCY luma and `chroma` to its
/// chroma, each clamped to `[0, 1]`, keeping its hue and alpha.
pub fn kcu_shade(c: Rgba, luma: f32, chroma: f32) -> Rgba {
    read(shade_q(enter(c), f64::from(luma), f64::from(chroma)))
}
