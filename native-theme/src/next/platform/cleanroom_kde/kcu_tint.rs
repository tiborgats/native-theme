//! `kcu_tint`: KColorUtils' tint of a colour toward another by a contrast-based amount.

#[allow(dead_code)]
#[path = "kde_color_common.rs"]
mod kde_color_common;

use kde_color_common::{enter, read, tint_q, Rgba};

/// Tints the base colour `a` toward `b` by `amount` (0 gives `a`, 1 gives `b`) as
/// `KColorUtils::tint` does: a bisection finds the mix whose contrast with `a` matches the
/// amount, and the result keeps that mix's hue and chroma at a luma pulled back toward `a`'s.
pub fn kcu_tint(a: Rgba, b: Rgba, amount: f32) -> Rgba {
    read(tint_q(enter(a), enter(b), f64::from(amount)))
}

#[cfg(test)]
mod tests {
    use super::kde_color_common::table::{check, colour, number};
    use super::kcu_tint;

    #[test]
    fn vectors() {
        let (total, failures) = check(include_str!("../../vectors/kcu_tint.tsv"), |t, row| {
            Some(kcu_tint(
                colour(t.cell(row, "a")?)?,
                colour(t.cell(row, "b")?)?,
                number(t.cell(row, "amount")?)?,
            ))
        });
        assert!(total > 0, "no rows");
        assert!(failures.is_empty(), "{} of {total} rows fail:\n{}", failures.len(), failures.join("\n"));
    }
}
