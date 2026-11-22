//! The progress bar, `progress_bar` (spec §5).

use super::{PartRules, WidgetOrigin, unstated_part};
use crate::next::query::StyleQuery;
use crate::next::style::PartStyle;

/// A progress bar. The track's height is exact: `track.min_size[1]` equals
/// `max_size[1]` on a horizontal bar.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ProgressBarWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The period of the indeterminate animation, in milliseconds; `None`
    /// where no source states it.
    pub indeterminate_period_ms: Option<u32>,
    pub(super) track: PartRules,
    pub(super) fill: PartRules,
}

/// The parts of a progress bar for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ProgressBarParts {
    /// The groove.
    pub track: PartStyle,
    /// The filled portion.
    pub fill: PartStyle,
}

impl ProgressBarWidget {
    /// A progress bar of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        track: PartRules,
        fill: PartRules,
        indeterminate_period_ms: Option<u32>,
    ) -> Self {
        Self {
            origin,
            indeterminate_period_ms,
            track,
            fill,
        }
    }

    /// A progress bar that states nothing: every part's base style empty, no
    /// rule, no metric.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(origin, unstated_part(), unstated_part(), None)
    }

    /// Whether the progress bar is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The groove's style for `query`.
    #[must_use]
    pub fn track(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.track.eval(&query.into())
    }

    /// The filled portion's style for `query`.
    #[must_use]
    pub fn fill(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.fill.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> ProgressBarParts {
        let query = query.into();
        ProgressBarParts {
            track: self.track.eval(&query),
            fill: self.fill.eval(&query),
        }
    }
}
