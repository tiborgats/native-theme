//! `kcu_mix`: KColorUtils' mix of two colours, in premultiplied alpha.

use super::kde_color_common::{Rgba, enter, mix_q, read};

/// Mixes `a` toward `b` by `bias` (0 gives `a`, 1 gives `b`) as `KColorUtils::mix` does: the
/// channels are mixed premultiplied by their alphas, then divided by the mixed alpha, through
/// Qt's 16-bit colour storage; the bias is the `qreal` (`f64`) `KColorUtils::mix` takes.
pub(crate) fn kcu_mix_qreal(a: Rgba, b: Rgba, bias: f64) -> Rgba {
    read(mix_q(enter(a), enter(b), bias))
}

/// `KColorUtils::mix(a, KColorUtils::mix(b, c, inner), outer)`: the inner mix kept in Qt's colour
/// storage, not read back to a model colour, before the outer mix.
pub(crate) fn kcu_mix_of_mix(a: Rgba, b: Rgba, c: Rgba, inner: f64, outer: f64) -> Rgba {
    read(mix_q(enter(a), mix_q(enter(b), enter(c), inner), outer))
}

#[cfg(test)]
mod tests {
    use super::super::vectors::{TestResult, assert_color, color, output, rows};
    use super::*;

    #[test]
    fn vectors() -> TestResult {
        let rows = rows(include_str!("../../../tests/vectors/kcu_mix.tsv"))?;
        assert_eq!(rows.len(), 88, "row count");
        for row in &rows {
            let qreal = kcu_mix_qreal(
                color(row.get("a")?)?,
                color(row.get("b")?)?,
                row.get("bias")?.parse::<f64>()?,
            );
            assert_color(qreal, output(row.get("result")?)?, row);
        }
        Ok(())
    }

    /// At the inner bias 0 and 1 the nested mix is one mix with `b` or `c`: this proves the
    /// order of the arguments, not the inner mix's storage between 0 and 1, which reader parity
    /// proves (`button.default_button_border_color` on the Breeze files and the custom accent).
    #[test]
    fn a_mix_of_a_mix_at_the_ends_is_one_mix() {
        let [a, b, c] = [
            [0.2, 0.4, 0.6, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        for outer in [0.0, 0.3, 0.5, 1.0] {
            assert_eq!(
                kcu_mix_of_mix(a, b, c, 0.0, outer),
                kcu_mix_qreal(a, b, outer)
            );
            assert_eq!(
                kcu_mix_of_mix(a, b, c, 1.0, outer),
                kcu_mix_qreal(a, c, outer)
            );
        }
    }
}
