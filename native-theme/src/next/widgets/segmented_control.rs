//! The segmented control (spec §5).

use super::{PartRules, TextRules, WidgetOrigin, unstated_part, unstated_text};
use crate::next::query::StyleQuery;
use crate::next::style::{PartStyle, TextStyle};

/// A segmented control: its track, the box of its own, its segments, their
/// labels and the lines between them. The current segment is the one
/// queried with `SELECTED`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SegmentedControlWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The width of the line between adjacent segments, in logical pixels.
    pub separator_width: f32,
    /// The opacity of the whole disabled control drawn as one group; `None`
    /// where no source states it.
    pub disabled_opacity: Option<f32>,
    pub(crate) track: PartRules,
    pub(crate) segment: PartRules,
    pub(crate) label: TextRules,
    pub(crate) separator: PartRules,
}

/// The parts of a segmented control for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SegmentedControlParts {
    /// The control's box.
    pub track: PartStyle,
    /// A segment.
    pub segment: PartStyle,
    /// A segment's text.
    pub label: TextStyle,
    /// The line between segments.
    pub separator: PartStyle,
}

impl SegmentedControlWidget {
    /// A segmented control of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        track: PartRules,
        segment: PartRules,
        label: TextRules,
        separator: PartRules,
        separator_width: f32,
        disabled_opacity: Option<f32>,
    ) -> Self {
        Self {
            origin,
            separator_width,
            disabled_opacity,
            track,
            segment,
            label,
            separator,
        }
    }

    /// A segmented control that states nothing: every part's base style
    /// empty, no rule, the separator width its type's default, no disabled
    /// opacity.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_part(),
            unstated_text(),
            unstated_part(),
            f32::default(),
            None,
        )
    }

    /// Whether the segmented control is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The track's style for `query`.
    #[must_use]
    pub fn track(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.track.eval(&query.into())
    }

    /// A segment's style for `query`.
    #[must_use]
    pub fn segment(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.segment.eval(&query.into())
    }

    /// A segment's text style for `query`.
    #[must_use]
    pub fn label(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.label.eval(&query.into())
    }

    /// The separator's style for `query`: its colour is the `fill`.
    #[must_use]
    pub fn separator(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.separator.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> SegmentedControlParts {
        let query = query.into();
        SegmentedControlParts {
            track: self.track.eval(&query),
            segment: self.segment.eval(&query),
            label: self.label.eval(&query),
            separator: self.separator.eval(&query),
        }
    }

    /// The dead rules of each rule set, named `segmented_control.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        vec![
            ("segmented_control.track", self.track.dead_rules()),
            ("segmented_control.segment", self.segment.dead_rules()),
            ("segmented_control.label", self.label.dead_rules()),
            ("segmented_control.separator", self.separator.dead_rules()),
        ]
    }
}
