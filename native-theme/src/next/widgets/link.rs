//! The hyperlink, `link` (spec §5).

use super::{PartRules, TextRules, WidgetOrigin};
use crate::next::query::StyleQuery;
use crate::next::style::{PartStyle, TextStyle};

/// A hyperlink. Its underline is its label's `decoration`, per state.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct LinkWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    pub(super) container: PartRules,
    pub(super) label: TextRules,
}

/// The parts of a hyperlink for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct LinkParts {
    /// The box behind the link's text.
    pub container: PartStyle,
    /// The link's text.
    pub label: TextStyle,
}

impl LinkWidget {
    /// A hyperlink of these rule sets.
    pub(crate) fn new(origin: WidgetOrigin, container: PartRules, label: TextRules) -> Self {
        Self {
            origin,
            container,
            label,
        }
    }

    /// Whether the hyperlink is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The container's style for `query`.
    #[must_use]
    pub fn container(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.container.eval(&query.into())
    }

    /// The text's style for `query`.
    #[must_use]
    pub fn label(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.label.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> LinkParts {
        let query = query.into();
        LinkParts {
            container: self.container.eval(&query),
            label: self.label.eval(&query),
        }
    }
}
