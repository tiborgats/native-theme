//! gpui toolkit connector for native-theme.
//!
//! Maps [`ResolvedTheme`] data to gpui-component's theming system:
//!
//! - **build** a gpui-component theme: [`from_system()`] (the OS theme),
//!   [`from_preset()`] (a bundled preset) or [`to_theme()`] (a
//!   `ResolvedTheme` you already hold);
//! - **install** it: [`apply`], or [`apply_system_theme`] for both variants
//!   of the OS theme at once, and [`apply_accessibility`] when a preference
//!   changes at run time;
//! - **per-widget geometry and colours** that `ThemeColor` cannot carry: the
//!   [`geometry`] builders and the [`variants`], fed by
//!   [`cx.native()`](ActiveNativeTheme::native);
//! - **scrollbars and resize handles**, which gpui-base draws: [`base_layer`],
//!   which [`apply`] installs and keeps installed;
//! - **icons** in the theme's icon set: [`icons`];
//! - **theme-drawn controls** for the parts gpui-component draws from literals
//!   of its own: the `widgets` module (feature `widgets`, on by default).
//!
//! # Quick Start
//!
//! Initialise gpui-component, then install the OS theme, or a bundled preset
//! where the OS theme cannot be read:
//!
//! ```no_run
//! use native_theme_gpui::{
//!     AccessibilityPreferences, SystemTheme, apply, apply_system_theme, from_preset,
//! };
//!
//! gpui_kit::application().run(|cx| {
//!     // Before `apply`: an `init` afterwards switches to light and resets the scroll bar mode.
//!     gpui_kit::init(cx);
//!     match SystemTheme::from_system() {
//!         // Both variants, so `Theme::sync_system_appearance` switches between them.
//!         Ok(sys) => apply_system_theme(&sys, cx),
//!         Err(_) => {
//!             let prefs = AccessibilityPreferences::from_system();
//!             if let Ok((theme, resolved)) = from_preset("adwaita", false, &prefs) {
//!                 apply(theme, &resolved, &prefs, cx);
//!             }
//!         }
//!     }
//!     // Then open the windows with `gpui_kit::open_window`.
//! });
//! ```
//!
//! [`SystemTheme::from_system()`] reads the OS theme with the `native` feature
//! (see [Features](#features)). Nothing reaches the windows until [`apply`] or
//! [`apply_system_theme`] runs; [`from_preset()`], [`from_system()`] and
//! [`to_theme()`] only build a theme. Once one is installed,
//! [`cx.native()`](ActiveNativeTheme::native) is the input of the [`geometry`]
//! builders.
//!
//! # Manual Path
//!
//! For full control over the resolve/validate/convert pipeline:
//!
//! ```no_run
//! use native_theme_gpui::{AccessibilityPreferences, ColorMode, Theme, to_theme};
//!
//! let nt = Theme::preset("catppuccin-mocha")?;
//! let resolved = nt.resolve(ColorMode::Dark)?;
//! let prefs = AccessibilityPreferences::from_system();
//! let theme = to_theme(&resolved.variant, &nt.name, resolved.mode.is_dark(), &prefs);
//! // Then `apply(theme, &resolved.variant, &prefs, cx)`.
//! # Ok::<(), native_theme_gpui::Error>(())
//! ```
//!
//! # Single-Mode Behavior
//!
//! Each call to [`from_preset()`] or [`to_theme()`] produces a theme for exactly
//! one mode (light or dark). The `is_dark` parameter selects which variant to
//! load from the preset's TOML. Presets that only define one variant will return
//! that variant for both `is_dark=true` and `is_dark=false` (the resolution
//! pipeline falls back to the available variant), and [`from_preset()`] builds
//! the theme in that variant's mode
//! ([`Resolved::mode`](crate::native_theme::theme::Resolved::mode)). To support
//! runtime light/dark switching, call [`from_preset()`] once per mode and
//! [`apply`] both: each `apply` stores its variant, and gpui-component's
//! `Theme::change` / `sync_system_appearance` then switch between the two
//! native palettes, the observer [`apply`] installs restoring after each switch
//! what the installed `ThemeConfig` cannot carry or gpui-component caps: the 12
//! base-palette colours and the native `selection`, `list_active` and
//! `table_active` ([`apply`], step 8).
//!
//! # What each flag does
//!
//! The four [`AccessibilityPreferences`] flags reach the connector through the
//! preferences [`from_preset()`], [`to_theme()`], [`apply`] and
//! [`apply_accessibility`] take; [`from_system()`] and [`apply_system_theme`]
//! use the OS reading.
//!
//! | Flag | The connector applies | The application forwards |
//! |------|-----------------------|--------------------------|
//! | `text_scaling_factor` | `font_size` and `mono_font_size` times [`AccessibilityPreferences::text_scale_factor`], through [`AccessibilityPreferences::scaled`] (a size the factor would make infinite, subnormal or zero stays as stated); `font_size` is the window rem, so every rem-relative size in gpui-component follows; the [`geometry`] builders scale the widget fonts they set | a size it sets itself, through [`scaled_text_size()`] |
//! | `reduce_motion` | [`apply`] and [`apply_accessibility`] switch gpui's `App::reduce_motion` on, which the `widgets` spinner reads | its own animations read `cx.reduce_motion()` |
//! | `reduce_transparency` | no dialog or sheet scrim: `ThemeColor::overlay` is transparent | translucent surfaces of its own |
//! | `high_contrast` | nothing, by design: high contrast belongs in the theme data, which a reader fills with what the platform draws under it (the Windows reader writes the high-contrast system colours, the GNOME reader libadwaita's high-contrast focus ring, and the macOS reader's system colours follow Increase Contrast, `docs/evidence/os-probe/2026-10-26-run-40320895749/os-probe-macos-14/increase-contrast-on/probe.toml`), so the theme carries it already; [`is_high_contrast`] reads the flag | its own colours, if they should change |
//!
//! # Theme Field Coverage
//!
//! The connector maps [`ResolvedTheme`] onto gpui-component's `ThemeColor`
//! (all 139 colour fields), `ThemeConfig` (fonts, radii, the window's corner
//! radius, shadow, highlighter), the styled `Theme`'s `focus_ring`,
//! `scrollbar_mode` and `highlight_theme`,
//! and gpui-base's scrollbar and resize-handle styles.
//!
//! | Category | Mapped | Notes |
//! |----------|--------|-------|
//! | `defaults` colors | 14 of 24 | text_color, muted_color, accent_color, border.color, focus_ring_color, shadow_color (the dialog overlay's colour, at an alpha of its own), and the four status colours with their text colours; the window background comes from `window.background_color` |
//! | `defaults` geometry | radius, radius_lg, shadow, focus ring | fonts scaled by the text-scaling factor |
//! | `button` | all 28 `button*` plus `primary*` / `secondary*` | solid native surfaces, the semantic colours each variant read in gpui-component 0.5.1, not upstream's tinted house style; flat buttons via [`variants::ghost_button`] |
//! | `tab` | 5 of 10 colours | geometry is upstream work (`Tab`'s render writes its own height, radius and text size into the style bag the caller's setters fill, `tab/tab.rs:958-966`) |
//! | `sidebar` | 5 of 6 | background, font.color, selection_background, selection_text_color, border.color |
//! | `window` | 6 of 6 | background_color, title_bar_background, border colour and corner radius (`window_radius`, the corners of a client-decorated window); title_bar_font family, size, weight, style and colour via `geometry::title_bar`; the inactive title bar's fill and text via `geometry::title_bar_inactive` |
//! | `input` | 3 of 13 + geometry | border, caret, selection_background; height, line height, padding, radius, border, text and its colour via `geometry::input`; the fill, and a disabled field's fill and text colour, via `geometry::input_fill`; the code `Editor`'s fill, and where it is translucent its gutter's, the fill over `window.background_color`, through the highlight theme (its current line, line numbers and syntax colours stay upstream's scheme), and its line height via `geometry::editor` |
//! | `scrollbar` | colours + geometry | track/thumb colours, widths, inset, min length via `base_layer` |
//! | `status_bar` | 3 of 3 | background, border colour; the border's padding sides and the font's family, size, weight, style and colour via `geometry::status_bar` |
//! | `table` | head, from `list.header_background` / `list.header_font` | the model states no footer colour, so `table_foot*` are derived |
//! | `slider`, `switch` | 2 colours each | fill/thumb colours; a checked switch's track, `switch.checked_background`, via [`variants::switch_color`] for `Switch::color`; geometry upstream |
//! | `progress_bar` | fill + geometry | height, radius, min width via `geometry::progress` |
//! | `list` | 7 of 14 + geometry | background, alternate row, selection, hover, grid, header background and header font colour; the background and the frame via `geometry::list`, the receiver of the fill (the `list` token has no reader in gpui-component 0.8.1); row height, padding, font via `geometry::list_item`; a selected row's text colour via `geometry::list_item_selected`; header height via `geometry::list_header_height` for `DataTable::header_row_height` |
//! | `menu` | 2 colours + geometry | hover_background, hover_text_color → `accent`, `accent_foreground` (upstream's item highlight); row height, padding, icon gap, font via `geometry::menu_item` on an application-drawn row |
//! | `segmented_control` | 1 of 9 | background → `tab_bar_segmented`, the track of a segmented `TabBar` |
//! | `popover` | 2 of 3 + geometry | background, font.color; padding, radius and edge (`border.color` at `border.line_width`) via `geometry::popover` |
//! | `combo_box` | geometry | height, width, padding, radius, font via `geometry::select` and `geometry::combobox`; the fill, and a disabled trigger's fill and text colour, via `geometry::combo_box_fill` |
//! | `card`, `expander` | geometry | the card's padding, radius, edge and fill via `geometry::group_box_content`; the expander's title height and colour via `geometry::accordion_title`, its edge colour via `geometry::accordion`, its hover fill via `geometry::accordion_hover` |
//! | `tooltip` | show delay + geometry | `show_delay` installed by [`apply`] as gpui-base's `TooltipDefaults` show delay, and [`tooltip_show_delay`] for GPUI's own `.tooltip()` on an element; background, font, padding, border, radius via `geometry::tooltip`, max width via `geometry::tooltip_content` |
//! | `dialog` | entrance + geometry | `entrance` via `geometry::dialog_entrance`, both styles as `DialogEntrance::Fade` (gpui-component draws neither a scaling nor a rising entrance); padding, min width, min height, radius, fill and edge colour via `geometry::dialog`, max width via `geometry::dialog_max_width`, button gap via `geometry::dialog_footer`, fonts via `geometry::dialog_title` and `geometry::dialog_description` |
//! | `link` | 3 of 8 + geometry | font.color, hover_text_color, active_text_color; background_color via `geometry::link`, where the `Link` widget paints no background of its own (`link.rs:73-90`); `link.hover_background` has no receiver — the `Link` widget paints no background in any state, a `ButtonVariant::Link` button paints `Theme::transparent` in every state, and that field is not one of the 139 `ThemeColor` colours |
//! | `splitter` | colours | divider/hover colours via `base_layer::resizable_theme`, drawn by gpui-base's resizables at their 1 px width (not gpui-component's, `Settings` or dock edges) |
//!
//! **Accessors.** The free functions of the crate root read what an application
//! needs beside the theme; one that returns one `ResolvedTheme` leaf is named
//! after the leaf's path, without the leading `defaults.` and a unit suffix:
//! `defaults.border.corner_radius` is `border_corner_radius`,
//! `scrollbar.groove_width` is `scrollbar_groove_width`, the same name in the
//! iced, gpui and egui connectors (`scripts/check_connector_parity.py` checks
//! it).
//!
//! **Per-widget geometry.** Heights, paddings, radii, borders and text sizes
//! reach the widgets through the [`geometry`] module: pure builders returning a
//! `StyleRefinement` that the application applies with
//! `gpui_component::StyledExt::refine_style`, where gpui-component applies the
//! caller's style after its own geometry. [`apply`] installs the theme, the
//! base-layer overrides and the observer that keeps them installed.
//!
//! **Limits.** Geometry on inner elements the caller's style cannot reach
//! (checkbox and radio indicators, switch, slider, separator thickness,
//! splitter width, button icon gap, popup-menu rows), and tab
//! height, radius and text size, which `Tab`'s render writes into the same
//! style bag the caller's setters fill (`tab/tab.rs:958-966`), stay upstream
//! work; §14 of the v0.5.8 specification
//! (<https://github.com/tiborgats/native-theme/blob/main/docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md>)
//! lists each item with the upstream line that makes it unreachable. For
//! the checkbox, radio, switch, slider, progress bar, spinner, tab bar and
//! separator, the `widgets` module (feature `widgets`) draws controls of its own on
//! gpui-base's headless primitives, every part from the theme.
//!
//! # Features
//!
//! The first six are on by default; `gpui-fast` is off, because it changes the
//! GPUI core rather than adding to it, and `native` and `watch` are off, as in
//! native-theme.
//!
//! | Feature | Enables |
//! |---------|---------|
//! | `widgets` | the `widgets` module: a checkbox, radio, switch, slider, progress bar, spinner, tab bar and separator drawn from the theme, for the parts gpui-component draws from literals of its own |
//! | `material-icons` | the bundled Material Symbols set (`native-theme/material-icons`) |
//! | `lucide-icons` | the bundled Lucide set (`native-theme/lucide-icons`) |
//! | `system-icons` | the platform's own icons (`native-theme/system-icons`) |
//! | `svg-rasterize` | SVG icons rasterized by this crate (`native-theme/svg-rasterize`) |
//! | `system-fonts` | a theme family the system lacks is drawn in the family the platform substitutes for it ([`font_family`]; `native-theme/system-fonts`) |
//! | `gpui-fast` | the crate built on GPUI Kit's GPUI Fast core instead of gpui-pre (`gpui-kit/gpui-fast`, `gpui-component/gpui-fast`) |
//! | `native` | the OS readers, `native-theme/native`; without one (or native-theme's `kde`, `portal`, `macos`, `windows`), [`from_system()`] is the platform preset on Linux and an error on macOS and Windows |
//! | `watch` | `on_theme_change`, a closure run on gpui's foreground, with a window, when the OS theme changes, through `native-theme/watch` and `futures` (see [Runtime theme change](#runtime-theme-change)) |
//!
//! The crate names GPUI through gpui-kit, so it is built on whichever core
//! gpui-kit is: an application that enables `gpui-kit/gpui-fast` with
//! gpui-kit's `component` feature (on by default) gets this crate on GPUI Fast
//! without `gpui-fast` here. Enable `gpui-fast` here only for an application
//! on GPUI Fast otherwise (one that depends on gpui-component directly, or
//! uses gpui-kit without `component`): it switches gpui-kit and
//! gpui-component together, since the two cores' types are distinct. The
//! README's *Backends* section has the details.
//!
//! Without `svg-rasterize` an SVG icon is still converted: [`icons`] hands
//! gpui the (colorized) SVG bytes as an undecoded `ImageSource::Image`, which
//! paints nothing the first time it comes up and is rasterized at a size gpui
//! chooses. [`icons::to_image_source`] documents it.
//!
//! # Runtime theme change
//!
//! With the `watch` feature and a reader feature (`native`),
//! `on_theme_change(window, cx, f)` runs `f` on gpui's foreground, with the
//! window and the `App`, each time the OS theme changes, for as long as the
//! `ThemeSubscription` it returns is held and the window is open; register
//! it on the main window, since events stop once that window closes. In
//! `f`, call `native_theme_gpui::native_theme::detect::invalidate_caches()`
//! (the source crate, re-exported, so no second dependency names it), read
//! the theme again with the entry point the application used
//! ([`SystemTheme::from_system()`], or
//! [`SystemTheme::from_system_with_preset`] for a pinned preset), and
//! [`apply_system_theme`] it; gpui-component's `Theme::sync_system_appearance`
//! keeps switching between its light and dark palettes after that:
//!
//! ```rust,ignore
//! use native_theme_gpui::native_theme::detect::invalidate_caches;
//! use native_theme_gpui::{SystemTheme, apply_system_theme, on_theme_change};
//!
//! // Held while the application follows the OS theme; dropping it stops
//! // the watcher.
//! let watcher = on_theme_change(window, cx, |_event, _window, cx| {
//!     invalidate_caches();
//!     if let Ok(sys) = SystemTheme::from_system() {
//!         apply_system_theme(&sys, cx);
//!     }
//! })?;
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::disallowed_methods)]
// GPUI through gpui-kit, which re-exports the core it is built on: gpui-pre,
// or GPUI Fast under `gpui-kit/gpui-fast` (gpui-kit `src/lib.rs:94, 101-103`).
extern crate gpui_kit as gpui;

pub mod base_layer;
pub(crate) mod colors;
#[cfg(test)]
mod compat;
pub(crate) mod config;
#[cfg(test)]
mod contract;
pub(crate) mod derive;
pub mod geometry;
// The freedesktop icon-theme fixture the tests run in a child process.
#[cfg(all(test, target_os = "linux", feature = "system-icons"))]
use native_theme_test_fixtures as icon_fixture;
pub mod icons;
#[cfg(test)]
mod showcase;
pub mod variants;
#[cfg(feature = "watch")]
mod watch;
#[cfg(feature = "widgets")]
pub mod widgets;

#[cfg(feature = "watch")]
pub use watch::on_theme_change;

// The source crate, re-exported as the egui and iced connectors do, so an
// application reaches the rest of native-theme's API
// (`native_theme_gpui::native_theme::detect::invalidate_caches()`) without a
// dependency of its own: a downstream `Cargo.toml` need not add a second
// one, whose version could drift from the connector's.
pub use native_theme;

// Re-export native-theme types that appear in public signatures so downstream
// crates don't need native-theme as a direct dependency.
// Issue 8 + 48: re-export Result, Rgba, Error, DialogButtonOrder
pub use native_theme::color::Rgba;
pub use native_theme::error::Error;
pub use native_theme::icons::{AnimatedIcon, IconData, IconProvider, IconRole, TransformAnimation};
pub use native_theme::theme::{
    ColorMode, DialogButtonOrder, IconSet, ResolvedTheme, Theme, ThemeMode,
};
#[cfg(feature = "watch")]
pub use native_theme::watch::{ThemeChangeEvent, ThemeSubscription};
pub use native_theme::{AccessibilityPreferences, Result, SystemTheme};

#[cfg(target_os = "linux")]
pub use native_theme::detect::LinuxDesktop;

/// The README's code blocks, compiled as doctests so they cannot drift from the API; the
/// runtime-theme-change block needs feature `watch`, and the theme-drawn widgets block
/// `widgets`.
#[cfg(all(doctest, feature = "watch", feature = "widgets"))]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;

use gpui::{App, Global, Hsla, SharedString, px};
use gpui_component::scroll::ScrollbarMode;
use gpui_component::theme::{Theme as GpuiTheme, ThemeMode as GpuiThemeMode};
use std::borrow::Cow;
use std::rc::Rc;
use std::time::Duration;

/// Convert a [`ResolvedTheme`] into a gpui-component [`GpuiTheme`].
///
/// Builds a complete GpuiTheme by:
/// 1. Mapping all 139 ThemeColor fields via `colors::to_theme_color`
/// 2. Setting font, geometry, and mode fields directly on the Theme
/// 3. Storing a ThemeConfig in light_theme/dark_theme Rc for gpui-component switching
///
/// All Theme fields are set explicitly -- no `apply_config` call is used.
/// This avoids the fragile apply-then-restore pattern where `apply_config`
/// would overwrite all 139 color fields with defaults.
///
/// A [`ResolvedTheme`] carries no colour mode, so `is_dark` is the theme's:
/// [`Resolved::mode`](native_theme::theme::Resolved::mode) of the variant
/// [`Theme::resolve`] resolved, as [`from_preset()`] passes it, or
/// [`SystemTheme::mode`], as [`from_system()`] does. [`is_dark_resolved()`]
/// guesses it from the background's lightness for a caller holding only a
/// `ResolvedTheme`; on a borderline background (solarized, gruvbox) the
/// guess can disagree with the theme's mode.
///
/// Note: `is_dark` is an explicit parameter here; the iced connector's
/// `to_theme` takes no mode.
///
/// The families are [`font_family`]'s: on macOS the stated family of the
/// system UI font, "SF Pro", is passed to gpui as its own alias
/// `.SystemUIFont`, and that of the system monospaced font, "SF Mono", as the
/// font's family name, `.AppleSystemUIFontMonospaced`, so Core Text supplies the
/// system fonts itself; a family
/// the system lacks is the one the platform substitutes for it (feature
/// `system-fonts`); every other family is passed as stated. The platform is
/// asked on the calling thread, once per family per process: on Linux and
/// the other non-Apple, non-Android unix systems the first theme to bring a
/// family runs one `fc-match` (about 12.5 ms measured on one Linux machine,
/// with a 2 s timeout), so a theme with a new UI and a new mono family costs
/// about 25 ms once, and every later theme with those families nothing.
#[must_use = "this returns the theme; it does not apply it"]
pub fn to_theme(
    resolved: &ResolvedTheme,
    name: &str,
    is_dark: bool,
    prefs: &AccessibilityPreferences,
) -> GpuiTheme {
    let theme_color = colors::to_theme_color(resolved, is_dark, prefs.reduce_transparency);
    let mode = if is_dark {
        GpuiThemeMode::Dark
    } else {
        GpuiThemeMode::Light
    };
    let d = &resolved.defaults;

    let mut theme = GpuiTheme::from(&theme_color);
    // Issue 53: Theme.transparent is set by Theme::from() to
    // Hsla::transparent_black() and is intentionally left unchanged.
    // It's used internally by gpui-component for transparent overlays.
    theme.mode = mode;
    theme.font_family = font_family(&d.font.family);
    // §3.4: gpui-component's `WindowState` root plugin sets the window rem
    // to font_size (gpui-component src/root.rs:714), so scaling these two sizes scales every rem-relative
    // size in gpui-component, as the platform toolkit scales its own text.
    theme.font_size = px(prefs.scaled(d.font.size));
    theme.mono_font_family = font_family(&d.mono_font.family);
    theme.mono_font_size = px(prefs.scaled(d.mono_font.size));
    theme.radius = px(d.border.corner_radius);
    theme.radius_lg = px(d.border.corner_radius_lg);
    // The corner radius of a client-decorated window, which gpui-component's
    // WindowBorder, Root and TitleBar round (gpui-component
    // src/window_border.rs:28-29): the window's own, `window.border`'s.
    theme.window_radius = px(resolved.window.border.corner_radius);
    theme.shadow = d.border.shadow_enabled;

    // §8.3: the ring is drawn only when the theme gives it a width; its width
    // and offset have no receiver (derived from the element's border upstream).
    theme.focus_ring = d.focus_ring_width > 0.0;

    // §8.3: Scrolling (overlay, auto-hide) when the platform draws overlay
    // scrollbars, Always otherwise.
    theme.scrollbar_mode = if resolved.scrollbar.overlay_mode {
        ScrollbarMode::Scrolling
    } else {
        ScrollbarMode::Always
    };

    // Issue 43/44: the highlight theme follows is_dark, so syntax highlighting
    // takes the mode's colours. Its style is config::highlight_style's, the one
    // the config below carries, which fills the code editor and its gutter
    // from the platform's field fill.
    let highlight = if is_dark {
        gpui_component::highlighter::HighlightTheme::default_dark()
    } else {
        gpui_component::highlighter::HighlightTheme::default_light()
    };
    let mut highlight = (*highlight).clone();
    highlight.style = config::highlight_style(resolved, mode);
    theme.highlight_theme = std::sync::Arc::new(highlight);

    // Store config for gpui-component's theme switching
    let config: Rc<_> = Rc::new(config::to_theme_config(resolved, name, mode, prefs));
    if mode == GpuiThemeMode::Dark {
        theme.dark_theme = config;
    } else {
        theme.light_theme = config;
    }
    theme
}

/// Load a bundled preset and convert it to a gpui-component [`GpuiTheme`] in one call.
///
/// The preset half of the common task; [`apply`] installs the result. It
/// handles the full pipeline: load preset, pick variant, resolve, validate,
/// and convert to gpui Theme.
///
/// Returns both the gpui Theme and the [`ResolvedTheme`] so callers can
/// access per-widget metrics (button padding, scrollbar width, etc.) that the
/// flat `ThemeColor` cannot represent.
///
/// The gpui theme is dark when the variant resolved is the dark one
/// ([`Resolved::mode`](native_theme::theme::Resolved::mode)), so a light-only
/// preset asked for dark gives a light theme.
///
/// The theme's display name is the preset's `name` field (`"Dracula"` for
/// `"dracula"`), not the name passed in.
///
/// Pass `&AccessibilityPreferences::default()` for no scaling, or the OS
/// preferences to honour them under a preset: accessibility is orthogonal to
/// the theme choice, and a user with large text wants it under a preset too.
/// On a preset switch,
/// `native_theme_gpui::native_theme::detect::accessibility_preferences()` is
/// the cached reading: the first call after a system read may still run the
/// reduced-motion detection (about 3 ms measured on one KDE machine), and
/// every later call returns the cached block in nanoseconds;
/// [`AccessibilityPreferences::from_system()`] runs the platform reader each
/// time.
///
/// The first preset to bring a font family not yet seen in the process asks
/// the platform for its substitute on the calling thread ([`font_family`],
/// feature `system-fonts`): on Linux and the other non-Apple, non-Android
/// unix systems one `fc-match` per new family, about 12.5 ms measured on one
/// Linux machine, so a first switch to a preset with a new UI and a new mono
/// family costs about 25 ms more, once.
///
/// # Errors
///
/// Returns an error if the preset name is not recognized or if resolution fails.
///
/// # Examples
///
/// ```no_run
/// use native_theme_gpui::AccessibilityPreferences;
///
/// let prefs = AccessibilityPreferences::from_system();
/// let (dark_theme, resolved) = native_theme_gpui::from_preset("dracula", true, &prefs)?;
/// let (light_theme, _) = native_theme_gpui::from_preset("catppuccin-latte", false, &prefs)?;
/// # Ok::<(), native_theme_gpui::Error>(())
/// ```
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_preset(
    name: &str,
    is_dark: bool,
    prefs: &AccessibilityPreferences,
) -> Result<(GpuiTheme, ResolvedTheme)> {
    from_spec(&Theme::preset(name)?, is_dark, prefs)
}

/// [`from_preset()`] of a theme already loaded.
fn from_spec(
    spec: &Theme,
    is_dark: bool,
    prefs: &AccessibilityPreferences,
) -> Result<(GpuiTheme, ResolvedTheme)> {
    let resolved = spec.resolve(if is_dark {
        ColorMode::Dark
    } else {
        ColorMode::Light
    })?;
    let theme = to_theme(
        &resolved.variant,
        &spec.name,
        resolved.mode.is_dark(),
        prefs,
    );
    Ok((theme, resolved.variant))
}

/// The OS theme as [`from_system()`] and [`SystemThemeExt::to_gpui_theme`]
/// return it: the gpui-component theme of the OS colour mode's variant, with
/// what the OS reading detected beside it.
///
/// `#[non_exhaustive]`, so a field added later is not a breaking change;
/// read it by field, or destructure it with `..`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Loaded {
    /// The gpui-component theme, built by [`to_theme()`] from
    /// [`resolved`](Self::resolved) with [`accessibility`](Self::accessibility).
    pub theme: GpuiTheme,
    /// The resolved variant of the OS colour mode, for the per-widget
    /// metrics the flat `ThemeColor` cannot represent.
    pub resolved: ResolvedTheme,
    /// The OS colour mode, the OS preference rather than a guess from the
    /// background colour; [`resolved`](Self::resolved) is its variant.
    pub mode: ColorMode,
    /// The OS accessibility preferences the theme was built with, which
    /// [`apply`] takes.
    pub accessibility: AccessibilityPreferences,
    /// The icon set the theme names ([`SystemTheme::icon_set`]).
    pub icon_set: IconSet,
    /// The icon theme of the OS colour mode's variant
    /// ([`SystemTheme::icon_theme_for`]), for a freedesktop icon or spinner;
    /// `None` where that variant states none and detection failed.
    pub icon_theme: Option<Cow<'static, str>>,
}

/// Detect the OS theme and convert it to a gpui-component [`GpuiTheme`] in one call.
///
/// Combines [`SystemTheme::from_system()`](native_theme::SystemTheme::from_system)
/// with [`to_theme()`] using the system-detected name, dark-mode preference
/// and accessibility preferences.
///
/// Returns the gpui Theme with the [`ResolvedTheme`], so callers can access
/// per-widget metrics that the flat `ThemeColor` cannot represent, and the
/// OS colour mode, the accessibility preferences, the icon set and the icon
/// theme beside them, as a [`Loaded`].
///
/// **Ownership note:** this function takes ownership of the
/// `SystemTheme`'s active variant. The non-active variant (light when dark
/// is active, or vice versa) is dropped. To install both variants, so a
/// light/dark switch keeps the native palette, use
/// [`SystemTheme::from_system()`](native_theme::SystemTheme::from_system) and
/// [`apply_system_theme`]; to build both, call [`to_theme()`] on each.
///
/// **Performance note:** `SystemTheme::from_system()` resolves both light
/// and dark variants before this function picks one; native-theme has no
/// system read that resolves only one variant.
///
/// # Errors
///
/// Returns an error if the platform theme cannot be read (e.g., unsupported platform,
/// missing desktop environment). Without a reader feature (`native`, or
/// native-theme's `kde`, `portal`, `macos` or `windows`) the theme is the
/// platform preset on Linux and an error on macOS and Windows.
///
/// # Examples
///
/// ```no_run
/// let loaded = native_theme_gpui::from_system()?;
/// # Ok::<(), native_theme_gpui::Error>(())
/// ```
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_system() -> Result<Loaded> {
    let sys = SystemTheme::from_system()?;
    let mode = sys.mode;
    let is_dark = mode.is_dark();
    let icon_theme = sys.icon_theme_for(mode).map(|s| Cow::Owned(s.to_owned()));
    let name = sys.name; // K-5: move instead of clone
    let accessibility = sys.accessibility;
    let resolved = if is_dark { sys.dark } else { sys.light };
    let theme = to_theme(&resolved, &name, is_dark, &accessibility);
    Ok(Loaded {
        theme,
        resolved,
        mode,
        accessibility,
        icon_set: sys.icon_set,
        icon_theme,
    })
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for native_theme::SystemTheme {}
}

/// Extension trait for converting a [`SystemTheme`] to a gpui-component [`GpuiTheme`].
///
/// Sealed, as the egui and iced connectors' `SystemThemeExt` are:
/// [`SystemTheme`] is its one implementor, so a method added to it later is
/// not a breaking change.
///
/// Useful when you already have a `SystemTheme` and want method syntax:
///
/// ```no_run
/// use native_theme_gpui::SystemThemeExt;
///
/// let sys = native_theme_gpui::SystemTheme::from_system()?;
/// let theme = sys.to_gpui_theme().theme;
/// # Ok::<(), native_theme_gpui::Error>(())
/// ```
pub trait SystemThemeExt: sealed::Sealed {
    /// Convert this system theme to a gpui-component [`GpuiTheme`], as a
    /// [`Loaded`].
    ///
    /// Uses the OS-active variant (based on `mode`), the theme name, the
    /// color mode and the accessibility preferences from the `SystemTheme`.
    /// The variant is cloned out of `self`; [`from_system()`] moves it.
    #[must_use = "this returns the theme; it does not apply it"]
    fn to_gpui_theme(&self) -> Loaded;
}

/// A type outside the crate cannot implement [`SystemThemeExt`]:
///
/// ```compile_fail,E0277
/// struct Mine;
/// impl native_theme_gpui::SystemThemeExt for Mine {
///     fn to_gpui_theme(&self) -> native_theme_gpui::Loaded {
///         unimplemented!()
///     }
/// }
/// ```
#[cfg(doctest)]
pub struct SystemThemeExtIsSealed;

impl SystemThemeExt for SystemTheme {
    fn to_gpui_theme(&self) -> Loaded {
        Loaded {
            theme: system_gpui_theme(self),
            resolved: self.pick(self.mode).clone(),
            mode: self.mode,
            accessibility: self.accessibility.clone(),
            icon_set: self.icon_set,
            icon_theme: self
                .icon_theme_for(self.mode)
                .map(|s| Cow::Owned(s.to_owned())),
        }
    }
}

/// [`SystemThemeExt::to_gpui_theme`]'s theme, without cloning the variant.
fn system_gpui_theme(sys: &SystemTheme) -> GpuiTheme {
    to_theme(
        sys.pick(sys.mode),
        &sys.name,
        sys.mode.is_dark(),
        &sys.accessibility,
    )
}

// ---------------------------------------------------------------------------
// Helper functions (Issues 15, 17, 25, 32, 36, 37, 47, 48, 13)
// ---------------------------------------------------------------------------

/// Derive `is_dark` from a [`ResolvedTheme`]'s background lightness.
///
/// Returns `true` when the background lightness is below 0.5. This is a
/// convenience for callers that do not have an explicit dark-mode flag.
/// Some presets (e.g. solarized, gruvbox) have borderline values where the
/// auto-detected result may differ from the user's intent.
#[must_use]
pub fn is_dark_resolved(resolved: &ResolvedTheme) -> bool {
    colors::rgba_to_hsla(resolved.defaults.background_color).l < 0.5
}

// --- Issue 32: Accessibility helpers ---

/// Whether the user/OS has requested reduced motion.
#[must_use]
pub fn is_reduced_motion(sys: &SystemTheme) -> bool {
    sys.accessibility.reduce_motion
}

/// Whether the OS reports a high-contrast mode is active.
#[must_use]
pub fn is_high_contrast(sys: &SystemTheme) -> bool {
    sys.accessibility.high_contrast
}

/// Whether the user/OS has requested reduced transparency.
#[must_use]
pub fn is_reduced_transparency(sys: &SystemTheme) -> bool {
    sys.accessibility.reduce_transparency
}

// --- Issue 15: Defaults field accessors ---

/// Border line width from the resolved theme defaults.
#[must_use]
pub fn border_line_width(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.border.line_width
}

/// Disabled control opacity from the resolved theme defaults: the whole
/// widget fades by it on top of its disabled colours; 1.0 where the platform
/// dims by colour alone.
#[must_use]
pub fn disabled_opacity(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.disabled_opacity
}

/// The platform's line opacity from the resolved theme defaults: the share
/// of the text colour already folded into every stated border colour. Apply
/// it to no colour; a border colour is the final line colour.
#[must_use]
pub fn border_opacity(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.border.opacity
}

/// Whether drop shadows are enabled.
#[must_use]
pub fn border_shadow_enabled(resolved: &ResolvedTheme) -> bool {
    resolved.defaults.border.shadow_enabled
}

/// Text scaling factor (1.0 = no scaling).
#[must_use]
pub fn text_scaling_factor(sys: &SystemTheme) -> f32 {
    sys.accessibility.text_scaling_factor
}

// --- Issue 17: Spacing / icon-size / text-scale accessors ---

/// Access the per-context icon sizes from the resolved theme.
///
/// Returns icon sizes for toolbar, small, large, dialog, and panel contexts.
#[must_use]
pub fn icon_sizes(resolved: &ResolvedTheme) -> &native_theme::theme::ResolvedIconSizes {
    &resolved.defaults.icon_sizes
}

/// Access the text scale entries from the resolved theme.
///
/// Returns the 4-entry text scale (caption, section_heading, dialog_title, display).
#[must_use]
pub fn text_scale(resolved: &ResolvedTheme) -> &native_theme::theme::ResolvedTextScale {
    &resolved.text_scale
}

/// Scales a text size from the resolved theme by the user's text-scaling
/// preference, as [`to_theme`] scales `font_size` and the `geometry` builders
/// scale the widget fonts they apply.
///
/// For a size the connector does not apply itself: a `text_scale` role's
/// (`resolved.text_scale.caption.size`, …) or a widget font set by hand. The
/// factor applies to every text size alike. A factor that is not finite and
/// positive is ignored, and a size whose product with the factor is not a
/// normal positive number is returned as it is
/// ([`AccessibilityPreferences::scaled`]). `native_theme_egui::scaled_text_size`
/// returns that product as it is, by the egui connector's spec, §8.6, and the
/// egui atlas writes egui's own size in its place, with a `Note::ValueSanitised`.
#[must_use]
pub fn scaled_text_size(size: f32, prefs: &AccessibilityPreferences) -> f32 {
    prefs.scaled(size)
}

/// `layer` composited over `base` as the connector composites a theme's
/// state layers: source-over with straight alpha in sRGB, so the result
/// covers `layer.a + base.a × (1 − layer.a)`
/// ([`native_theme::color::composite_over`], which takes the layer first,
/// through `gpui::Rgba`). The filled buttons' hover and pressed fills
/// (`ThemeColor::button_hover`, `button_active`, …) and the widgets' hover
/// states are this composite of the theme's layer over the fill it covers,
/// so an application drawing a layered state of its own — a stated
/// `hover_background` over a control's fill — matches them through it. An
/// opaque layer, and any layer over a transparent base, comes back as it
/// is, and a transparent one gives `base`. Where `base` or `layer` is opaque this equals `Hsla::blend`;
/// where both are translucent `blend` keeps `base`'s alpha, and this does
/// not.
#[must_use]
pub fn layer_over(base: Hsla, layer: Hsla) -> Hsla {
    colors::over(base, layer)
}

/// A colour of the theme as the `Hsla` gpui paints: the conversion the
/// connector maps every colour of a [`ResolvedTheme`] with, so a colour an
/// application converts for a widget of its own is the one the connector would
/// give. Each channel, alpha included, is the byte over 255, through
/// `gpui::Rgba`; nothing is premultiplied or rounded.
///
/// # Examples
///
/// ```
/// use native_theme_gpui::{Rgba, to_hsla};
///
/// // An opaque colour stays opaque, a translucent one keeps its alpha.
/// assert_eq!(to_hsla(Rgba::rgb(0x3d, 0xae, 0xe9)).a, 1.0);
/// assert_eq!(to_hsla(Rgba::new(0, 0, 0, 0x80)).a, 128.0 / 255.0);
/// ```
#[must_use]
pub fn to_hsla(rgba: Rgba) -> Hsla {
    colors::rgba_to_hsla(rgba)
}

/// The family gpui is asked to draw `family` in. On macOS (`macos`), the
/// system UI font's stated family — "SF Pro" caselessly,
/// `native_theme::fonts::is_macos_system_ui_family` — becomes gpui's own
/// alias `.SystemUIFont`, "used to identify the system UI font, which varies
/// based on platform" (gpui-pre 0.3.10 `src/text_system.rs` line 1328): gpui's
/// macOS text system maps it to `.AppleSystemUIFont` through
/// `font_name_with_fallbacks` (gpui-pre 0.3.10 `src/text_system.rs` lines 1453–1463,
/// gpui-pre-apple 0.3.10 `src/text_system.rs` line 290) and looks that up among
/// its memory fonts first, then in the system source (lines 294–297), so Core
/// Text supplies its own system UI font. Its twin, the system monospaced
/// font's stated family — "SF Mono" caselessly,
/// `native_theme::fonts::is_macos_system_mono_family` — becomes the font's
/// family name, `.AppleSystemUIFontMonospaced` (`docs/platform-facts.md:65`;
/// the familyName macOS 26 reports for `+monospacedSystemFontOfSize:weight:`,
/// `docs/evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-26/probe.toml:162`),
/// which the same lookup takes as it takes `.AppleSystemUIFont`: font-kit's
/// `select_family_by_name`, memory fonts first, then the system source
/// (gpui-pre-apple 0.3.10 `src/text_system.rs` lines 290–297), a Core Text
/// query by `NSFontFamilyAttribute` (zed-font-kit 0.14.1-zed
/// `src/sources/core_text.rs` lines 69–80); "SF Mono" is in no font database
/// of the runners (M9-167). Every other family, and every
/// family elsewhere, is passed as stated: fontdb-style databases file the
/// font under `.SF NS`, but gpui does not draw from one. The platform is a
/// parameter so the unit tests run on every platform; the callers pass
/// `cfg!(target_os = "macos")`.
pub(crate) fn ui_font_family(family: &std::sync::Arc<str>, macos: bool) -> SharedString {
    if macos && native_theme::fonts::is_macos_system_ui_family(family) {
        SharedString::from(".SystemUIFont")
    } else if macos && native_theme::fonts::is_macos_system_mono_family(family) {
        SharedString::from(".AppleSystemUIFontMonospaced")
    } else {
        SharedString::from(family.clone())
    }
}

/// The family gpui is asked to draw a theme's `family` in: the macOS alias
/// of `ui_font_family`; with feature `system-fonts`, where the system
/// lacks `family`, the family the platform substitutes for it
/// (`native_theme::fonts::substitute_family_cached`: fontconfig's match on
/// Linux and the other non-Apple, non-Android unix systems, the font every
/// native application of the system gets); else `family` as
/// stated. gpui's own answer to a family its text system cannot find is a
/// list of its own, `.ZedMono` first (gpui-pre 0.3.10 `src/text_system.rs`
/// lines 262-273, 605-615), not the platform's. The platform is asked once
/// per family and the answer kept for the process, so a caller may ask
/// every frame. The first ask for a family runs on the calling thread: on
/// Linux and the other non-Apple, non-Android unix systems one `fc-match`
/// process, about 12.5 ms measured on one Linux machine, with a 2 s timeout.
/// A thread asking for another family, or for one already answered, does
/// not wait for it.
#[must_use]
pub fn font_family(family: &std::sync::Arc<str>) -> SharedString {
    let alias = ui_font_family(family, cfg!(target_os = "macos"));
    if alias.as_ref() != family.as_ref() {
        return alias;
    }
    match substituted_family(family) {
        Some(substitute) => SharedString::from(substitute),
        None => alias,
    }
}

/// A theme's font style as gpui's, for the `geometry` builders and the
/// widgets' labels alike.
pub(crate) fn font_style(style: native_theme::theme::FontStyle) -> gpui::FontStyle {
    match style {
        native_theme::theme::FontStyle::Normal => gpui::FontStyle::Normal,
        native_theme::theme::FontStyle::Italic => gpui::FontStyle::Italic,
        native_theme::theme::FontStyle::Oblique => gpui::FontStyle::Oblique,
    }
}

/// [`font_family`]'s substitute for `family`, asked of the platform once.
#[cfg(feature = "system-fonts")]
fn substituted_family(family: &str) -> Option<String> {
    native_theme::fonts::substitute_family_cached(family)
}

/// Without feature `system-fonts`, no family is substituted.
#[cfg(not(feature = "system-fonts"))]
fn substituted_family(_family: &str) -> Option<String> {
    None
}

// --- Issue 36: Line height ---

/// Line height multiplier from the resolved theme defaults.
#[must_use]
pub fn line_height(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.line_height
}

// --- Issue 13: Font weight helper ---

/// Default font weight from the resolved theme.
///
/// Returns the CSS font weight value (100-900).
#[must_use]
pub fn font_weight(resolved: &ResolvedTheme) -> u16 {
    resolved.defaults.font.weight
}

// --- Issue 47: Mono font weight helper ---

/// Monospace font weight from the resolved theme.
///
/// Returns the CSS font weight value (100-900).
#[must_use]
pub fn mono_font_weight(resolved: &ResolvedTheme) -> u16 {
    resolved.defaults.mono_font.weight
}

// --- Issue 48: Dialog button order helper ---

/// The platform-appropriate dialog button order.
///
/// Returns whether a dialog's primary (affirmative) button sits at the
/// leading (left) or the trailing (right) end of its button row: Windows and
/// KDE put it at the left end ([`DialogButtonOrder::PrimaryLeft`]), GNOME,
/// macOS, iOS and Material at the right ([`DialogButtonOrder::PrimaryRight`];
/// `docs/platform-facts.md` §2.22 `button_order`, and the ios-13 preset).
#[must_use]
pub fn dialog_button_order(resolved: &ResolvedTheme) -> DialogButtonOrder {
    resolved.dialog.button_order
}

// --- Issue 37: Padding/geometry helpers ---

/// Dialog button gap in logical pixels.
#[must_use]
pub fn dialog_button_gap(resolved: &ResolvedTheme) -> f32 {
    resolved.dialog.button_gap
}

/// Scrollbar groove width in logical pixels.
#[must_use]
pub fn scrollbar_groove_width(resolved: &ResolvedTheme) -> f32 {
    resolved.scrollbar.groove_width
}

/// Selection text color (foreground for selected content).
#[must_use]
pub fn selection_text_color(resolved: &ResolvedTheme) -> Rgba {
    resolved.defaults.selection_text_color
}

/// Selection background when window is unfocused.
#[must_use]
pub fn selection_inactive_background(resolved: &ResolvedTheme) -> Rgba {
    resolved.defaults.selection_inactive_background
}

/// Foreground color for disabled elements.
#[must_use]
pub fn disabled_text_color(resolved: &ResolvedTheme) -> Rgba {
    resolved.defaults.disabled_text_color
}

/// Focus ring stroke width in logical pixels.
#[must_use]
pub fn focus_ring_width(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.focus_ring_width
}

/// Gap between element edge and focus ring.
#[must_use]
pub fn focus_ring_offset(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.focus_ring_offset
}

/// The delay between the pointer coming to rest on an element and its
/// tooltip showing, `tooltip.show_delay_ms`; `None` where the theme states
/// none.
///
/// [`apply`] installs it as gpui-base's application-wide `TooltipDefaults`
/// show delay, which every tooltip shown through gpui-component's tooltip
/// overlay waits (a `Button::tooltip`'s, among others). GPUI's own
/// `.tooltip()` on an element does not read those defaults: it waits GPUI's
/// 500 ms (gpui-pre `src/elements/div.rs:53`) unless the element is given
/// this value through `StatefulInteractiveElement::tooltip_show_delay`
/// (`:1755`).
#[must_use]
pub fn tooltip_show_delay(resolved: &ResolvedTheme) -> Option<Duration> {
    resolved
        .tooltip
        .show_delay
        .map(|ms| Duration::from_millis(u64::from(ms)))
}

// ---------------------------------------------------------------------------
// Installation: NativeTheme global, apply family, base-layer observer (§8.1, §3.3)
// ---------------------------------------------------------------------------

/// The native theme installed by [`apply`]; one per `App`, read with
/// [`ActiveNativeTheme::native_theme`].
///
/// Stores both resolved variants of one theme (when known) because upstream's
/// `Theme::change` switches modes without going through the connector, and the
/// re-apply observer must then find the variant of the mode upstream switched
/// to (`docs/archive/todo_v0.5.8_gpui-component-0.6-rationale.md` §2.22).
/// Each variant is kept with the display name of the theme that stored it, so
/// a later `apply` of another theme drops it. Every field's default is the
/// right initial state, so `Default` is derived (a hand-written impl would
/// trip clippy's `derivable_impls`).
#[derive(Debug, Default)]
pub struct NativeTheme {
    light: Option<StoredVariant>,
    dark: Option<StoredVariant>,
    accessibility: AccessibilityPreferences,
    /// Set by the observer around its own write of `gpui_base::Theme`, so it
    /// can tell that notification from an upstream rebuild (§3.3).
    reapplying: bool,
    observer_installed: bool,
    /// Mode `apply` last installed; the fallback when the styled theme global
    /// is absent (D29).
    last_is_dark: bool,
}

impl Global for NativeTheme {}

/// A resolved variant and the display name of the theme that stored it.
#[derive(Debug)]
struct StoredVariant {
    name: SharedString,
    resolved: ResolvedTheme,
}

impl NativeTheme {
    fn is_dark(&self, cx: &App) -> bool {
        cx.try_global::<GpuiTheme>()
            .map(GpuiTheme::is_dark)
            .unwrap_or(self.last_is_dark)
    }

    fn stored(&self, is_dark: bool) -> Option<&StoredVariant> {
        if is_dark {
            self.dark.as_ref()
        } else {
            self.light.as_ref()
        }
    }

    fn variant(&self, is_dark: bool) -> Option<&ResolvedTheme> {
        self.stored(is_dark).map(|stored| &stored.resolved)
    }

    /// The stored variant for the styled theme's current mode, if any.
    #[must_use]
    pub fn resolved(&self, cx: &App) -> Option<&ResolvedTheme> {
        self.variant(self.is_dark(cx))
    }

    /// The preferences `apply` / `apply_accessibility` last installed.
    #[must_use]
    pub fn accessibility(&self) -> &AccessibilityPreferences {
        &self.accessibility
    }

    /// Borrowed view for the `geometry` builders, if a variant is stored for
    /// the current mode.
    #[must_use]
    pub fn native(&self, cx: &App) -> Option<Native<'_>> {
        self.resolved(cx).map(|resolved| Native {
            resolved,
            accessibility: &self.accessibility,
        })
    }
}

/// `cx.native_theme()` and `cx.native()`, mirroring gpui-component's
/// `cx.theme()`.
pub trait ActiveNativeTheme {
    /// The installed [`NativeTheme`], or `None` before [`apply`] ran.
    fn native_theme(&self) -> Option<&NativeTheme>;

    /// The [`Native`] view for the styled theme's current mode, or `None`
    /// before [`apply`] ran or when no variant is stored for that mode
    /// ([`NativeTheme::native`] of [`native_theme`](Self::native_theme)).
    fn native(&self) -> Option<Native<'_>>;
}

impl ActiveNativeTheme for App {
    fn native_theme(&self) -> Option<&NativeTheme> {
        self.try_global::<NativeTheme>()
    }

    fn native(&self) -> Option<Native<'_>> {
        self.native_theme()?.native(self)
    }
}

/// Borrowed inputs of every `geometry` builder
/// (`docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md` §9.1).
#[derive(Clone, Copy, Debug)]
pub struct Native<'a> {
    /// The resolved theme the values come from.
    pub resolved: &'a ResolvedTheme,
    /// Accessibility preferences; only `text_scaling_factor` affects geometry.
    pub accessibility: &'a AccessibilityPreferences,
}

impl<'a> Native<'a> {
    /// Unscaled, no reductions: for the preset path and tests.
    #[must_use]
    pub fn unscaled(resolved: &'a ResolvedTheme) -> Self {
        Self {
            resolved,
            accessibility: &AccessibilityPreferences::DEFAULT,
        }
    }
}

/// Install `theme` as gpui-component's global theme with the native base-layer
/// overrides, and keep them installed across upstream rebuilds
/// (`docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md` §8.1, §3.3).
///
/// 1. stores `resolved` under the theme's mode and `prefs` in [`NativeTheme`],
///    dropping the other mode's stored variant when another theme (one of a
///    different display name) stored it;
/// 2. initialises gpui-component if its theme global is absent (upstream
///    requires `gpui_component::init` before any component use; calling it
///    here only when the global is missing means it runs at most once);
/// 3. writes the styled theme, keeping the application's own `notification`,
///    `list`, `sheet`, `motion` and `cursor_blink` settings of the installed
///    one, as upstream's
///    `Theme::change` does; then installs a `ThemeConfig` for the *other*
///    stored variant (if any) under the same display name, so upstream's
///    `Theme::change` / `sync_system_appearance` reproduces native colours in
///    either mode, all but the ones step 8 restores;
/// 4. projects into gpui-base (`sync_base`);
/// 5. writes the native scrollbar geometry/colours and resize-handle colours
///    onto gpui-base ([`base_layer::apply_overrides`]);
/// 6. forwards a reduced-motion *request*: `prefs.reduce_motion == true`
///    switches `App::reduce_motion` on if it is off; a later `false` undoes
///    only that switch, never clears a flag the connector did not set, and
///    asks gpui-base (which reads the OS preference itself since 0.6.2) to
///    re-read the system;
/// 7. installs the variant's [`tooltip_show_delay`] as gpui-base's
///    application-wide `TooltipDefaults` show delay, keeping the grace period
///    installed; where the theme states no delay, the application's comes back:
///    the one installed before the connector's first write, or set by the
///    application since (gpui-base's 500 ms where it set none);
/// 8. installs, once per `App`, the observer that, whenever upstream rebuilds
///    the theme (a `Theme::change` or `sync_system_appearance` mode switch among
///    them), restores the 12 base-palette colours a `ThemeConfig` cannot carry
///    (`red` … `cyan_light`, private in `ThemeConfigColors`); restores
///    `selection`, `list_active` and `table_active`, with their tokens, where each
///    holds what gpui-component's alpha cap made of the stored variant's colour
///    (0.3, 0.2 and 0.2, gpui-component src/theme/schema.rs:1039-1070), through
///    `Theme::update`, which also rebuilds gpui-base's copies of `selection`;
///    restores step 5; and installs step 7's delay for the mode then current;
///    together with one deferred repeat for the update that installs it (the
///    subscription activates only at the end of that update's effect flush);
/// 9. refreshes every window so the change paints at once.
///
/// Call `gpui_component::init` (or `gpui_kit::init`) *before* `apply`: an
/// `init` afterwards switches the theme to light mode (the stored light
/// variant, or upstream's light constants when none is stored) and sets
/// gpui-component's scroll bar mode from the platform (`Hover` on Linux), in
/// place of the one `apply` chose. A single-variant
/// `apply` of a theme whose other variant is not stored (none was, or another
/// theme's was, which step 1 drops) leaves the other mode's config at
/// `ThemeConfig::default()`, a light config with no colours and an empty name.
/// Upstream's `apply_config` takes the mode from the config and stores it in
/// that mode's slot (gpui-component src/theme/schema.rs:1065-1069, :1108), so
/// a `Theme::change` to the unstored mode shows upstream's built-in
/// `ThemeColor::light()` constants, in light mode. Where the stored variant is
/// the light one, a change to dark therefore leaves the window light and
/// replaces the theme's light config with the empty one: a later change back
/// to light shows upstream's constants too, while the base palette,
/// scrollbars, resize handles, tooltip delay and [`ActiveNativeTheme::native`]
/// still follow the stored light variant (`selection`, `list_active` and
/// `table_active` keep upstream's constants: they hold no cap of the light
/// variant's colours), until the next `apply` or
/// [`apply_accessibility`] rebuilds the theme from it. Call `apply` once per
/// variant, or [`apply_system_theme`], to store both and avoid this.
///
/// Reduced motion: a flag that is already on is left to whoever switched it on
/// — gpui-base following the OS, or the application. While the connector's own
/// request stands, gpui-base discards OS readings, including the Linux
/// portal's first answer if `apply` ran before it arrived; once the request is
/// withdrawn, macOS and Windows are re-read at once and Linux follows again
/// from the portal's next change signal. gpui-base decides by value, not by
/// provenance, so an application flag that happens to equal gpui-base's own
/// last reading is replaced when the OS reading changes. To force the flag
/// either way regardless of the OS, call `cx.set_reduce_motion` after `apply`.
///
/// `theme` is moved into the global; `resolved` is cloned once.
pub fn apply(
    theme: GpuiTheme,
    resolved: &ResolvedTheme,
    prefs: &AccessibilityPreferences,
    cx: &mut App,
) {
    let is_dark = theme.is_dark();
    let (light, dark) = if is_dark {
        (None, Some(resolved))
    } else {
        (Some(resolved), None)
    };
    apply_inner(theme, light, dark, prefs, cx);
}

/// [`apply`] for the OS theme: installs the theme of the OS colour mode, as
/// [`SystemThemeExt::to_gpui_theme`] builds it, and stores both variants
/// with a `ThemeConfig` for each, so upstream's
/// `Theme::sync_system_appearance` and `Theme::change` reproduce native
/// colours in either mode, with what a config cannot carry or gpui-component
/// caps restored by [`apply`]'s observer (step 8).
///
/// It installs the mode the OS reports ([`SystemTheme::mode`]), so it needs no
/// `Theme::sync_system_appearance` after it. Calling that right after it can show
/// a dark desktop's application light for its first frames: on Linux GPUI
/// reports a light appearance until the desktop portal answers (gpui-pre-linux
/// src/linux/platform.rs:144, src/linux/xdg_desktop_portal.rs:49-51). Call
/// `sync_system_appearance` on an appearance change, or follow the OS theme with
/// `on_theme_change` (feature `watch`).
pub fn apply_system_theme(sys: &SystemTheme, cx: &mut App) {
    let theme = system_gpui_theme(sys);
    apply_inner(
        theme,
        Some(&sys.light),
        Some(&sys.dark),
        &sys.accessibility,
        cx,
    );
}

/// Apply a runtime change of the accessibility preferences (a portal signal,
/// a settings toggle). When a variant is stored for the current mode, the
/// styled theme is rebuilt from it with `prefs` and re-installed through the
/// [`apply`] path, so text scaling and transparency take effect and both
/// configs are refreshed; the stored variants are kept. Without a stored
/// variant only the reduced-motion request is forwarded and the preferences
/// are stored.
pub fn apply_accessibility(prefs: &AccessibilityPreferences, cx: &mut App) {
    let rebuilt = cx.try_global::<NativeTheme>().and_then(|nt| {
        let is_dark = nt.is_dark(cx);
        let stored = nt.stored(is_dark)?;
        Some(to_theme(&stored.resolved, &stored.name, is_dark, prefs))
    });
    match rebuilt {
        // `None, None` keeps the stored variants; apply_inner stores `prefs`.
        Some(theme) => apply_inner(theme, None, None, prefs, cx),
        None => {
            if cx.has_global::<NativeTheme>() {
                cx.global_mut::<NativeTheme>().accessibility = prefs.clone();
            }
            forward_reduce_motion(prefs.reduce_motion, cx);
        }
    }
}

/// Whether the connector switched `App::reduce_motion` on and has not yet
/// switched it back (spec v0.5.9 §5).
#[derive(Default)]
struct ReduceMotionRequest(bool);

impl Global for ReduceMotionRequest {}

/// `true` is a request: switch the flag on, remembering that the connector did
/// so if it was off. `false` is the absence of a request: undo the connector's
/// own switch, if any, and ask gpui-base (which reads the OS preference itself
/// since 0.6.2) to re-read the system. gpui-base writes only a flag it still
/// owns, so a flag the application set is never cleared here.
fn forward_reduce_motion(reduce: bool, cx: &mut App) {
    if reduce {
        if !cx.reduce_motion() {
            cx.default_global::<ReduceMotionRequest>().0 = true;
            cx.set_reduce_motion(true);
        }
        return;
    }
    if std::mem::take(&mut cx.default_global::<ReduceMotionRequest>().0) {
        cx.set_reduce_motion(false);
    }
    gpui_base::apply_system_reduce_motion(cx);
}

/// The show delay the connector last installed on gpui-base's
/// `TooltipDefaults`, and the application's own, which a theme without a
/// delay brings back (spec §7.4).
#[derive(Default)]
struct TooltipDelayWrite {
    /// What the connector last installed; `None` before its first write.
    last_written: Option<Duration>,
    /// The delay installed before the connector's first write, or set by the
    /// application since.
    app_delay: Duration,
}

impl Global for TooltipDelayWrite {}

/// Install `theme`'s delay as the `TooltipDefaults` show delay, or the
/// application's where the theme states none. A show delay that is not the
/// one the connector last wrote was set by the application (or is gpui-base's
/// default, before the first write), so it becomes the application's delay.
/// The grace period is never touched. Decided by value, as gpui-base decides
/// reduced motion: an application delay equal to the one the connector last
/// wrote is taken for the connector's.
fn install_tooltip_delay(theme: Option<Duration>, cx: &mut App) {
    let current = gpui_base::TooltipDefaults::global(cx);
    let state = cx.default_global::<TooltipDelayWrite>();
    if state.last_written != Some(current.show_delay()) {
        state.app_delay = current.show_delay();
    }
    let delay = theme.unwrap_or(state.app_delay);
    state.last_written = Some(delay);
    if delay != current.show_delay() {
        current.with_show_delay(delay).install(cx);
    }
}

/// [`install_tooltip_delay`] for the stored variant of the styled theme's
/// current mode, or for no delay where none is stored.
fn install_variant_tooltip_delay(cx: &mut App) {
    let delay = cx
        .try_global::<NativeTheme>()
        .and_then(|nt| nt.resolved(cx))
        .and_then(tooltip_show_delay);
    install_tooltip_delay(delay, cx);
}

fn apply_inner(
    mut theme: GpuiTheme,
    light: Option<&ResolvedTheme>,
    dark: Option<&ResolvedTheme>,
    prefs: &AccessibilityPreferences,
    cx: &mut App,
) {
    let is_dark = theme.is_dark();
    // The display name of the config `to_theme` built for this mode; the other
    // variant's config takes the same name.
    let name: SharedString = theme.theme_name().clone();
    {
        let nt = cx.default_global::<NativeTheme>();
        // The stored pair is the two modes of one theme: another theme's
        // variant would be installed under this theme's name.
        nt.light.take_if(|stored| stored.name != name);
        nt.dark.take_if(|stored| stored.name != name);
        if let Some(light) = light {
            nt.light = Some(StoredVariant {
                name: name.clone(),
                resolved: light.clone(),
            });
        }
        if let Some(dark) = dark {
            nt.dark = Some(StoredVariant {
                name: name.clone(),
                resolved: dark.clone(),
            });
        }
        nt.accessibility = prefs.clone();
        nt.last_is_dark = is_dark;
    }

    // D29: the styled accessors panic on a missing global; init creates it.
    if !cx.has_global::<GpuiTheme>() {
        gpui_component::init(cx);
    }
    // The application's settings, not theme data: upstream's own switch
    // (`Theme::change` -> `apply_config`, gpui-component
    // src/theme/schema.rs:1064-1108) leaves them, and so does `apply`.
    let installed = GpuiTheme::global_mut(cx);
    theme.notification = installed.notification.clone();
    theme.list = installed.list.clone();
    theme.sheet = installed.sheet.clone();
    theme.motion = installed.motion.clone();
    theme.cursor_blink = installed.cursor_blink;
    *installed = theme;

    // D34: the other mode's config from its stored variant, so Theme::change
    // reproduces the native palette instead of the registry default.
    let other_config = cx.try_global::<NativeTheme>().and_then(|nt| {
        nt.variant(!is_dark).map(|other| {
            let mode = if is_dark {
                GpuiThemeMode::Light
            } else {
                GpuiThemeMode::Dark
            };
            Rc::new(config::to_theme_config(other, &name, mode, prefs))
        })
    });
    if let Some(cfg) = other_config {
        let styled = GpuiTheme::global_mut(cx);
        if is_dark {
            styled.light_theme = cfg;
        } else {
            styled.dark_theme = cfg;
        }
    }

    GpuiTheme::sync_base(cx);
    // `false`: this write's notification may be delivered before the observer
    // is active (§3.3), so it must not be marked as the observer's own.
    write_base_overrides(cx, false);
    forward_reduce_motion(prefs.reduce_motion, cx);
    install_variant_tooltip_delay(cx);
    install_observer_once(cx);
    // D37: paint now. A change from a timer, portal signal or menu action must
    // not wait for the next input event. The theme is written through
    // `Theme::global_mut` and `sync_base`, which refresh nothing -- upstream
    // leaves the refresh to such a caller (gpui-component
    // src/theme/mod.rs:468-469), where its own `Theme::change` refreshes every
    // window (`:334`) -- and `refresh_windows` schedules every window (gpui-pre
    // src/app.rs:1373).
    cx.refresh_windows();
}

/// The values to write onto gpui-base for `is_dark` (spec §3.3 step 2):
/// the stored variant when there is one; otherwise geometry from the other
/// variant and colours from the styled theme, which is exactly upstream's own
/// projection (`scrollbar`, `scrollbar_thumb`, `scrollbar_thumb_hover` with
/// active = hover; `border` / `drag_border` for the handles).
fn base_overrides_for(
    nt: &NativeTheme,
    is_dark: bool,
    styled: Option<&GpuiTheme>,
) -> Option<(base_layer::ScrollbarGeometry, gpui_base::ResizableTheme)> {
    if let Some(resolved) = nt.variant(is_dark) {
        return Some((
            base_layer::scrollbar_geometry(resolved),
            base_layer::resizable_theme(resolved),
        ));
    }
    let other = nt.variant(!is_dark)?;
    let mut geometry = base_layer::scrollbar_geometry(other);
    let mut resizable = base_layer::resizable_theme(other);
    if let Some(styled) = styled {
        geometry.track = styled.scrollbar;
        geometry.track_active_border = styled.border;
        geometry.thumb = styled.scrollbar_thumb;
        geometry.thumb_hover = styled.scrollbar_thumb_hover;
        geometry.thumb_active = styled.scrollbar_thumb_hover;
        resizable.handle = Some(styled.border);
        resizable.active_handle = Some(styled.drag_border);
    }
    Some((geometry, resizable))
}

/// Compute and write the overrides for the current mode. `mark` flags the
/// write as the observer's own so the observer ignores its notification
/// (§3.3); only the observer passes `true`, because a flag set by `apply`
/// could be delivered before the observer is active and would then never be
/// cleared (rationale errors 40, 42).
fn write_base_overrides(cx: &mut App, mark: bool) {
    let Some(nt) = cx.try_global::<NativeTheme>() else {
        return;
    };
    let is_dark = nt.is_dark(cx);
    let Some((geometry, resizable)) = base_overrides_for(nt, is_dark, cx.try_global::<GpuiTheme>())
    else {
        return;
    };
    if mark {
        cx.global_mut::<NativeTheme>().reapplying = true;
    }
    base_layer::apply_overrides(&geometry, resizable, cx);
}

/// Whether gpui-base's resize-handle colours are the ones the connector would
/// write for the current mode. `ResizableTheme` derives no `PartialEq`, so both
/// fields are compared. `true` when nothing is stored (nothing to repair).
fn handles_hold_native_values(cx: &App) -> bool {
    let Some(nt) = cx.try_global::<NativeTheme>() else {
        return true;
    };
    let Some((_, expected)) = base_overrides_for(nt, nt.is_dark(cx), cx.try_global::<GpuiTheme>())
    else {
        return true;
    };
    let Some(current) = cx.try_global::<gpui_base::Theme>().map(|t| t.resizable) else {
        return true;
    };
    current.handle == expected.handle && current.active_handle == expected.active_handle
}

/// Restore the 12 base-palette colours (`red` … `cyan_light`) of the styled
/// theme, and their tokens, from the stored variant for the current mode
/// (D43).
///
/// `ThemeConfigColors` keeps them private (gpui-component
/// `src/theme/schema.rs:643-677`), so the config `apply` installs for a variant
/// cannot carry them; every `Theme::change` / `sync_system_appearance` resets
/// them to `ThemeColor::dark()` / `light()` (`:691-699`, `:1079-1083`) and
/// ends in `sync_base` (`theme/mod.rs:333`), whose `cx.set_global` of
/// `gpui_base::Theme` (`:473-478`) is the one write of it; its notification
/// reaches the base-theme observer. Writes only when a field differs; the
/// styled theme has no upstream observer, so the write triggers no rebuild.
/// After `apply` the connector is the sole writer of these 12 fields: it
/// writes them back on every notification of gpui-base's theme, so a value an
/// application sets on them does not stay (its own `Theme::update` causes such a
/// notification).
///
/// The three colours gpui-component caps (`selection`, `list_active`,
/// `table_active`) are restored beside this repair by [`restore_capped_colors`],
/// which, unlike it, writes through `Theme::update`: gpui-base keeps copies of
/// `selection` that only `sync_base` refreshes.
fn repair_base_palette(cx: &mut App) {
    let Some(nt) = cx.try_global::<NativeTheme>() else {
        return;
    };
    let is_dark = nt.is_dark(cx);
    let Some(resolved) = nt.variant(is_dark) else {
        return;
    };
    let native = colors::base_palette(resolved, is_dark);
    // Probe a copy first so an unchanged palette causes no notification.
    let differs = cx.try_global::<GpuiTheme>().is_some_and(|styled| {
        let mut probe = styled.clone();
        colors::copy_base_palette(&native, &mut probe)
    });
    if differs {
        colors::copy_base_palette(&native, GpuiTheme::global_mut(cx));
    }
}

/// Restore `selection`, `list_active` and `table_active` of the styled theme,
/// with their tokens, from the stored variant for the current mode, where
/// each holds what gpui-component's alpha cap made of it; `true` when any was
/// restored.
///
/// gpui-component caps their alpha at 0.3, 0.2 and 0.2 whenever it applies a
/// `ThemeConfig` (gpui-component 0.9.1 `src/theme/schema.rs:1039-1070`, a
/// colour the config states included, `src/theme/color.rs:777-791`), so after
/// every `Theme::change`, `sync_system_appearance` or registry reload they
/// differ from what [`apply`] installed. A field is restored only when it holds
/// exactly that cap of the native colour (`colors::restore_capped`); any other
/// value is the application's and stays. A value the application sets to
/// exactly the capped native colour cannot be told from the cap and is
/// replaced too.
///
/// The write goes through `Theme::update`, which keeps the tokens in step and
/// ends in `sync_base`: gpui-base keeps copies of `selection` in its colour
/// tokens (gpui-component `src/theme/mod.rs:547`) and in the text view's
/// defaults (`src/text/mod.rs:54`), and paints selected text from the first
/// (gpui-base 0.9.1 `src/selectable_text.rs:221-223`). `sync_base` resets the
/// base overrides, so a caller writes them after a restore. A copy is probed
/// first, as in [`repair_base_palette`], because `Theme::update` notifies the
/// base theme's observers even when nothing changed.
fn restore_capped_colors(cx: &mut App) -> bool {
    let Some(nt) = cx.try_global::<NativeTheme>() else {
        return false;
    };
    let is_dark = nt.is_dark(cx);
    let Some(resolved) = nt.variant(is_dark) else {
        return false;
    };
    let native = colors::capped_natives(resolved);
    let capped = cx.try_global::<GpuiTheme>().is_some_and(|styled| {
        let mut probe = styled.clone();
        colors::restore_capped(&native, &mut probe)
    });
    if !capped {
        return false;
    }
    GpuiTheme::update(cx, |styled| colors::restore_capped(&native, styled))
}

/// Observe `gpui_base::Theme` (§3.3). On every delivery it repairs the base
/// palette ([`repair_base_palette`]), installs the current mode's tooltip delay,
/// and restores the three colours gpui-component caps ([`restore_capped_colors`]);
/// a restore ends in `sync_base`, which resets the base overrides, so it is
/// followed by a marked write of them and the delivery ends there. Otherwise the
/// delivery that carries the observer's own write is absorbed by `reapplying`,
/// and any other one is answered with a marked write. Terminates because
/// `global_mut` and `set_global` queue one deduplicated notification
/// (gpui-pre 0.3.12 `src/app.rs:2117-2131`, `:2555-2581`), delivered after the
/// pending mark is removed (`:2292-2297`): the observer's writes yield exactly one
/// further delivery, which finds nothing to restore and is absorbed.
///
/// The subscription activates through a deferred effect (`src/app.rs:2602-2615`)
/// at the end of the flush that follows this call, so a base-theme write made by
/// other code in the same update as the first `apply` (a
/// `Theme::sync_system_appearance` right after it) would stand until the next
/// rebuild. One deferred re-write, queued after the activation, closes that gap
/// (D38): its write is the first notification the active observer receives, and
/// that delivery repairs the palette and restores the three as any other does.
fn install_observer_once(cx: &mut App) {
    if cx
        .try_global::<NativeTheme>()
        .is_none_or(|nt| nt.observer_installed)
    {
        return;
    }
    cx.global_mut::<NativeTheme>().observer_installed = true;
    cx.observe_global::<gpui_base::Theme>(|cx| {
        // A rebuild also resets the 12 private base-palette colours (D43);
        // a plain comparison makes this safe on every delivery.
        repair_base_palette(cx);
        // A mode switch upstream (`Theme::change`) ends in this rebuild, and
        // the new mode's variant may state another delay. It writes only a
        // changed delay, to `TooltipDefaults`, which this observer does not
        // watch.
        install_variant_tooltip_delay(cx);
        // Before the branch below: a rebuild merged into the notification of
        // this observer's own write caps the three too, and the flag alone
        // would swallow it (spec §3.3, limit). The restore ended in
        // `sync_base`, which reset the base overrides; this write's
        // notification, merged with the sync's, is the one further delivery,
        // and `reapplying` absorbs it.
        if restore_capped_colors(cx) {
            write_base_overrides(cx, true);
            return;
        }
        let Some(nt) = cx.try_global::<NativeTheme>() else {
            return;
        };
        if nt.reapplying {
            cx.global_mut::<NativeTheme>().reapplying = false;
            // A rebuild another effect performs between this observer's write
            // and the delivery of its notification is merged into that
            // notification by GPUI's per-type deduplication, so the flag alone
            // would swallow it (spec §3.3, limit). The handle colours are
            // comparable and upstream's rebuild changes them, so repair when
            // they no longer hold the native values; the scrollbar styles are
            // opaque and cannot be checked.
            if !handles_hold_native_values(cx) {
                write_base_overrides(cx, true);
            }
            return;
        }
        write_base_overrides(cx, true);
    })
    .detach();
    // Effects are FIFO, so this runs after the activation above and after any
    // same-update write. `false`: the active observer re-applies once on its
    // delivery and absorbs its own write, like any other rebuild (§3.3).
    cx.defer(|cx| {
        write_base_overrides(cx, false);
        repair_base_palette(cx);
    });
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// Tripwire: `gpui_component::Theme` has exactly these 22 fields in
    /// 0.9.1. Upstream carried three `tile_*` fields through 0.6.0 and 0.6.1
    /// that this connector never set, and nothing noticed until 0.6.2 removed
    /// them again; a field *added* in a release would be left at upstream's
    /// default just as silently. The struct is `#[non_exhaustive]` since
    /// 0.9.0, so a pattern naming every field no longer compiles without `..`;
    /// the names are read from its derived `Debug` instead, which prints every
    /// field, `#[serde(skip)]` ones included, so either change fails here by
    /// name.
    #[test]
    fn every_theme_field_is_named() {
        let theme = to_theme(
            &test_resolved(),
            "Test",
            true,
            &AccessibilityPreferences::default(),
        );
        let named = [
            // Set by `to_theme` from the resolved native theme.
            "mode",
            "font_family",
            "font_size",
            "mono_font_family",
            "mono_font_size",
            "radius",
            "radius_lg",
            "window_radius",
            "shadow",
            "focus_ring",
            "scrollbar_mode",
            "highlight_theme",
            // `to_theme` fills the slot for `mode` only; the other keeps
            // `Theme::from(&ThemeColor)`'s empty `ThemeConfig::default()` until
            // a later `apply` stores the other variant (D34).
            "light_theme",
            "dark_theme",
            // Written by `Theme::from(&ThemeColor)`: the 139-field colour map,
            // the tokens upstream derives from it, and `transparent` (Issue 53).
            "colors",
            "tokens",
            "transparent",
            // Left to the application: native-theme models no notification
            // placement, list highlight default, sheet margin, motion tokens
            // or caret blink (`ResolvedTheme` has no such field), so
            // `to_theme` leaves upstream's default and `apply` keeps the
            // installed theme's.
            "notification",
            "list",
            "sheet",
            "motion",
            "cursor_blink",
        ];
        // `{:#?}` puts each top-level field on a line of its own, indented
        // by four spaces; nested fields are indented further.
        let debug = format!("{theme:#?}");
        let mut fields: Vec<&str> = debug
            .lines()
            .filter_map(|line| line.strip_prefix("    "))
            .filter(|rest| !rest.starts_with(' '))
            .filter_map(|rest| rest.split_once(": ").map(|(name, _)| name))
            .collect();
        let mut expected = named.to_vec();
        fields.sort_unstable();
        expected.sort_unstable();
        assert_eq!(fields, expected, "gpui_component::Theme's fields changed");
    }

    /// Issue 1: fixed to use into_variant(true) for catppuccin-mocha (dark theme).
    fn test_resolved() -> ResolvedTheme {
        let nt = Theme::preset("catppuccin-mocha").expect("preset must exist");
        let variant = nt
            .into_variant(ColorMode::Dark)
            .expect("preset must have dark variant");
        variant
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("resolved preset must validate")
    }

    /// A family the system lacks is drawn in the family the platform
    /// substitutes for it, where feature `system-fonts` asks it; the answer
    /// is kept, so asking again gives the same family.
    #[test]
    fn a_missing_family_is_the_platforms_substitute() {
        let family = std::sync::Arc::<str>::from("native-theme: no such family");
        let drawn = font_family(&family);
        let platform = if cfg!(feature = "system-fonts") {
            substituted_family(&family)
        } else {
            None
        };
        match platform {
            Some(substitute) => assert_eq!(drawn.as_ref(), substitute),
            None => assert_eq!(drawn.as_ref(), family.as_ref()),
        }
        assert_eq!(font_family(&family), drawn, "asked again, another family");
    }

    /// On macOS the system UI font's stated family becomes gpui's own alias
    /// for it, `.SystemUIFont`; every other family, and every family on
    /// another platform, stays as stated (egui spec §8.8). The platform is
    /// a parameter, so this runs everywhere.
    #[test]
    fn the_macos_system_ui_font_is_gpuis_alias_only_on_macos() {
        for name in ["SF Pro", "sf pro"] {
            let family = std::sync::Arc::<str>::from(name);
            assert_eq!(ui_font_family(&family, true).as_ref(), ".SystemUIFont");
            assert_eq!(ui_font_family(&family, false).as_ref(), name);
        }
        for name in ["SF Pro Text", "SF Mono Text", "Inter", "Menlo"] {
            let family = std::sync::Arc::<str>::from(name);
            assert_eq!(ui_font_family(&family, true).as_ref(), name);
            assert_eq!(ui_font_family(&family, false).as_ref(), name);
        }
    }

    /// M9x CI item 10 (M9-167): on macOS the system monospaced font's stated
    /// family, "SF Mono" caselessly, becomes the face's own name,
    /// `.AppleSystemUIFontMonospaced` (docs/platform-facts.md:65), which gpui's
    /// macOS text system looks up as it does `.AppleSystemUIFont`; on another
    /// platform it stays as stated. The platform is a parameter, so this runs
    /// everywhere.
    #[test]
    fn the_macos_system_mono_font_is_its_face_only_on_macos() {
        for name in ["SF Mono", "sf mono", "SF MONO"] {
            let family = std::sync::Arc::<str>::from(name);
            assert_eq!(
                ui_font_family(&family, true).as_ref(),
                ".AppleSystemUIFontMonospaced",
                "{name} on macOS"
            );
            assert_eq!(ui_font_family(&family, false).as_ref(), name);
        }
    }

    fn scaled(factor: f32) -> AccessibilityPreferences {
        let mut prefs = AccessibilityPreferences::default();
        prefs.text_scaling_factor = factor;
        prefs
    }

    #[test]
    fn to_theme_produces_valid_theme() {
        let resolved = test_resolved();
        let theme = to_theme(
            &resolved,
            "Test",
            true,
            &AccessibilityPreferences::default(),
        );

        // Theme should have the correct mode
        assert!(theme.is_dark());
    }

    #[test]
    fn to_theme_dark_mode() {
        let nt = Theme::preset("catppuccin-mocha").expect("preset must exist");
        let variant = nt
            .into_variant(ColorMode::Dark)
            .expect("preset must have dark variant");
        let resolved = variant
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("resolved preset must validate");
        let theme = to_theme(
            &resolved,
            "DarkTest",
            true,
            &AccessibilityPreferences::default(),
        );

        assert!(theme.is_dark());
    }

    #[test]
    fn to_theme_applies_font_and_geometry() {
        let resolved = test_resolved();
        let theme = to_theme(
            &resolved,
            "Test",
            true,
            &AccessibilityPreferences::default(),
        );

        assert_eq!(
            theme.font_family,
            font_family(&resolved.defaults.font.family)
        );
        assert_eq!(theme.font_size, px(resolved.defaults.font.size));
        assert_eq!(
            theme.mono_font_family,
            font_family(&resolved.defaults.mono_font.family)
        );
        assert_eq!(theme.mono_font_size, px(resolved.defaults.mono_font.size));
        assert_eq!(
            theme.radius,
            px(resolved.defaults.border.corner_radius.max(0.0))
        );
        assert_eq!(
            theme.radius_lg,
            px(resolved.defaults.border.corner_radius_lg.max(0.0))
        );
        assert_eq!(
            theme.window_radius,
            px(resolved.window.border.corner_radius)
        );
        assert_eq!(theme.shadow, resolved.defaults.border.shadow_enabled);
    }

    // §8.3: scrollbar_mode set from overlay_mode
    #[test]
    fn scrollbar_mode_from_overlay_mode() {
        let resolved = test_resolved();
        let theme = to_theme(
            &resolved,
            "Scroll",
            true,
            &AccessibilityPreferences::default(),
        );
        let expected = if resolved.scrollbar.overlay_mode {
            ScrollbarMode::Scrolling
        } else {
            ScrollbarMode::Always
        };
        assert_eq!(theme.scrollbar_mode, expected);
    }

    // Issue 43/44: highlight_theme matches is_dark
    #[test]
    fn highlight_theme_matches_is_dark() {
        let resolved = test_resolved();
        let dark_theme = to_theme(
            &resolved,
            "Dark",
            true,
            &AccessibilityPreferences::default(),
        );
        assert_eq!(
            dark_theme.highlight_theme.appearance,
            GpuiThemeMode::Dark,
            "dark theme should use dark highlight"
        );

        let light_resolved = {
            let spec = Theme::preset("catppuccin-latte").expect("preset must exist");
            let variant = spec.into_variant(ColorMode::Light).expect("light variant");
            variant
                .into_resolved(&native_theme::ResolutionContext::for_tests())
                .expect("must validate")
        };
        let light_theme = to_theme(
            &light_resolved,
            "Light",
            false,
            &AccessibilityPreferences::default(),
        );
        assert_eq!(
            light_theme.highlight_theme.appearance,
            GpuiThemeMode::Light,
            "light theme should use light highlight"
        );
    }

    /// The code editor takes the platform's field fill, `input.background_color`,
    /// in every preset and mode. Where that fill is translucent its gutter is
    /// the fill over the window, opaque wherever the window is, so text
    /// scrolling under the gutter cannot show through it; where it is opaque
    /// the gutter is left unset, so upstream paints it in the editor's fill
    /// and halves it with the fill in a disabled editor. Both branches occur.
    #[test]
    fn the_code_editor_takes_the_platforms_field_fill() {
        let prefs = AccessibilityPreferences::default();
        let (mut translucent, mut opaque) = (Vec::new(), Vec::new());
        for info in Theme::list_presets() {
            for is_dark in [false, true] {
                let (theme, r) =
                    from_preset(info.key, is_dark, &prefs).expect("preset should load");
                let at = format!("{}/{is_dark}", info.key);
                let fill = colors::rgba_to_hsla(r.input.background_color);
                let window = colors::rgba_to_hsla(r.window.background_color);
                let style = &theme.highlight_theme.style;
                assert_eq!(
                    style.editor_background,
                    Some(fill),
                    "{at}: the editor's fill"
                );
                let gutter = style.editor_gutter_background;
                if fill.a < 1.0 {
                    assert_eq!(gutter, Some(colors::over(window, fill)), "{at}: the gutter");
                    if window.a >= 1.0 {
                        assert_eq!(gutter.map(|g| g.a), Some(1.0), "{at}: a translucent gutter");
                    }
                    translucent.push(at);
                } else {
                    assert_eq!(gutter, None, "{at}: an opaque fill's gutter is set");
                    opaque.push(at);
                }
            }
        }
        assert!(
            !translucent.is_empty() && !opaque.is_empty(),
            "the presets no longer state both a translucent and an opaque field \
             fill, so one of the gutter's branches goes untested (translucent: \
             {translucent:?})"
        );
    }

    /// §3.4: font sizes carry the factor; the config copies too, so
    /// `Theme::change` reproduces them.
    #[test]
    fn to_theme_scales_font_sizes_by_the_text_scaling_factor() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Scaled", true, &scaled(1.5));
        assert_eq!(theme.font_size, px(resolved.defaults.font.size * 1.5));
        assert_eq!(
            theme.mono_font_size,
            px(resolved.defaults.mono_font.size * 1.5)
        );
        assert_eq!(
            theme.dark_theme.font_size,
            Some(resolved.defaults.font.size * 1.5)
        );
        assert_eq!(
            theme.dark_theme.mono_font_size,
            Some(resolved.defaults.mono_font.size * 1.5)
        );
    }

    /// A text size the connector does not apply itself scales as the ones it
    /// does, and a degenerate factor leaves it as the theme states it.
    #[test]
    fn scaled_text_size_scales_every_size_alike() {
        let resolved = test_resolved();
        for size in [resolved.button.font.size, resolved.text_scale.caption.size] {
            assert_eq!(scaled_text_size(size, &scaled(1.5)), size * 1.5);
            for factor in [0.0, -1.0, f32::NAN, f32::INFINITY] {
                assert_eq!(
                    scaled_text_size(size, &scaled(factor)),
                    size,
                    "factor {factor}"
                );
            }
        }
    }

    /// §7.2: a non-finite or non-positive factor means "no scaling".
    #[test]
    fn to_theme_ignores_a_degenerate_text_scaling_factor() {
        let resolved = test_resolved();
        for factor in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            let theme = to_theme(&resolved, "Degenerate", true, &scaled(factor));
            assert_eq!(
                theme.font_size,
                px(resolved.defaults.font.size),
                "factor {factor}"
            );
        }
    }

    /// A finite factor whose product with a font size overflows f32 leaves
    /// the sizes as the theme states them, in the theme and in its config.
    #[test]
    fn to_theme_leaves_a_font_size_the_factor_overflows_unscaled() {
        let resolved = test_resolved();
        let d = &resolved.defaults;
        for factor in [f32::MAX / 4.0, f32::MAX] {
            assert!((d.font.size * factor).is_infinite(), "factor {factor}");
            assert!((d.mono_font.size * factor).is_infinite(), "factor {factor}");
            let theme = to_theme(&resolved, "Overflow", true, &scaled(factor));
            assert!(f32::from(theme.font_size).is_finite(), "factor {factor}");
            assert_eq!(theme.font_size, px(d.font.size), "factor {factor}");
            assert_eq!(
                theme.mono_font_size,
                px(d.mono_font.size),
                "factor {factor}"
            );
            assert_eq!(
                theme.dark_theme.font_size,
                Some(d.font.size),
                "factor {factor}"
            );
            assert_eq!(
                theme.dark_theme.mono_font_size,
                Some(d.mono_font.size),
                "factor {factor}"
            );
        }
    }

    /// §8.3: only the flag has a receiver; a zero-width ring is not drawn.
    #[test]
    fn focus_ring_follows_focus_ring_width() {
        let resolved = test_resolved();
        let theme = to_theme(
            &resolved,
            "Ring",
            true,
            &AccessibilityPreferences::default(),
        );
        assert_eq!(theme.focus_ring, resolved.defaults.focus_ring_width > 0.0);
    }

    // -- from_preset tests --

    #[test]
    fn from_preset_takes_preferences() {
        let (theme, resolved) =
            from_preset("catppuccin-latte", false, &scaled(1.5)).expect("preset should load");
        assert_eq!(theme.font_size, px(resolved.defaults.font.size * 1.5));
    }

    #[test]
    fn from_preset_valid_light() {
        let (theme, _resolved) = from_preset(
            "catppuccin-latte",
            false,
            &AccessibilityPreferences::default(),
        )
        .expect("preset should load");
        assert!(!theme.is_dark());
    }

    #[test]
    fn from_preset_valid_dark() {
        let (theme, _resolved) = from_preset(
            "catppuccin-mocha",
            true,
            &AccessibilityPreferences::default(),
        )
        .expect("preset should load");
        assert!(theme.is_dark());
    }

    #[test]
    fn from_preset_returns_resolved() {
        let (_theme, resolved) = from_preset(
            "catppuccin-mocha",
            true,
            &AccessibilityPreferences::default(),
        )
        .expect("preset should load");
        // ResolvedTheme should have populated defaults
        assert!(resolved.defaults.font.size > 0.0);
    }

    #[test]
    fn from_preset_invalid_name() {
        let result = from_preset(
            "nonexistent-preset",
            false,
            &AccessibilityPreferences::default(),
        );
        assert!(result.is_err(), "invalid preset should return Err");
    }

    // Issue 23: error message includes the mode
    #[test]
    fn from_preset_error_message_includes_mode() {
        // Both modes should load for catppuccin-mocha (it has both variants)
        let _ = from_preset(
            "catppuccin-mocha",
            true,
            &AccessibilityPreferences::default(),
        )
        .expect("dark should work");
        let _ = from_preset(
            "catppuccin-mocha",
            false,
            &AccessibilityPreferences::default(),
        )
        .expect("light should work");
    }

    #[test]
    fn from_preset_takes_is_dark_from_the_resolved_mode() {
        let prefs = AccessibilityPreferences::default();
        let presets = Theme::list_presets();
        assert!(!presets.is_empty());
        for info in presets {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let (theme, _) = from_preset(info.key, mode.is_dark(), &prefs)
                    .unwrap_or_else(|e| panic!("{} {mode:?}: {e}", info.key));
                let resolved = Theme::preset(info.key)
                    .and_then(|spec| spec.resolve(mode))
                    .unwrap_or_else(|e| panic!("{} {mode:?}: {e}", info.key));
                assert_eq!(
                    theme.is_dark(),
                    resolved.mode.is_dark(),
                    "{} asked for {mode:?}",
                    info.key
                );
            }
        }
    }

    #[test]
    fn from_preset_light_only_asked_for_dark_is_light() {
        let mut spec = Theme::preset("catppuccin-latte").expect("preset should load");
        spec.dark = None;
        let (theme, resolved) = from_spec(&spec, true, &AccessibilityPreferences::default())
            .expect("the light variant should resolve");
        assert!(!theme.is_dark());
        assert_eq!(
            Some(resolved.defaults.background_color),
            spec.light
                .as_ref()
                .and_then(|light| light.defaults.background_color)
        );
    }

    // -- SystemThemeExt + from_system tests --

    /// The OS theme, for a test that reads it. On Linux it is never an
    /// error: a desktop with no reader falls back to the platform preset
    /// (native-theme `pipeline.rs`, `linux_fallback`), so an error there
    /// fails the test rather than skip it. Off Linux a build without a
    /// reader feature has no OS theme, and the test skips.
    pub(super) fn system_theme() -> Result<SystemTheme> {
        let sys = SystemTheme::from_system();
        if cfg!(target_os = "linux") {
            return Ok(sys.expect("Linux falls back to the platform preset"));
        }
        sys
    }

    #[test]
    fn system_theme_ext_to_gpui_theme() {
        let Ok(sys) = system_theme() else {
            return;
        };
        let loaded = sys.to_gpui_theme();
        assert_eq!(
            loaded.theme.is_dark(),
            sys.mode.is_dark(),
            "to_gpui_theme() is_dark should match SystemTheme.mode"
        );
        assert_eq!(&loaded.resolved, sys.pick(sys.mode));
        assert_eq!(loaded.mode, sys.mode);
        assert_eq!(loaded.accessibility, sys.accessibility);
        assert_eq!(loaded.icon_set, sys.icon_set);
        assert_eq!(loaded.icon_theme.as_deref(), sys.icon_theme_for(sys.mode));
    }

    /// `from_system` is a theme wherever the OS theme is one, on Linux
    /// always ([`system_theme`]).
    #[test]
    fn from_system_is_ok_where_the_os_theme_is() {
        assert_eq!(from_system().is_ok(), system_theme().is_ok());
    }

    #[test]
    fn from_system_matches_system_theme_ext() {
        let Ok(sys) = system_theme() else {
            return;
        };
        let loaded = from_system().expect("the OS theme was read");
        let expected = sys.to_gpui_theme();
        assert!(loaded.resolved.defaults.font.size > 0.0);
        assert_eq!(loaded.theme.is_dark(), loaded.mode.is_dark());
        assert_eq!(loaded.theme.is_dark(), expected.theme.is_dark());
        assert_eq!(loaded.theme.background, expected.theme.background);
        assert_eq!(loaded.resolved, expected.resolved);
        assert_eq!(loaded.mode, expected.mode);
        assert_eq!(loaded.accessibility, expected.accessibility);
        assert_eq!(loaded.icon_set, expected.icon_set);
        assert_eq!(loaded.icon_theme, expected.icon_theme);
    }

    #[test]
    fn from_system_matches_manual_path() {
        let Ok(sys) = system_theme() else {
            return;
        };
        let via_convenience = sys.to_gpui_theme().theme;
        let via_manual = to_theme(
            sys.pick(sys.mode),
            &sys.name,
            sys.mode.is_dark(),
            &sys.accessibility,
        );
        // Both paths should produce identical results
        assert_eq!(
            via_convenience.is_dark(),
            via_manual.is_dark(),
            "convenience and manual paths should agree on is_dark"
        );
        // Issue 39: the active variant has at least its accent or its
        // background populated.
        let resolved = sys.pick(sys.mode);
        assert!(
            resolved.defaults.accent_color != Rgba::TRANSPARENT
                || resolved.defaults.background_color != Rgba::TRANSPARENT,
            "resolved variant should have at least accent or background populated"
        );
    }

    /// The crate root re-exports native-theme, so an application names its
    /// API through `native_theme_gpui::native_theme` with no dependency of
    /// its own, and it is the copy the connector's own types come from.
    #[test]
    fn native_theme_is_reached_through_the_connector() {
        let _: fn() = crate::native_theme::detect::invalidate_caches;
        let theme: crate::native_theme::theme::Theme = Theme::preset("adwaita").unwrap();
        let resolved = theme.resolve(ColorMode::Light).unwrap();
        let _ = to_theme(
            &resolved.variant,
            &theme.name,
            false,
            &AccessibilityPreferences::default(),
        );
    }

    /// `to_hsla` is the conversion `to_theme` installs a theme's colours with.
    #[test]
    fn to_hsla_gives_the_colours_to_theme_installs() {
        let prefs = AccessibilityPreferences::default();
        for is_dark in [false, true] {
            let loaded = from_preset("macos-tahoe", is_dark, &prefs);
            assert!(
                loaded.is_ok(),
                "macos-tahoe should load: {:?}",
                loaded.as_ref().err()
            );
            let Ok((_, r)) = loaded else {
                return;
            };
            let theme = to_theme(&r, "Test", is_dark, &prefs);
            assert_eq!(theme.background, to_hsla(r.window.background_color));
            assert_eq!(theme.selection, to_hsla(r.input.selection_background));
            assert_eq!(theme.list_active, to_hsla(r.list.selection_background));
        }
    }

    // -- Issue 25/32: helper function tests --

    #[test]
    fn is_dark_resolved_matches_background() {
        let resolved = test_resolved();
        let bg = colors::rgba_to_hsla(resolved.defaults.background_color);
        assert_eq!(
            is_dark_resolved(&resolved),
            bg.l < 0.5,
            "is_dark_resolved should match background lightness"
        );
    }

    /// Each reader returns its own preference: with one flag set, only its
    /// reader is `true`, and the factor is the one stated.
    #[test]
    fn the_accessibility_readers_read_their_own_preference() {
        let Ok(mut sys) = system_theme() else {
            return;
        };
        type Reader = fn(&SystemTheme) -> bool;
        let readers: [(&str, Reader); 3] = [
            ("reduce_motion", is_reduced_motion),
            ("high_contrast", is_high_contrast),
            ("reduce_transparency", is_reduced_transparency),
        ];
        for (set, _) in readers {
            let a11y = &mut sys.accessibility;
            a11y.reduce_motion = set == "reduce_motion";
            a11y.high_contrast = set == "high_contrast";
            a11y.reduce_transparency = set == "reduce_transparency";
            for (name, reader) in readers {
                assert_eq!(reader(&sys), name == set, "{name} with only {set} set");
            }
        }
        sys.accessibility.text_scaling_factor = 1.25;
        assert_eq!(text_scaling_factor(&sys), 1.25);
    }

    #[test]
    fn defaults_field_helpers() {
        let resolved = test_resolved();
        assert!(border_line_width(&resolved) >= 0.0);
        assert!(disabled_opacity(&resolved) >= 0.0);
        assert!(disabled_opacity(&resolved) <= 1.0);
        assert!(border_opacity(&resolved) >= 0.0);
    }

    #[test]
    fn icon_sizes_helper() {
        let resolved = test_resolved();
        let sizes = icon_sizes(&resolved);
        assert!(sizes.toolbar > 0.0, "toolbar icon size should be positive");
    }

    #[test]
    fn text_scale_helper() {
        let resolved = test_resolved();
        let ts = text_scale(&resolved);
        assert!(ts.caption.size > 0.0, "caption size should be positive");
    }

    #[test]
    fn font_weight_helper() {
        let resolved = test_resolved();
        let w = font_weight(&resolved);
        assert!((100..=900).contains(&w), "font weight should be 100-900");
    }

    #[test]
    fn mono_font_weight_helper() {
        let resolved = test_resolved();
        let w = mono_font_weight(&resolved);
        assert!(
            (100..=900).contains(&w),
            "mono font weight should be 100-900"
        );
    }

    #[test]
    fn dialog_button_order_helper() {
        let resolved = test_resolved();
        let _order = dialog_button_order(&resolved);
        // Just verify it doesn't panic
    }

    #[test]
    fn line_height_helper() {
        let resolved = test_resolved();
        assert!(
            line_height(&resolved) > 0.0,
            "line height should be positive"
        );
    }

    #[test]
    fn geometry_helpers() {
        let resolved = test_resolved();
        assert!(dialog_button_gap(&resolved) >= 0.0);
        assert!(scrollbar_groove_width(&resolved) > 0.0);
    }

    #[test]
    fn selection_and_disabled_helpers() {
        let resolved = test_resolved();
        let _ = selection_text_color(&resolved);
        let _ = selection_inactive_background(&resolved);
        let _ = disabled_text_color(&resolved);
    }

    #[test]
    fn focus_ring_helpers() {
        let resolved = test_resolved();
        assert!(focus_ring_width(&resolved) >= 0.0);
        assert!(focus_ring_offset(&resolved) >= 0.0);
    }

    #[test]
    fn tooltip_show_delay_is_the_stated_delay_or_none() {
        let prefs = AccessibilityPreferences::default();
        let (_, breeze) = from_preset("kde-breeze", false, &prefs).unwrap();
        assert_eq!(breeze.tooltip.show_delay, Some(700), "precondition");
        assert_eq!(
            tooltip_show_delay(&breeze),
            Some(std::time::Duration::from_millis(700))
        );
        let (_, material) = from_preset("material-3", false, &prefs).unwrap();
        assert_eq!(material.tooltip.show_delay, Some(0), "precondition");
        assert_eq!(
            tooltip_show_delay(&material),
            Some(std::time::Duration::ZERO)
        );
        let (_, mocha) = from_preset("catppuccin-mocha", true, &prefs).unwrap();
        assert_eq!(mocha.tooltip.show_delay, None, "precondition");
        assert_eq!(tooltip_show_delay(&mocha), None);
    }

    // -- Issue 26: integration tests for all 17 presets in both modes --

    #[test]
    fn all_presets_dark_mode_no_panic() {
        let presets = Theme::list_presets();
        for info in presets {
            let name = info.key;
            let result = from_preset(name, true, &AccessibilityPreferences::default());
            assert!(
                result.is_ok(),
                "from_preset({name}, true) failed: {:?}",
                result.err()
            );
        }
    }

    #[test]
    fn all_presets_light_mode_no_panic() {
        let presets = Theme::list_presets();
        for info in presets {
            let name = info.key;
            let result = from_preset(name, false, &AccessibilityPreferences::default());
            assert!(
                result.is_ok(),
                "from_preset({name}, false) failed: {:?}",
                result.err()
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod apply_tests {
    use super::*;
    use gpui::TestAppContext;
    use gpui_component::scroll::ScrollbarMode;
    use std::cell::Cell;

    /// Spec v0.5.9 §5: `false` is the absence of a request. gpui-base 0.6.2+
    /// (or the application) may have switched the flag on; a preset applied
    /// with default preferences must not switch it off.
    #[gpui::test]
    fn apply_with_default_prefs_keeps_a_flag_it_did_not_set(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) =
            from_preset("catppuccin-mocha", true, &prefs).expect("preset should load");
        cx.update(|cx| {
            cx.set_reduce_motion(true); // stands for gpui-base's OS reading
            apply(theme, &resolved, &prefs, cx);
            assert!(cx.reduce_motion());
        });
    }

    #[gpui::test]
    fn releasing_a_request_keeps_a_flag_that_was_already_on(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.set_reduce_motion(true);
            apply_accessibility(&prefs(true), cx);
            apply_accessibility(&prefs(false), cx);
            assert!(cx.reduce_motion(), "the connector never switched it on");
        });
    }

    #[gpui::test]
    fn a_repeated_request_is_released_once(cx: &mut TestAppContext) {
        cx.update(|cx| {
            apply_accessibility(&prefs(true), cx);
            apply_accessibility(&prefs(true), cx);
            assert!(cx.reduce_motion());
            apply_accessibility(&prefs(false), cx);
            assert!(!cx.reduce_motion());
        });
    }

    fn prefs(reduce_motion: bool) -> AccessibilityPreferences {
        let mut prefs = AccessibilityPreferences::default();
        prefs.reduce_motion = reduce_motion;
        prefs
    }

    /// The dark catppuccin preset, with the precondition every observer test
    /// relies on: the native *active* handle colour (`splitter.hover_color`)
    /// differs from `drag_border`, which upstream's projection writes there.
    /// `handle` cannot serve as the observable: every preset inherits
    /// `splitter.divider_color` from `defaults.border.color`
    /// (`docs/inheritance-rules.toml:250`), which is also what upstream writes,
    /// so that slot holds the same value whichever side wrote it.
    fn preset_for_observer_tests(prefs: &AccessibilityPreferences) -> (GpuiTheme, ResolvedTheme) {
        let (theme, resolved) =
            from_preset("catppuccin-mocha", true, prefs).expect("preset should load");
        assert_ne!(
            colors::rgba_to_hsla(resolved.splitter.hover_color),
            theme.drag_border,
            "precondition: the native active-handle colour must differ from upstream's drag_border"
        );
        (theme, resolved)
    }

    /// A preset whose light and dark variants resolve to different backgrounds.
    fn preset_with_two_variants(
        prefs: &AccessibilityPreferences,
    ) -> ((GpuiTheme, ResolvedTheme), (GpuiTheme, ResolvedTheme)) {
        for info in Theme::list_presets() {
            if let (Ok(dark), Ok(light)) = (
                from_preset(info.key, true, prefs),
                from_preset(info.key, false, prefs),
            ) && dark.1.defaults.background_color != light.1.defaults.background_color
            {
                return (dark, light);
            }
        }
        panic!("no preset with distinct light and dark variants");
    }

    /// §12: after `apply`, every receiver holds the native value; after upstream
    /// rebuilds the base layer, the observer restores it; and the test returning
    /// proves the observer terminates. Two changes catch a stale `reapplying`.
    #[gpui::test]
    fn apply_installs_and_survives_theme_change(cx: &mut TestAppContext) {
        let prefs = prefs(true);
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let handle = colors::rgba_to_hsla(resolved.splitter.divider_color);
        let active = colors::rgba_to_hsla(resolved.splitter.hover_color);
        let expected_mode = if resolved.scrollbar.overlay_mode {
            ScrollbarMode::Scrolling
        } else {
            ScrollbarMode::Always
        };

        let native_palette = colors::base_palette(&resolved, true);

        // No gpui_component::init here: apply must initialise the styled layer
        // itself when it is absent (D29) and must not panic.
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));

        cx.update(|cx| {
            assert!(GpuiTheme::global(cx).is_dark());
            let base = gpui_base::Theme::global(cx);
            assert_eq!(base.scrollbar.mode(), expected_mode);
            assert_eq!(base.resizable.handle, Some(handle));
            assert_eq!(base.resizable.active_handle, Some(active));
            assert!(cx.reduce_motion());
            assert!(cx.native_theme().and_then(|t| t.resolved(cx)).is_some());
            // Upstream's projection writes `drag_border` into active_handle; the
            // helper's precondition makes the restore assertions below meaningful.
            assert_ne!(GpuiTheme::global(cx).drag_border, active);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx); // rebuilds gpui_base::Theme
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "observer restored the native value after the first rebuild"
            );
            // D43: the rebuild reset the 12 private base-palette fields to
            // upstream's constants; the observer copied the native ones back.
            let styled = GpuiTheme::global(cx);
            assert_eq!(
                styled.red, native_palette.red,
                "red restored after Theme::change"
            );
            assert_eq!(styled.magenta_light, native_palette.magenta_light);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "and after the second rebuild (no stale reapplying flag)"
            );
        });
    }

    /// §3.3, D38: the observer activates at the end of the effect flush that
    /// installs it, so a rebuild in the *same* update as the first `apply` (an
    /// application calling `Theme::sync_system_appearance` right after it) is
    /// delivered while the observer is inactive. The deferred re-write in
    /// `install_observer_once` closes that gap; without it this test fails.
    #[gpui::test]
    fn apply_then_change_in_the_same_update_keeps_overrides(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let active = colors::rgba_to_hsla(resolved.splitter.hover_color);
        let native_palette = colors::base_palette(&resolved, true);
        cx.update(|cx| {
            apply(theme, &resolved, &prefs, cx);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx); // same update: observer not yet active
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "the deferred re-write restored the overrides after a same-update rebuild"
            );
            assert_eq!(
                GpuiTheme::global(cx).red,
                native_palette.red,
                "and the base palette (D43)"
            );
        });
    }

    /// D43 precondition: upstream's rebuild really does reset the base palette
    /// (otherwise the repair assertions above would be vacuous). Upstream's
    /// own dark palette, read after a plain `init` + `change`, differs from the
    /// preset's; `Theme::change` after `apply` would install it were there no
    /// repair, and the repaired value is the preset's.
    #[gpui::test]
    fn theme_change_resets_private_base_palette_without_repair(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let native_palette = colors::base_palette(&resolved, true);
        let upstream_red = cx.update(|cx| {
            gpui_component::init(cx);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
            GpuiTheme::global(cx).red
        });
        assert_ne!(
            upstream_red, native_palette.red,
            "the preset's danger colour must differ from upstream's red for this test to mean anything"
        );
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        cx.update(|cx| {
            let red = GpuiTheme::global(cx).red;
            assert_ne!(red, upstream_red, "the repair replaced upstream's constant");
            assert_eq!(red, native_palette.red);
        });
    }

    /// D43 with the tokens: after `apply` and an upstream rebuild, each of the
    /// 12 base-palette tokens is its plain field's solid colour, the native
    /// one; without the repair the rebuild leaves upstream's constants in
    /// both, which the precondition shows for `red`.
    #[gpui::test]
    fn the_base_palette_repair_keeps_the_tokens_with_the_colours(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let native = colors::base_palette(&resolved, true);
        let upstream = cx.update(|cx| {
            gpui_component::init(cx);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
            GpuiTheme::global(cx).tokens.red
        });
        assert_ne!(
            upstream.color, native.red,
            "upstream's red token must differ from the preset's for this test to mean anything"
        );
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        cx.update(|cx| {
            let styled = GpuiTheme::global(cx);
            macro_rules! check {
                ($($f:ident),*) => {$(
                    assert_eq!(
                        styled.tokens.$f,
                        gpui_component::theme::ThemeToken::from(native.$f),
                        concat!("tokens.", stringify!($f), " is not the native solid colour")
                    );
                    assert_eq!(styled.colors.$f, native.$f);
                )*};
            }
            check!(
                red,
                red_light,
                green,
                green_light,
                blue,
                blue_light,
                yellow,
                yellow_light,
                magenta,
                magenta_light,
                cyan,
                cyan_light
            );
        });
        // A token that no longer matches a colour that does is repaired too:
        // the probe compares the tokens, not the colours alone.
        cx.update(|cx| {
            GpuiTheme::global_mut(cx).tokens.red = upstream;
            assert_eq!(GpuiTheme::global(cx).colors.red, native.red);
            repair_base_palette(cx);
            assert_eq!(
                GpuiTheme::global(cx).tokens.red,
                gpui_component::theme::ThemeToken::from(native.red),
                "a token that differs while its colour matches is not repaired"
            );
        });
    }

    /// macos-tahoe's light and dark variants, with the preconditions the tests
    /// of the capped colours rely on: both variants share one display name (so
    /// both are stored), the cap changes `selection` and `list_active` in each,
    /// and the four colours are distinct (so a restore from the wrong variant
    /// or the wrong field fails). A preset that does not load fails the
    /// assertion, so the result is never `None`.
    fn tahoe(
        prefs: &AccessibilityPreferences,
    ) -> Option<((GpuiTheme, ResolvedTheme), (GpuiTheme, ResolvedTheme))> {
        let light = from_preset("macos-tahoe", false, prefs);
        let dark = from_preset("macos-tahoe", true, prefs);
        assert!(
            light.is_ok() && dark.is_ok(),
            "macos-tahoe should load: {:?} {:?}",
            light.as_ref().err(),
            dark.as_ref().err()
        );
        let (light, dark) = (light.ok()?, dark.ok()?);
        assert_eq!(
            light.0.theme_name(),
            dark.0.theme_name(),
            "precondition: one display name, so both variants are stored"
        );
        let [light_selection, light_list, _] = natives(&light.1, false);
        let [dark_selection, dark_list, _] = natives(&dark.1, true);
        for (selection, list) in [(light_selection, light_list), (dark_selection, dark_list)] {
            assert!(selection.a > 0.3, "precondition: the cap changes selection");
            assert!(list.a > 0.2, "precondition: the cap changes list_active");
            assert_ne!(
                selection, list,
                "precondition: selection is not list_active"
            );
        }
        assert_ne!(
            light_selection, dark_selection,
            "precondition: two selections"
        );
        assert_ne!(light_list, dark_list, "precondition: two list_actives");
        Some((light, dark))
    }

    /// The `selection`, `list_active` and `table_active` `apply` installs.
    fn natives(resolved: &ResolvedTheme, is_dark: bool) -> [Hsla; 3] {
        let tc = colors::to_theme_color(resolved, is_dark, false);
        [tc.selection, tc.list_active, tc.table_active]
    }

    /// The three capped fields of `styled`, each with its token.
    fn three(styled: &GpuiTheme) -> [(&'static str, Hsla, gpui_component::theme::ThemeToken); 3] {
        [
            (
                "selection",
                styled.colors.selection,
                styled.tokens.selection,
            ),
            (
                "list_active",
                styled.colors.list_active,
                styled.tokens.list_active,
            ),
            (
                "table_active",
                styled.colors.table_active,
                styled.tokens.table_active,
            ),
        ]
    }

    /// Each of the three fields is `expected`, with its solid token, and
    /// gpui-base's selection token and the text view's selection are
    /// `expected[0]`.
    fn assert_three(cx: &App, expected: [Hsla; 3], what: &str) {
        for ((name, color, token), want) in three(GpuiTheme::global(cx)).into_iter().zip(expected) {
            assert_eq!(color, want, "{name}, {what}");
            assert_eq!(
                token,
                gpui_component::theme::ThemeToken::from(want),
                "the token of {name}, {what}"
            );
        }
        assert_eq!(
            gpui_base::Theme::global(cx).tokens.colors.selection,
            expected[0],
            "gpui-base's selection token, {what}"
        );
        assert_eq!(
            gpui_base::text::editor_host::RenderContext::from_app(cx)
                .style()
                .selection(),
            expected[0],
            "the text view's selection, {what}"
        );
    }

    /// Count the deliveries of `gpui_base::Theme` notifications. The
    /// subscription activates at the end of this update's flush, so register
    /// it in an update of its own, before the one it measures.
    fn count_deliveries(cx: &mut App) -> Rc<Cell<usize>> {
        let count = Rc::new(Cell::new(0));
        let counter = count.clone();
        cx.observe_global::<gpui_base::Theme>(move |_| counter.set(counter.get() + 1))
            .detach();
        count
    }

    /// The precondition of the restore and the exactness end to end: with no
    /// observer, upstream's `Theme::change` leaves each of the three exactly
    /// at the native colour with the cap's alpha, and its solid token.
    #[gpui::test]
    fn theme_change_caps_the_three_without_the_restore(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let Some((_, (_, dark))) = tahoe(&prefs) else {
            return;
        };
        let native = natives(&dark, true);
        cx.update(|cx| {
            gpui_component::init(cx);
            GpuiTheme::global_mut(cx).dark_theme = Rc::new(config::to_theme_config(
                &dark,
                "Test",
                GpuiThemeMode::Dark,
                &prefs,
            ));
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            let fields = three(GpuiTheme::global(cx));
            for ((name, color, token), (n, cap)) in fields
                .into_iter()
                .zip(native.into_iter().zip([0.3, 0.2, 0.2]))
            {
                let capped = n.alpha(cap);
                assert_eq!(color, capped, "{name} is not the native colour at the cap");
                assert_eq!(
                    token,
                    gpui_component::theme::ThemeToken::from(capped),
                    "the token of {name} is not the capped solid colour"
                );
                assert_ne!(color, n, "precondition: the cap changed {name}");
            }
        });
    }

    /// Sequence (a): a switch either way leaves the three native, with their
    /// tokens and gpui-base's copies, in two deliveries.
    #[gpui::test]
    fn a_mode_switch_restores_the_three_capped_colours(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let Some(((light_theme, light), (dark_theme, dark))) = tahoe(&prefs) else {
            return;
        };
        let (native_light, native_dark) = (natives(&light, false), natives(&dark, true));
        cx.update(|cx| {
            apply(light_theme, &light, &prefs, cx);
            apply(dark_theme, &dark, &prefs, cx);
        });
        cx.update(|cx| assert_three(cx, native_dark, "after both applys"));
        let count = cx.update(count_deliveries);
        cx.update(|cx| {
            count.set(0);
            GpuiTheme::change(GpuiThemeMode::Light, None, cx);
        });
        cx.update(|cx| {
            assert!(!GpuiTheme::global(cx).is_dark());
            assert_three(cx, native_light, "after the switch to light");
            assert_eq!(count.get(), 2, "deliveries of the switch to light");
        });
        cx.update(|cx| {
            count.set(0);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            assert!(GpuiTheme::global(cx).is_dark());
            assert_three(cx, native_dark, "after the switch back to dark");
            assert_eq!(count.get(), 2, "deliveries of the switch to dark");
        });
    }

    #[gpui::test]
    fn apply_system_theme_restores_the_three_after_a_switch(cx: &mut TestAppContext) {
        let Ok(mut sys) = super::tests::system_theme() else {
            return;
        };
        let Some(((_, light), (_, dark))) = tahoe(&AccessibilityPreferences::default()) else {
            return;
        };
        let (native_light, native_dark) = (natives(&light, false), natives(&dark, true));
        sys.light = light;
        sys.dark = dark;
        sys.mode = ColorMode::Dark;
        cx.update(|cx| apply_system_theme(&sys, cx));
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Light, None, cx));
        cx.update(|cx| assert_three(cx, native_light, "after the switch to light"));
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        cx.update(|cx| assert_three(cx, native_dark, "after the switch back to dark"));
    }

    /// Sequence (e): a `sync_system_appearance` in the update of the first
    /// `apply` is restored by the delivery the deferred re-write causes.
    #[gpui::test]
    fn sync_system_appearance_in_the_first_applys_update_is_restored(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let Some(((light_theme, light), (dark_theme, dark))) = tahoe(&prefs) else {
            return;
        };
        let native_light = natives(&light, false);
        cx.update(|cx| {
            assert_eq!(
                cx.window_appearance(),
                gpui::WindowAppearance::Light,
                "precondition: the test platform reports a light appearance"
            );
            apply(light_theme, &light, &prefs, cx);
            apply(dark_theme, &dark, &prefs, cx);
            GpuiTheme::sync_system_appearance(None, cx);
        });
        cx.update(|cx| {
            assert!(!GpuiTheme::global(cx).is_dark());
            assert_three(cx, native_light, "after the same-update sync");
        });
    }

    /// A rebuild merged into the notification of the observer's own write is
    /// restored: the restore runs before the `reapplying` branch.
    #[gpui::test]
    fn a_rebuild_merged_into_the_observers_own_notification_is_restored(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let Some(((light_theme, light), (dark_theme, dark))) = tahoe(&prefs) else {
            return;
        };
        let native_light = natives(&light, false);
        cx.update(|cx| {
            apply(light_theme, &light, &prefs, cx);
            apply(dark_theme, &dark, &prefs, cx);
        });
        cx.update(|cx| {
            let _ = gpui_base::Theme::global_mut(cx);
            cx.defer(|cx| GpuiTheme::change(GpuiThemeMode::Light, None, cx));
        });
        cx.update(|cx| assert_three(cx, native_light, "after the merged rebuild"));
    }

    /// Sequence (b): a value of the application's own stays through the
    /// delivery its write causes, is replaced by the next rebuild, and stays
    /// when written again in the update of the switch.
    #[gpui::test]
    fn an_applications_own_value_survives_the_delivery_after_its_write(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let Some(((light_theme, light), (dark_theme, dark))) = tahoe(&prefs) else {
            return;
        };
        let (native_light, native_dark) = (natives(&light, false), natives(&dark, true));
        cx.update(|cx| {
            apply(light_theme, &light, &prefs, cx);
            apply(dark_theme, &dark, &prefs, cx);
        });
        let count = cx.update(count_deliveries);
        let own = native_dark[2].alpha(0.25);
        let assert_own = |cx: &App, what: &str| {
            let styled = GpuiTheme::global(cx);
            assert_eq!(styled.colors.table_active, own, "table_active, {what}");
            assert_eq!(
                styled.tokens.table_active,
                gpui_component::theme::ThemeToken::from(own),
                "the token of table_active, {what}"
            );
            assert_eq!(styled.colors.selection, native_dark[0], "selection, {what}");
            assert_eq!(
                styled.colors.list_active, native_dark[1],
                "list_active, {what}"
            );
        };
        cx.update(|cx| {
            count.set(0);
            GpuiTheme::update(cx, |t| t.table_active = t.table_active.alpha(0.25));
        });
        cx.update(|cx| {
            assert_own(cx, "after the application's write");
            assert_eq!(count.get(), 2, "deliveries of the application's write");
        });
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Light, None, cx));
        cx.update(|cx| assert_three(cx, native_light, "after the switch to light"));
        cx.update(|cx| {
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
            GpuiTheme::update(cx, |t| t.table_active = t.table_active.alpha(0.25));
        });
        cx.update(|cx| assert_own(cx, "written again in the update of the switch"));
    }

    /// Sequence (c): a value equal to the capped native colour cannot be told
    /// from the cap and is replaced.
    #[gpui::test]
    fn a_value_equal_to_the_cap_is_replaced(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let Some(((light_theme, light), (dark_theme, dark))) = tahoe(&prefs) else {
            return;
        };
        let native_dark = natives(&dark, true);
        cx.update(|cx| {
            apply(light_theme, &light, &prefs, cx);
            apply(dark_theme, &dark, &prefs, cx);
        });
        let count = cx.update(count_deliveries);
        cx.update(|cx| {
            count.set(0);
            GpuiTheme::update(cx, |t| t.selection = t.selection.alpha(0.3));
        });
        cx.update(|cx| {
            assert_three(cx, native_dark, "after a write of the capped colour");
            assert_eq!(count.get(), 2, "deliveries of the write");
        });
    }

    /// Sequence (d), the dark variant stored: a switch to light shows what
    /// upstream's switch makes, and the switch back is restored.
    #[gpui::test]
    fn a_mode_without_a_stored_variant_keeps_upstreams_colours_dark_stored(
        cx: &mut TestAppContext,
    ) {
        let prefs = AccessibilityPreferences::default();
        let Some((_, (dark_theme, dark))) = tahoe(&prefs) else {
            return;
        };
        let native_dark = natives(&dark, true);
        cx.update(|cx| apply(dark_theme, &dark, &prefs, cx));
        let expected = cx.update(|cx| {
            let mut probe = GpuiTheme::global(cx).clone();
            let cfg = probe.light_theme.clone();
            probe.apply_config(&cfg);
            GpuiTheme::change(GpuiThemeMode::Light, None, cx);
            three(&probe)
        });
        cx.update(|cx| {
            for ((name, color, token), (_, want, want_token)) in
                three(GpuiTheme::global(cx)).into_iter().zip(expected)
            {
                assert_eq!(color, want, "{name} is not upstream's");
                assert_eq!(token, want_token, "the token of {name} is not upstream's");
            }
            assert_eq!(
                gpui_base::Theme::global(cx).tokens.colors.selection,
                expected[0].1,
                "gpui-base's selection token"
            );
            assert_eq!(
                gpui_base::text::editor_host::RenderContext::from_app(cx)
                    .style()
                    .selection(),
                expected[0].1,
                "the text view's selection"
            );
        });
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        cx.update(|cx| assert_three(cx, native_dark, "after the switch back to dark"));
    }

    /// Sequence (d), the light variant stored: a switch to dark installs the
    /// empty light config, and the three keep upstream's colours.
    #[gpui::test]
    fn a_mode_without_a_stored_variant_keeps_upstreams_colours_light_stored(
        cx: &mut TestAppContext,
    ) {
        let prefs = AccessibilityPreferences::default();
        let Some(((light_theme, light), _)) = tahoe(&prefs) else {
            return;
        };
        let native_light = natives(&light, false);
        cx.update(|cx| apply(light_theme, &light, &prefs, cx));
        let expected = cx.update(|cx| {
            let mut probe = GpuiTheme::global(cx).clone();
            let cfg = probe.dark_theme.clone();
            probe.apply_config(&cfg);
            let expected = three(&probe);
            for ((name, color, _), (n, cap)) in expected
                .into_iter()
                .zip(native_light.into_iter().zip([0.3, 0.2, 0.2]))
            {
                assert_ne!(
                    color,
                    n.alpha(cap),
                    "precondition: upstream's {name} is not the light native at its cap"
                );
            }
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
            expected
        });
        cx.update(|cx| {
            assert!(
                !GpuiTheme::global(cx).is_dark(),
                "the empty config is a light one"
            );
            for ((name, color, token), (_, want, want_token)) in
                three(GpuiTheme::global(cx)).into_iter().zip(expected)
            {
                assert_eq!(color, want, "{name} is not upstream's");
                assert_eq!(token, want_token, "the token of {name} is not upstream's");
            }
        });
    }

    /// The `reapplying` flag cannot tell the observer's own notification from a
    /// rebuild another effect performs before that notification is delivered:
    /// GPUI merges the two (spec §3.3, limit). The observer therefore repairs
    /// the handle colours when they no longer hold the native values. Without
    /// that repair the deferred `Theme::change` below would leave upstream's
    /// `drag_border` in `active_handle`.
    #[gpui::test]
    fn observer_repairs_a_rebuild_merged_into_its_own_notification(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let active = colors::rgba_to_hsla(resolved.splitter.hover_color);
        cx.update(|cx| apply(theme, &resolved, &prefs, cx)); // observer active after this flush
        cx.update(|cx| {
            // Queue: [N_base (this write), Defer(change)]. The observer answers
            // N_base with a marked write whose notification lands after the
            // deferred change; the change's own notification is deduplicated.
            let _ = gpui_base::Theme::global_mut(cx);
            cx.defer(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "the observer repaired the rebuild that its own notification had absorbed"
            );
        });
    }

    /// §3.3 step 2: with no stored variant for the new mode, colours come from
    /// the styled theme (upstream's own projection) and nothing panics.
    #[gpui::test]
    fn apply_without_stored_variant_falls_back(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs); // stores dark only
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Light, None, cx));
        cx.update(|cx| {
            let nt = cx.native_theme().expect("installed by apply");
            assert!(nt.resolved(cx).is_none(), "no light variant was stored");
            assert!(nt.native(cx).is_none());
            let styled = GpuiTheme::global(cx);
            let base = gpui_base::Theme::global(cx);
            assert_eq!(base.resizable.handle, Some(styled.border));
            assert_eq!(base.resizable.active_handle, Some(styled.drag_border));
        });
    }

    #[gpui::test]
    fn apply_system_theme_stores_both_variants(cx: &mut TestAppContext) {
        let Ok(sys) = super::tests::system_theme() else {
            return;
        };
        cx.update(|cx| apply_system_theme(&sys, cx));
        cx.update(|cx| {
            let nt = cx.native_theme().expect("installed");
            assert!(nt.resolved(cx).is_some());
            GpuiTheme::change(
                if sys.mode.is_dark() {
                    GpuiThemeMode::Light
                } else {
                    GpuiThemeMode::Dark
                },
                None,
                cx,
            );
        });
        cx.update(|cx| {
            let nt = cx.native_theme().expect("installed");
            assert!(nt.resolved(cx).is_some(), "the other variant is stored too");
            // D34: the other mode's palette is the native one, not the registry default.
            let other = if sys.mode.is_dark() {
                &sys.light
            } else {
                &sys.dark
            };
            assert_eq!(
                colors::hsla_to_hex(GpuiTheme::global(cx).background),
                colors::hsla_to_hex(colors::rgba_to_hsla(other.defaults.background_color))
            );
        });
    }

    /// D34: once both variants are applied, upstream's `Theme::change` reproduces
    /// the native palette of either mode through the installed `ThemeConfig`.
    #[gpui::test]
    fn apply_installs_configs_for_both_variants(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let ((dark_theme, dark), (light_theme, light)) = preset_with_two_variants(&prefs);
        cx.update(|cx| {
            apply(dark_theme, &dark, &prefs, cx);
            apply(light_theme, &light, &prefs, cx); // light is current; the dark config must survive
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            let styled = GpuiTheme::global(cx);
            assert!(styled.is_dark());
            // Hex comparison: the config round trip is exact for 8-bit colours.
            assert_eq!(
                colors::hsla_to_hex(styled.background),
                colors::hsla_to_hex(colors::rgba_to_hsla(dark.defaults.background_color))
            );
            assert_eq!(
                colors::hsla_to_hex(styled.button_primary),
                colors::hsla_to_hex(colors::rgba_to_hsla(dark.button.primary_background)),
                "button_* fields survive the config round trip"
            );
            // D41: the config carries the mode's default highlighter style, so the
            // switch to dark also switched code highlighting.
            assert_eq!(styled.highlight_theme.appearance, GpuiThemeMode::Dark);
            assert!(cx.native_theme().and_then(|t| t.resolved(cx)).is_some());
        });
    }

    /// D35: a runtime preference change rebuilds the styled theme from the
    /// stored variant, so scaling reaches `font_size` and its config copy.
    #[gpui::test]
    fn apply_accessibility_rescales_from_the_stored_variant(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) =
            from_preset("catppuccin-mocha", true, &prefs).expect("preset should load");
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));
        let mut scaled = AccessibilityPreferences::default();
        scaled.text_scaling_factor = 1.5;
        scaled.reduce_motion = true;
        cx.update(|cx| apply_accessibility(&scaled, cx));
        cx.update(|cx| {
            let styled = GpuiTheme::global(cx);
            assert_eq!(styled.font_size, px(resolved.defaults.font.size * 1.5));
            assert_eq!(
                styled.dark_theme.font_size,
                Some(resolved.defaults.font.size * 1.5)
            );
            assert!(cx.reduce_motion());
            let nt = cx.native_theme().expect("installed");
            assert_eq!(nt.accessibility().text_scaling_factor, 1.5);
            assert!(
                nt.resolved(cx).is_some(),
                "the stored variant survives the rebuild"
            );
        });
    }

    /// A preset that states `tooltip.show_delay_ms`, a second that states
    /// another, and one that states none.
    fn tooltip_delay_presets() -> [(GpuiTheme, ResolvedTheme); 3] {
        let prefs = AccessibilityPreferences::default();
        let load = |name| from_preset(name, false, &prefs).expect("preset should load");
        let (breeze, adwaita, latte) = (
            load("kde-breeze"),
            load("adwaita"),
            load("catppuccin-latte"),
        );
        assert_eq!(breeze.1.tooltip.show_delay, Some(700), "precondition");
        assert_eq!(adwaita.1.tooltip.show_delay, Some(500), "precondition");
        assert_eq!(latte.1.tooltip.show_delay, None, "precondition");
        [breeze, adwaita, latte]
    }

    fn ms(ms: u64) -> std::time::Duration {
        std::time::Duration::from_millis(ms)
    }

    fn apply_preset(preset: &(GpuiTheme, ResolvedTheme), cx: &mut App) {
        apply(
            preset.0.clone(),
            &preset.1,
            &AccessibilityPreferences::default(),
            cx,
        );
    }

    /// Spec §7.4: the theme's delay becomes the application-wide
    /// `TooltipDefaults` show delay; the grace period stays gpui-base's.
    #[gpui::test]
    fn a_stated_tooltip_delay_is_installed(cx: &mut TestAppContext) {
        let [breeze, _, _] = tooltip_delay_presets();
        cx.update(|cx| {
            apply_preset(&breeze, cx);
            let defaults = gpui_base::TooltipDefaults::global(cx);
            assert_eq!(defaults.show_delay(), ms(700));
            assert_eq!(
                defaults.grace_period(),
                gpui_base::TooltipDefaults::new().grace_period()
            );
        });
    }

    /// Review Focus 2: a theme without the field keeps the toolkit's delay.
    #[gpui::test]
    fn a_theme_without_a_delay_keeps_the_toolkits(cx: &mut TestAppContext) {
        let [_, _, latte] = tooltip_delay_presets();
        cx.update(|cx| {
            apply_preset(&latte, cx);
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx),
                gpui_base::TooltipDefaults::new()
            );
        });
    }

    /// Review Focus 3: from a theme that states a delay to one that does not,
    /// the delay goes back to gpui-base's 500 ms where the application set
    /// none.
    #[gpui::test]
    fn a_switch_to_a_theme_without_a_delay_restores_the_default(cx: &mut TestAppContext) {
        let [breeze, adwaita, latte] = tooltip_delay_presets();
        cx.update(|cx| {
            apply_preset(&breeze, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx).show_delay(), ms(700));
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx).show_delay(), ms(500));
            apply_preset(&breeze, cx);
            apply_preset(&adwaita, cx);
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx).show_delay(), ms(500));
        });
    }

    /// Review Focus 3 and 6: the delay and grace period the application
    /// installed before the first apply. A theme's delay replaces the
    /// application's delay and keeps its grace period; a theme without one
    /// brings the application's delay back.
    #[gpui::test]
    fn the_applications_defaults_installed_first_are_kept(cx: &mut TestAppContext) {
        let [breeze, _, latte] = tooltip_delay_presets();
        let own = gpui_base::TooltipDefaults::new()
            .with_show_delay(ms(250))
            .with_grace_period(ms(80));
        cx.update(|cx| {
            own.install(cx);
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx), own);
            apply_preset(&breeze, cx);
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx),
                own.with_show_delay(ms(700))
            );
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx), own);
        });
    }

    /// Review Focus 3 and 6: a delay the application sets between two
    /// applies is its own from then on, and outlives a later theme's delay.
    #[gpui::test]
    fn a_delay_the_application_sets_between_applies_is_kept(cx: &mut TestAppContext) {
        let [breeze, adwaita, latte] = tooltip_delay_presets();
        cx.update(|cx| {
            apply_preset(&breeze, cx);
            let own = gpui_base::TooltipDefaults::global(cx)
                .with_show_delay(ms(900))
                .with_grace_period(ms(40));
            own.install(cx);
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx), own);
            apply_preset(&adwaita, cx);
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx),
                own.with_show_delay(ms(500))
            );
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx), own);
        });
    }

    /// An upstream mode switch (`Theme::change`, which ends in a rebuild of
    /// gpui-base's theme) installs the new mode's variant's delay, so a theme
    /// whose two variants state different delays does not keep the previous
    /// mode's.
    #[gpui::test]
    fn an_upstream_mode_switch_installs_that_variants_delay(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (dark_theme, dark) =
            from_preset("kde-breeze", true, &prefs).expect("preset should load");
        let (light_theme, mut light) =
            from_preset("kde-breeze", false, &prefs).expect("preset should load");
        assert_eq!(dark.tooltip.show_delay, Some(700), "precondition");
        light.tooltip.show_delay = Some(300);
        cx.update(|cx| {
            apply(dark_theme, &dark, &prefs, cx);
            apply(light_theme, &light, &prefs, cx);
            assert!(!GpuiTheme::global(cx).is_dark());
            assert_eq!(gpui_base::TooltipDefaults::global(cx).show_delay(), ms(300));
        });
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        cx.update(|cx| {
            assert!(GpuiTheme::global(cx).is_dark());
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx).show_delay(),
                ms(700),
                "the dark variant's delay after the switch"
            );
            GpuiTheme::change(GpuiThemeMode::Light, None, cx);
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx).show_delay(),
                ms(300),
                "the light variant's delay after the switch back"
            );
        });
    }

    /// An upstream mode switch to a variant that states no delay brings the
    /// application's own delay back: a variant of the same theme, and the
    /// unstored mode of a theme applied in one variant after another theme
    /// (M6-02).
    #[gpui::test]
    fn the_applications_delay_survives_a_mode_switch_to_a_variant_without_one(
        cx: &mut TestAppContext,
    ) {
        let prefs = AccessibilityPreferences::default();
        let (dark_theme, dark) =
            from_preset("kde-breeze", true, &prefs).expect("preset should load");
        let (light_theme, mut light) =
            from_preset("kde-breeze", false, &prefs).expect("preset should load");
        assert_eq!(dark.tooltip.show_delay, Some(700), "precondition");
        light.tooltip.show_delay = None;
        let [_, _, latte] = tooltip_delay_presets();
        let own = gpui_base::TooltipDefaults::new().with_show_delay(ms(250));
        cx.update(|cx| {
            own.install(cx);
            apply(dark_theme, &dark, &prefs, cx);
            apply(light_theme, &light, &prefs, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx), own);
        });
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        cx.update(|cx| {
            assert_eq!(gpui_base::TooltipDefaults::global(cx).show_delay(), ms(700));
            GpuiTheme::change(GpuiThemeMode::Light, None, cx);
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx),
                own,
                "the light variant states no delay"
            );
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            apply_preset(&latte, cx);
            assert_eq!(gpui_base::TooltipDefaults::global(cx), own);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::TooltipDefaults::global(cx),
                own,
                "Catppuccin Latte stores no dark variant, and KDE Breeze's is gone"
            );
        });
    }

    #[gpui::test]
    fn apply_accessibility_forwards_reduce_motion(cx: &mut TestAppContext) {
        cx.update(|cx| {
            apply_accessibility(&prefs(true), cx); // works without a NativeTheme
            assert!(cx.reduce_motion());
            apply_accessibility(&prefs(false), cx);
            assert!(!cx.reduce_motion());
        });
    }

    /// M6-01: `notification`, `list`, `sheet`, `motion` and `cursor_blink`
    /// are the application's settings, not theme data, so `apply` and
    /// `apply_accessibility` keep the values the application set, as
    /// upstream's own `Theme::change` does.
    #[gpui::test]
    fn apply_keeps_the_applications_own_settings(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = from_preset("nord", true, &prefs).expect("preset should load");
        assert_ne!(theme.sheet.margin_top, px(40.), "precondition");
        assert_ne!(theme.notification.max_items, 3, "precondition");
        assert!(theme.list.active_highlight, "precondition");
        assert_ne!(theme.motion.duration_normal, Duration::ZERO, "precondition");
        let steady = gpui_base::input::CursorBlink::steady();
        assert_ne!(theme.cursor_blink, steady, "precondition");
        let assert_kept = |cx: &App| {
            let styled = GpuiTheme::global(cx);
            assert_eq!(styled.sheet.margin_top, px(40.));
            assert_eq!(styled.notification.max_items, 3);
            assert!(!styled.list.active_highlight);
            assert_eq!(styled.motion.duration_normal, Duration::ZERO);
            assert_eq!(styled.cursor_blink, steady);
        };
        cx.update(|cx| {
            gpui_component::init(cx);
            let styled = GpuiTheme::global_mut(cx);
            styled.sheet.margin_top = px(40.);
            styled.notification.max_items = 3;
            styled.list.active_highlight = false;
            styled.motion.duration_normal = Duration::ZERO;
            styled.cursor_blink = steady;
            apply(theme, &resolved, &prefs, cx);
            assert_kept(cx);
            let mut scaled = AccessibilityPreferences::default();
            scaled.text_scaling_factor = 1.5;
            apply_accessibility(&scaled, cx);
            assert_kept(cx);
        });
    }

    /// M6-02: a one-variant `apply` of another theme drops the variant an
    /// earlier theme stored for the other mode, so a `Theme::change` to that
    /// mode shows upstream's built-in palette, not the earlier theme's. The
    /// unstored config is `ThemeConfig::default()`, a light one: a change to
    /// an unstored dark mode keeps the window light (gpui-component
    /// src/theme/schema.rs:1108), as the `apply` doc says.
    #[gpui::test]
    fn a_one_variant_apply_of_another_theme_drops_the_earlier_themes_other_mode(
        cx: &mut TestAppContext,
    ) {
        let prefs = AccessibilityPreferences::default();
        let (breeze_theme, breeze) =
            from_preset("kde-breeze", false, &prefs).expect("preset should load");
        let (mocha_theme, mocha) =
            from_preset("catppuccin-mocha", true, &prefs).expect("preset should load");
        assert!(
            !breeze_theme.is_dark() && mocha_theme.is_dark(),
            "precondition"
        );
        assert_ne!(
            breeze_theme.theme_name(),
            mocha_theme.theme_name(),
            "precondition"
        );
        let upstream_light =
            colors::hsla_to_hex(gpui_component::theme::ThemeColor::light().background);
        for earlier in [&breeze, &mocha] {
            assert_ne!(
                colors::hsla_to_hex(colors::rgba_to_hsla(earlier.defaults.background_color)),
                upstream_light,
                "precondition"
            );
        }
        cx.update(|cx| {
            apply(breeze_theme.clone(), &breeze, &prefs, cx);
            apply(mocha_theme, &mocha, &prefs, cx);
            GpuiTheme::change(GpuiThemeMode::Light, None, cx);
        });
        cx.update(|cx| {
            let styled = GpuiTheme::global(cx);
            assert!(!styled.is_dark());
            assert_eq!(
                colors::hsla_to_hex(styled.background),
                upstream_light,
                "upstream's light palette, not KDE Breeze's"
            );
            assert!(
                cx.native().is_none(),
                "no light variant of Catppuccin Mocha"
            );
        });
        cx.update(|cx| {
            apply(breeze_theme, &breeze, &prefs, cx);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            let styled = GpuiTheme::global(cx);
            assert!(!styled.is_dark(), "the unstored dark config is a light one");
            assert_eq!(
                colors::hsla_to_hex(styled.background),
                upstream_light,
                "upstream's light palette, not Catppuccin Mocha's"
            );
            assert_eq!(
                styled.theme_name().as_ref(),
                "",
                "the empty config replaced KDE Breeze's light one"
            );
            assert!(
                cx.native().is_some(),
                "the stored light variant is the current mode's"
            );
            GpuiTheme::change(GpuiThemeMode::Light, None, cx);
        });
        cx.update(|cx| {
            assert_eq!(
                colors::hsla_to_hex(GpuiTheme::global(cx).background),
                upstream_light,
                "a change back to light does not bring KDE Breeze back"
            );
        });
    }

    /// After a change to an unstored dark mode the styled theme's name is the
    /// empty config's, while the stored light variant keeps its theme's name:
    /// `apply_accessibility` rebuilds from that variant under its own name, so
    /// the stored variants survive and the theme comes back.
    #[gpui::test]
    fn apply_accessibility_rebuilds_under_the_stored_name(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, breeze) = from_preset("kde-breeze", false, &prefs).expect("preset should load");
        let name = theme.theme_name().clone();
        assert!(!name.is_empty(), "precondition");
        cx.update(|cx| {
            apply(theme, &breeze, &prefs, cx);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            assert_eq!(
                GpuiTheme::global(cx).theme_name().as_ref(),
                "",
                "precondition"
            );
            let mut scaled = AccessibilityPreferences::default();
            scaled.text_scaling_factor = 1.5;
            apply_accessibility(&scaled, cx);
        });
        cx.update(|cx| {
            assert!(cx.native().is_some(), "the stored light variant survives");
            let styled = GpuiTheme::global(cx);
            assert_eq!(styled.theme_name(), &name);
            assert_eq!(styled.font_size, px(breeze.defaults.font.size * 1.5));
            assert_eq!(
                colors::hsla_to_hex(styled.background),
                colors::hsla_to_hex(colors::rgba_to_hsla(breeze.defaults.background_color))
            );
        });
    }

    #[test]
    fn native_unscaled_has_factor_one() {
        let (_, resolved) = from_preset(
            "catppuccin-mocha",
            true,
            &AccessibilityPreferences::default(),
        )
        .unwrap();
        let n = Native::unscaled(&resolved);
        assert_eq!(n.accessibility.text_scale_factor(), 1.0);
        assert!(!n.accessibility.reduce_motion);
    }

    #[test]
    fn base_overrides_fallback_takes_geometry_from_the_other_variant_and_colours_from_styled() {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = from_preset("catppuccin-mocha", true, &prefs).unwrap();
        let nt = NativeTheme {
            dark: Some(StoredVariant {
                name: theme.theme_name().clone(),
                resolved: resolved.clone(),
            }),
            ..NativeTheme::default()
        };
        let (g, r) =
            base_overrides_for(&nt, false, Some(&theme)).expect("falls back to the dark variant");
        let dark = base_layer::scrollbar_geometry(&resolved);
        assert_eq!(g.track_width, dark.track_width);
        assert_eq!(g.thumb_inset, dark.thumb_inset);
        assert_eq!(g.track, theme.scrollbar);
        assert_eq!(g.thumb_active, theme.scrollbar_thumb_hover);
        assert_eq!(r.handle, Some(theme.border));
        assert!(base_overrides_for(&NativeTheme::default(), false, Some(&theme)).is_none());
    }
}
