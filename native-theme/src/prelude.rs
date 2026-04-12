//! Convenience re-exports for common usage.
//!
//! # Usage
//!
//! ```
//! use native_theme::prelude::*;
//! ```
//!
//! This imports the six most-used items:
//! [`Theme`], [`ResolvedTheme`], [`SystemTheme`],
//! [`Rgba`], [`Error`], and [`Result`].

pub use crate::color::Rgba;
pub use crate::error::Error;
pub use crate::theme::{ResolvedTheme, Theme};
pub use crate::Result;
pub use crate::SystemTheme;
