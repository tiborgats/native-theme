//! The separator line, `separator` (spec §5).

use super::WidgetOrigin;
use crate::next::color::Color;

/// A separator line.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SeparatorWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The line's colour.
    pub color: Color,
    /// The line's thickness, in logical pixels.
    pub thickness: f32,
    /// The inset of the line's ends from its container's edges, in logical
    /// pixels; `None` where no source states it.
    pub inset: Option<f32>,
    /// The space on either side of the line, in logical pixels; `None` where
    /// no source states it.
    pub margin: Option<f32>,
}

impl SeparatorWidget {
    /// A separator of these metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        color: Color,
        thickness: f32,
        inset: Option<f32>,
        margin: Option<f32>,
    ) -> Self {
        Self {
            origin,
            color,
            thickness,
            inset,
            margin,
        }
    }

    /// Whether the separator is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }
}
