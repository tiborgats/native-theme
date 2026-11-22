//! A palette: the colours a theme is recoloured with (spec §7, "Tier 1:
//! `Palette`").

use std::sync::Arc;

use super::color::Color;
use super::query::Scheme;

#[cfg(feature = "palettes")]
mod bundled;

/// The colours a theme is recoloured with, for one scheme (spec §7).
///
/// [`Theme::with_palette`](crate::next::Theme::with_palette) locks a theme
/// to the palette's scheme; [`Theme::with_palettes`](crate::next::Theme::with_palettes)
/// and [`Theme::with_palette_for`](crate::next::Theme::with_palette_for)
/// give one palette per scheme.
///
/// # Bundled palettes
///
/// With the `palettes` feature (on by default), fifteen constructors return
/// the palettes of community themes, each for one scheme, and
/// [`from_name`](Self::from_name) finds one by its name in
/// [`BUNDLED_NAMES`](Self::BUNDLED_NAMES). Every slot holds a colour the
/// upstream defines; where the upstream states an opacity, the colour carries
/// it as alpha (spec §7, "How a bundled palette's slots are filled"). Each
/// constructor's documentation names, per slot, the upstream's colour and the
/// rule that filled the slot:
///
/// - `U`: the upstream assigns the role (its words quoted).
/// - `G1` to `G7`: the fallbacks for a slot no upstream role and no family
///   rule covers: `view_bg` is `window_bg`'s colour (G1), `tooltip_bg` is
///   `surface_bg`'s (G2), `divider` is `border`'s (G3), `selection_fg` (G4)
///   and `caret` (G5) are `foreground`'s, `accent_fg` is the one of
///   `window_bg` and `foreground` with the higher
///   [`contrast_ratio`](Color::contrast_ratio) on `accent` (G6), and `link`
///   and `info` are `accent`'s (G7).
/// - Another letter and a number: the rule of the palette's family, which
///   picks one of the colours the upstream names for the role (`C`
///   Catppuccin, `D` Dracula and Alucard, `N` Nord, `S` Solarized, `T` Tokyo
///   Night, `B` Gruvbox, `O` One), followed by its reason.
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
    /// The names [`from_name`](Self::from_name) knows, in the order of spec
    /// §7's list; empty without the `palettes` feature.
    #[cfg(feature = "palettes")]
    pub const BUNDLED_NAMES: &'static [&'static str] = &[
        "catppuccin-latte",
        "catppuccin-frappe",
        "catppuccin-macchiato",
        "catppuccin-mocha",
        "dracula",
        "alucard",
        "nord",
        "solarized-light",
        "solarized-dark",
        "tokyo-night",
        "tokyo-night-day",
        "gruvbox-light",
        "gruvbox-dark",
        "one-dark",
        "one-light",
    ];

    /// The names [`from_name`](Self::from_name) knows, in the order of spec
    /// §7's list; empty without the `palettes` feature.
    #[cfg(not(feature = "palettes"))]
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

    /// The bundled palette named `name`, one of
    /// [`BUNDLED_NAMES`](Self::BUNDLED_NAMES); `None` for any other name, and
    /// for every name without the `palettes` feature.
    #[must_use]
    #[cfg(feature = "palettes")]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "catppuccin-latte" => Some(Self::catppuccin_latte()),
            "catppuccin-frappe" => Some(Self::catppuccin_frappe()),
            "catppuccin-macchiato" => Some(Self::catppuccin_macchiato()),
            "catppuccin-mocha" => Some(Self::catppuccin_mocha()),
            "dracula" => Some(Self::dracula()),
            "alucard" => Some(Self::alucard()),
            "nord" => Some(Self::nord()),
            "solarized-light" => Some(Self::solarized_light()),
            "solarized-dark" => Some(Self::solarized_dark()),
            "tokyo-night" => Some(Self::tokyo_night()),
            "tokyo-night-day" => Some(Self::tokyo_night_day()),
            "gruvbox-light" => Some(Self::gruvbox_light()),
            "gruvbox-dark" => Some(Self::gruvbox_dark()),
            "one-dark" => Some(Self::one_dark()),
            "one-light" => Some(Self::one_light()),
            _ => None,
        }
    }

    /// The bundled palette named `name`, one of
    /// [`BUNDLED_NAMES`](Self::BUNDLED_NAMES); `None` for any other name, and
    /// for every name without the `palettes` feature.
    #[must_use]
    #[cfg(not(feature = "palettes"))]
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
    fn an_unknown_name_is_none() {
        assert_eq!(Palette::from_name("nope"), None);
    }

    #[cfg(not(feature = "palettes"))]
    #[test]
    fn without_the_feature_no_palette_is_bundled() {
        assert!(Palette::BUNDLED_NAMES.is_empty());
        assert_eq!(Palette::from_name("catppuccin-mocha"), None);
    }

    /// A bundled palette's name, constructor and scheme.
    #[cfg(feature = "palettes")]
    type SpecEntry = (&'static str, fn() -> Palette, Scheme);

    /// The bundled palettes, in the order of spec §7's list.
    #[cfg(feature = "palettes")]
    const SPEC_LIST: [SpecEntry; 15] = [
        ("catppuccin-latte", Palette::catppuccin_latte, Scheme::Light),
        (
            "catppuccin-frappe",
            Palette::catppuccin_frappe,
            Scheme::Dark,
        ),
        (
            "catppuccin-macchiato",
            Palette::catppuccin_macchiato,
            Scheme::Dark,
        ),
        ("catppuccin-mocha", Palette::catppuccin_mocha, Scheme::Dark),
        ("dracula", Palette::dracula, Scheme::Dark),
        ("alucard", Palette::alucard, Scheme::Light),
        ("nord", Palette::nord, Scheme::Dark),
        ("solarized-light", Palette::solarized_light, Scheme::Light),
        ("solarized-dark", Palette::solarized_dark, Scheme::Dark),
        ("tokyo-night", Palette::tokyo_night, Scheme::Dark),
        ("tokyo-night-day", Palette::tokyo_night_day, Scheme::Light),
        ("gruvbox-light", Palette::gruvbox_light, Scheme::Light),
        ("gruvbox-dark", Palette::gruvbox_dark, Scheme::Dark),
        ("one-dark", Palette::one_dark, Scheme::Dark),
        ("one-light", Palette::one_light, Scheme::Light),
    ];

    /// The lowest WCAG contrast ratio of `foreground` on a palette's
    /// backgrounds: WCAG 2.2's minimum for large text and user-interface
    /// components, SC 1.4.3 and 1.4.11 (spec §11, item 5).
    #[cfg(feature = "palettes")]
    const MIN_TEXT_CONTRAST: f32 = 3.0;

    /// The bundled palette `name`, or an error naming it.
    #[cfg(feature = "palettes")]
    fn named(name: &str) -> Result<Palette, String> {
        Palette::from_name(name).ok_or_else(|| format!("from_name({name:?}) is None"))
    }

    #[cfg(feature = "palettes")]
    #[test]
    fn every_name_round_trips_through_from_name() -> Result<(), String> {
        for name in Palette::BUNDLED_NAMES {
            assert_eq!(&*named(name)?.name, *name);
        }
        Ok(())
    }

    #[cfg(feature = "palettes")]
    #[test]
    fn the_names_constructors_and_schemes_are_the_specs() -> Result<(), String> {
        let names: Vec<&str> = SPEC_LIST.iter().map(|(n, _, _)| *n).collect();
        assert_eq!(Palette::BUNDLED_NAMES, names.as_slice());
        for (name, constructor, scheme) in SPEC_LIST {
            let p = named(name)?;
            assert_eq!(p, constructor(), "{name}");
            assert_eq!(p.scheme, scheme, "{name}");
        }
        Ok(())
    }

    #[cfg(feature = "palettes")]
    #[test]
    fn no_two_palettes_share_a_name() {
        let names: std::collections::BTreeSet<&str> =
            Palette::BUNDLED_NAMES.iter().copied().collect();
        assert_eq!(names.len(), Palette::BUNDLED_NAMES.len());
        assert_eq!(names.len(), SPEC_LIST.len());
    }

    #[cfg(feature = "palettes")]
    #[test]
    fn foreground_is_legible_on_every_background() -> Result<(), String> {
        for name in Palette::BUNDLED_NAMES {
            let p = named(name)?;
            let backgrounds = [
                ("window_bg", p.window_bg),
                ("view_bg", p.view_bg),
                ("surface_bg", p.surface_bg),
                ("header_bg", p.header_bg),
                ("sidebar_bg", p.sidebar_bg),
                ("tooltip_bg", p.tooltip_bg),
            ];
            for (slot, bg) in backgrounds {
                let ratio = p.foreground.contrast_ratio(bg);
                assert!(
                    ratio >= MIN_TEXT_CONTRAST,
                    "{name}: foreground on {slot} is {ratio:.2}:1"
                );
            }
        }
        Ok(())
    }

    fn assert_clone_debug_send_sync<T: Clone + std::fmt::Debug + Send + Sync>() {}

    #[test]
    fn palette_is_clone_debug_send_sync() {
        assert_clone_debug_send_sync::<Palette>();
    }
}
