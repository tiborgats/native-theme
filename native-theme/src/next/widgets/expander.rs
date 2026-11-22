//! The expander (spec §5).

use super::{
    IconRules, PartRules, TextRules, WidgetOrigin, unstated_icon, unstated_part, unstated_text,
};
use crate::next::layout::ArrowSide;
use crate::next::query::StyleQuery;
use crate::next::style::{IconStyle, PartStyle, TextStyle};

/// An expander: a header row with a disclosure arrow that shows or hides
/// the content below it. Its frame is `container`, drawn where
/// `frame_enabled`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ExpanderWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// Which end of the header row the arrow sits at; `None` where no source
    /// states it.
    pub arrow_side: Option<ArrowSide>,
    /// The space between the arrow and the header's text, in logical
    /// pixels; `None` where no source states it.
    pub arrow_gap: Option<f32>,
    /// The indent of the content below the header, in logical pixels;
    /// `None` where no source states it.
    pub content_indent: Option<f32>,
    /// Whether the expander draws its frame; `None` where no source states
    /// it.
    pub frame_enabled: Option<bool>,
    container: PartRules,
    header: PartRules,
    header_label: TextRules,
    arrow: IconRules,
}

/// The parts of an expander for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ExpanderParts {
    /// The frame around header and content.
    pub container: PartStyle,
    /// The header row.
    pub header: PartStyle,
    /// The header's text.
    pub header_label: TextStyle,
    /// The disclosure arrow.
    pub arrow: IconStyle,
}

impl ExpanderWidget {
    /// An expander of these rule sets and metrics.
    #[allow(
        clippy::too_many_arguments,
        reason = "one rule set per part of spec §5's table, and its metrics"
    )]
    pub(crate) fn new(
        origin: WidgetOrigin,
        container: PartRules,
        header: PartRules,
        header_label: TextRules,
        arrow: IconRules,
        arrow_side: Option<ArrowSide>,
        arrow_gap: Option<f32>,
        content_indent: Option<f32>,
        frame_enabled: Option<bool>,
    ) -> Self {
        Self {
            origin,
            arrow_side,
            arrow_gap,
            content_indent,
            frame_enabled,
            container,
            header,
            header_label,
            arrow,
        }
    }

    /// An expander that states nothing: every part's base style empty, no
    /// rule, no metric.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_part(),
            unstated_text(),
            unstated_icon(),
            None,
            None,
            None,
            None,
        )
    }

    /// Whether the expander is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The frame's style for `query`.
    #[must_use]
    pub fn container(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.container.eval(&query.into())
    }

    /// The header row's style for `query`.
    #[must_use]
    pub fn header(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.header.eval(&query.into())
    }

    /// The header's text style for `query`.
    #[must_use]
    pub fn header_label(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.header_label.eval(&query.into())
    }

    /// The arrow's style for `query`.
    #[must_use]
    pub fn arrow(&self, query: impl Into<StyleQuery>) -> IconStyle {
        self.arrow.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> ExpanderParts {
        let query = query.into();
        ExpanderParts {
            container: self.container.eval(&query),
            header: self.header.eval(&query),
            header_label: self.header_label.eval(&query),
            arrow: self.arrow.eval(&query),
        }
    }

    /// The dead rules of each rule set, named `expander.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        vec![
            ("expander.container", self.container.dead_rules()),
            ("expander.header", self.header.dead_rules()),
            ("expander.header_label", self.header_label.dead_rules()),
            ("expander.arrow", self.arrow.dead_rules()),
        ]
    }
}
