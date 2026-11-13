//! `kcu_mix`: KColorUtils' mix of two colours, in premultiplied alpha.

use super::kde_color_common::{Rgba, enter, mix_q, read};

/// Mixes `a` toward `b` by `bias` (0 gives `a`, 1 gives `b`) as `KColorUtils::mix` does: the
/// channels are mixed premultiplied by their alphas, then divided by the mixed alpha, through
/// Qt's 16-bit colour storage.
pub fn kcu_mix(a: Rgba, b: Rgba, bias: f32) -> Rgba {
    read(mix_q(enter(a), enter(b), f64::from(bias)))
}
