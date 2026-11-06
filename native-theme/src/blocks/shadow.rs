//! Shadow (S2.5).

use native_theme_derive::ThemeParts;

use crate::color::Color;
use crate::key::Key;
use crate::units::Px;
use crate::value::Value;

/// A part's shadow. No shadow is `Value::Absent`.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, own_resolved, check)]
pub enum Shadow {
    /// Shadow layers; at least one (S12).
    Layers(#[parts(block = "fields")] Vec<ShadowLayer>),
    /// An entry of the elevation ramp (S4), which may be empty (no shadow).
    Elevation(Key),
}

/// A resolved shadow: the query resolves an elevation reference to that
/// entry's layers, so a connector receives layers only; a reference to an
/// empty entry resolves to `None`, as `Absent` does (S2.5). So a resolved
/// shadow is never empty: the query turns empty layers into `None`, and a
/// connector meets no shadow in one form only.
pub type ResolvedShadow = Vec<ResolvedShadowLayer>;

/// One layer of a shadow.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct ShadowLayer {
    /// The horizontal offset, physical: not mirrored in right-to-left.
    #[parts(signed)]
    pub x: Value<Px>,
    /// The vertical offset.
    #[parts(signed)]
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
