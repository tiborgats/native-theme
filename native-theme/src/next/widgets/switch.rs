//! The switch (spec §5).

use super::{PartRules, TextRules, WidgetOrigin, unstated_part, unstated_text};
use crate::next::query::StyleQuery;
use crate::next::style::{PartStyle, TextStyle};

/// An on/off switch: its track, its thumb and its label. The on switch is
/// the one queried with `CHECKED`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SwitchWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The space between the switch and its label, in logical pixels; `None`
    /// where no source states it.
    pub label_gap: Option<f32>,
    /// Whether the track shows on and off glyphs; `None` where no source
    /// states it.
    pub show_state_glyphs: Option<bool>,
    /// The opacity of the whole disabled switch drawn as one group; `None`
    /// where no source states it.
    pub disabled_opacity: Option<f32>,
    pub(crate) track: PartRules,
    pub(crate) thumb: PartRules,
    pub(crate) label: TextRules,
}

/// The parts of a switch for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SwitchParts {
    /// The track.
    pub track: PartStyle,
    /// The thumb.
    pub thumb: PartStyle,
    /// The label.
    pub label: TextStyle,
}

impl SwitchWidget {
    /// A switch of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        track: PartRules,
        thumb: PartRules,
        label: TextRules,
        label_gap: Option<f32>,
        show_state_glyphs: Option<bool>,
        disabled_opacity: Option<f32>,
    ) -> Self {
        Self {
            origin,
            label_gap,
            show_state_glyphs,
            disabled_opacity,
            track,
            thumb,
            label,
        }
    }

    /// A switch that states nothing: every part's base style empty, no rule,
    /// no metric.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_part(),
            unstated_text(),
            None,
            None,
            None,
        )
    }

    /// Whether the switch is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The track's style for `query`.
    #[must_use]
    pub fn track(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.track.eval(&query.into())
    }

    /// The thumb's style for `query`.
    #[must_use]
    pub fn thumb(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.thumb.eval(&query.into())
    }

    /// The label's style for `query`.
    #[must_use]
    pub fn label(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.label.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> SwitchParts {
        let query = query.into();
        SwitchParts {
            track: self.track.eval(&query),
            thumb: self.thumb.eval(&query),
            label: self.label.eval(&query),
        }
    }

    /// The dead rules of each rule set, named `switch.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        vec![
            ("switch.track", self.track.dead_rules()),
            ("switch.thumb", self.thumb.dead_rules()),
            ("switch.label", self.label.dead_rules()),
        ]
    }
}
