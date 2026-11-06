//! `kcu_mix`: KColorUtils' mix of two colours, in premultiplied alpha.

#[allow(dead_code)]
#[path = "kde_color_common.rs"]
mod kde_color_common;

use kde_color_common::{enter, mix_q, read, Rgba};

/// Mixes `a` toward `b` by `bias` (0 gives `a`, 1 gives `b`) as `KColorUtils::mix` does: the
/// channels are mixed premultiplied by their alphas, then divided by the mixed alpha, through
/// Qt's 16-bit colour storage.
pub fn kcu_mix(a: Rgba, b: Rgba, bias: f32) -> Rgba {
    read(mix_q(enter(a), enter(b), f64::from(bias)))
}

#[cfg(test)]
mod tests {
    use super::kde_color_common::table::{check, colour, number};
    use super::kcu_mix;

    #[test]
    fn vectors() {
        let (total, failures) = check(include_str!("../../vectors/kcu_mix.tsv"), |t, row| {
            Some(kcu_mix(
                colour(t.cell(row, "a")?)?,
                colour(t.cell(row, "b")?)?,
                number(t.cell(row, "bias")?)?,
            ))
        });
        assert!(total > 0, "no rows");
        assert!(failures.is_empty(), "{} of {total} rows fail:\n{}", failures.len(), failures.join("\n"));
    }
}
