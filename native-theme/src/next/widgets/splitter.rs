//! The splitter, `splitter` (spec §5).

use super::{PartRules, WidgetOrigin, unstated_part};
use crate::next::query::StyleQuery;
use crate::next::style::PartStyle;

/// A splitter between two panes.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SplitterWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The handle's width across the split, in logical pixels.
    pub divider_width: f32,
    pub(crate) handle: PartRules,
}

/// The parts of a splitter for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SplitterParts {
    /// The handle.
    pub handle: PartStyle,
}

impl SplitterWidget {
    /// A splitter of this rule set and metric.
    pub(crate) fn new(origin: WidgetOrigin, handle: PartRules, divider_width: f32) -> Self {
        Self {
            origin,
            divider_width,
            handle,
        }
    }

    /// A splitter that states nothing: the handle's base style empty, no
    /// rule, the width its type's default.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(origin, unstated_part(), f32::default())
    }

    /// Whether the splitter is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The handle's style for `query`: its colour is the `fill`.
    #[must_use]
    pub fn handle(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.handle.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> SplitterParts {
        let query = query.into();
        SplitterParts {
            handle: self.handle.eval(&query),
        }
    }
}
