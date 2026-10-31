//! Interaction (S2.12): a part's state layer and press ripple, Compose's
//! ripple node, a block of the part apart from its paints.
//!
//! It is drawn over the part's fill and the parts nested in it and beneath
//! its stroke, the ripple beneath the state layer. The state layer's colour
//! changes at once and the `StateLayer` group's transitions time its alpha
//! alone; the press ripple is no transition but an effect the connector
//! starts each time `pressed` becomes kept, and which ends by its own
//! lifecycle ([`crate::next::ripple`]).

use native_theme_derive::ThemeParts;

use crate::next::color::Color;
use crate::next::units::{Fraction, Px};
use crate::next::value::Value;

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
    /// The colour, the content colour as the platform passes it.
    pub color: Value<Color>,
    /// The pressed alpha, replacing the colour's own.
    pub alpha: Value<Fraction>,
    /// Clipped to the outline, from the point pressed; `false`: from the
    /// box's centre, unclipped.
    pub bounded: bool,
    /// The radius the effect takes (Compose's target radius); absent: half
    /// the box's diagonal plus `padding`.
    #[parts(may_be_absent)]
    pub max_radius: Value<Px>,
    /// Added to half the box's diagonal where `max_radius` is absent.
    pub padding: Value<Px>,
}

/// A platform's press ripple; closed, each implemented once in native-theme
/// to its source ([`crate::next::ripple`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum RippleEffect {
    /// Android's `RippleDrawable` in its patterned style
    /// (`frameworks-base@android-17.0.0_r1
    /// graphics/java/android/graphics/drawable/RippleDrawable.java:161`,
    /// `:1336`), which Compose draws the press with on Android.
    AndroidPatterned,
}
