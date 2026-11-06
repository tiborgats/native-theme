//! # native-theme
//!
//! Cross-platform native theme detection and loading for Rust GUI applications.
//!
//! The crate is between two models: today's theme model, its presets and its
//! platform readers are removed, and the 0.8.0 model that replaces them is
//! not yet in place. What stays is the detection, the icons and the fonts.
//!
//! # Feature flags
//!
//! Each OS-specific dependency is compiled only on its own target, so
//! `native` on macOS pulls in only the macOS dependencies.
//!
//! | Feature | Enables |
//! |---|---|
//! | `kde` | The KDE icon theme, read from the `kdeglobals` files as KConfig cascades them (Linux) |
//! | `macos`, `windows` | The running OS version and its generation, in [`pipeline::diagnose_platform_support`] |
//! | `portal`, `watch` | Nothing until the platform readers return |
//! | `linux-kde`, `linux-portal` | Aliases of `kde` and `portal` |
//! | `linux` | `linux-kde` and `linux-portal` |
//! | `native` | `linux`, `macos` and `windows` |
//! | `material-icons`, `lucide-icons` | The bundled Material Symbols and Lucide icon sets |
//! | `system-icons` | The platform icon sets: freedesktop icon themes (Linux), SF Symbols (macOS), Segoe Fluent Icons and stock icons (Windows); also enables `material-icons` |
//! | `svg-rasterize` | `rasterize::rasterize_svg`, SVG icons to RGBA through resvg |
//! | `system-fonts` | `fonts::system_face`, the platform's own typeface for a family; on macOS the system UI font's file through Core Text |
//!
//! The minimum supported Rust version is 1.88.0.
//!
//! # Modules
//!
//! - The crate root: [`Error`] and [`Result`], and from [`pipeline`]
//!   [`VersionMatch`] and [`DiagnosticEntry`].
//! - [`detect`]: OS detection: the Linux desktop environment, and the
//!   detection caches.
//! - [`icons`]: the icon roles and sets, the per-set icon loaders, the icon
//!   data types and the icon-set choice.
//! - [`fonts`]: font face matching; the platform's own typeface with the
//!   `system-fonts` feature.
//! - [`pipeline`]: a diagnosis of the platform's theme support.
//! - [`error`]: the error type.
//! - `rasterize` (the `svg-rasterize` feature): SVG icons to RGBA.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// sRGB pixel utilities for the icon loaders.
pub(crate) mod color;
pub mod detect;
/// Error types.
pub mod error;
pub mod fonts;
/// Icon loading.
///
/// - The semantic roles, [`IconRole`], the icon sets, [`IconSet`], and the
///   icon data, [`IconData`](icons::IconData).
/// - A typed loader per icon set: [`MaterialLoader`](icons::MaterialLoader)
///   and [`LucideLoader`](icons::LucideLoader) for the bundled sets,
///   [`FreedesktopLoader`](icons::FreedesktopLoader) (Linux),
///   [`SfSymbolsLoader`](icons::SfSymbolsLoader) (macOS) and
///   [`SegoeFluentLoader`](icons::SegoeFluentLoader) (Windows) for the
///   platform sets. Each needs its set's feature, `material-icons`,
///   `lucide-icons` or `system-icons`, and gives `None` without it.
/// - [`load_icon`](icons::load_icon), which loads from whichever set it is
///   given.
/// - The icon-set choice of a settings UI,
///   [`IconSetChoice`](icons::IconSetChoice).
pub mod icons;
/// The KDE icon theme, read from `kdeglobals`.
#[cfg(all(target_os = "linux", feature = "kde"))]
pub(crate) mod kde;
#[cfg_attr(not(nt_next_docs), doc(hidden))]
pub mod next;
pub mod pipeline;

/// Freedesktop icon theme lookup (Linux).
///
/// Requires the `system-icons` feature, and Linux.
#[cfg(all(target_os = "linux", feature = "system-icons"))]
pub(crate) mod freedesktop;
/// macOS platform helpers.
#[cfg(target_os = "macos")]
pub(crate) mod macos;
/// SVG-to-RGBA rasterization utilities.
///
/// Requires the `svg-rasterize` feature.
#[cfg(feature = "svg-rasterize")]
pub mod rasterize;
/// SF Symbols icon loader (macOS).
///
/// Requires the `system-icons` feature, and macOS.
#[cfg(all(target_os = "macos", feature = "system-icons"))]
pub(crate) mod sficons;
/// The running Windows version.
///
/// Requires the `windows` feature, and Windows.
#[cfg(all(target_os = "windows", feature = "windows"))]
pub(crate) mod windows;
/// Windows Segoe Fluent / stock icon loader.
///
/// Requires the `system-icons` feature, and Windows.
#[cfg(all(target_os = "windows", feature = "system-icons"))]
pub(crate) mod winicons;
#[cfg(all(not(target_os = "windows"), feature = "system-icons"))]
#[allow(dead_code, unused_imports)]
pub(crate) mod winicons;

/// Convenience Result type alias for this crate.
pub type Result<T> = std::result::Result<T, error::Error>;

// Internal re-exports: keep crate::Type paths working for internal modules
// without exposing them in the public API. External users access the icon
// types via native_theme::icons::*.
pub use error::Error;
pub(crate) use icons::{IconRole, IconSet};
pub use pipeline::{DiagnosticEntry, VersionMatch};
