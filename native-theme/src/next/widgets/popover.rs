//! The popover (spec §5).

use super::WidgetOrigin;
use crate::next::style::{PartStyle, TextStyle};

/// A popover: a panel anchored to a control, with an arrow pointing at it.
/// A menu's popup is framed by `panel` where the menu states no frame of its
/// own (spec §5, notes to the table, "the menu's popup frame").
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PopoverWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The panel.
    pub panel: PartStyle,
    /// The arrow pointing at the anchor.
    pub arrow: PartStyle,
    /// The text.
    pub text: TextStyle,
}

impl PopoverWidget {
    /// A popover of these parts.
    pub(crate) fn new(
        origin: WidgetOrigin,
        panel: PartStyle,
        arrow: PartStyle,
        text: TextStyle,
    ) -> Self {
        Self {
            origin,
            panel,
            arrow,
            text,
        }
    }

    /// A popover that states nothing: every part empty.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            PartStyle::default(),
            PartStyle::default(),
            TextStyle::default(),
        )
    }

    /// Whether the popover is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }
}
