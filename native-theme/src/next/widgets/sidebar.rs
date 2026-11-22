//! The sidebar (spec §5).

use super::{
    IconRules, PartRules, TextRules, WidgetOrigin, unstated_icon, unstated_part, unstated_text,
};
use crate::next::query::StyleQuery;
use crate::next::style::{IconStyle, PartStyle, TextStyle};

/// A sidebar: its panel, its items and the headers of its sections. The
/// selected item is the one queried with `SELECTED`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SidebarWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The space between adjacent items, in logical pixels; `None` where no
    /// source states it.
    pub item_gap: Option<f32>,
    container: PartRules,
    item: PartRules,
    item_label: TextRules,
    item_icon: IconRules,
    section_header: TextRules,
}

/// The parts of a sidebar for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SidebarParts {
    /// The panel.
    pub container: PartStyle,
    /// An item.
    pub item: PartStyle,
    /// An item's text.
    pub item_label: TextStyle,
    /// An item's icon.
    pub item_icon: IconStyle,
    /// A section's header text.
    pub section_header: TextStyle,
}

impl SidebarWidget {
    /// A sidebar of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        container: PartRules,
        item: PartRules,
        item_label: TextRules,
        item_icon: IconRules,
        section_header: TextRules,
        item_gap: Option<f32>,
    ) -> Self {
        Self {
            origin,
            item_gap,
            container,
            item,
            item_label,
            item_icon,
            section_header,
        }
    }

    /// A sidebar that states nothing: every part's base style empty, no
    /// rule, no metric.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_part(),
            unstated_text(),
            unstated_icon(),
            unstated_text(),
            None,
        )
    }

    /// Whether the sidebar is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The panel's style for `query`.
    #[must_use]
    pub fn container(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.container.eval(&query.into())
    }

    /// An item's style for `query`.
    #[must_use]
    pub fn item(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.item.eval(&query.into())
    }

    /// An item's text style for `query`.
    #[must_use]
    pub fn item_label(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.item_label.eval(&query.into())
    }

    /// An item icon's style for `query`.
    #[must_use]
    pub fn item_icon(&self, query: impl Into<StyleQuery>) -> IconStyle {
        self.item_icon.eval(&query.into())
    }

    /// A section header's text style for `query`.
    #[must_use]
    pub fn section_header(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.section_header.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> SidebarParts {
        let query = query.into();
        SidebarParts {
            container: self.container.eval(&query),
            item: self.item.eval(&query),
            item_label: self.item_label.eval(&query),
            item_icon: self.item_icon.eval(&query),
            section_header: self.section_header.eval(&query),
        }
    }

    /// The dead rules of each rule set, named `sidebar.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        vec![
            ("sidebar.container", self.container.dead_rules()),
            ("sidebar.item", self.item.dead_rules()),
            ("sidebar.item_label", self.item_label.dead_rules()),
            ("sidebar.item_icon", self.item_icon.dead_rules()),
            ("sidebar.section_header", self.section_header.dead_rules()),
        ]
    }
}
