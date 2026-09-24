//! Button variants that carry the platform's state colours where gpui-component's
//! own variants cannot.
//!
//! gpui-component paints a flat button's hover from a token that means something
//! else. A standalone `.ghost()` button hovers with `tokens.accent`, the
//! item-highlight colour of menus and lists (`src/button/button.rs:1125-1132`);
//! an [`InputGroupButton`](gpui_component::input::InputGroupButton) hovers with
//! `muted`, the subdued-surface colour of `Kbd`, code blocks and chat bubbles
//! (`src/input/group.rs:544-583`). Both readings are hardcoded, and both tokens
//! are mapped correctly for what they are, so no theme value can make either
//! button hover the way the platform's buttons do: under KDE Breeze the first
//! turns selection blue and the second a grey that is barely distinguishable
//! from the field, while every ordinary button beside them takes Breeze's
//! `DecorationHover`.
//!
//! `ButtonVariants::custom` is upstream's supported seam for exactly this, and
//! it reaches both widgets: a custom variant replaces `.ghost()` on a `Button`,
//! and makes `InputGroupButton` skip its in-group repaint
//! (`src/input/group.rs:544-545`). Buttons gpui-component builds internally
//! (a dialog's close button, calendar navigation, the tab bar) stay upstream
//! work; see "Upstream PR candidates" in `docs/todo.md`.
//!
//! Upstream citations in this module are verified against gpui-component 0.6.6.

use gpui::App;
use gpui_component::ActiveTheme as _;
use gpui_component::button::ButtonCustomVariant;

/// A flat button with the platform's button state colours: transparent when
/// idle, exactly like upstream's `.ghost()`, but hovering and pressing with the
/// colours every other button of the installed theme uses.
///
/// ```ignore
/// use gpui_component::button::{Button, ButtonVariants as _};
/// use native_theme_gpui::variants;
///
/// // instead of `.ghost()`
/// Button::new("close").icon(IconName::Close).custom(variants::ghost_button(cx));
/// // inside an input group
/// InputGroupButton::new("copy").label("Copy").custom(variants::ghost_button(cx));
/// ```
///
/// Read from the installed theme at call time: `secondary_foreground`,
/// `secondary_hover` and `secondary_active`, which [`apply`](crate::apply)
/// fills from `button.font.color`, `button.hover_background` and, where
/// stated, `button.active_background` (else derived from the button's
/// background), so a button built in `render` follows a light/dark switch.
/// The label keeps its colour on hover, as platform-facts records for all
/// four platforms (§2.3 `hover_text_color`).
#[must_use]
pub fn ghost_button(cx: &App) -> ButtonCustomVariant {
    let theme = cx.theme();
    ButtonCustomVariant::new(cx)
        .foreground(theme.secondary_foreground)
        .hover(theme.secondary_hover)
        .active(theme.secondary_active)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::colors::rgba_to_hsla;
    use gpui::TestAppContext;
    use native_theme::AccessibilityPreferences;

    /// The hover is the platform's own `button.hover_background`, taken here
    /// from the resolved theme rather than from the installed one, and it is
    /// neither of the two tokens upstream would have painted a flat button
    /// with. kde-breeze is the input because all three colours differ there;
    /// if a preset change ever makes them equal, the `assert_ne!`s say so.
    #[gpui::test]
    fn ghost_button_hovers_with_the_platforms_button_hover(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) =
            crate::from_preset("kde-breeze", false, &prefs).expect("preset should load");
        cx.update(|cx| {
            gpui_component::init(cx);
            crate::apply(theme, &resolved, &prefs, cx);

            let native = ButtonCustomVariant::new(cx)
                .foreground(rgba_to_hsla(resolved.button.font.color))
                .hover(rgba_to_hsla(resolved.button.hover_background))
                .active(cx.theme().secondary_active);
            assert_eq!(ghost_button(cx), native);

            let like_upstreams_ghost = native.hover(cx.theme().accent);
            let like_upstreams_addon = native.hover(cx.theme().muted);
            assert_ne!(ghost_button(cx), like_upstreams_ghost);
            assert_ne!(ghost_button(cx), like_upstreams_addon);
        });
    }

    /// Built in `render`, the variant follows a mode switch: the dark theme's
    /// button hover replaces the light one's.
    #[gpui::test]
    fn ghost_button_follows_the_installed_theme(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        cx.update(|cx| {
            gpui_component::init(cx);
            let (light, resolved) =
                crate::from_preset("kde-breeze", false, &prefs).expect("light variant");
            crate::apply(light, &resolved, &prefs, cx);
            let in_light = ghost_button(cx);

            let (dark, resolved) =
                crate::from_preset("kde-breeze", true, &prefs).expect("dark variant");
            crate::apply(dark, &resolved, &prefs, cx);
            assert_ne!(ghost_button(cx), in_light);
            assert_eq!(
                ghost_button(cx),
                ButtonCustomVariant::new(cx)
                    .foreground(cx.theme().secondary_foreground)
                    .hover(cx.theme().secondary_hover)
                    .active(cx.theme().secondary_active)
            );
        });
    }
}
