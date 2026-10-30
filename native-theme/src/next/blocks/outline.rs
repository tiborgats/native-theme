//! Corners and stroke (S2.4): a part's outline and the line drawn along it.
//!
//! A `Corners`' corners and a `Stroke`'s edges are physical: they are not
//! mirrored under right-to-left; a platform that mirrors one states it by a
//! rule under the direction (S7).

use native_theme_derive::ThemeParts;

use super::paint::{Paint, PaintPatch, ResolvedPaint};
use crate::next::units::Px;
use crate::next::value::Value;

/// A part's four corners and their curve.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
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
#[parts(block)]
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
    /// A focus indicator's: the part's radius plus the offset, concentric.
    /// Only in a focus indicator's corners, and refused where the corner's
    /// two sides' offsets differ (S12).
    FollowPart,
}

/// A corner's curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CornerCurve {
    /// A circular arc.
    Circular,
    /// A continuous-curvature curve.
    Continuous,
}

/// The line drawn along a part's outline, per edge.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PixelSnap {
    /// Drawn where it falls.
    None,
    /// Aligned to device pixels.
    ToDevicePixel,
    /// Aligned to device pixels and at least one device pixel wide.
    AtLeastOneDevicePixel,
}

/// A line's cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LineCap {
    /// Ends at the line's end.
    Butt,
    /// A half circle past the end.
    Round,
    /// A half square past the end.
    Square,
}

/// How a line's segments join.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LineJoin {
    /// A sharp corner.
    Miter,
    /// A rounded corner.
    Round,
    /// A cut corner.
    Bevel,
}

/// Where a stroke lies relative to the part's box.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StrokePosition {
    /// Inside the box.
    Inside,
    /// Centred on the box's edge.
    Center,
    /// Outside the box.
    Outside,
}

/// How far a part's fill reaches under its stroke.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FillClip {
    /// The whole box, beneath the part of the stroke inside it (CSS's
    /// `background-clip: border-box`).
    Box,
    /// To the stroke's inner edge, its corners the outline's radii less the
    /// inset (CSS's `background-clip: padding-box`).
    InnerEdge,
}
