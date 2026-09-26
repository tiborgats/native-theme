//! native-theme icon payloads → egui image sources.

/// Re-exported **here and not at the crate root**, because `egui::IconData`
/// (`egui/src/viewport.rs:183`, the window/taskbar icon) already occupies that name and this
/// crate re-exports `egui`.
pub use native_theme::theme::IconData;

use crate::ResolvedTheme;

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

/// The icon size for a context, in logical pixels, for
/// `Image::fit_to_exact_size(Vec2::splat(..))` (`egui/src/widgets/image.rs:177`). Do **not**
/// pre-multiply by `Context::pixels_per_point`.
#[must_use]
pub fn icon_size(theme: &ResolvedTheme, context: IconContext) -> f32 {
    let sizes = &theme.defaults.icon_sizes;
    match context {
        IconContext::Small => sizes.small,
        IconContext::Toolbar => sizes.toolbar,
        IconContext::Panel => sizes.panel,
        IconContext::Dialog => sizes.dialog,
        IconContext::Large => sizes.large,
    }
}
