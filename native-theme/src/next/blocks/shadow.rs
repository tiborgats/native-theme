//! Shadow (S2.5).

use native_theme_derive::ThemeParts;

use crate::next::color::Color;
use crate::next::key::Key;
use crate::next::units::Px;
use crate::next::value::Value;

/// A part's shadow. No shadow is `Value::Absent`.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, own_resolved)]
pub enum Shadow {
    /// Shadow layers; at least one (S12).
    Layers(#[parts(block = "fields")] Vec<ShadowLayer>),
    /// An entry of the elevation ramp (S4), which may be empty (no shadow).
    Elevation(Key),
}

/// A resolved shadow: the query resolves an elevation reference to that
/// entry's layers, so a connector receives layers only; a reference to an
/// empty entry resolves to `None`, as `Absent` does (S2.5).
pub type ResolvedShadow = Vec<ResolvedShadowLayer>;

/// One layer of a shadow.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct ShadowLayer {
    /// The horizontal offset, physical: not mirrored in right-to-left.
    pub x: Value<Px>,
    /// The vertical offset.
    pub y: Value<Px>,
    /// The blur radius.
    pub blur: Value<Px>,
    /// The spread.
    pub spread: Value<Px>,
    /// The shadow's colour.
    pub color: Value<Color>,
    /// Drawn inside the box.
    pub inset: bool,
}
