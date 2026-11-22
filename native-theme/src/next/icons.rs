//! The icon set, icon theme and icon sizes of a resolved theme.
//!
//! The spec's `icons.rs` is 0.7.2's file until the 0.8.0 cut; this module
//! holds its two new types, and takes [`IconSet`] from 0.7.2.

use std::sync::Arc;

use crate::IconSet;

/// The icons of a resolved theme.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct IconThemeSpec {
    /// The icon set.
    pub set: IconSet,
    /// The freedesktop icon theme, matched to the scheme (e.g. `breeze` or
    /// `breeze-dark`).
    pub theme: Option<Arc<str>>, // scheme-matched freedesktop theme (e.g., "breeze" vs "breeze-dark")
    /// Whether `theme` was stated by a preset, an override or a patch
    /// (`true`), or detected at run time (`false`); 0.7.2's
    /// `icon_theme_explicit`.
    pub theme_explicit: bool, // true when stated by preset/override/patch, false when runtime-detected (0.7.2's `icon_theme_explicit`)
    /// The icon sizes.
    pub sizes: IconSizes,
}

/// The icon sizes, in logical pixels; each `None` where unstated, as in
/// 0.7.2.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct IconSizes {
    /// Toolbar buttons' icons.
    pub toolbar: Option<f32>,
    /// Small icons for inline use.
    pub small: Option<f32>,
    /// Large icons for menus and lists.
    pub large: Option<f32>,
    /// Dialog buttons' icons.
    pub dialog: Option<f32>,
    /// Panel headers' icons.
    pub panel: Option<f32>,
}

impl IconSizes {
    /// Icon sizes of these fields.
    #[must_use]
    pub fn new(
        toolbar: Option<f32>,
        small: Option<f32>,
        large: Option<f32>,
        dialog: Option<f32>,
        panel: Option<f32>,
    ) -> Self {
        Self {
            toolbar,
            small,
            large,
            dialog,
            panel,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_sizes_constructor() {
        let s = IconSizes::new(Some(22.0), Some(16.0), None, Some(32.0), None);
        assert_eq!(s.toolbar, Some(22.0));
        assert_eq!(s.small, Some(16.0));
        assert_eq!(s.large, None);
        assert_eq!(s.dialog, Some(32.0));
        assert_eq!(s.panel, None);
    }
}
