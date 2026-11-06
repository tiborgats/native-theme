//! The units every value is measured in, and the plain geometry built from them.

/// Logical pixels.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Px(pub f32);

/// Milliseconds.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Ms(pub f32);

/// Degrees.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Deg(pub f32);

/// A fraction in `0..=1`: an opacity or an amount.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Fraction(pub f32);

/// A keyed value whose unit its declaration states.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scalar {
    /// A length.
    Px(Px),
    /// A duration.
    Ms(Ms),
    /// An angle.
    Deg(Deg),
    /// A fraction in `0..=1`.
    Fraction(Fraction),
    /// A unitless number.
    Number(f32),
}

/// A font size, a line height or a letter or word spacing, as stated: in
/// points, which become logical pixels by the font DPI, or in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontSize {
    /// Typographic points (1/72 inch).
    Pt(f32),
    /// Logical pixels.
    Px(f32),
}

/// A resolved length that may be sized by content.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    /// Logical pixels.
    Px(f32),
    /// The content decides.
    Auto,
}

/// One value per side, stated for a left-to-right layout: `left` is the start
/// side, mirrored with the layout's direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sides<T> {
    /// The top side.
    pub top: T,
    /// The right side.
    pub right: T,
    /// The bottom side.
    pub bottom: T,
    /// The left side.
    pub left: T,
}

/// One value per axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Axes<T> {
    /// The horizontal axis.
    pub horizontal: T,
    /// The vertical axis.
    pub vertical: T,
}

/// A rectangle, its origin at the top left.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// The left edge.
    pub x: Px,
    /// The top edge.
    pub y: Px,
    /// The width.
    pub width: Px,
    /// The height.
    pub height: Px,
}

/// A width and a height.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    /// The width.
    pub width: Px,
    /// The height.
    pub height: Px,
}

/// A point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// The horizontal coordinate, growing rightward.
    pub x: Px,
    /// The vertical coordinate, growing downward.
    pub y: Px,
}

/// The rectangle a path's own coordinates span, unitless.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewBox {
    /// The left edge.
    pub x: f32,
    /// The top edge.
    pub y: f32,
    /// The width.
    pub width: f32,
    /// The height.
    pub height: f32,
}
