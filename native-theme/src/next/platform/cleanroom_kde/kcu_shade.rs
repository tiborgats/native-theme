//! `kcu_shade`: KColorUtils' shade, an offset of a colour's luma and chroma in KDE's HCY space.

#[allow(dead_code)]
#[path = "kde_color_common.rs"]
mod kde_color_common;

use kde_color_common::{enter, read, shade_q, Rgba};

/// Shades `c` as `KColorUtils::shade` does: adds `luma` to its HCY luma and `chroma` to its
/// chroma, each clamped to `[0, 1]`, keeping its hue and alpha.
pub fn kcu_shade(c: Rgba, luma: f32, chroma: f32) -> Rgba {
    read(shade_q(enter(c), f64::from(luma), f64::from(chroma)))
}

#[cfg(test)]
mod tests {
    use super::kde_color_common::table::{check, colour, number};
    use super::kcu_shade;

    #[test]
    fn vectors() {
        let (total, failures) = check(include_str!("../../vectors/kcu_shade.tsv"), |t, row| {
            Some(kcu_shade(
                colour(t.cell(row, "c")?)?,
                number(t.cell(row, "luma")?)?,
                number(t.cell(row, "chroma")?)?,
            ))
        });
        assert!(total > 0, "no rows");
        assert!(failures.is_empty(), "{} of {total} rows fail:\n{}", failures.len(), failures.join("\n"));
    }
}
