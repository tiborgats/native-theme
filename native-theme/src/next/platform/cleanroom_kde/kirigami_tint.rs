//! `kirigami_tint`: Kirigami's `ColorUtils::tintWithAlpha`, a straight-alpha source-over tint.

#[allow(dead_code)]
#[path = "kde_color_common.rs"]
mod kde_color_common;

use kde_color_common::{enter, from_rgbd, mind, read, Rgba};

/// Tints the target colour `c` with `tint` at the tint's alpha times `alpha`, as Kirigami's
/// `ColorUtils::tintWithAlpha` does: the red, green and blue channels are blended straight (not
/// premultiplied) and the alpha is composed source-over.
pub fn kirigami_tint(c: Rgba, tint: Rgba, alpha: f32) -> Rgba {
    let target = enter(c);
    let over = enter(tint);
    let [tr, tg, tb, tint_alpha] = read(over);
    let [cr, cg, cb, ca] = read(target);
    let ta = f64::from(tint_alpha) * f64::from(alpha);
    let ia = 1.0 - ta;
    if (ta - 1.0).abs() * 1e12 <= mind(ta.abs(), 1.0) {
        return read(over);
    }
    if ta.abs() <= 1e-12 {
        return read(target);
    }
    let blend = |t: f32, x: f32| f64::from(t) * ta + f64::from(x) * ia;
    read(from_rgbd(
        blend(tr, cr),
        blend(tg, cg),
        blend(tb, cb),
        ta + ia * f64::from(ca),
    ))
}

#[cfg(test)]
mod tests {
    use super::kde_color_common::table::{check, colour, number};
    use super::kirigami_tint;

    #[test]
    fn vectors() {
        let (total, failures) = check(include_str!("../../vectors/kirigami_tint.tsv"), |t, row| {
            Some(kirigami_tint(
                colour(t.cell(row, "c")?)?,
                colour(t.cell(row, "tint")?)?,
                number(t.cell(row, "alpha")?)?,
            ))
        });
        assert!(total > 0, "no rows");
        assert!(failures.is_empty(), "{} of {total} rows fail:\n{}", failures.len(), failures.join("\n"));
    }
}
