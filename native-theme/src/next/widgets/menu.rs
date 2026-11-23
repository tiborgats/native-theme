//! The menu (spec §5).

use super::{
    IconRules, PartRules, TextRules, WidgetOrigin, unstated_icon, unstated_part, unstated_text,
};
use crate::next::query::StyleQuery;
use crate::next::style::{IconStyle, PartStyle, TextStyle};

/// A popup menu. An item's height is `item.min_size[1]`, with `max_size[1]`
/// equal to it where the platform's row height is exact.
///
/// `panel` holds the menu's own values (its fill, the padding the theme
/// states for it); the popup frame a connector draws where the menu states
/// none is `popover.panel`'s (spec §5, notes to the table, "the menu's popup
/// frame").
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct MenuWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The space between an item's icon and its label, in logical pixels.
    pub icon_text_gap: f32,
    pub(crate) panel: PartRules,
    pub(crate) item: PartRules,
    pub(crate) item_label: TextRules,
    pub(crate) shortcut: TextRules,
    pub(crate) icon: IconRules,
    pub(crate) submenu_arrow: IconRules,
    pub(crate) separator: PartRules,
}

/// The parts of a menu for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct MenuParts {
    /// The popup's box.
    pub panel: PartStyle,
    /// An item.
    pub item: PartStyle,
    /// An item's text.
    pub item_label: TextStyle,
    /// An item's keyboard shortcut.
    pub shortcut: TextStyle,
    /// An item's icon.
    pub icon: IconStyle,
    /// The arrow of an item that opens a submenu.
    pub submenu_arrow: IconStyle,
    /// The line between groups of items.
    pub separator: PartStyle,
}

impl MenuWidget {
    /// A menu of these rule sets and metrics.
    #[allow(
        clippy::too_many_arguments,
        reason = "one rule set per part of spec §5's table, and its metric"
    )]
    pub(crate) fn new(
        origin: WidgetOrigin,
        panel: PartRules,
        item: PartRules,
        item_label: TextRules,
        shortcut: TextRules,
        icon: IconRules,
        submenu_arrow: IconRules,
        separator: PartRules,
        icon_text_gap: f32,
    ) -> Self {
        Self {
            origin,
            icon_text_gap,
            panel,
            item,
            item_label,
            shortcut,
            icon,
            submenu_arrow,
            separator,
        }
    }

    /// A menu that states nothing: every part's base style empty, no rule,
    /// the metric its type's default.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_part(),
            unstated_text(),
            unstated_text(),
            unstated_icon(),
            unstated_icon(),
            unstated_part(),
            f32::default(),
        )
    }

    /// Whether the menu is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The popup box's style for `query`.
    #[must_use]
    pub fn panel(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.panel.eval(&query.into())
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

    /// The shortcut's text style for `query`.
    #[must_use]
    pub fn shortcut(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.shortcut.eval(&query.into())
    }

    /// An item icon's style for `query`.
    #[must_use]
    pub fn icon(&self, query: impl Into<StyleQuery>) -> IconStyle {
        self.icon.eval(&query.into())
    }

    /// The submenu arrow's style for `query`.
    #[must_use]
    pub fn submenu_arrow(&self, query: impl Into<StyleQuery>) -> IconStyle {
        self.submenu_arrow.eval(&query.into())
    }

    /// The separator's style for `query`: its colour is the `fill`.
    #[must_use]
    pub fn separator(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.separator.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> MenuParts {
        let query = query.into();
        MenuParts {
            panel: self.panel.eval(&query),
            item: self.item.eval(&query),
            item_label: self.item_label.eval(&query),
            shortcut: self.shortcut.eval(&query),
            icon: self.icon.eval(&query),
            submenu_arrow: self.submenu_arrow.eval(&query),
            separator: self.separator.eval(&query),
        }
    }

    /// The dead rules of each rule set, named `menu.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        vec![
            ("menu.panel", self.panel.dead_rules()),
            ("menu.item", self.item.dead_rules()),
            ("menu.item_label", self.item_label.dead_rules()),
            ("menu.shortcut", self.shortcut.dead_rules()),
            ("menu.icon", self.icon.dead_rules()),
            ("menu.submenu_arrow", self.submenu_arrow.dead_rules()),
            ("menu.separator", self.separator.dead_rules()),
        ]
    }
}
