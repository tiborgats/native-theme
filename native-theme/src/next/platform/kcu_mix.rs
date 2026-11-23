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

    /// The nested mix against `KColorUtils::mix(a, KColorUtils::mix(b, c, inner), outer)` as
    /// kguiaddons runs it, exact: the 16-bit channels Qt holds, and the unrounded channels.
    #[test]
    fn mix_of_mix_vectors() -> TestResult {
        let rows = rows(include_str!("../../../tests/vectors/kcu_mix_of_mix.tsv"))?;
        assert_eq!(rows.len(), 102, "row count");
        for row in &rows {
            let got = kcu_mix_of_mix(
                color(row.get("a")?)?,
                color(row.get("b")?)?,
                color(row.get("c")?)?,
                row.get("inner")?.parse::<f64>()?,
                row.get("outer")?.parse::<f64>()?,
            );
            assert_eq!(
                got.map(|x| (f64::from(x) * 65535.0).round()),
                rgba64(row.get("result16")?)?,
                "result16: `{row:?}`"
            );
            assert_color(got, output(row.get("result")?)?, row);
        }
        Ok(())
    }

    /// A `rgba64(r, g, b, a)` cell's 16-bit channels.
    fn rgba64(cell: &str) -> Result<[f64; 4], String> {
        let channels = cell
            .strip_prefix("rgba64(")
            .and_then(|rest| rest.strip_suffix(')'))
            .ok_or_else(|| format!("no `rgba64(…)` in `{cell}`"))?
            .split(',')
            .map(|n| n.trim().parse::<u16>().map(f64::from))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("{e} in `{cell}`"))?;
        match channels.as_slice() {
            [r, g, b, a] => Ok([*r, *g, *b, *a]),
            _ => Err(format!("no four channels in `{cell}`")),
        }
    }

    /// At the inner bias 0 and 1 the nested mix is one mix with `b` or `c`: this proves the
    /// order of the arguments.
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
