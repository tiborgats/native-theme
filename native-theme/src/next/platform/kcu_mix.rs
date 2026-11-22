//! `kcu_mix`: KColorUtils' mix of two colours, in premultiplied alpha.

use super::kde_color_common::{Rgba, enter, mix_q, read};

/// Mixes `a` toward `b` by `bias` (0 gives `a`, 1 gives `b`) as `KColorUtils::mix` does: the
/// channels are mixed premultiplied by their alphas, then divided by the mixed alpha, through
/// Qt's 16-bit colour storage.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the kernel its vector table proves; the builder passes the bias as the `qreal` \
                  0.7.2 passes, through `kcu_mix_qreal`"
    )
)]
pub(crate) fn kcu_mix(a: Rgba, b: Rgba, bias: f32) -> Rgba {
    read(mix_q(enter(a), enter(b), f64::from(bias)))
}

/// [`kcu_mix`] with the bias as the `qreal` (`f64`) `KColorUtils::mix` takes.
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
    use super::super::vectors::{TestResult, assert_color, color, number, output, rows};
    use super::*;

    #[test]
    fn vectors() -> TestResult {
        let rows = rows(include_str!("../../../tests/vectors/kcu_mix.tsv"))?;
        assert_eq!(rows.len(), 88, "row count");
        for row in &rows {
            let got = kcu_mix(
                color(row.get("a")?)?,
                color(row.get("b")?)?,
                number(row.get("bias")?)?,
            );
            assert_color(got, output(row.get("result")?)?, row);
            let qreal = kcu_mix_qreal(
                color(row.get("a")?)?,
                color(row.get("b")?)?,
                f64::from(number(row.get("bias")?)?),
            );
            assert_color(qreal, output(row.get("result")?)?, row);
        }
        Ok(())
    }

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
