//! `qt_tint`: Qt Quick's `Qt.tint(base, Qt.alpha(tint, factor))` on colours of 8-bit channels.
//!
//! The behaviour is qtdeclarative's and qtbase's at `v6.12.0` (the same at `v6.11.2`), as Qt
//! 6.12.0 (PySide6 6.12.0) runs it: qtdeclarative `src/qml/qml/qqmlbuiltinfunctions.cpp`,
//! `QtObject::alpha` and `QtObject::tint`, `src/quick/util/qquickglobal.cpp`, the colour
//! provider's `alpha` and `tint`, `src/quick/util/qquickvaluetypes.cpp`,
//! `QQuickColorValueType::alpha` and `QQuickColorValueType::tint`; qtbase `qcolor.cpp`,
//! `setAlphaF`, `QColor::fromRgbF`, `redF()` and its siblings, `red()` and its siblings, and
//! `qnumeric.h`, `qRound(float)` (the x86-64 SSE2 branch). Written in a clean room, from a
//! behavioural description of that code and the vector table `tests/vectors/qt_tint.tsv`
//! alone; no line of Qt's code is in it. Every step is one IEEE operation of the precision the
//! description states, its result rounded once; nothing is fused.

use std::num::NonZeroU32;

/// 257, the ratio of a 16-bit channel to an 8-bit one, as a divisor.
const TWO_FIVE_SEVEN: NonZeroU32 = NonZeroU32::MIN.saturating_add(256);
/// The largest 16-bit channel, as a single.
const MAX16: f32 = 65535.0;

/// `Qt.tint(base, Qt.alpha(tint, factor))` read back as its 8-bit `red()`, `green()`, `blue()`
/// and `alpha()`. `tint`'s own alpha plays no part: `Qt.alpha` replaces it by `factor`, clamped
/// to `[0, 1]` (a NaN as 0). A tint alpha that reads as 255 in 8 bits returns the tint with that
/// alpha, one that reads as 0 returns the base; between them the tint is blended over the base.
pub(crate) fn qt_tint(base: [u8; 4], tint: [u8; 4], factor: f64) -> [u8; 4] {
    tint_stored(base, tint, factor).map(to_8bit)
}

/// The 16-bit channels Qt holds for `Qt.tint(base, Qt.alpha(tint, factor))`.
pub(crate) fn tint_stored(base: [u8; 4], tint: [u8; 4], factor: f64) -> [u16; 4] {
    let base = base.map(to_16bit);
    let [rt, gt, bt, _] = tint.map(to_16bit);
    let alpha = alpha_held(factor);
    match to_8bit(alpha) {
        u8::MAX => [rt, gt, bt, alpha],
        0 => base,
        _ => blend(base, [rt, gt, bt], alpha),
    }
}

/// The 16-bit alpha `Qt.alpha` holds for `factor`: the factor as a single, a NaN as 0, clamped
/// to `[0, 1]`, stored.
fn alpha_held(factor: f64) -> u16 {
    let f = factor as f32;
    let f = if f.is_nan() { 0.0 } else { f.clamp(0.0, 1.0) };
    store(f)
}

/// The blend of the tint's channels `tint`, of held alpha `alpha`, over `base`.
fn blend(base: [u16; 4], tint: [u16; 3], alpha: u16) -> [u16; 4] {
    let a = f64::from(read(alpha));
    let complement = 1.0 - a;
    let [xr, xg, xb, xa] = base.map(|x| f64::from(read(x)));
    let [tr, tg, tb] = tint.map(|t| f64::from(read(t)));
    let channel = |t: f64, x: f64| t * a + x * complement;
    [
        channel(tr, xr),
        channel(tg, xg),
        channel(tb, xb),
        a + complement * xa,
    ]
    .map(|d| store(d as f32))
}

/// `redF()` and its siblings: a held channel over 65535, one single-precision division.
fn read(x: u16) -> f32 {
    f32::from(x) / MAX16
}

/// The 16-bit channel Qt holds for a single `v` in `[0, 1]`: `qRound(v · 65535)`, the product
/// rounded to a single, one half added in single precision, the sum truncated.
fn store(v: f32) -> u16 {
    (v * MAX16 + 0.5) as u16
}

/// The 16-bit channel Qt stores for an 8-bit one, `c · 257`.
fn to_16bit(c: u8) -> u16 {
    u16::from_be_bytes([c, c])
}

/// The 8-bit channel Qt reads from a 16-bit one, `⌊(x + 128) / 257⌋`.
fn to_8bit(x: u16) -> u8 {
    u8::try_from(floor_div(u32::from(x).saturating_add(128), TWO_FIVE_SEVEN)).unwrap_or(u8::MAX)
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
        let rows = rows(include_str!("../../../tests/vectors/qt_tint.tsv"))?;
        assert_eq!(rows.len(), 174, "row count");
        for row in &rows {
            let base = bytes(row.get("base")?)?;
            let tint = bytes(row.get("tint")?)?;
            let factor = row.get("factor")?.parse::<f64>()?;
            let stored = tint_stored(base, tint, factor).map(f64::from);
            let expected = output(row.get("stored")?)?.map(|x| (x * 65535.0).round());
            assert_eq!(stored, expected, "stored: `{row:?}`");
            assert_eq!(
                qt_tint(base, tint, factor),
                bytes(row.get("result")?)?,
                "result: `{row:?}`"
            );
        }
        Ok(())
    }

    /// The description's switch factor: `Qt.alpha(…, 0.9)` holds the alpha 58982.
    #[test]
    fn alpha_at_point_nine() {
        assert_eq!(alpha_held(0.9), 58982);
    }

    /// A NaN factor gives the base unchanged.
    #[test]
    fn nan_factor_is_the_base() {
        let base = [0x23, 0x26, 0x29, 0xff];
        assert_eq!(qt_tint(base, [0xfc, 0xfc, 0xfc, 0xff], f64::NAN), base);
    }
}
