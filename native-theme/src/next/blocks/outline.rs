//! Corners and stroke (S2.4): a part's outline and the line drawn along it.
//!
//! A `Corners`' corners and a `Stroke`'s edges are physical: they are not
//! mirrored under right-to-left; a platform that mirrors one states it by a
//! rule under the direction (S7).

use native_theme_derive::ThemeParts;

use super::paint::{Paint, PaintPatch, ResolvedPaint};
use crate::next::units::{Axes, Point, Px, Rect};
use crate::next::value::Value;

/// A part's four corners and their curve.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, own_toml)]
#[non_exhaustive]
pub struct Corners {
    /// The top-left corner.
    #[parts(block = "set")]
    pub top_left: Radius,
    /// The top-right corner.
    #[parts(block = "set")]
    pub top_right: Radius,
    /// The bottom-right corner.
    #[parts(block = "set")]
    pub bottom_right: Radius,
    /// The bottom-left corner.
    #[parts(block = "set")]
    pub bottom_left: Radius,
    /// The corners' curve.
    pub curve: CornerCurve,
}

/// A corner's radius. `Full`, `Concentric` and `FollowPart` resolve as they
/// are; the connector computes each such corner at layout.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, own_toml)]
pub enum Radius {
    /// A plain radius.
    Px(Value<Px>),
    /// A capsule: half the smaller side of the part's box, at most
    /// `maximum`.
    Full {
        /// The largest radius; absent: no limit.
        #[parts(may_be_absent)]
        maximum: Value<Px>,
    },
    /// The container's radius less the inset, at least `minimum`.
    Concentric {
        /// The smallest radius; absent: 0.
        #[parts(may_be_absent)]
        minimum: Value<Px>,
    },
    /// A focus indicator's inner corner: the part's radius plus the offset,
    /// concentric ([`follow_part`]). Only in a focus indicator's corners, and
    /// refused where the corner's two sides' offsets can differ (S12).
    FollowPart,
}

/// A corner's curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum CornerCurve {
    /// A circular arc.
    Circular,
    /// A continuous-curvature curve.
    Continuous,
}

/// The line drawn along a part's outline, per edge.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, own_toml)]
#[non_exhaustive]
pub struct Stroke {
    /// The top edge.
    #[parts(block = "fields")]
    pub top: Edge,
    /// The right edge.
    #[parts(block = "fields")]
    pub right: Edge,
    /// The bottom edge.
    #[parts(block = "fields")]
    pub bottom: Edge,
    /// The left edge.
    #[parts(block = "fields")]
    pub left: Edge,
    /// Where the stroke lies relative to the box.
    pub position: StrokePosition,
    /// The gap between the box and an `Outside` stroke; absent where no
    /// outset is drawn.
    #[parts(may_be_absent)]
    pub outset: Value<Px>,
    /// How far the part's fill reaches under the stroke.
    pub fill_clip: FillClip,
}

/// One edge of a stroke. Where two drawn edges meet, the corner is painted
/// once, divided between them where their paints or widths differ.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Edge {
    /// The edge's width; absent on an edge not drawn.
    #[parts(may_be_absent)]
    pub width: Value<Px>,
    /// The edge's paint; absent on an edge not drawn.
    #[parts(block = "append", may_be_absent)]
    pub paint: Value<Paint>,
    /// The edge's pattern; a drawn edge states it, `solid` included, and it
    /// is absent only on an edge not drawn (S12).
    #[parts(may_be_absent)]
    pub dash: Value<DashPattern>,
    /// How the edge meets the device's pixels.
    pub snap: PixelSnap,
}

/// A line's pattern.
#[derive(Clone, Copy, Debug, PartialEq, ThemeParts)]
#[parts(toml)]
pub enum DashPattern {
    /// A solid line: the one route for one.
    Solid,
    /// Dots.
    Dotted,
    /// Dashes `on` long, `off` apart.
    Dashed {
        /// A dash's length.
        on: Px,
        /// The gap between dashes.
        off: Px,
    },
}

/// How a line meets the device's pixels; its width stays logical.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum PixelSnap {
    /// Drawn where it falls.
    None,
    /// Aligned to device pixels.
    ToDevicePixel,
    /// Aligned to device pixels and at least one device pixel wide.
    AtLeastOneDevicePixel,
}

/// A line's cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum LineCap {
    /// Ends at the line's end.
    Butt,
    /// A half circle past the end.
    Round,
    /// A half square past the end.
    Square,
}

/// How a line's segments join.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum LineJoin {
    /// A sharp corner.
    Miter,
    /// A rounded corner.
    Round,
    /// A cut corner.
    Bevel,
}

/// Where a stroke lies relative to the part's box.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum StrokePosition {
    /// Inside the box.
    Inside,
    /// Centred on the box's edge.
    Center,
    /// Outside the box.
    Outside,
}

/// How far a part's fill reaches under its stroke.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum FillClip {
    /// The whole box, beneath the part of the stroke inside it (CSS's
    /// `background-clip: border-box`).
    Box,
    /// To the stroke's inner edge, its corners the outline's radii less the
    /// inset (CSS's `background-clip: padding-box`).
    InnerEdge,
}

/// A shape replacing a part's corners' outline: a closed set implemented once
/// in native-theme to `androidx.graphics.shapes` and `MaterialShapes.kt` at a
/// pinned commit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum ShapeKind {
    /// One of Material's shapes.
    Material(MaterialShape),
}

/// The shapes `MaterialShapes.kt` defines at the pinned commit, in that
/// file's order, each named as the file names it. The list is written from
/// the file when the commit is pinned; until then it has no shape, so a
/// preset states `shape = "absent"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum MaterialShape {}

/// A path, in logical px, in the coordinates of the rectangle it was made
/// for. Each `MoveTo` starts a closed outline; it is filled by the non-zero
/// rule.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Path {
    /// The path's commands, in order.
    pub commands: Vec<PathCommand>,
}

/// A command of a [`Path`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathCommand {
    /// Starts a closed outline at a point.
    MoveTo(Point),
    /// A line to a point.
    LineTo(Point),
    /// A quadratic curve: its control point, then its end.
    QuadTo(Point, Point),
    /// A cubic curve: its two control points, then its end.
    CubicTo(Point, Point, Point),
    /// Closes the outline.
    Close,
}

/// A part's outline: its shape where its `shape` is stated, else its corners.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Outline {
    /// The rounded rectangle of four radii.
    Corners(OutlineCorners),
    /// A shape.
    Shape(ShapeKind),
}

/// A rounded rectangle's corners as the connector computed them at layout (a
/// stated `Radius::Px`, or the [`corners`](crate::next::corners)
/// functions).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OutlineCorners {
    /// The radii, in the order top-left, top-right, bottom-right,
    /// bottom-left.
    pub radii: [Px; 4],
    /// The corners' curve.
    pub curve: CornerCurve,
}

/// The path of `shape` made for `rect`.
#[must_use]
pub fn outline(shape: &ShapeKind, rect: Rect) -> Path {
    match (*shape, rect) {
        (ShapeKind::Material(shape), _) => match shape {},
    }
}

/// A `Full` corner: half the smaller side of the part's box, at most
/// `maximum`, and at least 0.
#[must_use]
pub fn full(width: Px, height: Px, maximum: Option<Px>) -> Px {
    let half = width.0.min(height.0) / 2.0;
    let capped = maximum.map_or(half, |maximum| half.min(maximum.0));
    Px(capped.max(0.0))
}

/// A `Concentric` corner: the container's radius less the part's inset from
/// the container's edges at that corner, at least `minimum` (0 when absent).
/// `container_radius` is the radius the connector computed for the
/// container's matching corner; 0 for a square container, or the display
/// with no radius.
///
/// `inset` is, per axis, the part's stated distance from the container's
/// edge on that corner's side, resolved, not its laid-out position: the
/// container's `padding` on that side, the `margin` and `padding` on that
/// side of every part between them, and the part's own `margin` there (a
/// floating part's distance from the window is its `margin`). The connector
/// sums it from the resolved styles it queried on its way down.
///
/// Where the two insets are equal, the result is the larger of
/// `container_radius` less the inset and `minimum`. Where they differ, the
/// rule is UIKit's (`UICornerConfiguration`, `UIView.effectiveRadius(corner:)`,
/// X-CR-2), read in the value phase; until then such a corner is `None`, no
/// rule being invented for it, and adding a layer refuses a `Concentric`
/// corner whose insets can differ (S12), so a connector never meets it.
#[must_use]
pub fn concentric(container_radius: Px, inset: Axes<Px>, minimum: Option<Px>) -> Option<Px> {
    if inset.horizontal != inset.vertical {
        return None;
    }
    let floor = minimum.map_or(0.0, |minimum| minimum.0);
    Some(Px((container_radius.0 - inset.horizontal.0)
        .max(floor)
        .max(0.0)))
}

/// A `FollowPart` corner, a focus ring's inner corner: the part's radius
/// plus the ring's offset, at least 0 (S2.10). Its outer corner is
/// `follow_part(part_radius, offset + width)`, so an inner corner clamped to
/// 0 still has a rounded outer one.
#[must_use]
pub fn follow_part(part_radius: Px, offset: Px) -> Px {
    Px((part_radius.0 + offset.0).max(0.0))
}
