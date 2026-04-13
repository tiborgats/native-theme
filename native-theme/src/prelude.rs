//! Convenience re-exports for common usage.
//!
//! # Usage
//!
//! ```
//! use native_theme::prelude::*;
//! ```
//!
//! This imports the seven most-used items:
//! [`Theme`], [`ResolvedTheme`], [`SystemTheme`],
//! [`AccessibilityPreferences`],
//! [`Rgba`], [`Error`], and [`Result`].

pub use crate::AccessibilityPreferences;
pub use crate::Result;
pub use crate::SystemTheme;
pub use crate::color::Rgba;
pub use crate::error::Error;
pub use crate::theme::{ResolvedTheme, Theme};
