//! egui toolkit connector for native-theme.
//!
//! Target: **egui 0.36.2**.
//!
//! `native-theme-egui` gives an egui application an excellent global theme out
//! of the box and opt-in per-widget geometry: egui 0.36.2's `Style` has an
//! interaction-state axis and provably no widget-type axis, so what one global
//! `Style` can hold reaches every widget automatically, and the per-widget
//! values reach the screen only where the application asks for them.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use native_theme_egui::from_system;
//!
//! # fn install(cc: &eframe::CreationContext<'_>) -> native_theme_egui::Result<()> {
//! let (atlas, _resolved, _is_dark) = from_system()?;
//! atlas.install(&cc.egui_ctx);
//! cc.egui_ctx.set_theme(egui::ThemePreference::System);
//! # Ok(())
//! # }
//! ```
//!
//! Or from a bundled preset:
//!
//! ```rust,no_run
//! use native_theme_egui::{from_preset, AccessibilityPreferences};
//!
//! # fn install(ctx: &egui::Context) -> native_theme_egui::Result<()> {
//! let (atlas, _resolved) = from_preset("catppuccin-mocha", true, &AccessibilityPreferences::default())?;
//! atlas.install(ctx);
//! # Ok(())
//! # }
//! ```
//!
//! # Where the theme reaches
//!
//! [`ThemeAtlas::install`] publishes the base style into both `egui::Theme`s; a role's
//! style reaches a widget through [`NativeThemeUiExt::native_scope`],
//! [`NativeThemeUiExt::native_set_style`] or [`ThemeAtlas::role_modifier`], and a
//! container's frame through [`NativeThemeUiExt::native_frame`] or
//! [`ThemeAtlas::surface_frame`]. The values no `Style` field carries are the free
//! accessors of this crate root; fonts are [`fonts`]' and icons [`icons`]'.
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
mod contract;
pub mod convert;
mod ext;
pub mod fonts;
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
    border_color, border_radius, border_radius_lg, dialog_button_order, disabled_opacity,
    disabled_text_color, expander_icon, focus_ring_color, focus_ring_offset, focus_ring_width,
    font_family, font_size, font_weight, info_color, info_text_color, input_frame, input_margin,
    is_high_contrast, is_reduced_motion, is_reduced_transparency, line_height_multiplier,
    list_header_font, mono_font_family, mono_font_size, mono_font_weight, role_font_is_italic,
    role_font_weight, scaled_text_size, scrollbar_width, selection_inactive_background,
    text_role_font, text_role_line_height, text_role_weight, text_scaling_factor,
    warning_text_color, window_title_bar_font, window_title_bar_text_color,
};
pub use atlas::Note;
pub use atlas::{Builder, ThemeAtlas};
pub use ext::NativeThemeUiExt;
pub use ext::{SystemThemeExt, from_preset, from_system, to_theme};
pub use plugin::register_focus_shape;
pub use roles::{PanelSide, Role, RoleVariant, Surface, TextRole};
#[cfg(feature = "watch")]
pub use watch::ThemeWatcher;
#[cfg(test)]
mod tripwires;

// ---- toolkit and source-crate re-exports -----------------------------------
// Both are re-exported so a downstream `Cargo.toml` cannot introduce a second
// copy of either crate. Two `egui` versions in one graph produce
// "expected `egui::Style`, found `egui::Style`", which is the single most
// common downstream failure with a toolkit connector.
pub use egui;
pub use native_theme;

// ---- convenience re-exports ------------------------------------------------
// RULE: the crate root re-exports no name that also exists at `egui`'s root.
// Four native-theme names are therefore deliberately NOT here:
//   * `native_theme::color::Rgba`     -> `convert::Rgba`   (egui/src/lib.rs:442, opposite colour space)
//   * `native_theme::theme::IconData` -> `icons::IconData` (egui/src/viewport.rs:183)
//   * `native_theme::theme::Theme`    -> reachable as `native_theme::theme::Theme`
//                                        (egui/src/lib.rs:483 already exports `Theme`)
//   * `native_theme::SystemTheme`     -> reachable as `native_theme::SystemTheme`
//                                        (egui/src/viewport.rs:1036, re-exported to egui's
//                                         root by `viewport::*` at egui/src/lib.rs:493;
//                                         it resolves as `crate::SystemTheme` inside egui at
//                                         context.rs:252 and :2479)
pub use native_theme::error::Error;
pub use native_theme::theme::{
    AnimatedIcon, ColorMode, DialogButtonOrder, FontStyle, IconProvider, IconRole, IconSet,
    LayoutTheme, ResolvedTheme, ThemeMode, TransformAnimation,
};
pub use native_theme::{AccessibilityPreferences, Result};

#[cfg(target_os = "linux")]
pub use native_theme::detect::LinuxDesktop;
