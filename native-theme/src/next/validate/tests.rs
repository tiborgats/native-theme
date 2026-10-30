//! S12's structural checks, one `refusal_<item>` test per item: a layer
//! that triggers exactly that problem, and a near miss that does not. Every
//! number and name here is synthetic.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::{Context, LayerKind, check, check_preset};
use crate::key;
use crate::next::blocks::BlockPatch;
use crate::next::blocks::focus::FocusIndicatorPatch;
use crate::next::blocks::icon::IconStylePatch;
use crate::next::blocks::motion::{
    Keyframe, Loop, PropertyGroup, StepJump, Timing, Transition, TransitionKind,
};
use crate::next::blocks::outline::{CornersPatch, Radius};
use crate::next::blocks::paint::{BlendMode, Paint, PaintKind, PaintLayer, PaintPatch, Stop};
use crate::next::blocks::part_style::PartStylePatch;
use crate::next::blocks::shadow::Shadow;
use crate::next::blocks::text::{AxisTag, AxisValue, FontPatch, TextStylePatch};
use crate::next::color::Color;
use crate::next::declare::Declarations;
use crate::next::error::{LayerError, Problem};
use crate::next::expr::{Expr, Func, MAX_DEPTH, Ref};
use crate::next::layer::Layer;
use crate::next::roles::{ColorRolesPatch, GlobalRolesPatch};
use crate::next::tokens::{MotionTokensPatch, ThemeChange, TokensPatch};
use crate::next::toml::layer::read_preset;
use crate::next::units::{Fraction, Ms, Px};
use crate::next::value::Value;
use crate::next::widgets::{ButtonPatch, PartPatch};

const PRESET_HEAD: &str = "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n";

/// The declarations a `[declare]` table's body states.
fn declare(body: &str) -> Declarations {
    read_preset(&format!("{PRESET_HEAD}[declare]\n{body}"))
        .unwrap()
        .declarations
}

fn strings(result: Result<(), LayerError>) -> Vec<String> {
    match result {
        Ok(()) => Vec::new(),
        Err(LayerError::Invalid(problems)) => problems.iter().map(Problem::to_string).collect(),
    }
}

/// The problems `layer` has as a layer of `kind` over `decl`.
fn found_as(layer: &Layer, decl: &Declarations, kind: LayerKind) -> Vec<String> {
    let cx = Context {
        declarations: decl,
        preset: None,
        kind,
    };
    strings(check(layer, &cx))
}

/// The problems of an application layer over `decl`.
fn found(layer: &Layer, decl: &Declarations) -> Vec<String> {
    found_as(layer, decl, LayerKind::Application)
}

/// The problems of an application layer over no declarations.
fn bare(layer: &Layer) -> Vec<String> {
    found(layer, &Declarations::default())
}

fn toml(text: &str) -> Layer {
    Layer::from_toml(text).unwrap()
}

/// A layer whose bases style `button.body` by `style`.
fn body(style: PartStylePatch) -> Layer {
    let mut layer = Layer::default();
    layer.bases.widgets.button = Some(ButtonPatch {
        body: Some(PartPatch::Style(style)),
        ..ButtonPatch::default()
    });
    layer
}

/// A layer whose bases state `tokens`.
fn tokens(tokens: TokensPatch) -> Layer {
    let mut layer = Layer::default();
    layer.bases.tokens = tokens;
    layer
}

fn color() -> Color {
    Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    }
}

fn layer_of(blend: BlendMode) -> PaintLayer {
    PaintLayer {
        kind: PaintKind::Color(Value::Stated(color())),
        blend,
    }
}

fn fill(layers: Vec<PaintLayer>) -> PartStylePatch {
    PartStylePatch {
        fill: Some(PaintPatch::Set(Value::Stated(Paint { layers }))),
        ..PartStylePatch::default()
    }
}

fn transition(properties: Vec<PropertyGroup>, timing: Timing) -> Transition {
    Transition {
        properties,
        kind: TransitionKind::Interpolate,
        duration: Value::Stated(Ms(100.0)),
        delay: Value::Stated(Ms(0.0)),
        timing: Value::Stated(timing),
        suspended_by_theme_change: false,
    }
}

fn transitions(items: Vec<Transition>) -> Layer {
    body(PartStylePatch {
        transitions: Some(items),
        ..PartStylePatch::default()
    })
}

const NONE: [&str; 0] = [];

#[test]
fn refusal_absent_not_allowed() {
    assert_eq!(
        bare(&toml("[widgets.button.body]\nopacity = \"absent\"")),
        ["widgets.button.body.opacity: may not be absent"]
    );
    // The core marks `shadow`, the catalogue `icon` as a whole.
    assert_eq!(
        bare(&toml("[widgets.button.body]\nshadow = \"absent\"")),
        NONE
    );
    assert_eq!(bare(&toml("[widgets.button.icon]\nabsent = true")), NONE);
    // A widget's root part is never absent as a whole.
    assert_eq!(
        bare(&toml("[widgets.button.body]\nabsent = true")),
        ["widgets.button.body: may not be absent"]
    );
}

#[test]
fn refusal_auto_not_allowed() {
    assert_eq!(
        bare(&toml("[widgets.button.body]\npadding.top_px = \"auto\"")),
        ["widgets.button.body.padding.top_px: may not be auto"]
    );
    assert_eq!(
        bare(&toml("[widgets.button.body]\nmin_width_px = \"auto\"")),
        NONE
    );
}

#[test]
fn refusal_call_not_allowed() {
    let layer = |text: &str| toml(&format!("[widgets.button.values]\nshows_icon = \"{text}\""));
    assert_eq!(
        bare(&layer("= max(1, 2)")),
        ["widgets.button.values.shows_icon: takes a literal or a reference, not a call"]
    );
    assert_eq!(bare(&layer("= values.back_title_fallback")), NONE);
}

#[test]
fn refusal_out_of_range() {
    assert_eq!(
        bare(&toml(
            "[widgets.button.body]\nopacity = 1.5\nmin_width_px = -1"
        )),
        [
            "widgets.button.body.opacity: out of range: 0..=1",
            "widgets.button.body.min_width_px: out of range: at least zero",
        ]
    );
    // A margin may be negative (S1); an opacity of one is in range.
    assert_eq!(
        bare(&toml(
            "[widgets.button.body]\nopacity = 1\nmargin.top_px = -2"
        )),
        NONE
    );
    let spring = |mass: f32| Timing::Spring {
        mass,
        stiffness: 100.0,
        damping_ratio: 1.0,
        initial_velocity: 0.0,
        rest_epsilon: 0.001,
        clamp: false,
        time_scale: Value::Stated(1.0),
    };
    assert_eq!(
        bare(&transitions(vec![
            transition(vec![PropertyGroup::Fill], spring(0.0)),
            transition(
                vec![PropertyGroup::Opacity],
                Timing::CubicBezier(1.5, 0.0, 0.5, 1.0)
            ),
            transition(
                vec![PropertyGroup::Stroke],
                Timing::Steps {
                    count: 1,
                    jump: StepJump::JumpNone
                }
            ),
        ])),
        [
            "widgets.button.body.transitions[0].timing.spring.mass: out of range: above zero",
            "widgets.button.body.transitions[1].timing.cubic_bezier[0]: out of range: 0..=1",
            "widgets.button.body.transitions[2].timing.steps.count: out of range: at least two \
             steps under `jump-none`",
        ]
    );
    assert_eq!(
        bare(&transitions(vec![
            transition(vec![PropertyGroup::Fill], spring(1.0)),
            transition(
                vec![PropertyGroup::Stroke],
                Timing::Steps {
                    count: 1,
                    jump: StepJump::JumpEnd
                }
            ),
        ])),
        NONE
    );
}

#[test]
fn refusal_not_finite() {
    let opacity = |n: f32| {
        body(PartStylePatch {
            opacity: Some(Value::Stated(Fraction(n))),
            ..PartStylePatch::default()
        })
    };
    assert_eq!(
        bare(&opacity(f32::NAN)),
        ["widgets.button.body.opacity: not a finite number"]
    );
    assert_eq!(bare(&opacity(0.5)), NONE);
}

#[test]
fn refusal_no_layer() {
    assert_eq!(
        bare(&body(fill(Vec::new()))),
        ["widgets.button.body.fill: has no layer"]
    );
    assert_eq!(bare(&body(fill(vec![layer_of(BlendMode::Normal)]))), NONE);
    let shadow = |layers| {
        body(PartStylePatch {
            shadow: Some(Value::Stated(Shadow::Layers(layers))),
            ..PartStylePatch::default()
        })
    };
    assert_eq!(
        bare(&shadow(Vec::new())),
        ["widgets.button.body.shadow: has no layer"]
    );
}

#[test]
fn refusal_mask_at_bottom() {
    assert_eq!(
        bare(&body(fill(vec![
            layer_of(BlendMode::Mask),
            layer_of(BlendMode::Normal)
        ]))),
        ["widgets.button.body.fill.layers[0].blend: its bottom layer is a mask"]
    );
    assert_eq!(
        bare(&body(fill(vec![
            layer_of(BlendMode::Normal),
            layer_of(BlendMode::Mask)
        ]))),
        NONE
    );
}

#[test]
fn refusal_too_few_stops() {
    let gradient = |count: usize| {
        let stops = (0..count)
            .map(|_| Stop {
                position: 0.0,
                color: Value::Stated(color()),
            })
            .collect();
        let kind = PaintKind::Linear {
            angle: crate::next::units::Deg(0.0),
            stops,
            repeat: false,
            extent: None,
            offset: None,
        };
        body(fill(vec![PaintLayer {
            kind,
            blend: BlendMode::Normal,
        }]))
    };
    assert_eq!(
        bare(&gradient(1)),
        ["widgets.button.body.fill.layers[0].kind: a gradient has at least two stops"]
    );
    assert_eq!(bare(&gradient(2)), NONE);
}

#[test]
fn refusal_no_properties() {
    assert_eq!(
        bare(&transitions(vec![transition(Vec::new(), Timing::Linear)])),
        ["widgets.button.body.transitions[0].properties: a transition of no property group"]
    );
    assert_eq!(
        bare(&transitions(vec![transition(
            vec![PropertyGroup::Fill],
            Timing::Linear
        )])),
        NONE
    );
}

#[test]
fn refusal_zero_duration() {
    let change = |ms: f32| {
        tokens(TokensPatch {
            theme_change: Some(ThemeChange::Suspend {
                duration: Value::Stated(Ms(ms)),
            }),
            ..TokensPatch::default()
        })
    };
    assert_eq!(
        bare(&change(0.0)),
        ["tokens.theme_change.suspend.duration: a zero duration is `instant`"]
    );
    assert_eq!(bare(&change(200.0)), NONE);
}

#[test]
fn refusal_keyframes_not_increasing() {
    let animated = |ats: [f32; 2]| {
        let keyframes = ats
            .into_iter()
            .map(|at| Keyframe {
                at: Fraction(at),
                part: None,
                timing: Value::Absent,
                patch: PartStylePatch::default(),
            })
            .collect();
        let animation = Loop {
            period: Value::Stated(Ms(1000.0)),
            timing: Value::Stated(Timing::Linear),
            keyframes,
        };
        let mut layer = Layer::default();
        layer.bases.widgets.button = Some(ButtonPatch {
            icon: Some(PartPatch::Style(PartStylePatch {
                icon: Some(BlockPatch::Fields(IconStylePatch {
                    animation: Some(BlockPatch::Set(Value::Stated(animation))),
                    ..IconStylePatch::default()
                })),
                ..PartStylePatch::default()
            })),
            ..ButtonPatch::default()
        });
        layer
    };
    assert_eq!(
        bare(&animated([0.5, 0.5])),
        ["widgets.button.icon.icon.animation.keyframes: the keyframes' `at` do not increase"]
    );
    assert_eq!(bare(&animated([0.0, 0.5])), NONE);
}

#[test]
fn refusal_follow_part_outside_focus() {
    let corner = |focus: bool| {
        let corners = CornersPatch {
            top_left: Some(Radius::FollowPart),
            ..CornersPatch::default()
        };
        let style = if focus {
            PartStylePatch {
                focus: Some(BlockPatch::Fields(FocusIndicatorPatch {
                    corners: Some(corners),
                    ..FocusIndicatorPatch::default()
                })),
                ..PartStylePatch::default()
            }
        } else {
            PartStylePatch {
                corners: Some(BlockPatch::Fields(corners)),
                ..PartStylePatch::default()
            }
        };
        body(style)
    };
    assert_eq!(
        bare(&corner(false)),
        ["widgets.button.body.corners.top_left: `follow_part` outside a focus indicator's corners"]
    );
    assert_eq!(bare(&corner(true)), NONE);
}

#[test]
fn refusal_route_duplicated() {
    let axis = |tag: &str| {
        let mut layer = Layer::default();
        layer.bases.widgets.button = Some(ButtonPatch {
            label: Some(PartPatch::Style(PartStylePatch {
                text: Some(BlockPatch::Fields(TextStylePatch {
                    font: Some(FontPatch {
                        axes: Some(vec![(AxisTag::new(tag).unwrap(), AxisValue::Fixed(1.0))]),
                        ..FontPatch::default()
                    }),
                    ..TextStylePatch::default()
                })),
                ..PartStylePatch::default()
            })),
            ..ButtonPatch::default()
        });
        layer
    };
    assert_eq!(
        bare(&axis("wght")),
        ["widgets.button.label.text.font.axes: `wght` duplicates a field's route"]
    );
    assert_eq!(bare(&axis("opsz")), NONE);
}

/// A layer whose light bases state `global.accent` as `text`.
fn accent(text: &str) -> Layer {
    toml(&format!("[colors.light.global]\naccent = \"{text}\""))
}

#[test]
fn refusal_kde_accent_argument() {
    assert_eq!(
        bare(&accent("= kde_accent(osd, background_normal, disabled)")),
        [
            "colors.light.global.accent: `kde_accent` does not take `osd`",
            "colors.light.global.accent: `kde_accent` does not take `disabled`",
        ]
    );
    assert_eq!(
        bare(&accent("= kde_accent(window, background_normal, inactive)")),
        NONE
    );
}

#[test]
fn refusal_pick_without_colors() {
    assert_eq!(
        bare(&accent("= pick(1)")),
        ["colors.light.global.accent: `pick` with no colours"]
    );
    assert_eq!(bare(&accent("= pick(1, #ff0000)")), NONE);
}

#[test]
fn refusal_kirigami_lerp_balance() {
    assert_eq!(
        bare(&accent("= kirigami_lerp(#000000, #ffffff, 1.5)")),
        ["colors.light.global.accent: out of range: 0..=1"]
    );
    assert_eq!(
        bare(&accent("= kirigami_lerp(#000000, #ffffff, 0.15)")),
        NONE
    );
}

#[test]
fn refusal_too_deep() {
    let nested = |calls: usize| {
        let inner = Expr::Lit(crate::next::expr::Literal::Number(1.0));
        let expr = (0..calls).fold(inner, |expr, _| Expr::Call(Func::Floor, vec![expr]));
        body(PartStylePatch {
            opacity: Some(Value::Expr(expr)),
            ..PartStylePatch::default()
        })
    };
    assert_eq!(
        bare(&nested(MAX_DEPTH + 1)),
        ["widgets.button.body.opacity: an expression nested deeper than 32"]
    );
    assert_eq!(bare(&nested(MAX_DEPTH)), NONE);
}

#[test]
fn refusal_base_in_bases() {
    assert_eq!(
        bare(&accent("= base")),
        ["colors.light.global.accent: `base` in a layer's bases, where nothing lies beneath"]
    );
    let rule = toml("[[rules]]\ntarget = \"colors\"\nset = { global.accent = \"= base\" }");
    assert_eq!(bare(&rule), NONE);
}

#[test]
fn refusal_append_in_bases() {
    let append = PartStylePatch {
        fill: Some(PaintPatch::Append(vec![layer_of(BlendMode::Normal)])),
        ..PartStylePatch::default()
    };
    assert_eq!(
        bare(&body(append.clone())),
        [
            "widgets.button.body.fill.append: `append` in a layer's bases, where nothing lies beneath"
        ]
    );
    let mut rule = Layer::default();
    rule.rules.push(crate::next::selector::Rule::new(
        crate::next::selector::Selector::default(),
        crate::next::selector::RuleBody::Part(
            crate::next::selector::PartRef::Typed(crate::next::widgets::TypedPart::Button(
                crate::next::widgets::ButtonPart::Body,
            )),
            PartPatch::Style(append),
        ),
    ));
    assert_eq!(bare(&rule), NONE);
}

#[test]
fn refusal_element_operand() {
    assert_eq!(
        bare(&accent("= mix(#000000, #ffffff, element.progress)")),
        ["colors.light.global.accent: `element.progress` is an element's, not a token's or role's"]
    );
    // A text style token's expression may read its own font size.
    let line_height = toml("[tokens.text.roles.body]\nline_height = \"= mul(font_size, 1.2)\"");
    assert_eq!(bare(&line_height), NONE);
}

#[test]
fn refusal_surface_in_roles() {
    assert_eq!(
        bare(&accent("= surface.foreground")),
        ["colors.light.global.accent: a colour role names `surface.`, not `role.<set>.`"]
    );
    let decl = declare("surface_sets = [\"window\"]");
    assert_eq!(found(&accent("= role.window.foreground"), &decl), NONE);
}

#[test]
fn refusal_undeclared_data() {
    let min_width = |text: &str| toml(&format!("[widgets.button.body]\nmin_width_px = \"{text}\""));
    assert_eq!(
        bare(&min_width("= element.progress")),
        [
            "widgets.button.body.min_width_px: `element.progress` is element data the part does not declare"
        ]
    );
    let decl = declare("[declare.widget.button]\ndata = { body = [\"progress\"] }");
    assert_eq!(found(&min_width("= element.progress"), &decl), NONE);
    // The root's inputs need no declaration.
    assert_eq!(bare(&min_width("= element.window_width")), NONE);
}

#[test]
fn refusal_undeclared() {
    let layer = toml(
        "[colors.light]\nkeyed.shade = \"#000000\"\n[[rules]]\nwhen = { variants = \
         [\"primary\"], conditions = [\"device_model:phone_x\"] }\ntarget = \"button.body\"\nset \
         = { opacity = 0.5 }",
    );
    assert_eq!(
        bare(&layer),
        [
            "colors.light.keyed.shade: `shade` is not declared in `color_roles`",
            "rules[0].when.conditions[0]: `@phone_x` is not declared in `device_models`",
            "rules[0].when.variants[0]: `primary` is not declared in `widget.button.variants`",
        ]
    );
    let decl = declare(
        "color_roles = [\"shade\"]\ndevice_models = [\"phone_x\"]\n[declare.widget.button]\n\
         variants = [\"primary\"]",
    );
    assert_eq!(found(&layer, &decl), NONE);
}

#[test]
fn refusal_not_conditions() {
    let rule = |when: &str| {
        toml(&format!(
            "[[rules]]\nwhen = {when}\ntarget = \"tokens\"\nset = {{}}"
        ))
    };
    assert_eq!(
        bare(&rule("{ states = [\"hover\"] }")),
        ["rules[0].when: a colours or tokens rule selects on conditions only"]
    );
    assert_eq!(bare(&rule("{ conditions = [\"scheme:dark\"] }")), NONE);
}

#[test]
fn refusal_widget_unavailable() {
    let layer = toml("[widgets.button.icon]\ndrawn_as = { widget = \"button\", variants = [] }");
    let unavailable = declare("[declare.widget.button]\navailable = false");
    assert_eq!(
        found(&layer, &unavailable),
        ["widgets.button.icon.drawn_as.widget: `button` is not available on the preset"]
    );
    assert_eq!(bare(&layer), NONE);
}

#[test]
fn refusal_accent_input() {
    let layer = toml("[inputs.accent]\nlight = \"#3584e4\"\ndark = \"#3584e4\"");
    let decl = declare("inputs = [\"accent\"]\naccent_inputs = [\"accent\"]");
    assert_eq!(
        found(&layer, &decl),
        ["inputs.accent: the accent is `Theme::set_accent`'s"]
    );
    assert_eq!(found_as(&layer, &decl, LayerKind::Reader), NONE);
    assert_eq!(found_as(&layer, &decl, LayerKind::Palette), NONE);
}

#[test]
fn refusal_listed_twice() {
    let decl = declare("surface_sets = [\"window\", \"view\"]");
    let groups = |list: &str| toml(&format!("[inputs.kde_scheme.groups]\nlight = {list}"));
    assert_eq!(
        found(&groups("[\"window\", \"window\"]"), &decl),
        ["inputs.kde_scheme.groups.light[1]: `window` listed twice"]
    );
    assert_eq!(found(&groups("[\"window\", \"view\"]"), &decl), NONE);
}

/// The problems of a preset stating `body` after its header.
fn preset(body: &str) -> Vec<String> {
    let file = read_preset(&format!("{PRESET_HEAD}{body}")).unwrap();
    strings(check_preset(&file.declarations, &file.layer, None))
        .into_iter()
        .filter(|problem| !problem.ends_with(": missing"))
        .collect()
}

#[test]
fn refusal_size_not_taken() {
    let entry = |size: &str| {
        preset(&format!(
            "[declare.widget.button]\nvariants = [\"primary\"]\nsizes = [\"small\", \"regular\"]\n\
             default_size_by_variant = [{{ variant = \"primary\", size = \"{size}\" }}]"
        ))
    };
    assert_eq!(
        entry("large"),
        [
            "declare.widget.button.default_size_by_variant[0].size: the variant does not take the size `large`"
        ]
    );
    assert_eq!(entry("regular"), NONE);
}

#[test]
fn refusal_unit_mismatch() {
    let decl = declare("params = { layout = { unit = \"px\", keys = [\"gap\"] } }");
    let gap = |key: &str| toml(&format!("[tokens.params.layout]\n{key} = 4"));
    assert_eq!(
        found(&gap("gap"), &decl),
        ["tokens.params.layout.gap: declared in `px`"]
    );
    assert_eq!(found(&gap("gap_px"), &decl), NONE);
}

#[test]
fn refusal_unused() {
    let flavors = |rules: &str| {
        preset(&format!(
            "[declare]\nflavors = [\"widgets\", \"kirigami\"]\n{rules}"
        ))
    };
    assert_eq!(
        flavors(""),
        ["declare.flavors: `kirigami` is declared but unused"]
    );
    assert_eq!(
        flavors(
            "[[rules]]\nwhen = { conditions = [\"flavor:kirigami\"] }\ntarget = \"tokens\"\nset = {}"
        ),
        NONE
    );
}

#[test]
fn refusal_duplicate_key() {
    let keyed = |keys: [&str; 2]| {
        let mut layer = Layer::default();
        layer.bases.colors.light = Some(ColorRolesPatch {
            keyed: Some(
                keys.into_iter()
                    .map(|key| (crate::next::Key::new(key).unwrap(), Value::Stated(color())))
                    .collect(),
            ),
            ..ColorRolesPatch::default()
        });
        layer
    };
    let decl = declare("color_roles = [\"a\", \"b\"]");
    assert_eq!(
        found(&keyed(["a", "a"]), &decl),
        ["colors.light.keyed.a: states the field a states too"]
    );
    assert_eq!(found(&keyed(["a", "b"]), &decl), NONE);
}

#[test]
fn refusal_not_a_part_field() {
    let label = |style: PartStylePatch| {
        let mut layer = Layer::default();
        layer.bases.widgets.button = Some(ButtonPatch {
            label: Some(PartPatch::Style(style)),
            ..ButtonPatch::default()
        });
        layer
    };
    assert_eq!(
        bare(&label(fill(vec![layer_of(BlendMode::Normal)]))),
        ["widgets.button.label.fill: a field `button.label` does not have"]
    );
    assert_eq!(
        bare(&label(PartStylePatch {
            padding: Some(crate::next::units::Sides {
                top: Some(Value::Stated(Px(2.0))),
                right: None,
                bottom: None,
                left: None,
            }),
            ..PartStylePatch::default()
        })),
        NONE
    );
}

#[test]
fn refusal_part_without_widget() {
    let problems = |text: &str| match Layer::from_toml(text) {
        Ok(_) => Vec::new(),
        Err(LayerError::Invalid(problems)) => problems.iter().map(Problem::to_string).collect(),
    };
    assert_eq!(
        problems("[widgets.button.icon]\ndrawn_as = { part = \"label\", variants = [] }"),
        ["widgets.button.icon.drawn_as.part: states `part` without `widget`"]
    );
    assert_eq!(
        problems(
            "[widgets.button.icon]\ndrawn_as = { widget = \"button\", part = \"label\", variants \
             = [] }"
        ),
        NONE
    );
}

#[test]
fn refusal_missing() {
    // The preset's bases, the bottom of the stack, state every field: here
    // the tokens' motion block states only its speed factor.
    let file = read_preset(&format!(
        "{PRESET_HEAD}[tokens.motion]\nspeed_factor = 1\ndurations = {{}}"
    ))
    .unwrap();
    let found = strings(check_preset(&file.declarations, &file.layer, None));
    assert!(
        found.contains(&"tokens.motion.timings: missing".to_owned()),
        "{found:?}"
    );
    assert!(
        !found
            .iter()
            .any(|p| p.starts_with("tokens.motion.durations")),
        "{found:?}"
    );
    assert!(
        found.contains(&"colors.dark: missing".to_owned()),
        "{found:?}"
    );
    assert!(
        found.contains(&"widgets.button: missing".to_owned()),
        "{found:?}"
    );
    // An application layer states what it changes.
    let motion = tokens(TokensPatch {
        motion: Some(MotionTokensPatch {
            speed_factor: Some(Value::Stated(1.0)),
            ..MotionTokensPatch::default()
        }),
        ..TokensPatch::default()
    });
    assert_eq!(bare(&motion), NONE);
}

#[test]
fn accent_ramp_entries_are_declared() {
    let ramp = |key: &str| {
        let mut layer = Layer::default();
        layer.bases.colors.light = Some(ColorRolesPatch {
            global: Some(GlobalRolesPatch {
                accent_ramp: Some(vec![(
                    crate::next::Key::new(key).unwrap(),
                    Value::Expr(Expr::Ref(Ref::GlobalRole(vec![key!("accent")]))),
                )]),
                ..GlobalRolesPatch::default()
            }),
            ..ColorRolesPatch::default()
        });
        layer
    };
    let decl = declare("accent_ramp = [\"light1\"]");
    assert_eq!(
        found(&ramp("light9"), &decl),
        ["colors.light.global.accent_ramp.light9: `light9` is not declared in `accent_ramp`"]
    );
    assert_eq!(found(&ramp("light1"), &decl), NONE);
}

/// A layer whose bases style `button.body`'s corners by `corners`.
fn corners(corners: CornersPatch) -> Layer {
    body(PartStylePatch {
        corners: Some(BlockPatch::Fields(corners)),
        ..PartStylePatch::default()
    })
}

/// Problems name their places as the layer was written (S1): a layer built
/// in Rust by the model's paths, an enum's variant a step of its own; one
/// read from TOML by its keys as written.
#[test]
fn problem_paths_spell_the_layer() {
    let radius = |radius: Radius| {
        corners(CornersPatch {
            top_left: Some(radius),
            ..CornersPatch::default()
        })
    };
    assert_eq!(
        bare(&radius(Radius::Concentric {
            minimum: Value::Stated(Px(-1.0))
        })),
        ["widgets.button.body.corners.top_left.concentric.minimum: out of range: at least zero"]
    );
    assert_eq!(
        bare(&radius(Radius::Px(Value::Stated(Px(-1.0))))),
        ["widgets.button.body.corners.top_left.px: out of range: at least zero"]
    );
    assert_eq!(
        bare(&toml(
            "[widgets.button.body]\ncorners.top_left = { concentric = { minimum_px = -1 } }"
        )),
        [
            "widgets.button.body.corners.top_left.concentric.minimum_px: out of range: at least \
             zero"
        ]
    );
    assert_eq!(
        bare(&toml("[widgets.button.body]\ncorners.top_left_px = -1")),
        ["widgets.button.body.corners.top_left_px: out of range: at least zero"]
    );
    // A font size takes its stated variant's suffix, and none for an
    // expression.
    assert_eq!(
        bare(&toml("[widgets.button.label]\ntext.font.size_pt = -1")),
        ["widgets.button.label.text.font.size_pt: out of range: at least zero"]
    );
    // A map entry with a unit takes its suffix in TOML, none in the model.
    let decl = declare("durations = [\"button\"]");
    let durations = |text: &str| toml(&format!("[tokens.motion.durations]\n{text}"));
    assert_eq!(
        found(&durations("other_ms = 100"), &decl),
        ["tokens.motion.durations.other_ms: `other` is not declared in `durations`"]
    );
    let mut built = Layer::default();
    built.bases.tokens.motion = Some(MotionTokensPatch {
        durations: Some(vec![(key!("other"), Value::Stated(Ms(100.0)))]),
        ..MotionTokensPatch::default()
    });
    assert_eq!(
        found(&built, &decl),
        ["tokens.motion.durations.other: `other` is not declared in `durations`"]
    );
    // A `DrawnAs`'s target: the model's field, TOML's key.
    let unavailable = declare("[declare.widget.button]\navailable = false");
    let mut drawn = Layer::default();
    drawn.bases.widgets.button = Some(ButtonPatch {
        icon: Some(PartPatch::DrawnAs {
            target: crate::next::selector::WidgetOrPart::Widget(
                crate::next::selector::WidgetRef::Typed(crate::next::widgets::WidgetKind::Button),
            ),
            variants: Vec::new(),
        }),
        ..ButtonPatch::default()
    });
    assert_eq!(
        found(&drawn, &unavailable),
        ["widgets.button.icon.drawn_as.target: `button` is not available on the preset"]
    );
    // A theme change's duration below its variant.
    assert_eq!(
        bare(&toml(
            "[tokens]\ntheme_change = { suspend = { duration_ms = 0 } }"
        )),
        ["tokens.theme_change.suspend.duration_ms: a zero duration is `instant`"]
    );
}

/// One fault is one problem at one path (S12).
#[test]
fn one_problem_per_fault() {
    let spring = Timing::Spring {
        mass: f32::NAN,
        stiffness: 100.0,
        damping_ratio: 1.0,
        initial_velocity: 0.0,
        rest_epsilon: 0.001,
        clamp: false,
        time_scale: Value::Stated(1.0),
    };
    assert_eq!(
        bare(&transitions(vec![
            transition(vec![PropertyGroup::Fill], spring),
            transition(
                vec![PropertyGroup::Opacity],
                Timing::CubicBezier(f32::NAN, 0.0, 0.5, 1.0)
            ),
        ])),
        [
            "widgets.button.body.transitions[0].timing.spring.mass: not a finite number",
            "widgets.button.body.transitions[1].timing.cubic_bezier[0]: not a finite number",
        ]
    );
    // A keyed scalar of a `fraction` group that is not finite.
    let decl = declare("params = { opacity = { unit = \"fraction\", keys = [\"dim\"] } }");
    let mut layer = Layer::default();
    layer.bases.tokens.params = Some(vec![(
        key!("opacity"),
        vec![(
            key!("dim"),
            Value::Stated(crate::next::units::Scalar::Number(f32::NAN)),
        )],
    )]);
    assert_eq!(
        found(&layer, &decl),
        ["tokens.params.opacity.dim: not a finite number"]
    );
    // An undeclared group is one problem, not one per key.
    assert_eq!(
        bare(&toml("[tokens.params.layout]\ngap = 4\nspacing = 2")),
        ["tokens.params.layout: `layout` is not declared in `params`"]
    );
}

/// An `inner_may_be_absent` map keeps its place for each entry: a keyed
/// icon context's fields but `size` may be absent (S4.5, S6.3).
#[test]
fn keyed_icon_context_fields_may_be_absent() {
    let decl = declare("icon_contexts = [\"menu\"]");
    let context = |text: &str| toml(&format!("[tokens.icons.sizes.keyed.menu]\n{text}"));
    assert_eq!(
        found(&context("size_px = 16\nrendering_mode = \"absent\""), &decl),
        NONE
    );
    assert_eq!(
        found(&context("size_px = \"absent\""), &decl),
        ["tokens.icons.sizes.keyed.menu.size_px: may not be absent"]
    );
}
