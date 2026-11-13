//! Interaction (S2.12): a part's state layer and press ripple, Compose's
//! ripple node, a block of the part apart from its paints.
//!
//! It is drawn over the part's fill and the parts nested in it and beneath
//! its stroke, the ripple beneath the state layer. The state layer's colour
//! changes at once and the `StateLayer` group's transitions time its alpha
//! alone; the press ripple is no transition but an effect the connector
//! starts each time `pressed` becomes kept, and which ends by its own
//! lifecycle ([`crate::ripple`]).

use native_theme_derive::ThemeParts;

use crate::color::Color;
use crate::selector::PartRef;
use crate::units::{Fraction, Px};
use crate::value::Value;

/// A part's state layer and press ripple.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Interaction {
    /// The hover, focus or drag feedback.
    #[parts(block = "fields")]
    pub state_layer: StateLayerStyle,
    /// The press ripple.
    #[parts(block = "fields")]
    pub ripple: RippleStyle,
}

/// The state layer: the part's outline filled in `color` at `alpha`, the
/// colour's own alpha replaced.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct StateLayerStyle {
    /// The colour; not interpolated.
    pub color: Value<Color>,
    /// The alpha, which the `StateLayer` group's transitions time.
    pub alpha: Value<Fraction>,
}

/// The press ripple's effect and the inputs the platform gives it.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct RippleStyle {
    /// The platform's effect; absent: no press ripple.
    #[parts(may_be_absent)]
    pub effect: Value<RippleEffect>,
    /// The colour, the content colour as the platform passes it; absent
    /// only where `effect` is (S6.3, S12).
    #[parts(may_be_absent)]
    pub color: Value<Color>,
    /// The pressed alpha, replacing the colour's own; absent only where
    /// `effect` is (S6.3, S12).
    #[parts(may_be_absent)]
    pub alpha: Value<Fraction>,
    /// Ignored where `effect` is absent. Clipped to the indication's box
    /// and the part's outline, from the point pressed; `false`: from the
    /// box's centre, clipped only to the ripple's circle, which may reach
    /// past the indication's box.
    pub bounded: bool,
    /// The radius the effect takes (Compose's target radius); absent: half
    /// the box's diagonal plus `padding`.
    #[parts(may_be_absent)]
    pub max_radius: Value<Px>,
    /// Added to half the box's diagonal where `max_radius` is absent;
    /// absent only where `effect` is (S6.3, S12).
    #[parts(may_be_absent)]
    pub padding: Value<Px>,
    /// The box the ripple measures, the indication's box; `None`: the
    /// part's own box.
    #[parts(block = "set")]
    pub indication_box: Option<IndicationBox>,
}

/// The indication's box (S2.12): a part of the ripple's widget, its box
/// grown on each side by `outset`, as the connector lays that part out,
/// its centre floored to the device pixel.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, check)]
#[non_exhaustive]
pub struct IndicationBox {
    /// The part, of the ripple's widget, as a keyframe's `part` names one;
    /// never one that can be absent where the ripple can be drawn (S12).
    pub part: PartRef,
    /// How far the box grows on each side; at least zero (S12).
    pub outset: Value<Px>,
}

impl IndicationBox {
    /// The box of `part` grown on each side by `outset`.
    #[must_use]
    pub fn new(part: PartRef, outset: Value<Px>) -> IndicationBox {
        IndicationBox { part, outset }
    }
}

/// A platform's press ripple; closed, each implemented once in native-theme
/// to its source ([`crate::ripple`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum RippleEffect {
    /// Android's `RippleDrawable` in its patterned style
    /// (`frameworks-base@android-17.0.0_r1
    /// graphics/java/android/graphics/drawable/RippleDrawable.java:161`,
    /// `:1336`), which Compose draws the press with on Android.
    AndroidPatterned,
}
