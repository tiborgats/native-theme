//! The menu bar (spec §5).

use super::{PartRules, TextRules, WidgetOrigin, unstated_part, unstated_text};
use crate::next::query::StyleQuery;
use crate::next::style::{PartStyle, TextStyle};

/// A window's menu bar: the strip and its menu titles.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct MenuBarWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The space between adjacent titles, in logical pixels; `None` where no
    /// source states it.
    pub item_gap: Option<f32>,
    bar: PartRules,
    item: PartRules,
    item_label: TextRules,
}

/// The parts of a menu bar for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct MenuBarParts {
    /// The strip.
    pub bar: PartStyle,
    /// A menu title.
    pub item: PartStyle,
    /// A menu title's text.
    pub item_label: TextStyle,
}

impl MenuBarWidget {
    /// A menu bar of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        bar: PartRules,
        item: PartRules,
        item_label: TextRules,
        item_gap: Option<f32>,
    ) -> Self {
        Self {
            origin,
            item_gap,
            bar,
            item,
            item_label,
        }
    }

    /// A menu bar that states nothing: every part's base style empty, no
    /// rule, no metric.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_part(),
            unstated_text(),
            None,
        )
    }

    /// Whether the menu bar is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The strip's style for `query`.
    #[must_use]
    pub fn bar(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.bar.eval(&query.into())
    }

    /// A menu title's style for `query`.
    #[must_use]
    pub fn item(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.item.eval(&query.into())
    }

    /// A menu title's text style for `query`.
    #[must_use]
    pub fn item_label(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.item_label.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> MenuBarParts {
        let query = query.into();
        MenuBarParts {
            bar: self.bar.eval(&query),
            item: self.item.eval(&query),
            item_label: self.item_label.eval(&query),
        }
    }

    /// The dead rules of each rule set, named `menu_bar.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        vec![
            ("menu_bar.bar", self.bar.dead_rules()),
            ("menu_bar.item", self.item.dead_rules()),
            ("menu_bar.item_label", self.item_label.dead_rules()),
        ]
    }
}
