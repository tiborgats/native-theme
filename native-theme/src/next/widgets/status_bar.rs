//! The status bar (spec §5).

use super::WidgetOrigin;
use crate::next::style::{PartStyle, TextStyle};

/// A window's status bar.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StatusBarWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The bar.
    pub bar: PartStyle,
    /// The text.
    pub label: TextStyle,
    /// The line between sections: its colour is the `fill`.
    pub separator: PartStyle,
}

impl StatusBarWidget {
    /// A status bar of these parts.
    pub(crate) fn new(
        origin: WidgetOrigin,
        bar: PartStyle,
        label: TextStyle,
        separator: PartStyle,
    ) -> Self {
        Self {
            origin,
            bar,
            label,
            separator,
        }
    }

    /// A status bar that states nothing: every part empty.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            PartStyle::default(),
            TextStyle::default(),
            PartStyle::default(),
        )
    }

    /// Whether the status bar is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }
}
