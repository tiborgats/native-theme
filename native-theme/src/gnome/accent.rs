//! libadwaita's standalone accent colour, `--accent-color`.
//!
//! libadwaita derives the standalone colour from the accent background
//! (`--accent-bg-color`, the portal's accent) in the Oklab colour space: the
//! lightness is capped at 0.5 for a light background and raised to 0.85 for a
//! dark one, `a` and `b` kept (`--standalone-color-oklab`, libadwaita 1.10.0
//! `src/stylesheet/_colors.scss:146`, `:151`, `:168-170`;
//! `doc/css-variables.md:54-99`). Its focus ring is that colour at 50 %
//! (`$focus_border_color`, `_colors.scss:260`, `:307`; `outline-color`,
//! `_drawing.scss:20`).
//!
//! This is a port of libadwaita's own C routine for it, `adw_rgba_to_standalone`
//! (`src/adw-accent-color.c:130-157`), with its Oklab conversions
//! (`src/adw-color-utils.c:73-159`), in `f32` throughout where the C runs its
//! gamma steps in double (`pow`, `:73-89`) and cubes with `powf (l, 3)`; the
//! documented colours below come out exactly all the same. The colour is converted
//! to Oklab, its lightness clamped, converted back, and each channel clamped to
//! `[0, 1]` (the gamut clipping `doc/css-variables.md:88-99` describes). The
//! documented accent table (`doc/css-variables.md:101-230`) is its test vector.

use crate::Rgba;

/// sRGB transfer function, encoded to linear (`unapply_gamma`).
fn to_linear(v: f32) -> f32 {
    if v >= 0.04045 {
        ((v + 0.055) / (1.0 + 0.055)).powf(2.4)
    } else {
        v / 12.92
    }
}

/// sRGB transfer function, linear to encoded (`apply_gamma`).
fn to_encoded(v: f32) -> f32 {
    if v > 0.003_130_8 {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    } else {
        12.92 * v
    }
}

/// `adw_rgb_to_oklab`: an sRGB colour, each channel in `[0, 1]`, as Oklab.
fn rgb_to_oklab(red: f32, green: f32, blue: f32) -> (f32, f32, f32) {
    let (r, g, b) = (to_linear(red), to_linear(green), to_linear(blue));
    let l = (0.412_221_47 * r + 0.536_332_55 * g + 0.051_445_993 * b).cbrt();
    let m = (0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b).cbrt();
    let s = (0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b).cbrt();
    (
        0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s,
        1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s,
        0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s,
    )
}

/// `adw_oklab_to_rgb`: an Oklab colour as sRGB, unclamped.
fn oklab_to_rgb(lightness: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let l = lightness + 0.396_337_78 * a + 0.215_803_76 * b;
    let m = lightness - 0.105_561_346 * a - 0.063_854_17 * b;
    let s = lightness - 0.089_484_18 * a - 1.291_485_5 * b;
    let (l, m, s) = (l * l * l, m * m * m, s * s * s);
    (
        to_encoded(4.076_741_7 * l - 3.307_711_6 * m + 0.230_969_94 * s),
        to_encoded(-1.268_438 * l + 2.609_757_4 * m - 0.341_319_38 * s),
        to_encoded(-0.004_196_086_3 * l - 0.703_418_6 * m + 1.707_614_7 * s),
    )
}

/// libadwaita's standalone colour for the accent background `accent`, on a
/// dark background or a light one (`adw_rgba_to_standalone`), with the
/// accent's alpha.
#[must_use]
pub(crate) fn standalone(accent: Rgba, dark: bool) -> Rgba {
    let [red, green, blue, alpha] = accent.to_f32_array();
    let (lightness, a, b) = rgb_to_oklab(red, green, blue);
    let lightness = if dark {
        lightness.max(0.85)
    } else {
        lightness.min(0.5)
    };
    let (r, g, b) = oklab_to_rgb(lightness, a, b);
    Rgba::from_f32(r, g, b, alpha)
}

/// libadwaita's focus ring for the accent background `accent`:
/// `$focus_border_color`, the standalone colour at 50 %
/// (`color-mix(in srgb, var(--accent-color) 50%, transparent)`,
/// `_colors.scss:260`, `:307`), or under `prefers-contrast: more`
/// `$focus_border_color_hc`, at 80 % (`_colors.scss:261`, `:308`;
/// `_drawing.scss:24-26`).
#[must_use]
pub(crate) fn focus_ring(accent: Rgba, dark: bool, high_contrast: bool) -> Rgba {
    let s = standalone(accent, dark);
    Rgba::from_f32(
        f32::from(s.r) / 255.0,
        f32::from(s.g) / 255.0,
        f32::from(s.b) / 255.0,
        if high_contrast { 0.8 } else { 0.5 },
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// libadwaita 1.10.0 `doc/css-variables.md:101-230`: each accent's
    /// background and its standalone colour on a light and a dark background.
    const DOCUMENTED: [(&str, &str, &str); 9] = [
        ("#3584e4", "#0461be", "#81d0ff"),
        ("#2190a4", "#007184", "#7bdff4"),
        ("#3a944a", "#15772e", "#8de698"),
        ("#c88800", "#905300", "#ffc057"),
        ("#ed5b00", "#b62200", "#ff9c5b"),
        ("#e62d42", "#c00023", "#ff888c"),
        ("#d56199", "#a2326c", "#ffa0d8"),
        ("#9141ac", "#8939a4", "#fba7ff"),
        ("#6f8396", "#526678", "#bbd1e5"),
    ];

    fn hex(s: &str) -> Rgba {
        s.parse().expect("a hex colour")
    }

    #[test]
    fn every_documented_accent_gives_its_documented_standalone_colours() {
        for (bg, light, dark) in DOCUMENTED {
            assert_eq!(standalone(hex(bg), false), hex(light), "{bg} light");
            assert_eq!(standalone(hex(bg), true), hex(dark), "{bg} dark");
        }
    }

    #[test]
    fn the_focus_ring_is_the_standalone_colour_at_half_alpha() {
        assert_eq!(focus_ring(hex("#3584e4"), false, false), hex("#0461be80"));
        assert_eq!(focus_ring(hex("#3584e4"), true, false), hex("#81d0ff80"));
        assert_eq!(focus_ring(hex("#9141ac"), false, false), hex("#8939a480"));
        assert_eq!(focus_ring(hex("#3584e4"), false, true), hex("#0461becc"));
    }
}
