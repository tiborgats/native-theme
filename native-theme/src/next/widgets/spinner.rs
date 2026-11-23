//! The busy indicator, `spinner` (spec §5).

use super::WidgetOrigin;
use crate::next::color::Color;

/// A spinner, the indeterminate busy indicator.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SpinnerWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The arc's colour.
    pub color: Color,
    /// The outer diameter, in logical pixels.
    pub diameter: f32,
    /// The least diameter it is drawn at, in logical pixels.
    pub min_diameter: f32,
    /// The arc's stroke width, in logical pixels.
    pub stroke_width: f32,
    /// The period of one turn, in milliseconds; `None` where no source states
    /// it.
    pub period_ms: Option<u32>,
}

impl SpinnerWidget {
    /// A spinner of these metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        color: Color,
        diameter: f32,
        min_diameter: f32,
        stroke_width: f32,
        period_ms: Option<u32>,
    ) -> Self {
        Self {
            origin,
            color,
            diameter,
            min_diameter,
            stroke_width,
            period_ms,
        }
    }

    /// A spinner that states nothing: the colour and sizes their types'
    /// defaults, no period.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            Color::default(),
            f32::default(),
            f32::default(),
            f32::default(),
            None,
        )
    }

    /// Whether the spinner is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }
}
