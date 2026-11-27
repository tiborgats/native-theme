//! The slider, `slider` (spec §5).

use super::{PartRules, WidgetOrigin, unstated_part};
use crate::next::query::StyleQuery;
use crate::next::style::PartStyle;

/// A slider. The track's thickness is exact: `track.min_size` equals
/// `max_size` across the slider's axis; the thumb's size likewise on
/// `thumb`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SliderWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// A tick mark's length, in logical pixels.
    pub tick_length: f32,
    /// The opacity of the whole disabled slider drawn as one group; `None`
    /// where no source states it.
    pub disabled_opacity: Option<f32>,
    pub(crate) track: PartRules,
    pub(crate) active_track: PartRules,
    pub(crate) thumb: PartRules,
    pub(crate) tick: PartRules,
}

/// The parts of a slider for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SliderParts {
    /// The whole track.
    pub track: PartStyle,
    /// The track's filled portion, up to the thumb.
    pub active_track: PartStyle,
    /// The handle.
    pub thumb: PartStyle,
    /// A tick mark.
    pub tick: PartStyle,
}

impl SliderWidget {
    /// A slider of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        track: PartRules,
        active_track: PartRules,
        thumb: PartRules,
        tick: PartRules,
        tick_length: f32,
        disabled_opacity: Option<f32>,
    ) -> Self {
        Self {
            origin,
            tick_length,
            disabled_opacity,
            track,
            active_track,
            thumb,
            tick,
        }
    }

    /// A slider that states nothing: every part's base style empty, no rule,
    /// the tick length its type's default, no disabled opacity.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_part(),
            unstated_part(),
            unstated_part(),
            f32::default(),
            None,
        )
    }

    /// Whether the slider is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The track's style for `query`.
    #[must_use]
    pub fn track(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.track.eval(&query.into())
    }

    /// The filled portion's style for `query`.
    #[must_use]
    pub fn active_track(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.active_track.eval(&query.into())
    }

    /// The handle's style for `query`.
    #[must_use]
    pub fn thumb(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.thumb.eval(&query.into())
    }

    /// A tick mark's style for `query`: its colour is the `fill`.
    #[must_use]
    pub fn tick(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.tick.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> SliderParts {
        let query = query.into();
        SliderParts {
            track: self.track.eval(&query),
            active_track: self.active_track.eval(&query),
            thumb: self.thumb.eval(&query),
            tick: self.tick.eval(&query),
        }
    }
}
