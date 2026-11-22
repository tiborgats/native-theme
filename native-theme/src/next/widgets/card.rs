//! The card (spec §5).

use super::{PartRules, WidgetOrigin, unstated_part};
use crate::next::query::StyleQuery;
use crate::next::style::PartStyle;

/// A card: a box grouping related content.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct CardWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    container: PartRules,
}

/// The parts of a card for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct CardParts {
    /// The box.
    pub container: PartStyle,
}

impl CardWidget {
    /// A card of this rule set.
    pub(crate) fn new(origin: WidgetOrigin, container: PartRules) -> Self {
        Self { origin, container }
    }

    /// A card that states nothing: the box's base style empty, no rule.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(origin, unstated_part())
    }

    /// Whether the card is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The box's style for `query`.
    #[must_use]
    pub fn container(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.container.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> CardParts {
        let query = query.into();
        CardParts {
            container: self.container.eval(&query),
        }
    }

    /// The dead rules of each rule set, named `card.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        vec![("card.container", self.container.dead_rules())]
    }
}
