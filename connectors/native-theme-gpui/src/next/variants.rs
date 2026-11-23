//! The button variants and the switch colour of [`crate::variants`] that
//! read the native theme, from the 0.8.0 model: one function of the same
//! name for each, taking the 0.8.0
//! [`ResolvedTheme`] in place of
//! [`Native`](crate::Native) or of the installed native theme, and
//! returning the same value, or the [`Unmapped`](super::Unmapped) source it
//! lacks.
//!
//! [`crate::variants::ghost_button`] reads only gpui-component's installed
//! theme, which both models fill, so it has no counterpart here. Each
//! variant below takes, as its 0.7.2 counterpart does, its other colours from
//! that installed theme.

use gpui::{App, Hsla};
use gpui_component::button::ButtonCustomVariant;
use native_theme::next::ResolvedTheme;

use super::{Mapped, builders};
use crate::variants::{bordered_button_from, default_button_from, toolbar_button_from};

/// [`crate::variants::switch_color`], from the 0.8.0 model: the checked
/// switch's track fill.
pub fn switch_color(r: &ResolvedTheme) -> Mapped<Hsla> {
    builders::switch_color(r)
}

/// [`crate::variants::bordered_button`], from the 0.8.0 model: `None` where
/// no rule of the theme states a hovered button's stroke (its `from_state`
/// bit, spec §2).
pub fn bordered_button(r: &ResolvedTheme, cx: &App) -> Mapped<Option<ButtonCustomVariant>> {
    Ok(bordered_button_from(builders::hover_border(r)?, cx))
}

/// [`crate::variants::toolbar_button`], from the 0.8.0 model: the toolbar
/// button states `r` states, over [`crate::variants::ghost_button`].
pub fn toolbar_button(r: &ResolvedTheme, cx: &App) -> Mapped<ButtonCustomVariant> {
    Ok(toolbar_button_from(
        Some(builders::toolbar_button_states(r)?),
        cx,
    ))
}

/// [`crate::variants::default_button`], from the 0.8.0 model: `None` where
/// no rule of the theme states the default button's fill.
pub fn default_button(r: &ResolvedTheme, cx: &App) -> Mapped<Option<ButtonCustomVariant>> {
    Ok(default_button_from(builders::default_button_states(r)?, cx))
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use native_theme::next::{Preset, Theme, ThemeContext};

    use super::*;

    /// Every public function returns what the internal mapping, which the
    /// parity test compares with 0.7.2's, gives the variant.
    #[gpui::test]
    fn every_variant_is_built_from_the_internal_mappings_values(cx: &mut TestAppContext) {
        cx.update(|cx| {
            gpui_component::init(cx);
            for ctx in [ThemeContext::light(), ThemeContext::dark()] {
                let r = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
                assert_eq!(switch_color(&r), builders::switch_color(&r));
                assert_eq!(
                    bordered_button(&r, cx),
                    builders::hover_border(&r).map(|c| bordered_button_from(c, cx))
                );
                assert_eq!(
                    toolbar_button(&r, cx),
                    builders::toolbar_button_states(&r).map(|s| toolbar_button_from(Some(s), cx))
                );
                assert_eq!(
                    default_button(&r, cx),
                    builders::default_button_states(&r).map(|s| default_button_from(s, cx))
                );
            }
        });
    }

    /// Every public function of [`crate::variants`] but `ghost_button` has
    /// one here.
    #[test]
    fn every_variant_has_a_counterpart() {
        let here: Vec<&str> = include_str!("variants.rs")
            .lines()
            .filter_map(|line| line.strip_prefix("pub fn "))
            .filter_map(|rest| rest.split(['(', '<']).next())
            .collect();
        let old: Vec<&str> = include_str!("../variants.rs")
            .lines()
            .filter_map(|line| line.strip_prefix("pub fn "))
            .filter_map(|rest| rest.split(['(', '<']).next())
            .filter(|name| *name != "ghost_button")
            .collect();
        assert!(
            !old.is_empty() && here == old,
            "0.7.2 {old:?}, 0.8.0 {here:?}"
        );
    }
}
