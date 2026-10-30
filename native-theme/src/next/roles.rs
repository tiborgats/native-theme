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
