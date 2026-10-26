//! Button variants and a switch colour that carry the platform's state colours
//! where gpui-component's own variants and defaults cannot.
//!
//! gpui-component paints a flat button's hover from a token that means something
//! else. A standalone `.ghost()` button hovers with `tokens.accent`, the
//! item-highlight colour of menus and lists (`src/button/button.rs:1205-1212`);
//! an [`InputGroupButton`](gpui_component::input::InputGroupButton) hovers with
//! `muted`, the subdued-surface colour of `Kbd`, code blocks and chat bubbles
//! (`src/input/group.rs:556-595`). Both readings are hardcoded, and both tokens
//! are mapped correctly for what they are, so no theme value can make either
//! button hover the way the platform's buttons do: under KDE Breeze the first
//! turns selection blue and the second a grey that is barely distinguishable
//! from the field, while every ordinary button beside them takes Breeze's
//! `DecorationHover`.
//!
//! `ButtonVariants::custom` is upstream's supported seam for exactly this, and
//! it reaches both widgets: a custom variant replaces `.ghost()` on a `Button`,
//! and makes `InputGroupButton` skip its in-group repaint
//! (`src/input/group.rs:556-557`). Buttons gpui-component builds internally
//! (a dialog's close button, calendar navigation, the tab bar) stay upstream
//! work; see "Upstream PR candidates" in `docs/todo.md`.
//!
//! A checked `Switch` is the same case at another seam: upstream fills its
//! track with `tokens.primary` unless `Switch::color` hands it a colour, and
//! `ThemeColor` has no checked-switch field the connector could fill, so
//! [`switch_color`] reads the platform's from the model.
//!
//! Upstream citations in this module are verified against gpui-component 0.8.1.

use gpui::{App, Hsla};
use gpui_component::ActiveTheme as _;
use gpui_component::button::ButtonCustomVariant;

use crate::Native;
use crate::colors::rgba_to_hsla;

/// A flat button with the platform's button state colours: transparent when
/// idle, exactly like upstream's `.ghost()`, but hovering and pressing with the
/// colours every other button of the installed theme uses.
///
/// ```ignore
/// use gpui_kit::component::button::{Button, ButtonVariants as _};
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
/// The label keeps its colour on hover, as docs/platform-facts.md records
/// for all four platforms (§2.3, `hover_text_color`).
#[must_use]
pub fn ghost_button(cx: &App) -> ButtonCustomVariant {
    let theme = cx.theme();
    ButtonCustomVariant::new(cx)
        .foreground(theme.secondary_foreground)
        .hover(theme.secondary_hover)
        .active(theme.secondary_active)
}

/// The colour for gpui-component's `Switch::color`, its checked track: the
/// platform's `switch.checked_background`.
///
/// Upstream fills a checked `Switch` with `tokens.primary` unless it is handed
/// a colour (`src/switch.rs:101-106`, `:181-184`), and `ThemeColor` has only
/// `switch` and `switch_thumb`, so no token carries this value. It is a leaf
/// of the model, read from the [`Native`] view the
/// [`geometry`](crate::geometry) builders take, not from the installed theme.
///
/// ```ignore
/// use gpui_kit::component::switch::Switch;
/// use native_theme_gpui::{ActiveNativeTheme as _, variants};
///
/// let switch = Switch::new("wifi").checked(on);
/// let switch = match cx.native() {
///     Some(n) => switch.color(variants::switch_color(n)),
///     None => switch,
/// };
/// ```
///
/// The connector's own `widgets::Switch` (feature `widgets`) draws the
/// checked track from the theme itself.
#[must_use]
pub fn switch_color(n: Native<'_>) -> Hsla {
    rgba_to_hsla(n.resolved.switch.checked_background)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use native_theme::AccessibilityPreferences;

    /// `switch_color` is the platform's `switch.checked_background` in every
    /// preset and mode, and differs from the `primary` token upstream would
    /// have filled the track with somewhere, or it would be a second name for
    /// that token.
    #[test]
    fn switch_color_is_the_platforms_checked_track() {
        let prefs = AccessibilityPreferences::default();
        let mut not_primary = Vec::new();
        for info in crate::Theme::list_presets() {
            for is_dark in [false, true] {
                let (theme, resolved) =
                    crate::from_preset(info.key, is_dark, &prefs).expect("preset should load");
                let colour = switch_color(Native::unscaled(&resolved));
                assert_eq!(
                    colour,
                    rgba_to_hsla(resolved.switch.checked_background),
                    "{}/{is_dark}",
                    info.key
                );
                if colour != theme.primary {
                    not_primary.push(info.key);
                }
            }
        }
        assert!(
            !not_primary.is_empty(),
            "every preset's checked switch is its primary token, so this proves nothing"
        );
    }

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
