//! The tab bar (spec §5).

use super::{
    IconRules, PartRules, TextRules, WidgetOrigin, unstated_icon, unstated_part, unstated_text,
};
use crate::next::layout::TabIndicatorSide;
use crate::next::query::{TabQuery, TabVariant};
use crate::next::style::{IconStyle, PartStyle, TextStyle};

/// A rule set's matcher of a [`TabVariant`]: `None` matches every variant.
type TabMatcher = Option<TabVariant>;

/// A tab bar and its tabs. The current tab is the one queried with
/// `SELECTED`; its indicator is `indicator(SELECTED)`, whose thickness is an
/// exact height (`min_size[1]` equal to `max_size[1]`), and at rest the
/// indicator is unstated, so the line is drawn for the selected tab only
/// (spec §5, notes to the table).
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct TabWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The space between adjacent tabs, in logical pixels; `None` where no
    /// source states it.
    pub item_gap: Option<f32>,
    /// The edge of the selected tab its indicator lies along, for tabs above
    /// their pages; `None` where no source states it.
    pub indicator_side: Option<TabIndicatorSide>,
    /// The radius of the outward curves at the selected tab's two feet,
    /// which spread its fill into the bar's edge; `None` where the
    /// platform's selected tab does not curve out.
    pub active_outward_corner_radius: Option<f32>,
    bar: PartRules<TabMatcher>,
    tab: PartRules<TabMatcher>,
    label: TextRules<TabMatcher>,
    indicator: PartRules<TabMatcher>,
    close_button: PartRules<TabMatcher>,
    close_icon: IconRules<TabMatcher>,
}

/// The parts of a tab bar for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct TabParts {
    /// The strip behind the tabs.
    pub bar: PartStyle,
    /// A tab.
    pub tab: PartStyle,
    /// A tab's text.
    pub label: TextStyle,
    /// The line that marks the current tab.
    pub indicator: PartStyle,
    /// A tab's close button.
    pub close_button: PartStyle,
    /// The close button's icon.
    pub close_icon: IconStyle,
}

impl TabWidget {
    /// A tab bar of these rule sets and metrics.
    #[allow(
        clippy::too_many_arguments,
        reason = "one rule set per part of spec §5's table, and its metrics"
    )]
    pub(crate) fn new(
        origin: WidgetOrigin,
        bar: PartRules<TabMatcher>,
        tab: PartRules<TabMatcher>,
        label: TextRules<TabMatcher>,
        indicator: PartRules<TabMatcher>,
        close_button: PartRules<TabMatcher>,
        close_icon: IconRules<TabMatcher>,
        item_gap: Option<f32>,
        indicator_side: Option<TabIndicatorSide>,
        active_outward_corner_radius: Option<f32>,
    ) -> Self {
        Self {
            origin,
            item_gap,
            indicator_side,
            active_outward_corner_radius,
            bar,
            tab,
            label,
            indicator,
            close_button,
            close_icon,
        }
    }

    /// A tab bar that states nothing: every part's base style empty, no
    /// rule, no metric.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_part(),
            unstated_text(),
            unstated_part(),
            unstated_part(),
            unstated_icon(),
            None,
            None,
            None,
        )
    }

    /// Whether the tab bar is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The strip's style for `query`.
    #[must_use]
    pub fn bar(&self, query: impl Into<TabQuery>) -> PartStyle {
        self.bar.eval(&query.into())
    }

    /// A tab's style for `query`.
    #[must_use]
    pub fn tab(&self, query: impl Into<TabQuery>) -> PartStyle {
        self.tab.eval(&query.into())
    }

    /// A tab's text style for `query`.
    #[must_use]
    pub fn label(&self, query: impl Into<TabQuery>) -> TextStyle {
        self.label.eval(&query.into())
    }

    /// The indicator's style for `query`: its colour is the `fill`.
    #[must_use]
    pub fn indicator(&self, query: impl Into<TabQuery>) -> PartStyle {
        self.indicator.eval(&query.into())
    }

    /// The close button's style for `query`.
    #[must_use]
    pub fn close_button(&self, query: impl Into<TabQuery>) -> PartStyle {
        self.close_button.eval(&query.into())
    }

    /// The close icon's style for `query`.
    #[must_use]
    pub fn close_icon(&self, query: impl Into<TabQuery>) -> IconStyle {
        self.close_icon.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<TabQuery>) -> TabParts {
        let query = query.into();
        TabParts {
            bar: self.bar.eval(&query),
            tab: self.tab.eval(&query),
            label: self.label.eval(&query),
            indicator: self.indicator.eval(&query),
            close_button: self.close_button.eval(&query),
            close_icon: self.close_icon.eval(&query),
        }
    }

    /// The dead rules of each rule set, named `tab.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        vec![
            ("tab.bar", self.bar.dead_rules()),
            ("tab.tab", self.tab.dead_rules()),
            ("tab.label", self.label.dead_rules()),
            ("tab.indicator", self.indicator.dead_rules()),
            ("tab.close_button", self.close_button.dead_rules()),
            ("tab.close_icon", self.close_icon.dead_rules()),
        ]
    }
}
