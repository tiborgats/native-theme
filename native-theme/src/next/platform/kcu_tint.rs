//! `kcu_tint`: KColorUtils' tint of a colour toward another by a contrast-based amount.

use super::kde_color_common::{Rgba, enter, read, tint_q};

/// Tints the base colour `a` toward `b` by `amount` (0 gives `a`, 1 gives `b`) as
/// `KColorUtils::tint` does: a bisection finds the mix whose contrast with `a` matches the
/// amount, and the result keeps that mix's hue and chroma at a luma pulled back toward `a`'s.
pub(crate) fn kcu_tint(a: Rgba, b: Rgba, amount: f32) -> Rgba {
    read(tint_q(enter(a), enter(b), f64::from(amount)))
}

#[cfg(test)]
mod tests {
    use super::super::vectors::{TestResult, assert_color, color, number, output, rows};
    use super::*;

    #[test]
    fn vectors() -> TestResult {
        let rows = rows(include_str!("../../../tests/vectors/kcu_tint.tsv"))?;
        assert_eq!(rows.len(), 124, "row count");
        for row in &rows {
            let got = kcu_tint(
                color(row.get("a")?)?,
                color(row.get("b")?)?,
                number(row.get("amount")?)?,
            );
            assert_color(got, output(row.get("result")?)?, row);
        }
        Ok(())
    }
}
