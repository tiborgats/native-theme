//! Focus indicator (S2.10).

use native_theme_derive::ThemeParts;

use super::motion::{ResolvedTransition, Transition};
use super::outline::{Corners, CornersPatch, DashPattern, ResolvedCorners};
use super::paint::{Paint, PaintPatch, ResolvedPaint};
use super::shadow::{ResolvedShadow, Shadow};
use crate::next::key::Key;
use crate::next::units::{Ms, Px, Sides};
use crate::next::value::Value;

/// A part's focus indicator, drawn only while the element has
/// `FocusVisible`; a part with no indicator states its `focus` `Absent`.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct FocusIndicator {
    /// What is drawn.
    pub kind: FocusKind,
    /// The line's width.
    pub width: Value<Px>,
    /// The line's paint.
    #[parts(block = "append")]
    pub paint: Value<Paint>,
    /// The line's pattern, `solid` included (WinUI's dotted line).
    pub dash: Value<DashPattern>,
    /// The indicator's distance from the part's box, per side.
    pub offset: Sides<Value<Px>>,
    /// The indicator's corners: `FollowPart`, concentric with the part, or
    /// stated where the platform draws otherwise.
    #[parts(block = "fields")]
    pub corners: Corners,
    /// The indicator's shadow; an inner ring is a shadow layer.
    #[parts(block = "set", may_be_absent)]
    pub shadow: Value<Shadow>,
    /// The indicator's own motion.
    #[parts(block = "fields", may_be_absent)]
    pub transition: Value<Transition>,
}

/// When focus becomes visible.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
pub enum FocusVisibleWhen {
    /// After keyboard navigation.
    Keyboard {
        /// The keyboard focus's timeout; `None`: no timeout.
        timeout: Option<Value<Ms>>,
    },
    /// On any focus.
    AnyFocus,
    /// While the element or one of its descendants has focus (CSS's
    /// `:focus-within`).
    FocusWithin,
    /// Never.
    Never,
}

/// What a focus indicator draws.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum FocusKind {
    /// A ring around the part.
    Ring,
    /// A filled ring.
    FilledRing,
    /// An underline.
    Underline,
    /// A platform's own indicator (WinUI's Reveal).
    Keyed(Key),
}
