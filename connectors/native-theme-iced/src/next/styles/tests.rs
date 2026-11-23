//! The 0.8.0 style functions on themes built for the case: a stated value
//! the presets do not show, or one they leave unstated.

use std::sync::Arc;

use iced_core::Background;
use iced_core::Color as IcedColor;
use native_theme::next::{
    ButtonRule, ButtonVariant, CheckboxRule, Color, InputRule, LinkRule, MenuRule, Paint,
    PaintLayer, PartPatch, PartStated, Preset, RadioRule, ResolvedTheme, ShadowLayer, SliderRule,
    Stroke, StrokeAlign, TextPatch, TextStated, Theme, ThemeContext, ThemePatch, ToolbarRule,
    WidgetState,
};

use super::super::borders::line;
use super::super::{fill, text_color, to_color, to_theme};
use crate::styles::ICED_DISABLED_BUTTON_ALPHA;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// The presets, each scheme, each with `patch` laid over it.
fn every_preset_with(patch: &ThemePatch) -> Vec<(Preset, ResolvedTheme)> {
    crate::next::parity::PRESETS
        .iter()
        .map(|&(_, p)| p)
        .flat_map(|p| {
            [ThemeContext::light(), ThemeContext::dark()]
                .map(|ctx| (p, Theme::preset(p).with_patch(patch.clone()).resolve(&ctx)))
        })
        .collect()
}

const LABEL: Color = Color::new(0.1, 0.2, 0.3, 1.0);

/// A part patch that states a fill and a stroke for every state, which
/// leaves no state a rule of its own for either (spec §3, the rule engine: a
/// later matching rule with no required states clears the `from_state`
/// bit).
fn every_state() -> PartPatch {
    PartPatch::new()
        .fill(Paint::Solid(DISTINCT))
        .stroke(Stroke::uniform(1.0, LABEL))
}

/// The presets, each scheme, as the parity test covers them.
fn every_preset() -> Vec<(Preset, ResolvedTheme)> {
    crate::next::parity::PRESETS
        .iter()
        .map(|&(_, p)| p)
        .flat_map(|p| {
            [ThemeContext::light(), ThemeContext::dark()]
                .map(|ctx| (p, Theme::preset(p).resolve(&ctx)))
        })
        .collect()
}

fn background_color(b: Option<Background>) -> Option<IcedColor> {
    match b {
        Some(Background::Color(c)) => Some(c),
        _ => None,
    }
}

/// A button whose theme states no disabled opacity draws each disabled
/// colour the theme states as stated, and fades only the ones it leaves to
/// iced, as iced's own classes do; its stroke is laid over the fill so
/// painted.
#[test]
fn a_button_with_no_disabled_opacity_halves_only_what_the_theme_leaves_to_iced() -> TestResult {
    let (mut stated, mut left) = (0, 0);
    for (preset, mut r) in every_preset() {
        r.button.disabled_opacity = None;
        let theme = to_theme(&r)?;
        let style = super::button(&r)?(&theme, iced_widget::button::Status::Disabled);

        let body = r.button.body(WidgetState::DISABLED);
        let expected_fill = if body.from_state.contains(PartStated::FILL) {
            stated += 1;
            fill(&body, "fill")?
        } else {
            left += 1;
            fill(&r.button.body(WidgetState::NONE), "fill")?.scale_alpha(ICED_DISABLED_BUTTON_ALPHA)
        };
        let label = r.button.label(WidgetState::DISABLED);
        let expected_label = if label.from_state.contains(TextStated::COLOR) {
            text_color(&label, "label")?
        } else {
            text_color(&r.button.label(WidgetState::NONE), "label")?
                .scale_alpha(ICED_DISABLED_BUTTON_ALPHA)
        };
        let stroke = line(&r.button.body(WidgetState::NONE), "stroke")?;

        assert_eq!(
            background_color(style.background),
            Some(expected_fill),
            "{preset:?}"
        );
        assert_eq!(style.text_color, expected_label, "{preset:?}");
        assert_eq!(
            style.border.color,
            stroke.over(expected_fill),
            "{preset:?}: the stroke over the fill painted"
        );
    }
    assert!(
        stated > 0 && left > 0,
        "both cases covered: {stated} stated, {left} left to iced"
    );
    Ok(())
}

/// A disabled toolbar button whose theme states an opacity and no disabled
/// label fades the label at rest by that opacity, as the plain button does.
#[test]
fn a_toolbar_button_fades_its_label_by_a_stated_opacity() -> TestResult {
    const OPACITY: f32 = 0.5;
    for (preset, r) in every_preset() {
        let mut b = super::button_source(&r)?;
        b.disabled_label = None;
        b.disabled_opacity = Some(OPACITY);
        let t = super::toolbar_button_source(&r)?;
        let theme = to_theme(&r)?;
        let style = crate::styles::toolbar_button_from(&b, &t, false)(
            &theme,
            iced_widget::button::Status::Disabled,
        );
        assert_eq!(
            style.text_color,
            crate::styles::faded(b.label, OPACITY),
            "{preset:?}"
        );
    }
    Ok(())
}

/// A field whose theme states no disabled opacity is drawn in the disabled
/// colours the theme states, not faded again.
#[test]
fn a_field_with_no_disabled_opacity_is_drawn_as_stated() -> TestResult {
    for (preset, mut r) in every_preset() {
        r.input.disabled_opacity = None;
        let theme = to_theme(&r)?;
        let style = super::text_input(&r)?(&theme, iced_widget::text_input::Status::Disabled);

        let field = r.input.field(WidgetState::DISABLED);
        let expected_fill = if field.from_state.contains(PartStated::FILL) {
            fill(&field, "fill")?
        } else {
            fill(&r.input.field(WidgetState::NONE), "fill")?
        };
        assert_eq!(
            style.background,
            Background::Color(expected_fill),
            "{preset:?}"
        );
        assert_eq!(
            style.value,
            text_color(&r.input.text(WidgetState::DISABLED), "text")?,
            "{preset:?}"
        );
    }
    Ok(())
}

const DISTINCT: Color = Color::new(0.25, 0.5, 0.75, 1.0);

/// The editor reads the text area's own field, which may differ from the
/// single-line field's.
#[test]
fn the_text_editor_reads_the_text_area_not_the_input() -> TestResult {
    let patch = ThemePatch::new()
        .text_area(InputRule::new().field(PartPatch::new().fill(Paint::Solid(DISTINCT))));
    let r = Theme::preset(Preset::KdeBreeze)
        .with_patch(patch)
        .resolve(&ThemeContext::light());
    let theme = to_theme(&r)?;
    let editor = super::text_editor(&r)?(&theme, iced_widget::text_editor::Status::Active);
    let input = super::text_input(&r)?(&theme, iced_widget::text_input::Status::Active);
    assert_eq!(editor.background, Background::Color(to_color(DISTINCT)));
    assert_ne!(input.background, editor.background);
    Ok(())
}

/// A layered fill is drawn in the colour a one-colour toolkit renders in
/// its place: the topmost opaque layer.
#[test]
fn a_layered_fill_is_drawn_in_its_fallback() -> TestResult {
    let translucent = Color::new(1.0, 1.0, 1.0, 0.5);
    let layers = Paint::layers(vec![
        PaintLayer::Solid {
            color: DISTINCT,
            blend: Default::default(),
        },
        PaintLayer::Solid {
            color: translucent,
            blend: Default::default(),
        },
    ]);
    let patch = ThemePatch::new().input(InputRule::new().field(PartPatch::new().fill(layers)));
    let r = Theme::preset(Preset::Adwaita)
        .with_patch(patch)
        .resolve(&ThemeContext::light());
    let theme = to_theme(&r)?;
    let style = super::text_input(&r)?(&theme, iced_widget::text_input::Status::Active);
    assert_eq!(style.background, Background::Color(to_color(DISTINCT)));
    Ok(())
}

/// A stated `Paint::None` is no fill, not an unmapped one.
#[test]
fn a_stated_no_fill_is_drawn_as_nothing() -> TestResult {
    let patch = ThemePatch::new().input(InputRule::new().field(PartPatch::new().fill(Paint::None)));
    let r = Theme::preset(Preset::Adwaita)
        .with_patch(patch)
        .resolve(&ThemeContext::light());
    let theme = to_theme(&r)?;
    let style = super::text_input(&r)?(&theme, iced_widget::text_input::Status::Active);
    assert_eq!(style.background, Background::Color(IcedColor::TRANSPARENT));
    Ok(())
}

/// A stated `Paint::None` for a state paints nothing in that state, not the
/// fill at rest under a transparent layer.
#[test]
fn a_stated_no_fill_for_a_state_paints_nothing() -> TestResult {
    use iced_widget::button::Status;
    let none = PartPatch::new().fill(Paint::None);
    let patch = ThemePatch::new()
        .button(
            ButtonRule::new()
                .states(WidgetState::HOVER)
                .body(none.clone()),
        )
        .link(LinkRule::new().states(WidgetState::HOVER).container(none));
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let rest = fill(&r.button.body(WidgetState::NONE), "fill")?;
        assert_ne!(rest, IcedColor::TRANSPARENT, "{preset:?}");
        for style in [
            super::button(&r)?(&theme, Status::Hovered),
            super::button_link(&r)?(&theme, Status::Hovered),
        ] {
            assert_eq!(
                background_color(style.background),
                Some(IcedColor::TRANSPARENT),
                "{preset:?}"
            );
        }
    }
    Ok(())
}

/// A hovered or pressed button whose theme states no fill and no label for
/// the state is iced's own neutral button for the status, fill and label;
/// a checked toggle that states no checked look is that pressed button, and
/// a toolbar button with no hovered or pressed look of its own or of the
/// button's is too.
#[test]
fn an_unstated_state_is_iceds_secondary_button() -> TestResult {
    use iced_widget::button::Status;
    let patch = ThemePatch::new()
        .button(
            ButtonRule::new()
                .body(every_state())
                .label(TextPatch::new().color(LABEL)),
        )
        .toolbar(ToolbarRule::new().button(every_state()));
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let plain = super::button(&r)?;
        let toggle = super::button_toggle(&r, true)?;
        let toolbar = super::toolbar_button(&r, false)?;
        for status in [Status::Hovered, Status::Pressed] {
            let own = iced_widget::button::secondary(&theme, status);
            let style = plain(&theme, status);
            assert_eq!(style.background, own.background, "{preset:?} {status:?}");
            assert_eq!(style.text_color, own.text_color, "{preset:?} {status:?}");
            let style = toolbar(&theme, status);
            assert_eq!(
                background_color(style.background),
                background_color(own.background),
                "{preset:?} toolbar {status:?}"
            );
            assert_eq!(
                style.text_color, own.text_color,
                "{preset:?} toolbar {status:?}"
            );
        }
        let pressed = iced_widget::button::secondary(&theme, Status::Pressed);
        for status in [Status::Active, Status::Hovered, Status::Pressed] {
            let style = toggle(&theme, status);
            assert_eq!(
                background_color(style.background),
                background_color(pressed.background),
                "{preset:?} checked {status:?}"
            );
            assert_eq!(
                style.text_color, pressed.text_color,
                "{preset:?} checked {status:?}"
            );
        }
    }
    Ok(())
}

/// A hovered or pressed link whose theme states no fill and no label for
/// the state is iced's own text button for the status.
#[test]
fn an_unstated_state_is_iceds_text_button() -> TestResult {
    use iced_widget::button::Status;
    let patch = ThemePatch::new().link(
        LinkRule::new()
            .container(PartPatch::new().fill(Paint::Solid(DISTINCT)))
            .label(TextPatch::new().color(LABEL)),
    );
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let link = super::button_link(&r)?;
        for status in [Status::Hovered, Status::Pressed] {
            let own = iced_widget::button::text(&theme, status);
            let style = link(&theme, status);
            assert_eq!(style.background, own.background, "{preset:?} {status:?}");
            assert_eq!(style.text_color, own.text_color, "{preset:?} {status:?}");
        }
    }
    Ok(())
}

/// A hovered check box or radio whose theme states no fill, stroke or label
/// for the state is iced's own for the status: fill, border colour, label.
#[test]
fn an_unstated_hover_is_iceds_check_box_and_radio() -> TestResult {
    let label = TextPatch::new().color(LABEL);
    let patch = ThemePatch::new()
        .checkbox(
            CheckboxRule::new()
                .indicator(every_state())
                .label(label.clone()),
        )
        .radio(RadioRule::new().indicator(every_state()).label(label));
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let status = iced_widget::checkbox::Status::Hovered { is_checked: false };
        let own = iced_widget::checkbox::primary(&theme, status);
        let style = super::checkbox(&r)?(&theme, status);
        assert_eq!(style.background, own.background, "{preset:?}");
        assert_eq!(style.border.color, own.border.color, "{preset:?}");
        assert_eq!(style.text_color, own.text_color, "{preset:?}");

        let status = iced_widget::radio::Status::Hovered { is_selected: false };
        let own = iced_widget::radio::default(&theme, status);
        let style = super::radio(&r)?(&theme, status);
        assert_eq!(style.background, own.background, "{preset:?}");
        assert_eq!(style.border_color, own.border_color, "{preset:?}");
        assert_eq!(style.text_color, own.text_color, "{preset:?}");
    }
    Ok(())
}

/// The disabled fill and label a button with no disabled opacity shows:
/// the disabled fill the theme states, with its disabled label or the label
/// at rest faded as iced fades it; or, where it states no disabled fill,
/// `idle` and `idle_label` faded as iced fades them.
fn disabled_without_opacity(
    r: &ResolvedTheme,
    idle: IcedColor,
    idle_label: IcedColor,
) -> Result<(IcedColor, IcedColor, bool), super::Unmapped> {
    let body = r.button.body(WidgetState::DISABLED);
    if !body.from_state.contains(PartStated::FILL) {
        return Ok((
            idle.scale_alpha(ICED_DISABLED_BUTTON_ALPHA),
            idle_label.scale_alpha(ICED_DISABLED_BUTTON_ALPHA),
            false,
        ));
    }
    let label = r.button.label(WidgetState::DISABLED);
    let label = if label.from_state.contains(TextStated::COLOR) {
        text_color(&label, "label")?
    } else {
        text_color(&r.button.label(WidgetState::NONE), "label")?
            .scale_alpha(ICED_DISABLED_BUTTON_ALPHA)
    };
    Ok((fill(&body, "fill")?, label, true))
}

/// A status button and a checked toggle whose theme states no disabled
/// opacity draw each disabled colour the theme states as stated and fade
/// only the ones it leaves to iced, as iced's own classes do.
#[test]
fn a_class_button_and_a_checked_toggle_with_no_disabled_opacity_halve_what_is_left_to_iced()
-> TestResult {
    use iced_widget::button::Status;
    let (mut stated, mut left) = (0, 0);
    for (preset, mut r) in every_preset() {
        r.button.disabled_opacity = None;
        let theme = to_theme(&r)?;

        let role = &r.colors.error;
        let style = super::button_danger(&r)?(&theme, Status::Disabled);
        let (fill, label, is_stated) =
            disabled_without_opacity(&r, to_color(role.fill), to_color(role.foreground))?;
        assert_eq!(background_color(style.background), Some(fill), "{preset:?}");
        assert_eq!(style.text_color, label, "{preset:?}");

        let toggle = super::button_toggle(&r, true)?;
        let checked = toggle(&theme, Status::Active);
        let checked_fill = background_color(checked.background).ok_or("no checked fill")?;
        let (fill, label, _) = disabled_without_opacity(&r, checked_fill, checked.text_color)?;
        let style = toggle(&theme, Status::Disabled);
        assert_eq!(
            background_color(style.background),
            Some(fill),
            "{preset:?} checked"
        );
        assert_eq!(style.text_color, label, "{preset:?} checked");

        if is_stated {
            stated += 1;
        } else {
            left += 1;
        }
    }
    assert!(
        stated > 0 && left > 0,
        "both cases covered: {stated} stated, {left} left to iced"
    );
    Ok(())
}

/// The menu's shadow is the panel's first outset layer, its spread and the
/// other layers dropped; a stated none is no shadow, and an unstated one
/// iced's own.
#[test]
fn a_menu_shadow_is_its_first_outset_layer() -> TestResult {
    use iced_core::{Shadow, Vector};
    let layer = |inset, offset_y, spread| ShadowLayer {
        offset_x: 1.0,
        offset_y,
        blur: 8.0,
        spread,
        color: LABEL,
        inset,
    };
    let shadow_of = |shadows: Option<Arc<[ShadowLayer]>>| -> Result<Shadow, super::Unmapped> {
        let patch =
            ThemePatch::new().menu(MenuRule::new().panel(PartPatch::new().shadows(shadows)));
        let r = Theme::preset(Preset::Material3)
            .with_patch(patch)
            .resolve(&ThemeContext::light());
        Ok(super::menu(&r)?(&to_theme(&r)?).shadow)
    };
    let expected = |offset_y| Shadow {
        color: to_color(LABEL),
        offset: Vector::new(1.0, offset_y),
        blur_radius: 8.0,
    };
    assert_eq!(
        shadow_of(Some(Arc::from([layer(false, 2.0, 0.0)])))?,
        expected(2.0)
    );
    assert_eq!(
        shadow_of(Some(Arc::from([
            layer(true, 1.0, 0.0),
            layer(false, 4.0, 3.0),
            layer(false, 6.0, 0.0),
        ])))?,
        expected(4.0)
    );
    assert_eq!(shadow_of(None)?, Shadow::default());

    let r = Theme::preset(Preset::Material3).resolve(&ThemeContext::light());
    let theme = to_theme(&r)?;
    assert_eq!(
        super::menu(&r)?(&theme).shadow,
        iced_widget::overlay::menu::default(&theme).shadow
    );
    Ok(())
}

/// The menu's own panel stroke frames the popup where the theme states one,
/// laid over the panel's fill; the popover's frames it where not.
#[test]
fn a_menu_is_framed_by_its_own_stroke_where_stated() -> TestResult {
    let translucent = Color::new(0.0, 0.0, 0.0, 0.25);
    let patch = ThemePatch::new().menu(MenuRule::new().panel(PartPatch::new().stroke(
        Stroke::uniform_aligned(2.0, translucent, StrokeAlign::Inside),
    )));
    for (preset, r) in every_preset_with(&patch) {
        let style = super::menu(&r)?(&to_theme(&r)?);
        let panel = fill(&r.menu.panel(WidgetState::NONE), "fill")?;
        assert_eq!(style.border.width, 2.0, "{preset:?}");
        assert_eq!(
            style.border.color,
            crate::styles::composite_over(to_color(translucent), panel),
            "{preset:?}"
        );
    }
    Ok(())
}

/// A dragged thumb is the pressed fill the theme states, laid over the
/// thumb's own; a translucent thumb stroke inside the thumb is laid over
/// the fill each status paints.
#[test]
fn the_slider_thumb_takes_its_pressed_fill_and_its_aligned_stroke() -> TestResult {
    use iced_widget::slider::Status;
    let layer = Color::new(0.0, 0.0, 0.0, 0.5);
    let translucent = Color::new(0.0, 0.0, 0.0, 0.25);
    let patch = ThemePatch::new()
        .slider(
            SliderRule::new().thumb(PartPatch::new().stroke(Stroke::uniform_aligned(
                1.0,
                translucent,
                StrokeAlign::Inside,
            ))),
        )
        .slider(
            SliderRule::new()
                .states(WidgetState::PRESSED)
                .thumb(PartPatch::new().fill(Paint::Solid(layer))),
        );
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let slider = super::slider(&r)?;
        let thumb = fill(&r.slider.thumb(WidgetState::NONE), "fill")?;
        let pressed = crate::styles::composite_over(to_color(layer), thumb);
        for (status, under) in [(Status::Active, thumb), (Status::Dragged, pressed)] {
            let style = slider(&theme, status);
            assert_eq!(
                style.handle.background,
                Background::Color(under),
                "{preset:?} {status:?}"
            );
            assert_eq!(style.handle.border_width, 1.0, "{preset:?} {status:?}");
            assert_eq!(
                style.handle.border_color,
                crate::styles::composite_over(to_color(translucent), under),
                "{preset:?} {status:?}"
            );
        }
    }
    Ok(())
}

/// The primary button's hovered and pressed outline is read at the
/// variant's own query, not the plain button's.
#[test]
fn the_primary_button_reads_its_own_hovered_and_pressed_outline() -> TestResult {
    use iced_widget::button::Status;
    let opaque = Color::new(0.9, 0.1, 0.1, 1.0);
    for (flag, status) in [
        (WidgetState::HOVER, Status::Hovered),
        (WidgetState::PRESSED, Status::Pressed),
    ] {
        let patch = ThemePatch::new().button(
            ButtonRule::new()
                .variant(ButtonVariant::Primary)
                .states(flag)
                .body(PartPatch::new().stroke(Stroke::uniform(1.0, opaque))),
        );
        for (preset, r) in every_preset_with(&patch) {
            let theme = to_theme(&r)?;
            let primary = super::button_primary(&r)?(&theme, status);
            assert_eq!(
                primary.border.color,
                to_color(opaque),
                "{preset:?} {status:?}"
            );
            let plain = super::button(&r)?(&theme, status);
            assert_ne!(
                plain.border.color,
                to_color(opaque),
                "{preset:?} {status:?}"
            );
        }
    }
    Ok(())
}
