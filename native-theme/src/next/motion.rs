//! Motion: transitions and their timing curves.

/// A property transition of a part.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transition {
    /// The duration, in milliseconds.
    pub duration_ms: u32,
    /// The delay before it starts, in milliseconds.
    pub delay_ms: u32,
    /// The timing curve.
    pub timing: Timing,
}

/// A timing curve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum Timing {
    /// Linear.
    #[default]
    Linear,
    /// A cubic Bézier curve through `(0, 0)`, `(x1, y1)`, `(x2, y2)`, `(1, 1)`.
    CubicBezier(f32, f32, f32, f32),
    /// A damped spring.
    Spring {
        /// The mass.
        mass: f32,
        /// The stiffness.
        stiffness: f32,
        /// The damping.
        damping: f32,
    },
    /// A step function of this many steps.
    Steps(u16),
    /// Sine ease-in-out.
    SineInOut,
    /// Exponential ease-out.
    ExpoOut,
}
