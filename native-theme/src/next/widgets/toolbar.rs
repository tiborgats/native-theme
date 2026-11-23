//! The toolbar (spec §5).

use super::{
    IconRules, PartRules, TextRules, WidgetOrigin, unstated_icon, unstated_part, unstated_text,
};
use crate::next::query::StyleQuery;
use crate::next::style::{IconStyle, PartStyle, TextStyle};

/// A toolbar and its buttons. The bar's least height is `bar.min_size[1]`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ToolbarWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The space between adjacent items, in logical pixels; `None` where no
    /// source states it.
    pub item_gap: Option<f32>,
    pub(crate) bar: PartRules,
    pub(crate) button: PartRules,
    pub(crate) button_label: TextRules,
    pub(crate) button_icon: IconRules,
    pub(crate) separator: PartRules,
}

/// The parts of a toolbar for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ToolbarParts {
    /// The bar.
    pub bar: PartStyle,
    /// A button.
    pub button: PartStyle,
    /// A button's text.
    pub button_label: TextStyle,
    /// A button's icon.
    pub button_icon: IconStyle,
    /// The line between groups of items.
    pub separator: PartStyle,
}

impl ToolbarWidget {
    /// A toolbar of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        bar: PartRules,
        button: PartRules,
        button_label: TextRules,
        button_icon: IconRules,
        separator: PartRules,
        item_gap: Option<f32>,
    ) -> Self {
        Self {
            origin,
            item_gap,
            bar,
            button,
            button_label,
            button_icon,
            separator,
        }
    }

    /// A toolbar that states nothing: every part's base style empty, no
    /// rule, no metric.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_part(),
            unstated_text(),
            unstated_icon(),
            unstated_part(),
            None,
        )
    }

    /// Whether the toolbar is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The bar's style for `query`.
    #[must_use]
    pub fn bar(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.bar.eval(&query.into())
    }

    /// A button's style for `query`.
    #[must_use]
    pub fn button(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.button.eval(&query.into())
    }

    /// A button's text style for `query`.
    #[must_use]
    pub fn button_label(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.button_label.eval(&query.into())
    }

    /// A button icon's style for `query`.
    #[must_use]
    pub fn button_icon(&self, query: impl Into<StyleQuery>) -> IconStyle {
        self.button_icon.eval(&query.into())
    }

    /// The separator's style for `query`: its colour is the `fill`.
    #[must_use]
    pub fn separator(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.separator.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> ToolbarParts {
        let query = query.into();
        ToolbarParts {
            bar: self.bar.eval(&query),
            button: self.button.eval(&query),
            button_label: self.button_label.eval(&query),
            button_icon: self.button_icon.eval(&query),
            separator: self.separator.eval(&query),
        }
    }

    /// The dead rules of each rule set, named `toolbar.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        vec![
            ("toolbar.bar", self.bar.dead_rules()),
            ("toolbar.button", self.button.dead_rules()),
            ("toolbar.button_label", self.button_label.dead_rules()),
            ("toolbar.button_icon", self.button_icon.dead_rules()),
            ("toolbar.separator", self.separator.dead_rules()),
        ]
    }
}
