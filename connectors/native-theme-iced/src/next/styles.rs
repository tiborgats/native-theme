//! [`crate::styles`] on the 0.8.0 model: each function reads what its 0.7.2
//! namesake reads, at its home in the table of homes, and hands it to the
//! function that namesake hands its own reads to, so both return the same
//! style function from the same values.
//!
//! Requires the `widgets` feature (on by default).

use iced_core::theme::Theme;
use native_theme::next::{ButtonQuery, ResolvedTheme, WidgetState};

use super::{
    Mapped, border, fill, state_fill, state_stroke_color, state_text_color, stated, text_color,
    to_color,
};
use crate::styles::{self as old, ButtonSource, LinkSource, ToolbarButtonSource, UNFADED};

/// A widget's disabled opacity: the stated one, or, where the theme states
/// none, [`UNFADED`], iced's own disabled look, which fades nothing.
fn opacity(stated: Option<f32>) -> f32 {
    stated.unwrap_or(UNFADED)
}

/// [`ButtonSource`] of the 0.8.0 model.
fn button_source(r: &ResolvedTheme) -> Mapped<ButtonSource> {
    let b = &r.button;
    let rest = b.body(WidgetState::NONE);
    let hover = b.body(WidgetState::HOVER);
    let default = b.body(WidgetState::DEFAULT);
    let c = &r.colors;
    Ok(ButtonSource {
        background: fill(&rest, "button.body(NONE).fill")?,
        hover_background: fill(&hover, "button.body(HOVER).fill")?,
        active_background: state_fill(&b.body(WidgetState::PRESSED), "button.body(PRESSED).fill")?,
        disabled_background: state_fill(
            &b.body(WidgetState::DISABLED),
            "button.body(DISABLED).fill",
        )?,
        checked_background: state_fill(&b.body(WidgetState::CHECKED), "button.body(CHECKED).fill")?,
        label: text_color(&b.label(WidgetState::NONE), "button.label(NONE).color")?,
        hover_label: text_color(&b.label(WidgetState::HOVER), "button.label(HOVER).color")?,
        active_label: text_color(
            &b.label(WidgetState::PRESSED),
            "button.label(PRESSED).color",
        )?,
        disabled_label: text_color(
            &b.label(WidgetState::DISABLED),
            "button.label(DISABLED).color",
        )?,
        checked_label: state_text_color(
            &b.label(WidgetState::CHECKED),
            "button.label(CHECKED).color",
        )?,
        disabled_opacity: opacity(b.disabled_opacity),
        border: border!(rest, "button.body(NONE)")?,
        hover_border_color: state_stroke_color(&hover, "button.body(HOVER).stroke.color")?,
        primary_background: fill(
            &b.body(ButtonQuery::primary()),
            "button.body(NONE; variant=Primary).fill",
        )?,
        primary_label: text_color(
            &b.label(ButtonQuery::primary()),
            "button.label(NONE; variant=Primary).color",
        )?,
        default_background: state_fill(&default, "button.body(DEFAULT).fill")?,
        default_label: state_text_color(
            &b.label(WidgetState::DEFAULT),
            "button.label(DEFAULT).color",
        )?,
        default_border_color: state_stroke_color(&default, "button.body(DEFAULT).stroke.color")?,
        window_background: to_color(stated(c.window.background, "colors.window.background")?),
        window_text: to_color(stated(c.window.foreground, "colors.window.foreground")?),
    })
}

/// [`ToolbarButtonSource`] of the 0.8.0 model.
fn toolbar_button_source(r: &ResolvedTheme) -> Mapped<ToolbarButtonSource> {
    let t = &r.toolbar;
    let hover = t.button(WidgetState::HOVER);
    let checked = t.button(WidgetState::CHECKED);
    Ok(ToolbarButtonSource {
        hover_background: state_fill(&hover, "toolbar.button(HOVER).fill")?,
        active_background: state_fill(
            &t.button(WidgetState::PRESSED),
            "toolbar.button(PRESSED).fill",
        )?,
        checked_background: state_fill(&checked, "toolbar.button(CHECKED).fill")?,
        hover_border_color: state_stroke_color(&hover, "toolbar.button(HOVER).stroke.color")?,
        checked_border_color: state_stroke_color(&checked, "toolbar.button(CHECKED).stroke.color")?,
    })
}

/// [`crate::styles::button`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::button_from(&button_source(r)?))
}

/// [`crate::styles::button_toggle`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_toggle(
    r: &ResolvedTheme,
    checked: bool,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::button_toggle_from(&button_source(r)?, checked))
}

/// [`crate::styles::toolbar_button`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn toolbar_button(
    r: &ResolvedTheme,
    checked: bool,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::toolbar_button_from(
        &button_source(r)?,
        &toolbar_button_source(r)?,
        checked,
    ))
}

/// [`crate::styles::button_primary`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_primary(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::button_primary_from(&button_source(r)?))
}

/// [`crate::styles::button_default`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_default(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    Ok(old::button_default_from(&button_source(r)?))
}

/// [`crate::styles::button_danger`]: the error role's fill and foreground.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_danger(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let role = &r.colors.error;
    Ok(old::class_button(
        &button_source(r)?,
        to_color(role.fill),
        to_color(role.foreground),
        iced_widget::button::danger,
    ))
}

/// [`crate::styles::button_success`]: the success role's fill and
/// foreground.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_success(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let role = &r.colors.success;
    Ok(old::class_button(
        &button_source(r)?,
        to_color(role.fill),
        to_color(role.foreground),
        iced_widget::button::success,
    ))
}

/// [`crate::styles::button_warning`]: the warning role's fill and
/// foreground.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the button needs.
pub fn button_warning(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let role = &r.colors.warning;
    Ok(old::class_button(
        &button_source(r)?,
        to_color(role.fill),
        to_color(role.foreground),
        iced_widget::button::warning,
    ))
}

/// [`crate::styles::button_link`]: faded when disabled by the theme's
/// `colors.disabled_opacity`, the link stating none of its own.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the link needs.
pub fn button_link(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<>,
> {
    let l = &r.link;
    Ok(old::button_link_from(&LinkSource {
        background: fill(&l.container(WidgetState::NONE), "link.container(NONE).fill")?,
        hover_background: fill(
            &l.container(WidgetState::HOVER),
            "link.container(HOVER).fill",
        )?,
        label: text_color(&l.label(WidgetState::NONE), "link.label(NONE).color")?,
        hover_label: text_color(&l.label(WidgetState::HOVER), "link.label(HOVER).color")?,
        active_label: text_color(&l.label(WidgetState::PRESSED), "link.label(PRESSED).color")?,
        disabled_label: text_color(
            &l.label(WidgetState::DISABLED),
            "link.label(DISABLED).color",
        )?,
        disabled_opacity: r.colors.disabled_opacity,
    }))
}
