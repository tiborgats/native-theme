//! `kcu_tint`: KColorUtils' tint of a colour toward another by a contrast-based amount.
//!
//! Written in a clean room on the parked 0.8.0 branch, from behavioural
//! descriptions and the vector tables alone, and carried here unchanged but for
//! the clippy fixes its commit names, below this paragraph
//! (`native-theme/src/platform/kcu_tint.rs` and its notice in
//! `native-theme/src/platform/mod.rs:1-18` on `v0.8.0-rc1` at 78ee0efd). No line
//! of KDE's or Qt's code is in it. Its table, `tests/vectors/kcu_tint.tsv`, and the
//! reader's inactive selection's, `tests/vectors/kde_selection_inactive.tsv`, which
//! reaches it through `kcu_tint`, were generated there by `tools/kde-vectors` from
//! kcolorscheme 6.31.0, kguiaddons 6.31.0 and qtbase 6.12.0, as their headers say;
//! the generator is not on this branch.

use super::kde_color_common::{Rgba, enter, read, tint_q};

/// Tints the base colour `a` toward `b` by `amount` (0 gives `a`, 1 gives `b`) as
/// `KColorUtils::tint` does: a bisection finds the mix whose contrast with `a` matches the
/// amount, and the result keeps that mix's hue and chroma at a luma pulled back toward `a`'s.
pub fn kcu_tint(a: Rgba, b: Rgba, amount: f32) -> Rgba {
    read(tint_q(enter(a), enter(b), f64::from(amount)))
}
