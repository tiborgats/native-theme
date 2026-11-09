//! Kirigami's `Units.iconSizes.sizeForLabels`: the icon size beside a line of body text.
//! Written from a behavioural description only, without reading LGPL source.

/// The truncated height where the C++ conversion is undefined: what the reference build's
/// x86-64 truncating conversion gives for NaN, infinities and values beyond the 32-bit range.
const UNDEFINED_HEIGHT: i32 = i32::MIN;

/// The height with its fraction dropped toward zero, as a 32-bit integer.
///
/// Non-finite heights and heights whose truncation does not fit an `i32` give
/// [`UNDEFINED_HEIGHT`].
fn truncate(font_height: f32) -> i32 {
    // 2^31 as an exact f32; every f32 strictly inside (-2^31 - 1, 2^31) fits an i32.
    const LIMIT: f32 = 2_147_483_648.0;
    let whole = font_height.trunc();
    if whole.is_finite() && whole >= -LIMIT && whole < LIMIT {
        // Guarded above: the value is whole and within the i32 range, so nothing is lost.
        whole as i32
    } else {
        UNDEFINED_HEIGHT
    }
}

/// Kirigami's step function on a truncated height: 16 to 21 give 16, 22 to 31 give 22,
/// 32 to 47 give 32, 48 to 63 give 48; below 16 and from 64 the height itself.
fn rounded_icon_size(n: i32) -> i32 {
    if n < 16 {
        n
    } else if n < 22 {
        16
    } else if n < 32 {
        22
    } else if n < 48 {
        32
    } else if n < 64 {
        48
    } else {
        n
    }
}

/// The icon size Kirigami picks for an icon beside body text, in logical pixels.
///
/// `font_height` is the body font's unrounded height as an `f32`; the result is a whole
/// number of pixels, also an `f32`. The height is truncated toward zero (never rounded) and
/// stepped. A non-finite height, or one whose truncation does not fit a 32-bit integer,
/// gives -2147483648, as the reference build does.
pub fn kirigami_label_icon_size(font_height: f32) -> f32 {
    f64::from(rounded_icon_size(truncate(font_height))) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pixels(cell: &str) -> Result<f32, String> {
        cell.strip_suffix("px")
            .ok_or_else(|| format!("no px suffix: {cell}"))?
            .parse::<f32>()
            .map_err(|_| format!("not a number: {cell}"))
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn vectors_kirigami_label_icon_size() {
        let table = include_str!("../../tests/vectors/kirigami_label_icon_size.tsv");
        let mut lines = table.lines().filter(|l| !l.starts_with('#'));
        assert_eq!(lines.next(), Some("font_height\tsize"));
        let mut rows = 0usize;
        let mut failures = Vec::new();
        for line in lines {
            rows += 1;
            let cells: Vec<&str> = line.split('\t').collect();
            let [input, output] = cells.as_slice() else {
                failures.push(format!("{line}\n  {} cells", cells.len()));
                continue;
            };
            match (pixels(input), pixels(output)) {
                (Ok(height), Ok(want)) => {
                    let got = kirigami_label_icon_size(height);
                    if got.to_bits() != want.to_bits() {
                        failures.push(format!("{line}\n  got {got}"));
                    }
                }
                (Err(e), _) | (_, Err(e)) => failures.push(format!("{line}\n  unreadable: {e}")),
            }
        }
        assert!(
            failures.is_empty(),
            "{} of {rows} rows fail:\n{}",
            failures.len(),
            failures.join("\n")
        );
        assert_eq!(rows, 49);
        println!("kirigami_label_icon_size: {rows} of {rows} rows pass");
    }

    #[test]
    fn undefined_heights_give_the_reference_builds_value() {
        let want = -2_147_483_648.0_f32;
        assert_eq!(kirigami_label_icon_size(f32::NAN), want);
        assert_eq!(kirigami_label_icon_size(f32::INFINITY), want);
        assert_eq!(kirigami_label_icon_size(f32::NEG_INFINITY), want);
        assert_eq!(kirigami_label_icon_size(3.0e9), want);
        assert_eq!(kirigami_label_icon_size(-0.0), 0.0);
        assert_eq!(kirigami_label_icon_size(-0.9), 0.0);
    }
}
