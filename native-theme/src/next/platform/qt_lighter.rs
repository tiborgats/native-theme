//! `qt_lighter`: Qt's `QColor::lighter(int)` on a colour of 8-bit channels.
//!
//! The behaviour is qtbase `v6.12.0`'s, as Qt 6.12.0 (PySide6 6.12.0) runs it: `qcolor.cpp`,
//! `QColor::lighter(int)`, `QColor::darker(int)`, `QColor::toHsv()`, `QColor::toRgb()` and
//! `QColor::red()` and its siblings, and `qcolor.h`, the constructor `QColor(int, int, int, int)`.
//! Written in a clean room, from a behavioural description of that code and the vector table
//! `tests/vectors/qt_lighter.tsv` alone; no line of Qt's code is in it. Every multiply-add the
//! description fuses is one `mul_add` (one rounding); every other step is one IEEE
//! single-precision operation.

use std::num::NonZeroU32;

/// 100, the factor that keeps the value, as a divisor.
const HUNDRED: NonZeroU32 = NonZeroU32::MIN.saturating_add(99);
/// 257, the ratio of a 16-bit channel to an 8-bit one, as a divisor.
const TWO_FIVE_SEVEN: NonZeroU32 = NonZeroU32::MIN.saturating_add(256);
/// The largest 16-bit channel, as a single.
const MAX16: f32 = 65535.0;
/// Hundredths of a degree in a sixth of the hue circle.
const SIXTH: f32 = 6000.0;

/// A colour in Qt's HSV form, alpha apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Hsv {
    /// The hue in hundredths of a degree; `None` for an achromatic colour.
    hue: Option<u16>,
    /// The saturation, 65535 full.
    saturation: u16,
    /// The value.
    value: u16,
}

/// `QColor(r, g, b, a).lighter(factor)` read back as its 8-bit `red()`, `green()`, `blue()` and
/// `alpha()`: from factor 100 the HSV value is scaled by `factor / 100` and what passes the
/// maximum is taken off the saturation; from 1 to 99 it is `darker(10000 / factor)`; at 0 and
/// below the colour is returned as it is.
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "called by the kde-breeze builder for L3, plan M2 KDE unit"
    )
)]
pub(crate) fn qt_lighter(color: [u8; 4], factor: i32) -> [u8; 4] {
    lighter_stored(color, factor).map(to_8bit)
}

/// The 16-bit channels `R', G', B', A'` Qt holds for `QColor(r, g, b, a).lighter(factor)`.
fn lighter_stored(color: [u8; 4], factor: i32) -> [u16; 4] {
    let [r, g, b, a] = color.map(to_16bit);
    let Some(factor) = u32::try_from(factor).ok().and_then(NonZeroU32::new) else {
        return [r, g, b, a];
    };
    let hsv = to_hsv(r, g, b);
    let value = u32::from(hsv.value);
    let scaled = if factor < HUNDRED {
        // `factor ≤ 99`, so `⌊10000 / factor⌋ ≥ 101` and the saturations change nothing.
        let d = NonZeroU32::MIN.saturating_add(floor_div(10_000, factor).saturating_sub(1));
        Hsv {
            value: clamp16(floor_div(value.saturating_mul(100), d)),
            ..hsv
        }
    } else {
        let v1 = floor_div(factor.get().wrapping_mul(value), HUNDRED);
        let excess = v1.saturating_sub(u32::from(u16::MAX));
        Hsv {
            saturation: clamp16(u32::from(hsv.saturation).saturating_sub(excess)),
            value: clamp16(v1),
            ..hsv
        }
    };
    let [r, g, b] = to_rgb(scaled);
    [r, g, b, a]
}

/// `QColor::toHsv()` of the 16-bit channels `r, g, b`.
fn to_hsv(r: u16, g: u16, b: u16) -> Hsv {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    if max == min {
        return Hsv {
            hue: None,
            saturation: 0,
            value: max,
        };
    }
    let [rf, gf, bf, maxf] = [r, g, b, max].map(f32::from);
    let chroma = maxf - f32::from(min);
    let saturation = (chroma / maxf).mul_add(MAX16, 0.5) as u16;
    let sixths = if max == r {
        let h = (gf - bf) / chroma;
        if h < 0.0 { h + 6.0 } else { h }
    } else if max == g {
        2.0 + (bf - rf) / chroma
    } else {
        4.0 + (rf - gf) / chroma
    };
    Hsv {
        hue: Some(sixths.mul_add(SIXTH, 0.5) as u16),
        saturation,
        value: max,
    }
}

/// `QColor::toRgb()` of an HSV colour: its 16-bit `R', G', B'`.
fn to_rgb(hsv: Hsv) -> [u16; 3] {
    let hue = match hsv.hue {
        Some(hue) if hsv.saturation != 0 => hue,
        _ => return [hsv.value; 3],
    };
    let h = if hue == 36_000 {
        0.0
    } else {
        f32::from(hue) / SIXTH
    };
    let s = f32::from(hsv.saturation) / MAX16;
    let v = f32::from(hsv.value) / MAX16;
    let sector = h as u8;
    let f = h - f32::from(sector);
    let p = v * (1.0 - s);
    let q = v * (-s).mul_add(f, 1.0);
    let t = v * (-s).mul_add(1.0 - f, 1.0);
    let [v, p, q, t] = [v, p, q, t].map(|x| x.mul_add(MAX16, 0.5) as u16);
    match sector {
        0 => [v, t, p],
        1 => [q, v, p],
        2 => [p, v, t],
        3 => [p, q, v],
        4 => [t, p, v],
        _ => [v, p, q],
    }
}

/// The 16-bit channel Qt stores for an 8-bit one, `c · 257`.
fn to_16bit(c: u8) -> u16 {
    u16::from_be_bytes([c, c])
}

/// The 8-bit channel Qt reads from a 16-bit one, `⌊(x + 128) / 257⌋`.
fn to_8bit(x: u16) -> u8 {
    clamp8(floor_div(u32::from(x).saturating_add(128), TWO_FIVE_SEVEN))
}

/// `⌊n / d⌋`: the floor is the arithmetic the description states, and a non-zero divisor
/// cannot panic.
#[allow(
    clippy::integer_division,
    reason = "the floor of the quotient is the result wanted"
)]
fn floor_div(n: u32, d: NonZeroU32) -> u32 {
    n / d
}

/// `x`, or 65535 where it is larger.
fn clamp16(x: u32) -> u16 {
    u16::try_from(x).unwrap_or(u16::MAX)
}

/// `x`, or 255 where it is larger.
fn clamp8(x: u32) -> u8 {
    u8::try_from(x).unwrap_or(u8::MAX)
}

#[cfg(test)]
mod tests {
    use super::super::vectors::{TestResult, output, rows};
    use super::*;

    /// A `#rrggbbaa` cell's four bytes.
    fn bytes(cell: &str) -> Result<[u8; 4], String> {
        let hex = cell
            .strip_prefix('#')
            .filter(|hex| hex.len() == 8)
            .ok_or_else(|| format!("no `#rrggbbaa` in `{cell}`"))?;
        let byte = |i: usize| {
            hex.get(i..i.saturating_add(2))
                .and_then(|pair| u8::from_str_radix(pair, 16).ok())
                .ok_or_else(|| format!("no colour `{cell}`"))
        };
        Ok([byte(0)?, byte(2)?, byte(4)?, byte(6)?])
    }

    #[test]
    fn vectors() -> TestResult {
        let rows = rows(include_str!("../../../tests/vectors/qt_lighter.tsv"))?;
        assert_eq!(rows.len(), 275, "row count");
        for row in &rows {
            let c = bytes(row.get("c")?)?;
            let factor = row.get("factor")?.parse::<i32>()?;
            let stored = lighter_stored(c, factor).map(f64::from);
            let expected = output(row.get("stored")?)?.map(|x| (x * 65535.0).round());
            assert_eq!(stored, expected, "stored: `{row:?}`");
            assert_eq!(
                qt_lighter(c, factor),
                bytes(row.get("result")?)?,
                "result: `{row:?}`"
            );
        }
        Ok(())
    }

    /// The description's worked example: Breeze's highlight at 110 is `#44bfff`, its 16-bit
    /// channels `17491, 49056, 65535`.
    #[test]
    fn breeze_highlight_at_110() {
        assert_eq!(
            lighter_stored([0x3d, 0xae, 0xe9, 0xff], 110),
            [17491, 49056, 65535, 65535]
        );
        assert_eq!(
            qt_lighter([0x3d, 0xae, 0xe9, 0xff], 110),
            [0x44, 0xbf, 0xff, 0xff]
        );
    }
}
