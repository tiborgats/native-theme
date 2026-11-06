//! `kcu_tint`: KColorUtils' tint of a colour toward another by a contrast-based amount.

use super::kde_color_common::{Rgba, enter, read, tint_q};

/// Tints the base colour `a` toward `b` by `amount` (0 gives `a`, 1 gives `b`) as
/// `KColorUtils::tint` does: a bisection finds the mix whose contrast with `a` matches the
/// amount, and the result keeps that mix's hue and chroma at a luma pulled back toward `a`'s.
pub fn kcu_tint(a: Rgba, b: Rgba, amount: f32) -> Rgba {
    read(tint_q(enter(a), enter(b), f64::from(amount)))
}
