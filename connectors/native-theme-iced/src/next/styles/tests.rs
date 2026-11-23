//! The 0.8.0 style functions on themes built for the case: a stated value
//! the presets do not show, or one they leave unstated.

use std::sync::Arc;

use iced_core::Background;
use iced_core::Color as IcedColor;
use native_theme::next::{
    ButtonRule, ButtonVariant, CheckboxRule, Color, ComboBoxRule, Corners, ExpanderRule, InputRule,
    LinkRule, MenuRule, Paint, PaintLayer, PartPatch, PartStated, Preset, RadioRule, ResolvedTheme,
    ScrollbarRule, SegmentedControlRule, ShadowLayer, SliderRule, Stroke, StrokeAlign, SwitchRule,
    TextPatch, TextStated, Theme, ThemeContext, ThemePatch, ToolbarRule, WidgetState,
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
/// checked toggle that states no checked look is the primary button in the
/// same status (spec §10, rule 0a), and a toolbar button with no hovered or
/// pressed look of its own or of the button's is flat where iced makes no
/// change.
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

        let primary = super::button_primary(&r)?;
        for status in [Status::Active, Status::Hovered, Status::Pressed] {
            let (style, shown) = (toggle(&theme, status), primary(&theme, status));
            assert_eq!(
                style.background, shown.background,
                "{preset:?} checked {status:?}"
            );
            assert_eq!(
                style.text_color, shown.text_color,
                "{preset:?} checked {status:?}"
            );
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

/// A scroller is outlined and rounded as the theme states, its stroke laid
/// over the fill each status paints and a state's own stroke where one is
/// stated; a floating bar hidden at rest draws no stroke; an outline or
/// corners the theme leaves unstated are iced's own, and a pill is iced's.
#[test]
fn a_scroller_is_outlined_and_rounded_as_stated_else_iceds_own() -> TestResult {
    use iced_widget::scrollable::Status;
    let active = Status::Active {
        is_horizontal_scrollbar_disabled: false,
        is_vertical_scrollbar_disabled: false,
    };
    let hovered = Status::Hovered {
        is_horizontal_scrollbar_hovered: false,
        is_vertical_scrollbar_hovered: true,
        is_horizontal_scrollbar_disabled: false,
        is_vertical_scrollbar_disabled: false,
    };
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let rest = r.scrollbar.thumb(WidgetState::UNFOCUSED_VIEW);
        let border = super::scrollable(&r)?(&theme, active)
            .vertical_rail
            .scroller
            .border;
        let own = iced_widget::scrollable::default(&theme, active)
            .vertical_rail
            .scroller
            .border;
        if rest.stated.contains(PartStated::CORNERS) {
            assert_eq!(border.radius, crate::styles::pill(), "{preset:?}");
        } else {
            assert_eq!(border.radius, own.radius, "{preset:?}");
        }
        if !rest.stated.contains(PartStated::STROKE) {
            assert_eq!(
                (border.color, border.width),
                (own.color, own.width),
                "{preset:?}"
            );
        }
    }

    let translucent = Color::new(0.0, 0.0, 0.0, 0.25);
    let opaque = Color::new(0.9, 0.1, 0.1, 1.0);
    let patch = ThemePatch::new()
        .scrollbar(
            ScrollbarRule::new().thumb(
                PartPatch::new()
                    .fill(Paint::Solid(DISTINCT))
                    .stroke(Stroke::uniform_aligned(
                        2.0,
                        translucent,
                        StrokeAlign::Inside,
                    ))
                    .corners(Corners::uniform(3.0)),
            ),
        )
        .scrollbar(
            ScrollbarRule::new()
                .states(WidgetState::HOVER)
                .thumb(PartPatch::new().stroke(Stroke::uniform(2.0, opaque))),
        );
    for (preset, mut r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        r.scrollbar.overlay_mode = false;
        let style = super::scrollable(&r)?;
        let at_rest = style(&theme, active).vertical_rail.scroller.border;
        assert_eq!(
            at_rest.color,
            crate::styles::composite_over(to_color(translucent), to_color(DISTINCT)),
            "{preset:?}"
        );
        assert_eq!(at_rest.width, 2.0, "{preset:?}");
        assert_eq!(
            at_rest.radius,
            iced_core::border::Radius::new(3.0),
            "{preset:?}"
        );
        let under = style(&theme, hovered);
        assert_eq!(
            under.vertical_rail.scroller.border.color,
            to_color(opaque),
            "{preset:?}"
        );
        assert_eq!(under.horizontal_rail.scroller.border, at_rest, "{preset:?}");

        r.scrollbar.overlay_mode = true;
        let hidden = super::scrollable(&r)?(&theme, active)
            .vertical_rail
            .scroller
            .border;
        assert_eq!(hidden.width, 0.0, "{preset:?}");
    }
    Ok(())
}

/// The segmented control's track and the expander's frame cast the first
/// outset layer of the shadows the theme states, its spread dropped; a
/// disabled control's shadow fades with it, and the frame casts it only
/// where it is drawn.
#[test]
fn the_segmented_track_and_the_expander_frame_cast_their_first_outset_shadow() -> TestResult {
    use iced_core::{Shadow, Vector};
    let layer = |inset| ShadowLayer {
        offset_x: 1.0,
        offset_y: 2.0,
        blur: 8.0,
        spread: 3.0,
        color: LABEL,
        inset,
    };
    let shadows = || Some(Arc::from([layer(true), layer(false)]));
    let patch = ThemePatch::new()
        .segmented_control(SegmentedControlRule::new().track(PartPatch::new().shadows(shadows())))
        .expander(ExpanderRule::new().container(PartPatch::new().shadows(shadows())));
    let expected = Shadow {
        color: to_color(LABEL),
        offset: Vector::new(1.0, 2.0),
        blur_radius: 8.0,
    };
    for (preset, mut r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        r.segmented_control.disabled_opacity = Some(0.5);
        assert_eq!(
            super::segmented_control(&r, false)?(&theme).shadow,
            expected,
            "{preset:?}"
        );
        assert_eq!(
            super::segmented_control(&r, true)?(&theme).shadow,
            Shadow {
                color: IcedColor {
                    a: 0.5,
                    ..to_color(LABEL)
                },
                ..expected
            },
            "{preset:?}"
        );
        r.expander.frame_enabled = Some(true);
        assert_eq!(
            super::expander_frame(&r)?(&theme).shadow,
            expected,
            "{preset:?}"
        );
        r.expander.frame_enabled = Some(false);
        assert_eq!(
            super::expander_frame(&r)?(&theme).shadow,
            Shadow::default(),
            "{preset:?}"
        );
    }
    Ok(())
}

/// A press the theme states no fill or stroke for is as `in_state` gives
/// it, not the hovered look: the button's fill at rest (iced's secondary
/// button does not change it pressed), no fill for a segment and an
/// expander header, and the slider thumb's stroke at rest.
#[test]
fn an_unstated_press_is_not_the_hovered_look() -> TestResult {
    use iced_widget::button::Status;
    let layer = Color::new(0.0, 0.0, 0.0, 0.5);
    let fill = |color| PartPatch::new().fill(Paint::Solid(color));
    let patch = ThemePatch::new()
        .button(ButtonRule::new().body(fill(DISTINCT)))
        .button(
            ButtonRule::new()
                .states(WidgetState::HOVER)
                .body(fill(layer)),
        )
        .segmented_control(SegmentedControlRule::new().segment(fill(DISTINCT)))
        .segmented_control(
            SegmentedControlRule::new()
                .states(WidgetState::HOVER)
                .segment(fill(LABEL)),
        )
        .expander(ExpanderRule::new().header(fill(DISTINCT)))
        .expander(
            ExpanderRule::new()
                .states(WidgetState::HOVER)
                .header(fill(LABEL)),
        )
        .slider(SliderRule::new().thumb(PartPatch::new().stroke(Stroke::uniform(1.0, DISTINCT))))
        .slider(
            SliderRule::new()
                .states(WidgetState::HOVER)
                .thumb(PartPatch::new().stroke(Stroke::uniform(1.0, LABEL))),
        );
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let button = super::button(&r)?;
        assert_eq!(
            background_color(button(&theme, Status::Hovered).background),
            Some(crate::styles::composite_over(
                to_color(layer),
                to_color(DISTINCT)
            )),
            "{preset:?}"
        );
        assert_eq!(
            background_color(button(&theme, Status::Pressed).background),
            Some(to_color(DISTINCT)),
            "{preset:?}"
        );
        let segment = super::segment(&r, false, crate::styles::SegmentPosition::Only)?;
        assert_eq!(
            background_color(segment(&theme, Status::Hovered).background),
            Some(to_color(LABEL)),
            "{preset:?}"
        );
        assert_eq!(
            segment(&theme, Status::Pressed).background,
            None,
            "{preset:?}"
        );
        let header = super::expander(&r)?;
        assert_eq!(
            background_color(header(&theme, Status::Hovered).background),
            Some(to_color(LABEL)),
            "{preset:?}"
        );
        assert_eq!(
            header(&theme, Status::Pressed).background,
            None,
            "{preset:?}"
        );
        let slider = super::slider(&r)?;
        let edge = |status| slider(&theme, status).handle.border_color;
        assert_eq!(
            edge(iced_widget::slider::Status::Hovered),
            to_color(LABEL),
            "{preset:?}"
        );
        assert_eq!(
            edge(iced_widget::slider::Status::Dragged),
            to_color(DISTINCT),
            "{preset:?}"
        );
    }

    let stated = patch
        .segmented_control(
            SegmentedControlRule::new()
                .states(WidgetState::PRESSED)
                .segment(fill(DISTINCT)),
        )
        .expander(
            ExpanderRule::new()
                .states(WidgetState::PRESSED)
                .header(fill(DISTINCT)),
        );
    for (preset, r) in every_preset_with(&stated) {
        let theme = to_theme(&r)?;
        let segment = super::segment(&r, false, crate::styles::SegmentPosition::Only)?;
        assert_eq!(
            background_color(segment(&theme, Status::Pressed).background),
            Some(to_color(DISTINCT)),
            "{preset:?}"
        );
        assert_eq!(
            background_color(super::expander(&r)?(&theme, Status::Pressed).background),
            Some(to_color(DISTINCT)),
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

/// A track stated with no stroke and no separator fill gives no line
/// between two segments, and the control and its segments are drawn.
#[test]
fn a_track_with_no_stroke_and_no_separator_fill_has_no_separator_line() -> TestResult {
    use iced_widget::button::Status;
    let patch = ThemePatch::new().segmented_control(
        SegmentedControlRule::new().track(PartPatch::new().stroke(Stroke::None)),
    );
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        assert!(
            !r.segmented_control
                .separator(WidgetState::NONE)
                .stated
                .contains(PartStated::FILL),
            "{preset:?}: no separator fill stated"
        );
        let separator = super::segment_separator(&r, false)?(&theme);
        assert_eq!(separator.background, None, "{preset:?}");
        assert_eq!(
            super::segmented_control(&r, false)?(&theme).border.width,
            0.0,
            "{preset:?}"
        );
        let segment = super::segment(&r, true, crate::styles::SegmentPosition::Only)?;
        assert_eq!(
            background_color(segment(&theme, Status::Active).background),
            Some(fill(
                &r.segmented_control.segment(WidgetState::SELECTED),
                "segment(SELECTED).fill"
            )?),
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

/// The expected value of a property the theme states nothing for in a
/// status (spec §10, rule 0): iced's own in the status (`own`) where it
/// differs from iced's own at rest (`own_rest`), counted in `iceds`; else
/// the theme's at rest (`rest`).
fn unstated<T: PartialEq>(rest: T, own_rest: T, own: T, iceds: &mut usize) -> T {
    if own == own_rest {
        rest
    } else {
        *iceds += 1;
        own
    }
}

/// A switch's thumb under the pointer is iced's own where no rule states
/// the thumb's fill for the state and iced changes it (iced_widget 0.14.2
/// `toggler::default`: the toggled thumb at half alpha, the other
/// `background.weak`).
#[test]
fn a_switch_thumb_no_hover_rule_states_is_iceds_own() -> TestResult {
    use iced_widget::toggler::Status;
    let patch = ThemePatch::new()
        .switch(SwitchRule::new().thumb(PartPatch::new().fill(Paint::Solid(DISTINCT))));
    let mut iceds = 0;
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let style = super::toggler(&r)?;
        for is_toggled in [false, true] {
            let (active, hovered) = (
                Status::Active { is_toggled },
                Status::Hovered { is_toggled },
            );
            let expected = unstated(
                style(&theme, active).foreground,
                iced_widget::toggler::default(&theme, active).foreground,
                iced_widget::toggler::default(&theme, hovered).foreground,
                &mut iceds,
            );
            assert_eq!(
                style(&theme, hovered).foreground,
                expected,
                "{preset:?} {is_toggled}"
            );
        }
    }
    assert!(iceds > 0, "iced's own thumb covered");
    Ok(())
}

/// A slider's filled rail under the pointer and dragged is iced's own where
/// no rule states the filled rail's fill for the state (iced_widget 0.14.2
/// `slider::default`: `primary.strong` and `primary.weak`).
#[test]
fn a_slider_rail_no_state_rule_states_is_iceds_own() -> TestResult {
    use iced_widget::slider::Status;
    let patch = ThemePatch::new()
        .slider(SliderRule::new().active_track(PartPatch::new().fill(Paint::Solid(DISTINCT))));
    let mut iceds = 0;
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let style = super::slider(&r)?;
        let own_rest = iced_widget::slider::default(&theme, Status::Active);
        for status in [Status::Hovered, Status::Dragged] {
            let expected = unstated(
                style(&theme, Status::Active).rail.backgrounds.0,
                own_rest.rail.backgrounds.0,
                iced_widget::slider::default(&theme, status)
                    .rail
                    .backgrounds
                    .0,
                &mut iceds,
            );
            assert_eq!(
                style(&theme, status).rail.backgrounds.0,
                expected,
                "{preset:?} {status:?}"
            );
        }
    }
    assert!(iceds > 0, "iced's own rail covered");
    Ok(())
}

/// An open drop-down's border is iced's own colour where no rule states the
/// field's stroke for the state (iced_widget 0.14.2 `pick_list::default`:
/// `primary.strong`), its width and corners the theme's at rest, which
/// iced does not change.
#[test]
fn an_open_drop_down_no_open_rule_states_is_iceds_own() -> TestResult {
    use iced_widget::pick_list::Status;
    let patch = ThemePatch::new()
        .combo_box(ComboBoxRule::new().field(PartPatch::new().stroke(Stroke::uniform(1.0, LABEL))));
    let open = Status::Opened { is_hovered: false };
    let mut iceds = 0;
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let style = super::pick_list(&r)?;
        let rest = style(&theme, Status::Active).border;
        let color = unstated(
            rest.color,
            iced_widget::pick_list::default(&theme, Status::Active)
                .border
                .color,
            iced_widget::pick_list::default(&theme, open).border.color,
            &mut iceds,
        );
        assert_eq!(
            style(&theme, open).border,
            iced_core::Border { color, ..rest },
            "{preset:?}"
        );
    }
    assert!(iceds > 0, "iced's own open border covered");
    Ok(())
}

/// A menu's highlighted item is iced's own fill and label where no rule
/// states the item's fill or its label's colour for `HOVER` (iced_widget
/// 0.14.2 `overlay::menu::default`: `primary.strong`).
#[test]
fn a_menu_item_no_hover_rule_states_is_iceds_own() -> TestResult {
    let patch = ThemePatch::new().menu(
        MenuRule::new()
            .item(PartPatch::new().fill(Paint::Solid(DISTINCT)))
            .item_label(TextPatch::new().color(LABEL)),
    );
    let (mut fills, mut labels) = (0, 0);
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let own = iced_widget::overlay::menu::default(&theme);
        let style = super::menu(&r)?(&theme);
        let background = unstated(
            style.background,
            own.background,
            own.selected_background,
            &mut fills,
        );
        let label = unstated(
            style.text_color,
            own.text_color,
            own.selected_text_color,
            &mut labels,
        );
        assert_eq!(style.selected_background, background, "{preset:?}");
        assert_eq!(style.selected_text_color, label, "{preset:?}");
    }
    assert!(fills > 0 && labels > 0, "iced's own fill and label covered");
    Ok(())
}

/// An `iced_aw` tab, sidebar item and list row under the pointer take, of
/// `iced_aw`'s own, what it changes for the status where no rule states the
/// part's fill or label for `HOVER` (iced_aw 0.14.1 `tab_bar::primary` and
/// `sidebar::primary` against a tab at rest, which they style as
/// `Disabled`: the fill; `selection_list::primary` against `Active`: the
/// fill and the label), and keep the theme's value at rest for the rest.
#[cfg(feature = "iced_aw")]
#[test]
fn an_aw_tab_sidebar_and_list_no_hover_rule_states_are_iced_aws_own() -> TestResult {
    use iced_aw::style::Status;
    use native_theme::next::{ListRule, SidebarRule, TabRule};
    let patch = ThemePatch::new()
        .tab(
            TabRule::new()
                .tab(PartPatch::new().fill(Paint::Solid(DISTINCT)))
                .label(TextPatch::new().color(LABEL)),
        )
        .sidebar(SidebarRule::new().item(PartPatch::new().fill(Paint::Solid(DISTINCT))))
        .list(
            ListRule::new()
                .highlight(PartPatch::new().fill(Paint::Solid(DISTINCT)))
                .item_label(TextPatch::new().color(LABEL)),
        );
    let mut iceds = [0; 5];
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let [tab_fill, tab_label, sidebar_fill, list_fill, list_label] = &mut iceds;

        let tab = super::aw::tab_bar(&r)?;
        let (rest, now) = (tab(&theme, Status::Disabled), tab(&theme, Status::Hovered));
        let own_rest = iced_aw::style::tab_bar::primary(&theme, Status::Disabled);
        let own = iced_aw::style::tab_bar::primary(&theme, Status::Hovered);
        let fill = unstated(
            rest.tab_label_background,
            own_rest.tab_label_background,
            own.tab_label_background,
            tab_fill,
        );
        let label = unstated(
            rest.text_color,
            own_rest.text_color,
            own.text_color,
            tab_label,
        );
        assert_eq!(now.tab_label_background, fill, "{preset:?}");
        assert_eq!(now.text_color, label, "{preset:?}");

        let sidebar = super::aw::sidebar(&r)?;
        let fill = unstated(
            sidebar(&theme, Status::Disabled).tab_label_background,
            iced_aw::style::sidebar::primary(&theme, Status::Disabled).tab_label_background,
            iced_aw::style::sidebar::primary(&theme, Status::Hovered).tab_label_background,
            sidebar_fill,
        );
        assert_eq!(
            sidebar(&theme, Status::Hovered).tab_label_background,
            fill,
            "{preset:?}"
        );

        let list = super::aw::selection_list(&r)?;
        let (rest, now) = (list(&theme, Status::Active), list(&theme, Status::Hovered));
        let own_rest = iced_aw::style::selection_list::primary(&theme, Status::Active);
        let own = iced_aw::style::selection_list::primary(&theme, Status::Hovered);
        let fill = unstated(
            rest.background,
            own_rest.background,
            own.background,
            list_fill,
        );
        let label = unstated(
            rest.text_color,
            own_rest.text_color,
            own.text_color,
            list_label,
        );
        assert_eq!(now.background, fill, "{preset:?}");
        assert_eq!(now.text_color, label, "{preset:?}");
    }
    let [tab_fill, tab_label, sidebar_fill, list_fill, list_label] = iceds;
    assert!(
        tab_fill > 0 && sidebar_fill > 0 && list_fill > 0 && list_label > 0,
        "iced_aw's own hovered fills and list label covered: {iceds:?}"
    );
    assert_eq!(tab_label, 0, "iced_aw changes no hovered tab label");
    Ok(())
}

/// A checked toggle and toolbar button whose theme states no checked fill
/// are the primary button's fill and label in the same status (spec §10,
/// rule 0a), which differ from the unchecked button at rest, a stated
/// checked label laid over, the outline the unchecked button's; a stated
/// checked fill wins.
#[test]
fn an_unstated_checked_fill_is_the_primary_buttons() -> TestResult {
    use iced_widget::button::Status;
    let mut unstated = (0, 0);
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let primary = super::button_primary(&r)?;
        let plain = super::button(&r)?;
        let label = r.button.label(WidgetState::CHECKED);
        let checked_label = label
            .from_state
            .contains(TextStated::COLOR)
            .then(|| text_color(&label, "label"))
            .transpose()?;
        for (stated, style, counted) in [
            (
                r.button.body(WidgetState::CHECKED),
                Box::new(super::button_toggle(&r, true)?)
                    as Box<dyn Fn(&iced_core::theme::Theme, Status) -> iced_widget::button::Style>,
                &mut unstated.0,
            ),
            (
                r.toolbar.button(WidgetState::CHECKED),
                Box::new(super::toolbar_button(&r, true)?),
                &mut unstated.1,
            ),
        ] {
            if stated.from_state.contains(PartStated::FILL) {
                continue;
            }
            *counted += 1;
            for status in [Status::Active, Status::Hovered, Status::Pressed] {
                let (checked, shown) = (style(&theme, status), primary(&theme, status));
                assert_eq!(
                    checked.background, shown.background,
                    "{preset:?} {status:?}"
                );
                assert_eq!(
                    checked.text_color,
                    checked_label.unwrap_or(shown.text_color),
                    "{preset:?} {status:?}"
                );
                assert_eq!(
                    checked.border.radius,
                    plain(&theme, status).border.radius,
                    "{preset:?} {status:?}"
                );
            }
            assert_ne!(
                style(&theme, Status::Active).background,
                plain(&theme, Status::Active).background,
                "{preset:?}"
            );
        }
    }
    assert!(
        unstated.0 > 0 && unstated.1 > 0,
        "unstated checked fills covered: {unstated:?}"
    );

    let patch = ThemePatch::new()
        .button(
            ButtonRule::new()
                .states(WidgetState::CHECKED)
                .body(PartPatch::new().fill(Paint::Solid(DISTINCT))),
        )
        .toolbar(
            ToolbarRule::new()
                .states(WidgetState::CHECKED)
                .button(PartPatch::new().fill(Paint::Solid(DISTINCT))),
        );
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        for status in [Status::Active, Status::Hovered, Status::Pressed] {
            assert_eq!(
                super::button_toggle(&r, true)?(&theme, status).background,
                Some(Background::Color(to_color(DISTINCT))),
                "{preset:?} {status:?}"
            );
            assert_eq!(
                super::toolbar_button(&r, true)?(&theme, status).background,
                Some(Background::Color(to_color(DISTINCT))),
                "{preset:?} {status:?}"
            );
        }
    }
    Ok(())
}

/// A stated checked fill the theme states no checked label for is lettered
/// in the label at rest, not in a stated pressed label (spec §10, rule 0a:
/// no chain of the connector's own), on the toggle and the toolbar button.
#[test]
fn a_stated_checked_fill_without_a_checked_label_is_lettered_at_rest() -> TestResult {
    use iced_widget::button::Status;
    let patch = ThemePatch::new()
        .button(
            ButtonRule::new()
                .states(WidgetState::CHECKED)
                .body(PartPatch::new().fill(Paint::Solid(DISTINCT))),
        )
        .button(
            ButtonRule::new()
                .states(WidgetState::PRESSED)
                .label(TextPatch::new().color(LABEL)),
        )
        .toolbar(
            ToolbarRule::new()
                .states(WidgetState::CHECKED)
                .button(PartPatch::new().fill(Paint::Solid(DISTINCT))),
        );
    let mut covered = 0;
    for (preset, r) in every_preset_with(&patch) {
        let pressed = r.button.label(WidgetState::PRESSED);
        assert!(pressed.from_state.contains(TextStated::COLOR), "{preset:?}");
        assert_eq!(
            text_color(&pressed, "pressed label")?,
            to_color(LABEL),
            "{preset:?}"
        );
        if r.button
            .label(WidgetState::CHECKED)
            .from_state
            .contains(TextStated::COLOR)
        {
            continue;
        }
        covered += 1;
        let at_rest = text_color(&r.button.label(WidgetState::NONE), "label")?;
        assert_ne!(at_rest, to_color(LABEL), "{preset:?}");
        let theme = to_theme(&r)?;
        let toggle = super::button_toggle(&r, true)?;
        let toolbar = super::toolbar_button(&r, true)?;
        for status in [Status::Active, Status::Hovered, Status::Pressed] {
            assert_eq!(
                toggle(&theme, status).text_color,
                at_rest,
                "{preset:?} {status:?}"
            );
            assert_eq!(
                toolbar(&theme, status).text_color,
                at_rest,
                "{preset:?} {status:?}"
            );
        }
    }
    assert!(covered > 0, "a theme with no checked label covered");
    Ok(())
}
