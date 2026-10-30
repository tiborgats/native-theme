#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

//! S11's reader: one test per row of the Spelling table, the unknown key's
//! TOML path, the shorthands' round trips, the `when` and `declare` readers
//! and a preset's whole blocks.

use super::layer::read_preset;
use crate::key;
use crate::next::blocks::BlockPatch;
use crate::next::blocks::focus::FocusVisibleWhen;
use crate::next::blocks::motion::Timing;
use crate::next::blocks::outline::{
    CornerCurve, Corners, CornersPatch, EdgePatch, PixelSnap, Radius, StrokePatch,
};
use crate::next::blocks::paint::{BlendMode, Paint, PaintKind, PaintLayer, PaintPatch};
use crate::next::blocks::part_style::PartStylePatch;
use crate::next::blocks::shadow::Shadow;
use crate::next::blocks::text::TextStylePatch;
use crate::next::color::Color;
use crate::next::declare::{DataName, DataOwner, ScalarUnit};
use crate::next::error::{LayerError, Problem, ProblemKind};
use crate::next::expr::{ElementDatum, Expr};
use crate::next::inputs::{Contrast, InputName};
use crate::next::layer::Layer;
use crate::next::roles::SurfaceSet;
use crate::next::selector::{
    Condition, ContextScope, ControlSize, Density, Direction, Idiom, InputKind, Parity, PartRef,
    RuleBody, Scheme, SharedFlag, SiblingScope, SizeClass, StateFlag, TiledEdges, ValuesPatch,
    VariantRef, WidgetOrPart, WidgetRef,
};
use crate::next::units::{FontSize, Fraction, Px, Scalar};
use crate::next::value::Value;
use crate::next::widgets::{
    ButtonPart, ButtonValuesPatch, IconPlacement, PartPatch, TypedPart, Variant, WidgetKind,
    WidgetValuesPatch,
};

/// The layer `text` states, or a failure naming its problems.
fn layer(text: &str) -> Layer {
    match Layer::from_toml(text) {
        Ok(layer) => layer,
        Err(error) => panic!("{error}"),
    }
}

/// The problems `text` has, as `path: kind` lines.
fn problems(text: &str) -> Vec<String> {
    match Layer::from_toml(text) {
        Ok(_) => Vec::new(),
        Err(LayerError::Invalid(problems)) => problems.iter().map(Problem::to_string).collect(),
    }
}

/// The patch of `button.body` the layer's bases state.
fn body(layer: &Layer) -> PartStylePatch {
    match layer
        .bases
        .widgets
        .button
        .as_ref()
        .and_then(|b| b.body.clone())
    {
        Some(PartPatch::Style(style)) => style,
        other => panic!("not a style: {other:?}"),
    }
}

/// `button.body`'s style read from its table's lines.
fn body_of(lines: &str) -> PartStylePatch {
    body(&layer(&format!("[widgets.button.body]\n{lines}")))
}

fn color(hex: &str) -> Color {
    Color::parse(hex).unwrap()
}

fn expr(text: &str) -> Expr {
    Expr::parse(text).unwrap()
}

fn color_paint(hex: &str) -> Paint {
    Paint {
        layers: vec![PaintLayer {
            kind: PaintKind::Color(Value::Stated(color(hex))),
            blend: BlendMode::Normal,
        }],
    }
}

// The Spelling table, row by row.

#[test]
fn spelling_stated_value() {
    let style = body_of("min_width_px = 4\nopacity = 0.5\nfill = \"#ff0000\"");
    assert_eq!(style.min_width, Some(Value::Stated(Px(4.0))));
    assert_eq!(style.opacity, Some(Value::Stated(Fraction(0.5))));
    assert_eq!(
        style.fill,
        Some(PaintPatch::Set(Value::Stated(color_paint("#ff0000"))))
    );
    let label = layer("[widgets.button.label]\ntext.case = \"upper\"\ntext.wrap = \"word\"");
    let Some(PartPatch::Style(PartStylePatch {
        text: Some(BlockPatch::Fields(text)),
        ..
    })) = label.bases.widgets.button.unwrap().label
    else {
        panic!("a table under an application layer's bases is a `Fields` patch");
    };
    assert!(text.case.is_some() && text.wrap.is_some() && text.font.is_none());
    let values = layer("[widgets.button.values]\nback_title_fallback = true");
    let values = values.bases.widgets.button.unwrap().values.unwrap();
    assert_eq!(values.back_title_fallback, Some(Value::Stated(true)));
}

#[test]
fn spelling_key_of_a_field_with_a_unit() {
    let style =
        body_of("min_width_px = \"= 2px\"\nmax_width_px = \"auto\"\ntouch_target_px = \"absent\"");
    assert_eq!(style.min_width, Some(Value::Expr(expr("2px"))));
    assert_eq!(style.max_width, Some(Value::Auto));
    assert_eq!(style.touch_target, Some(Value::Absent));
    assert_eq!(
        problems("[widgets.button.body]\nmin_width = 4"),
        ["widgets.button.body.min_width: unknown key"]
    );

    let text = |lines: &str| {
        let layer = layer(&format!("[widgets.button.label.text]\n{lines}"));
        let label = layer.bases.widgets.button.unwrap().label;
        match label {
            Some(PartPatch::Style(PartStylePatch {
                text: Some(BlockPatch::Fields(text)),
                ..
            })) => text,
            other => panic!("{other:?}"),
        }
    };
    let font = |text: TextStylePatch| text.font.unwrap_or_default();
    assert_eq!(
        font(text("font.size_pt = 10")).size,
        Some(Value::Stated(FontSize::Pt(10.0)))
    );
    assert_eq!(
        font(text("font.size_px = 13")).size,
        Some(Value::Stated(FontSize::Px(13.0)))
    );
    assert_eq!(
        text("line_height = \"= mul(font_size, 1.2)\"").line_height,
        Some(Value::Expr(expr("mul(font_size, 1.2)")))
    );
    assert_eq!(
        text("line_height = \"auto\"").line_height,
        Some(Value::Auto)
    );
    assert_eq!(
        problems("[widgets.button.label.text.font]\nsize = 10\nweight = 400"),
        [
            "widgets.button.label.text.font.size: expected an expression, \"auto\" or \"absent\"; \
             a number under the key's `_pt` or `_px` form"
        ]
    );
    assert_eq!(
        problems("[widgets.button.label.text.font]\nsize_pt = \"auto\""),
        ["widgets.button.label.text.font.size_pt: expected a number"]
    );
    assert_eq!(
        problems("[widgets.button.label.text.font]\nsize_pt = 10\nsize_px = 13"),
        ["widgets.button.label.text.font.size_px: states the field size_pt states too"]
    );

    let corners = |lines: &str| match body_of(lines).corners {
        Some(BlockPatch::Fields(corners)) => corners,
        other => panic!("{other:?}"),
    };
    assert_eq!(
        corners("corners.top_left_px = 4").top_left,
        Some(Radius::Px(Value::Stated(Px(4.0))))
    );
    assert_eq!(
        corners("corners.top_left_px = \"= 2px\"").top_left,
        Some(Radius::Px(Value::Expr(expr("2px"))))
    );
    assert_eq!(
        corners("corners.top_left = \"follow_part\"").top_left,
        Some(Radius::FollowPart)
    );
    assert_eq!(
        corners("corners.top_left = { full = { maximum_px = \"absent\" } }").top_left,
        Some(Radius::Full {
            maximum: Value::Absent
        })
    );
    assert_eq!(
        corners("corners.top_left = { concentric = { minimum_px = 2 } }").top_left,
        Some(Radius::Concentric {
            minimum: Value::Stated(Px(2.0))
        })
    );
    assert_eq!(
        problems("[widgets.button.body.corners]\ntop_left = { px = 4 }"),
        ["widgets.button.body.corners.top_left.px: unknown key"]
    );
}

#[test]
fn spelling_text_value() {
    let font = |lines: &str| {
        let layer = layer(&format!("[widgets.button.label.text.font]\n{lines}"));
        match layer.bases.widgets.button.unwrap().label {
            Some(PartPatch::Style(PartStylePatch {
                text:
                    Some(BlockPatch::Fields(TextStylePatch {
                        font: Some(font), ..
                    })),
                ..
            })) => font,
            other => panic!("{other:?}"),
        }
    };
    let family = |value: &str| font(&format!("family = {value:?}")).family;
    assert_eq!(
        family(".AppleSystemUIFont"),
        Some(Value::Stated(".AppleSystemUIFont".to_owned()))
    );
    assert_eq!(
        family("= token.text.keyed.body.font.family"),
        Some(Value::Expr(expr("token.text.keyed.body.font.family")))
    );
    // A text field's type allows `Absent`, not `Auto`: "auto" is text there.
    assert_eq!(family("absent"), Some(Value::Absent));
    assert_eq!(family("auto"), Some(Value::Stated("auto".to_owned())));
    // A plain `String` is always text.
    let layer = layer(
        "[widgets.button.icon.glyph]\nsource = { theme_icon = { name = \"absent\", fallbacks = \
         [\"none\"] } }",
    );
    let Some(PartPatch::Style(icon)) = layer.bases.widgets.button.unwrap().icon else {
        panic!("a style");
    };
    let Some(BlockPatch::Fields(glyph)) = icon.glyph else {
        panic!("fields");
    };
    assert_eq!(
        format!("{:?}", glyph.source),
        "Some(ThemeIcon { name: \"absent\", fallbacks: [\"none\"] })"
    );
}

#[test]
fn spelling_expression() {
    assert_eq!(
        body_of("opacity = \"= mix(0.2, 0.4, 0.5)\"").opacity,
        Some(Value::Expr(expr("mix(0.2, 0.4, 0.5)")))
    );
    let found = problems("[widgets.button.body]\nopacity = \"= mix(0.2,\"");
    assert_eq!(found.len(), 1);
    assert!(
        found[0].starts_with("widgets.button.body.opacity: expression: "),
        "{found:?}"
    );
}

#[test]
fn spelling_auto() {
    assert_eq!(
        body_of("max_width_px = \"auto\"").max_width,
        Some(Value::Auto)
    );
    // On an enum field "auto" is no `Auto`: the enum has no such variant.
    assert_eq!(
        problems("[widgets.button.body]\ncursor = \"auto\""),
        ["widgets.button.body.cursor: unknown name \"auto\""]
    );
}

#[test]
fn spelling_absent() {
    assert_eq!(body_of("shadow = \"absent\"").shadow, Some(Value::Absent));
    assert_eq!(
        body_of("fill = \"absent\"").fill,
        Some(PaintPatch::Set(Value::Absent))
    );
    let layer = layer("[widgets.button.subtitle]\nabsent = true");
    assert_eq!(
        layer.bases.widgets.button.unwrap().subtitle,
        Some(PartPatch::Absent)
    );
    assert_eq!(
        problems("[widgets.button.subtitle]\nabsent = true\ntext.case = \"upper\""),
        ["widgets.button.subtitle.text: unknown key"]
    );
    let set = layer_rule("target = \"button.icon\"\nset = { absent = true }");
    assert_eq!(
        set,
        RuleBody::Part(button(ButtonPart::Icon), PartPatch::Absent)
    );
}

#[test]
fn spelling_none() {
    let layer = layer("[tokens.caret]\nblink_timeout_ms = \"none\"");
    assert_eq!(layer.bases.tokens.caret.unwrap().blink_timeout, Some(None));
    let label = super::layer::read_layer("[widgets.button.label.text]\nmax_lines = \"none\"")
        .unwrap()
        .bases
        .widgets
        .button
        .unwrap()
        .label;
    assert!(matches!(
        label,
        Some(PartPatch::Style(PartStylePatch {
            text: Some(BlockPatch::Fields(TextStylePatch {
                max_lines: Some(None),
                ..
            })),
            ..
        }))
    ));
    // An `Option` inside an enum variant's payload left out is `None`.
    let values =
        Layer::from_toml("[widgets.button.values]\nfocus_visible_when = { keyboard = {} }")
            .unwrap();
    assert_eq!(
        values
            .bases
            .widgets
            .button
            .unwrap()
            .values
            .unwrap()
            .focus_visible_when,
        Some(Value::Stated(FocusVisibleWhen::Keyboard { timeout: None }))
    );
}

#[test]
fn spelling_enum_variant() {
    let style = body_of("corners.curve = \"continuous\"\nshadow = { elevation = \"2\" }");
    assert_eq!(
        style.corners,
        Some(BlockPatch::Fields(CornersPatch {
            curve: Some(CornerCurve::Continuous),
            ..CornersPatch::default()
        }))
    );
    assert_eq!(
        style.shadow,
        Some(Value::Stated(Shadow::Elevation(key!("2"))))
    );
    let layer = layer(
        "[tokens.motion.timings]\nstandard = { spring = { mass = 1, stiffness = 200, \
         damping_ratio = 1, initial_velocity = 0, rest_epsilon = 0.001, clamp = false, \
         time_scale = \"= token.motion.speed_factor\" } }",
    );
    let timings = layer.bases.tokens.motion.unwrap().timings.unwrap();
    assert!(matches!(
        timings.as_slice(),
        [(name, Timing::Spring { mass, clamp: false, .. })] if *name == key!("standard") && *mass == 1.0
    ));
    // A keyed variant is `@<key>`, as a keyed name is.
    let style = body_of("focus.kind = \"@reveal\"");
    let Some(BlockPatch::Fields(focus)) = style.focus else {
        panic!("fields");
    };
    assert_eq!(
        focus.kind,
        Some(crate::next::blocks::focus::FocusKind::Keyed(key!("reveal")))
    );
}

#[test]
fn spelling_list() {
    let style = body_of("bindings = []\ntransitions = []");
    assert_eq!(style.bindings, Some(Vec::new()));
    assert_eq!(style.transitions, Some(Vec::new()));
    let style = body_of("bindings = [{ surface = \"button\" }]");
    assert_eq!(style.bindings.map(|b| b.len()), Some(1));
    assert_eq!(
        problems("[widgets.button.body]\nbindings = \"absent\""),
        ["widgets.button.body.bindings: expected an array"]
    );
}

#[test]
fn spelling_tuple() {
    let style = body_of(
        "transform = { origin = [0.5, 0.25] }\ntransitions = [{ properties = [\"fill\"], kind = \
         \"interpolate\", duration_ms = 100, delay_ms = 0, timing = { cubic_bezier = [0.2, 0, 0, \
         1] }, suspended_by_theme_change = true }]",
    );
    let Some(BlockPatch::Fields(transform)) = style.transform else {
        panic!("fields");
    };
    assert_eq!(transform.origin, Some((0.5, 0.25)));
    let transitions = style.transitions.unwrap();
    assert_eq!(
        transitions[0].timing,
        Value::Stated(Timing::CubicBezier(0.2, 0.0, 0.0, 1.0))
    );
    assert_eq!(
        problems("[widgets.button.body.transform]\norigin = [0.5]"),
        ["widgets.button.body.transform.origin: expected an array of two numbers"]
    );
}

#[test]
fn spelling_kde_scheme_lists() {
    let layer = layer(
        "[inputs.kde_scheme.groups]\nlight = [\"header\", \"view\"]\ndark = []\n\
         [inputs.kde_scheme.inactive_groups]\nlight = [\"window\"]\n\
         [inputs.kde_scheme.unset_keys]\ndark = [\"kde_scheme.view.foreground_visited\"]",
    );
    let inputs = layer.bases.inputs;
    assert_eq!(
        inputs.kde_groups.light,
        Some(vec![SurfaceSet::Header, SurfaceSet::View])
    );
    assert_eq!(inputs.kde_groups.dark, Some(Vec::new()));
    assert_eq!(
        inputs.kde_inactive_groups.light,
        Some(vec![SurfaceSet::Window])
    );
    assert_eq!(
        inputs.kde_unset_keys.dark,
        Some(vec![
            InputName::new("kde_scheme.view.foreground_visited").unwrap()
        ])
    );
    assert_eq!(
        problems("[inputs.kde_scheme.groups]\nlight = [\"nowhere\"]"),
        ["inputs.kde_scheme.groups.light[0]: unknown name \"nowhere\""]
    );
}

#[test]
fn spelling_drawn_as() {
    let read = |table: &str| {
        let layer = layer(&format!(
            "[widgets.button.menu_indicator]\ndrawn_as = {table}"
        ));
        layer.bases.widgets.button.unwrap().menu_indicator.unwrap()
    };
    assert_eq!(
        read("{ widget = \"button\", part = \"icon\", variants = [\"flat\", \"@raised\"] }"),
        PartPatch::DrawnAs {
            target: WidgetOrPart::Part(button(ButtonPart::Icon)),
            variants: vec![
                VariantRef::Typed(Variant::Flat),
                VariantRef::Keyed(key!("raised"))
            ],
        }
    );
    assert_eq!(
        read("{ widget = \"button\", variants = [] }"),
        PartPatch::DrawnAs {
            target: WidgetOrPart::Widget(WidgetRef::Typed(WidgetKind::Button)),
            variants: Vec::new(),
        }
    );
    assert_eq!(
        problems("[widgets.button.icon]\ndrawn_as = { part = \"icon\", variants = [] }"),
        ["widgets.button.icon.drawn_as.part: states `part` without `widget`"]
    );
    assert_eq!(
        problems(
            "[widgets.button.icon]\ndrawn_as = { widget = \"button\", part = \"knob\", variants = [] }"
        ),
        ["widgets.button.icon.drawn_as.part: unknown name \"knob\""]
    );
}

#[test]
fn spelling_condition() {
    let rule = |conditions: &str| {
        let layer = layer(&format!(
            "[[rules]]\nwhen = {{ conditions = [{conditions}] }}\ntarget = \"tokens\"\nset = {{}}"
        ));
        layer.rules[0].selector.conditions.all_of.clone()
    };
    assert_eq!(
        rule(
            "\"window_inactive\", \"scheme:dark\", \"tiled:left,top\", \"input:touch\", \
             \"idiom:pad\", \"flavor:kirigami\", \"system_setting:graphic_effects_level\", \
             \"container_size:button:small\", \"posture:book\", \"device_model:iphone16\""
        ),
        [
            Condition::WindowInactive,
            Condition::Scheme(Scheme::Dark),
            Condition::Tiled(TiledEdges {
                top: true,
                left: true,
                ..TiledEdges::default()
            }),
            Condition::Input(InputKind::Touch),
            Condition::Idiom(Idiom::Pad),
            Condition::Flavor(key!("kirigami")),
            Condition::SystemSetting(key!("graphic_effects_level")),
            Condition::ContainerSize {
                container: WidgetRef::Typed(WidgetKind::Button),
                size: key!("small"),
            },
            Condition::Posture(crate::next::selector::Posture::Book),
            Condition::DeviceModel(key!("iphone16")),
        ]
    );
    assert_eq!(
        problems(
            "[[rules]]\nwhen = { conditions = [\"scheme:blue\", \"windowinactive\", \
             \"preset:adwaita\", \"scheme\", \"maximized:yes\"] }\ntarget = \"tokens\"\nset = {}"
        ),
        [
            "rules[0].when.conditions[0]: unknown name \"blue\"",
            "rules[0].when.conditions[1]: unknown name \"windowinactive\"",
            "rules[0].when.conditions[2]: unknown name \"adwaita\"",
            "rules[0].when.conditions[3]: expected a condition and its value, `<name>:<value>`",
            "rules[0].when.conditions[4]: expected a condition with no `:<value>`",
        ]
    );
}

/// A layer's one rule's body, from the rule's `target` and `set` lines.
fn layer_rule(lines: &str) -> RuleBody {
    layer(&format!("[[rules]]\n{lines}")).rules.remove(0).body
}

fn button(part: ButtonPart) -> PartRef {
    PartRef::Typed(TypedPart::Button(part))
}

#[test]
fn spelling_rule_target() {
    assert!(matches!(
        layer_rule("target = \"colors\"\nset = { global.accent = \"#3584e4\" }"),
        RuleBody::Colors(_)
    ));
    assert!(matches!(
        layer_rule("target = \"tokens\"\nset = { caret.width_px = 1 }"),
        RuleBody::Tokens(_)
    ));
    assert!(matches!(
        layer_rule("target = \"button.body\"\nset = { opacity = 0.5 }"),
        RuleBody::Part(part, PartPatch::Style(_)) if part == button(ButtonPart::Body)
    ));
    assert_eq!(
        layer_rule("target = \"button.values\"\nset = { icon_placement = \"top\" }"),
        RuleBody::Values(ValuesPatch::Typed(WidgetValuesPatch::Button(
            ButtonValuesPatch {
                icon_placement: Some(Value::Stated(IconPlacement::Top)),
                ..ButtonValuesPatch::default()
            }
        )))
    );
    assert_eq!(
        problems(
            "[[rules]]\ntarget = \"component.rule_editor.row\"\nset = {}\n[[rules]]\ntarget = \
             \"slider.track\"\nset = {}\n[[rules]]\ntarget = \"button.knob\"\nset = {}\n\
             [[rules]]\ntarget = \"button\"\nset = {}"
        ),
        [
            "rules[0].target: unknown name \"component.rule_editor.row\"",
            "rules[1].target: unknown name \"slider\"",
            "rules[2].target: unknown name \"knob\"",
            "rules[3].target: expected \"colors\", \"tokens\", \"<widget>.<part>\" or \
             \"<widget>.values\"",
        ]
    );
}

#[test]
fn spelling_rule_patch() {
    let layer = layer(
        "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\nset = { fill.append \
         = [\"= alpha(surface.foreground, 0.1)\"] }",
    );
    let rule = &layer.rules[0];
    assert_eq!(rule.selector.states.all_of, [StateFlag::Hover]);
    let RuleBody::Part(_, PartPatch::Style(style)) = &rule.body else {
        panic!("a part's style");
    };
    assert!(matches!(&style.fill, Some(PaintPatch::Append(layers)) if layers.len() == 1));
    assert_eq!(
        problems("[[rules]]\ntarget = \"button.body\"\nsets = {}\nwhen = {}"),
        ["rules[0].sets: unknown key", "rules[0].set: missing",]
    );
    // `append` in a layer's bases has nothing beneath it (S6.1).
    assert_eq!(
        problems("[widgets.button.body]\nfill.append = [\"#000000\"]"),
        [
            "widgets.button.body.fill.append: `append` in a layer's bases, where nothing lies beneath"
        ]
    );
}

#[test]
fn spelling_per_contrast_input_bases() {
    let layer = layer(
        "[inputs.accent]\nlight = \"#3584e4\"\ndark = \"#78aeed\"\n\
         [inputs.accent.contrast.high]\nlight = \"#1c71d8\"\ndark = \"#99c1f1\"\n\
         [inputs.accent_ramp.light1]\nlight = \"#ffffff\"",
    );
    let entries = layer.bases.inputs.entries;
    assert_eq!(entries.len(), 2);
    let (name, accent) = &entries[0];
    assert_eq!(name.to_string(), "accent");
    assert_eq!(
        accent.per_scheme.dark,
        Some(Value::Stated(color("#78aeed")))
    );
    assert_eq!(accent.per_contrast.len(), 1);
    assert_eq!(accent.per_contrast[0].0, Contrast::High);
    assert_eq!(entries[1].0.to_string(), "accent_ramp.light1");
    assert_eq!(
        problems("[inputs.accent.contrast.medium]\nlight = \"#000000\""),
        ["inputs.accent.contrast.medium: unknown name \"medium\""]
    );
}

#[test]
fn spelling_for_preset() {
    // No preset is bundled yet, so every `for_preset` id is an unknown key;
    // its bases are read all the same, their problems reported.
    assert_eq!(
        problems("[for_preset.adwaita.widgets.button.body]\nshine = 1"),
        [
            "for_preset.adwaita: unknown key",
            "for_preset.adwaita.widgets.button.body.shine: unknown key",
        ]
    );
}

#[test]
fn spelling_widget_values_and_component_parts() {
    let layer = layer("[widgets.button.values]\nicon_text_gap_px = 4\nhit_extension.top_px = 2");
    let values = layer.bases.widgets.button.unwrap().values.unwrap();
    assert_eq!(values.icon_text_gap, Some(Value::Stated(Px(4.0))));
    assert_eq!(
        values.hit_extension.map(|sides| sides.top),
        Some(Some(Value::Stated(Px(2.0))))
    );
    // No unit enables a component yet (S5.1).
    assert_eq!(
        problems("[components.rule_editor.row]\nopacity = 1"),
        ["components: unknown key"]
    );
}

#[test]
fn unknown_key_reports_toml_path() {
    assert_eq!(
        problems(
            "[widgets.button.body]\nshine = 1\n[widgets.button.body.stroke.top]\nglow = 2\n\
             [tokens.caret]\ncolour = \"#000000\"\n[colors.light.surfaces.window]\nbackgroud = \
             \"#ffffff\"\n[[rules]]\ntarget = \"button.label\"\nset = { text = { font = { \
             wieght = 400 } } }\nwhen = { states = [\"hover\"], colour = 1 }\n[widgets.button]\n\
             knob = {}\n[nonsense]"
        ),
        [
            "widgets.button.body.shine: unknown key",
            "widgets.button.body.stroke.top.glow: unknown key",
            "widgets.button.knob: unknown key",
            "tokens.caret.colour: unknown key",
            "colors.light.surfaces.window.backgroud: unknown key",
            "rules[0].when.colour: unknown key",
            "rules[0].set.text.font.wieght: unknown key",
            "nonsense: unknown key",
        ]
    );
}

#[test]
fn a_part_takes_only_its_catalogue_fields() {
    assert_eq!(
        problems("[widgets.button.label]\nfill = \"#000000\"\npadding.top_px = 2"),
        ["widgets.button.label.fill: a field `button.label` does not have"]
    );
}

#[test]
fn every_problem_is_collected() {
    let found = problems(
        "[widgets.button.body]\nopacity = \"high\"\nmin_width_px = true\ncursor = \"blink\"\n\
         [widgets.button.values]\nicon_placement = \"left\"\nrepeat_delay_ms = nan",
    );
    assert_eq!(
        found,
        [
            "widgets.button.body.opacity: expected a number",
            "widgets.button.body.min_width_px: expected a number",
            "widgets.button.body.cursor: unknown name \"blink\"",
            "widgets.button.values.icon_placement: unknown name \"left\"",
            "widgets.button.values.repeat_delay_ms: not a finite number",
        ]
    );
    assert_eq!(
        problems("[widgets.button.body\nopacity = 1").len(),
        1,
        "TOML that does not parse is one problem"
    );
}

// The shorthands' round trips: each states what its long form states.

#[test]
fn corners_px_round_trips() {
    let short = body_of("corners_px = 6\ncorners.curve = \"circular\"").corners;
    let long = body_of(
        "corners.top_left_px = 6\ncorners.top_right_px = 6\ncorners.bottom_right_px = 6\n\
         corners.bottom_left_px = 6\ncorners.curve = \"circular\"",
    )
    .corners;
    assert_eq!(short, long);
    // On a bare `Corners` (a focus indicator's) too.
    let focus = |lines: &str| match body_of(lines).focus {
        Some(BlockPatch::Fields(focus)) => focus.corners,
        other => panic!("{other:?}"),
    };
    assert_eq!(
        focus("focus.corners_px = \"= 2px\""),
        focus(
            "focus.corners.top_left_px = \"= 2px\"\nfocus.corners.top_right_px = \"= 2px\"\n\
             focus.corners.bottom_right_px = \"= 2px\"\nfocus.corners.bottom_left_px = \"= 2px\""
        )
    );
    // A corner both ways is a duplicate key.
    assert_eq!(
        problems("[widgets.button.body]\ncorners_px = 6\ncorners.top_left_px = 4"),
        ["widgets.button.body.corners: states the field corners_px states too"]
    );
}

#[test]
fn stroke_shorthand_round_trips() {
    let stroke = |lines: &str| match body_of(lines).stroke {
        Some(BlockPatch::Fields(stroke)) => stroke,
        other => panic!("{other:?}"),
    };
    let short = stroke(
        "stroke = { width_px = 1, paint = \"#000000\", dash = \"solid\", snap = \
         \"to_device_pixel\", position = \"inside\", outset_px = \"absent\", fill_clip = \"box\" }",
    );
    let edge =
        "{ width_px = 1, paint = \"#000000\", dash = \"solid\", snap = \"to_device_pixel\" }";
    let long = stroke(&format!(
        "stroke = {{ top = {edge}, right = {edge}, bottom = {edge}, left = {edge}, position = \
         \"inside\", outset_px = \"absent\", fill_clip = \"box\" }}"
    ));
    assert_eq!(short, long);
    let dotted = stroke("stroke.width_px = 2\nstroke.snap = \"none\"");
    let expected = EdgePatch {
        width: Some(Value::Stated(Px(2.0))),
        snap: Some(PixelSnap::None),
        ..EdgePatch::default()
    };
    assert_eq!(
        dotted,
        StrokePatch {
            top: Some(expected.clone()),
            right: Some(expected.clone()),
            bottom: Some(expected.clone()),
            left: Some(expected),
            ..StrokePatch::default()
        }
    );
    assert_eq!(
        problems("[widgets.button.body.stroke]\nwidth_px = 1\ntop.width_px = 2"),
        ["widgets.button.body.stroke.top.width_px: states the field width_px states too"]
    );
}

#[test]
fn a_whole_block_replaces_and_reports_its_missing_fields() {
    let style = body_of(
        "corners = { replace = { top_left_px = 1, top_right_px = 1, bottom_right_px = 1, \
         bottom_left_px = 1, curve = \"circular\" } }",
    );
    let radius = Radius::Px(Value::Stated(Px(1.0)));
    assert_eq!(
        style.corners,
        Some(BlockPatch::Set(Value::Stated(Corners {
            top_left: radius.clone(),
            top_right: radius.clone(),
            bottom_right: radius.clone(),
            bottom_left: radius,
            curve: CornerCurve::Circular,
        })))
    );
    assert_eq!(
        problems("[widgets.button.body]\ncorners = { replace = { top_left_px = 1 } }"),
        [
            "widgets.button.body.corners.replace.top_right: missing",
            "widgets.button.body.corners.replace.bottom_right: missing",
            "widgets.button.body.corners.replace.bottom_left: missing",
            "widgets.button.body.corners.replace.curve: missing",
        ]
    );
}

// `when` (S7).

#[test]
fn when_reads_every_selector_part() {
    let layer = layer(
        "[[rules]]\ntarget = \"button.body\"\nset = {}\n[rules.when]\n\
         conditions = [\"scheme:dark\"]\nnot_conditions = [\"contrast_high\"]\n\
         context = [{ widget = \"button\", part = \"body\", variants = [\"tool\"], states = \
         [\"hover\"], sizes = [\"small\"], surface = \"header\" }, { scope = { nearest = \
         [\"button\"] }, widget = \"button\" }]\n\
         not_context = [{ widget = \"button\" }]\n\
         position = { among = \"any\", first = true, parity = \"odd\", previous = { widget = \
         \"button\", not_states = [\"checked\"] } }\n\
         variants = [\"flat\", \"@pill_keyed\"]\nnot_variants = [\"primary\"]\n\
         sizes = [\"regular\"]\nnot_sizes = [\"mini\"]\ndensities = [\"compact\", \"@dense\"]\n\
         not_densities = [\"default\"]\nsize_class = { horizontal = [\"compact\"], not_vertical = \
         [\"regular\"] }\ndirection = \"rtl\"\nstates = [\"focus_visible\", \"empty\", \
         \"@keyboard_focus_change\"]\nnot_states = [\"disabled\"]",
    );
    let selector = &layer.rules[0].selector;
    assert_eq!(selector.conditions.none_of, [Condition::ContrastHigh]);
    let context = &selector.context;
    assert_eq!(context.len(), 2);
    assert_eq!(context[0].scope, ContextScope::AnyAncestor);
    assert_eq!(
        context[0].target,
        Some(WidgetOrPart::Part(button(ButtonPart::Body)))
    );
    assert_eq!(context[0].sizes.all_of, [ControlSize::Small]);
    assert_eq!(context[0].surface, Some(SurfaceSet::Header));
    assert_eq!(
        context[1].scope,
        ContextScope::NearestAmong(vec![WidgetRef::Typed(WidgetKind::Button)])
    );
    assert_eq!(selector.not_context.len(), 1);
    let position = selector.position.as_ref().unwrap();
    assert_eq!(position.among, SiblingScope::Any);
    assert_eq!(
        (position.first, position.parity),
        (Some(true), Some(Parity::Odd))
    );
    assert_eq!(
        position.previous.as_ref().unwrap().states.none_of,
        [StateFlag::Checked]
    );
    assert_eq!(
        selector.variants.all_of,
        [
            VariantRef::Typed(Variant::Flat),
            VariantRef::Keyed(key!("pill_keyed"))
        ]
    );
    assert_eq!(selector.size.sizes.none_of, [ControlSize::Mini]);
    assert_eq!(
        selector.size.densities.all_of,
        [Density::Compact, Density::Keyed(key!("dense"))]
    );
    assert_eq!(
        selector.size.size_classes.horizontal.all_of,
        [SizeClass::Compact]
    );
    assert_eq!(
        selector.size.size_classes.vertical.none_of,
        [SizeClass::Regular]
    );
    assert_eq!(selector.direction, Some(Direction::Rtl));
    assert_eq!(
        selector.states.all_of,
        [
            StateFlag::FocusVisible,
            StateFlag::Shared(SharedFlag::Empty),
            StateFlag::Widget(key!("keyboard_focus_change")),
        ]
    );
    assert_eq!(
        problems(
            "[[rules]]\ntarget = \"tokens\"\nset = {}\nwhen = { states = [\"hovered\", \
             \"@Bad\"], context = [{ part = \"body\" }], position = { among = \"most\" } }"
        ),
        [
            "rules[0].when.states[0]: unknown name \"hovered\"",
            "rules[0].when.states[1]: \"Bad\" is not a key: 'B' is outside `[a-z0-9][a-z0-9_]*`",
            "rules[0].when.context[0].part: states `part` without `widget`",
            "rules[0].when.position.among: expected \"same\", \"any\" or an array of widgets",
        ]
    );
}

// A preset's file: its header, its declarations, its whole blocks.

const PRESET_HEAD: &str = "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n";

#[test]
fn a_presets_tables_are_whole_blocks() {
    let preset = read_preset(&format!(
        "{PRESET_HEAD}[widgets.button.body]\ncorners_px = 4\ncorners.curve = \"circular\""
    ))
    .unwrap();
    assert_eq!(preset.id, "probe");
    assert_eq!(preset.generation, "probe-1");
    let radius = Radius::Px(Value::Stated(Px(4.0)));
    assert_eq!(
        body(&preset.layer).corners,
        Some(BlockPatch::Set(Value::Stated(Corners {
            top_left: radius.clone(),
            top_right: radius.clone(),
            bottom_right: radius.clone(),
            bottom_left: radius,
            curve: CornerCurve::Circular,
        })))
    );
    let error = read_preset(&format!(
        "{PRESET_HEAD}[widgets.button.label.text]\ncase = \"upper\""
    ))
    .unwrap_err();
    let LayerError::Invalid(found) = error;
    let found: Vec<String> = found.iter().map(Problem::to_string).collect();
    assert!(
        found.contains(&"widgets.button.label.text.font.size_pt: missing".to_owned()),
        "{found:?}"
    );
    assert!(
        found.contains(&"widgets.button.label.text.max_lines: missing".to_owned()),
        "an `Option` in a whole block is stated like any other field: {found:?}"
    );
    assert_eq!(
        read_preset("[widgets.button.body]\nopacity = 1").map(|_| ()),
        Err(LayerError::Invalid(vec![Problem::new(
            "preset",
            ProblemKind::Missing
        )]))
    );
}

#[test]
fn declarations_read_s10s_keys() {
    let preset = read_preset(&format!(
        "{PRESET_HEAD}[declare]\nflavors = [\"widgets\", \"kirigami\"]\nmaterials = [\"mica\"]\n\
         elevation = [\"1\", \"2\"]\nsurface_sets = [\"window\", \"view\"]\nconditions = \
         [\"@graphic_effects_level\", {{ key = \"@show_sounds\", for_applications = true }}]\n\
         params = {{ layout = {{ unit = \"px\", keys = [\"grid_unit\"] }} }}\ninputs = \
         [\"accent\", \"accent_ramp.light1\"]\naccent_inputs = [\"accent\"]\ntransition_kinds = \
         {{ bounce = {{ height = \"px\" }} }}\ncontainer_sizes = {{ button = [\"small\", \
         \"large\"] }}\nmaterial_params = {{ shimmer = \"fraction\" }}\nschemes = [\"light\", \
         \"dark\"]\n[declare.widget.button]\nvariants = [\"primary\", \"flat\", \"@pill\"]\n\
         sizes = [\"small\", \"regular\"]\ndensities = [\"compact\"]\nflags = \
         [\"@keyboard_focus_change\"]\nexclusive = [[\"disabled\", \"pressed\", \"hover\"]]\n\
         exclusive_variants = [[\"primary\", \"flat\"]]\nsizes_by_variant = [{{ variant = \
         \"@pill\", sizes = [\"small\"] }}]\ndefault_size = \"regular\"\n\
         default_size_by_variant = [{{ variant = \"@pill\", size = \"small\" }}]\navailable = \
         true\nrequires_variants = []\ndata = {{ body = [\"color\", \"width\"], icon = \
         [\"body_font_height\", \"widget.label.char_width\"] }}\nbreakpoints = [{{ measured = {{ \
         part = \"body\" }}, width = {{ at_most = 400 }}, variant = \"@narrow\" }}]\n\
         [declare.flavor.kirigami.widget.button]\nsizes = [\"regular\"]"
    ))
    .unwrap();
    let declared = preset.declarations;
    assert_eq!(declared.flavors, [key!("widgets"), key!("kirigami")]);
    assert_eq!(declared.elevation, [key!("1"), key!("2")]);
    assert_eq!(declared.conditions.len(), 2);
    assert!(declared.conditions[1].for_applications);
    assert_eq!(declared.params[0].1.unit, ScalarUnit::Px);
    assert_eq!(declared.accent_inputs.len(), 1);
    assert_eq!(
        declared.transition_kinds[0].1,
        [(key!("height"), ScalarUnit::Px)]
    );
    assert_eq!(
        declared.container_sizes[0].1,
        [key!("small"), key!("large")]
    );
    assert_eq!(
        declared.material_params,
        [(key!("shimmer"), ScalarUnit::Fraction)]
    );
    assert_eq!(declared.schemes, [Scheme::Light, Scheme::Dark]);
    let (widget, decl) = &declared.widgets[0];
    assert_eq!(*widget, WidgetKind::Button);
    assert_eq!(
        decl.variants.as_deref(),
        Some(
            &[
                VariantRef::Typed(Variant::Primary),
                VariantRef::Typed(Variant::Flat),
                VariantRef::Keyed(key!("pill")),
            ][..]
        )
    );
    assert_eq!(
        decl.exclusive.as_ref().map(|groups| groups[0].len()),
        Some(3)
    );
    assert_eq!(decl.default_size, Some(ControlSize::Regular));
    assert_eq!(decl.available, Some(true));
    assert_eq!(decl.requires_variants, Some(Vec::new()));
    let data = decl.data.as_ref().unwrap();
    assert_eq!(
        data[0],
        (
            DataOwner::Part(TypedPart::Button(ButtonPart::Body)),
            vec![
                DataName::Element(ElementDatum::Color),
                DataName::Element(ElementDatum::Width)
            ]
        )
    );
    assert_eq!(data[1].1.len(), 2);
    let breakpoint = &decl.breakpoints.as_ref().unwrap()[0];
    assert_eq!(
        breakpoint.measured,
        crate::next::selector::Measured::Part(button(ButtonPart::Body))
    );
    assert_eq!(
        breakpoint.width,
        Some((
            crate::next::selector::Comparison::AtMost,
            Value::Stated(Px(400.0))
        ))
    );
    assert_eq!(breakpoint.combine, crate::next::selector::Combine::All);
    let flavor = &declared.flavor[0];
    assert_eq!(flavor.0, key!("kirigami"));
    assert_eq!(
        flavor.1.widgets[0].1.sizes.as_deref(),
        Some(&[ControlSize::Regular][..])
    );
    assert_eq!(flavor.1.widgets[0].1.variants, None);
}

#[test]
fn declarations_refuse_what_s10_does_not_spell() {
    let error = read_preset(&format!(
        "{PRESET_HEAD}[declare]\nflavours = []\nconditions = [\"graphic_effects_level\"]\n\
         params = {{ layout = {{ unit = \"em\", keys = [] }} }}\n[declare.widget.button]\n\
         sizes = [\"huge\"]\ndata = {{ knob = [\"width\"], body = [\"heat\"] }}\n\
         breakpoints = [{{ measured = \"window\", width = {{ at_most = 1 }}, height = {{ \
         at_most = 2 }}, variant = \"@x\" }}]\n[declare.widget.slider]\nsizes = []\n\
         [declare.component.rule_editor]\navailable = true"
    ))
    .unwrap_err();
    let LayerError::Invalid(found) = error;
    let found: Vec<String> = found.iter().map(Problem::to_string).collect();
    assert_eq!(
        found,
        [
            "declare.flavours: unknown key",
            "declare.conditions[0]: expected a keyed name, `@<key>`",
            "declare.params.layout.unit: unknown name \"em\"",
            "declare.widget.button.sizes[0]: unknown name \"huge\"",
            "declare.widget.button.data.knob: unknown name \"knob\"",
            "declare.widget.button.data.body[0]: unknown name \"heat\"",
            "declare.widget.button.breakpoints[0].combine: missing",
            "declare.widget.slider: unknown key",
            "declare.component: unknown key",
        ]
    );
}

/// A map entry whose value has a unit takes the unit's suffix on its key
/// for every value, as a field does; a keyed scalar takes none, its number
/// in the unit its declaration names (S11, ADD-A123).
#[test]
fn map_entry_keys_take_their_units_suffix_and_scalars_none() {
    let read = layer(
        "[tokens.params.layout]\ngrid_unit = 18\n[tokens.params.opacity]\ndisabled = 0.5\n\
         [tokens.motion.durations]\nbutton_ms = 150\nlong_ms = \"= token.motion.durations.button\"",
    );
    let tokens = read.bases.tokens;
    assert_eq!(
        tokens.params,
        Some(vec![
            (
                key!("layout"),
                vec![(key!("grid_unit"), Value::Stated(Scalar::Number(18.0)))]
            ),
            (
                key!("opacity"),
                vec![(key!("disabled"), Value::Stated(Scalar::Number(0.5)))]
            ),
        ])
    );
    let durations = tokens.motion.unwrap().durations.unwrap();
    assert_eq!(
        durations.first(),
        Some(&(key!("button"), Value::Stated(crate::next::units::Ms(150.0))))
    );
    assert_eq!(durations.len(), 2);
    // Every value of a duration takes the suffixed key: a number, an
    // expression, `"absent"`.
    for value in ["150", "\"= token.motion.durations.long\"", "\"absent\""] {
        for key in ["button", "button_px"] {
            assert_eq!(
                problems(&format!("[tokens.motion.durations]\n{key} = {value}")),
                [format!("tokens.motion.durations.{key}: unknown key")],
                "{key} = {value}"
            );
        }
    }
    // A keyed scalar's suffixed key is another key, named as written.
    let suffixed = layer("[tokens.params.layout]\ngap_px = 4");
    assert_eq!(
        suffixed.bases.tokens.params,
        Some(vec![(
            key!("layout"),
            vec![(key!("gap_px"), Value::Stated(Scalar::Number(4.0)))]
        )])
    );
}

/// `"auto"`, `"none"` and `"absent"` are exact: `"absent"` is `Absent`,
/// `"none"` is `None` only in an `Option` field, `"auto"` is `Auto` only in a
/// length or a font size, and elsewhere each is what the field's type spells
/// by it (S11, ADD-A123).
#[test]
fn auto_none_and_absent_are_exact() {
    let theme = |text: &str| {
        let layer = layer(&format!("[tokens.icons]\ntheme = {text}"));
        layer.bases.tokens.icons.and_then(|icons| icons.theme)
    };
    assert_eq!(theme("\"absent\""), Some(Value::Absent));
    assert_eq!(theme("\"auto\""), Some(Value::Stated("auto".to_owned())));
    assert_eq!(theme("\"none\""), Some(Value::Stated("none".to_owned())));
    assert_eq!(
        theme("\"Absent\""),
        Some(Value::Stated("Absent".to_owned()))
    );
    assert_eq!(
        body_of("max_width_px = \"auto\"").max_width,
        Some(Value::Auto)
    );
    assert_eq!(
        problems("[widgets.button.body]\nmax_width_px = \"Auto\""),
        ["widgets.button.body.max_width_px: expected a number"]
    );
    assert_eq!(
        problems("[tokens.caret]\nblink_on_ms = \"auto\""),
        ["tokens.caret.blink_on_ms: expected a number"]
    );
    let timeout = |text: &str| {
        let layer = layer(&format!("[tokens.caret]\nblink_timeout_ms = {text}"));
        layer
            .bases
            .tokens
            .caret
            .and_then(|caret| caret.blink_timeout)
    };
    assert_eq!(timeout("\"none\""), Some(None));
    assert_eq!(
        problems("[tokens.caret]\nblink_timeout_ms = \"None\""),
        ["tokens.caret.blink_timeout_ms: expected a number"]
    );
}

#[test]
fn try_from_names_every_missing_leaf() {
    let error = Corners::try_from(CornersPatch {
        top_left: Some(Radius::FollowPart),
        ..CornersPatch::default()
    })
    .unwrap_err();
    assert_eq!(
        error,
        LayerError::Invalid(
            ["top_right", "bottom_right", "bottom_left", "curve"]
                .map(|path| Problem::new(path, ProblemKind::Missing))
                .to_vec()
        )
    );
}

#[test]
fn a_missing_field_is_reported_at_its_key() {
    let error = crate::next::FocusIndicator::try_from(crate::next::FocusIndicatorPatch {
        kind: Some(crate::next::FocusKind::Ring),
        ..crate::next::FocusIndicatorPatch::default()
    })
    .unwrap_err();
    let LayerError::Invalid(found) = error;
    let found: Vec<String> = found.iter().map(Problem::to_string).collect();
    for key in [
        "width_px",
        "offset.top_px",
        "offset.left_px",
        "corners.top_left",
    ] {
        assert!(
            found.contains(&format!("{key}: missing")),
            "{key}: {found:?}"
        );
    }
}

#[test]
fn a_breakpoints_part_is_the_widgets_own() {
    let declare = |measured: &str| {
        read_preset(&format!(
            "{PRESET_HEAD}[declare.widget.button]\nbreakpoints = [{{ measured = {measured}, \
             width = {{ at_most = 1 }}, variant = \"@x\" }}]"
        ))
        .map(|preset| preset.declarations.widgets.len())
    };
    assert_eq!(declare("{ part = \"label\" }"), Ok(1));
    let LayerError::Invalid(found) = declare("{ part = \"button.label\" }").unwrap_err();
    assert_eq!(
        found,
        [Problem::new(
            "declare.widget.button.breakpoints[0].measured.part",
            ProblemKind::UnknownName {
                name: "button.label".to_owned()
            }
        )]
    );
}

/// A map keyed by text is a TOML table with the text as its keys, quoted
/// where TOML does not allow it bare; two keys are one entry exactly where
/// they are equal as TOML keys, case-sensitively (S4.4, S11, ADD-A123).
#[test]
fn a_map_keyed_by_text_is_a_table_of_its_texts() {
    let read = layer(
        "[tokens.text.fallbacks.per_family]\n\"Segoe Fluent Icons\" = [\"Segoe MDL2 Assets\"]\n\
         \"segoe fluent icons\" = []",
    );
    let fallbacks = read
        .bases
        .tokens
        .text
        .and_then(|text| text.fallbacks)
        .and_then(|fallbacks| fallbacks.per_family);
    assert_eq!(
        fallbacks,
        Some(vec![
            (
                "Segoe Fluent Icons".to_owned(),
                vec!["Segoe MDL2 Assets".to_owned()]
            ),
            ("segoe fluent icons".to_owned(), Vec::new()),
        ])
    );
    // A problem names the key as written, quoted.
    assert_eq!(
        problems("[tokens.text.fallbacks.per_family]\n\"Segoe Fluent Icons\" = 4"),
        ["tokens.text.fallbacks.per_family.\"Segoe Fluent Icons\": expected an array"]
    );
    // Equal as TOML keys: one key, which TOML itself refuses twice.
    assert_eq!(
        problems("[tokens.text.fallbacks.per_family]\n\"Segoe UI\" = []\n\"Segoe UI\" = []").len(),
        1
    );
}

/// A `PartRef` stated as a value is a bare name of the parts of the widget
/// whose table it is in, or `<widget>.<part>`; a bare name where no
/// widget's table is read is mistyped (S11, ADD-A123).
#[test]
fn a_part_ref_value_names_its_widget_outside_a_widgets_table() {
    let keyframe = |part: &str| {
        format!(
            "[tokens.icons.sizes.toolbar]\nsize_px = 16\nanimation = {{ period_ms = 100, timing = \
             \"linear\", keyframes = [{{ at = 0, part = \"{part}\", timing = \"absent\", patch \
             = {{}} }}] }}"
        )
    };
    assert_eq!(
        problems(&keyframe("icon")),
        [
            "tokens.icons.sizes.toolbar.animation.keyframes[0].part: expected `<widget>.<part>`: \
             no widget's table is read here"
        ]
    );
    let named = layer(&keyframe("button.icon"));
    assert!(named.bases.tokens.icons.is_some());
}
