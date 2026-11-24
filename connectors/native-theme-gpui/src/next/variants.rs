//! The button variants and the switch colour of [`crate::variants`] that
//! read the native theme, from the 0.8.0 model: one function of the same
//! name for each, taking the 0.8.0
//! [`ResolvedTheme`] in place of
//! [`Native`](crate::Native) or of the installed native theme, and
//! returning the same value, or the [`Unmapped`](super::Unmapped) source it
//! lacks.
//!
//! [`ghost_button`] adds to [`crate::variants::ghost_button`], which reads
//! only the installed theme, the flat button's hovered outline. Each
//! variant below takes, as its 0.7.2 counterpart does, its other colours from
//! that installed theme.

use gpui::{App, Hsla};
use gpui_component::ActiveTheme as _;
use gpui_component::button::ButtonCustomVariant;
use native_theme::next::ResolvedTheme;

use super::{Mapped, builders};
use crate::variants::{
    ToolbarButtonStates, bordered_button_from, default_button_from, toolbar_button_from,
};

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
/// button states `r` states, over [`crate::variants::ghost_button`]; a
/// hovered or pressed fill no rule states is gpui-component's own toolbar
/// button's (spec §10, rule 0), the ghost's `accent` and `button_active`,
/// not the flat button's.
pub fn toolbar_button(r: &ResolvedTheme, cx: &App) -> Mapped<ButtonCustomVariant> {
    let stated = builders::toolbar_button_states(r)?;
    let own = own_toolbar_states(cx);
    Ok(toolbar_button_from(
        Some(ToolbarButtonStates {
            hover: stated.hover.or(own.hover),
            hover_border: stated.hover_border,
            active: stated.active.or(own.active),
        }),
        cx,
    ))
}

/// gpui-component's own toolbar button's hovered and pressed fills, read
/// from the installed theme: a Button its `Toolbar` lays out by `child` is
/// made a ghost one (`Button::prepare_for_toolbar`, gpui-component 0.10.0
/// `button/button.rs:596-598`; `toolbar.rs:17-22`), hovered in
/// `tokens.accent`, at half its alpha in a dark theme (`:1226-1233`), and
/// pressed in `tokens.button_active` (`:1281`). Its label and its rest fill
/// and border are [`crate::variants::ghost_button`]'s already: the ghost's
/// text is `secondary_foreground` (`:1065`), its fill and border
/// transparent (`:1043`, `:1133`). No outline is its own.
fn own_toolbar_states(cx: &App) -> ToolbarButtonStates {
    let theme = cx.theme();
    let accent = Hsla::from(theme.tokens.accent);
    let hover = if theme.mode.is_dark() {
        accent.opacity(0.5)
    } else {
        accent
    };
    ToolbarButtonStates {
        hover: Some(hover),
        hover_border: None,
        active: Some(Hsla::from(theme.tokens.button_active)),
    }
}

/// [`crate::variants::default_button`], from the 0.8.0 model: `None` where
/// no rule of the theme states the default button's fill.
pub fn default_button(r: &ResolvedTheme, cx: &App) -> Mapped<Option<ButtonCustomVariant>> {
    Ok(default_button_from(builders::default_button_states(r)?, cx))
}

/// [`crate::variants::ghost_button`], from the 0.8.0 model, with the flat
/// button's hovered outline, `button.body(HOVER; variant=Flat).stroke.color`,
/// as its colour where a rule states it for the state; with none, the 0.7.2
/// variant as it is.
///
/// Upstream draws a custom variant's colour as the border in every state and
/// mixes it toward transparent into the fill at rest, with no border width
/// (gpui-component 0.10.0 `button/button.rs:1044`, `:727-732`), so an
/// outlined variant goes with its companion
/// [`geometry::ghost_button_rest`](super::geometry::ghost_button_rest),
/// refined after [`geometry::button`](super::geometry::button): it hides both
/// at rest and lays the border out inside the button's padding. Until a
/// preset states a flat rule, a flat query returns the push button's rule
/// (spec §1, "Axis values without data"), so the outline is the push
/// button's hovered stroke.
pub fn ghost_button(r: &ResolvedTheme, cx: &App) -> Mapped<ButtonCustomVariant> {
    let variant = crate::variants::ghost_button(cx);
    Ok(match builders::ghost_hover_border(r)? {
        Some(edge) => variant.color(edge),
        None => variant,
    })
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
                let own = own_toolbar_states(cx);
                assert_eq!(
                    toolbar_button(&r, cx),
                    builders::toolbar_button_states(&r).map(|s| toolbar_button_from(
                        Some(ToolbarButtonStates {
                            hover: s.hover.or(own.hover),
                            hover_border: s.hover_border,
                            active: s.active.or(own.active),
                        }),
                        cx
                    ))
                );
                assert_eq!(
                    default_button(&r, cx),
                    builders::default_button_states(&r).map(|s| default_button_from(s, cx))
                );
            }
        });
    }

    /// A toolbar button whose hovered and pressed fills no rule states
    /// (macos-tahoe states no toolbar button state) hovers and presses as
    /// gpui-component's own toolbar button, its ghost one: `tokens.accent`,
    /// at half its alpha in the dark scheme, and `tokens.button_active`, not
    /// the flat button's `secondary_hover` and `secondary_active`; its label
    /// and its rest are the flat button's, which are the ghost's too. The
    /// flat button's pressed fill is patched to one of its own, which
    /// macos-tahoe gives the push button's, so the pressed fills differ.
    #[gpui::test]
    fn an_unstated_toolbar_state_is_gpui_components_own(cx: &mut TestAppContext) {
        use native_theme::next::{
            ButtonRule, ButtonVariant, Color, Paint, PartPatch, ThemePatch, WidgetState,
        };
        cx.update(|cx| {
            gpui_component::init(cx);
            for ctx in [ThemeContext::light(), ThemeContext::dark()] {
                let mut r = Theme::preset(Preset::MacosTahoe).resolve(&ctx);
                r.apply_patch(
                    &ThemePatch::new().button(
                        ButtonRule::new()
                            .variant(ButtonVariant::Flat)
                            .states(WidgetState::PRESSED)
                            .body(PartPatch::new().fill(Paint::Solid(Color::rgb(1.0, 0.0, 0.0)))),
                    ),
                );
                assert_eq!(
                    builders::toolbar_button_states(&r),
                    Ok(ToolbarButtonStates {
                        hover: None,
                        hover_border: None,
                        active: None,
                    }),
                    "macos-tahoe {:?} states a toolbar button state",
                    ctx.scheme
                );
                assert_eq!(super::super::apply(&r, cx), Ok(()));
                let theme = cx.theme();
                let accent = Hsla::from(theme.tokens.accent);
                let hover = if r.is_dark() {
                    accent.opacity(0.5)
                } else {
                    accent
                };
                let active = Hsla::from(theme.tokens.button_active);
                let flat = crate::variants::ghost_button(cx);
                assert!(
                    hover != theme.secondary_hover && active != theme.secondary_active,
                    "macos-tahoe {:?}: the ghost's own fills are the flat button's, so this \
                     proves nothing",
                    ctx.scheme
                );
                assert_eq!(
                    toolbar_button(&r, cx),
                    Ok(flat.hover(hover).active(active)),
                    "macos-tahoe {:?}",
                    ctx.scheme
                );
            }
        });
    }

    /// A ghost button whose flat hovered outline a rule states (kde-breeze's
    /// push button's, a flat query with no flat rule returning the push
    /// button's rule) takes it as its variant's colour, over 0.7.2's
    /// variant; a patched flat rule's outline replaces it; and the rest
    /// refinement lays out that rule's width inside the push button's
    /// padding, with a transparent fill and border. (Every bundled preset
    /// states a hovered push button stroke, so none leaves the outline
    /// unstated.)
    #[gpui::test]
    fn a_stated_flat_hover_outline_edges_the_ghost(cx: &mut TestAppContext) {
        use native_theme::next::{
            ButtonRule, ButtonVariant, Color, PartPatch, Stroke, ThemePatch, WidgetState,
        };
        const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
        cx.update(|cx| {
            gpui_component::init(cx);
            for ctx in [ThemeContext::light(), ThemeContext::dark()] {
                let mut r = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
                assert_eq!(super::super::apply(&r, cx), Ok(()));
                let push = builders::hover_border(&r);
                assert!(
                    matches!(push, Ok(Some(_))),
                    "kde-breeze {:?} states no hovered push button stroke, so this proves \
                     nothing",
                    ctx.scheme
                );
                assert_eq!(builders::ghost_hover_border(&r), push, "{:?}", ctx.scheme);
                assert_eq!(
                    ghost_button(&r, cx),
                    push.map(|edge| match edge {
                        Some(edge) => crate::variants::ghost_button(cx).color(edge),
                        None => crate::variants::ghost_button(cx),
                    }),
                    "{:?}",
                    ctx.scheme
                );
                r.apply_patch(
                    &ThemePatch::new().button(
                        ButtonRule::new()
                            .variant(ButtonVariant::Flat)
                            .states(WidgetState::HOVER)
                            .body(PartPatch::new().stroke(Stroke::uniform(3.0, GREEN))),
                    ),
                );
                assert_eq!(super::super::apply(&r, cx), Ok(()));
                assert_eq!(
                    ghost_button(&r, cx),
                    Ok(crate::variants::ghost_button(cx).color(super::super::hsla(GREEN))),
                    "{:?}",
                    ctx.scheme
                );
                let padding = super::super::padding_of(&r.button.body(WidgetState::NONE));
                assert!(
                    padding.iter().all(|side| side.is_some_and(|v| v > 3.0)),
                    "kde-breeze {:?} pads its push button by no more than the patched width \
                     on some side, so the padding kept proves nothing",
                    ctx.scheme
                );
                assert_eq!(
                    super::super::geometry::ghost_button_rest(&r),
                    Ok(crate::geometry::toolbar_button_rest_from(
                        true, 3.0, padding
                    )),
                    "{:?}",
                    ctx.scheme
                );
            }
        });
    }

    /// Every public function of [`crate::variants`] has one here.
    #[test]
    fn every_variant_has_a_counterpart() {
        let here: Vec<&str> = include_str!("variants.rs")
            .lines()
            .filter_map(|line| line.strip_prefix("pub fn "))
            .filter_map(|rest| rest.split(['(', '<']).next())
            .collect();
        let mut here = here;
        here.sort_unstable();
        let mut old: Vec<&str> = include_str!("../variants.rs")
            .lines()
            .filter_map(|line| line.strip_prefix("pub fn "))
            .filter_map(|rest| rest.split(['(', '<']).next())
            .collect();
        old.sort_unstable();
        assert!(
            !old.is_empty() && here == old,
            "0.7.2 {old:?}, 0.8.0 {here:?}"
        );
    }
}
