//! The 0.8.0 style functions on themes built for the case: a stated value
//! the presets do not show, or one they leave unstated.

use iced_core::Background;
use iced_core::Color as IcedColor;
use native_theme::next::{
    Color, InputRule, Paint, PaintLayer, PartPatch, PartStated, Preset, ResolvedTheme, TextStated,
    Theme, ThemeContext, ThemePatch, WidgetState,
};

use super::super::borders::line;
use super::super::{fill, text_color, to_color, to_theme};
use crate::styles::ICED_DISABLED_BUTTON_ALPHA;

type TestResult = Result<(), Box<dyn std::error::Error>>;

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
