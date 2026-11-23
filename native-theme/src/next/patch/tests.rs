use super::*;

use crate::next::geometry::StrokeAlign;
use crate::next::preset::{Preset, Theme};
use crate::next::query::{ButtonQuery, ListQuery};
use crate::next::style::PartStated;
use crate::next::window::WindowState;

const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);

/// The factor the scale tests patch with: a test input, not a theme value.
const SCALE: f32 = 2.0;

fn breeze() -> ResolvedTheme {
    Theme::preset(Preset::KdeBreeze).resolve_current()
}

fn patched(patches: impl IntoIterator<Item = ThemePatch>) -> ResolvedTheme {
    patches
        .into_iter()
        .fold(Theme::preset(Preset::KdeBreeze), Theme::with_patch)
        .resolve_current()
}

/// `corners` with a uniform radius scaled by `k`; corners without a radius
/// as they are.
fn scaled(corners: Corners, k: f32) -> Result<Corners, String> {
    match corners.radius {
        CornerRadius::Uniform(r) => Ok(Corners {
            radius: CornerRadius::Uniform(r * k),
            ..corners
        }),
        CornerRadius::None => Ok(corners),
        other => Err(format!("a radius the test does not scale: {other:?}")),
    }
}

fn radius(corners: &Corners) -> Result<f32, String> {
    match corners.radius {
        CornerRadius::Uniform(r) => Ok(r),
        other => Err(format!("not a uniform radius: {other:?}")),
    }
}

/// `edges` with every side scaled by [`SCALE`].
fn scaled_edges(edges: Edges) -> Edges {
    Edges::new(
        edges.top * SCALE,
        edges.right * SCALE,
        edges.bottom * SCALE,
        edges.left * SCALE,
    )
}

fn hover_rule(part: PartPatch) -> ThemePatch {
    ThemePatch::new().button(ButtonRule::new().states(WidgetState::HOVER).body(part))
}

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

// ---------------------------------------------------------------------------
// The application, on kde-breeze
// ---------------------------------------------------------------------------

#[test]
fn color_overrides_change_only_the_roles() {
    let base = breeze();
    assert_ne!(base.colors.window.background, Some(RED));
    let mut colors = ColorPatch::new();
    colors.window.background = Some(RED);
    colors.error.fill = Some(RED);
    let theme = patched([ThemePatch::new().color_overrides(colors)]);
    assert_eq!(theme.colors.window.background, Some(RED));
    assert_eq!(
        theme.colors.window.foreground,
        base.colors.window.foreground
    );
    assert_eq!(theme.colors.error.fill, RED);
    assert_eq!(theme.colors.error.foreground, base.colors.error.foreground);
    assert_eq!(theme.colors.error.origin, RoleOrigin::Custom);
    assert_eq!(theme.colors.warning, base.colors.warning);
    assert_eq!(theme.colors.view, base.colors.view);
    // The parts derived from the window's background keep their colours.
    let rest = WindowState::default();
    assert_eq!(theme.window.frame(rest), base.window.frame(rest));
    assert_eq!(theme.tooltip, base.tooltip);
    assert_eq!(
        theme.button.body(ButtonQuery::default()),
        base.button.body(ButtonQuery::default())
    );
}

#[test]
fn font_family_replaces_the_ui_family_wherever_it_stands() -> Result<(), String> {
    let base = breeze();
    let ui = base.typography.body.family.clone();
    assert!(!ui.is_empty());
    assert_ne!(base.typography.monospace.family, ui);
    // A rule of an earlier patch that names the UI family is rewritten too.
    let earlier = ThemePatch::new().button(
        ButtonRule::new()
            .states(WidgetState::HOVER)
            .label(TextPatch::new().family(ui.clone())),
    );
    let theme = patched([earlier, ThemePatch::new().font_family("Patch Sans")]);
    assert_eq!(&*theme.typography.body.family, "Patch Sans");
    let label = theme.button.label(ButtonQuery::default());
    assert_eq!(base.button.label(ButtonQuery::default()).family, ui);
    assert_eq!(&*label.family, "Patch Sans");
    let hovered = theme.button.label(ButtonQuery::new(WidgetState::HOVER));
    assert_eq!(&*hovered.family, "Patch Sans");
    assert_eq!(theme.typography.monospace, base.typography.monospace);
    // Everything but the family stays.
    let mut expected = base.button.label(ButtonQuery::default());
    expected.family = Arc::from("Patch Sans");
    assert_eq!(label, expected);
    Ok(())
}

#[test]
fn monospace_family_replaces_only_the_monospace_family() {
    let base = breeze();
    let theme = patched([ThemePatch::new().monospace_family("Patch Mono")]);
    assert_eq!(&*theme.typography.monospace.family, "Patch Mono");
    assert!(
        theme
            .typography
            .monospace
            .stated
            .contains(TextStated::FAMILY)
    );
    assert_eq!(theme.typography.body, base.typography.body);
    assert_eq!(
        theme.button.label(ButtonQuery::default()),
        base.button.label(ButtonQuery::default())
    );
}

#[test]
fn corner_scale_scales_the_radii_and_nothing_else() -> Result<(), String> {
    let base = breeze();
    let theme = patched([ThemePatch::new().corner_scale(SCALE)]);
    let rest = ButtonQuery::default();
    let (before, after) = (base.button.body(rest), theme.button.body(rest));
    assert!(radius(&before.corners)? > 0.0);
    assert_eq!(after.corners, scaled(before.corners, SCALE)?);
    assert_eq!(after.padding, before.padding);
    assert_eq!(after.stroke, before.stroke);
    // A radius a rule states.
    let hover = ListQuery::new(WidgetState::HOVER);
    let (before, after) = (base.list.highlight(hover), theme.list.highlight(hover));
    assert!(radius(&before.corners)? > 0.0);
    assert_eq!(after.corners, scaled(before.corners, SCALE)?);
    assert_eq!(after.margin, before.margin);
    // The window chrome and the layout's radii.
    let window = WindowState::default();
    assert_eq!(
        theme.window.frame(window).corners,
        scaled(base.window.frame(window).corners, SCALE)?
    );
    assert_eq!(
        theme.layout.corner_radius,
        base.layout.corner_radius * SCALE
    );
    assert_eq!(
        theme.layout.corner_radius_lg,
        base.layout.corner_radius_lg * SCALE
    );
    assert_eq!(theme.layout.widget_gap, base.layout.widget_gap);
    Ok(())
}

#[test]
fn spacing_scale_scales_padding_margins_and_gaps() -> Result<(), String> {
    let base = breeze();
    let theme = patched([ThemePatch::new().spacing_scale(SCALE)]);
    let rest = ButtonQuery::default();
    let (before, after) = (base.button.body(rest), theme.button.body(rest));
    assert_ne!(before.padding, Edges::ZERO);
    assert_eq!(after.padding, scaled_edges(before.padding));
    assert_eq!(after.corners, before.corners);
    let hover = ListQuery::new(WidgetState::HOVER);
    let (before, after) = (base.list.highlight(hover), theme.list.highlight(hover));
    assert_ne!(before.margin, Edges::ZERO);
    assert_eq!(after.margin, scaled_edges(before.margin));
    assert_eq!(
        theme.button.icon_text_gap,
        base.button.icon_text_gap * SCALE
    );
    assert_eq!(theme.checkbox.label_gap, base.checkbox.label_gap * SCALE);
    assert_eq!(theme.dialog.button_gap, base.dialog.button_gap * SCALE);
    let gap = base
        .layout
        .widget_gap
        .ok_or("kde-breeze states no widget_gap")?;
    assert_eq!(theme.layout.widget_gap, Some(gap * SCALE));
    assert_eq!(theme.layout.corner_radius, base.layout.corner_radius);
    // Not a gap: the scroll bar's least thumb length.
    assert_eq!(
        theme.scrollbar.min_thumb_length,
        base.scrollbar.min_thumb_length
    );
    Ok(())
}

#[test]
fn scales_leave_widths_and_min_size_alone() {
    let base = breeze();
    let theme = patched([ThemePatch::new().corner_scale(SCALE).spacing_scale(SCALE)]);
    assert_eq!(theme.layout.border_width, base.layout.border_width);
    assert_eq!(theme.layout.border_opacity, base.layout.border_opacity);
    for state in [WidgetState::NONE, WidgetState::HOVER, WidgetState::PRESSED] {
        let query = ButtonQuery::new(state);
        let (before, after) = (base.button.body(query), theme.button.body(query));
        assert_eq!(after.stroke, before.stroke, "{state:?}");
        assert_eq!(after.min_size, before.min_size, "{state:?}");
        assert_eq!(after.max_size, before.max_size, "{state:?}");
    }
    assert!(
        base.button.body(ButtonQuery::default()).min_size[0].is_some(),
        "kde-breeze states the button's least width"
    );
    for state in [WidgetState::NONE, WidgetState::HOVER] {
        let (before, after) = (base.scrollbar.thumb(state), theme.scrollbar.thumb(state));
        assert_eq!(after.min_size, before.min_size, "{state:?}");
        assert_eq!(after.max_size, before.max_size, "{state:?}");
        assert_eq!(after.stroke, before.stroke, "{state:?}");
    }
    assert_eq!(theme.separator.thickness, base.separator.thickness);
    assert_eq!(theme.spinner, base.spinner);
}

#[test]
fn widget_rules_append_rules_patch_static_parts_and_set_metrics() {
    let base = breeze();
    let gap = base.button.icon_text_gap + 1.0;
    let theme = patched([ThemePatch::new()
        .widget_rules(
            WidgetPatchMap::new()
                .button(ButtonPatch::new().icon_text_gap(gap))
                .tooltip(TooltipPatch::new().panel(PartPatch::new().fill(Paint::Solid(RED))))
                .spinner(SpinnerPatch::new().period_ms(500)),
        )
        .button(
            ButtonRule::new()
                .states(WidgetState::HOVER)
                .body(PartPatch::new().fill(Paint::Solid(RED))),
        )]);
    let hover = ButtonQuery::new(WidgetState::HOVER);
    assert_ne!(base.button.body(hover).fill, Paint::Solid(RED));
    assert_eq!(theme.button.body(hover).fill, Paint::Solid(RED));
    assert_eq!(
        theme.button.body(hover).stroke,
        base.button.body(hover).stroke
    );
    // A rule that states nothing of a part appends nothing to it.
    assert_eq!(theme.button.label(hover), base.button.label(hover));
    // The preset's exclusive group still decides: pressed wins over hover.
    let pressed = ButtonQuery::new(WidgetState::PRESSED | WidgetState::HOVER);
    assert_eq!(theme.button.body(pressed), base.button.body(pressed));
    let rest = ButtonQuery::default();
    assert_eq!(theme.button.body(rest), base.button.body(rest));
    assert_eq!(theme.button.icon_text_gap, gap);
    assert_eq!(theme.tooltip.panel.fill, Paint::Solid(RED));
    assert!(theme.tooltip.panel.stated.contains(PartStated::FILL));
    assert_eq!(theme.tooltip.text, base.tooltip.text);
    assert_eq!(theme.spinner.period_ms, Some(500));
    assert_eq!(theme.spinner.diameter, base.spinner.diameter);
    assert_eq!(theme.checkbox, base.checkbox);
    assert!(theme.dead_rules().iter().all(|(_, dead)| dead.is_empty()));
}

#[test]
fn custom_tokens_are_merged() {
    let theme = patched([
        ThemePatch::new()
            .custom_color("gutter", RED)
            .custom_metric("rail", 3.0),
        ThemePatch::new()
            .custom_color("gutter", GREEN)
            .custom_color("ruler", BLUE),
    ]);
    assert_eq!(theme.custom_colors.get("gutter"), Some(&GREEN));
    assert_eq!(theme.custom_colors.get("ruler"), Some(&BLUE));
    assert_eq!(theme.custom_metrics.get("rail"), Some(&3.0));
    assert_eq!(theme.custom_colors.len(), 2);
}

#[test]
fn patches_apply_in_the_order_given() -> Result<(), String> {
    let red = {
        let mut colors = ColorPatch::new();
        colors.window.background = Some(RED);
        ThemePatch::new().color_overrides(colors)
    };
    let green = {
        let mut colors = ColorPatch::new();
        colors.window.background = Some(GREEN);
        ThemePatch::new().color_overrides(colors)
    };
    assert_eq!(
        patched([red.clone(), green.clone()])
            .colors
            .window
            .background,
        Some(GREEN)
    );
    assert_eq!(patched([green, red]).colors.window.background, Some(RED));
    // A patch's scales leave the rules it appends as written; a later
    // patch's scales reach them.
    let hover = ButtonQuery::new(WidgetState::HOVER);
    let corners = Corners::uniform(3.0);
    let rule = hover_rule(PartPatch::new().corners(corners));
    let same = patched([rule.clone().corner_scale(SCALE)]);
    assert_eq!(same.button.body(hover).corners, corners);
    let later = patched([rule, ThemePatch::new().corner_scale(SCALE)]);
    assert_eq!(later.button.body(hover).corners, scaled(corners, SCALE)?);
    Ok(())
}

#[test]
fn apply_patch_in_a_transform_is_with_patch() {
    let patch = ThemePatch::new()
        .corner_scale(SCALE)
        .font_family("Patch Sans")
        .custom_color("gutter", RED);
    let with_patch = patched([patch.clone()]);
    let in_transform = Theme::preset(Preset::KdeBreeze)
        .with_transform(move |theme| theme.apply_patch(&patch))
        .resolve_current();
    assert_eq!(with_patch, in_transform);
    // The patches come before the transforms.
    let mut colors = ColorPatch::new();
    colors.window.background = Some(RED);
    let theme = Theme::preset(Preset::KdeBreeze)
        .with_transform(|theme| theme.colors.window.background = Some(BLUE))
        .with_patch(ThemePatch::new().color_overrides(colors));
    assert_eq!(theme.resolve_current().colors.window.background, Some(BLUE));
    assert!(format!("{theme:?}").contains("patches: ["));
}
