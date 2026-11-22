//! The dialog (spec §5).

use super::WidgetOrigin;
use crate::next::layout::DialogButtonOrder;
use crate::next::motion::DialogEntranceStyle;
use crate::next::style::{IconStyle, PartStyle, TextStyle};

/// A dialog window. Its size limits are `panel.min_size` and
/// `panel.max_size`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct DialogWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The space between adjacent buttons of the button row, in logical
    /// pixels.
    pub button_gap: f32,
    /// Which end of the button row the primary button takes; `None` where no
    /// source states it.
    pub button_order: Option<DialogButtonOrder>,
    /// How the dialog moves onto the screen as it opens; `None` where no
    /// source states it.
    pub entrance: Option<DialogEntranceStyle>,
    /// The panel.
    pub panel: PartStyle,
    /// The title.
    pub title: TextStyle,
    /// The body text.
    pub body: TextStyle,
    /// The icon beside the message.
    pub icon: IconStyle,
}

impl DialogWidget {
    /// A dialog of these parts and metrics.
    #[allow(
        clippy::too_many_arguments,
        reason = "one style per part of spec §5's table, and its metrics"
    )]
    pub(crate) fn new(
        origin: WidgetOrigin,
        panel: PartStyle,
        title: TextStyle,
        body: TextStyle,
        icon: IconStyle,
        button_gap: f32,
        button_order: Option<DialogButtonOrder>,
        entrance: Option<DialogEntranceStyle>,
    ) -> Self {
        Self {
            origin,
            button_gap,
            button_order,
            entrance,
            panel,
            title,
            body,
            icon,
        }
    }

    /// A dialog that states nothing: every part empty, the button gap its
    /// type's default, no button order, no entrance.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            PartStyle::default(),
            TextStyle::default(),
            TextStyle::default(),
            IconStyle::default(),
            f32::default(),
            None,
            None,
        )
    }

    /// Whether the dialog is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }
}
