//! native-theme icon payloads → egui image sources.

/// Re-exported **here and not at the crate root**, because `egui::IconData`
/// (`egui/src/viewport.rs:183`, the window/taskbar icon) already occupies that name and this
/// crate re-exports `egui`.
pub use native_theme::theme::IconData;

/// Which of native-theme's five per-context icon sizes to use
/// (`ResolvedIconSizes`, `native-theme/src/model/resolved.rs:15-26`). egui has no icon-size
/// vocabulary of its own (§9.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IconContext {
    /// Small icon size for inline use: `defaults.icon_sizes.small`.
    Small,
    /// Icon size for toolbar buttons: `defaults.icon_sizes.toolbar`.
    Toolbar,
    /// Icon size for panel headers: `defaults.icon_sizes.panel`.
    Panel,
    /// Icon size for dialog buttons: `defaults.icon_sizes.dialog`.
    Dialog,
    /// Large icon size for menus and lists: `defaults.icon_sizes.large`.
    Large,
}
