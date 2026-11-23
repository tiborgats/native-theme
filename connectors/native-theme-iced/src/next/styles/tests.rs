//! The 0.8.0 style functions on themes built for the case: a stated value
//! the presets do not show, or one they leave unstated.

use std::sync::Arc;

use iced_core::Background;
use iced_core::Color as IcedColor;
use native_theme::next::{
    ButtonRule, ButtonVariant, CheckboxRule, Color, InputRule, LinkRule, MenuRule, Paint,
    PaintLayer, PartPatch, PartStated, Preset, RadioRule, ResolvedTheme, ScrollbarRule,
    SegmentedControlRule, ShadowLayer, SliderRule, Stroke, StrokeAlign, SwitchRule, TextPatch,
    TextStated, Theme, ThemeContext, ThemePatch, ToolbarRule, WidgetState,
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
        .link(LinkRule::new().container(PartPatch::new().fill(Paint::Solid(DISTINCT))))
        .link(LinkRule::new().states(WidgetState::HOVER).container(none));
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let rest = fill(&r.button.body(WidgetState::NONE), "fill")?;
        assert_ne!(rest, IcedColor::TRANSPARENT, "{preset:?}");
        let link_rest = fill(&r.link.container(WidgetState::NONE), "fill")?;
        assert_eq!(link_rest, to_color(DISTINCT), "{preset:?}");
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
/// the state takes, of iced's own neutral button, what iced changes for the
/// status (iced_widget 0.14.2 `button::secondary`: the hovered fill, and
/// nothing pressed), and keeps the theme's value at rest for the rest; a
/// checked toggle that states no checked look is that pressed button, and a
/// toolbar button with no hovered or pressed look of its own or of the
/// button's is flat where iced makes no change.
#[test]
fn an_unstated_state_is_iceds_change_to_the_secondary_button() -> TestResult {
    use iced_widget::button::Status;
    let patch = ThemePatch::new()
        .button(
            ButtonRule::new()
                .body(every_state())
                .label(TextPatch::new().color(LABEL)),
        )
        .toolbar(ToolbarRule::new().button(every_state()));
    let fill = Some(Background::Color(to_color(DISTINCT)));
    let label = to_color(LABEL);
    let mut changed = 0;
    // The connector's iced theme gives the same strong and base secondary
    // fill on every preset; iced's own light theme does not.
    let themes = every_preset_with(&patch)
        .into_iter()
        .map(|(preset, r)| Ok((preset, to_theme(&r)?, r)))
        .chain(
            every_preset_with(&patch)
                .into_iter()
                .map(|(preset, r)| Ok((preset, iced_core::theme::Theme::Light, r))),
        )
        .collect::<Result<Vec<_>, super::super::Unmapped>>()?;
    for (preset, theme, r) in themes {
        let plain = super::button(&r)?;
        let toggle = super::button_toggle(&r, true)?;
        let toolbar = super::toolbar_button(&r, false)?;
        let rest = iced_widget::button::secondary(&theme, Status::Active);
        let hovered = iced_widget::button::secondary(&theme, Status::Hovered);
        let pressed = iced_widget::button::secondary(&theme, Status::Pressed);
        assert_eq!(pressed.background, rest.background, "{preset:?}");
        assert_eq!(hovered.text_color, rest.text_color, "{preset:?}");
        // Where iced's palette gives the same strong and base secondary
        // fill, iced makes no change under the pointer either.
        let iced_changes = hovered.background != rest.background;
        changed += usize::from(iced_changes);

        let style = plain(&theme, Status::Hovered);
        let expected = if iced_changes {
            hovered.background
        } else {
            fill
        };
        assert_eq!(style.background, expected, "{preset:?}");
        assert_eq!(style.text_color, label, "{preset:?}");
        let style = plain(&theme, Status::Pressed);
        assert_eq!(style.background, fill, "{preset:?}");
        assert_eq!(style.text_color, label, "{preset:?}");

        let style = toolbar(&theme, Status::Hovered);
        let expected = if iced_changes {
            background_color(hovered.background)
        } else {
            Some(IcedColor::TRANSPARENT)
        };
        assert_eq!(
            background_color(style.background),
            expected,
            "{preset:?} toolbar"
        );
        assert_eq!(style.text_color, label, "{preset:?} toolbar");
        let style = toolbar(&theme, Status::Pressed);
        assert_eq!(
            background_color(style.background),
            Some(IcedColor::TRANSPARENT),
            "{preset:?} toolbar"
        );
        assert_eq!(style.text_color, label, "{preset:?} toolbar");

        for status in [Status::Active, Status::Hovered, Status::Pressed] {
            let style = toggle(&theme, status);
            assert_eq!(style.background, fill, "{preset:?} checked {status:?}");
            assert_eq!(style.text_color, label, "{preset:?} checked {status:?}");
        }
    }
    assert!(changed > 0, "no preset shows iced's hovered change");
    Ok(())
}

/// A hovered or pressed link whose theme states no fill and no label for
/// the state takes, of iced's own text button, what iced changes for the
/// status (iced_widget 0.14.2 `button::text`: the hovered label, and
/// nothing pressed), and keeps its fill and label at rest for the rest.
#[test]
fn an_unstated_state_is_iceds_change_to_the_text_button() -> TestResult {
    use iced_widget::button::Status;
    let patch = ThemePatch::new().link(
        LinkRule::new()
            .container(PartPatch::new().fill(Paint::Solid(DISTINCT)))
            .label(TextPatch::new().color(LABEL)),
    );
    let fill = Some(Background::Color(to_color(DISTINCT)));
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let link = super::button_link(&r)?;
        let rest = iced_widget::button::text(&theme, Status::Active);
        let hovered = iced_widget::button::text(&theme, Status::Hovered);
        assert_ne!(hovered.text_color, rest.text_color, "{preset:?}");
        assert_eq!(hovered.background, rest.background, "{preset:?}");

        let style = link(&theme, Status::Hovered);
        assert_eq!(style.background, fill, "{preset:?}");
        assert_eq!(style.text_color, hovered.text_color, "{preset:?}");
        let style = link(&theme, Status::Pressed);
        assert_eq!(style.background, fill, "{preset:?}");
        assert_eq!(style.text_color, to_color(LABEL), "{preset:?}");
    }
    Ok(())
}

/// A hovered check box or radio whose theme states no fill, stroke or label
/// for the state takes, of iced's own, what iced changes for the status
/// (iced_widget 0.14.2 `checkbox::primary`, `radio::default`: the
/// unchecked fill, and neither the unchecked border nor the label), and
/// keeps the theme's value at rest for the rest.
#[test]
fn an_unstated_hover_is_iceds_change_to_the_check_box_and_radio() -> TestResult {
    let label = TextPatch::new().color(LABEL);
    let patch = ThemePatch::new()
        .checkbox(
            CheckboxRule::new()
                .indicator(every_state())
                .label(label.clone()),
        )
        .radio(RadioRule::new().indicator(every_state()).label(label));
    let mut changed = 0;
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let rest = iced_widget::checkbox::primary(
            &theme,
            iced_widget::checkbox::Status::Active { is_checked: false },
        );
        let status = iced_widget::checkbox::Status::Hovered { is_checked: false };
        let own = iced_widget::checkbox::primary(&theme, status);
        assert_eq!(own.border.color, rest.border.color, "{preset:?}");
        // Where iced's palette gives the same weak and base background,
        // iced makes no change to the fill either.
        let iced_changes = own.background != rest.background;
        changed += usize::from(iced_changes);
        let style = super::checkbox(&r)?(&theme, status);
        let expected = if iced_changes {
            own.background
        } else {
            Background::Color(to_color(DISTINCT))
        };
        assert_eq!(style.background, expected, "{preset:?}");
        assert_eq!(style.border.color, to_color(LABEL), "{preset:?}");
        assert_eq!(style.text_color, Some(to_color(LABEL)), "{preset:?}");

        let rest = iced_widget::radio::default(
            &theme,
            iced_widget::radio::Status::Active { is_selected: false },
        );
        let status = iced_widget::radio::Status::Hovered { is_selected: false };
        let own = iced_widget::radio::default(&theme, status);
        assert_ne!(own.background, rest.background, "{preset:?}");
        assert_eq!(own.border_color, rest.border_color, "{preset:?}");
        let style = super::radio(&r)?(&theme, status);
        assert_eq!(style.background, own.background, "{preset:?}");
        assert_eq!(style.border_color, to_color(LABEL), "{preset:?}");
        assert_eq!(style.text_color, Some(to_color(LABEL)), "{preset:?}");
    }
    assert!(changed > 0, "no preset shows iced's hovered change");
    Ok(())
}

/// A rule for `CHECKED` alone states nothing for the hovered state: a
/// hovered checked box's fill and border are iced's own, which iced changes
/// under the pointer (iced_widget 0.14.2 `checkbox::primary`: `primary.base`
/// to `primary.strong`), and a hovered switch's on track is the checked
/// track the rule states, which iced's own toggler does not change.
#[test]
fn a_checked_rule_alone_leaves_the_hovered_state_to_iceds_change() -> TestResult {
    let checked = PartPatch::new()
        .fill(Paint::Solid(LABEL))
        .stroke(Stroke::uniform(1.0, LABEL));
    let patch = ThemePatch::new()
        .checkbox(CheckboxRule::new().indicator(every_state()))
        .checkbox(
            CheckboxRule::new()
                .states(WidgetState::CHECKED)
                .indicator(checked.clone()),
        )
        .switch(SwitchRule::new().track(every_state()))
        .switch(
            SwitchRule::new()
                .states(WidgetState::CHECKED)
                .track(checked),
        );
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let rest = iced_widget::checkbox::primary(
            &theme,
            iced_widget::checkbox::Status::Active { is_checked: true },
        );
        let status = iced_widget::checkbox::Status::Hovered { is_checked: true };
        let own = iced_widget::checkbox::primary(&theme, status);
        assert_ne!(own.background, rest.background, "{preset:?}");
        assert_ne!(own.border.color, rest.border.color, "{preset:?}");
        let style = super::checkbox(&r)?(&theme, status);
        assert_eq!(style.background, own.background, "{preset:?}");
        assert_eq!(style.border.color, own.border.color, "{preset:?}");

        let status = iced_widget::toggler::Status::Hovered { is_toggled: true };
        let style = super::toggler(&r)?(&theme, status);
        assert_eq!(
            style.background,
            Background::Color(to_color(LABEL)),
            "{preset:?}"
        );
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

/// A scroller under the pointer or dragged is iced's own where no rule
/// states the thumb's fill for the state, and the theme's where one does.
#[test]
fn a_scroller_no_state_rule_states_is_iceds_own() -> TestResult {
    use iced_widget::scrollable::Status;
    let hovered = Status::Hovered {
        is_horizontal_scrollbar_hovered: false,
        is_vertical_scrollbar_hovered: true,
        is_horizontal_scrollbar_disabled: false,
        is_vertical_scrollbar_disabled: false,
    };
    let dragged = Status::Dragged {
        is_horizontal_scrollbar_dragged: false,
        is_vertical_scrollbar_dragged: true,
        is_horizontal_scrollbar_disabled: false,
        is_vertical_scrollbar_disabled: false,
    };
    let base = ThemePatch::new()
        .scrollbar(ScrollbarRule::new().thumb(PartPatch::new().fill(Paint::Solid(DISTINCT))));
    for (preset, r) in every_preset_with(&base) {
        let theme = to_theme(&r)?;
        let style = super::scrollable(&r)?;
        for status in [hovered, dragged] {
            let own = iced_widget::scrollable::default(&theme, status);
            assert_eq!(
                style(&theme, status).vertical_rail.scroller.background,
                own.vertical_rail.scroller.background,
                "{preset:?} {status:?}"
            );
        }
    }
    let stated = base.scrollbar(
        ScrollbarRule::new()
            .states(WidgetState::HOVER)
            .thumb(PartPatch::new().fill(Paint::Solid(LABEL))),
    );
    for (preset, r) in every_preset_with(&stated) {
        let theme = to_theme(&r)?;
        assert_eq!(
            super::scrollable(&r)?(&theme, hovered)
                .vertical_rail
                .scroller
                .background,
            Background::Color(to_color(LABEL)),
            "{preset:?}"
        );
    }
    Ok(())
}

/// The line between two segments is the separator part's fill where the
/// theme states one, and the outline's colour where it does not.
#[test]
fn a_segment_separator_is_the_separator_parts_fill_where_stated() -> TestResult {
    for (preset, r) in every_preset() {
        let style = super::segment_separator(&r, false)?(&to_theme(&r)?);
        let outline = line(&r.segmented_control.track(WidgetState::NONE), "stroke")?;
        assert_eq!(
            background_color(style.background),
            Some(outline.color),
            "{preset:?}"
        );
    }
    let patch = ThemePatch::new().segmented_control(
        SegmentedControlRule::new().separator(PartPatch::new().fill(Paint::Solid(DISTINCT))),
    );
    for (preset, r) in every_preset_with(&patch) {
        let style = super::segment_separator(&r, false)?(&to_theme(&r)?);
        assert_eq!(
            background_color(style.background),
            Some(to_color(DISTINCT)),
            "{preset:?}"
        );
    }
    Ok(())
}

/// A segmented control whose theme states no disabled opacity is drawn
/// disabled in the colours it states.
#[test]
fn a_segmented_control_with_no_disabled_opacity_is_drawn_as_stated() -> TestResult {
    use iced_widget::button::Status;
    for (preset, mut r) in every_preset() {
        r.segmented_control.disabled_opacity = None;
        let theme = to_theme(&r)?;
        assert_eq!(
            super::segmented_control(&r, true)?(&theme),
            super::segmented_control(&r, false)?(&theme),
            "{preset:?}"
        );
        assert_eq!(
            super::segment_separator(&r, true)?(&theme),
            super::segment_separator(&r, false)?(&theme),
            "{preset:?}"
        );
        for selected in [false, true] {
            let segment = super::segment(&r, selected, crate::styles::SegmentPosition::Only)?;
            let disabled = segment(&theme, Status::Disabled);
            let active = segment(&theme, Status::Active);
            assert_eq!(disabled.background, active.background, "{preset:?}");
            assert_eq!(disabled.text_color, active.text_color, "{preset:?}");
        }
    }
    Ok(())
}
