//! Geometry of a part: edges, corners, strokes, shadows, transforms and the
//! pointer cursor.

use std::sync::Arc;

use super::color::Color;
use super::paint::Paint;

/// Four side lengths in logical pixels: padding, margins, insets.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Edges {
    /// The top side.
    pub top: f32,
    /// The right side.
    pub right: f32,
    /// The bottom side.
    pub bottom: f32,
    /// The left side.
    pub left: f32,
}

impl Edges {
    /// Zero on every side.
    pub const ZERO: Self = Self {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    };

    /// `v` on every side.
    #[must_use]
    pub const fn all(v: f32) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }

    /// `vertical` at the top and bottom, `horizontal` at the left and right.
    #[must_use]
    pub const fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Each side as given.
    #[must_use]
    pub const fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

/// A part's corner radii and the curve that draws them.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Corners {
    /// The radii.
    pub radius: CornerRadius,
    /// The curve.
    pub curve: CornerCurve,
}

impl Corners {
    /// Square corners.
    pub const NONE: Self = Self {
        radius: CornerRadius::None,
        curve: CornerCurve::Circular,
    };

    /// Circular corners of radius `r`.
    #[must_use]
    pub const fn uniform(r: f32) -> Self {
        Self {
            radius: CornerRadius::Uniform(r),
            curve: CornerCurve::Circular,
        }
    }

    /// Continuous (squircle) corners of radius `r`.
    #[must_use]
    pub const fn continuous(r: f32) -> Self {
        Self {
            radius: CornerRadius::Uniform(r),
            curve: CornerCurve::Continuous,
        }
    }

    /// Circular ends of half the part's shorter side.
    #[must_use]
    pub const fn pill() -> Self {
        Self {
            radius: CornerRadius::Pill,
            curve: CornerCurve::Circular,
        }
    }
}

/// The radii of a part's corners, in logical pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum CornerRadius {
    /// Square corners.
    #[default]
    None,
    /// One radius for every corner.
    Uniform(f32),
    /// A radius per corner.
    PerCorner {
        /// The top-left corner.
        top_left: f32,
        /// The top-right corner.
        top_right: f32,
        /// The bottom-right corner.
        bottom_right: f32,
        /// The bottom-left corner.
        bottom_left: f32,
    },
    /// Half the part's shorter side.
    Pill,
    /// The container's radius less `inset`, at least `min_radius`.
    Concentric {
        /// The distance from the container's edge.
        inset: f32,
        /// The smallest radius.
        min_radius: f32,
    },
}

/// The curve of a rounded corner.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CornerCurve {
    /// A circular arc.
    #[default]
    Circular,
    /// A continuous-curvature (squircle) curve.
    Continuous,
}

/// A part's outline.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Stroke {
    /// No outline.
    #[default]
    None,
    /// One width and colour on every side.
    Uniform {
        /// The width, in logical pixels.
        width: f32,
        /// The colour.
        color: Color,
        /// Where it lies relative to the part's edge.
        align: StrokeAlign,
    },
    /// A width and paint per side.
    Complex(Arc<ComplexStroke>),
}

impl Stroke {
    /// One width and colour on every side, inside the edge.
    #[must_use]
    pub const fn uniform(width: f32, color: Color) -> Self {
        Self::Uniform {
            width,
            color,
            align: StrokeAlign::Inside,
        }
    }

    /// One width and colour on every side, aligned as given.
    #[must_use]
    pub const fn uniform_aligned(width: f32, color: Color, align: StrokeAlign) -> Self {
        Self::Uniform {
            width,
            color,
            align,
        }
    }

    /// A width and paint per side.
    #[must_use]
    pub fn sides(
        top: StrokeSide,
        right: StrokeSide,
        bottom: StrokeSide,
        left: StrokeSide,
        align: StrokeAlign,
    ) -> Self {
        Self::Complex(Arc::new(ComplexStroke {
            top,
            right,
            bottom,
            left,
            align,
        }))
    }
}

/// The sides of a [`Stroke::Complex`]; made by [`Stroke::sides`].
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ComplexStroke {
    /// The top side.
    pub top: StrokeSide,
    /// The right side.
    pub right: StrokeSide,
    /// The bottom side.
    pub bottom: StrokeSide,
    /// The left side.
    pub left: StrokeSide,
    /// Where the stroke lies relative to the part's edge.
    pub align: StrokeAlign,
}

/// One side of a [`ComplexStroke`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StrokeSide {
    /// The width, in logical pixels.
    pub width: f32,
    /// The paint.
    pub paint: Paint,
}

/// Where a stroke lies relative to the part's edge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StrokeAlign {
    /// Inside the edge.
    #[default]
    Inside,
    /// Centred on the edge.
    Center,
    /// Outside the edge.
    Outside,
}

/// One shadow of a part.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShadowLayer {
    /// The horizontal offset, in logical pixels.
    pub offset_x: f32,
    /// The vertical offset, in logical pixels.
    pub offset_y: f32,
    /// The blur radius, in logical pixels.
    pub blur: f32,
    /// The spread, in logical pixels.
    pub spread: f32,
    /// The colour.
    pub color: Color,
    /// Whether the shadow lies inside the part.
    pub inset: bool,
}

/// A 2D transform of a part about an origin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// The horizontal and vertical scale.
    pub scale: [f32; 2], // default [1.0, 1.0]
    /// The rotation, in degrees.
    pub rotation_deg: f32, // default 0.0
    /// The translation, in logical pixels.
    pub translation: [f32; 2], // default [0.0, 0.0]
    /// The origin, normalised to the part.
    pub origin: [f32; 2], // normalized [0.5, 0.5]
}

impl Transform {
    /// No transform.
    pub const IDENTITY: Self = Self {
        scale: [1.0, 1.0],
        rotation_deg: 0.0,
        translation: [0.0, 0.0],
        origin: [0.5, 0.5],
    };

    /// Whether this transform changes nothing: unit scale, no rotation and
    /// no translation (the origin then does not matter).
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.scale == Self::IDENTITY.scale
            && self.rotation_deg == Self::IDENTITY.rotation_deg
            && self.translation == Self::IDENTITY.translation
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// The pointer cursor over a part.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Cursor {
    /// The platform's default arrow.
    #[default]
    Default,
    /// A pointing hand.
    Pointer,
    /// A text beam.
    Text,
    /// An open hand.
    Grab,
    /// A closed hand.
    Grabbing,
    /// Column resize.
    ColResize,
    /// Row resize.
    RowResize,
    /// Not allowed.
    NotAllowed,
    /// Busy in the background.
    Progress,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        assert_eq!(Edges::all(2.0), Edges::new(2.0, 2.0, 2.0, 2.0));
        assert_eq!(Edges::symmetric(1.0, 3.0), Edges::new(1.0, 3.0, 1.0, 3.0));
        assert_eq!(Edges::default(), Edges::ZERO);
        assert_eq!(Corners::default(), Corners::NONE);
        assert_eq!(Corners::continuous(4.0).curve, CornerCurve::Continuous);
        assert_eq!(Corners::uniform(4.0).radius, CornerRadius::Uniform(4.0));
        assert_eq!(Corners::pill().radius, CornerRadius::Pill);
        assert_eq!(
            Stroke::uniform(1.0, Color::BLACK),
            Stroke::uniform_aligned(1.0, Color::BLACK, StrokeAlign::Inside)
        );
        let side = StrokeSide {
            width: 1.0,
            paint: Paint::solid(Color::BLACK),
        };
        let Stroke::Complex(sides) = Stroke::sides(
            side.clone(),
            StrokeSide::default(),
            side.clone(),
            StrokeSide::default(),
            StrokeAlign::Outside,
        ) else {
            panic!("Stroke::sides must give Stroke::Complex");
        };
        assert_eq!(sides.top, side);
        assert_eq!(sides.right, StrokeSide::default());
        assert_eq!(sides.align, StrokeAlign::Outside);
    }

    #[test]
    fn transform_identity() {
        assert_eq!(Transform::default(), Transform::IDENTITY);
        assert!(Transform::IDENTITY.is_identity());
        let moved_origin = Transform {
            origin: [0.0, 0.0],
            ..Transform::IDENTITY
        };
        assert!(moved_origin.is_identity());
        let scaled = Transform {
            scale: [2.0, 1.0],
            ..Transform::IDENTITY
        };
        assert!(!scaled.is_identity());
    }
}
