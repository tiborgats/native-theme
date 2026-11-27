//! The 0.8.0 style functions on themes built for the case: a stated value
//! the presets do not show, or one they leave unstated.

use std::sync::Arc;

use iced_core::Background;
use iced_core::Color as IcedColor;
use native_theme::next::{
    ButtonRule, ButtonVariant, CheckboxRule, Color, ComboBoxRule, Corners, ExpanderRule,
    GradientStop, InputRule, LinkRule, MaterialKind, MaterialSpec, MenuRule, Paint, PaintLayer,
    PartPatch, PartStated, Preset, RadioRule, ResolvedTheme, ScrollbarRule, SegmentedControlRule,
    ShadowLayer, SliderRule, Stroke, StrokeAlign, SwitchRule, TextPatch, TextStated, Theme,
    ThemeContext, ThemePatch, ToolbarRule, WidgetState,
};

use super::super::borders::{line, line_or_none};
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
        let stroke = line_or_none(&r.button.body(WidgetState::NONE), "stroke")?;

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
        b.disabled_opacity = Some(OPACITY);
        let mut t = super::toolbar_button_source(&r)?;
        let labels = t
            .labels
            .as_mut()
            .ok_or("the 0.8.0 source names the toolbar's labels")?;
        labels.disabled = None;
        let rest = labels.rest;
        let theme = to_theme(&r)?;
        let rest = rest.unwrap_or(
            iced_widget::button::secondary(&theme, iced_widget::button::Status::Active).text_color,
        );
        let style = crate::styles::toolbar_button_from(&b, &t, false)(
            &theme,
            iced_widget::button::Status::Disabled,
        );
        assert_eq!(
            style.text_color,
            crate::styles::faded(rest, OPACITY),
            "{preset:?}"
        );
    }
    Ok(())
}

/// A toolbar button is lettered in `toolbar.button_label(<state>)`, the
/// label at rest iced's own where the toolbar states none (spec §10, rule
/// 0), never in the push button's labels.
#[test]
fn a_toolbar_button_reads_the_toolbar_labels() -> TestResult {
    use iced_widget::button::Status;
    let push = IcedColor::from_rgb8(0xf0, 0xa0, 0x14);
    let (mut stated, mut left) = (0, 0);
    for (preset, r) in every_preset() {
        let mut b = super::button_source(&r)?;
        b.label = Some(push);
        b.hover_label = Some(push);
        b.active_label = Some(push);
        b.checked_label = Some(push);
        b.disabled_label = Some(push);
        let t = super::toolbar_button_source(&r)?;
        let labels = t
            .labels
            .ok_or("the 0.8.0 source names the toolbar's labels")?;
        let theme = to_theme(&r)?;
        let style = crate::styles::toolbar_button_from(&b, &t, false);
        let rest = style(&theme, Status::Active).text_color;
        match labels.rest {
            Some(label) => {
                stated += 1;
                assert_eq!(rest, label, "{preset:?}: the stated label at rest");
            }
            None => {
                left += 1;
                assert_eq!(
                    rest,
                    iced_widget::button::secondary(&theme, Status::Active).text_color,
                    "{preset:?}: iced's own label at rest"
                );
            }
        }
        for (status, label) in [
            (Status::Hovered, labels.hover),
            (Status::Pressed, labels.pressed),
            (Status::Disabled, labels.disabled),
        ] {
            let drawn = style(&theme, status).text_color;
            if let Some(label) = label {
                assert_eq!(drawn, label, "{preset:?} {status:?}: the stated label");
            }
            assert_ne!(drawn, push, "{preset:?} {status:?}: not the push button's");
        }
        assert_ne!(rest, push, "{preset:?}: not the push button's");
    }
    assert!(
        stated > 0,
        "a stated label at rest covered ({left} left to iced)"
    );
    Ok(())
}

/// A toolbar button is outlined as the toolbar's own button (spec §5, the
/// toolbar's `button` part), never as the push button: its corners
/// `toolbar.button(NONE)`'s, iced's own neutral class's where unstated, and
/// a hovered stroke a rule states drawn at that stroke's width, on every
/// preset with the push button's corners and stroke patched apart.
#[test]
fn a_toolbar_button_is_outlined_as_its_own_part() -> TestResult {
    use iced_widget::button::{Status, secondary};
    const WIDTH: f32 = 3.0;
    let patch = ThemePatch::new()
        .button(
            ButtonRule::new().body(
                PartPatch::new()
                    .corners(Corners::uniform(7.0))
                    .stroke(Stroke::uniform(5.0, DISTINCT)),
            ),
        )
        .toolbar(
            ToolbarRule::new()
                .states(WidgetState::HOVER)
                .button(PartPatch::new().stroke(Stroke::uniform(WIDTH, LABEL))),
        );
    let (mut stated, mut left) = (0, 0);
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let style = super::toolbar_button(&r, false)?;
        let rest = r.toolbar.button(WidgetState::NONE);
        let own = secondary(&theme, Status::Active).border.radius;
        let expected = if rest.stated.contains(PartStated::CORNERS) {
            stated += 1;
            super::super::borders::part_radii(&rest, "corners")?
        } else {
            left += 1;
            own
        };
        for status in [Status::Active, Status::Hovered, Status::Pressed] {
            assert_eq!(
                style(&theme, status).border.radius,
                expected,
                "{preset:?} {status:?} corners"
            );
        }
        let hovered = style(&theme, Status::Hovered).border;
        assert_eq!(hovered.width, WIDTH, "{preset:?} hovered width");
        assert_eq!(hovered.color, to_color(LABEL), "{preset:?} hovered colour");
    }
    assert!(left > 0, "iced's own corners covered ({stated} stated)");
    Ok(())
}

/// A toolbar button whose toolbar states no hovered or pressed fill is drawn
/// in iced's own there (spec §10, rule 0), never in the push button's.
#[test]
fn a_toolbar_button_leaves_an_unstated_state_fill_to_iced() -> TestResult {
    use crate::styles::Layer;
    use iced_widget::button::Status;
    let push = Some(Layer::Whole(IcedColor::from_rgb8(0xf0, 0xa0, 0x14)));
    for (preset, r) in every_preset() {
        let mut t = super::toolbar_button_source(&r)?;
        t.hover_background = None;
        t.active_background = None;
        let mut b = super::button_source(&r)?;
        let theme = to_theme(&r)?;
        let without = crate::styles::toolbar_button_from(&b, &t, false);
        b.hover_background = push;
        b.active_background = push;
        let with = crate::styles::toolbar_button_from(&b, &t, false);
        for status in [Status::Hovered, Status::Pressed] {
            assert_eq!(
                with(&theme, status).background,
                without(&theme, status).background,
                "{preset:?} {status:?}"
            );
        }
    }
    Ok(())
}

/// A disabled toolbar button is filled in the theme's answer for the state
/// it is in: `toolbar.button(DISABLED)`'s off, `(DISABLED | CHECKED)`'s
/// checked (spec §10, rule 0), on a theme whose two answers differ.
#[test]
fn a_disabled_checked_toolbar_button_takes_the_disabled_checked_fill() -> TestResult {
    use iced_widget::button::Status;
    let patch = ThemePatch::new()
        .toolbar(
            ToolbarRule::new()
                .states(WidgetState::DISABLED)
                .button(PartPatch::new().fill(Paint::Solid(DISTINCT))),
        )
        .toolbar(
            ToolbarRule::new()
                .states(WidgetState::DISABLED | WidgetState::CHECKED)
                .button(PartPatch::new().fill(Paint::Solid(LABEL))),
        );
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        for (checked, fill) in [(false, DISTINCT), (true, LABEL)] {
            assert_eq!(
                super::toolbar_button(&r, checked)?(&theme, Status::Disabled).background,
                Some(Background::Color(to_color(fill))),
                "{preset:?} checked {checked}"
            );
        }
    }
    Ok(())
}

/// A disabled checked toggle is filled in the theme's answer for the state
/// it is in, `button.body(DISABLED | CHECKED)`'s, not `(DISABLED)`'s (spec
/// §10, rule 0), as a disabled checked toolbar button is: on a theme whose
/// two answers differ, as an unchecked toggle disabled is on a theme that
/// states the first answer's colour for `DISABLED`, and not as one that is
/// off on the same theme.
#[test]
fn a_disabled_checked_toggle_takes_the_disabled_checked_fill() -> TestResult {
    use iced_widget::button::Status;
    fn disabled(
        r: &ResolvedTheme,
        checked: bool,
    ) -> Result<Option<Background>, Box<dyn std::error::Error>> {
        Ok(super::button_toggle(r, checked)?(&to_theme(r)?, Status::Disabled).background)
    }
    let in_disabled = |fill: Color| {
        ButtonRule::new()
            .states(WidgetState::DISABLED)
            .body(PartPatch::new().fill(Paint::Solid(fill)))
    };
    let patch = ThemePatch::new().button(in_disabled(DISTINCT)).button(
        ButtonRule::new()
            .states(WidgetState::DISABLED | WidgetState::CHECKED)
            .body(PartPatch::new().fill(Paint::Solid(LABEL))),
    );
    let in_label = every_preset_with(&ThemePatch::new().button(in_disabled(LABEL)));
    for ((preset, r), (_, in_label)) in every_preset_with(&patch).into_iter().zip(in_label) {
        let checked = disabled(&r, true)?;
        assert_eq!(checked, disabled(&in_label, false)?, "{preset:?}");
        assert_ne!(checked, disabled(&r, false)?, "{preset:?}");
    }
    Ok(())
}

/// A checked toggle whose theme states the checked fill and label for
/// `CHECKED` alone is filled and lettered under the pointer and pressed in
/// the theme's answers for `CHECKED | HOVER` and `CHECKED | PRESSED`, at
/// rest in `CHECKED`'s (spec §10, rule 0: "what the theme
/// states for the state is laid over"), as the toolbar button is; on a
/// theme whose three answers differ.
#[test]
fn a_checked_toggle_takes_the_checked_hovered_and_pressed_fill_and_label() -> TestResult {
    use iced_widget::button::Status;
    const HOVERED: Color = Color::new(0.6, 0.3, 0.1, 1.0);
    const PRESSED: Color = Color::new(0.2, 0.7, 0.4, 1.0);
    let state = |states: WidgetState, colour: Color| {
        ButtonRule::new()
            .states(states)
            .body(PartPatch::new().fill(Paint::Solid(colour)))
            .label(TextPatch::new().color(colour))
    };
    let patch = ThemePatch::new()
        .button(state(WidgetState::CHECKED, DISTINCT))
        .button(state(WidgetState::CHECKED | WidgetState::HOVER, HOVERED))
        .button(state(WidgetState::CHECKED | WidgetState::PRESSED, PRESSED));
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let toggle = super::button_toggle(&r, true)?;
        for (status, colour) in [
            (Status::Active, DISTINCT),
            (Status::Hovered, HOVERED),
            (Status::Pressed, PRESSED),
        ] {
            let drawn = toggle(&theme, status);
            assert_eq!(
                drawn.background,
                Some(Background::Color(to_color(colour))),
                "{preset:?} {status:?}"
            );
            assert_eq!(drawn.text_color, to_color(colour), "{preset:?} {status:?}");
        }
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
        let text = r.input.text(WidgetState::DISABLED);
        if text.from_state.contains(TextStated::COLOR) {
            assert_eq!(style.value, text_color(&text, "text")?, "{preset:?}");
        }
    }
    Ok(())
}

/// A disabled field's value is the colour a rule for `DISABLED` states
/// (`input.text(DISABLED)`, `text_area.text(DISABLED)`); where none does,
/// iced's own disabled value, which `text_input::default` and
/// `text_editor::default` change from the value at rest (spec §2,
/// `from_state`; §10, rule 0), never the value at rest the query returns.
#[test]
fn a_disabled_fields_value_is_stated_for_the_state_else_iceds_own() -> TestResult {
    let rest = ThemePatch::new()
        .input(InputRule::new().text(TextPatch::new().color(DISTINCT)))
        .text_area(InputRule::new().text(TextPatch::new().color(DISTINCT)));
    let stated = rest
        .clone()
        .input(
            InputRule::new()
                .states(WidgetState::DISABLED)
                .text(TextPatch::new().color(LABEL)),
        )
        .text_area(
            InputRule::new()
                .states(WidgetState::DISABLED)
                .text(TextPatch::new().color(LABEL)),
        );
    for (patch, rule) in [(&rest, false), (&stated, true)] {
        for (preset, mut r) in every_preset_with(patch) {
            r.input.disabled_opacity = None;
            r.text_area.disabled_opacity = None;
            let theme = to_theme(&r)?;
            let what = format!("{preset:?} rule {rule}");
            let input = super::text_input(&r)?(&theme, iced_widget::text_input::Status::Disabled);
            let editor =
                super::text_editor(&r)?(&theme, iced_widget::text_editor::Status::Disabled);
            let (own_input, own_editor) = (
                iced_widget::text_input::default(&theme, iced_widget::text_input::Status::Disabled)
                    .value,
                iced_widget::text_editor::default(
                    &theme,
                    iced_widget::text_editor::Status::Disabled,
                )
                .value,
            );
            if rule {
                assert_eq!(input.value, to_color(LABEL), "{what} input");
                assert_eq!(editor.value, to_color(LABEL), "{what} editor");
            } else {
                assert_eq!(input.value, own_input, "{what} input");
                assert_eq!(editor.value, own_editor, "{what} editor");
                assert_ne!(input.value, to_color(DISTINCT), "{what} input");
            }
        }
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

/// A backdrop material of the case, `fallback` its colour for a toolkit
/// without backdrop blur.
fn material(fallback: Color) -> PaintLayer {
    PaintLayer::Material(MaterialSpec::new(
        MaterialKind::Acrylic,
        30.0,
        Color::TRANSPARENT,
        1.0,
        0.0,
        0.0,
        fallback,
    ))
}

/// A material is drawn in its fallback colour (spec §10, rule 2: "render
/// `material.fallback`"), on the surface it lies on whatever that is.
#[test]
fn a_material_fill_is_drawn_in_its_fallback() -> TestResult {
    let patch = ThemePatch::new().input(
        InputRule::new().field(PartPatch::new().fill(Paint::layers(vec![material(DISTINCT)]))),
    );
    let r = Theme::preset(Preset::Adwaita)
        .with_patch(patch)
        .resolve(&ThemeContext::light());
    let theme = to_theme(&r)?;
    let style = super::text_input(&r)?(&theme, iced_widget::text_input::Status::Active);
    assert_eq!(style.background, Background::Color(to_color(DISTINCT)));
    Ok(())
}

/// A translucent solid over a material is drawn in the topmost opaque
/// layer's colour, the material's fallback, not flattened onto a surface.
#[test]
fn a_solid_over_a_material_is_drawn_in_the_topmost_opaque_layer() -> TestResult {
    use iced_widget::button::Status;
    let layers = Paint::layers(vec![
        material(DISTINCT),
        PaintLayer::Solid {
            color: Color::new(1.0, 1.0, 1.0, 0.5),
            blend: Default::default(),
        },
    ]);
    let patch = ThemePatch::new().button(ButtonRule::new().body(PartPatch::new().fill(layers)));
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let style = super::button(&r)?(&theme, Status::Active);
        assert_eq!(
            background_color(style.background),
            Some(to_color(DISTINCT)),
            "{preset:?}"
        );
    }
    Ok(())
}

/// Layers that are all gradients have no single colour: a style iced
/// draws in one colour reports them `Unmapped` at their home (spec §10,
/// rule 4), never iced's own fill in their place.
#[test]
fn an_all_gradient_fill_is_unmapped() -> TestResult {
    let gradient = Paint::layers(vec![PaintLayer::LinearGradient {
        angle_deg: 90.0,
        stops: Arc::from(vec![
            GradientStop {
                offset: 0.0,
                color: DISTINCT,
            },
            GradientStop {
                offset: 1.0,
                color: LABEL,
            },
        ]),
        blend: Default::default(),
    }]);
    let fill = PartPatch::new().fill(gradient);
    let patch = ThemePatch::new()
        .button(ButtonRule::new().body(fill.clone()))
        .button(
            ButtonRule::new()
                .states(WidgetState::HOVER)
                .body(fill.clone()),
        )
        .toolbar(ToolbarRule::new().bar(fill.clone()))
        .input(InputRule::new().field(fill));
    let r = Theme::preset(Preset::KdeBreeze)
        .with_patch(patch)
        .resolve(&ThemeContext::light());
    assert_eq!(
        super::super::fill_of(&r.button.body(WidgetState::NONE), "at rest"),
        Err(super::super::Unmapped("at rest"))
    );
    assert_eq!(
        super::super::state_fill_of(&r.button.body(WidgetState::HOVER), "hovered"),
        Err(super::super::Unmapped("hovered"))
    );
    assert_eq!(
        super::button(&r).err(),
        Some(super::super::Unmapped("button.body(NONE).fill"))
    );
    assert_eq!(
        super::text_input(&r).err(),
        Some(super::super::Unmapped("input.field(NONE).fill"))
    );
    assert_eq!(
        super::toolbar(&r).err(),
        Some(super::super::Unmapped("toolbar.bar(NONE).fill"))
    );
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
/// pressed look of its own or of the button's keeps its fill at rest where
/// iced makes no change.
#[test]
fn an_unstated_state_is_iceds_change_to_the_secondary_button() -> TestResult {
    use iced_widget::button::Status;
    let patch = ThemePatch::new()
        .button(
            ButtonRule::new()
                .body(every_state())
                .label(TextPatch::new().color(LABEL)),
        )
        .toolbar(
            ToolbarRule::new()
                .button(every_state())
                .button_label(TextPatch::new().color(LABEL)),
        );
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
            hovered.background
        } else {
            fill
        };
        assert_eq!(style.background, expected, "{preset:?} toolbar");
        assert_eq!(style.text_color, label, "{preset:?} toolbar");
        let style = toolbar(&theme, Status::Pressed);
        assert_eq!(style.background, fill, "{preset:?} toolbar");
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

/// A disabled check box, checked or not, is stroked in what a rule for
/// `DISABLED` states (asked with that flag alone, spec §2, `from_state`);
/// where none does, its border colour is iced's own change, which iced's
/// own box makes when disabled (iced_widget 0.14.2 `checkbox::primary`:
/// `background.weak` unchecked, `background.strong` checked), the theme's
/// at rest where iced's palette makes none, and its width the theme's at
/// rest. The box is not faded here (no disabled opacity).
#[test]
fn a_disabled_check_box_is_stroked_as_stated_else_iceds_own() -> TestResult {
    use iced_widget::checkbox::{Status, primary};
    const DISABLED_STROKE: Color = Color::new(0.7, 0.1, 0.2, 1.0);
    let base = ThemePatch::new().checkbox(
        CheckboxRule::new().indicator(
            PartPatch::new()
                .fill(Paint::Solid(DISTINCT))
                .stroke(Stroke::uniform(2.0, LABEL)),
        ),
    );
    let stated = base.clone().checkbox(
        CheckboxRule::new()
            .states(WidgetState::DISABLED)
            .indicator(PartPatch::new().stroke(Stroke::uniform(2.0, DISABLED_STROKE))),
    );
    let mut iceds = 0;
    for (patch, rule) in [(&base, false), (&stated, true)] {
        for (preset, mut r) in every_preset_with(patch) {
            r.checkbox.disabled_opacity = None;
            let theme = to_theme(&r)?;
            let style = super::checkbox(&r)?;
            for is_checked in [false, true] {
                let what = format!("{preset:?} checked {is_checked} rule {rule}");
                let status = Status::Disabled { is_checked };
                let border = style(&theme, status).border;
                let own_rest = primary(&theme, Status::Active { is_checked }).border;
                let own = primary(&theme, status).border;
                let expected = if rule {
                    to_color(DISABLED_STROKE)
                } else if own.color != own_rest.color {
                    iceds += 1;
                    own.color
                } else {
                    to_color(LABEL)
                };
                assert_eq!(border.color, expected, "{what}");
                assert_eq!(border.width, 2.0, "{what}");
            }
        }
    }
    assert!(iceds > 0, "iced's own disabled border covered");
    Ok(())
}

/// A disabled checked box's label is the one a rule states for `DISABLED`
/// by spec §2's two queries (`checkbox.label(DISABLED)`, or
/// `checkbox.label(DISABLED|CHECKED)` asking `DISABLED`), read at
/// `DISABLED|CHECKED`; else the disabled box's, read at `DISABLED`. On
/// every preset with the label first stated with no state, so the query at
/// `DISABLED` alone lacks its bit: (i) a rule for `DISABLED|CHECKED` alone
/// letters the checked box and leaves the unchecked one, (ii) a rule for
/// `CHECKED` alone states nothing for `DISABLED`. The box is not faded here
/// (no disabled opacity).
#[test]
fn a_rule_for_disabled_and_checked_letters_the_disabled_checked_box() -> TestResult {
    use iced_widget::checkbox::Status;
    let base = ThemePatch::new().checkbox(CheckboxRule::new().label(TextPatch::new().color(LABEL)));
    let ruled = |states| {
        base.clone().checkbox(
            CheckboxRule::new()
                .states(states)
                .label(TextPatch::new().color(DISTINCT)),
        )
    };
    let both = ruled(WidgetState::DISABLED | WidgetState::CHECKED);
    let checked = ruled(WidgetState::CHECKED);
    for (patch, case, on) in [
        (&both, "(i)", to_color(DISTINCT)),
        (&checked, "(ii)", to_color(LABEL)),
    ] {
        for (preset, mut r) in every_preset_with(patch) {
            assert!(
                !r.checkbox
                    .label(WidgetState::DISABLED)
                    .from_state
                    .contains(TextStated::COLOR),
                "{case} {preset:?}: the bit at DISABLED alone stays set"
            );
            r.checkbox.disabled_opacity = None;
            let theme = to_theme(&r)?;
            let style = super::checkbox(&r)?;
            for (is_checked, expected) in [(false, to_color(LABEL)), (true, on)] {
                let label = style(&theme, Status::Disabled { is_checked }).text_color;
                assert_eq!(
                    label,
                    Some(expected),
                    "{case} {preset:?} checked {is_checked}"
                );
            }
        }
    }
    Ok(())
}

/// A vertical slider's rail is as wide as the vertical track
/// (`slider.track(NONE; orientation=Vertical).min_size[0]`), a horizontal
/// one's as tall as the horizontal track (`min_size[1]`), each from its own
/// orientation's query.
#[test]
fn a_vertical_sliders_rail_is_the_vertical_tracks_width() -> TestResult {
    use iced_widget::slider::Status;
    use native_theme::next::Orientation;
    let patch = ThemePatch::new()
        .slider(
            SliderRule::new()
                .orientation(Orientation::Vertical)
                .track(PartPatch::new().min_size([Some(9.0), Some(31.0)])),
        )
        .slider(
            SliderRule::new()
                .orientation(Orientation::Horizontal)
                .track(PartPatch::new().min_size([Some(37.0), Some(5.0)])),
        );
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let vertical = super::vertical_slider(&r)?(&theme, Status::Active);
        assert_eq!(vertical.rail.width, 9.0, "{preset:?}");
        let horizontal = super::slider(&r)?(&theme, Status::Active);
        assert_eq!(horizontal.rail.width, 5.0, "{preset:?}");
    }
    Ok(())
}

/// The disabled fill and label a button with no disabled opacity shows:
/// the disabled fill the theme states, its answer for `DISABLED | CHECKED`
/// where `checked`, with its disabled label or the label at rest faded as
/// iced fades it; or, where it states no disabled fill, `idle` and
/// `idle_label` faded as iced fades them.
fn disabled_without_opacity(
    r: &ResolvedTheme,
    checked: bool,
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
    let body = if checked {
        r.button.body(WidgetState::DISABLED | WidgetState::CHECKED)
    } else {
        body
    };
    Ok((fill(&body, "fill")?, label, true))
}

/// A status button and a checked toggle whose theme states no disabled
/// opacity draw each disabled colour the theme states as stated, the
/// toggle's fill its answer for `DISABLED | CHECKED`, and fade only the ones
/// it leaves to iced, as iced's own classes do.
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
            disabled_without_opacity(&r, false, to_color(role.fill), to_color(role.foreground))?;
        assert_eq!(background_color(style.background), Some(fill), "{preset:?}");
        assert_eq!(style.text_color, label, "{preset:?}");

        let toggle = super::button_toggle(&r, true)?;
        let checked = toggle(&theme, Status::Active);
        let checked_fill = background_color(checked.background).ok_or("no checked fill")?;
        let (fill, label, _) =
            disabled_without_opacity(&r, true, checked_fill, checked.text_color)?;
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

/// A dragged thumb is the pressed fill the theme states, the state's whole
/// fill (spec §9, step 2); a translucent thumb stroke inside the thumb is laid over
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
        let pressed = to_color(layer);
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
        let style = super::scrollable(&r, WidgetState::NONE)?;
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
            super::scrollable(&r, WidgetState::NONE)?(&theme, hovered)
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
        let rest = r.scrollbar.thumb(WidgetState::NONE);
        let border = super::scrollable(&r, WidgetState::NONE)?(&theme, active)
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
        let style = super::scrollable(&r, WidgetState::NONE)?;
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
        let hidden = super::scrollable(&r, WidgetState::NONE)?(&theme, active)
            .vertical_rail
            .scroller
            .border;
        assert_eq!(hidden.width, 0.0, "{preset:?}");
    }
    Ok(())
}

/// A scroller at rest in the view that has the focus is that view's handle,
/// `thumb(NONE)`: a rule stated for `UNFOCUSED_VIEW` alone does not paint it;
/// in a view without the focus it is `thumb(UNFOCUSED_VIEW)`, that rule's
/// (spec §3, `WidgetState`; spec §5, `S3`).
#[test]
fn a_scroller_at_rest_is_the_handle_of_its_views_focus() -> TestResult {
    use iced_widget::scrollable::Status;
    let active = Status::Active {
        is_horizontal_scrollbar_disabled: false,
        is_vertical_scrollbar_disabled: false,
    };
    let unfocused = Color::new(0.9, 0.1, 0.1, 1.0);
    let patch = ThemePatch::new()
        .scrollbar(ScrollbarRule::new().thumb(PartPatch::new().fill(Paint::Solid(DISTINCT))))
        .scrollbar(
            ScrollbarRule::new()
                .states(WidgetState::UNFOCUSED_VIEW)
                .thumb(PartPatch::new().fill(Paint::Solid(unfocused))),
        );
    for (preset, mut r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        r.scrollbar.overlay_mode = false;
        let scroller = super::scrollable(&r, WidgetState::NONE)?(&theme, active)
            .vertical_rail
            .scroller;
        assert_eq!(
            background_color(Some(scroller.background)),
            Some(to_color(DISTINCT)),
            "{preset:?}"
        );
        let unfocused_scroller =
            super::scrollable(&r, WidgetState::UNFOCUSED_VIEW)?(&theme, active)
                .vertical_rail
                .scroller;
        assert_eq!(
            background_color(Some(unfocused_scroller.background)),
            Some(to_color(unfocused)),
            "{preset:?}"
        );
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
            Some(to_color(layer)),
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
        let track = r.segmented_control.track(WidgetState::NONE);
        let separator = r.segmented_control.separator(WidgetState::NONE);
        // a track stated with no stroke gives no line (the test below)
        let expected = if separator.stated.contains(PartStated::FILL) {
            Some(fill(&separator, "separator(NONE).fill")?)
        } else if track.stated.contains(PartStated::STROKE) && matches!(track.stroke, Stroke::None)
        {
            None
        } else {
            Some(line(&track, "stroke")?.color)
        };
        assert_eq!(background_color(style.background), expected, "{preset:?}");
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
    let mut tested = 0;
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        // a preset that states the separator's fill draws that line (the test above)
        if r.segmented_control
            .separator(WidgetState::NONE)
            .stated
            .contains(PartStated::FILL)
        {
            continue;
        }
        tested += 1;
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
    assert!(tested > 0, "no preset leaves the separator's fill unstated");
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

/// A switch's thumb is outlined in the stroke its own state states, on and
/// off, at rest and under the pointer: a checked thumb's stated outline is
/// the checked one, not the off thumb's (spec §10, rule 0, per property).
#[test]
fn a_switch_thumb_is_outlined_in_its_own_states_stroke() -> TestResult {
    use iced_widget::toggler::Status;
    let off = Color::new(0.9, 0.1, 0.1, 1.0);
    let on = Color::new(0.1, 0.1, 0.9, 1.0);
    let patch = ThemePatch::new()
        .switch(SwitchRule::new().thumb(PartPatch::new().stroke(Stroke::uniform(1.0, off))))
        .switch(
            SwitchRule::new()
                .states(WidgetState::CHECKED)
                .thumb(PartPatch::new().stroke(Stroke::uniform(2.0, on))),
        );
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let style = super::toggler(&r)?;
        for (is_toggled, colour, width) in [(false, off, 1.0), (true, on, 2.0)] {
            for status in [
                Status::Active { is_toggled },
                Status::Hovered { is_toggled },
            ] {
                let drawn = style(&theme, status);
                assert_eq!(
                    (drawn.foreground_border_color, drawn.foreground_border_width),
                    (to_color(colour), width),
                    "{preset:?} {status:?}"
                );
            }
        }
    }
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
        let stated_label = |label: native_theme::next::TextStyle| {
            label
                .from_state
                .contains(TextStated::COLOR)
                .then(|| text_color(&label, "label"))
                .transpose()
        };
        type Styled = Box<dyn Fn(&iced_core::theme::Theme, Status) -> iced_widget::button::Style>;
        for (stated, checked_label, style, unchecked, counted) in [
            (
                r.button.body(WidgetState::CHECKED),
                stated_label(r.button.label(WidgetState::CHECKED))?,
                Box::new(super::button_toggle(&r, true)?) as Styled,
                Box::new(super::button(&r)?) as Styled,
                &mut unstated.0,
            ),
            (
                r.toolbar.button(WidgetState::CHECKED),
                stated_label(r.toolbar.button_label(WidgetState::CHECKED))?,
                Box::new(super::toolbar_button(&r, true)?),
                Box::new(super::toolbar_button(&r, false)?),
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
                    unchecked(&theme, status).border.radius,
                    "{preset:?} {status:?}"
                );
            }
            assert_ne!(
                style(&theme, Status::Active).background,
                unchecked(&theme, Status::Active).background,
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
/// "a stated `CHECKED` fill is lettered in the stated `CHECKED` label, else
/// in the label at rest"), on the toggle and the toolbar button.
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
        )
        .toolbar(
            ToolbarRule::new()
                .states(WidgetState::PRESSED)
                .button_label(TextPatch::new().color(LABEL)),
        );
    let (mut covered, mut toolbars) = (0, 0);
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let pressed = r.toolbar.button_label(WidgetState::PRESSED);
        assert!(pressed.from_state.contains(TextStated::COLOR), "{preset:?}");
        // the toolbar button is lettered in the toolbar's own labels
        if !r
            .toolbar
            .button_label(WidgetState::CHECKED)
            .from_state
            .contains(TextStated::COLOR)
        {
            toolbars += 1;
            let at_rest = text_color(&r.toolbar.button_label(WidgetState::NONE), "label")?;
            assert_ne!(at_rest, to_color(LABEL), "{preset:?}");
            let toolbar = super::toolbar_button(&r, true)?;
            for status in [Status::Active, Status::Hovered, Status::Pressed] {
                assert_eq!(
                    toolbar(&theme, status).text_color,
                    at_rest,
                    "{preset:?} {status:?}"
                );
            }
        }
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
        let toggle = super::button_toggle(&r, true)?;
        for status in [Status::Active, Status::Hovered, Status::Pressed] {
            assert_eq!(
                toggle(&theme, status).text_color,
                at_rest,
                "{preset:?} {status:?}"
            );
        }
    }
    assert!(covered > 0, "a theme with no checked label covered");
    assert!(
        toolbars > 0,
        "a theme with no checked toolbar label covered"
    );
    Ok(())
}

/// A border in a status no rule states the stroke for: each of its colour,
/// width and radius as [`unstated`] gives it against iced's own border at
/// rest (`own_rest`) and in the status (`own`).
fn unstated_border(
    rest: iced_core::Border,
    own_rest: iced_core::Border,
    own: iced_core::Border,
    iceds: &mut usize,
) -> iced_core::Border {
    iced_core::Border {
        color: unstated(rest.color, own_rest.color, own.color, iceds),
        width: unstated(rest.width, own_rest.width, own.width, iceds),
        radius: unstated(rest.radius, own_rest.radius, own.radius, iceds),
    }
}

/// A text input's and a text editor's border, hovered and focused, is
/// iced's own change where no rule states the field's stroke for the state
/// (iced_widget 0.14.2 `text_input::default` and `text_editor::default`:
/// `background.base.text` hovered, `primary.strong` focused), each value
/// iced leaves as at rest the theme's at rest. The focus ring is moved off
/// the field's edge, so no field draws it as its focused border.
#[test]
fn a_field_stroke_no_hover_or_focus_rule_states_is_iceds_own() -> TestResult {
    let stroke = || PartPatch::new().stroke(Stroke::uniform(1.0, LABEL));
    let patch = ThemePatch::new()
        .input(InputRule::new().field(stroke()))
        .text_area(InputRule::new().field(stroke()));
    let mut iceds = 0;
    for (preset, mut r) in every_preset_with(&patch) {
        r.focus_ring.offset = 0.0;
        let theme = to_theme(&r)?;
        {
            use iced_widget::text_input::{Status, default};
            let style = super::text_input(&r)?;
            let rest = style(&theme, Status::Active).border;
            let own_rest = default(&theme, Status::Active).border;
            for status in [Status::Hovered, Status::Focused { is_hovered: false }] {
                let own = default(&theme, status).border;
                assert_eq!(
                    style(&theme, status).border,
                    unstated_border(rest, own_rest, own, &mut iceds),
                    "text_input {preset:?} {status:?}"
                );
            }
        }
        {
            use iced_widget::text_editor::{Status, default};
            let style = super::text_editor(&r)?;
            let rest = style(&theme, Status::Active).border;
            let own_rest = default(&theme, Status::Active).border;
            for status in [Status::Hovered, Status::Focused { is_hovered: false }] {
                let own = default(&theme, status).border;
                assert_eq!(
                    style(&theme, status).border,
                    unstated_border(rest, own_rest, own, &mut iceds),
                    "text_editor {preset:?} {status:?}"
                );
            }
        }
    }
    assert!(iceds > 0, "iced's own field border covered");
    Ok(())
}

/// A focus ring inset at the field's edge (offset minus its width) is
/// painted over the field, not drawn as its border (spec §2,
/// `FocusIndicator`): the focused field's border is the one it has with
/// the ring off its edge, and the ring is the band over the field's
/// outermost width, its outer corners the field's, in the colour of the
/// ring's stroke (`focus_ring.stroke`), whatever `colors.focus_ring` is.
#[test]
fn an_inset_ring_is_the_band_over_the_field_not_its_border() -> TestResult {
    use iced_core::border::Radius;
    use iced_core::{Point, Rectangle, Size};
    use iced_widget::{text_editor, text_input};
    let opaque = Color::new(0.6, 0.1, 0.4, 1.0);
    let field = Rectangle::new(Point::new(10.0, 20.0), Size::new(100.0, 30.0));
    for (preset, mut r) in every_preset() {
        r.colors.focus_ring = DISTINCT;
        r.focus_ring.stroke = Stroke::uniform(2.0, opaque);
        let borders = |r: &ResolvedTheme| -> Result<_, super::super::Unmapped> {
            let theme = to_theme(r)?;
            let focused = |is_hovered| text_input::Status::Focused { is_hovered };
            let editor = |is_hovered| text_editor::Status::Focused { is_hovered };
            Ok([false, true].map(|hovered| {
                (
                    super::text_input(r).map(|s| s(&theme, focused(hovered)).border),
                    super::text_editor(r).map(|s| s(&theme, editor(hovered)).border),
                )
            }))
        };
        r.focus_ring.offset = 1.0;
        let off_the_edge = borders(&r)?;
        r.focus_ring.offset = -2.0;
        assert_eq!(borders(&r)?, off_the_edge, "{preset:?}");

        let look = super::super::ring_look(&r, 5.0)?.ok_or("the inset ring")?;
        let (ring, _) = look.quad(field).ok_or("the inset ring's quad")?;
        assert_eq!(ring.bounds, field, "{preset:?}");
        assert_eq!(ring.border.width, 2.0, "{preset:?}");
        assert_eq!(ring.border.color, to_color(opaque), "{preset:?}");
        assert_eq!(ring.border.radius, Radius::from(5.0), "{preset:?}");
    }
    Ok(())
}

/// A state's fill on the 0.8.0 model is the whole fill the part shows in
/// that state (spec §9, step 2), painted as given: a translucent hovered
/// fill a rule states is the style's, not composited over the resting
/// fill, and so is an opaque one.
#[test]
fn a_stated_hovered_fill_is_the_states_own() -> TestResult {
    use iced_widget::{button, checkbox, pick_list, slider, toggler};

    for (marker, color) in [
        (
            "#ff00ff80",
            IcedColor::from_rgba8(0xff, 0, 0xff, 128.0 / 255.0),
        ),
        ("#f0a014", IcedColor::from_rgb8(0xf0, 0xa0, 0x14)),
    ] {
        let rule = |widget: &str, part: &str| {
            format!(
                "[[widgets.{widget}.rules]]\nstates = [\"hover\"]\n{part}.fill = \"{marker}\"\n"
            )
        };
        let patch = [
            rule("button", "body"),
            rule("checkbox", "indicator"),
            rule("switch", "track"),
            rule("combo_box", "field"),
            rule("slider", "thumb"),
        ]
        .concat();
        let r = Theme::preset(Preset::KdeBreeze)
            .with_patch(ThemePatch::from_toml(&patch)?)
            .resolve(&ThemeContext::light());
        let theme = to_theme(&r)?;
        let fills = [
            (
                "button",
                background_color(super::button(&r)?(&theme, button::Status::Hovered).background),
            ),
            (
                "checkbox",
                background_color(Some(
                    super::checkbox(&r)?(&theme, checkbox::Status::Hovered { is_checked: false })
                        .background,
                )),
            ),
            (
                "toggler",
                background_color(Some(
                    super::toggler(&r)?(&theme, toggler::Status::Hovered { is_toggled: false })
                        .background,
                )),
            ),
            (
                "pick_list",
                background_color(Some(
                    super::pick_list(&r)?(&theme, pick_list::Status::Hovered).background,
                )),
            ),
            (
                "slider",
                background_color(Some(
                    super::slider(&r)?(&theme, slider::Status::Hovered)
                        .handle
                        .background,
                )),
            ),
        ];
        for (what, fill) in fills {
            assert_eq!(fill, Some(color), "{what} {marker}");
        }
    }
    Ok(())
}

/// On the macOS presets, whose button the pointer does not change
/// (`platform-facts.md` §2.3), the hovered button's fill is its resting
/// fill wherever the preset states it so: the state's own fill, not a layer
/// laid over the rest again.
#[test]
fn a_macos_buttons_hovered_fill_is_its_resting_one() -> TestResult {
    use iced_widget::button::Status;

    let mut tried = 0;
    let mut layered_would_differ = 0;
    for preset in [Preset::MacosMonterey, Preset::MacosTahoe] {
        for ctx in [ThemeContext::light(), ThemeContext::dark()] {
            let r = Theme::preset(preset).resolve(&ctx);
            let hover = r.button.body(WidgetState::HOVER);
            let rest = fill(&r.button.body(WidgetState::NONE), "fill")?;
            if !hover.from_state.contains(PartStated::FILL) || fill(&hover, "fill")? != rest {
                continue;
            }
            tried += 1;
            if crate::styles::composite_over(rest, rest) != rest {
                layered_would_differ += 1;
            }
            let theme = to_theme(&r)?;
            let style = super::button(&r)?;
            assert_eq!(
                background_color(style(&theme, Status::Hovered).background),
                background_color(style(&theme, Status::Active).background),
                "{preset:?}"
            );
        }
    }
    assert!(tried > 0, "no macOS preset states its hover as rest");
    assert!(
        layered_would_differ > 0,
        "no macOS scheme's rest fill is translucent, so the test cannot tell a whole fill from a layered one"
    );
    Ok(())
}

/// iced's own container's look, which a panel keeps per property where the
/// theme leaves it unstated: no fill, the inherited text colour, no line
/// and square corners (`container::Style::default()`).
fn iced_container() -> iced_widget::container::Style {
    iced_widget::container::Style::default()
}

/// The tooltip and the dialog keep iced's own container's look per property
/// (spec §10, rule 0), not as a whole: with the panel's fill unstated, no
/// fill and the theme's text colour and border; with the text colour
/// unstated, the inherited one and the theme's fill; with the panel's
/// stroke and corners unstated, iced's own border and the theme's fill.
#[test]
fn a_tooltip_and_a_dialog_keep_iceds_own_per_property() -> TestResult {
    let unfilled = |part: &mut native_theme::next::PartStyle| {
        part.stated = part.stated.difference(PartStated::FILL);
    };
    let unlined = |part: &mut native_theme::next::PartStyle| {
        part.stated = part
            .stated
            .difference(PartStated::STROKE | PartStated::CORNERS);
    };
    let uncoloured = |text: &mut native_theme::next::TextStyle| {
        text.stated = text.stated.difference(TextStated::COLOR);
    };
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let tooltip = super::tooltip(&r)?(&theme);
        let dialog = super::dialog(&r)?(&theme);
        let iced = iced_container();

        let mut t = r.clone();
        unfilled(&mut t.tooltip.panel);
        unfilled(&mut t.dialog.panel);
        let (tip, dlg) = (super::tooltip(&t)?(&theme), super::dialog(&t)?(&theme));
        assert_eq!(tip.background, iced.background, "{preset:?} tooltip fill");
        assert_eq!(
            tip.text_color, tooltip.text_color,
            "{preset:?} tooltip text"
        );
        assert_eq!(dlg.background, iced.background, "{preset:?} dialog fill");
        assert_eq!(dlg.text_color, dialog.text_color, "{preset:?} dialog text");

        let mut t = r.clone();
        uncoloured(&mut t.tooltip.text);
        uncoloured(&mut t.dialog.body);
        let (tip, dlg) = (super::tooltip(&t)?(&theme), super::dialog(&t)?(&theme));
        assert_eq!(tip.text_color, iced.text_color, "{preset:?} tooltip text");
        assert_eq!(
            tip.background, tooltip.background,
            "{preset:?} tooltip fill"
        );
        assert_eq!(dlg.text_color, iced.text_color, "{preset:?} dialog text");
        assert_eq!(dlg.background, dialog.background, "{preset:?} dialog fill");

        let mut t = r.clone();
        unlined(&mut t.tooltip.panel);
        unlined(&mut t.dialog.panel);
        let (tip, dlg) = (super::tooltip(&t)?(&theme), super::dialog(&t)?(&theme));
        assert_eq!(tip.border, iced.border, "{preset:?} tooltip border");
        assert_eq!(
            tip.background, tooltip.background,
            "{preset:?} tooltip fill"
        );
        assert_eq!(dlg.border, iced.border, "{preset:?} dialog border");
        assert_eq!(dlg.background, dialog.background, "{preset:?} dialog fill");
    }
    Ok(())
}

/// The toolbar keeps iced's own container's look per property (spec §10,
/// rule 0): a stated bar fill is drawn whether or not the theme states the
/// button label's colour, and a stated label colour whether or not it
/// states the fill, each unstated one iced's own.
#[test]
fn a_toolbar_keeps_iceds_own_per_property() -> TestResult {
    let filled = ThemePatch::new()
        .toolbar(ToolbarRule::new().bar(PartPatch::new().fill(Paint::Solid(DISTINCT))));
    let lettered =
        ThemePatch::new().toolbar(ToolbarRule::new().button_label(TextPatch::new().color(LABEL)));
    let iced = iced_container();
    for (preset, r) in every_preset_with(&filled) {
        let theme = to_theme(&r)?;
        let bar = super::toolbar(&r)?(&theme);
        assert_eq!(
            background_color(bar.background),
            Some(to_color(DISTINCT)),
            "{preset:?}"
        );
        let label = super::text_color_of(&r.toolbar.button_label(WidgetState::NONE));
        assert_eq!(bar.text_color, label.or(iced.text_color), "{preset:?}");
    }
    for (preset, r) in every_preset_with(&lettered) {
        let theme = to_theme(&r)?;
        let bar = super::toolbar(&r)?(&theme);
        assert_eq!(bar.text_color, Some(to_color(LABEL)), "{preset:?}");
        let fill = super::super::fill_of(&r.toolbar.bar(WidgetState::NONE), "toolbar")?;
        assert_eq!(
            background_color(bar.background),
            fill.or(background_color(iced.background)),
            "{preset:?}"
        );
    }
    Ok(())
}

/// The card keeps iced's own container's look per property (spec §10,
/// rule 0): a stated fill is drawn with the theme's border, iced's own
/// where the theme states none, and a card whose border is iced's own still
/// draws its stated fill.
#[test]
fn a_card_keeps_iceds_own_per_property() -> TestResult {
    use native_theme::next::CardRule;
    let filled = ThemePatch::new()
        .card(CardRule::new().container(PartPatch::new().fill(Paint::Solid(DISTINCT))));
    for (preset, r) in every_preset_with(&filled) {
        let theme = to_theme(&r)?;
        let card = super::container_card(&r)?(&theme);
        assert_eq!(
            background_color(card.background),
            Some(to_color(DISTINCT)),
            "{preset:?}"
        );
        let container = r.card.container(WidgetState::NONE);
        if !container
            .stated
            .intersects(PartStated::STROKE | PartStated::CORNERS)
        {
            assert_eq!(card.border, iced_container().border, "{preset:?}");
        }
    }
    Ok(())
}

/// The plain and the primary button keep iced's own class's look at rest
/// per property (spec §10, rule 0), not as a whole: with the fill at rest
/// unstated, iced's own fill and the theme's label; with the label
/// unstated, iced's own label and the theme's fill; with the stroke and the
/// corners unstated, iced's own border and the theme's fill.
#[test]
fn a_button_keeps_iceds_own_per_property_at_rest() -> TestResult {
    use crate::styles::{ButtonSource, PartOutline, button_from, button_primary_from};
    use iced_widget::button::{Status, primary, secondary};
    let unlined = PartOutline {
        stroke: None,
        radius: None,
    };
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let b = super::button_source(&r)?;
        let at_rest = |source: &ButtonSource| button_from(source)(&theme, Status::Active);
        let stated = at_rest(&b);
        let own = secondary(&theme, Status::Active);

        let s = at_rest(&ButtonSource {
            background: None,
            ..b
        });
        assert_eq!(s.background, own.background, "{preset:?} fill");
        assert_eq!(s.text_color, stated.text_color, "{preset:?} label");
        let s = at_rest(&ButtonSource { label: None, ..b });
        assert_eq!(s.text_color, own.text_color, "{preset:?} label");
        assert_eq!(s.background, stated.background, "{preset:?} fill");
        let s = at_rest(&ButtonSource {
            border: unlined,
            ..b
        });
        assert_eq!(s.border, own.border, "{preset:?} border");
        assert_eq!(s.background, stated.background, "{preset:?} fill");

        let at_rest = |source: &ButtonSource| button_primary_from(source)(&theme, Status::Active);
        let stated = at_rest(&b);
        let own = primary(&theme, Status::Active);
        let s = at_rest(&ButtonSource {
            primary_background: None,
            ..b
        });
        assert_eq!(s.background, own.background, "{preset:?} primary fill");
        assert_eq!(s.text_color, stated.text_color, "{preset:?} primary label");
        let s = at_rest(&ButtonSource {
            primary_label: None,
            ..b
        });
        assert_eq!(s.text_color, own.text_color, "{preset:?} primary label");
        assert_eq!(s.background, stated.background, "{preset:?} primary fill");
        let s = at_rest(&ButtonSource {
            primary_border: unlined,
            ..b
        });
        assert_eq!(s.border, own.border, "{preset:?} primary border");
        assert_eq!(s.background, stated.background, "{preset:?} primary fill");
    }
    Ok(())
}

/// The link keeps iced's own text button's look per property (spec §10,
/// rule 0): with the fill at rest unstated, iced's own fill and the
/// theme's label; with the label at rest unstated, iced's own label and
/// the theme's fill; with the disabled label unstated, iced's own disabled
/// label.
#[test]
fn a_link_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::{LinkSource, button_link_from};
    use iced_widget::button::{Status, text};
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let l = super::link_source(&r)?;
        let at = |source: &LinkSource, status| button_link_from(source)(&theme, status);
        let stated = at(&l, Status::Active);
        let own = text(&theme, Status::Active);

        let s = at(
            &LinkSource {
                background: None,
                ..l
            },
            Status::Active,
        );
        assert_eq!(s.background, own.background, "{preset:?} fill");
        assert_eq!(s.text_color, stated.text_color, "{preset:?} label");
        let s = at(&LinkSource { label: None, ..l }, Status::Active);
        assert_eq!(s.text_color, own.text_color, "{preset:?} label");
        assert_eq!(s.background, stated.background, "{preset:?} fill");
        let s = at(
            &LinkSource {
                disabled_label: None,
                ..l
            },
            Status::Disabled,
        );
        assert_eq!(
            s.text_color,
            text(&theme, Status::Disabled).text_color,
            "{preset:?} disabled label"
        );
    }
    Ok(())
}

/// The check box keeps iced's own box's look per property (spec §10, rule
/// 0): with the fill at rest unstated, iced's own fill and the theme's
/// label; with the label unstated, iced's own and the theme's fill; with
/// the mark unstated, iced's own mark; with the stroke, its width and the
/// corners unstated, iced's own border and the theme's fill.
#[test]
fn a_check_box_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::{CheckboxSource, checkbox_from};
    use iced_widget::checkbox::{Status, primary};
    let (off, on) = (
        Status::Active { is_checked: false },
        Status::Active { is_checked: true },
    );
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let c = super::checkbox_source(&r)?;
        let ground = IcedColor::WHITE;
        let at = |source: &CheckboxSource, status| checkbox_from(source, ground)(&theme, status);
        let stated = at(&c, off);

        let mut fills = c.fills;
        fills.unchecked = None;
        let s = at(&CheckboxSource { fills, ..c }, off);
        assert_eq!(
            s.background,
            primary(&theme, off).background,
            "{preset:?} fill"
        );
        assert_eq!(s.text_color, stated.text_color, "{preset:?} label");

        let s = at(&CheckboxSource { label: None, ..c }, off);
        assert_eq!(
            s.text_color,
            primary(&theme, off).text_color,
            "{preset:?} label"
        );
        assert_eq!(s.background, stated.background, "{preset:?} fill");

        let s = at(&CheckboxSource { mark: None, ..c }, on);
        assert_eq!(
            s.icon_color,
            primary(&theme, on).icon_color,
            "{preset:?} mark"
        );

        let mut fills = c.fills;
        fills.unchecked_border = None;
        let s = at(
            &CheckboxSource {
                fills,
                border_width: None,
                border_radius: None,
                ..c
            },
            off,
        );
        assert_eq!(s.border, primary(&theme, off).border, "{preset:?} border");
        assert_eq!(s.background, stated.background, "{preset:?} fill");
    }
    Ok(())
}

/// The radio keeps iced's own radio's look per property (spec §10, rule
/// 0): with the fill at rest unstated, iced's own fill and the theme's
/// label; with the label unstated, iced's own and the theme's fill; with
/// the dot unstated, iced's own dot; with the stroke and its width
/// unstated, iced's own border and the theme's fill.
#[test]
fn a_radio_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::radio_from;
    use iced_widget::radio::{Status, default};
    let off = Status::Active { is_selected: false };
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let (fills, dot, label, width) = super::radio_source(&r)?;
        let own = default(&theme, off);
        let stated = radio_from(fills, dot, label, width)(&theme, off);

        let mut unfilled = fills;
        unfilled.unchecked = None;
        let s = radio_from(unfilled, dot, label, width)(&theme, off);
        assert_eq!(s.background, own.background, "{preset:?} fill");
        assert_eq!(s.text_color, stated.text_color, "{preset:?} label");

        let s = radio_from(fills, dot, None, width)(&theme, off);
        assert_eq!(s.text_color, own.text_color, "{preset:?} label");
        assert_eq!(s.background, stated.background, "{preset:?} fill");

        let s = radio_from(fills, None, label, width)(&theme, off);
        assert_eq!(s.dot_color, own.dot_color, "{preset:?} dot");

        let mut unlined = fills;
        unlined.unchecked_border = None;
        let s = radio_from(unlined, dot, label, None)(&theme, off);
        assert_eq!(
            (s.border_color, s.border_width),
            (own.border_color, own.border_width),
            "{preset:?} border"
        );
        assert_eq!(s.background, stated.background, "{preset:?} fill");
    }
    Ok(())
}

/// The switch keeps iced's own switch's look per property (spec §10, rule
/// 0), on and off: with the track at rest unstated, iced's own track and
/// the theme's thumb; with the thumb at rest unstated, iced's own thumb and
/// the theme's track.
#[test]
fn a_switch_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::{SwitchSource, toggler_from};
    use iced_widget::toggler::{Status, default};
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let source = super::switch_source(&r)?;
        for is_toggled in [false, true] {
            let status = Status::Active { is_toggled };
            let at = |source: &SwitchSource| toggler_from(source, IcedColor::WHITE)(&theme, status);
            let stated = at(&source);
            let own = default(&theme, status);

            let s = at(&SwitchSource {
                checked_background: None,
                unchecked_background: None,
                ..source
            });
            assert_eq!(
                s.background, own.background,
                "{preset:?} {is_toggled} track"
            );
            assert_eq!(
                s.foreground, stated.foreground,
                "{preset:?} {is_toggled} thumb"
            );

            let s = at(&SwitchSource {
                thumb_background: None,
                unchecked_thumb_background: None,
                ..source
            });
            assert_eq!(
                s.foreground, own.foreground,
                "{preset:?} {is_toggled} thumb"
            );
            assert_eq!(
                s.background, stated.background,
                "{preset:?} {is_toggled} track"
            );
        }
    }
    Ok(())
}

/// The combo box's field keeps iced's own field's look per property (spec
/// §10, rule 0): with the fill at rest unstated, iced's own fill and the
/// theme's label; with the label unstated, iced's own and the theme's fill;
/// with the placeholder unstated, iced's own; with the stroke and the
/// corners unstated, iced's own border and the theme's fill.
#[test]
fn a_pick_list_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::{PartOutline, PickListSource, pick_list_from};
    use iced_widget::pick_list::{Status, default};
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let c = super::pick_list_source(&r)?;
        let at = |source: &PickListSource| pick_list_from(source)(&theme, Status::Active);
        let stated = at(&c);
        let own = default(&theme, Status::Active);

        let s = at(&PickListSource {
            background: None,
            ..c
        });
        assert_eq!(s.background, own.background, "{preset:?} fill");
        assert_eq!(s.text_color, stated.text_color, "{preset:?} label");

        let s = at(&PickListSource { label: None, ..c });
        assert_eq!(s.text_color, own.text_color, "{preset:?} label");
        assert_eq!(s.background, stated.background, "{preset:?} fill");

        let s = at(&PickListSource {
            placeholder: None,
            ..c
        });
        assert_eq!(
            s.placeholder_color, own.placeholder_color,
            "{preset:?} placeholder"
        );

        let s = at(&PickListSource {
            border: PartOutline {
                stroke: None,
                radius: None,
            },
            ..c
        });
        assert_eq!(s.border, own.border, "{preset:?} border");
        assert_eq!(s.background, stated.background, "{preset:?} fill");
    }
    Ok(())
}

/// The drop-down menu keeps iced's own menu's look per property (spec §10,
/// rule 0): with the panel's fill unstated, iced's own fill and the theme's
/// label; with the label unstated, iced's own label and the theme's fill.
#[test]
fn a_menu_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::{MenuSource, menu_from};
    use iced_widget::overlay::menu::default;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let m = super::menu_source(&r)?;
        let stated = menu_from(&m)(&theme);
        let own = default(&theme);

        let s = menu_from(&MenuSource {
            background: None,
            ..m
        })(&theme);
        assert_eq!(s.background, own.background, "{preset:?} fill");
        assert_eq!(s.text_color, stated.text_color, "{preset:?} label");

        let s = menu_from(&MenuSource { label: None, ..m })(&theme);
        assert_eq!(s.text_color, own.text_color, "{preset:?} label");
        assert_eq!(s.background, stated.background, "{preset:?} fill");
    }
    Ok(())
}

/// The text area and the text field keep iced's own field's look per
/// property (spec §10, rule 0): with the fill at rest unstated, iced's own
/// fill and the theme's value; with the value unstated, iced's own and the
/// theme's fill; with the disabled value or the placeholder unstated,
/// iced's own; with the stroke and the corners unstated, iced's own border
/// and the theme's fill.
#[test]
fn a_text_area_and_a_text_field_keep_iceds_own_per_property() -> TestResult {
    use crate::styles::{InputSource, PartOutline, text_editor_from, text_input_from};
    macro_rules! per_property {
        ($preset:expr, $theme:expr, $source:expr, $from:ident, $widget:ident) => {{
            use iced_widget::$widget::{Status, default};
            let (preset, theme, i) = ($preset, $theme, $source);
            let at = |source: InputSource, status| $from(&source)(theme, status);
            let stated = at(i, Status::Active);
            let own = default(theme, Status::Active);
            let what = stringify!($widget);

            let s = at(
                InputSource {
                    background: None,
                    ..i
                },
                Status::Active,
            );
            assert_eq!(s.background, own.background, "{preset:?} {what} fill");
            assert_eq!(s.value, stated.value, "{preset:?} {what} value");

            let s = at(InputSource { text: None, ..i }, Status::Active);
            assert_eq!(s.value, own.value, "{preset:?} {what} value");
            assert_eq!(s.background, stated.background, "{preset:?} {what} fill");

            let s = at(
                InputSource {
                    placeholder: None,
                    ..i
                },
                Status::Active,
            );
            assert_eq!(
                s.placeholder, own.placeholder,
                "{preset:?} {what} placeholder"
            );

            let s = at(
                InputSource {
                    disabled_text: None,
                    ..i
                },
                Status::Disabled,
            );
            assert_eq!(
                s.value,
                default(theme, Status::Disabled).value,
                "{preset:?} {what} disabled value"
            );

            let s = at(
                InputSource {
                    border: PartOutline {
                        stroke: None,
                        radius: None,
                    },
                    ..i
                },
                Status::Active,
            );
            assert_eq!(s.border, own.border, "{preset:?} {what} border");
            assert_eq!(s.background, stated.background, "{preset:?} {what} fill");
        }};
    }
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        per_property!(
            preset,
            &theme,
            super::text_area_source(&r)?,
            text_editor_from,
            text_editor
        );
        per_property!(
            preset,
            &theme,
            super::text_input_source(&r)?,
            text_input_from,
            text_input
        );
    }
    Ok(())
}

/// The slider keeps iced's own slider's look per property (spec §10, rule
/// 0), along either axis: with the thumb's fill unstated, iced's own and
/// the theme's rail; with the filled rail unstated, iced's own and the
/// theme's thumb; with the rest of the rail unstated, iced's own.
#[test]
fn a_slider_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::{SliderSource, slider_from};
    use iced_widget::slider::{Status, default};
    use native_theme::next::Orientation;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let own = default(&theme, Status::Active);
        for orientation in [Orientation::Horizontal, Orientation::Vertical] {
            let source = super::slider_source(&r, orientation)?;
            let at = |source: &SliderSource| slider_from(source)(&theme, Status::Active);
            let stated = at(&source);

            let s = at(&SliderSource {
                thumb_color: None,
                ..source
            });
            assert_eq!(
                s.handle.background, own.handle.background,
                "{preset:?} {orientation:?} thumb"
            );
            assert_eq!(
                s.rail.backgrounds, stated.rail.backgrounds,
                "{preset:?} {orientation:?} rail"
            );

            let s = at(&SliderSource {
                fill_color: None,
                ..source
            });
            assert_eq!(
                s.rail.backgrounds.0, own.rail.backgrounds.0,
                "{preset:?} {orientation:?} filled rail"
            );
            assert_eq!(
                s.handle.background, stated.handle.background,
                "{preset:?} {orientation:?} thumb"
            );

            let s = at(&SliderSource {
                track_color: None,
                ..source
            });
            assert_eq!(
                s.rail.backgrounds.1, own.rail.backgrounds.1,
                "{preset:?} {orientation:?} rail"
            );
        }
    }
    Ok(())
}

/// The progress bar keeps iced's own bar's look per property (spec §10,
/// rule 0): with the track's fill unstated, iced's own track and the
/// theme's bar; with the bar's fill unstated, iced's own bar and the
/// theme's track; with the stroke and the corners unstated, iced's own
/// border.
#[test]
fn a_progress_bar_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::{PartOutline, progress_bar_from};
    use iced_widget::progress_bar::primary;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let (track, filled, border) = super::progress_bar_source(&r)?;
        let stated = progress_bar_from(track, filled, border)(&theme);
        let own = primary(&theme);

        let s = progress_bar_from(None, filled, border)(&theme);
        assert_eq!(s.background, own.background, "{preset:?} track");
        assert_eq!(s.bar, stated.bar, "{preset:?} bar");

        let s = progress_bar_from(track, None, border)(&theme);
        assert_eq!(s.bar, own.bar, "{preset:?} bar");
        assert_eq!(s.background, stated.background, "{preset:?} track");

        let unlined = PartOutline {
            stroke: None,
            radius: None,
        };
        let s = progress_bar_from(track, filled, unlined)(&theme);
        assert_eq!(s.border, own.border, "{preset:?} border");
    }
    Ok(())
}

/// The scrollable keeps iced's own scrollable's look per property (spec
/// §10, rule 0), in a view with the focus and in one without: with the
/// scroller's fill at rest unstated, iced's own scroller and the theme's
/// rail; with the rail's fill unstated, iced's own rail and the theme's
/// scroller. The pointer is over the view, not a scroller, which shows a
/// floating bar at rest.
#[test]
fn a_scrollable_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::{ScrollbarSource, scrollable_from};
    use iced_widget::scrollable::{Status, default};
    let over_the_view = Status::Hovered {
        is_horizontal_scrollbar_hovered: false,
        is_vertical_scrollbar_hovered: false,
        is_horizontal_scrollbar_disabled: false,
        is_vertical_scrollbar_disabled: false,
    };
    let at_rest = Status::Active {
        is_horizontal_scrollbar_disabled: false,
        is_vertical_scrollbar_disabled: false,
    };
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let own = default(&theme, over_the_view);
        let own_scroller = default(&theme, at_rest).vertical_rail.scroller.background;
        for view in [WidgetState::NONE, WidgetState::UNFOCUSED_VIEW] {
            let source = super::scrollbar_source(&r, view)?;
            let at = |source: &ScrollbarSource| scrollable_from(source)(&theme, over_the_view);
            let stated = at(&source);

            let s = at(&ScrollbarSource {
                thumb: None,
                ..source
            });
            assert_eq!(
                s.vertical_rail.scroller.background, own_scroller,
                "{preset:?} {view:?} scroller"
            );
            assert_eq!(
                s.vertical_rail.background, stated.vertical_rail.background,
                "{preset:?} {view:?} rail"
            );

            let s = at(&ScrollbarSource {
                track: None,
                ..source
            });
            assert_eq!(
                s.vertical_rail.background, own.vertical_rail.background,
                "{preset:?} {view:?} rail"
            );
            assert_eq!(
                s.vertical_rail.scroller.background, stated.vertical_rail.scroller.background,
                "{preset:?} {view:?} scroller"
            );
        }
    }
    Ok(())
}

/// The segmented control keeps iced's own look per property (spec §10,
/// rule 0), the group iced's own container's and a segment iced's own
/// button's (`button::Style::default()`): with the group fill unstated, no
/// fill and the theme's label; with the outline unstated, iced's own border
/// and the theme's fill; with an unselected segment's label unstated,
/// iced's own and the selected one's as stated; with the selected
/// segment's fill or label unstated, iced's own and the other as stated.
#[test]
fn a_segmented_control_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::{
        PartOutline, SegmentPosition, SegmentedSource, segment_from, segmented_control_from,
    };
    use iced_widget::button::Status;
    let group = iced_container();
    let own = iced_widget::button::Style::default();
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let source = super::segmented_source(&r)?;
        let control = |s: &SegmentedSource| segmented_control_from(s, false)(&theme);
        let segment = |s: &SegmentedSource, selected: bool| {
            segment_from(s, selected, SegmentPosition::Middle)(&theme, Status::Active)
        };
        let stated = control(&source);
        let (stated_off, stated_on) = (segment(&source, false), segment(&source, true));

        let s = SegmentedSource {
            background: None,
            ..source
        };
        assert_eq!(control(&s).background, group.background, "{preset:?} fill");
        assert_eq!(
            control(&s).text_color,
            stated.text_color,
            "{preset:?} label"
        );

        let s = SegmentedSource {
            border: PartOutline {
                stroke: None,
                radius: None,
            },
            ..source
        };
        assert_eq!(control(&s).border, group.border, "{preset:?} border");
        assert_eq!(control(&s).background, stated.background, "{preset:?} fill");

        let s = SegmentedSource {
            label: None,
            ..source
        };
        assert_eq!(control(&s).text_color, group.text_color, "{preset:?} label");
        assert_eq!(
            segment(&s, false).text_color,
            own.text_color,
            "{preset:?} segment label"
        );
        assert_eq!(
            segment(&s, true).text_color,
            stated_on.text_color,
            "{preset:?} selected label"
        );

        let s = SegmentedSource {
            active_background: None,
            ..source
        };
        assert_eq!(
            segment(&s, true).background,
            own.background,
            "{preset:?} selected fill"
        );
        assert_eq!(
            segment(&s, true).text_color,
            stated_on.text_color,
            "{preset:?} selected label"
        );

        let s = SegmentedSource {
            active_text_color: None,
            ..source
        };
        assert_eq!(
            segment(&s, true).text_color,
            own.text_color,
            "{preset:?} selected label"
        );
        assert_eq!(
            segment(&s, true).background,
            stated_on.background,
            "{preset:?} selected fill"
        );
        assert_eq!(
            segment(&s, false).text_color,
            stated_off.text_color,
            "{preset:?} segment label"
        );
    }
    Ok(())
}

/// The expander keeps iced's own look per property (spec §10, rule 0):
/// with the header's label unstated, iced's own header's
/// (`button::Style::default()`), at rest and disabled, and the frame's
/// outline as stated; with the frame's stroke and corners unstated, iced's
/// own container's border and the header's label as stated.
#[test]
fn an_expander_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::{ExpanderSource, PartOutline, expander_frame_from, expander_from};
    use iced_widget::button::Status;
    let own = iced_widget::button::Style::default();
    let unlined = PartOutline {
        stroke: None,
        radius: None,
    };
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let x = ExpanderSource {
            frame_enabled: Some(true),
            ..super::expander_source(&r)?
        };
        let stated_frame = expander_frame_from(&x)(&theme);

        let s = ExpanderSource { label: None, ..x };
        for status in [Status::Active, Status::Disabled] {
            assert_eq!(
                expander_from(&s)(&theme, status).text_color,
                own.text_color,
                "{preset:?} {status:?} label"
            );
        }
        assert_eq!(
            expander_frame_from(&s)(&theme).border,
            stated_frame.border,
            "{preset:?} frame"
        );

        let s = ExpanderSource {
            border: unlined,
            ..x
        };
        assert_eq!(
            expander_frame_from(&s)(&theme).border,
            iced_container().border,
            "{preset:?} frame"
        );
        assert_eq!(
            expander_from(&s)(&theme, Status::Active).text_color,
            expander_from(&x)(&theme, Status::Active).text_color,
            "{preset:?} label"
        );
    }
    Ok(())
}

/// The pane grid keeps iced's own split per property (spec §10, rule 0):
/// with the hovered handle's fill unstated, iced's own hovered and picked
/// split colours at the theme's divider width; with it stated, that fill
/// at the same width; the drop region iced's own either way.
#[test]
fn a_pane_grid_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::pane_grid_from;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let own = iced_widget::pane_grid::default(&theme);
        let width = r.splitter.divider_width;
        let handle = r.splitter.handle(WidgetState::HOVER);
        let grid = super::pane_grid(&r)?(&theme);
        let expected = super::super::fill_of(&handle, "handle")?;
        assert_eq!(
            grid.hovered_split.color,
            expected.unwrap_or(own.hovered_split.color),
            "{preset:?}"
        );

        let s = pane_grid_from(None, width)(&theme);
        assert_eq!(s.hovered_split.color, own.hovered_split.color, "{preset:?}");
        assert_eq!(s.picked_split.color, own.picked_split.color, "{preset:?}");
        assert_eq!(s.hovered_split.width, width, "{preset:?}");
        assert_eq!(s.picked_split.width, width, "{preset:?}");
        assert_eq!(s.hovered_region, own.hovered_region, "{preset:?}");

        let s = pane_grid_from(Some(to_color(DISTINCT)), width)(&theme);
        assert_eq!(s.picked_split.color, to_color(DISTINCT), "{preset:?}");
        assert_eq!(s.hovered_split.width, width, "{preset:?}");
    }
    Ok(())
}

/// The application's style keeps the theme's base style per property
/// (spec §10, rule 0): with the window frame's fill unstated, the base
/// style's background; the text colour the base style's either way.
#[test]
fn an_application_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::application_from;
    use iced_core::theme::Base;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let base = theme.base();
        let frame = r.window.frame(native_theme::next::WindowState::default());
        let stated = super::application(&r)?(&theme);
        assert_eq!(
            stated.background_color,
            super::super::fill_of(&frame, "frame")?.unwrap_or(base.background_color),
            "{preset:?}"
        );
        assert_eq!(stated.text_color, base.text_color, "{preset:?}");

        let s = application_from(None)(&theme);
        assert_eq!(s.background_color, base.background_color, "{preset:?}");
        assert_eq!(s.text_color, base.text_color, "{preset:?}");
    }
    Ok(())
}

/// The toolbar's rule keeps iced's own rule per property (spec §10, rule
/// 0): the bar's stroke colour where stated, else iced's own rule's colour;
/// the corners, the fill mode and the snapping iced's own either way.
#[test]
fn a_toolbar_rule_keeps_iceds_own_per_property() -> TestResult {
    use crate::styles::rule_from;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let own = iced_widget::rule::default(&theme);
        let bar = r.toolbar.bar(WidgetState::NONE);
        let stated = super::toolbar_rule(&r)?(&theme);
        assert_eq!(
            stated.color,
            super::super::stroke_color_of(&bar).unwrap_or(own.color),
            "{preset:?}"
        );

        let s = rule_from(None)(&theme);
        assert_eq!(s.color, own.color, "{preset:?}");
        assert_eq!(s.radius, own.radius, "{preset:?}");
        assert_eq!(s.fill_mode, own.fill_mode, "{preset:?}");
        assert_eq!(s.snap, own.snap, "{preset:?}");
    }
    Ok(())
}

/// A toolbar stroke stated as none is drawn as none (spec §10, rule 0): the
/// rule under the bar is transparent, its corners, fill mode and snapping
/// iced's own rule's; on every preset with a rule stating no stroke.
#[test]
fn a_toolbar_stroke_stated_none_draws_no_rule() -> TestResult {
    let patch = ThemePatch::from_toml("[[widgets.toolbar.rules]]\nbar.stroke = \"none\"\n")?;
    for (preset, r) in every_preset_with(&patch) {
        let bar = r.toolbar.bar(WidgetState::NONE);
        assert!(bar.stated.contains(PartStated::STROKE), "{preset:?}");
        assert_eq!(bar.stroke, Stroke::None, "{preset:?}");
        let theme = to_theme(&r)?;
        let own = iced_widget::rule::default(&theme);
        let s = super::toolbar_rule(&r)?(&theme);
        assert_eq!(s.color, IcedColor::TRANSPARENT, "{preset:?}");
        assert_eq!(s.radius, own.radius, "{preset:?}");
        assert_eq!(s.fill_mode, own.fill_mode, "{preset:?}");
        assert_eq!(s.snap, own.snap, "{preset:?}");
    }
    Ok(())
}

/// The `iced_aw` card keeps `iced_aw`'s own card's look per property (spec
/// §10, rule 0): with the fill unstated, its own fill and the theme's
/// text; with the window's foreground unstated, its own text colours and
/// close icon and the theme's fill.
#[cfg(feature = "iced_aw")]
#[test]
fn an_aw_card_keeps_iced_aws_own_per_property() -> TestResult {
    use crate::styles::aw::{CardSource, card_from};
    use iced_aw::style::Status;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let c = super::aw::card_source(&r)?;
        let at = |source: &CardSource| card_from(source)(&theme, Status::Active);
        let stated = at(&c);
        let own = iced_aw::style::card::primary(&theme, Status::Active);

        let s = at(&CardSource {
            background: None,
            ..c
        });
        assert_eq!(s.background, own.background, "{preset:?} fill");
        assert_eq!(s.body_text_color, stated.body_text_color, "{preset:?} text");

        let s = at(&CardSource { label: None, ..c });
        assert_eq!(
            (
                s.head_text_color,
                s.body_text_color,
                s.foot_text_color,
                s.close_color
            ),
            (
                own.head_text_color,
                own.body_text_color,
                own.foot_text_color,
                own.close_color
            ),
            "{preset:?} text"
        );
        assert_eq!(s.background, stated.background, "{preset:?} fill");
    }
    Ok(())
}

/// The `iced_aw` menu bar keeps `iced_aw`'s own look per property (spec
/// §10, rule 0): with the bar's fill unstated, its own bar and the theme's
/// menu panel; with the panel's fill unstated, its own panel and the
/// theme's bar.
#[cfg(feature = "iced_aw")]
#[test]
fn an_aw_menu_keeps_iced_aws_own_per_property() -> TestResult {
    use crate::styles::aw::{MenuSource, menu_from};
    use iced_aw::style::Status;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let m = super::aw::menu_source(&r)?;
        let at = |source: &MenuSource| menu_from(source)(&theme, Status::Active);
        let stated = at(&m);
        let own = iced_aw::style::menu_bar::primary(&theme, Status::Active);

        let s = at(&MenuSource { bar: None, ..m });
        assert_eq!(s.bar_background, own.bar_background, "{preset:?} bar");
        assert_eq!(
            s.menu_background, stated.menu_background,
            "{preset:?} panel"
        );

        let s = at(&MenuSource { panel: None, ..m });
        assert_eq!(s.menu_background, own.menu_background, "{preset:?} panel");
        assert_eq!(s.bar_background, stated.bar_background, "{preset:?} bar");
    }
    Ok(())
}

/// The `iced_aw` tab bar keeps `iced_aw`'s own look per property (spec
/// §10, rule 0), each in the status `iced_aw` draws it in (`Active` the
/// selected tab, `Disabled` one at rest): with the strip's fill unstated,
/// its own strip; with a tab's fill or label unstated, its own for that
/// tab, and the other tab's as stated.
#[cfg(feature = "iced_aw")]
#[test]
fn an_aw_tab_bar_keeps_iced_aws_own_per_property() -> TestResult {
    use crate::styles::aw::{TabBarSource, tab_bar_from};
    use iced_aw::style::Status;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let t = super::aw::tab_bar_source(&r)?;
        let at = |source: &TabBarSource, status| tab_bar_from(source)(&theme, status);
        let (on, off) = (at(&t, Status::Active), at(&t, Status::Disabled));
        let own_on = iced_aw::style::tab_bar::primary(&theme, Status::Active);
        let own_off = iced_aw::style::tab_bar::primary(&theme, Status::Disabled);

        let s = TabBarSource {
            bar_background: None,
            ..t
        };
        assert_eq!(
            at(&s, Status::Active).background,
            own_on.background,
            "{preset:?} strip"
        );
        assert_eq!(
            at(&s, Status::Active).tab_label_background,
            on.tab_label_background,
            "{preset:?} selected fill"
        );

        let s = TabBarSource {
            background: None,
            ..t
        };
        assert_eq!(
            at(&s, Status::Disabled).tab_label_background,
            own_off.tab_label_background,
            "{preset:?} fill"
        );
        assert_eq!(
            at(&s, Status::Active).tab_label_background,
            on.tab_label_background,
            "{preset:?} selected fill"
        );

        let s = TabBarSource {
            active_background: None,
            ..t
        };
        assert_eq!(
            at(&s, Status::Active).tab_label_background,
            own_on.tab_label_background,
            "{preset:?} selected fill"
        );
        assert_eq!(
            at(&s, Status::Disabled).tab_label_background,
            off.tab_label_background,
            "{preset:?} fill"
        );

        let s = TabBarSource { label: None, ..t };
        let s_off = at(&s, Status::Disabled);
        assert_eq!(
            (s_off.text_color, s_off.icon_color),
            (own_off.text_color, own_off.icon_color),
            "{preset:?} label"
        );
        assert_eq!(
            at(&s, Status::Active).text_color,
            on.text_color,
            "{preset:?} selected label"
        );

        let s = TabBarSource {
            active_label: None,
            ..t
        };
        let s_on = at(&s, Status::Active);
        assert_eq!(
            (s_on.text_color, s_on.icon_color),
            (own_on.text_color, own_on.icon_color),
            "{preset:?} selected label"
        );
        assert_eq!(
            at(&s, Status::Disabled).text_color,
            off.text_color,
            "{preset:?} label"
        );
    }
    Ok(())
}

/// An `iced_aw` sidebar item under the pointer is lettered, its icon too, in
/// the label a rule for `HOVER` states (`sidebar.item_label(HOVER).color`),
/// on every preset; with no such rule, in the label at rest.
#[cfg(feature = "iced_aw")]
#[test]
fn a_hovered_aw_sidebar_item_takes_the_stated_hover_label() -> TestResult {
    use iced_aw::style::Status;
    use native_theme::next::SidebarRule;
    let patch = ThemePatch::new().sidebar(
        SidebarRule::new()
            .states(WidgetState::HOVER)
            .item_label(TextPatch::new().color(LABEL)),
    );
    for (preset, r) in every_preset_with(&patch) {
        let theme = to_theme(&r)?;
        let hovered = super::aw::sidebar(&r)?(&theme, Status::Hovered);
        assert_eq!(hovered.text_color, to_color(LABEL), "{preset:?} label");
        assert_eq!(hovered.icon_color, to_color(LABEL), "{preset:?} icon");
    }
    for (preset, r) in every_preset() {
        if r.sidebar
            .item_label(WidgetState::HOVER)
            .from_state
            .contains(TextStated::COLOR)
        {
            continue;
        }
        let theme = to_theme(&r)?;
        let style = super::aw::sidebar(&r)?;
        let (hovered, rest) = (
            style(&theme, Status::Hovered),
            style(&theme, Status::Disabled),
        );
        assert_eq!(hovered.text_color, rest.text_color, "{preset:?} label");
    }
    Ok(())
}

/// The `iced_aw` sidebar keeps `iced_aw`'s own look per property (spec
/// §10, rule 0), each in the status `iced_aw` draws it in (`Active` the
/// selected item, `Disabled` one at rest): with the panel's fill unstated,
/// its own panel and item at rest; with the selected item's fill or a
/// label unstated, its own for that item, and the other's as stated.
#[cfg(feature = "iced_aw")]
#[test]
fn an_aw_sidebar_keeps_iced_aws_own_per_property() -> TestResult {
    use crate::styles::aw::{ListSource, sidebar_from};
    use iced_aw::style::Status;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let l = super::aw::sidebar_source(&r)?;
        let at = |source: &ListSource, status| sidebar_from(source)(&theme, status);
        let (on, off) = (at(&l, Status::Active), at(&l, Status::Disabled));
        let own_on = iced_aw::style::sidebar::primary(&theme, Status::Active);
        let own_off = iced_aw::style::sidebar::primary(&theme, Status::Disabled);

        let s = ListSource {
            background: None,
            ..l
        };
        let s_off = at(&s, Status::Disabled);
        assert_eq!(
            (s_off.background, s_off.tab_label_background),
            (own_off.background, own_off.tab_label_background),
            "{preset:?} panel"
        );
        assert_eq!(
            at(&s, Status::Active).tab_label_background,
            on.tab_label_background,
            "{preset:?} selected fill"
        );

        let s = ListSource {
            selection_background: None,
            ..l
        };
        assert_eq!(
            at(&s, Status::Active).tab_label_background,
            own_on.tab_label_background,
            "{preset:?} selected fill"
        );
        assert_eq!(
            at(&s, Status::Disabled).background,
            off.background,
            "{preset:?} panel"
        );

        let s = ListSource { label: None, ..l };
        let s_off = at(&s, Status::Disabled);
        assert_eq!(
            (s_off.text_color, s_off.icon_color),
            (own_off.text_color, own_off.icon_color),
            "{preset:?} label"
        );
        assert_eq!(
            at(&s, Status::Active).text_color,
            on.text_color,
            "{preset:?} selected label"
        );

        let s = ListSource {
            selection_label: None,
            ..l
        };
        let s_on = at(&s, Status::Active);
        assert_eq!(
            (s_on.text_color, s_on.icon_color),
            (own_on.text_color, own_on.icon_color),
            "{preset:?} selected label"
        );
        assert_eq!(
            at(&s, Status::Disabled).text_color,
            off.text_color,
            "{preset:?} label"
        );
    }
    Ok(())
}

/// The `iced_aw` selection list keeps `iced_aw`'s own look per property
/// (spec §10, rule 0), each in the status `iced_aw` draws it in: with the
/// list's fill unstated, its own and the theme's label; with a row's label
/// unstated at rest, selected or disabled, its own in that status and the
/// other rows' as stated; with the selected row's fill unstated, its own
/// and the theme's list fill.
#[cfg(feature = "iced_aw")]
#[test]
fn an_aw_selection_list_keeps_iced_aws_own_per_property() -> TestResult {
    use crate::styles::aw::{ListSource, selection_list_from};
    use iced_aw::style::Status;
    for (preset, r) in every_preset() {
        let theme = to_theme(&r)?;
        let l = super::aw::list_source(&r)?;
        let at = |source: &ListSource, status| selection_list_from(source)(&theme, status);
        let own = |status| iced_aw::style::selection_list::primary(&theme, status);
        let stated = |status| at(&l, status);

        let s = ListSource {
            background: None,
            ..l
        };
        assert_eq!(
            at(&s, Status::Active).background,
            own(Status::Active).background,
            "{preset:?} fill"
        );
        assert_eq!(
            at(&s, Status::Active).text_color,
            stated(Status::Active).text_color,
            "{preset:?} label"
        );

        let s = ListSource {
            selection_background: None,
            ..l
        };
        assert_eq!(
            at(&s, Status::Selected).background,
            own(Status::Selected).background,
            "{preset:?} selected fill"
        );
        assert_eq!(
            at(&s, Status::Active).background,
            stated(Status::Active).background,
            "{preset:?} fill"
        );

        for (s, status, other) in [
            (
                ListSource { label: None, ..l },
                Status::Active,
                Status::Selected,
            ),
            (
                ListSource {
                    selection_label: None,
                    ..l
                },
                Status::Selected,
                Status::Active,
            ),
            (
                ListSource {
                    disabled_label: None,
                    ..l
                },
                Status::Disabled,
                Status::Active,
            ),
        ] {
            assert_eq!(
                at(&s, status).text_color,
                own(status).text_color,
                "{preset:?} {status:?} label"
            );
            assert_eq!(
                at(&s, other).text_color,
                stated(other).text_color,
                "{preset:?} {other:?} label"
            );
        }
    }
    Ok(())
}
