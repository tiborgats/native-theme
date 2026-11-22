//! `kcu_mix`: KColorUtils' mix of two colours, in premultiplied alpha.

use super::kde_color_common::{Rgba, enter, mix_q, read};

/// Mixes `a` toward `b` by `bias` (0 gives `a`, 1 gives `b`) as `KColorUtils::mix` does: the
/// channels are mixed premultiplied by their alphas, then divided by the mixed alpha, through
/// Qt's 16-bit colour storage.
pub(crate) fn kcu_mix(a: Rgba, b: Rgba, bias: f32) -> Rgba {
    read(mix_q(enter(a), enter(b), f64::from(bias)))
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
        }
        Ok(())
    }
}
