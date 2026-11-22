//! A palette: the colours a theme is recoloured with (spec §7, "Tier 1:
//! `Palette`").

use std::sync::Arc;

use super::color::Color;
use super::query::Scheme;

/// The colours a theme is recoloured with, for one scheme (spec §7).
///
/// [`Theme::with_palette`](crate::next::Theme::with_palette) locks a theme
/// to the palette's scheme; [`Theme::with_palettes`](crate::next::Theme::with_palettes)
/// and [`Theme::with_palette_for`](crate::next::Theme::with_palette_for)
/// give one palette per scheme.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Palette {
    /// The palette's name, the resolved theme's name where the theme has no
    /// explicit name.
    pub name: Arc<str>,
    /// The scheme the palette is made for.
    pub scheme: Scheme,
    /// The window's background.
    pub window_bg: Color,
    /// A content view's background.
    pub view_bg: Color,
    /// The background of cards, dialogs and popovers.
    pub surface_bg: Color,
    /// A header bar's background.
    pub header_bg: Color,
    /// A sidebar's background.
    pub sidebar_bg: Color,
    /// A tooltip's background.
    pub tooltip_bg: Color,
    /// The text's colour.
    pub foreground: Color,
    /// The colour of secondary text.
    pub foreground_muted: Color,
    /// The colour of borders.
    pub border: Color,
    /// The colour of dividers.
    pub divider: Color,
    /// The accent.
    pub accent: Color,
    /// The colour drawn on the accent.
    pub accent_fg: Color,
    /// The selection's background.
    pub selection_bg: Color,
    /// The selection's text colour.
    pub selection_fg: Color,
    /// The colour of links.
    pub link: Color,
    /// The text caret's colour.
    pub caret: Color,
    /// The error colour.
    pub error: Color,
    /// The warning colour.
    pub warning: Color,
    /// The success colour.
    pub success: Color,
    /// The information colour.
    pub info: Color,
}

impl Palette {
    /// The names [`from_name`](Self::from_name) knows: none yet, until the
    /// bundled palettes land (plan, M1, item 3).
    pub const BUNDLED_NAMES: &'static [&'static str] = &[];

    /// A palette of these fields.
    #[must_use]
    #[allow(
        clippy::too_many_arguments,
        reason = "spec §1, \"Constructors\": `new` takes every present field"
    )]
    pub fn new(
        name: impl Into<Arc<str>>,
        scheme: Scheme,
        window_bg: Color,
        view_bg: Color,
        surface_bg: Color,
        header_bg: Color,
        sidebar_bg: Color,
        tooltip_bg: Color,
        foreground: Color,
        foreground_muted: Color,
        border: Color,
        divider: Color,
        accent: Color,
        accent_fg: Color,
        selection_bg: Color,
        selection_fg: Color,
        link: Color,
        caret: Color,
        error: Color,
        warning: Color,
        success: Color,
        info: Color,
    ) -> Self {
        Self {
            name: name.into(),
            scheme,
            window_bg,
            view_bg,
            surface_bg,
            header_bg,
            sidebar_bg,
            tooltip_bg,
            foreground,
            foreground_muted,
            border,
            divider,
            accent,
            accent_fg,
            selection_bg,
            selection_fg,
            link,
            caret,
            error,
            warning,
            success,
            info,
        }
    }

    /// The bundled palette named `name`: `None` for every name until the
    /// bundled palettes land (plan, M1, item 3).
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        let _ = name;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_every_field() {
        let c = |v: u8| Color::from_srgb8(v, v, v, 255);
        let p = Palette::new(
            "P",
            Scheme::Dark,
            c(1),
            c(2),
            c(3),
            c(4),
            c(5),
            c(6),
            c(7),
            c(8),
            c(9),
            c(10),
            c(11),
            c(12),
            c(13),
            c(14),
            c(15),
            c(16),
            c(17),
            c(18),
            c(19),
            c(20),
        );
        assert_eq!(&*p.name, "P");
        assert_eq!(p.scheme, Scheme::Dark);
        let got = [
            p.window_bg,
            p.view_bg,
            p.surface_bg,
            p.header_bg,
            p.sidebar_bg,
            p.tooltip_bg,
            p.foreground,
            p.foreground_muted,
            p.border,
            p.divider,
            p.accent,
            p.accent_fg,
            p.selection_bg,
            p.selection_fg,
            p.link,
            p.caret,
            p.error,
            p.warning,
            p.success,
            p.info,
        ];
        let want: Vec<Color> = (1..=20u8).map(c).collect();
        assert_eq!(got.to_vec(), want);
    }

    #[test]
    fn no_palette_is_bundled_yet() {
        assert!(Palette::BUNDLED_NAMES.is_empty());
        assert_eq!(Palette::from_name("catppuccin-mocha"), None);
    }

    fn assert_clone_debug_send_sync<T: Clone + std::fmt::Debug + Send + Sync>() {}

    #[test]
    fn palette_is_clone_debug_send_sync() {
        assert_clone_debug_send_sync::<Palette>();
    }
}
