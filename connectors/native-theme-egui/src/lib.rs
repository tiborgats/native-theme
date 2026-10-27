//! egui toolkit connector for native-theme.
//!
//! Target: **egui 0.36.2**.
//!
//! `native-theme-egui` themes an egui application from a [`native_theme`] theme: the OS's
//! own, read at run time, or one of native-theme's bundled presets. One call,
//! [`ThemeAtlas::install`], gives every widget the theme's colours, strokes, radii, spacing
//! and text sizes with no further code. The values that belong to one kind of widget — an
//! input's border, a checkbox's checked fill, a menu's items — reach the widgets the
//! application puts in that widget's role scope ([`NativeThemeUiExt::native_scope`]):
//! egui's `Style` holds one set of widget visuals per interaction state (inactive, hovered,
//! active, …), shared by every kind of widget, so one global `Style` cannot hold them all.
//!
//! The companion crate `native-theme-egui-widgets` draws, from the same installed atlas,
//! the widgets no `Style` can make look native: a switch, a segmented control, and a
//! slider, spinner, links, drop-down, radio button, progress bar and expander drawn as the
//! theme states them.
//!
//! # Spec citations
//!
//! A `spec §N` in these docs cites section N of the connector's design specification,
//! [`docs/archive/todo_v0.6.0_egui-connector-spec.md`](https://github.com/tiborgats/native-theme/blob/main/docs/archive/todo_v0.6.0_egui-connector-spec.md)
//! in the repository; the published crate does not include it. The mapping document below
//! cites the same document by bare section numbers, as its preamble says.
//!
//! # Quick start
//!
//! With feature `native`, the OS readers ([`from_system`] says what it returns without them):
//!
//! ```rust,no_run
//! use native_theme_egui::egui;
//! use native_theme_egui::from_system;
//!
//! # fn install(ctx: &egui::Context) -> native_theme_egui::Result<()> {
//! let loaded = from_system()?;
//! loaded.atlas.install(ctx);
//! ctx.set_theme(egui::ThemePreference::System);
//! # Ok(())
//! # }
//! ```
//!
//! Or from a bundled preset. The atlas holds both of the preset's variants; this recipe pins
//! the scheme `is_dark` named. With `ThemePreference::System` instead, the preset follows the
//! OS light/dark preference, as [`from_system`]'s atlas does ([`from_preset`] says how):
//!
//! ```rust,no_run
//! use native_theme_egui::egui::{self, ThemePreference};
//! use native_theme_egui::{from_preset, AccessibilityPreferences};
//!
//! # fn install(ctx: &egui::Context) -> native_theme_egui::Result<()> {
//! let is_dark = true;
//! let (atlas, _resolved) = from_preset("catppuccin-mocha", is_dark, &AccessibilityPreferences::default())?;
//! atlas.install(ctx);
//! ctx.set_theme(if is_dark { ThemePreference::Dark } else { ThemePreference::Light });
//! # Ok(())
//! # }
//! ```
//!
//! # Window background
//!
//! eframe paints `App::clear_color` behind every panel: its root `Ui` has no margin or
//! background colour (eframe `src/epi.rs:173`), and the default clear colour is a translucent
//! near-black, `rgba(12, 12, 12, 180)` (`:248-252`). A panel fill the theme states translucent
//! or transparent — `windows-11`'s sidebar, WinUI's docked `NavigationView` pane over the
//! window backdrop — lies over that colour, so an eframe application returns the base style's
//! `panel_fill`, the theme's `defaults.background_color`, from `App::clear_color`:
//!
//! ```rust,no_run
//! use native_theme_egui::egui;
//!
//! // The body of `eframe::App::clear_color(&self, visuals)`.
//! fn clear_color(visuals: &egui::Visuals) -> [f32; 4] {
//!     visuals.panel_fill.to_normalized_gamma_f32()
//! }
//! ```
//!
//! # Where the theme reaches
//!
//! [`ThemeAtlas::install`] publishes the base style into both `egui::Theme`s; a role's
//! style reaches a widget through [`NativeThemeUiExt::native_scope`],
//! [`NativeThemeUiExt::native_set_style`] or [`ThemeAtlas::role_modifier`] (from a `Ui`,
//! [`role_modifier`]), and a container's frame through [`NativeThemeUiExt::native_frame`] or
//! [`ThemeAtlas::surface_frame`]. The values no `Style` field carries, and some the
//! `Style` also holds, are the free accessors of this crate root, for the application's own
//! use; fonts are [`fonts`]' and icons [`icons`]'.
//!
//! The base style alone carries, among others: the panel background (`visuals.panel_fill`)
//! and the menu background (`visuals.window_fill`); the text colours; the button's fills,
//! borders, corner radius, padding and minimum height, which every interactive widget
//! shares, since egui's `widgets` states belong to no one kind of widget; the selection,
//! link, warning and error colours; the window stroke, the window and menu corners and the
//! window and popup shadows; a text field's fill and caret colour; the scroll bar; the widget
//! gap (`spacing.item_spacing`); the slider rail's height; the check box's size; the combo
//! box's width; the tooltip's delay and width; and the body, monospace, small and heading
//! text sizes. The mapping document below lists every native leaf with the egui field it
//! reaches, or why it reaches none.
//!
//! An accessor that returns one `ResolvedTheme` leaf, converted to egui's type
//! or scaled by the text-scaling factor, is named after the leaf's path,
//! without the leading `defaults.` and a unit suffix:
//! `defaults.border.corner_radius` is `border_corner_radius`,
//! `scrollbar.groove_width` is `scrollbar_groove_width`, the same name in the
//! iced, gpui and egui connectors (`scripts/check_connector_parity.py` checks
//! it).
//!
//! # What each flag does
//!
//! The four [`AccessibilityPreferences`] flags reach the atlas through
//! [`Builder::accessibility`] and [`from_preset`]; [`from_system`] uses the OS reading.
//!
//! | Flag | The connector applies | The application forwards |
//! |------|-----------------------|--------------------------|
//! | `text_scaling_factor` | every text size in the styles, and the font accessors' sizes, times [`AccessibilityPreferences::text_scale_factor`]; no other length | a size it reads from the `ResolvedTheme` itself, through [`scaled_text_size`] |
//! | `reduce_motion` | `Style::animation_time = 0.0` and `ScrollAnimation::none()` in every style | animations of its own |
//! | `reduce_transparency` | nothing: egui's `Modal` backdrop is a per-call `Modal::backdrop_color`, not a `Style` field; [`is_reduced_transparency`] reads it | the backdrop colour and its own translucent surfaces |
//! | `high_contrast` | nothing, by design: high contrast belongs in the theme data, which a reader fills with what the platform draws under it (the Windows reader writes the high-contrast system colours, the GNOME reader libadwaita's high-contrast focus ring, and the macOS reader's system colours follow Increase Contrast, `docs/evidence/os-probe/2026-10-26-run-40320895749/os-probe-macos-14/increase-contrast-on/probe.toml`), so the theme carries it already; [`is_high_contrast`] reads the flag | its own colours, if they should change |
//!
//! # Feature flags
//!
//! Each forwards to the `native-theme` feature of the same name.
//!
//! | Feature | Default | Effect |
//! |---------|---------|--------|
//! | `material-icons` | on | the bundled Material icon set |
//! | `lucide-icons` | on | the bundled Lucide icon set |
//! | `system-icons` | on | the system icon lookup: freedesktop icon themes, SF Symbols and Windows' Segoe Fluent glyphs (native-theme's feature also turns `material-icons` on) |
//! | `system-fonts` | on | `fonts::FontPlan::from_system`, and the OS typefaces in [`from_preset`], [`from_system`] and [`SystemThemeExt::to_egui_atlas`] |
//! | `svg-rasterize` | off | native-theme's in-process SVG rasteriser (`native_theme::rasterize::rasterize_svg`), whose `IconData::Rgba` output [`icons::to_color_image`] converts |
//! | `native` | off | the OS readers; without one, [`from_system`] is the platform preset on Linux and an error on macOS and Windows |
//! | `watch` | off | `ThemeWatcher`, which follows OS theme changes; it needs a reader feature (`native`) too |
//!
//! The minimum supported Rust version is 1.95, egui 0.36.2's own.
//!
//! # Semver
//!
//! 1. The crate version equals the workspace version.
//! 2. One egui minor per release line of this crate; an egui minor bump is a
//!    **breaking change** for this crate.
//! 3. **The numeric contents of a produced `egui::Style` are not covered.** A
//!    mapping fix — electing a different base owner, correcting a sink, closing a
//!    `Note` — is never a breaking change.
//! 4. Every public enum is `#[non_exhaustive]` except `PanelSide`, so a new `Role`,
//!    `Surface`, `RoleVariant`, `Note`, `TextRole`, `IconContext` or `FontBytes`
//!    variant is additive; each of `Note`'s struct variants carries its own
//!    `#[non_exhaustive]`, so enriching one stays additive too.
//! 5. Adding a free accessor is additive.
//! 6. `ThemeAtlas`, `Builder`, `FontPlan`, `IconKey` and — behind feature `watch` —
//!    `ThemeWatcher` are opaque, and `NativeThemeUiExt` and `SystemThemeExt` are
//!    sealed, so adding a method to any of them is additive.
//!
//! # Limits
//!
//! Every native leaf this crate cannot reach is a `mapping.toml` row marked `unmappable`;
//! the mapping document below lists them all.

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![doc = include_str!("mapping.md")]

mod accessors;
mod atlas;
#[cfg(test)]
mod compat;
#[cfg(test)]
mod contract;
pub mod convert;
mod ext;
pub mod fonts;
// The freedesktop icon-theme fixture the tests run in a child process.
#[cfg(all(test, target_os = "linux", feature = "system-icons"))]
use native_theme_test_fixtures as icon_fixture;
pub mod icons;
#[cfg(test)]
mod install_tests;
#[cfg(test)]
mod mapping_doc;
#[cfg(test)]
mod mapping_tests;
mod plugin;
mod roles;
mod style;
#[cfg(test)]
mod style_diff;
#[cfg(feature = "watch")]
mod watch;

pub use accessors::{
    border_color, border_corner_radius, border_corner_radius_lg, dialog_button_order,
    disabled_opacity, disabled_text_color, expander_icon, focus_ring_color, focus_ring_offset,
    focus_ring_width, font_family, font_size, font_weight, info_color, info_text_color,
    input_frame, input_margin, is_high_contrast, is_reduced_motion, is_reduced_transparency,
    line_height, list_header_font, list_header_height, mono_font_family, mono_font_size,
    mono_font_weight, role_font_is_italic, role_font_weight, scaled_text_size,
    scrollbar_groove_width, selection_inactive_background, text_area_frame, text_area_margin,
    text_role_font, text_role_line_height, text_role_weight, text_scaling_factor,
    tooltip_show_delay, warning_text_color, window_title_bar_font, window_title_bar_text_color,
};
pub use atlas::Note;
pub use atlas::{Builder, ThemeAtlas};
pub use ext::{Loaded, SystemThemeExt, from_preset, from_system, to_theme};
pub use ext::{NativeThemeUiExt, role_modifier};
pub use plugin::register_focus_shape;
pub use roles::{PanelSide, Role, RoleVariant, Surface, TextRole};
#[cfg(feature = "watch")]
pub use watch::ThemeWatcher;
#[cfg(test)]
mod tripwires;

// ---- toolkit and source-crate re-exports -----------------------------------
// Both are re-exported so a downstream `Cargo.toml` need not add a second
// dependency on either crate, whose version could drift from the connector's.
// Two `egui` versions in one graph produce "expected `egui::Style`, found
// `egui::Style`", which is the single most common downstream failure with a
// toolkit connector.
pub use egui;
pub use native_theme;

// ---- convenience re-exports ------------------------------------------------
// RULE: the crate root re-exports no name that also exists at `egui`'s root.
// Four native-theme names are therefore deliberately NOT here:
//   * `native_theme::color::Rgba`     -> `convert::Rgba`   (egui/src/lib.rs:442, opposite colour space)
//   * `native_theme::icons::IconData` -> `icons::IconData` (egui/src/viewport.rs:183)
//   * `native_theme::theme::Theme`    -> reachable as `native_theme::theme::Theme`
//                                        (egui/src/lib.rs:483 already exports `Theme`)
//   * `native_theme::SystemTheme`     -> reachable as `native_theme::SystemTheme`
//                                        (egui/src/viewport.rs:1036, re-exported to egui's
//                                         root by `viewport::*` at egui/src/lib.rs:493;
//                                         it resolves as `crate::SystemTheme` inside egui at
//                                         context.rs:252 and :2479)
pub use native_theme::error::Error;
pub use native_theme::icons::{AnimatedIcon, IconProvider, IconRole, TransformAnimation};
pub use native_theme::theme::{
    ColorMode, DialogButtonOrder, FontStyle, IconSet, LayoutTheme, ResolvedTheme, ThemeMode,
};
pub use native_theme::{AccessibilityPreferences, Result};

#[cfg(target_os = "linux")]
pub use native_theme::detect::LinuxDesktop;

/// The README's code blocks, compiled as doctests so they cannot drift from the API; the
/// runtime-theme-change block needs feature `watch`, and the fonts block `system-fonts`.
#[cfg(all(doctest, feature = "watch", feature = "system-fonts"))]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;
