//! egui toolkit connector for native-theme.
//!
//! Target: **egui 0.36.2**.
//!
//! `native-theme-egui` gives an egui application an excellent global theme out
//! of the box and opt-in per-widget geometry: egui 0.36.2's `Style` has an
//! interaction-state axis and provably no widget-type axis, so what one global
//! `Style` can hold reaches every widget automatically, and the per-widget
//! values reach the screen only where the application asks for them.

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![allow(
    dead_code,
    reason = "the library is assembled over Tasks 8-32; Task 32 removes this line"
)]
#![allow(
    rustdoc::broken_intra_doc_links,
    reason = "link targets land by Task 26; Task 32 removes this line"
)]

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
