//! Button variants and a switch colour that carry the platform's state colours
//! where gpui-component's own variants and defaults cannot.
//!
//! gpui-component paints a flat button's hover from a token that means something
//! else. A standalone `.ghost()` button hovers with `tokens.accent`, the
//! item-highlight colour of menus and lists (`src/button/button.rs:1210-1217`);
//! an [`InputGroupButton`](gpui_component::input::InputGroupButton) hovers with
//! `muted`, the subdued-surface colour of `Kbd`, code blocks and chat bubbles
//! (`src/input/group.rs:556-595`). Both readings are hardcoded, and both tokens
//! are mapped correctly for what they are, so no theme value can make either
//! button hover the way the platform's buttons do: under KDE Breeze the first
//! turns the menu's hover tint, the focus colour at 30 % in both modes,
//! and the second a grey barely distinguishable from the field, while ordinary
//! buttons keep their resting fill, as Breeze's do; Breeze marks hover by outline.
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
//! Upstream citations are gpui-component 0.9.1's, but where one names its version.

use gpui::{App, Hsla};
use gpui_component::ActiveTheme as _;
use gpui_component::button::ButtonCustomVariant;

use crate::colors::rgba_to_hsla;
use crate::{ActiveNativeTheme as _, Native};

/// A flat button with the platform's button state colours: transparent when
/// idle, exactly like upstream's `.ghost()`, but hovering and pressing with the
/// colours every other button of the installed theme uses.
///
/// ```no_run
/// use gpui_kit::component::{IconName, button::{Button, ButtonVariants as _}, input::InputGroupButton};
/// use native_theme_gpui::variants;
/// # fn f(cx: &gpui_kit::App) {
/// // Instead of `.ghost()`, on a button and on a button inside an input group:
/// Button::new("close").icon(IconName::Close).custom(variants::ghost_button(cx));
/// InputGroupButton::new("copy").label("Copy").custom(variants::ghost_button(cx));
/// # }
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
/// ```no_run
/// use gpui_kit::component::switch::Switch;
/// use native_theme_gpui::{ActiveNativeTheme as _, variants};
/// # fn f(cx: &gpui_kit::App, on: bool) {
/// let switch = Switch::new("wifi").checked(on);
/// let switch = match cx.native() {
///     Some(n) => switch.color(variants::switch_color(n)),
///     None => switch,
/// };
/// # }
/// ```
///
/// The connector's own `widgets::Switch` (feature `widgets`) draws the
/// checked track from the theme itself.
#[must_use]
pub fn switch_color(n: Native<'_>) -> Hsla {
    rgba_to_hsla(n.resolved.switch.checked_background)
}

/// A gpui-component `Button` with the platform's hover and press border
/// (`button.hover_border_color`), which the stock `Default` variant cannot take: it draws
/// the `input` token there (gpui-component 0.10.0 button/button.rs:1100-1102). `None`
/// where the theme states no hover border; the caller then keeps the `Default` variant.
///
/// ```no_run
/// use gpui_kit::Styled as _;
/// use gpui_kit::component::button::{Button, ButtonVariants as _};
/// use gpui_kit::component::{ActiveTheme as _, Disableable as _, Selectable as _, StyledExt as _};
/// use native_theme_gpui::{ActiveNativeTheme as _, geometry, variants};
/// # fn f(cx: &gpui_kit::App) {
/// let Some(n) = cx.native() else { return };
/// let Some(v) = variants::bordered_button(n, cx) else { return };
/// // At rest: the Default Button's rest fill, which the variant does not give.
/// let apply = Button::new("apply")
///     .custom(v)
///     .refine_style(&geometry::button(n))
///     .bg(cx.theme().button);
/// // Selected: no rest fill; upstream fills it with the variant's `active`.
/// let bold = Button::new("bold")
///     .custom(v)
///     .refine_style(&geometry::button(n))
///     .selected(true)
///     .refine_style(&geometry::button_checked(n));
/// // Disabled: `button_disabled`'s fill replaces the rest fill.
/// let undo = Button::new("undo")
///     .custom(v)
///     .refine_style(&geometry::button(n))
///     .bg(cx.theme().button)
///     .disabled(true)
///     .refine_style(&geometry::button_disabled(n));
/// # }
/// ```
#[must_use]
pub fn bordered_button(n: Native<'_>, cx: &App) -> Option<ButtonCustomVariant> {
    hover_border(n).map(|c| {
        let theme = cx.theme();
        ButtonCustomVariant::new(cx)
            .color(c)
            .foreground(theme.button_foreground)
            .hover(theme.button_hover)
            .active(theme.button_active)
            .shadow(false)
    })
}

/// The colour [`bordered_button`] gives its variant: `button.hover_border_color`, where
/// the theme states one.
pub(crate) fn hover_border(n: Native<'_>) -> Option<Hsla> {
    n.resolved.button.hover_border_color.map(rgba_to_hsla)
}

/// The toolbar button states the theme states, each `None` where it states
/// none: what [`toolbar_button`] gives its variant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ToolbarButtonStates {
    /// `toolbar.button_hover_background`.
    pub hover: Option<Hsla>,
    /// `toolbar.button_hover_border_color`.
    pub hover_border: Option<Hsla>,
    /// `toolbar.button_active_background`.
    pub active: Option<Hsla>,
}

/// The toolbar button states of `n`'s theme.
pub(crate) fn toolbar_button_states(n: Native<'_>) -> ToolbarButtonStates {
    let t = &n.resolved.toolbar;
    ToolbarButtonStates {
        hover: t.button_hover_background.map(rgba_to_hsla),
        hover_border: t.button_hover_border_color.map(rgba_to_hsla),
        active: t.button_active_background.map(rgba_to_hsla),
    }
}

/// A toolbar's flat button: [`ghost_button`] with the platform's toolbar
/// button states where the installed native theme states them.
///
/// ```no_run
/// use gpui_kit::component::{IconName, button::{Button, ButtonVariants as _}, toolbar::Toolbar};
/// use native_theme_gpui::variants;
/// # fn f(cx: &gpui_kit::App) {
/// let copy = Button::new("copy").icon(IconName::Copy).custom(variants::toolbar_button(cx));
/// // `content`, not `child`: `child` would make it a `.ghost()` again.
/// let toolbar = Toolbar::new("tools").content(copy);
/// # }
/// ```
///
/// A toolbar button is a variant of its own because the platform states a
/// toolbar button's hover, pressed and checked colours apart from a flat
/// button's (`toolbar.button_hover_background`, `button_active_background`,
/// `button_checked_background`). Where the theme states one of the first two
/// it replaces [`ghost_button`]'s token; with none, or no native theme
/// installed, the variant is [`ghost_button`]'s.
///
/// Where the theme states the hover outline,
/// `toolbar.button_hover_border_color`, it is the variant's `color`, which
/// upstream draws as the border in every state. So the variant goes with its
/// companion [`geometry::toolbar_button_rest`](crate::geometry::toolbar_button_rest):
/// without it upstream mixes that colour toward transparent into the
/// button's fill at rest (gpui-component 0.10.0 `button/button.rs:1044`) and
/// lays out no border width for a custom variant (0.10.0
/// `button/button.rs:727-732`), so the button is tinted at rest and no edge
/// shows under the pointer. The caller pads the Button with
/// `button.border.padding`, which that builder assumes, and then refines it
/// with the builder.
///
/// A checked toolbar button (`Button::selected(true)`) takes
/// [`geometry::toolbar_button_checked`](crate::geometry::toolbar_button_checked)
/// after the rest refinement: a custom variant fills a selected Button with
/// its pressed colour and has no checked colour of its own, and upstream
/// replays every refinement over the selected state, so the rest one applied
/// last would cover the checked fill.
///
/// Such a Button goes into gpui-component's `Toolbar` or `ToolbarGroup` by
/// `content(..)`, not `child(..)`: `child` prepares a Button for the toolbar
/// by `.ghost().compact()` (gpui-component 0.10.0 `toolbar.rs:17-22`,
/// `:71-74`; `button/button.rs:597-599`), which replaces the custom variant
/// (`with_variant`, 0.10.0 `button/button.rs:603-606`).
#[must_use]
pub fn toolbar_button(cx: &App) -> ButtonCustomVariant {
    let variant = ghost_button(cx);
    let Some(n) = cx.native() else {
        return variant;
    };
    let states = toolbar_button_states(n);
    let variant = match states.hover {
        Some(hover) => variant.hover(hover),
        None => variant,
    };
    let variant = match states.active {
        Some(active) => variant.active(active),
        None => variant,
    };
    match states.hover_border {
        Some(edge) => variant.color(edge),
        None => variant,
    }
}

/// What [`default_button`] gives its variant where the theme gives a
/// dialog's default button a look of its own.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DefaultButtonStates {
    /// `button.default_button_background`: the fill under the pointer.
    pub hover: Hsla,
    /// `button.default_button_text_color`, else `button.font.color`: the
    /// label in every state.
    pub foreground: Hsla,
    /// `button.hover_border_color`, else the default button's frame,
    /// `button.default_button_border_color`, else `button.border.color`: the
    /// border hovered and pressed.
    pub hover_frame: Hsla,
}

/// The default button states of `n`'s theme: `None` where it states no
/// `button.default_button_background`, and the default button is the
/// primary one.
pub(crate) fn default_button_states(n: Native<'_>) -> Option<DefaultButtonStates> {
    let b = &n.resolved.button;
    let fill = b.default_button_background?;
    let frame = b.default_button_border_color.unwrap_or(b.border.color);
    Some(DefaultButtonStates {
        hover: rgba_to_hsla(fill),
        foreground: rgba_to_hsla(b.default_button_text_color.unwrap_or(b.font.color)),
        hover_frame: rgba_to_hsla(b.hover_border_color.unwrap_or(frame)),
    })
}

/// A dialog's default button where the theme gives it a look of its own
/// (`button.default_button_background`, KDE): `None` where it does not, and the
/// caller builds the primary button (`.primary()`), as macOS and Windows draw it.
///
/// ```no_run
/// use gpui_kit::component::button::{Button, ButtonVariants as _};
/// use gpui_kit::component::{Disableable as _, StyledExt as _};
/// use native_theme_gpui::{ActiveNativeTheme as _, geometry, variants};
/// # fn f(cx: &gpui_kit::App) {
/// let Some(n) = cx.native() else { return };
/// let ok = match (variants::default_button(n, cx), geometry::default_button_rest(n)) {
///     // At rest: the theme's fill and frame, which the variant does not give.
///     (Some(v), Some(rest)) => Button::new("ok")
///         .custom(v)
///         .refine_style(&geometry::button(n))
///         .refine_style(&rest),
///     // No look of its own: the default button is the primary one.
///     _ => Button::new("ok").primary().refine_style(&geometry::button(n)),
/// };
/// // Disabled: `button_disabled` in place of the rest look, whose frame it
/// // would not replace.
/// if let Some(v) = variants::default_button(n, cx) {
///     let off = Button::new("ok-off")
///         .custom(v)
///         .refine_style(&geometry::button(n))
///         .disabled(true)
///         .refine_style(&geometry::button_disabled(n));
/// }
/// # }
/// ```
///
/// The variant's `color` is the border hovered and pressed,
/// `button.hover_border_color`, else the default button's frame
/// (`button.default_button_border_color`, else `button.border.color`); its
/// `hover` the default fill, which Breeze keeps under the pointer; its
/// `active` the plain button's pressed fill, the `button_active` token that
/// [`apply`](crate::apply) fills from `button.active_background`; and its
/// `foreground` `button.default_button_text_color`, else `button.font.color`.
/// A custom variant has one foreground for every state (gpui-component
/// 0.10.0 `button/button.rs:1096`), so a pressed default button keeps that
/// label where a plain pressed button is lettered in
/// `button.active_text_color`: on a theme that states the two apart, the
/// pressed label differs.
///
/// The variant's own rest fill is a fifth of its `color` (0.10.0
/// `button/button.rs:1044`), so an enabled Button takes
/// [`geometry::default_button_rest`](crate::geometry::default_button_rest),
/// the theme's fill and frame, after
/// [`geometry::button`](crate::geometry::button). A disabled one takes
/// [`geometry::button_disabled`](crate::geometry::button_disabled) in its
/// place and keeps `geometry::button`'s `button.border.color`: upstream
/// replays a refinement in the disabled state too (0.10.0
/// `button/button.rs:834-851`), and `button_disabled` sets no border colour,
/// so the rest frame would stand there.
///
/// A plain `Dialog`'s footer is the application's (`Dialog::footer`), so
/// such a button goes there; an `AlertDialog`'s default footer builds its OK
/// button from a `ButtonVariant` alone (0.10.0 `dialog/dialog.rs:149-162`),
/// so that one stays the primary button.
#[must_use]
pub fn default_button(n: Native<'_>, cx: &App) -> Option<ButtonCustomVariant> {
    default_button_states(n).map(|s| {
        ButtonCustomVariant::new(cx)
            .color(s.hover_frame)
            .foreground(s.foreground)
            .hover(s.hover)
            .active(cx.theme().button_active)
            .shadow(false)
    })
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

    /// `toolbar_button` takes the toolbar's states where the installed theme
    /// states them: kde-breeze light hovers transparent, presses in
    /// #3daee94d and draws its outline as the variant's colour; macos-tahoe,
    /// which states none of the three, keeps `ghost_button`'s tokens.
    #[gpui::test]
    fn toolbar_button_takes_the_toolbars_states(cx: &mut TestAppContext) {
        use native_theme::color::Rgba;
        let prefs = AccessibilityPreferences::default();
        cx.update(|cx| {
            gpui_component::init(cx);
            let loaded = crate::from_preset("kde-breeze", false, &prefs);
            assert!(loaded.is_ok(), "kde-breeze: {:?}", loaded.as_ref().err());
            let Ok((theme, resolved)) = loaded else {
                return;
            };
            crate::apply(theme, &resolved, &prefs, cx);
            let edge = resolved.toolbar.button_hover_border_color;
            assert!(edge.is_some(), "kde-breeze states the toolbar's outline");
            let Some(edge) = edge else {
                return;
            };
            assert_eq!(
                toolbar_button(cx),
                ghost_button(cx)
                    .hover(rgba_to_hsla(Rgba::new(0, 0, 0, 0)))
                    .active(rgba_to_hsla(Rgba::new(0x3d, 0xae, 0xe9, 0x4d)))
                    .color(rgba_to_hsla(edge)),
                "kde-breeze light"
            );

            let loaded = crate::from_preset("macos-tahoe", false, &prefs);
            assert!(loaded.is_ok(), "macos-tahoe: {:?}", loaded.as_ref().err());
            let Ok((theme, resolved)) = loaded else {
                return;
            };
            let t = &resolved.toolbar;
            assert_eq!(
                (
                    t.button_hover_background,
                    t.button_hover_border_color,
                    t.button_active_background
                ),
                (None, None, None),
                "macos-tahoe states no toolbar button state"
            );
            crate::apply(theme, &resolved, &prefs, cx);
            assert_eq!(toolbar_button(cx), ghost_button(cx), "macos-tahoe");
        });
    }

    /// Where the theme states no `button.hover_border_color` (adwaita, both
    /// modes), `bordered_button` builds no variant, so the caller keeps the
    /// stock `Default` button; kde-breeze, which states one, gets a variant.
    #[gpui::test]
    fn an_unset_hover_border_builds_no_bordered_button(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        cx.update(|cx| {
            gpui_component::init(cx);
            for (preset, states_one) in [("adwaita", false), ("kde-breeze", true)] {
                for is_dark in [false, true] {
                    let loaded = crate::from_preset(preset, is_dark, &prefs);
                    assert!(
                        loaded.is_ok(),
                        "{preset}/{is_dark}: {:?}",
                        loaded.as_ref().err()
                    );
                    let Ok((theme, resolved)) = loaded else {
                        return;
                    };
                    assert_eq!(
                        resolved.button.hover_border_color.is_some(),
                        states_one,
                        "{preset}/{is_dark}: the preset's button.hover_border_color"
                    );
                    crate::apply(theme, &resolved, &prefs, cx);
                    assert_eq!(
                        bordered_button(Native::unscaled(&resolved), cx).is_some(),
                        states_one,
                        "{preset}/{is_dark}: bordered_button"
                    );
                }
            }
        });
    }

    /// Where the theme states no `button.default_button_background`, every
    /// bundled preset but kde-breeze in both modes, the default button is
    /// the primary one: `default_button_states`,
    /// `geometry::default_button_rest` and `default_button` are all `None`.
    /// kde-breeze, which states the fill, gets all three.
    #[gpui::test]
    fn default_button_is_none_where_no_fill_is_stated(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        cx.update(|cx| {
            gpui_component::init(cx);
            let mut unstated = 0usize;
            for info in crate::Theme::list_presets() {
                let states_one = info.key == "kde-breeze";
                for is_dark in [false, true] {
                    let loaded = crate::from_preset(info.key, is_dark, &prefs);
                    assert!(
                        loaded.is_ok(),
                        "{}/{is_dark}: {:?}",
                        info.key,
                        loaded.as_ref().err()
                    );
                    let Ok((theme, resolved)) = loaded else {
                        return;
                    };
                    crate::apply(theme, &resolved, &prefs, cx);
                    let n = Native::unscaled(&resolved);
                    let built = (
                        default_button_states(n).is_some(),
                        crate::geometry::default_button_rest(n).is_some(),
                        default_button(n, cx).is_some(),
                    );
                    assert_eq!(
                        built,
                        (states_one, states_one, states_one),
                        "{}/{is_dark}: (default_button_states, default_button_rest, default_button)",
                        info.key
                    );
                    if !states_one {
                        unstated += 1;
                    }
                }
            }
            assert!(
                unstated > 0,
                "every preset states a default button fill, so this proves nothing"
            );
        });
    }

    /// Built in `render`, the default button's variant follows a mode
    /// switch: kde-breeze dark's fill, label and frame replace the light
    /// ones', with the installed theme's `button_active`.
    #[gpui::test]
    fn default_button_follows_the_installed_theme(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        cx.update(|cx| {
            gpui_component::init(cx);
            let loaded = crate::from_preset("kde-breeze", false, &prefs);
            assert!(loaded.is_ok(), "light: {:?}", loaded.as_ref().err());
            let Ok((light, resolved)) = loaded else {
                return;
            };
            crate::apply(light, &resolved, &prefs, cx);
            let in_light = default_button(Native::unscaled(&resolved), cx);
            assert!(in_light.is_some(), "kde-breeze light states the fill");

            let loaded = crate::from_preset("kde-breeze", true, &prefs);
            assert!(loaded.is_ok(), "dark: {:?}", loaded.as_ref().err());
            let Ok((dark, resolved)) = loaded else {
                return;
            };
            crate::apply(dark, &resolved, &prefs, cx);
            let in_dark = default_button(Native::unscaled(&resolved), cx);
            assert_ne!(in_dark, in_light);
            let b = &resolved.button;
            let stated = (b.default_button_background, b.hover_border_color);
            assert!(
                stated.0.is_some() && stated.1.is_some(),
                "kde-breeze dark states the fill and the hover border: {stated:?}"
            );
            let (Some(fill), Some(frame)) = stated else {
                return;
            };
            assert_eq!(
                in_dark,
                Some(
                    ButtonCustomVariant::new(cx)
                        .color(rgba_to_hsla(frame))
                        .foreground(rgba_to_hsla(
                            b.default_button_text_color.unwrap_or(b.font.color)
                        ))
                        .hover(rgba_to_hsla(fill))
                        .active(cx.theme().button_active)
                        .shadow(false)
                )
            );
        });
    }
}
