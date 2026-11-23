//! The gpui-base values of [`crate::base_layer`], from the 0.8.0 model: one
//! function of the same name for each that reads a resolved theme, taking
//! the 0.8.0 [`ResolvedTheme`], and
//! returning the same value, or the [`Unmapped`](super::Unmapped) source it
//! lacks. [`crate::base_layer::scrollbar_styles`] and
//! [`crate::base_layer::apply_overrides`] take these values, from either
//! model.

use gpui_base::ResizableTheme;
use native_theme::next::ResolvedTheme;

use super::Mapped;
use crate::base_layer::ScrollbarGeometry;

/// [`crate::base_layer::scrollbar_geometry`], from the 0.8.0 model: the
/// vertical track's and thumb's widths, the layout's corner radius, the
/// thumb's minimum length, the track's fill, the window's border colour,
/// and the thumb's fill at rest, hovered and pressed.
pub fn scrollbar_geometry(r: &ResolvedTheme) -> Mapped<ScrollbarGeometry> {
    super::scrollbar_geometry(r)
}

/// [`crate::base_layer::resizable_theme`], from the 0.8.0 model: the
/// splitter's handle fill at rest and hovered.
pub fn resizable_theme(r: &ResolvedTheme) -> Mapped<ResizableTheme> {
    super::resizable_theme(r)
}

#[cfg(test)]
mod tests {
    use native_theme::next::{Preset, Theme, ThemeContext};

    use super::*;

    /// Every public function returns what the internal mapping, which the
    /// parity test compares with 0.7.2's, returns.
    #[test]
    fn every_value_is_the_internal_mappings() {
        for ctx in [ThemeContext::light(), ThemeContext::dark()] {
            let r = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
            assert_eq!(scrollbar_geometry(&r), super::super::scrollbar_geometry(&r));
            let handles = |t: Mapped<ResizableTheme>| t.map(|t| (t.handle, t.active_handle));
            assert_eq!(
                handles(resizable_theme(&r)),
                handles(super::super::resizable_theme(&r))
            );
        }
    }
}
