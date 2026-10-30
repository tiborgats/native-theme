//! Colour roles (S3).

/// A surface set: a group of colour roles a part draws on. A preset declares
/// the sets it has; `Window` and `Selection` every preset declares.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SurfaceSet {
    /// A window's own surface.
    Window,
    /// A view: a list, a text field, a content area.
    View,
    /// A button.
    Button,
    /// A header bar or title bar.
    Header,
    /// A sidebar.
    Sidebar,
    /// A secondary sidebar.
    SecondarySidebar,
    /// A card.
    Card,
    /// A popover or menu.
    Popover,
    /// A dialog.
    Dialog,
    /// A tooltip.
    Tooltip,
    /// Selected items.
    Selection,
    /// KDE's complementary set: a surface in the opposite scheme.
    Complementary,
    /// An on-screen display.
    Osd,
}

impl SurfaceSet {
    /// Every surface set, in declaration order.
    pub const ALL: [SurfaceSet; 13] = [
        SurfaceSet::Window,
        SurfaceSet::View,
        SurfaceSet::Button,
        SurfaceSet::Header,
        SurfaceSet::Sidebar,
        SurfaceSet::SecondarySidebar,
        SurfaceSet::Card,
        SurfaceSet::Popover,
        SurfaceSet::Dialog,
        SurfaceSet::Tooltip,
        SurfaceSet::Selection,
        SurfaceSet::Complementary,
        SurfaceSet::Osd,
    ];

    /// Every surface set's name, in [`SurfaceSet::ALL`]'s order.
    pub const NAMES: [&'static str; 13] = [
        "window",
        "view",
        "button",
        "header",
        "sidebar",
        "secondary_sidebar",
        "card",
        "popover",
        "dialog",
        "tooltip",
        "selection",
        "complementary",
        "osd",
    ];

    /// The set's name in TOML and in expressions, its variant in snake_case.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            SurfaceSet::Window => "window",
            SurfaceSet::View => "view",
            SurfaceSet::Button => "button",
            SurfaceSet::Header => "header",
            SurfaceSet::Sidebar => "sidebar",
            SurfaceSet::SecondarySidebar => "secondary_sidebar",
            SurfaceSet::Card => "card",
            SurfaceSet::Popover => "popover",
            SurfaceSet::Dialog => "dialog",
            SurfaceSet::Tooltip => "tooltip",
            SurfaceSet::Selection => "selection",
            SurfaceSet::Complementary => "complementary",
            SurfaceSet::Osd => "osd",
        }
    }

    /// The set `name` names.
    #[must_use]
    pub fn from_name(name: &str) -> Option<SurfaceSet> {
        SurfaceSet::ALL.into_iter().find(|set| set.name() == name)
    }
}

/// A surface set's colour role: a field of S3's `SurfaceRoles`, by its path
/// (`disabled.foreground`). Its variants come with the role records; until
/// then it has none.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SurfaceRole {}

/// A global colour role: a field of S3's `GlobalRoles`, by its path
/// (`status.error.fill`). Its variants come with the role records; until then
/// it has none.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GlobalRole {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_set_names_round_trip() {
        assert_eq!(SurfaceSet::ALL.map(SurfaceSet::name), SurfaceSet::NAMES);
        for set in SurfaceSet::ALL {
            assert_eq!(SurfaceSet::from_name(set.name()), Some(set));
        }
        assert_eq!(SurfaceSet::from_name("Window"), None);
        assert_eq!(SurfaceSet::from_name("global"), None);
    }
}
