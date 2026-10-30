//! Transform (S2.6).

use native_theme_derive::ThemeParts;

use crate::next::units::{Deg, Px};
use crate::next::value::Value;

/// A part's transform. Its translation is stated for a left-to-right layout
/// and mirrored with it: under right-to-left the query returns `translate_x`
/// negated. The origin, the rotation, the scales and `translate_y` are not
/// mirrored.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Transform {
    /// The horizontal scale.
    pub scale_x: Value<f32>,
    /// The vertical scale.
    pub scale_y: Value<f32>,
    /// The rotation.
    pub rotate: Value<Deg>,
    /// The horizontal translation, mirrored in right-to-left.
    pub translate_x: Value<Px>,
    /// The vertical translation.
    pub translate_y: Value<Px>,
    /// The origin, in fractions of the part's box from its top-left corner,
    /// physical.
    pub origin: (f32, f32),
}
