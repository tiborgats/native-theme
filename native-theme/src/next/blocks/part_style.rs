//! Part style (S2.11): the style of one part of a widget.
//!
//! A part has the fields its catalogue entry lists; the derive generates
//! that set per part, and a field outside it is not part of that part.

use native_theme_derive::ThemeParts;

use super::focus::{FocusIndicator, ResolvedFocusIndicator};
use super::icon::{Glyph, IconStyle, ResolvedGlyph, ResolvedIconStyle};
use super::motion::{ResolvedTransition, Transition};
use super::outline::{Corners, ResolvedCorners, ResolvedStroke, Stroke};
use super::paint::{Paint, PaintPatch, ResolvedPaint};
use super::shadow::{ResolvedShadow, Shadow};
use super::text::{ResolvedTextStyle, TextStyle};
use super::transform::{ResolvedTransform, Transform};
use super::{KeyedItem, merge_by};
use crate::next::color::Color;
use crate::next::key::Key;
use crate::next::roles::{GlobalRole, SurfaceRole, SurfaceSet};
use crate::next::shapes::ShapeKind;
use crate::next::units::{Fraction, Px, Sides};
use crate::next::value::Value;

/// The style of one part of a widget.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, resolved_patch)]
#[non_exhaustive]
pub struct PartStyle {
    /// The fill; absent on a part that paints nothing at rest.
    #[parts(block = "append", may_be_absent)]
    pub fill: Value<Paint>,
    /// The stroke.
    #[parts(block = "fields", may_be_absent)]
    pub stroke: Value<Stroke>,
    /// The corners.
    #[parts(block = "fields")]
    pub corners: Value<Corners>,
    /// A shape replacing the corners' outline.
    #[parts(may_be_absent)]
    pub shape: Value<ShapeKind>,
    /// The shadow.
    #[parts(block = "set", may_be_absent)]
    pub shadow: Value<Shadow>,
    /// The opacity, applied to the element and its descendants as a group.
    pub opacity: Value<Fraction>,
    /// The transform, applied to the element and its descendants as a group.
    #[parts(block = "fields")]
    pub transform: Value<Transform>,
    /// The text style.
    #[parts(block = "fields", may_be_absent)]
    pub text: Value<TextStyle>,
    /// The icon style.
    #[parts(block = "fields", may_be_absent)]
    pub icon: Value<IconStyle>,
    /// The glyph.
    #[parts(block = "fields", may_be_absent)]
    pub glyph: Value<Glyph>,
    /// The focus indicator.
    #[parts(block = "fields", may_be_absent)]
    pub focus: Value<FocusIndicator>,
    /// The padding inside the box, per side.
    pub padding: Sides<Value<Px>>,
    /// The margin outside the box, per side; may be negative.
    #[parts(may_be_absent)]
    pub margin: Sides<Value<Px>>,
    /// The minimum width; `Auto`: no minimum beyond the content.
    #[parts(auto)]
    pub min_width: Value<Px>,
    /// The minimum height; `Auto`: no minimum beyond the content.
    #[parts(auto)]
    pub min_height: Value<Px>,
    /// The maximum width; `Auto`: no maximum, the content decides.
    #[parts(auto)]
    pub max_width: Value<Px>,
    /// The maximum height; `Auto`: no maximum, the content decides.
    #[parts(auto)]
    pub max_height: Value<Px>,
    /// The pointer's shape over the part; absent: none set, so the window's
    /// stays.
    #[parts(may_be_absent)]
    pub cursor: Value<CursorShape>,
    /// The minimum interactive size around the drawn box; layout only, never
    /// drawn.
    #[parts(may_be_absent)]
    pub touch_target: Value<Px>,
    /// The colour bindings for the element and its descendants (the root
    /// part's for the whole widget); merged by role or surface (S6.1).
    #[parts(block = "set", keyed)]
    pub bindings: Vec<Binding>,
    /// The transitions; merged by property group (S6.1).
    #[parts(block = "fields", keyed)]
    pub transitions: Vec<Transition>,
}

/// A colour binding: a surface set, or one role's value, for the element and
/// its descendants (S3.3).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
pub enum Binding {
    /// Binds a surface set: `surface.x` reads that set's roles.
    Surface(SurfaceSet),
    /// Binds one role to a value, evaluated in the scope outside the binding.
    Role {
        /// The role.
        role: RoleRef,
        /// Its value.
        value: Value<Color>,
    },
}

/// A list of bindings merges by key (S6.1): a part binds one surface set, so
/// a later `Surface` binding replaces an earlier one, and a later `Role`
/// binding replaces the earlier binding of the same role.
impl KeyedItem for Binding {
    fn merge_keyed(earlier: &mut Vec<Self>, later: Vec<Self>) {
        merge_by(earlier, later, |a, b| match (a, b) {
            (Binding::Surface(_), Binding::Surface(_)) => true,
            (Binding::Role { role: a, .. }, Binding::Role { role: b, .. }) => a == b,
            (Binding::Surface(_), Binding::Role { .. })
            | (Binding::Role { .. }, Binding::Surface(_)) => false,
        });
    }
}

/// A colour role a binding binds: `surface.x`, `role.<set>.x`,
/// `role.global.x`, `role.@x`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum RoleRef {
    /// A role of whichever surface set is in scope.
    Surface(SurfaceRole),
    /// A role of a named surface set.
    Set(SurfaceSet, SurfaceRole),
    /// A global role.
    Global(GlobalRole),
    /// A keyed role the preset declares.
    Keyed(Key),
}

/// The pointer's shape: CSS's cursor keywords (CSS Basic UI 4), plus Qt's
/// `WhatsThis` and `None`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum CursorShape {
    /// CSS's `default`.
    Default,
    /// CSS's `context-menu`.
    ContextMenu,
    /// CSS's `help`.
    Help,
    /// CSS's `pointer`.
    Pointer,
    /// CSS's `progress`.
    Progress,
    /// CSS's `wait`.
    Wait,
    /// CSS's `cell`.
    Cell,
    /// CSS's `crosshair`.
    Crosshair,
    /// CSS's `text`.
    Text,
    /// CSS's `vertical-text`.
    VerticalText,
    /// CSS's `alias`.
    Alias,
    /// CSS's `copy`.
    Copy,
    /// CSS's `move`.
    Move,
    /// CSS's `no-drop`.
    NoDrop,
    /// CSS's `not-allowed`.
    NotAllowed,
    /// CSS's `grab`.
    Grab,
    /// CSS's `grabbing`.
    Grabbing,
    /// CSS's `all-scroll`.
    AllScroll,
    /// CSS's `col-resize`.
    ColResize,
    /// CSS's `row-resize`.
    RowResize,
    /// CSS's `n-resize`.
    NResize,
    /// CSS's `e-resize`.
    EResize,
    /// CSS's `s-resize`.
    SResize,
    /// CSS's `w-resize`.
    WResize,
    /// CSS's `ne-resize`.
    NeResize,
    /// CSS's `nw-resize`.
    NwResize,
    /// CSS's `se-resize`.
    SeResize,
    /// CSS's `sw-resize`.
    SwResize,
    /// CSS's `ew-resize`.
    EwResize,
    /// CSS's `ns-resize`.
    NsResize,
    /// CSS's `nesw-resize`.
    NeswResize,
    /// CSS's `nwse-resize`.
    NwseResize,
    /// CSS's `zoom-in`.
    ZoomIn,
    /// CSS's `zoom-out`.
    ZoomOut,
    /// Qt's `Qt::WhatsThisCursor`.
    WhatsThis,
    /// CSS's `none`: no pointer drawn.
    None,
}
