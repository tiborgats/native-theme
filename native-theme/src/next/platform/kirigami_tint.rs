//! `kirigami_tint`: Kirigami's `ColorUtils::tintWithAlpha`, a straight-alpha source-over tint.

use super::kde_color_common::{Rgba, enter, from_rgbd, mind, read};

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
