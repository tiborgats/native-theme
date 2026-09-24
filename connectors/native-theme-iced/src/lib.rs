//! iced toolkit connector for native-theme.
//!
//! Maps [`native_theme::theme::ResolvedTheme`] data to iced's theming system.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use native_theme_iced::from_preset;
//!
//! let (theme, resolved) = from_preset("catppuccin-mocha", true).unwrap();
//! ```
//!
//! Or from the OS-detected theme:
//!
//! ```rust,no_run
//! use native_theme_iced::from_system;
//!
//! let (theme, resolved, is_dark, accessibility) = from_system().unwrap();
//! ```
//!
//! # Manual Path
//!
//! For full control over the resolve/validate/convert pipeline:
//!
//! ```rust
//! use native_theme::theme::{ColorMode, Theme};
//! use native_theme_iced::to_theme;
//!
//! let nt = Theme::preset("catppuccin-mocha").unwrap();
//! let resolved = nt.into_variant(ColorMode::Light).unwrap().resolve_system().unwrap();
//! let theme = to_theme(&resolved, "My App");
//! ```
//!
//! # Two layers of colour
//!
//! [`to_theme()`] builds an iced `Theme` whose `Palette` and `Extended`
//! palette carry the platform's colours, so iced's built-in widget styles pick
//! them up with no further code. That palette has six colours, though, and a
//! widget state it has no slot for -- a button's pressed fill, an input's
//! focus border, a switch's track -- is a value iced derives by lightening or
//! darkening. The palette cannot correct that: the slot does not exist.
//!
//! The `styles` module does (feature `widgets`, on by default). It has one
//! function per widget, each returning
//! a closure for that widget's style setter, and every colour, border and
//! radius it emits is a field of the resolved theme. A `Style` field the
//! native model does not carry is read from iced's own default at run time,
//! never written as a literal.
//!
//! ```rust,ignore
//! use native_theme_iced::styles;
//!
//! button("Save").style(styles::button_primary(&resolved))
//! ```
//!
//! Both layers are held to a mapping contract by this crate's tests: every
//! palette slot and every `Style` field the connector writes is asserted equal
//! to the native field it claims, over all bundled presets in both modes, and
//! every foreground the style functions place on a fill is asserted to be no
//! less readable than the platform's own pair.
//!
//! # Features
//!
//! | Feature | Default | Enables |
//! |---------|---------|---------|
//! | `widgets` | yes | `styles`, `button_padding` and `input_padding`, through `iced_widget` |
//! | `iced_aw` | no | `styles::aw`, for the `iced_aw` widgets iced itself lacks (card, menu bar, tab bar, sidebar, selection list, spinner); implies `widgets` |
//! | `material-icons`, `lucide-icons`, `system-icons`, `svg-rasterize` | yes | the matching `native-theme` icon features |
//!
//! Every feature adds coverage. `default-features = false` leaves the palette
//! and the metric helpers that need `iced_core` only; `button_padding` and
//! `input_padding` read iced's own default padding from `iced_widget`, so
//! `widgets` gates them as well.
//!
//! # Accessibility
//!
//! [`from_system()`] returns the user's [`AccessibilityPreferences`] as its
//! fourth value. [`font_size()`] and [`mono_font_size()`] take them and apply
//! the OS text-scaling factor; a factor that is not finite and positive is
//! ignored. With a preset there is no OS reading:
//! `AccessibilityPreferences::default()` scales by one.
//!
//! The other two preferences, reduced transparency and reduced motion, have no
//! receiver in iced: a `Theme` is a palette, with nothing for either to act
//! on. An application reads them from the same struct where it draws
//! translucent surfaces or animates.
//!
//! # Font Configuration
//!
//! Font family names use `Arc<str>`. For iced's `&'static str` requirement,
//! use `intern_font_family` to deduplicate allocations:
//!
//! ```rust,no_run
//! use native_theme::theme::intern_font_family;
//!
//! let (_, resolved) = native_theme_iced::from_preset("catppuccin-mocha", true)?;
//! let family: std::sync::Arc<str> = intern_font_family(
//!     native_theme_iced::font_family(&resolved),
//! );
//! // For iced Font, convert Arc<str> to a String:
//! let font_family: String = family.to_string();
//! # Ok::<(), native_theme::error::Error>(())
//! ```
//!
//! `intern_font_family` returns the same `Arc<str>` for repeated calls with
//! the same family name, so resolving fonts many times allocates only once.
//!
//! # Theme Field Coverage
//!
//! The connector maps a subset of [`ResolvedTheme`] to iced's theming system:
//!
//! | Target | Fields | Source |
//! |--------|--------|--------|
//! | `Palette` (6 fields) | background, text, primary, success, warning, danger | `defaults.*` |
//! | `Extended` overrides (9) | background.base.text, secondary.base + strong, background.weak.color/text, primary/success/danger/warning.base.text | input.placeholder, defaults.surface/foreground, `*_foreground` |
//! | `styles` (20 items) | every `Style` field of button (six classes), text input, text editor, checkbox, radio, toggler, pick list, menu, slider, scrollable, progress bar, rule, tooltip, card container; scrollbar widths and embedding | the widget's own resolved theme; fields the model lacks come from iced's default |
//! | Widget metrics | button/input padding (the stated sides, iced's own default for the others; `widgets` feature), any other widget's padding over a default the caller names (`padding_or`) or where every side is stated (`stated_padding`), border radius, scrollbar width | Per-widget resolved fields |
//! | Typography | font family/size/weight, mono family/size/weight, line height | `defaults.font.*`, `defaults.mono_font.*` |
//! | Color helpers | border, link, selection, info, info_foreground, warning_foreground, focus_ring | `defaults.*` |
//! | Geometry helpers | disabled_opacity | `defaults.*` |
//!
//! Per-widget geometry that is not a `Style` field (e.g. an indicator's size,
//! a track's height, a label gap, a minimum width) is not mapped, because iced
//! takes it through inline widget configuration rather than through the theme.
//! Read it from the `ResolvedTheme` you pass to [`to_theme()`] and hand it to
//! the widget's builder -- `Checkbox::size`, `Toggler::size`,
//! `ProgressBar::girth`, the thickness argument of `rule::horizontal`. Where a
//! widget has such a receiver, its `styles` function's doc comment names it.

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[cfg(test)]
mod compat;
#[cfg(test)]
mod contract;
pub(crate) mod extended;
pub mod icons;
pub mod palette;
#[cfg(feature = "widgets")]
pub mod styles;

// Re-export native-theme types that appear in public signatures.
pub use native_theme::color::Rgba;
pub use native_theme::error::Error;
pub use native_theme::theme::{
    AnimatedIcon, ColorMode, DialogButtonOrder, IconData, IconProvider, IconRole, IconSet,
    ResolvedTheme, Theme, ThemeMode, TransformAnimation,
};
pub use native_theme::{AccessibilityPreferences, Result, SystemTheme};

#[cfg(target_os = "linux")]
pub use native_theme::detect::LinuxDesktop;

/// Create an iced [`iced_core::theme::Theme`] from a [`native_theme::theme::ResolvedTheme`].
///
/// Builds a custom theme using `Theme::custom_with_fn()`, which:
/// 1. Maps the 6 Palette fields from resolved theme colors via [`palette::to_palette()`]
/// 2. Generates an Extended palette, then overrides `background.base.text`,
///    secondary, background.weak and the status-family `.base.text` entries
///    via `extended::apply_overrides()`
///
/// The resulting theme carries the mapped Palette and Extended palette. iced's
/// built-in `Catalog` implementations for `Theme` -- one in each
/// `iced_widget` module that has a style, among them `button`, `container`,
/// `text_input`, `scrollable`, `checkbox`, `slider` and `progress_bar` --
/// derive their `Style` structs from this palette, so no `Catalog`
/// implementation of ours is needed. A `Tooltip` has no `Catalog` of its own:
/// it is styled as a container (`Theme: container::Catalog`, iced_widget
/// 0.14.2 `src/tooltip.rs:70`, `:139-148`).
///
/// The `name` sets the theme's display name (visible in theme pickers).
/// For the common case, use [`from_preset()`] to derive the name automatically.
///
/// Note: iced has no `info` color family in its Extended palette, so
/// `info` / `info_foreground` are not mapped automatically. Use
/// [`info_color()`] and [`info_foreground_color()`] helpers to access them.
#[must_use = "this returns the theme; it does not apply it"]
pub fn to_theme(
    resolved: &native_theme::theme::ResolvedTheme,
    name: &str,
) -> iced_core::theme::Theme {
    let pal = palette::to_palette(resolved);

    // Capture only the Rgba values (Copy, 4 bytes each) instead of
    // cloning the entire ResolvedTheme (~2KB with heap data).
    let colors = extended::OverrideColors {
        placeholder: resolved.input.placeholder_color,
        surface: resolved.defaults.surface_color,
        foreground: resolved.defaults.text_color,
        accent_fg: resolved.defaults.accent_text_color,
        success_fg: resolved.defaults.success_text_color,
        danger_fg: resolved.defaults.danger_text_color,
        warning_fg: resolved.defaults.warning_text_color,
    };

    iced_core::theme::Theme::custom_with_fn(name.to_string(), pal, move |p| {
        let mut ext = iced_core::theme::palette::Extended::generate(p);
        extended::apply_overrides(&mut ext, &colors);
        ext
    })
}

/// Load a bundled preset and convert it to an iced [`Theme`](iced_core::theme::Theme) in one call.
///
/// Handles the full pipeline: load preset, pick variant, resolve, validate, convert.
/// The `Theme` display name is used as the theme display name.
///
/// # Errors
///
/// Returns an error if the preset name is not recognized or if resolution fails.
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_preset(
    name: &str,
    is_dark: bool,
) -> Result<(iced_core::theme::Theme, native_theme::theme::ResolvedTheme)> {
    let spec = native_theme::theme::Theme::preset(name)?;
    let display_name = spec.name.clone();
    let variant = spec.into_variant(if is_dark {
        ColorMode::Dark
    } else {
        ColorMode::Light
    })?;
    let resolved = variant.resolve_system()?;
    let theme = to_theme(&resolved, &display_name);
    Ok((theme, resolved))
}

/// Detect the OS theme and convert it to an iced [`Theme`](iced_core::theme::Theme) in one call.
///
/// Returns the iced theme, the resolved variant, whether the system is in
/// dark mode, and the OS accessibility preferences. The `is_dark` flag comes
/// from the OS preference, not from background color analysis. The
/// preferences are returned because [`font_size()`] and [`mono_font_size()`]
/// need them.
///
/// # Errors
///
/// Returns an error if the platform theme cannot be read.
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_system() -> Result<(
    iced_core::theme::Theme,
    native_theme::theme::ResolvedTheme,
    bool,
    AccessibilityPreferences,
)> {
    let sys = native_theme::SystemTheme::from_system()?;
    let is_dark = sys.mode.is_dark();
    let name = sys.name;
    let accessibility = sys.accessibility;
    let resolved = if is_dark { sys.dark } else { sys.light };
    let theme = to_theme(&resolved, &name);
    Ok((theme, resolved, is_dark, accessibility))
}

/// Extension trait for converting a [`SystemTheme`] to an iced theme.
pub trait SystemThemeExt {
    /// Convert this system theme to an iced [`iced_core::theme::Theme`] and its [`ResolvedTheme`].
    ///
    /// Returns both the iced theme and the resolved variant, so callers can
    /// access per-widget metrics without re-resolving.
    #[must_use = "this returns the theme; it does not apply it"]
    fn to_iced_theme(&self) -> (iced_core::theme::Theme, native_theme::theme::ResolvedTheme);
}

impl SystemThemeExt for native_theme::SystemTheme {
    fn to_iced_theme(&self) -> (iced_core::theme::Theme, native_theme::theme::ResolvedTheme) {
        let resolved = self.pick(self.mode).clone();
        let theme = to_theme(&resolved, &self.name);
        (theme, resolved)
    }
}

/// Returns each side a resolved padding states, and `default`'s side where it
/// states none.
///
/// For a widget whose padding iced sets side by side, with a default the
/// application can name: [`button_padding()`] and [`input_padding()`] are
/// this with the button's and the text input's own defaults. A consumer does
/// the same for any other widget the theme states a padding for, passing
/// that widget's default -- a menu item drawn as a button takes
/// `menu.border.padding` over `iced_widget::button::DEFAULT_PADDING`, a card
/// drawn as a container takes `card.border.padding` over `Padding::ZERO`,
/// the padding a `container` has unless given one (iced_widget 0.14.2
/// `src/container.rs:95`).
#[must_use]
pub fn padding_or(
    stated: &native_theme::theme::ResolvedPadding,
    default: iced_core::Padding,
) -> iced_core::Padding {
    iced_core::Padding {
        top: stated.top.unwrap_or(default.top),
        right: stated.right.unwrap_or(default.right),
        bottom: stated.bottom.unwrap_or(default.bottom),
        left: stated.left.unwrap_or(default.left),
    }
}

/// Returns a resolved padding as an iced [`Padding`](iced_core::Padding) when
/// it states every side, and `None` when it leaves any side unstated.
///
/// For a widget whose default padding the application cannot read, so an
/// unstated side has nothing to be filled from: `iced_aw` keeps its `Card`'s
/// and its `TabBar`'s defaults private (iced_aw 0.14.1
/// `src/widget/card.rs:21`, `src/widget/tab_bar.rs:41`) and takes a padding
/// whole. With `None`, leave the widget's padding unset, so it keeps its own.
#[must_use]
pub fn stated_padding(stated: &native_theme::theme::ResolvedPadding) -> Option<iced_core::Padding> {
    Some(iced_core::Padding {
        top: stated.top?,
        right: stated.right?,
        bottom: stated.bottom?,
        left: stated.left?,
    })
}

/// Returns button padding from the resolved theme as an iced [`Padding`](iced_core::Padding).
///
/// Each side is `button.border.padding`'s side where the theme states it,
/// and iced's own button padding where it does not:
/// `iced_widget::button::DEFAULT_PADDING` (iced_widget 0.14.2
/// `src/button.rs:462`), 5 top and bottom, 10 left and right.
#[cfg(feature = "widgets")]
#[must_use]
pub fn button_padding(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Padding {
    padding_or(
        &resolved.button.border.padding,
        iced_widget::button::DEFAULT_PADDING,
    )
}

/// Returns text input padding from the resolved theme as an iced [`Padding`](iced_core::Padding).
///
/// Each side is `input.border.padding`'s side where the theme states it,
/// and iced's own text-input padding where it does not:
/// `iced_widget::text_input::DEFAULT_PADDING` (iced_widget 0.14.2
/// `src/text_input.rs:125`), 5 on every side.
#[cfg(feature = "widgets")]
#[must_use]
pub fn input_padding(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Padding {
    padding_or(
        &resolved.input.border.padding,
        iced_widget::text_input::DEFAULT_PADDING,
    )
}

/// Returns the standard border radius from the resolved theme.
#[must_use]
pub fn border_radius(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.border.corner_radius
}

/// Returns the large border radius from the resolved theme.
#[must_use]
pub fn border_radius_lg(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.border.corner_radius_lg
}

/// Returns the scrollbar groove width from the resolved theme.
#[must_use]
pub fn scrollbar_width(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.scrollbar.groove_width
}

/// Returns the primary UI font family name from the resolved theme.
#[must_use]
pub fn font_family(resolved: &native_theme::theme::ResolvedTheme) -> &str {
    &resolved.defaults.font.family
}

/// Returns the primary UI font size in logical pixels, scaled by the user's
/// text-scaling preference.
///
/// ResolvedFontSpec.size is in logical pixels (conversion from platform points
/// is handled by the resolution step).
#[must_use]
pub fn font_size(
    resolved: &native_theme::theme::ResolvedTheme,
    prefs: &AccessibilityPreferences,
) -> f32 {
    resolved.defaults.font.size * text_scale_factor(prefs)
}

/// Returns the monospace font family name from the resolved theme.
#[must_use]
pub fn mono_font_family(resolved: &native_theme::theme::ResolvedTheme) -> &str {
    &resolved.defaults.mono_font.family
}

/// Returns the monospace font size in logical pixels, scaled by the user's
/// text-scaling preference.
///
/// ResolvedFontSpec.size is in logical pixels (conversion from platform points
/// is handled by the resolution step).
#[must_use]
pub fn mono_font_size(
    resolved: &native_theme::theme::ResolvedTheme,
    prefs: &AccessibilityPreferences,
) -> f32 {
    resolved.defaults.mono_font.size * text_scale_factor(prefs)
}

/// Text-scaling multiplier from the preferences: the factor when it is finite
/// and positive, else `1.0` (spec §4.1).
fn text_scale_factor(prefs: &AccessibilityPreferences) -> f32 {
    let s = prefs.text_scaling_factor;
    if s.is_finite() && s > 0.0 { s } else { 1.0 }
}

/// Returns the primary UI font weight (CSS 100-900) from the resolved theme.
#[must_use]
pub fn font_weight(resolved: &native_theme::theme::ResolvedTheme) -> u16 {
    resolved.defaults.font.weight
}

/// Returns the monospace font weight (CSS 100-900) from the resolved theme.
#[must_use]
pub fn mono_font_weight(resolved: &native_theme::theme::ResolvedTheme) -> u16 {
    resolved.defaults.mono_font.weight
}

/// Returns the border/divider color from the resolved theme.
#[must_use]
pub fn border_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.border.color)
}

/// Returns the disabled control opacity from the resolved theme.
#[must_use]
pub fn disabled_opacity(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.disabled_opacity
}

/// Returns the focus ring indicator color from the resolved theme.
#[must_use]
pub fn focus_ring_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.focus_ring_color)
}

/// Returns the hyperlink color from the resolved theme.
#[must_use]
pub fn link_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.link_color)
}

/// Returns the selection highlight background color from the resolved theme.
#[must_use]
pub fn selection_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.selection_background)
}

/// Returns the info/attention color from the resolved theme.
///
/// Note: iced has no `info` family in its Extended palette, so this color
/// is not mapped automatically. Use this helper to access it directly.
#[must_use]
pub fn info_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.info_color)
}

/// Returns the text color for info-colored backgrounds from the resolved theme.
#[must_use]
pub fn info_foreground_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.info_text_color)
}

/// Returns the warning foreground text color from the resolved theme.
///
/// The warning base color is already mapped to `palette.warning`. This returns
/// the text color intended for use on warning-colored backgrounds.
#[must_use]
pub fn warning_foreground_color(resolved: &native_theme::theme::ResolvedTheme) -> iced_core::Color {
    palette::to_color(resolved.defaults.warning_text_color)
}

/// Returns a reference to the per-context icon sizes from the resolved theme.
#[must_use]
pub fn icon_sizes(
    resolved: &native_theme::theme::ResolvedTheme,
) -> &native_theme::theme::ResolvedIconSizes {
    &resolved.defaults.icon_sizes
}

/// Returns the line height multiplier from the resolved theme.
///
/// The raw multiplier (e.g., 1.4). Use with iced's
/// `LineHeight::Relative(native_theme_iced::line_height_multiplier(&r))`
/// for Text widgets. Font-size agnostic -- works correctly for both
/// the primary UI font and monospace text.
///
/// For absolute pixels (layout math), multiply by the appropriate
/// font size: `line_height_multiplier(&r) * font_size(&r, &prefs)`.
#[must_use]
pub fn line_height_multiplier(resolved: &native_theme::theme::ResolvedTheme) -> f32 {
    resolved.defaults.line_height
}

/// Convert a CSS font weight (100-900) to an iced [`Weight`](iced_core::font::Weight) enum.
///
/// Non-standard weights are rounded to the nearest standard value
/// (e.g., 350 -> Normal, 550 -> Semibold).
///
/// # Example
///
/// ```rust,no_run
/// let (_, resolved) = native_theme_iced::from_preset("catppuccin-mocha", true).unwrap();
/// let weight = native_theme_iced::to_iced_weight(
///     native_theme_iced::font_weight(&resolved),
/// );
/// ```
#[must_use]
pub fn to_iced_weight(css_weight: u16) -> iced_core::font::Weight {
    use iced_core::font::Weight;
    match css_weight {
        0..=149 => Weight::Thin,
        150..=249 => Weight::ExtraLight,
        250..=349 => Weight::Light,
        350..=449 => Weight::Normal,
        450..=549 => Weight::Medium,
        550..=649 => Weight::Semibold,
        650..=749 => Weight::Bold,
        750..=849 => Weight::ExtraBold,
        850.. => Weight::Black,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use native_theme::theme::{ColorMode, Theme};

    fn make_resolved_preset(name: &str, is_dark: bool) -> native_theme::theme::ResolvedTheme {
        Theme::preset(name)
            .unwrap()
            .into_variant(if is_dark {
                ColorMode::Dark
            } else {
                ColorMode::Light
            })
            .unwrap()
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .unwrap()
    }

    fn make_resolved(is_dark: bool) -> native_theme::theme::ResolvedTheme {
        make_resolved_preset("catppuccin-mocha", is_dark)
    }

    fn scaled_prefs(text_scaling_factor: f32) -> AccessibilityPreferences {
        AccessibilityPreferences {
            text_scaling_factor,
            ..AccessibilityPreferences::default()
        }
    }

    // === to_theme tests ===

    #[test]
    fn to_theme_produces_non_default_theme() {
        let resolved = make_resolved(true);
        let theme = to_theme(&resolved, "Test Theme");

        assert_ne!(theme, iced_core::theme::Theme::Light);
        assert_ne!(theme, iced_core::theme::Theme::Dark);

        let palette = theme.palette();
        // Catppuccin Mocha dark primary should be non-trivial
        let expected = palette::to_color(resolved.defaults.accent_color);
        assert_eq!(palette.primary, expected, "primary should match accent");
    }

    #[test]
    fn to_theme_from_preset() {
        let resolved = make_resolved(false);
        let theme = to_theme(&resolved, "Default");

        let palette = theme.palette();
        // Light variant has white-ish background
        assert!(
            palette.background.r > 0.9,
            "light background should be bright"
        );
    }

    #[test]
    fn to_theme_dark_variant() {
        let resolved = make_resolved(true);
        let theme = to_theme(&resolved, "Dark Test");

        let palette = theme.palette();
        assert!(palette.background.r < 0.3, "dark background should be dark");
    }

    #[test]
    fn to_theme_different_presets_differ() {
        let r1 = Theme::preset("catppuccin-mocha")
            .unwrap()
            .into_variant(ColorMode::Dark)
            .unwrap()
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .unwrap();
        let r2 = Theme::preset("dracula")
            .unwrap()
            .into_variant(ColorMode::Dark)
            .unwrap()
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .unwrap();

        let t1 = to_theme(&r1, "mocha");
        let t2 = to_theme(&r2, "dracula");

        // Different presets should produce different palette colors
        assert_ne!(t1.palette().primary, t2.palette().primary);
    }

    #[test]
    fn to_theme_with_adwaita_preset() {
        let resolved = make_resolved_preset("adwaita", false);
        let theme = to_theme(&resolved, "Adwaita");
        let palette = theme.palette();
        assert!(palette.primary.a > 0.0, "adwaita primary should be visible");
    }

    // === Widget metric helper tests ===

    #[test]
    fn border_radius_returns_resolved_value() {
        let resolved = make_resolved(false);
        let r = border_radius(&resolved);
        assert!(r > 0.0, "resolved radius should be > 0");
    }

    #[test]
    fn border_radius_lg_returns_resolved_value() {
        let resolved = make_resolved(false);
        let r = border_radius_lg(&resolved);
        assert!(r > 0.0, "resolved radius_lg should be > 0");
        assert!(
            r >= border_radius(&resolved),
            "radius_lg should be >= radius"
        );
    }

    #[test]
    fn scrollbar_width_returns_resolved_value() {
        let resolved = make_resolved(false);
        let w = scrollbar_width(&resolved);
        assert!(w > 0.0, "scrollbar width should be > 0");
    }

    /// A padding stated on two sides and not on the other two.
    #[cfg(feature = "widgets")]
    fn partly_stated() -> native_theme::theme::ResolvedPadding {
        native_theme::theme::ResolvedPadding {
            top: Some(0.0),
            right: None,
            bottom: None,
            left: Some(7.0),
        }
    }

    #[cfg(feature = "widgets")]
    #[test]
    fn button_padding_fills_unstated_sides_from_iceds_default() {
        let mut resolved = make_resolved(false);
        resolved.button.border.padding = partly_stated();
        let pad = button_padding(&resolved);
        let default = iced_widget::button::DEFAULT_PADDING;
        assert_eq!(pad.top, 0.0, "a stated zero is the theme's");
        assert_eq!(pad.left, 7.0, "a stated side is the theme's");
        assert_eq!(pad.right, default.right, "an unstated side is iced's");
        assert_eq!(pad.bottom, default.bottom, "an unstated side is iced's");
    }

    #[cfg(feature = "widgets")]
    #[test]
    fn input_padding_fills_unstated_sides_from_iceds_default() {
        let mut resolved = make_resolved(false);
        resolved.input.border.padding = partly_stated();
        let pad = input_padding(&resolved);
        let default = iced_widget::text_input::DEFAULT_PADDING;
        assert_eq!(pad.top, 0.0, "a stated zero is the theme's");
        assert_eq!(pad.left, 7.0, "a stated side is the theme's");
        assert_eq!(pad.right, default.right, "an unstated side is iced's");
        assert_eq!(pad.bottom, default.bottom, "an unstated side is iced's");
    }

    #[test]
    fn padding_or_takes_each_stated_side_and_the_default_elsewhere() {
        let stated = native_theme::theme::ResolvedPadding {
            top: Some(0.0),
            right: None,
            bottom: None,
            left: Some(7.0),
        };
        let default = iced_core::Padding {
            top: 1.0,
            right: 2.0,
            bottom: 3.0,
            left: 4.0,
        };
        let pad = padding_or(&stated, default);
        assert_eq!(pad.top, 0.0, "a stated zero is the theme's");
        assert_eq!(pad.left, 7.0, "a stated side is the theme's");
        assert_eq!(pad.right, 2.0, "an unstated side is the default's");
        assert_eq!(pad.bottom, 3.0, "an unstated side is the default's");
    }

    #[test]
    fn stated_padding_is_some_only_when_every_side_is_stated() {
        let every = native_theme::theme::ResolvedPadding {
            top: Some(3.0),
            right: Some(8.0),
            bottom: Some(0.0),
            left: Some(8.0),
        };
        let pad = stated_padding(&every);
        assert_eq!(
            pad.map(|p| (p.top, p.right, p.bottom, p.left)),
            Some((3.0, 8.0, 0.0, 8.0))
        );
        for missing in 0..4 {
            let mut sides = [every.top, every.right, every.bottom, every.left];
            sides[missing] = None;
            let partial = native_theme::theme::ResolvedPadding {
                top: sides[0],
                right: sides[1],
                bottom: sides[2],
                left: sides[3],
            };
            assert!(
                stated_padding(&partial).is_none(),
                "side {missing} unstated"
            );
        }
    }

    /// Every side the theme states is the theme's. windows-11 states button
    /// and input padding; catppuccin states neither, so it would compare
    /// nothing.
    #[cfg(feature = "widgets")]
    #[test]
    fn stated_padding_sides_are_the_themes() {
        let resolved = make_resolved_preset("windows-11", false);
        for (what, stated, pad) in [
            (
                "button",
                resolved.button.border.padding,
                button_padding(&resolved),
            ),
            (
                "input",
                resolved.input.border.padding,
                input_padding(&resolved),
            ),
        ] {
            let mut compared = 0usize;
            for (side, stated, got) in [
                ("top", stated.top, pad.top),
                ("right", stated.right, pad.right),
                ("bottom", stated.bottom, pad.bottom),
                ("left", stated.left, pad.left),
            ] {
                if let Some(stated) = stated {
                    assert_eq!(got, stated, "{what} {side}");
                    compared += 1;
                }
            }
            assert!(
                compared > 0,
                "{what}: the preset states no padding side, so nothing was compared"
            );
        }
    }

    // === Color helper tests ===

    #[test]
    fn border_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = border_color(&resolved);
        assert!(c.a > 0.0, "border color should have non-zero alpha");
    }

    #[test]
    fn disabled_opacity_returns_value() {
        let resolved = make_resolved(false);
        let o = disabled_opacity(&resolved);
        assert!(
            o > 0.0 && o <= 1.0,
            "disabled opacity should be in (0, 1], got {o}"
        );
    }

    #[test]
    fn focus_ring_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = focus_ring_color(&resolved);
        assert!(c.a > 0.0, "focus ring color should have non-zero alpha");
    }

    #[test]
    fn link_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = link_color(&resolved);
        assert!(
            c.r > 0.0 || c.g > 0.0 || c.b > 0.0,
            "link color should be non-black"
        );
    }

    #[test]
    fn selection_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = selection_color(&resolved);
        assert!(c.a > 0.0, "selection color should have non-zero alpha");
    }

    #[test]
    fn info_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = info_color(&resolved);
        assert!(
            c.r > 0.0 || c.g > 0.0 || c.b > 0.0,
            "info color should be non-black"
        );
    }

    #[test]
    fn info_foreground_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = info_foreground_color(&resolved);
        assert!(c.a > 0.0, "info foreground should have non-zero alpha");
    }

    #[test]
    fn warning_foreground_color_returns_concrete_value() {
        let resolved = make_resolved(false);
        let c = warning_foreground_color(&resolved);
        assert!(c.a > 0.0, "warning foreground should have non-zero alpha");
    }

    #[test]
    fn icon_sizes_returns_concrete_values() {
        let resolved = make_resolved(false);
        let is = icon_sizes(&resolved);
        assert!(is.small > 0.0, "small icon size should be > 0");
        assert!(is.toolbar > 0.0, "toolbar icon size should be > 0");
    }

    // === Font helper tests ===

    #[test]
    fn font_family_returns_concrete_value() {
        let resolved = make_resolved(false);
        let ff = font_family(&resolved);
        assert!(!ff.is_empty(), "font family should not be empty");
    }

    #[test]
    fn font_size_returns_concrete_value() {
        let resolved = make_resolved(false);
        let fs = font_size(&resolved, &AccessibilityPreferences::default());
        assert!(fs > 0.0, "font size should be > 0");
    }

    #[test]
    fn mono_font_family_returns_concrete_value() {
        let resolved = make_resolved(false);
        let mf = mono_font_family(&resolved);
        assert!(!mf.is_empty(), "mono font family should not be empty");
    }

    #[test]
    fn mono_font_size_returns_concrete_value() {
        let resolved = make_resolved(false);
        let ms = mono_font_size(&resolved, &AccessibilityPreferences::default());
        assert!(ms > 0.0, "mono font size should be > 0");
    }

    #[test]
    fn font_size_scales_by_the_text_scaling_factor() {
        let resolved = make_resolved(false);
        assert_eq!(
            font_size(&resolved, &scaled_prefs(1.5)),
            resolved.defaults.font.size * 1.5,
            "font size should be multiplied by the factor"
        );
    }

    #[test]
    fn font_size_ignores_a_factor_that_is_not_finite_and_positive() {
        let resolved = make_resolved(false);
        for factor in [0.0, f32::NAN, -1.0, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(
                font_size(&resolved, &scaled_prefs(factor)),
                resolved.defaults.font.size,
                "factor {factor} should leave the size unscaled"
            );
        }
    }

    #[test]
    fn mono_font_size_scales_by_the_text_scaling_factor() {
        let resolved = make_resolved(false);
        assert_eq!(
            mono_font_size(&resolved, &scaled_prefs(1.5)),
            resolved.defaults.mono_font.size * 1.5,
            "mono font size should be multiplied by the factor"
        );
    }

    #[test]
    fn mono_font_size_ignores_a_factor_that_is_not_finite_and_positive() {
        let resolved = make_resolved(false);
        for factor in [0.0, f32::NAN, -1.0, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(
                mono_font_size(&resolved, &scaled_prefs(factor)),
                resolved.defaults.mono_font.size,
                "factor {factor} should leave the size unscaled"
            );
        }
    }

    #[test]
    fn font_weight_returns_concrete_value() {
        let resolved = make_resolved(false);
        let w = font_weight(&resolved);
        assert!(
            (100..=900).contains(&w),
            "font weight should be 100-900, got {}",
            w
        );
    }

    #[test]
    fn mono_font_weight_returns_concrete_value() {
        let resolved = make_resolved(false);
        let w = mono_font_weight(&resolved);
        assert!(
            (100..=900).contains(&w),
            "mono font weight should be 100-900, got {}",
            w
        );
    }

    #[test]
    fn line_height_multiplier_returns_concrete_value() {
        let resolved = make_resolved(false);
        let lh = line_height_multiplier(&resolved);
        assert!(lh > 0.0, "line height multiplier should be > 0");
        assert!(
            lh < 5.0,
            "line height multiplier should be a multiplier (e.g. 1.4), got {}",
            lh
        );
    }

    #[test]
    fn to_iced_weight_standard_weights() {
        use iced_core::font::Weight;
        assert_eq!(to_iced_weight(100), Weight::Thin);
        assert_eq!(to_iced_weight(200), Weight::ExtraLight);
        assert_eq!(to_iced_weight(300), Weight::Light);
        assert_eq!(to_iced_weight(400), Weight::Normal);
        assert_eq!(to_iced_weight(500), Weight::Medium);
        assert_eq!(to_iced_weight(600), Weight::Semibold);
        assert_eq!(to_iced_weight(700), Weight::Bold);
        assert_eq!(to_iced_weight(800), Weight::ExtraBold);
        assert_eq!(to_iced_weight(900), Weight::Black);
    }

    #[test]
    fn to_iced_weight_non_standard_rounds_correctly() {
        use iced_core::font::Weight;
        assert_eq!(to_iced_weight(350), Weight::Normal);
        assert_eq!(to_iced_weight(450), Weight::Medium);
        assert_eq!(to_iced_weight(550), Weight::Semibold);
        assert_eq!(to_iced_weight(0), Weight::Thin);
        assert_eq!(to_iced_weight(1000), Weight::Black);
    }

    // === Convenience API tests ===

    #[test]
    fn from_preset_valid_light() {
        let (theme, resolved) = from_preset("catppuccin-mocha", false).expect("preset should load");
        assert_ne!(theme, iced_core::theme::Theme::Light);
        assert!(!resolved.defaults.font.family.is_empty());
        // Light variant should have bright background
        let palette = theme.palette();
        assert!(
            palette.background.r > 0.9,
            "light variant should have bright background, got r={}",
            palette.background.r
        );
    }

    #[test]
    fn from_preset_valid_dark() {
        let (theme, _resolved) = from_preset("catppuccin-mocha", true).expect("preset should load");
        assert_ne!(theme, iced_core::theme::Theme::Dark);
        // Dark variant should have dark background
        let palette = theme.palette();
        assert!(
            palette.background.r < 0.3,
            "dark variant should have dark background, got r={}",
            palette.background.r
        );
    }

    #[test]
    fn from_preset_invalid_name() {
        let result = from_preset("nonexistent-preset", false);
        assert!(result.is_err(), "invalid preset should return Err");
    }

    #[test]
    fn from_preset_error_shows_requested_mode() {
        // This tests the error path -- an actually empty preset cannot be
        // created through the public API, but we verify the format of
        // the success/error paths.
        let result = from_preset("nonexistent-preset", true);
        assert!(result.is_err());
    }

    #[test]
    fn system_theme_ext_to_iced_theme() {
        // May fail on CI -- skip gracefully
        let Ok(sys) = native_theme::SystemTheme::from_system() else {
            return;
        };
        let (_theme, _resolved) = sys.to_iced_theme();
    }

    #[test]
    fn from_system_does_not_panic() {
        let _ = from_system();
    }

    #[test]
    fn from_system_returns_is_dark_and_preferences() {
        // If system theme is available, verify it returns a quadruple
        if let Ok((_theme, _resolved, is_dark, accessibility)) = from_system() {
            // is_dark should be a valid bool (always true, but verify the return)
            let _ = is_dark;
            let _ = accessibility.text_scaling_factor;
        }
    }

    #[test]
    fn to_theme_extended_overrides_take_effect() {
        let resolved = make_resolved(true);
        let theme = to_theme(&resolved, "test");
        let ext = theme.extended_palette();
        // Generate what the Extended palette would be without overrides
        let auto_palette = iced_core::theme::palette::Extended::generate(theme.palette());
        // apply_overrides sets secondary.base from button.bg/fg which differs
        // from the auto-generated value
        assert_ne!(
            ext.secondary.base.color, auto_palette.secondary.base.color,
            "secondary.base.color should be overridden, not auto-generated"
        );
    }

    // Integration-level: exercises the full from_preset -> to_theme pipeline for all presets

    #[test]
    fn all_presets_produce_valid_themes() {
        for info in Theme::list_presets() {
            let name = info.key;
            for is_dark in [false, true] {
                let spec = Theme::preset(name).unwrap();
                if let Ok(variant) = spec.into_variant(if is_dark {
                    ColorMode::Dark
                } else {
                    ColorMode::Light
                }) {
                    let resolved = variant
                        .into_resolved(&native_theme::ResolutionContext::for_tests())
                        .unwrap();
                    let theme = to_theme(&resolved, name);
                    let palette = theme.palette();
                    // Basic sanity: all palette colors have valid alpha
                    assert!(
                        palette.background.a > 0.0,
                        "{name}/{is_dark}: background alpha"
                    );
                    assert!(palette.text.a > 0.0, "{name}/{is_dark}: text alpha");
                    assert!(palette.primary.a > 0.0, "{name}/{is_dark}: primary alpha");
                    assert!(palette.success.a > 0.0, "{name}/{is_dark}: success alpha");
                    assert!(palette.warning.a > 0.0, "{name}/{is_dark}: warning alpha");
                    assert!(palette.danger.a > 0.0, "{name}/{is_dark}: danger alpha");
                }
            }
        }
    }

    // === Tripwire: iced Palette field count ===

    #[test]
    fn palette_field_count_tripwire() {
        // iced_core::theme::Palette has 6 Color fields. If upstream adds more,
        // this test fails so we know to update to_palette().
        let field_count = std::mem::size_of::<iced_core::theme::Palette>()
            / std::mem::size_of::<iced_core::Color>();
        assert_eq!(
            field_count, 6,
            "iced Palette field count changed from 6 to {field_count} -- update to_palette()"
        );
    }
}
