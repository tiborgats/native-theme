//! Convenience re-exports for common usage.
//!
//! # Usage
//!
//! ```
//! use native_theme::prelude::*;
//! ```
//!
//! This imports the eight most-used items:
//! [`Theme`], [`ResolvedTheme`], [`SystemTheme`],
//! [`AccessibilityPreferences`], [`ResolutionContext`],
//! [`Rgba`], [`Error`], and [`Result`].

pub use crate::AccessibilityPreferences;
pub use crate::ResolutionContext;
pub use crate::Result;
pub use crate::SystemTheme;
pub use crate::color::Rgba;
pub use crate::error::Error;
pub use crate::theme::{ResolvedTheme, Theme};
