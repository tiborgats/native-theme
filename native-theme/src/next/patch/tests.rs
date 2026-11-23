use super::*;

use crate::next::geometry::StrokeAlign;

const RED: Color = Color::rgb(1.0, 0.0, 0.0);

/// The path and reason `patch.validate()` rejects it with.
fn rejected(patch: &ThemePatch) -> Result<(String, String), String> {
    match patch.validate() {
        Err(Error::InvalidTheme { path, reason }) => Ok((path, reason)),
        other => Err(format!("expected InvalidTheme, got {other:?}")),
    }
}

fn assert_rejected(patch: &ThemePatch, path: &str, reason: &str) -> Result<(), String> {
    let (got_path, got_reason) = rejected(patch)?;
    assert_eq!((got_path.as_str(), got_reason.as_str()), (path, reason));
    Ok(())
}

fn body(part: PartPatch) -> ThemePatch {
    ThemePatch::new().button(ButtonRule::new().body(part))
}

#[test]
fn setters_fill_the_fields() -> Result<(), String> {
    let patch = ThemePatch::new()
        .corner_scale(1.25)
        .spacing_scale(0.5)
        .font_family("Inter")
        .monospace_family("Iosevka")
        .custom_color("gutter", RED)
        .custom_metric("rail", 3.0)
        .button(
            ButtonRule::new()
                .variant(ButtonVariant::Primary)
                .shape(ButtonShape::Pill)
                .states(WidgetState::HOVER)
                .not_states(WidgetState::DISABLED)
                .size(ControlSize::Small)
                .body(PartPatch::new().fill(Paint::Solid(RED))),
        );
    assert_eq!(patch.corner_scale, Some(1.25));
    assert_eq!(patch.spacing_scale, Some(0.5));
    assert_eq!(patch.font_family.as_deref(), Some("Inter"));
    assert_eq!(patch.monospace_family.as_deref(), Some("Iosevka"));
    assert_eq!(patch.custom_colors.get("gutter"), Some(&RED));
    assert_eq!(patch.custom_metrics.get("rail"), Some(&3.0));
    let rule = patch
        .widget_rules
        .button
        .rules
        .first()
        .ok_or("no rule appended")?;
    assert_eq!(
        rule.selector.matcher,
        ButtonMatcher {
            variant: Some(ButtonVariant::Primary),
            shape: Some(ButtonShape::Pill),
        }
    );
    assert_eq!(rule.selector.required_states, WidgetState::HOVER);
    assert_eq!(rule.selector.forbidden_states, WidgetState::DISABLED);
    assert_eq!(rule.selector.size, Some(ControlSize::Small));
    assert_eq!(rule.body.fill, Some(Paint::Solid(RED)));
    assert_eq!(rule.label, TextPatch::new());
    patch.validate().map_err(|e| e.to_string())
}

#[test]
fn scales_are_clamped() {
    assert_eq!(
        ThemePatch::new().corner_scale(f32::NAN).corner_scale,
        Some(1.0)
    );
    assert_eq!(ThemePatch::new().corner_scale(-2.0).corner_scale, Some(0.0));
    assert_eq!(
        ThemePatch::new().spacing_scale(f32::INFINITY).spacing_scale,
        Some(f32::MAX)
    );
    assert_eq!(
        ThemePatch::new()
            .spacing_scale(f32::NEG_INFINITY)
            .spacing_scale,
        Some(0.0)
    );
}

#[test]
fn to_props_states_only_what_is_set() {
    let part = PartPatch::new()
        .fill(Paint::Solid(RED))
        .opacity(0.5)
        .shadows(None)
        .min_size([Some(4.0), None]);
    assert_eq!(
        part.to_props().collect::<Vec<_>>(),
        vec![
            PartProp::Fill(Paint::Solid(RED)),
            PartProp::Shadows(None),
            PartProp::Opacity(0.5),
            PartProp::MinSize([Some(4.0), None]),
        ]
    );
    assert_eq!(PartPatch::new().to_props().count(), 0);
    let text = TextPatch::new().family(Arc::from("Inter")).size_px(13.0);
    assert_eq!(
        text.to_props().collect::<Vec<_>>(),
        vec![TextProp::Family(Arc::from("Inter")), TextProp::SizePx(13.0)]
    );
    let icon = IconPatch::new().color(RED).stroke_width(None);
    assert_eq!(
        icon.to_props().collect::<Vec<_>>(),
        vec![IconProp::Color(RED), IconProp::StrokeWidth(None)]
    );
}

#[test]
fn a_patch_of_valid_values_validates() -> Result<(), String> {
    let mut colors = ColorPatch::new();
    colors.window.background = Some(RED);
    colors.disabled_opacity = Some(0.4);
    colors.error.fill = Some(RED);
    let patch = ThemePatch::new()
        .corner_scale(0.0)
        .color_overrides(colors)
        .button(
            ButtonRule::new().body(
                PartPatch::new()
                    .stroke(Stroke::uniform_aligned(1.0, RED, StrokeAlign::Center))
                    .corners(Corners::uniform(0.0))
                    .opacity(1.0)
                    .padding(Edges::all(2.0))
                    .margin(Edges::all(-1.0)),
            ),
        )
        .widget_rules(
            WidgetPatchMap::new()
                .tooltip(TooltipPatch::new().panel(PartPatch::new().fill(Paint::Solid(RED)))),
        )
        .custom_metric("offset", -3.0);
    patch.validate().map_err(|e| e.to_string())?;
    ThemePatch::new().validate().map_err(|e| e.to_string())
}

#[test]
fn rejects_a_number_that_is_not_finite() -> Result<(), String> {
    let mut patch = ThemePatch::new();
    patch.corner_scale = Some(f32::NAN);
    assert_rejected(&patch, "metrics.corner_scale", "not a finite number")?;
    assert_rejected(
        &ThemePatch::new().custom_metric("rail", f32::INFINITY),
        "custom_metrics.rail",
        "not a finite number",
    )?;
    let mut transform = Transform::IDENTITY;
    transform.rotation_deg = f32::NAN;
    assert_rejected(
        &body(PartPatch::new().transform(transform)),
        "widgets.button.rules[0].body.transform.rotation_deg",
        "not a finite number",
    )
}

#[test]
fn rejects_a_negative_size() -> Result<(), String> {
    let mut patch = ThemePatch::new();
    patch.spacing_scale = Some(-1.0);
    assert_rejected(&patch, "metrics.spacing_scale", "negative")?;
    assert_rejected(
        &body(PartPatch::new().stroke(Stroke::uniform(-1.0, RED))),
        "widgets.button.rules[0].body.stroke.width",
        "negative",
    )?;
    assert_rejected(
        &body(PartPatch::new().corners(Corners::uniform(-2.0))),
        "widgets.button.rules[0].body.corners.radius",
        "negative",
    )?;
    let blurred = ShadowLayer {
        blur: -4.0,
        ..ShadowLayer::default()
    };
    assert_rejected(
        &body(PartPatch::new().shadows(Some(Arc::from([blurred])))),
        "widgets.button.rules[0].body.shadows[0].blur",
        "negative",
    )?;
    assert_rejected(
        &body(PartPatch::new().padding(Edges::new(0.0, -1.0, 0.0, 0.0))),
        "widgets.button.rules[0].body.padding.right",
        "negative",
    )?;
    assert_rejected(
        &ThemePatch::new().button(ButtonRule::new().label(TextPatch::new().size_px(-13.0))),
        "widgets.button.rules[0].label.size_px",
        "negative",
    )?;
    assert_rejected(
        &ThemePatch::new()
            .widget_rules(WidgetPatchMap::new().button(ButtonPatch::new().icon_text_gap(-4.0))),
        "widgets.button.icon_text_gap",
        "negative",
    )?;
    assert_rejected(
        &ThemePatch::new()
            .widget_rules(WidgetPatchMap::new().spinner(SpinnerPatch::new().diameter(-1.0))),
        "widgets.spinner.diameter",
        "negative",
    )
}

#[test]
fn rejects_an_opacity_outside_0_to_1() -> Result<(), String> {
    assert_rejected(
        &body(PartPatch::new().opacity(1.5)),
        "widgets.button.rules[0].body.opacity",
        "outside 0.0..=1.0",
    )?;
    assert_rejected(
        &ThemePatch::new().menu(MenuRule::new().icon(IconPatch::new().opacity(-0.1))),
        "widgets.menu.rules[0].icon.opacity",
        "outside 0.0..=1.0",
    )?;
    let translucent = Color { a: 2.0, ..RED };
    assert_rejected(
        &ThemePatch::new().custom_color("gutter", translucent),
        "custom_colors.gutter.a",
        "outside 0.0..=1.0",
    )?;
    let mut colors = ColorPatch::new();
    colors.disabled_opacity = Some(1.5);
    assert_rejected(
        &ThemePatch::new().color_overrides(colors),
        "colors.disabled_opacity",
        "outside 0.0..=1.0",
    )?;
    assert_rejected(
        &ThemePatch::new()
            .widget_rules(WidgetPatchMap::new().switch(SwitchPatch::new().disabled_opacity(1.01))),
        "widgets.switch.disabled_opacity",
        "outside 0.0..=1.0",
    )?;
    assert_rejected(
        &ThemePatch::new().widget_rules(
            WidgetPatchMap::new()
                .tooltip(TooltipPatch::new().panel(PartPatch::new().opacity(-1.0))),
        ),
        "widgets.tooltip.panel.opacity",
        "outside 0.0..=1.0",
    )
}
