use super::*;

use serde_json::json;

use crate::next::geometry::{Cursor, ShadowLayer, Transform};
use crate::next::patch::{ButtonRule, IconPatch, PartPatch, TextPatch, ThemePatch};
use crate::next::preset::Theme;
use crate::next::query::{ButtonVariant, Orientation};
use crate::next::rules::PartProp;
use crate::next::style::FocusIndicator;
use crate::next::style::FocusKind;
use crate::next::widgets::ResolvedTheme;
use crate::next::window::EdgeFlags;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);

/// A test input group, not a theme's.
const HOVER_GROUP: &[&[WidgetState]] = &[&[WidgetState::DISABLED, WidgetState::HOVER]];

fn breeze() -> ResolvedTheme {
    Theme::preset(Preset::KdeBreeze).resolve_current()
}

/// A rule set is written as its exclusive groups, its base's stated
/// properties and its rules, each its selector's fields then the part's
/// properties; a padding of which one side is stated is that side alone.
#[test]
fn rule_set_snapshot_form() -> TestResult {
    let set: RuleSet<PartStyle, PartProp, ButtonMatcher> = RuleSet::new(
        PartStyle::default(),
        HOVER_GROUP,
        vec![
            (
                RuleSelector::on(WidgetState::NONE),
                vec![PartProp::Fill(Paint::Solid(RED))],
            ),
            (
                RuleSelector::on(WidgetState::HOVER)
                    .except(WidgetState::DISABLED)
                    .orientation(Orientation::Vertical)
                    .matcher(ButtonMatcher {
                        variant: Some(ButtonVariant::Primary),
                        shape: None,
                    }),
                vec![
                    PartProp::Fill(Paint::Solid(GREEN)),
                    PartProp::PaddingSides(Edges::new(4.0, 0.0, 0.0, 0.0), EdgeFlags::TOP),
                    PartProp::MinSize([None, Some(24.0)]),
                ],
            ),
        ],
    );
    let value = serde_json::to_value(RulesView::new("body", &set))?;
    assert_eq!(
        value,
        json!({
            "exclusive": [["disabled", "hover"]],
            "base": {},
            "rules": [
                { "body": { "fill": "#ff0000" } },
                {
                    "states": ["hover"],
                    "not_states": ["disabled"],
                    "variant": "primary",
                    "orientation": "vertical",
                    "body": {
                        "fill": "#00ff00",
                        "padding": { "top": 4.0 },
                        "min_size": ["none", 24.0],
                    },
                },
            ],
        })
    );
    Ok(())
}

/// A resolved theme serializes to JSON and to TOML, every global and widget
/// a key, and a ruled widget's part as its rule set.
#[test]
fn resolved_theme_serializes() -> TestResult {
    let theme = breeze();
    let value = serde_json::to_value(&theme)?;
    let keys: Vec<&str> = value
        .as_object()
        .ok_or("not a map")?
        .keys()
        .map(String::as_str)
        .collect();
    let mut expected = vec![
        "name",
        "preset",
        "context",
        "colors",
        "typography",
        "icons",
        "layout",
        "timings",
        "motion_tokens",
        "focus_ring",
        "caret",
        "elevations",
        "window",
        "button",
        "input",
        "text_area",
        "checkbox",
        "radio",
        "switch",
        "slider",
        "progress_bar",
        "scrollbar",
        "tab",
        "menu",
        "menu_bar",
        "list",
        "sidebar",
        "toolbar",
        "status_bar",
        "combo_box",
        "segmented_control",
        "expander",
        "card",
        "popover",
        "dialog",
        "tooltip",
        "spinner",
        "splitter",
        "separator",
        "link",
        "custom_colors",
        "custom_metrics",
    ];
    let mut keys = keys;
    keys.sort_unstable();
    expected.sort_unstable();
    assert_eq!(keys, expected);
    assert_eq!(value["preset"], json!("kde-breeze"));
    assert_eq!(
        value["button"]["body"]["exclusive"],
        serde_json::to_value(Exclusive(theme.button.body.exclusive()))?
    );
    let rules = value["button"]["body"]["rules"]
        .as_array()
        .ok_or("the button's body has no rules")?;
    assert!(!rules.is_empty());
    let text = toml::to_string(&theme)?;
    assert!(text.contains("[button.body]"), "{text}");
    Ok(())
}

/// Each rule of the snapshot reads back as a patch's rule, which states
/// the same properties: the snapshot spells a rule as a theme file does.
#[test]
fn snapshot_rules_read_as_patch_rules() -> TestResult {
    let value = serde_json::to_value(breeze().button)?;
    for part in ["body", "label", "icon"] {
        let rules = value[part]["rules"].as_array().cloned().unwrap_or_default();
        for rule in rules {
            let read: ButtonRule = serde_json::from_value(rule.clone())?;
            assert_eq!(serde_json::to_value(&read)?, rule, "{part}");
        }
    }
    Ok(())
}

/// A patch that uses every serialized form round-trips through JSON.
#[test]
fn patch_json_round_trip() -> TestResult {
    let gradient = Paint::Complex(Arc::new(ComplexPaint {
        layers: Arc::from([
            PaintLayer::LinearGradient {
                angle_deg: 135.0,
                stops: Arc::from([
                    GradientStop {
                        offset: 0.0,
                        color: RED,
                    },
                    GradientStop {
                        offset: 1.0,
                        color: GREEN,
                    },
                ]),
                blend: BlendMode::Normal,
            },
            PaintLayer::Solid {
                color: GREEN,
                blend: BlendMode::Screen,
            },
        ]),
    }));
    let sides = Stroke::Complex(Arc::new(ComplexStroke {
        top: StrokeSide {
            width: 1.0,
            paint: Paint::Solid(RED),
        },
        right: StrokeSide {
            width: 0.0,
            paint: Paint::None,
        },
        bottom: StrokeSide {
            width: 2.0,
            paint: Paint::Solid(GREEN),
        },
        left: StrokeSide {
            width: 0.0,
            paint: Paint::None,
        },
        align: StrokeAlign::Center,
    }));
    let corners = Corners {
        radius: CornerRadius::PerCorner {
            top_left: 1.0,
            top_right: 2.0,
            bottom_right: 3.0,
            bottom_left: 4.0,
        },
        curve: CornerCurve::Continuous,
    };
    let focus = FocusIndicator::new(
        FocusKind::Ring,
        Stroke::Uniform {
            width: 2.0,
            color: RED,
            align: StrokeAlign::Outside,
        },
        Stroke::None,
        1.0,
        Some(circular(CornerRadius::Pill)),
        None,
    );
    let patch = ThemePatch::new()
        .corner_scale(1.25)
        .font_family("Inter")
        .custom_color("gutter", GREEN)
        .custom_metric("rail", 3.0)
        .button(
            ButtonRule::new()
                .variant(ButtonVariant::Primary)
                .states(WidgetState::HOVER | WidgetState::FOCUSED)
                .body(
                    PartPatch::new()
                        .fill(gradient)
                        .stroke(sides)
                        .corners(corners)
                        .shadows(None)
                        .transform(Transform::default())
                        .min_size([Some(24.0), None])
                        .focus(Some(Arc::new(focus)))
                        .cursor(Cursor::Pointer),
                )
                .label(TextPatch::new().optical_size(None).width(FontWidth::new(7)))
                .icon(IconPatch::new().stroke_width(Some(1.5))),
        )
        .button(
            ButtonRule::new().body(PartPatch::new().shadows(Some(Arc::from([ShadowLayer {
                offset_y: 4.0,
                blur: 18.0,
                color: RED,
                ..ShadowLayer::default()
            }])))),
        );
    let text = serde_json::to_string(&patch)?;
    let read: ThemePatch = serde_json::from_str(&text)?;
    assert_eq!(read, patch, "{text}");
    Ok(())
}

#[test]
fn states_are_names() -> TestResult {
    let state = WidgetState::HOVER | WidgetState::READ_ONLY;
    assert_eq!(serde_json::to_value(state)?, json!(["hover", "read_only"]));
    assert_eq!(
        serde_json::from_value::<WidgetState>(json!(["read_only", "hover"]))?,
        state
    );
    assert!(serde_json::from_value::<WidgetState>(json!(["hovered"])).is_err());
    Ok(())
}

#[test]
fn font_width_outside_its_classes_is_an_error() {
    assert!(serde_json::from_value::<FontWidth>(json!(5)).is_ok());
    assert!(serde_json::from_value::<FontWidth>(json!(0)).is_err());
    assert!(serde_json::from_value::<FontWidth>(json!(10)).is_err());
}

#[test]
fn styles_write_only_what_they_state() -> TestResult {
    let mut style = PartStyle::default();
    style.apply(&PartProp::Opacity(0.5));
    assert_eq!(serde_json::to_value(&style)?, json!({ "opacity": 0.5 }));
    assert_eq!(serde_json::to_value(TextStyle::default())?, json!({}));
    assert_eq!(serde_json::to_value(IconStyle::default())?, json!({}));
    Ok(())
}
