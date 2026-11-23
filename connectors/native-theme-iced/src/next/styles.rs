//! [`crate::styles`] on the 0.8.0 model: each function reads what its 0.7.2
//! namesake reads, at its home in the table of homes, and hands it to the
//! function that namesake hands its own reads to, so both return the same
//! style function from the same values.
//!
//! Requires the `widgets` feature (on by default).

use iced_core::theme::Theme;
use native_theme::next::{
    ButtonQuery, PartStated, ResolvedTheme, Stroke, WidgetState, WindowState,
};

use super::borders::{outline, outline_or_iced, state_line};
use super::{Mapped, Unmapped, fill, state_fill, state_text_color, stated, text_color, to_color};
use crate::styles::{
    self as old, ButtonSource, InputSource, LinkSource, Outline, ToolbarButtonSource,
};

/// [`ButtonSource`] of the 0.8.0 model. A state option no rule states for
/// its state (its `from_state` bit clear) is `None`, and the state is then
/// iced's own for that status (spec §2, `from_state`). A button that states
/// no disabled opacity takes iced's own disabled look for each colour the
/// theme leaves to iced (spec §10, rule 0).
fn button_source(r: &ResolvedTheme) -> Mapped<ButtonSource> {
    let b = &r.button;
    let rest = b.body(WidgetState::NONE);
    let hover = b.body(WidgetState::HOVER);
    let default = b.body(WidgetState::DEFAULT);
    let primary = b.body(ButtonQuery::primary());
    let c = &r.colors;
    Ok(ButtonSource {
        background: fill(&rest, "button.body(NONE).fill")?,
        hover_background: state_fill(&hover, "button.body(HOVER).fill")?,
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
        disabled_label: state_text_color(
            &b.label(WidgetState::DISABLED),
            "button.label(DISABLED).color",
        )?,
        checked_label: state_text_color(
            &b.label(WidgetState::CHECKED),
            "button.label(CHECKED).color",
        )?,
        disabled_opacity: b.disabled_opacity,
        border: outline!(rest, "button.body(NONE)")?,
        primary_border: outline!(primary, "button.body(NONE; variant=Primary)")?,
        hover_border_color: state_line(&hover, "button.body(HOVER).stroke.color")?,
        pressed_border_color: state_line(
            &b.body(WidgetState::PRESSED),
            "button.body(PRESSED).stroke.color",
        )?,
        primary_background: fill(&primary, "button.body(NONE; variant=Primary).fill")?,
        primary_label: text_color(
            &b.label(ButtonQuery::primary()),
            "button.label(NONE; variant=Primary).color",
        )?,
        default_background: state_fill(&default, "button.body(DEFAULT).fill")?,
        default_label: state_text_color(
            &b.label(WidgetState::DEFAULT),
            "button.label(DEFAULT).color",
        )?,
        default_border_color: state_line(&default, "button.body(DEFAULT).stroke.color")?,
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
        hover_border_color: state_line(&hover, "toolbar.button(HOVER).stroke.color")?,
        checked_border_color: state_line(&checked, "toolbar.button(CHECKED).stroke.color")?,
        pressed_border_color: state_line(
            &t.button(WidgetState::PRESSED),
            "toolbar.button(PRESSED).stroke.color",
        )?,
        checked_hover_border_color: state_line(
            &t.button(WidgetState::HOVER | WidgetState::CHECKED),
            "toolbar.button(HOVER|CHECKED).stroke.color",
        )?,
        checked_pressed_border_color: state_line(
            &t.button(WidgetState::PRESSED | WidgetState::HOVER | WidgetState::CHECKED),
            "toolbar.button(PRESSED|HOVER|CHECKED).stroke.color",
        )?,
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
/// `colors.disabled_opacity`, the link stating none of its own. A hover
/// fill no `HOVER` rule states is iced's own text button's.
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
        hover_background: state_fill(
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

/// [`InputSource`] of the 0.8.0 model, of the field `$w` of `$r` (`input`
/// or `text_area`, each its own widget), every home under `$home`, and the
/// focus indicator.
macro_rules! input_source {
    ($r:expr, $w:ident, $home:literal) => {{
        let r: &ResolvedTheme = $r;
        let i = &r.$w;
        let ring = &r.focus_ring;
        let rest = i.field(WidgetState::NONE);
        Ok::<InputSource, Unmapped>(InputSource {
            background: fill(&rest, concat!($home, ".field(NONE).fill"))?,
            disabled_background: state_fill(
                &i.field(WidgetState::DISABLED),
                concat!($home, ".field(DISABLED).fill"),
            )?,
            text: text_color(
                &i.text(WidgetState::NONE),
                concat!($home, ".text(NONE).color"),
            )?,
            disabled_text: text_color(
                &i.text(WidgetState::DISABLED),
                concat!($home, ".text(DISABLED).color"),
            )?,
            disabled_opacity: i.disabled_opacity,
            border: outline!(rest, concat!($home, ".field(NONE)"))?,
            hover_border_color: state_line(
                &i.field(WidgetState::HOVER),
                concat!($home, ".field(HOVER).stroke.color"),
            )?,
            focus_border_color: state_line(
                &i.field(WidgetState::FOCUSED),
                concat!($home, ".field(FOCUSED).stroke.color"),
            )?,
            focus_ring_color: to_color(r.colors.focus_ring),
            focus_ring_width: match &ring.stroke {
                Stroke::None => 0.0,
                Stroke::Uniform { width, .. } => *width,
                _ => return Err(Unmapped("focus_ring.stroke.width")),
            },
            focus_ring_offset: ring.offset,
            placeholder: text_color(
                &i.placeholder(WidgetState::NONE),
                concat!($home, ".placeholder(NONE).color"),
            )?,
            selection: to_color(r.colors.text_selection.background),
        })
    }};
}

/// [`crate::styles::text_input`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the field needs.
pub fn text_input(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::text_input::Status) -> iced_widget::text_input::Style + Clone + use<>,
> {
    Ok(old::text_input_from(&input_source!(r, input, "input")?))
}

/// [`crate::styles::text_editor`]: the text area's own colours and border.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no value the field needs.
pub fn text_editor(
    r: &ResolvedTheme,
) -> Mapped<
    impl Fn(&Theme, iced_widget::text_editor::Status) -> iced_widget::text_editor::Style + Clone + use<>,
> {
    Ok(old::text_editor_from(&input_source!(
        r,
        text_area,
        "text_area"
    )?))
}

/// [`crate::styles::progress_bar`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no track or fill colour.
pub fn progress_bar(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::progress_bar::Style + Clone + use<>> {
    let p = &r.progress_bar;
    let track = p.track(WidgetState::NONE);
    Ok(old::progress_bar_from(
        fill(&track, "progress_bar.track(NONE).fill")?,
        fill(&p.fill(WidgetState::NONE), "progress_bar.fill(NONE).fill")?,
        outline!(track, "progress_bar.track(NONE)")?,
    ))
}

/// [`crate::styles::rule`]: the separator's colour.
#[must_use = "this returns the style function; it does not apply it"]
pub fn rule(r: &ResolvedTheme) -> impl Fn(&Theme) -> iced_widget::rule::Style + Clone + use<> {
    old::rule_from(to_color(r.separator.color))
}

/// [`crate::styles::toolbar_rule`]: the toolbar's stroke colour as stated.
/// The rule is a line of its own under the bar, not a border over the bar's
/// fill, so no fill is composited under it.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no toolbar stroke of one
/// colour.
pub fn toolbar_rule(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::rule::Style + Clone + use<>> {
    let bar = r.toolbar.bar(WidgetState::NONE);
    let line = match &bar.stroke {
        Stroke::Uniform { color, .. } if bar.stated.contains(PartStated::STROKE) => *color,
        _ => return Err(Unmapped("toolbar.bar(NONE).stroke.color")),
    };
    Ok(old::rule_from(to_color(line)))
}

/// [`crate::styles::pane_grid`]: the splitter's handle under the pointer
/// and its divider width.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no hovered handle fill.
pub fn pane_grid(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::pane_grid::Style + Clone + use<>> {
    let s = &r.splitter;
    Ok(old::pane_grid_from(
        fill(&s.handle(WidgetState::HOVER), "splitter.handle(HOVER).fill")?,
        s.divider_width,
    ))
}

/// [`crate::styles::tooltip`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no panel fill or text colour.
pub fn tooltip(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let t = &r.tooltip;
    Ok(old::panel_from(
        fill(&t.panel, "tooltip.panel.fill")?,
        Some(text_color(&t.text, "tooltip.text.color")?),
        Some(outline!(t.panel, "tooltip.panel")?),
    ))
}

/// [`crate::styles::container_card`]. The card's border is iced's own where
/// the theme states none (the table of homes, `card.border.*`:
/// `unstated_if_unset`).
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no card fill.
pub fn container_card(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let card = r.card.container(WidgetState::NONE);
    Ok(old::panel_from(
        fill(&card, "card.container(NONE).fill")?,
        None,
        Some(outline_or_iced!(card, "card.container(NONE)")?),
    ))
}

/// [`crate::styles::dialog`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no panel fill or body colour.
pub fn dialog(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let d = &r.dialog;
    Ok(old::panel_from(
        fill(&d.panel, "dialog.panel.fill")?,
        Some(text_color(&d.body, "dialog.body.color")?),
        Some(outline!(d.panel, "dialog.panel")?),
    ))
}

/// [`crate::styles::toolbar`].
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no bar fill or button label
/// colour.
pub fn toolbar(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<>> {
    let t = &r.toolbar;
    Ok(old::panel_from(
        fill(&t.bar(WidgetState::NONE), "toolbar.bar(NONE).fill")?,
        Some(text_color(
            &t.button_label(WidgetState::NONE),
            "toolbar.button_label(NONE).color",
        )?),
        None::<Outline>,
    ))
}

/// [`crate::styles::application`]: the fill of a normal window's frame.
///
/// # Errors
///
/// [`super::Unmapped`] where the theme states no frame fill.
pub fn application(
    r: &ResolvedTheme,
) -> Mapped<impl Fn(&Theme) -> iced_core::theme::Style + Clone + use<>> {
    Ok(old::application_from(fill(
        &r.window.frame(WindowState::default()),
        "window.frame(normal).fill",
    )?))
}

#[cfg(test)]
mod tests;
