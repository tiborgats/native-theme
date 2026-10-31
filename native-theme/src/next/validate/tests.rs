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

/// A layer's error that is no problem list: none of these fixtures gives
/// one. Its own frame, so the fixtures' frames stay small.
#[cold]
#[inline(never)]
fn not_invalid(error: &LayerError) -> ! {
    panic!("not a problem list: {error}")
}

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
        Err(other) => not_invalid(&other),
    }
}

/// The problems `layer` has as a layer of `kind` over `decl`.
fn found_as(layer: &Layer, decl: &Declarations, kind: LayerKind) -> Vec<String> {
    let cx = Context {
        declarations: decl,
        preset: None,
        kind,
        below: &[],
    };
    strings(check(layer, &cx))
}

/// The problems of an application layer over `decl`, `below` the preset's
/// layer beneath it.
fn found_over(layer: &Layer, decl: &Declarations, below: &Layer) -> Vec<String> {
    let below = [super::Stacked {
        layer: below,
        kind: LayerKind::Preset,
    }];
    let cx = Context {
        declarations: decl,
        preset: None,
        kind: LayerKind::Application,
        below: &below,
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
        from: None,
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

/// A transition's `from` (S2.7, S12): an empty list applies from no
/// state, a flag listed twice, a keyed flag the widget does not declare.
#[test]
fn refusal_transition_from() {
    let from = |from: &str| {
        toml(&format!(
            "[widgets.button.body]\ntransitions = [{{ properties = [\"fill\"], kind = \
             \"interpolate\", duration_ms = 100, delay_ms = 0, timing = \"linear\", \
             suspended_by_theme_change = true, from = {from} }}]"
        ))
    };
    assert_eq!(
        bare(&from("[]")),
        ["widgets.button.body.transitions[0].from: a transition applying from no state"]
    );
    assert_eq!(
        bare(&from("[\"hover\", \"pressed\", \"hover\"]")),
        ["widgets.button.body.transitions[0].from[2]: `hover` listed twice"]
    );
    assert_eq!(
        bare(&from("[\"@spin\"]")),
        [
            "widgets.button.body.transitions[0].from[0]: `@spin` is not declared in `widget.button.flags`"
        ]
    );
    let declared = crate::next::toml::layer::read_preset(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n[declare.widget.button]\n\
         flags = [\"@spin\"]",
    )
    .unwrap()
    .declarations;
    assert_eq!(found(&from("[\"@spin\", \"hover\"]"), &declared), NONE);
}

/// A `StateLayer` transition times the part's `interaction`, so a part
/// without it may not state one (S2.7, S2.12, S12).
#[test]
fn refusal_state_layer_without_interaction() {
    let on = |part: &str| {
        toml(&format!(
            "[widgets.button.{part}]\ntransitions = [{{ properties = [\"state_layer\"], kind = \
             \"interpolate\", duration_ms = 15, delay_ms = 0, timing = \"linear\", \
             suspended_by_theme_change = false }}]"
        ))
    };
    assert_eq!(
        bare(&on("icon")),
        [
            "widgets.button.icon.transitions[0].properties: a `state_layer` transition on a part without `interaction`"
        ]
    );
    assert_eq!(bare(&on("body")), NONE);
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
    // The near miss reaches typing, which refuses the call: `kde_accent`
    // is not implemented yet (S2.2).
    let decl = declare("surface_sets = [\"window\"]");
    assert_eq!(
        found(
            &accent("= kde_accent(window, background_normal, inactive)"),
            &decl
        ),
        [NOT_IMPLEMENTED]
    );
}

/// What typing says of a call of `kde_accent`, which no task has
/// implemented yet (S2.2).
const NOT_IMPLEMENTED: &str = "colors.light.global.accent: the expression does not type: the \
                               platform function `kde_accent` is not implemented yet";

#[test]
fn kde_accent_set_is_declared() {
    // One of KDE's sets, declared by the preset (S2.2, S12).
    let decl = declare("surface_sets = [\"window\"]");
    assert_eq!(
        found(
            &accent("= kde_accent(header, background_normal, inactive)"),
            &decl
        ),
        ["colors.light.global.accent: `header` is not declared in `surface_sets`"]
    );
    assert_eq!(
        found(
            &accent("= kde_accent(window, background_normal, inactive)"),
            &decl
        ),
        [NOT_IMPLEMENTED]
    );
}

/// A palette's surface set the current preset does not declare is skipped,
/// its roles with it, not refused (S12, S14).
#[test]
fn palette_skips_undeclared_surface_sets() {
    let decl = declare("surface_sets = [\"window\"]");
    let layer =
        toml("[colors.light.surfaces.header]\nbackground = \"#000000\"\nforeground = \"= base\"");
    assert_eq!(found_as(&layer, &decl, LayerKind::Palette), NONE);
    // Only the statement is skipped: a palette naming an undeclared set
    // elsewhere is refused there.
    let naming = toml("[colors.light.surfaces.window]\nforeground = \"= role.header.foreground\"");
    assert!(
        found_as(&naming, &decl, LayerKind::Palette).contains(
            &"colors.light.surfaces.window.foreground: `header` is not declared in `surface_sets`"
                .to_owned()
        )
    );
    assert_eq!(
        found(&layer, &decl),
        [
            "colors.light.surfaces.header: `header` is not declared in `surface_sets`",
            "colors.light.surfaces.header.foreground: `base` in a layer's bases, where nothing \
             lies beneath",
        ]
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

/// `kde_shade`'s role is one of `KdeShadeRole`'s, typed when the layer is
/// added (S2.2, S12); a role reference reads the window set's background.
#[test]
fn kde_shade_typed_when_added() {
    let decl = declare("surface_sets = [\"window\"]");
    assert_eq!(
        found(
            &accent("= kde_shade(role.window.background, highlight, 4)"),
            &decl
        ),
        [
            "colors.light.global.accent: the expression does not type: `highlight` is not a \
             KdeShadeRole"
        ]
    );
    assert_eq!(
        found(
            &accent("= kde_shade(role.window.background, light, 4)"),
            &decl
        ),
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
    let opacity = |text: &str| toml(&format!("[widgets.button.body]\nopacity = \"{text}\""));
    assert_eq!(
        bare(&opacity("= element.progress")),
        [
            "widgets.button.body.opacity: `element.progress` is element data the part does not declare"
        ]
    );
    let decl = declare("[declare.widget.button]\ndata = { body = [\"progress\"] }");
    assert_eq!(found(&opacity("= element.progress"), &decl), NONE);
    // The root's inputs need no declaration.
    let min_width = toml("[widgets.button.body]\nmin_width_px = \"= element.window_width\"");
    assert_eq!(bare(&min_width), NONE);
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
            "rules[0].when.conditions[0]: `phone_x` is not declared in `device_models`",
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
    let layer = toml(
        "[widgets.button.icon]\ndrawn_as = { widget = \"button\", part = \"body\", variants = [] }",
    );
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

/// The inputs' values take every value check, as colour roles' (S9, S12).
#[test]
fn input_values_are_checked() {
    let decl = declare("inputs = [\"accent\"]\nsurface_sets = [\"window\"]");
    let layer = toml(
        "[inputs.accent]\nlight = \"= mix(role.header.foreground, #000000, element.progress)\"\n\
         [inputs.accent.contrast.high]\ndark = \"absent\"",
    );
    assert_eq!(
        found_as(&layer, &decl, LayerKind::Reader),
        [
            "inputs.accent.light: `header` is not declared in `surface_sets`",
            "inputs.accent.light: `element.progress` is an element's, not a token's or role's",
            "inputs.accent.contrast.high.dark: may not be absent",
        ]
    );
    let mut built = Layer::default();
    built.bases.inputs.entries.push((
        crate::next::InputName::new("accent").unwrap(),
        crate::next::InputValue::new(crate::next::SchemeMap::one(
            crate::next::Scheme::Light,
            Value::Stated(Color {
                r: f32::NAN,
                ..color()
            }),
        )),
    ));
    assert_eq!(
        found_as(&built, &decl, LayerKind::Reader),
        ["inputs.entries.accent.per_scheme.light: not a finite number"]
    );
    let fine = toml("[inputs.accent]\nlight = \"= role.window.foreground\"");
    assert_eq!(found_as(&fine, &decl, LayerKind::Reader), NONE);
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

/// The display radii may be `Absent`, for a device model the preset does
/// not list (S4.6); another group's keys may not.
#[test]
fn display_params_may_be_absent() {
    let decl = declare(
        "params = { display = { unit = \"px\", keys = [\"corner_radius_top_left\"] }, layout = \
         { unit = \"px\", keys = [\"gap\"] } }",
    );
    let param = |group: &str, key: &str| {
        let mut layer = Layer::default();
        layer.bases.tokens.params = Some(vec![(
            crate::next::Key::new(group).unwrap(),
            vec![(crate::next::Key::new(key).unwrap(), Value::Absent)],
        )]);
        layer
    };
    assert_eq!(
        found(&param("display", "corner_radius_top_left"), &decl),
        NONE
    );
    assert_eq!(
        found(&param("layout", "gap"), &decl),
        ["tokens.params.layout.gap: may not be absent"]
    );
}

/// A flavour's section states only the keys that differ; each key it
/// leaves out is the widget's own (S10).
#[test]
fn flavour_sections_inherit_the_widgets_keys() {
    let entry = "[declare]\nflavors = [\"widgets\", \"kirigami\"]\n[declare.widget.button]\n\
                 variants = [\"@fab\"]\nsizes = [\"regular\"]\n\
                 [declare.flavor.kirigami.widget.button]\n\
                 default_size_by_variant = [{ variant = \"@fab\", size = \"regular\" }]";
    let rule = "\n[[rules]]\nwhen = { conditions = [\"flavor:kirigami\"], variants = [\"@fab\"] }\n\
                target = \"button.body\"\nset = { opacity = 0.5 }";
    assert_eq!(preset(&format!("{entry}{rule}")), NONE);
    // A flavour's own variant set replaces the widget's under it: a rule
    // that can apply under the default flavour selects a variant only
    // `kirigami` declares, and one under `kirigami` a variant `kirigami`'s
    // set leaves out (S12's combinations under each `flavor:` condition).
    let replaced = entry.replace(
        "[declare.flavor.kirigami.widget.button]\n",
        "[declare.flavor.kirigami.widget.button]\nvariants = [\"@pill\"]\n",
    );
    let pill = "\n[[rules]]\nwhen = { variants = [\"@pill\"] }\ntarget = \"button.body\"\n\
                set = { opacity = 0.5 }";
    assert_eq!(
        preset(&format!("{replaced}{pill}{rule}")),
        [
            "rules[0].when.variants[0]: `@pill` is not declared in \
             `flavor.widgets.widget.button.variants`",
            "rules[1].when.variants[0]: `@fab` is not declared in \
             `flavor.kirigami.widget.button.variants`",
            "declare.flavor.kirigami.widget.button.default_size_by_variant[0].variant: `@fab` is \
             not declared in `declare.flavor.kirigami.widget.button.variants`"
        ]
    );
}

/// The structural check takes availability from the default flavour's
/// declarations; a flavour that makes a widget unavailable is S12's
/// combination check's.
#[test]
fn availability_is_the_default_flavours() {
    let decl = declare(
        "flavors = [\"widgets\", \"kirigami\"]\n[declare.flavor.kirigami.widget.button]\n\
         available = false",
    );
    let layer = toml(
        "[[rules]]\nwhen = { conditions = [\"flavor:widgets\"] }\ntarget = \"button.icon\"\n\
         set = { drawn_as = { widget = \"button\", part = \"body\", variants = [] } }",
    );
    assert_eq!(found(&layer, &decl), NONE);
    let unavailable = declare(
        "flavors = [\"widgets\", \"kirigami\"]\n[declare.flavor.widgets.widget.button]\n\
         available = false",
    );
    assert_eq!(
        found(&layer, &unavailable),
        ["rules[0].set.drawn_as.widget: `button` is not available on the preset"]
    );
}

/// An entry of a map of bare blocks holds a block: a patch setting one to
/// a reference, `Absent` or `Auto` is ill-typed, in a base or a rule,
/// whatever lies below it (S6.1, S12, ADD-A123).
#[test]
fn refusal_not_a_block() {
    let decl = declare(
        "text_styles = [\"body\"]\nicon_contexts = [\"menu\"]\nsurface_sets = [\"window\"]",
    );
    let layer = toml(
        "[tokens.text.keyed]\nbody = \"= token.text.roles.body\"\n\
         [tokens.icons.sizes.keyed]\nmenu = \"absent\"\n\
         [[rules]]\nwhen = { conditions = [\"scheme:dark\"] }\ntarget = \"colors\"\n\
         set = { surfaces.window = \"= base\" }",
    );
    assert_eq!(
        found(&layer, &decl),
        [
            "tokens.text.keyed.body: an entry of a map of blocks holds a block, never a \
             reference, `absent` or `auto`",
            "tokens.icons.sizes.keyed.menu: an entry of a map of blocks holds a block, never a \
             reference, `absent` or `auto`",
            "rules[0].set.surfaces.window: an entry of a map of blocks holds a block, never a \
             reference, `absent` or `auto`",
        ]
    );
    let fields = toml("[tokens.text.keyed.body]\nline_height = \"= mul(font_size, 1.2)\"");
    let preset = toml("[tokens.text.keyed.body]\nline_height = \"auto\"");
    assert_eq!(found_over(&fields, &decl, &preset), NONE);
    // A keyed timing is a block stated whole: a reference names no variant.
    assert!(
        Layer::from_toml("[tokens.motion.timings]\nfast = \"= token.motion.timings.slow\"")
            .is_err()
    );
}

#[test]
fn refusal_unit_mismatch() {
    let decl = declare("params = { layout = { unit = \"px\", keys = [\"gap\"] } }");
    let gap = |scalar: crate::next::units::Scalar| {
        let mut layer = Layer::default();
        layer.bases.tokens.params = Some(vec![(
            key!("layout"),
            vec![(key!("gap"), Value::Stated(scalar))],
        )]);
        layer
    };
    assert_eq!(
        found(&gap(crate::next::units::Scalar::Ms(Ms(4.0))), &decl),
        ["tokens.params.layout.gap: declared in `px`"]
    );
    assert_eq!(
        found(&gap(crate::next::units::Scalar::Px(Px(4.0))), &decl),
        NONE
    );
    // In TOML a keyed scalar's number is in its declared unit; a suffixed
    // key is another key (S11, ADD-A123).
    let read = |text: &str| toml(&format!("[tokens.params.layout]\n{text}"));
    assert_eq!(found(&read("gap = 4"), &decl), NONE);
    assert_eq!(found(&read("gap = \"= max(2px, 4px)\""), &decl), NONE);
    assert_eq!(
        found(&read("gap_ms = \"= token.params.layout.gap\""), &decl),
        ["tokens.params.layout.gap_ms: `gap_ms` is not declared in `params`"]
    );
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

/// Keyed variants and device models no rule selects on, and keyed icon
/// contexts and status colours no base states, are unused (S12); the
/// variants S12 exempts are not.
#[test]
fn refusal_unused_variants_models_and_maps() {
    let rule = "[[rules]]\nwhen = { variants = [\"@pill\"], conditions = \
                [\"device_model:phone_x\"] }\ntarget = \"button.body\"\nset = { opacity = 0.5 }";
    let head = "[declare]\ndevice_models = [\"phone_x\"]\nicon_contexts = [\"menu\"]\n\
                status_colors = [\"error\"]\n[declare.widget.button]\nvariants = [\"@pill\", \
                \"@with_arrow\", \"@tall\", \"@fab\"]\nrequires_variants = [\"@tall\"]\n\
                sizes_by_variant = [{ variant = \"@fab\", sizes = [\"regular\"] }]";
    assert_eq!(
        preset(head),
        [
            "declare.widget.button.variants: `@pill` is declared but unused",
            "declare.device_models: `phone_x` is declared but unused",
            "declare.status_colors: `error` is declared but unused",
            "declare.icon_contexts: `menu` is declared but unused",
        ]
    );
    // The rule selects on both; the bases state the context and the colour.
    let file = read_preset(&format!("{PRESET_HEAD}{head}\n{rule}")).unwrap();
    let mut layer = file.layer;
    let size = IconStylePatch {
        size: Some(Value::Stated(Px(16.0))),
        ..IconStylePatch::default()
    };
    layer.bases.tokens.icons = Some(crate::next::tokens::IconTokensPatch {
        sizes: Some(crate::next::tokens::IconSizeTokensPatch {
            keyed: Some(vec![(key!("menu"), BlockPatch::Fields(size))]),
            ..crate::next::tokens::IconSizeTokensPatch::default()
        }),
        ..crate::next::tokens::IconTokensPatch::default()
    });
    let error = PaintPatch::Set(Value::Stated(Paint {
        layers: vec![layer_of(BlendMode::Normal)],
    }));
    layer.bases.widgets.button = Some(ButtonPatch {
        icon: Some(PartPatch::Style(PartStylePatch {
            icon: Some(BlockPatch::Fields(IconStylePatch {
                status_colors: Some(vec![(key!("error"), error)]),
                ..IconStylePatch::default()
            })),
            ..PartStylePatch::default()
        })),
        ..ButtonPatch::default()
    });
    let problems: Vec<String> = strings(check_preset(&file.declarations, &layer, None))
        .into_iter()
        .filter(|problem| !problem.ends_with(": missing"))
        .collect();
    assert_eq!(problems, NONE);
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
        Err(other) => not_invalid(&other),
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
            target: crate::next::selector::WidgetOrPart::Part(
                crate::next::selector::PartRef::Typed(crate::next::widgets::TypedPart::Button(
                    crate::next::widgets::ButtonPart::Body,
                )),
            ),
            variants: Vec::new(),
        }),
        ..ButtonPatch::default()
    });
    assert_eq!(
        found(&drawn, &unavailable),
        ["widgets.button.icon.drawn_as.target: `button` is not available on the preset"]
    );
    // A keyed variant is `"@<key>"` in TOML, no key of its own; in the
    // model a step.
    assert_eq!(
        bare(&toml("[widgets.button.body]\nfocus.kind = \"@reveal\"")),
        ["widgets.button.body.focus.kind: `@reveal` is not declared in `focus_kinds`"]
    );
    // A theme change's duration below its variant.
    assert_eq!(
        bare(&toml(
            "[tokens]\ntheme_change = { suspend = { duration_ms = 0 } }"
        )),
        ["tokens.theme_change.suspend.duration_ms: a zero duration is `instant`"]
    );
}

/// A shorthand is one key: one fault in it is one problem, at the key as
/// written (S11, S12).
#[test]
fn shorthand_faults_at_the_key_as_written() {
    for (text, problem) in [
        (
            "[widgets.button.body]\ncorners_px = -1",
            "widgets.button.body.corners_px: out of range: at least zero",
        ),
        (
            "[widgets.button.body]\nstroke = { width_px = -1 }",
            "widgets.button.body.stroke.width_px: out of range: at least zero",
        ),
        (
            "[widgets.button.body]\nstroke = { paint = \"= 4px\" }",
            "widgets.button.body.stroke.paint: the expression does not type: a length where a \
             paint is expected",
        ),
        (
            "[widgets.button.body]\nfill = { layers = [\"= 4px\"] }",
            "widgets.button.body.fill.layers[0]: the expression does not type: a length where a \
             colour is expected",
        ),
        (
            "[[rules]]\ntarget = \"button.body\"\nset = { fill.append = [\"= 4px\"] }",
            "rules[0].set.fill.append[0]: the expression does not type: a length where a colour \
             is expected",
        ),
        (
            "[[rules]]\ntarget = \"button.body\"\nset = { corners_px = -1 }",
            "rules[0].set.corners_px: out of range: at least zero",
        ),
    ] {
        assert_eq!(bare(&toml(text)), [problem], "{text}");
    }
    // A corner of its own is its own key.
    assert_eq!(
        bare(&toml(
            "[widgets.button.body]\ncorners = { top_left_px = 2, bottom_left_px = -1 }"
        )),
        ["widgets.button.body.corners.bottom_left_px: out of range: at least zero"]
    );
}

/// A rule's shorthand holds while the rule read stays at its index: a rule
/// inserted before it through the layer's fields is named by its own keys,
/// not by the shorthand of the rule it displaced.
#[test]
fn shorthand_follows_its_rule_after_an_edit() {
    let mut layer = toml("[[rules]]\ntarget = \"button.body\"\nset = { corners_px = 4 }");
    let faulty =
        toml("[[rules]]\ntarget = \"button.body\"\nset = { corners = { top_left_px = -1 } }");
    layer.rules.insert(0, faulty.rules[0].clone());
    assert_eq!(
        bare(&layer),
        ["rules[0].set.corners.top_left_px: out of range: at least zero"]
    );
}

/// A layer built in Rust names its places by the model's paths throughout:
/// a rule's selector and body by their fields, an input below `entries`.
#[test]
fn rust_built_rules_spell_model_paths() {
    let mut selector = crate::next::selector::Selector::default();
    selector
        .conditions
        .none_of
        .push(crate::next::selector::Condition::Flavor(key!("kirigami")));
    let mut appended = Layer::default();
    appended.rules.push(crate::next::selector::Rule::new(
        selector,
        crate::next::selector::RuleBody::Part(
            crate::next::selector::PartRef::Typed(crate::next::widgets::TypedPart::Button(
                crate::next::widgets::ButtonPart::Body,
            )),
            PartPatch::Style(PartStylePatch {
                fill: Some(PaintPatch::Append(vec![PaintLayer {
                    kind: PaintKind::Color(Value::Expr(Expr::parse("4px").unwrap())),
                    blend: BlendMode::Normal,
                }])),
                ..PartStylePatch::default()
            }),
        ),
    ));
    assert_eq!(
        bare(&appended),
        [
            "rules[0].body.fill.append[0].kind.color: the expression does not type: a length \
             where a colour is expected",
            "rules[0].selector.conditions.none_of[0]: `kirigami` is not declared in `flavors`",
        ]
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
    // The preset beneath states the context, so the patch creates no entry.
    let preset = context("size_px = 16");
    assert_eq!(
        found_over(
            &context("size_px = 16\nrendering_mode = \"absent\""),
            &decl,
            &preset
        ),
        NONE
    );
    assert_eq!(
        found_over(&context("size_px = \"absent\""), &decl, &preset),
        ["tokens.icons.sizes.keyed.menu.size_px: may not be absent"]
    );
}

/// A text-keyed map's key stated twice is one problem at the key as TOML
/// writes it, quoted (S11, S12).
#[test]
fn a_text_keyed_map_names_its_key_quoted() {
    let mut layer = Layer::default();
    layer.bases.tokens.text = Some(crate::next::tokens::TextTokensPatch {
        fallbacks: Some(crate::next::tokens::FontFallbacksPatch {
            per_family: Some(vec![
                ("Segoe UI".to_owned(), Vec::new()),
                ("Segoe UI".to_owned(), Vec::new()),
            ]),
            ..crate::next::tokens::FontFallbacksPatch::default()
        }),
        ..crate::next::tokens::TextTokensPatch::default()
    });
    assert_eq!(
        bare(&layer),
        ["tokens.text.fallbacks.per_family.\"Segoe UI\": states the field \"Segoe UI\" states too"]
    );
}

/// A font's weight is the CSS and OpenType weight, 1 to 1000 (S2.8,
/// ADD-A123).
#[test]
fn refusal_weight_out_of_range() {
    let weight = |n: u16| toml(&format!("[widgets.button.label]\ntext.font.weight = {n}"));
    assert_eq!(
        bare(&weight(0)),
        ["widgets.button.label.text.font.weight: out of range: 1..=1000"]
    );
    assert_eq!(
        bare(&weight(1001)),
        ["widgets.button.label.text.font.weight: out of range: 1..=1000"]
    );
    assert_eq!(bare(&weight(400)), NONE);
    assert_eq!(bare(&weight(1000)), NONE);
}

/// A reference naming a whole map, no key, names nothing (S2.2, S12): a
/// reference names one entry's value.
#[test]
fn refusal_whole_map_reference() {
    let decl = declare("durations = [\"button\"]");
    let duration = |text: &str| {
        let mut item = transition(vec![PropertyGroup::Fill], Timing::Linear);
        item.duration = Value::Expr(Expr::parse(text).unwrap());
        transitions(vec![item])
    };
    assert_eq!(
        found(&duration("token.motion.durations"), &decl),
        [
            "widgets.button.body.transitions[0].duration: the expression does not type: \
             `token.motion.durations` names nothing"
        ]
    );
    assert_eq!(
        found(&duration("token.motion.durations.button"), &decl),
        NONE
    );
    assert_eq!(
        bare(&toml(
            "[widgets.button.body]\nmin_width_px = \"= token.icons.sizes.keyed\""
        )),
        [
            "widgets.button.body.min_width_px: the expression does not type: \
             `token.icons.sizes.keyed` names nothing"
        ]
    );
}

/// Every expression is typed when its layer is added (S2.2, S12): one that
/// mistypes or names nothing is refused, and a call is refused on an enum
/// block's field as on any field that takes a literal or a reference.
#[test]
fn refusal_ill_typed() {
    let body = |line: &str| toml(&format!("[widgets.button.body]\n{line}"));
    assert_eq!(
        bare(&body("opacity = \"= #ff0000\"")),
        [
            "widgets.button.body.opacity: the expression does not type: a colour where a number \
             is expected"
        ]
    );
    assert_eq!(
        bare(&body("opacity = \"= widget.nope.opacity\"")),
        [
            "widgets.button.body.opacity: the expression does not type: `widget.nope.opacity` names nothing"
        ]
    );
    assert_eq!(
        bare(&body("min_width_px = \"= token.nowhere\"")),
        [
            "widgets.button.body.min_width_px: the expression does not type: `token.nowhere` names nothing"
        ]
    );
    // A part's own fields: the catalogue's list, which gives `label` no fill.
    assert_eq!(
        bare(&toml(
            "[widgets.button.label]\npadding.top_px = \"= part.min_width\""
        )),
        [
            "widgets.button.label.padding.top_px: the expression does not type: `part.min_width` \
             names nothing"
        ]
    );
    // A call on a block-mode enum's field.
    let mut called = transitions(vec![transition(vec![PropertyGroup::Fill], Timing::Linear)]);
    if let Some(button) = called.bases.widgets.button.as_mut()
        && let Some(PartPatch::Style(style)) = button.body.as_mut()
        && let Some(items) = style.transitions.as_mut()
        && let Some(first) = items.first_mut()
    {
        first.timing = Value::Expr(Expr::Call(
            Func::Max,
            vec![
                Expr::Lit(crate::next::expr::Literal::Number(1.0)),
                Expr::Lit(crate::next::expr::Literal::Number(2.0)),
            ],
        ));
    }
    assert_eq!(
        bare(&called),
        ["widgets.button.body.transitions[0].timing: takes a literal or a reference, not a call"]
    );
    // Near misses: a reference of the field's type, a call on a number.
    let decl = declare("timings = [\"fast\"]");
    assert_eq!(bare(&body("opacity = \"= mul(0.5, 2)\"")), NONE);
    assert_eq!(
        bare(&body("min_width_px = \"= add(token.caret.width, 2px)\"")),
        NONE
    );
    let referenced = toml(
        "[widgets.button.body]\ntransitions = [{ properties = [\"fill\"], kind = \
         \"interpolate\", duration_ms = 100, delay_ms = 0, timing = \"= token.motion.timings.fast\", \
         suspended_by_theme_change = false }]",
    );
    assert_eq!(found(&referenced, &decl), NONE);
}

/// A layer whose bases style `button.icon`'s icon by `icon`.
fn icon(icon: IconStylePatch) -> Layer {
    let mut layer = Layer::default();
    layer.bases.widgets.button = Some(ButtonPatch {
        icon: Some(PartPatch::Style(PartStylePatch {
            icon: Some(BlockPatch::Fields(icon)),
            ..PartStylePatch::default()
        })),
        ..ButtonPatch::default()
    });
    layer
}

/// A layer whose bases give `button` one presentation of `event` and
/// `kind`.
fn presenting(
    event: crate::next::blocks::motion::PresentationEvent,
    kind: crate::next::blocks::motion::PresentationKind,
) -> Layer {
    use crate::next::blocks::motion::{PresentationStep, PresentationTransition};
    let mut layer = Layer::default();
    layer.bases.widgets.button = Some(ButtonPatch {
        values: Some(crate::next::widgets::ButtonValuesPatch {
            presentations: Some(vec![PresentationTransition {
                event,
                steps: vec![PresentationStep {
                    part: None,
                    kind,
                    duration: Value::Stated(Ms(100.0)),
                    delay: Value::Stated(Ms(0.0)),
                    timing: Value::Stated(Timing::Linear),
                    stagger: Value::Stated(Ms(0.0)),
                }],
            }]),
            ..crate::next::widgets::ButtonValuesPatch::default()
        }),
        ..ButtonPatch::default()
    });
    layer
}

/// A keyed transition's or presentation step's parameter is typed by its
/// declared unit, as any other keyed scalar (S2.7, S4.6, S12).
#[test]
fn keyed_kind_params_are_typed() {
    let expr = |text: &str| Value::Expr(Expr::parse(text).unwrap());
    let keyed = |value: Value<crate::next::units::Scalar>| {
        let mut item = transition(vec![PropertyGroup::Fill], Timing::Linear);
        item.kind = TransitionKind::Keyed {
            key: key!("bounce"),
            params: vec![(key!("amplitude"), value)],
        };
        transitions(vec![item])
    };
    let decl = declare("transition_kinds = { bounce = { amplitude = \"px\" } }");
    assert_eq!(
        found(&keyed(expr("token.nowhere")), &decl),
        [
            "widgets.button.body.transitions[0].kind.keyed.params.amplitude: the expression does \
             not type: `token.nowhere` names nothing"
        ]
    );
    assert_eq!(
        found(&keyed(expr("#ff0000")), &decl),
        [
            "widgets.button.body.transitions[0].kind.keyed.params.amplitude: the expression does \
             not type: a colour where a length is expected"
        ]
    );
    assert_eq!(found(&keyed(expr("4px")), &decl), NONE);
    let step = |value: Value<crate::next::units::Scalar>| {
        presenting(
            crate::next::blocks::motion::PresentationEvent::Appear,
            crate::next::blocks::motion::PresentationKind::Keyed {
                key: key!("wipe"),
                params: vec![(key!("angle"), value)],
            },
        )
    };
    let decl = declare("presentation_kinds = { wipe = { angle = \"deg\" } }");
    assert_eq!(
        found(&step(expr("4px")), &decl),
        [
            "widgets.button.values.presentations[0].steps[0].kind.keyed.params.angle: the \
             expression does not type: a length where an angle is expected"
        ]
    );
    assert_eq!(found(&step(expr("90deg")), &decl), NONE);
}

/// Every kind of name S10 declares, each refused undeclared and passed
/// declared (S10, S12).
#[test]
fn refusal_undeclared_every_kind() {
    use crate::next::blocks::focus::{FocusIndicatorPatch, FocusKind};
    use crate::next::blocks::motion::{PresentationEvent, PresentationKind};
    use crate::next::blocks::paint::Material;
    let rule = |when: &str| {
        toml(&format!(
            "[[rules]]\nwhen = {when}\ntarget = \"button.body\"\nset = {{ opacity = 0.5 }}"
        ))
    };
    let material = Material {
        identity: key!("mica"),
        blur_radius: Value::Absent,
        tint: Value::Absent,
        saturation: Value::Absent,
        luminosity: Value::Absent,
        noise_opacity: Value::Absent,
        fallback: Value::Stated(color()),
        params: Vec::new(),
    };
    let keyed_transition = Transition {
        kind: TransitionKind::Keyed {
            key: key!("bounce"),
            params: Vec::new(),
        },
        ..transition(vec![PropertyGroup::Icon], Timing::Linear)
    };
    let rows: Vec<(Layer, &str, &str)> = vec![
        (
            rule("{ states = [\"@spin\"] }"),
            "rules[0].when.states[0]: `@spin` is not declared in `widget.button.flags`",
            "[declare.widget.button]\nflags = [\"@spin\"]",
        ),
        (
            rule("{ sizes = [\"large\"] }"),
            "rules[0].when.sizes[0]: `large` is not declared in `widget.button.sizes`",
            "[declare.widget.button]\nsizes = [\"large\"]",
        ),
        (
            rule("{ not_densities = [\"compact\"] }"),
            "rules[0].when.not_densities[0]: `compact` is not declared in `widget.button.densities`",
            "[declare.widget.button]\ndensities = [\"compact\"]",
        ),
        (
            rule("{ conditions = [\"flavor:kirigami\"] }"),
            "rules[0].when.conditions[0]: `kirigami` is not declared in `flavors`",
            "flavors = [\"widgets\", \"kirigami\"]",
        ),
        (
            rule("{ conditions = [\"container_size:button:large\"] }"),
            "rules[0].when.conditions[0]: `large` is not declared in `container_sizes`",
            "container_sizes = { button = [\"regular\", \"large\"] }",
        ),
        (
            rule("{ conditions = [\"system_setting:glow\"] }"),
            "rules[0].when.conditions[0]: `glow` is not declared in `conditions`",
            "conditions = [\"@glow\"]",
        ),
        (
            toml("[colors.light.global]\naccent = \"= role.window.background\""),
            "colors.light.global.accent: `window` is not declared in `surface_sets`",
            "surface_sets = [\"window\"]",
        ),
        (
            toml("[inputs.accent]\nlight = \"#000000\""),
            "inputs.accent: `accent` is not declared in `inputs`",
            "inputs = [\"accent\"]",
        ),
        (
            toml("[tokens.params.layout]\ngap = 4"),
            "tokens.params.layout: `layout` is not declared in `params`",
            "params = { layout = { unit = \"px\", keys = [\"gap\"] } }",
        ),
        (
            toml("[tokens.text.keyed.body]\nline_height = \"auto\""),
            "tokens.text.keyed.body: `body` is not declared in `text_styles`",
            "text_styles = [\"body\"]",
        ),
        (
            toml("[tokens.motion.timings]\nfast = \"linear\""),
            "tokens.motion.timings.fast: `fast` is not declared in `timings`",
            "timings = [\"fast\"]",
        ),
        (
            toml("[tokens.motion.durations]\nfast_ms = 100"),
            "tokens.motion.durations.fast_ms: `fast` is not declared in `durations`",
            "durations = [\"fast\"]",
        ),
        (
            toml("[tokens.elevation]\nraised = []"),
            "tokens.elevation.raised: `raised` is not declared in `elevation`",
            "elevation = [\"raised\"]",
        ),
        (
            toml("[widgets.button.body]\nshadow = { elevation = \"raised\" }"),
            "widgets.button.body.shadow.elevation: `raised` is not declared in `elevation`",
            "elevation = [\"raised\"]",
        ),
        (
            body(fill(vec![PaintLayer {
                kind: PaintKind::Material(material),
                blend: BlendMode::Normal,
            }])),
            "widgets.button.body.fill.layers[0].kind.material.identity: `mica` is not declared \
             in `materials`",
            "materials = [\"mica\"]",
        ),
        (
            body(fill(vec![layer_of(BlendMode::Keyed(key!("glow")))])),
            "widgets.button.body.fill.layers[0].blend.keyed: `@glow` is not declared in \
             `blend_modes`",
            "blend_modes = [\"glow\"]",
        ),
        (
            body(PartStylePatch {
                focus: Some(BlockPatch::Fields(FocusIndicatorPatch {
                    kind: Some(FocusKind::Keyed(key!("reveal"))),
                    ..FocusIndicatorPatch::default()
                })),
                ..PartStylePatch::default()
            }),
            "widgets.button.body.focus.kind.keyed: `@reveal` is not declared in `focus_kinds`",
            "focus_kinds = [\"reveal\"]",
        ),
        (
            transitions(vec![keyed_transition]),
            "widgets.button.body.transitions[0].kind.keyed.key: `@bounce` is not declared in \
             `transition_kinds`",
            "transition_kinds = { bounce = {} }",
        ),
        (
            presenting(
                PresentationEvent::Keyed(key!("expand")),
                PresentationKind::Fade,
            ),
            "widgets.button.values.presentations[0].event.keyed: `@expand` is not declared in \
             `presentation_events`",
            "presentation_events = [\"expand\"]",
        ),
        (
            presenting(
                PresentationEvent::Appear,
                PresentationKind::Keyed {
                    key: key!("wipe"),
                    params: Vec::new(),
                },
            ),
            "widgets.button.values.presentations[0].steps[0].kind.keyed.key: `@wipe` is not \
             declared in `presentation_kinds`",
            "presentation_kinds = { wipe = {} }",
        ),
        (
            icon(IconStylePatch {
                scale: Some(Value::Stated(key!("large"))),
                ..IconStylePatch::default()
            }),
            "widgets.button.icon.icon.scale: `large` is not declared in `symbol_scales`",
            "symbol_scales = [\"large\"]",
        ),
        (
            icon(IconStylePatch {
                symbol_variants: Some(vec![key!("fill")]),
                ..IconStylePatch::default()
            }),
            "widgets.button.icon.icon.symbol_variants[0]: `fill` is not declared in \
             `symbol_variants`",
            "symbol_variants = [\"fill\"]",
        ),
        (
            toml("[tokens.text.rendering]\nengine = \"qt\""),
            "tokens.text.rendering.engine: `qt` is not declared in `rendering_engines`",
            "rendering_engines = [\"qt\"]",
        ),
        (
            toml("[tokens.text.rendering]\nlcd_filter = \"light\""),
            "tokens.text.rendering.lcd_filter: `light` is not declared in `lcd_filters`",
            "lcd_filters = [\"light\"]",
        ),
        (
            toml("[tokens.material]\nspec_version = \"spec2025\""),
            "tokens.material.spec_version: `spec2025` is not declared in `spec_versions`",
            "spec_versions = [\"spec2025\"]",
        ),
    ];
    // The near miss over a preset stating the same, so no map entry is
    // created (S12).
    for (layer, problem, declaration) in rows {
        assert_eq!(bare(&layer), [problem], "{problem}");
        let decl = declare(declaration);
        assert_eq!(found_over(&layer, &decl, &layer), NONE, "{declaration}");
    }
    // A component: none is declared until components exist (M3), and
    // TOML names none yet.
    let mut component = Layer::default();
    component.rules.push(crate::next::selector::Rule::new(
        crate::next::selector::Selector::default(),
        crate::next::selector::RuleBody::Part(
            crate::next::selector::PartRef::Component {
                key: key!("sidebar"),
                part: key!("body"),
            },
            PartPatch::Style(PartStylePatch::default()),
        ),
    ));
    assert_eq!(
        bare(&component),
        ["rules[0].body: `component.sidebar.body` is not declared in `component`"]
    );
}

/// The other structural items, each refused, with its near miss (S2.7,
/// S2.8, S9.1, S9.4, S10, S12).
#[test]
fn refusal_other_items() {
    // Routes: small caps is `TextCase::SmallCaps`, an icon's weight its
    // field.
    let features = |tag: &str| {
        let mut layer = Layer::default();
        layer.bases.widgets.button = Some(ButtonPatch {
            label: Some(PartPatch::Style(PartStylePatch {
                text: Some(BlockPatch::Fields(TextStylePatch {
                    font: Some(FontPatch {
                        features: Some(vec![(
                            crate::next::blocks::text::FeatureTag::new(tag).unwrap(),
                            1,
                        )]),
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
        bare(&features("smcp")),
        ["widgets.button.label.text.font.features: `smcp` duplicates a field's route"]
    );
    assert_eq!(bare(&features("liga")), NONE);
    let axis = |tag: &str| {
        icon(IconStylePatch {
            axes: Some(vec![(AxisTag::new(tag).unwrap(), AxisValue::Fixed(1.0))]),
            ..IconStylePatch::default()
        })
    };
    assert_eq!(
        bare(&axis("wght")),
        ["widgets.button.icon.icon.axes: `wght` duplicates a field's route"]
    );
    assert_eq!(bare(&axis("wdth")), NONE);
    // The font DPI is above zero.
    let dpi = |n: &str| toml(&format!("[tokens.text]\nfont_dpi = {n}"));
    assert_eq!(
        bare(&dpi("0")),
        ["tokens.text.font_dpi: out of range: above zero"]
    );
    assert_eq!(bare(&dpi("96")), NONE);
    // A saturation multiplier is at least zero.
    let saturation = |n: f32| {
        let material = crate::next::blocks::paint::Material {
            identity: key!("mica"),
            blur_radius: Value::Absent,
            tint: Value::Absent,
            saturation: Value::Stated(n),
            luminosity: Value::Absent,
            noise_opacity: Value::Absent,
            fallback: Value::Stated(color()),
            params: Vec::new(),
        };
        body(fill(vec![PaintLayer {
            kind: PaintKind::Material(material),
            blend: BlendMode::Normal,
        }]))
    };
    let mica = declare("materials = [\"mica\"]");
    assert_eq!(
        found(&saturation(-1.0), &mica),
        [
            "widgets.button.body.fill.layers[0].kind.material.saturation: out of range: at least zero"
        ]
    );
    assert_eq!(found(&saturation(1.5), &mica), NONE);
    // A spring's stiffness, damping ratio and rest epsilon.
    let spring = |stiffness: f32, damping_ratio: f32, rest_epsilon: f32| Timing::Spring {
        mass: 1.0,
        stiffness,
        damping_ratio,
        initial_velocity: 0.0,
        rest_epsilon,
        clamp: false,
        time_scale: Value::Stated(1.0),
    };
    assert_eq!(
        bare(&transitions(vec![transition(
            vec![PropertyGroup::Fill],
            spring(0.0, -0.5, 0.0)
        )])),
        [
            "widgets.button.body.transitions[0].timing.spring.stiffness: out of range: above zero",
            "widgets.button.body.transitions[0].timing.spring.damping_ratio: out of range: at \
             least zero",
            "widgets.button.body.transitions[0].timing.spring.rest_epsilon: out of range: above \
             zero",
        ]
    );
    assert_eq!(
        bare(&transitions(vec![transition(
            vec![PropertyGroup::Fill],
            spring(100.0, 0.0, 0.001)
        )])),
        NONE
    );
    // A crossfade's zero duration is `instant`.
    let crossfade = |ms: &str| {
        toml(&format!(
            "[tokens]\ntheme_change = {{ crossfade = {{ duration_ms = {ms}, timing = \"linear\" }} }}"
        ))
    };
    assert_eq!(
        bare(&crossfade("0")),
        ["tokens.theme_change.crossfade.duration_ms: a zero duration is `instant`"]
    );
    assert_eq!(bare(&crossfade("250")), NONE);
    // KDE's lists: a set or a key listed twice, a name that is no scheme
    // key.
    let decl = declare(
        "surface_sets = [\"window\"]\ninputs = [\"kde_scheme.window.background_normal\", \
         \"accent\"]",
    );
    let lists = |text: &str| toml(&format!("[inputs.kde_scheme.{text}"));
    assert_eq!(
        found(
            &lists("inactive_groups]\nlight = [\"window\", \"window\"]"),
            &decl
        ),
        ["inputs.kde_scheme.inactive_groups.light[1]: `window` listed twice"]
    );
    assert_eq!(
        found(
            &lists(
                "unset_keys]\nlight = [\"kde_scheme.window.background_normal\", \
                 \"kde_scheme.window.background_normal\", \"accent\"]"
            ),
            &decl
        ),
        [
            "inputs.kde_scheme.unset_keys.light[1]: `kde_scheme.window.background_normal` listed \
             twice",
            "inputs.kde_scheme.unset_keys.light[2]: `accent` is not declared in `inputs`",
        ]
    );
    assert_eq!(
        found(
            &lists("unset_keys]\ndark = [\"kde_scheme.window.background_normal\"]"),
            &decl
        ),
        NONE
    );
    // A `DrawnAs`'s variant, declared.
    let drawn = toml(
        "[widgets.button.icon]\ndrawn_as = { widget = \"button\", part = \"body\", variants = [\"@pill\"] }",
    );
    assert_eq!(
        bare(&drawn),
        [
            "widgets.button.icon.drawn_as.variants[0]: `@pill` is not declared in `widget.button.variants`"
        ]
    );
    assert_eq!(
        found(
            &drawn,
            &declare("[declare.widget.button]\nvariants = [\"@pill\"]")
        ),
        NONE
    );
}

/// The declarations' own items (S10, S12): an exclusive variant group
/// naming a variant twice; a `default_size_by_variant` entry naming an
/// undeclared variant, or a variant an earlier entry names.
#[test]
fn refusal_declarations() {
    assert_eq!(
        preset(
            "[declare.widget.button]\nvariants = [\"primary\", \"flat\"]\n\
             exclusive_variants = [[\"primary\", \"flat\", \"primary\"]]\n\
             [[rules]]\nwhen = { variants = [\"primary\", \"flat\"] }\ntarget = \"button.body\"\n\
             set = { opacity = 0.5 }"
        ),
        ["declare.widget.button.exclusive_variants[0][2]: `primary` listed twice"]
    );
    let entries = |entries: &str| {
        preset(&format!(
            "[declare.widget.button]\nvariants = [\"primary\"]\nsizes = [\"regular\"]\n\
             default_size_by_variant = {entries}"
        ))
    };
    assert_eq!(
        entries(
            "[{ variant = \"primary\", size = \"regular\" }, { variant = \"primary\", size = \
             \"regular\" }, { variant = \"flat\", size = \"regular\" }]"
        ),
        [
            "declare.widget.button.default_size_by_variant[1]: `primary` listed twice",
            "declare.widget.button.default_size_by_variant[2].variant: `flat` is not declared in \
             `declare.widget.button.variants`",
        ]
    );
    assert_eq!(
        entries("[{ variant = \"primary\", size = \"regular\" }]"),
        NONE
    );
}

/// Each declared keyed name nothing uses, by kind (S12): a system setting
/// (not one for applications), a container size (not the default), an
/// event, a flag, a keyed role no base states.
#[test]
fn refusal_unused_every_kind() {
    let head = "[declare]\nconditions = [\"@glow\", { key = \"@sounds\", for_applications = \
                true }]\ncontainer_sizes = { button = [\"regular\", \"large\"] }\n\
                presentation_events = [\"expand\"]\ncolor_roles = [\"shade\"]\n\
                [declare.widget.button]\nflags = [\"@spin\"]";
    assert_eq!(
        preset(head),
        [
            "declare.widget.button.flags: `@spin` is declared but unused",
            "declare.conditions: `@glow` is declared but unused",
            "declare.container_sizes: `large` is declared but unused",
            "declare.presentation_events: `expand` is declared but unused",
            "declare.color_roles: `shade` is declared but unused",
        ]
    );
}

/// One fieldless variant per `ProblemKind` variant, of the same name, in
/// `ProblemKind`'s order: `layer_error_every_variant_triggers` maps each kind
/// to its own, so the count is the last one's, and two kinds sharing one
/// fail the test.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum V {
    Toml,
    UnknownKey,
    Mistyped,
    UnknownName,
    NotAKey,
    Expression,
    Missing,
    DuplicateKey,
    NotFinite,
    PartWithoutWidget,
    NotAPartField,
    AppendInBases,
    AbsentNotAllowed,
    AutoNotAllowed,
    CallNotAllowed,
    OutOfRange,
    NoLayer,
    MaskAtBottom,
    TooFewStops,
    NoProperties,
    FromNoState,
    RecencyTooSmall,
    RecencyOrder,
    NotInRecencySet,
    StateLayerWithoutInteraction,
    ZeroDuration,
    KeyframesNotIncreasing,
    KeyframeWithoutPart,
    FollowPartOutsideFocus,
    RouteDuplicated,
    KdeAccentArgument,
    PickWithoutColors,
    TooDeep,
    BaseInBases,
    ElementOperand,
    SurfaceInRoles,
    IllTyped,
    OtherElementsPart,
    UndeclaredData,
    Undeclared,
    NotConditions,
    WidgetUnavailable,
    AccentInput,
    ListedTwice,
    SizeNotTaken,
    NotABlock,
    UnitMismatch,
    FieldsOver,
    DrawnAsCycle,
    TextScaleUnderDynamicType,
    AbsentOperand,
    AutoOperand,
    ExpressionCycle,
    ChainTooLong,
    PartNotDrawn,
    CornerAtLayout,
    UnequalOffsets,
    EllipseCorners,
    TextScalingNotImplemented,
    UnequalInsets,
    TextRoleShared,
    TextRoleOrder,
    ShapeOverCorners,
    EntryCreatedUnset,
    ElementSizeInLayout,
    LeftToPreset,
    Unused,
}

impl V {
    /// How many there are: the last one's place, plus one.
    const COUNT: usize = V::Unused as usize + 1;
}

/// Every `LayerError` and `ProblemKind` variant, each triggered by a
/// fixture: S11's reader's kinds by text it refuses, S12's by a layer or a
/// stack it refuses. `index` matches every variant without a wildcard,
/// each to its own `V`, so a new variant does not compile until it is
/// indexed, and fails here until a fixture triggers it.
#[test]
fn layer_error_every_variant_triggers() {
    use crate::next::error::ProblemKind as K;
    fn index(kind: &K) -> V {
        match kind {
            K::Toml(_) => V::Toml,
            K::UnknownKey => V::UnknownKey,
            K::Mistyped { .. } => V::Mistyped,
            K::UnknownName { .. } => V::UnknownName,
            K::NotAKey(_) => V::NotAKey,
            K::Expression(_) => V::Expression,
            K::Missing => V::Missing,
            K::DuplicateKey { .. } => V::DuplicateKey,
            K::NotFinite => V::NotFinite,
            K::PartWithoutWidget => V::PartWithoutWidget,
            K::NotAPartField { .. } => V::NotAPartField,
            K::AppendInBases => V::AppendInBases,
            K::AbsentNotAllowed => V::AbsentNotAllowed,
            K::AutoNotAllowed => V::AutoNotAllowed,
            K::CallNotAllowed => V::CallNotAllowed,
            K::OutOfRange { .. } => V::OutOfRange,
            K::NoLayer => V::NoLayer,
            K::MaskAtBottom => V::MaskAtBottom,
            K::TooFewStops => V::TooFewStops,
            K::NoProperties => V::NoProperties,
            K::FromNoState => V::FromNoState,
            K::RecencyTooSmall => V::RecencyTooSmall,
            K::RecencyOrder { .. } => V::RecencyOrder,
            K::NotInRecencySet { .. } => V::NotInRecencySet,
            K::StateLayerWithoutInteraction => V::StateLayerWithoutInteraction,
            K::ZeroDuration => V::ZeroDuration,
            K::KeyframesNotIncreasing => V::KeyframesNotIncreasing,
            K::KeyframeWithoutPart => V::KeyframeWithoutPart,
            K::FollowPartOutsideFocus => V::FollowPartOutsideFocus,
            K::RouteDuplicated { .. } => V::RouteDuplicated,
            K::KdeAccentArgument { .. } => V::KdeAccentArgument,
            K::PickWithoutColors => V::PickWithoutColors,
            K::TooDeep => V::TooDeep,
            K::BaseInBases => V::BaseInBases,
            K::ElementOperand { .. } => V::ElementOperand,
            K::SurfaceInRoles => V::SurfaceInRoles,
            K::IllTyped { .. } => V::IllTyped,
            K::OtherElementsPart { .. } => V::OtherElementsPart,
            K::UndeclaredData { .. } => V::UndeclaredData,
            K::Undeclared { .. } => V::Undeclared,
            K::NotConditions => V::NotConditions,
            K::WidgetUnavailable { .. } => V::WidgetUnavailable,
            K::AccentInput => V::AccentInput,
            K::ListedTwice { .. } => V::ListedTwice,
            K::SizeNotTaken { .. } => V::SizeNotTaken,
            K::NotABlock => V::NotABlock,
            K::UnitMismatch { .. } => V::UnitMismatch,
            K::FieldsOver { .. } => V::FieldsOver,
            K::DrawnAsCycle => V::DrawnAsCycle,
            K::TextScaleUnderDynamicType => V::TextScaleUnderDynamicType,
            K::AbsentOperand { .. } => V::AbsentOperand,
            K::AutoOperand { .. } => V::AutoOperand,
            K::ExpressionCycle => V::ExpressionCycle,
            K::ChainTooLong => V::ChainTooLong,
            K::PartNotDrawn { .. } => V::PartNotDrawn,
            K::CornerAtLayout { .. } => V::CornerAtLayout,
            K::UnequalOffsets { .. } => V::UnequalOffsets,
            K::UnequalInsets { .. } => V::UnequalInsets,
            K::TextRoleShared { .. } => V::TextRoleShared,
            K::TextRoleOrder { .. } => V::TextRoleOrder,
            K::ShapeOverCorners => V::ShapeOverCorners,
            K::EllipseCorners => V::EllipseCorners,
            K::TextScalingNotImplemented => V::TextScalingNotImplemented,
            K::EntryCreatedUnset { .. } => V::EntryCreatedUnset,
            K::ElementSizeInLayout { .. } => V::ElementSizeInLayout,
            K::LeftToPreset { .. } => V::LeftToPreset,
            K::Unused { .. } => V::Unused,
        }
    }
    fn problems(result: Result<(), LayerError>) -> Vec<Problem> {
        match result {
            Ok(()) => Vec::new(),
            Err(LayerError::Invalid(problems)) => problems,
            Err(other) => not_invalid(&other),
        }
    }
    let over = |layer: &Layer, decl: &Declarations, below: &[Layer]| {
        let below: Vec<super::Stacked<'_>> = below
            .iter()
            .map(|layer| super::Stacked {
                layer,
                kind: LayerKind::Preset,
            })
            .collect();
        let cx = Context {
            declarations: decl,
            preset: None,
            kind: LayerKind::Application,
            below: &below,
        };
        problems(check(layer, &cx))
    };
    let none = Declarations::default();
    // S11's reader: text it refuses.
    let read = |text: &str| match Layer::from_toml(text) {
        Ok(layer) => over(&layer, &none, &[]),
        Err(LayerError::Invalid(problems)) => problems,
        Err(other) => not_invalid(&other),
    };
    let preset_problems = |body: &str| {
        let file = read_preset(&format!("{PRESET_HEAD}{body}")).unwrap();
        problems(check_preset(&file.declarations, &file.layer, None))
    };
    let mut keyed = Layer::default();
    keyed.bases.colors.light = Some(ColorRolesPatch {
        keyed: Some(vec![
            (key!("a"), Value::Stated(color())),
            (key!("a"), Value::Stated(color())),
        ]),
        ..ColorRolesPatch::default()
    });
    let nan = body(PartStylePatch {
        opacity: Some(Value::Stated(Fraction(f32::NAN))),
        ..PartStylePatch::default()
    });
    let append = body(PartStylePatch {
        fill: Some(PaintPatch::Append(vec![layer_of(BlendMode::Normal)])),
        ..PartStylePatch::default()
    });
    let stops = body(fill(vec![PaintLayer {
        kind: PaintKind::Linear {
            angle: crate::next::units::Deg(0.0),
            stops: vec![Stop {
                position: 0.0,
                color: Value::Stated(color()),
            }],
            repeat: false,
            extent: None,
            offset: None,
        },
        blend: BlendMode::Normal,
    }]));
    let suspend = tokens(TokensPatch {
        theme_change: Some(ThemeChange::Suspend {
            duration: Value::Stated(Ms(0.0)),
        }),
        ..TokensPatch::default()
    });
    let keyframes = {
        let keyframes = [0.5, 0.5]
            .into_iter()
            .map(|at| Keyframe {
                at: Fraction(at),
                part: None,
                timing: Value::Absent,
                patch: PartStylePatch::default(),
            })
            .collect();
        let mut layer = Layer::default();
        layer.bases.widgets.button = Some(ButtonPatch {
            icon: Some(PartPatch::Style(PartStylePatch {
                icon: Some(BlockPatch::Fields(IconStylePatch {
                    animation: Some(BlockPatch::Set(Value::Stated(Loop {
                        period: Value::Stated(Ms(1000.0)),
                        timing: Value::Stated(Timing::Linear),
                        keyframes,
                    }))),
                    ..IconStylePatch::default()
                })),
                ..PartStylePatch::default()
            })),
            ..ButtonPatch::default()
        });
        layer
    };
    let route = {
        let mut layer = Layer::default();
        layer.bases.widgets.button = Some(ButtonPatch {
            label: Some(PartPatch::Style(PartStylePatch {
                text: Some(BlockPatch::Fields(TextStylePatch {
                    font: Some(FontPatch {
                        axes: Some(vec![(AxisTag::new("wght").unwrap(), AxisValue::Fixed(1.0))]),
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
    let deep = body(PartStylePatch {
        opacity: Some(Value::Expr((0..=MAX_DEPTH).fold(
            Expr::Lit(crate::next::expr::Literal::Number(1.0)),
            |expr, _| Expr::Call(Func::Floor, vec![expr]),
        ))),
        ..PartStylePatch::default()
    });
    let mismatch = {
        let mut layer = Layer::default();
        layer.bases.tokens.params = Some(vec![(
            key!("layout"),
            vec![(
                key!("gap"),
                Value::Stated(crate::next::units::Scalar::Ms(Ms(4.0))),
            )],
        )]);
        layer
    };
    let layout = declare("params = { layout = { unit = \"px\", keys = [\"gap\"] } }");
    let unavailable = declare("[declare.widget.button]\navailable = false");
    let accents = declare("inputs = [\"accent\"]\naccent_inputs = [\"accent\"]");
    let sets = declare("surface_sets = [\"window\", \"view\"]");
    let chain = {
        let names: Vec<String> = (0..34).map(|i| format!("\"k{i}\"")).collect();
        let decl = declare(&format!(
            "params = {{ layout = {{ unit = \"px\", keys = [{}] }} }}",
            names.join(", ")
        ));
        let mut text = String::from("[tokens.params.layout]\n");
        for i in 0..33 {
            text.push_str(&format!("k{i} = \"= token.params.layout.k{}\"\n", i + 1));
        }
        text.push_str("k33 = 4\n");
        over(&toml(&text), &decl, &[])
    };
    let styles = declare("text_styles = [\"body\", \"caption\"]");
    let data = declare("[declare.widget.button]\ndata = { body = [\"width\"] }");
    let shape = {
        // No catalogue part lists `shape` with `corners` yet, and no shape
        // can be stated (`MaterialShape` is filled at M1b): a reference on
        // `button.body`'s, which the structural check refuses, is checked
        // over its combinations here directly.
        let mut layer = Layer::default();
        layer.rules.push(crate::next::selector::Rule::new(
            crate::next::selector::Selector::default(),
            crate::next::selector::RuleBody::Part(
                crate::next::selector::PartRef::Typed(crate::next::widgets::TypedPart::Button(
                    crate::next::widgets::ButtonPart::Body,
                )),
                PartPatch::Style(PartStylePatch {
                    shape: Some(Value::Expr(Expr::Ref(Ref::Base))),
                    corners: Some(BlockPatch::Fields(CornersPatch {
                        curve: Some(crate::next::blocks::outline::CornerCurve::Continuous),
                        ..CornersPatch::default()
                    })),
                    ..PartStylePatch::default()
                }),
            ),
        ));
        let layers = [super::Stacked {
            layer: &layer,
            kind: LayerKind::Application,
        }];
        let cx = Context {
            declarations: &none,
            preset: None,
            kind: LayerKind::Application,
            below: &[],
        };
        super::combos::check(&super::stack::Stack::new(&layers, cx))
            .into_iter()
            .map(|(_, problem)| problem)
            .collect::<Vec<Problem>>()
    };
    let left = {
        let decl = declare("surface_sets = [\"window\"]");
        let preset = toml("[colors.light.surfaces.window]\ndecoration_focus = \"#000001\"");
        let palette = toml("[colors.light.surfaces.window]\nlink = \"#000002\"");
        let layers = [
            super::Stacked {
                layer: &preset,
                kind: LayerKind::Preset,
            },
            super::Stacked {
                layer: &palette,
                kind: LayerKind::Palette,
            },
        ];
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Palette,
            below: &[],
        };
        super::combos::mapping_palette(&super::stack::Stack::new(&layers, cx), 1)
    };
    let rule = |when: &str, target: &str, set: &str| {
        format!("[[rules]]\nwhen = {when}\ntarget = \"{target}\"\nset = {set}\n")
    };
    let fixtures: Vec<(V, Vec<Problem>)> = vec![
        (V::Toml, read("[widgets")),
        (V::UnknownKey, read("[widgets.button.body]\nshine = 1")),
        (V::Mistyped, read("[widgets.button.body]\nopacity = true")),
        (
            V::UnknownName,
            read("[widgets.button.icon.icon]\nrendering_mode = \"glossy\""),
        ),
        (V::NotAKey, read("[colors.light.keyed]\nBad = \"#000000\"")),
        (
            V::Expression,
            read("[widgets.button.body]\nopacity = \"= add(\""),
        ),
        (V::Missing, preset_problems("")),
        (
            V::DuplicateKey,
            over(&keyed, &declare("color_roles = [\"a\"]"), &[]),
        ),
        (V::NotFinite, over(&nan, &none, &[])),
        (
            V::PartWithoutWidget,
            read("[widgets.button.icon]\ndrawn_as = { part = \"label\", variants = [] }"),
        ),
        (
            V::NotAPartField,
            read("[widgets.button.label]\nopacity = 0.5"),
        ),
        (V::AppendInBases, over(&append, &none, &[])),
        (
            V::AbsentNotAllowed,
            read("[widgets.button.body]\nopacity = \"absent\""),
        ),
        (
            V::AutoNotAllowed,
            read("[widgets.button.body]\npadding.top_px = \"auto\""),
        ),
        (
            V::CallNotAllowed,
            read("[widgets.button.values]\nshows_icon = \"= max(1, 2)\""),
        ),
        (V::OutOfRange, read("[widgets.button.body]\nopacity = 1.5")),
        (V::NoLayer, over(&body(fill(Vec::new())), &none, &[])),
        (
            V::MaskAtBottom,
            over(
                &body(fill(vec![
                    layer_of(BlendMode::Mask),
                    layer_of(BlendMode::Normal),
                ])),
                &none,
                &[],
            ),
        ),
        (V::TooFewStops, over(&stops, &none, &[])),
        (
            V::NoProperties,
            over(
                &transitions(vec![transition(Vec::new(), Timing::Linear)]),
                &none,
                &[],
            ),
        ),
        (V::ZeroDuration, over(&suspend, &none, &[])),
        (V::KeyframesNotIncreasing, over(&keyframes, &none, &[])),
        (
            V::KeyframeWithoutPart,
            super::checks::triggers::keyframe_without_part(),
        ),
        (
            V::FollowPartOutsideFocus,
            read("[widgets.button.body]\ncorners.top_left = \"follow_part\""),
        ),
        (V::RouteDuplicated, over(&route, &none, &[])),
        (
            V::KdeAccentArgument,
            read(
                "[colors.light.global]\naccent = \"= kde_accent(osd, background_normal, inactive)\"",
            ),
        ),
        (
            V::PickWithoutColors,
            read("[colors.light.global]\naccent = \"= pick(1)\""),
        ),
        (V::TooDeep, over(&deep, &none, &[])),
        (
            V::BaseInBases,
            read("[colors.light.global]\naccent = \"= base\""),
        ),
        (
            V::ElementOperand,
            read("[colors.light.global]\naccent = \"= mix(#000000, #ffffff, element.progress)\""),
        ),
        (
            V::SurfaceInRoles,
            read("[colors.light.global]\naccent = \"= surface.foreground\""),
        ),
        (
            V::IllTyped,
            read("[widgets.button.body]\nopacity = \"= #ff0000\""),
        ),
        (
            V::OtherElementsPart,
            super::checks::triggers::other_elements_part(),
        ),
        (
            V::UndeclaredData,
            read("[widgets.button.body]\nopacity = \"= element.progress\""),
        ),
        (
            V::Undeclared,
            read("[colors.light.global]\naccent = \"= role.window.background\""),
        ),
        (
            V::NotConditions,
            read(&rule("{ states = [\"hover\"] }", "tokens", "{}")),
        ),
        (
            V::WidgetUnavailable,
            over(
                &toml(
                    "[widgets.button.icon]\ndrawn_as = { widget = \"button\", part = \"body\", variants = [] }",
                ),
                &unavailable,
                &[],
            ),
        ),
        (
            V::AccentInput,
            over(&toml("[inputs.accent]\nlight = \"#3584e4\""), &accents, &[]),
        ),
        (
            V::ListedTwice,
            over(
                &toml("[inputs.kde_scheme.groups]\nlight = [\"window\", \"window\"]"),
                &sets,
                &[],
            ),
        ),
        (
            V::SizeNotTaken,
            preset_problems(
                "[declare.widget.button]\nvariants = [\"primary\"]\nsizes = [\"regular\"]\n\
                 default_size_by_variant = [{ variant = \"primary\", size = \"large\" }]",
            ),
        ),
        (
            V::NotABlock,
            over(
                &toml("[tokens.text.keyed]\nbody = \"= token.text.roles.body\""),
                &styles,
                &[],
            ),
        ),
        (V::UnitMismatch, over(&mismatch, &layout, &[])),
        (
            V::FieldsOver,
            read(&format!(
                "{}{}",
                rule(
                    "{ conditions = [\"scheme:dark\"] }",
                    "button.body",
                    "{ stroke = \"absent\" }"
                ),
                rule(
                    "{ conditions = [\"reduced_motion\"] }",
                    "button.body",
                    "{ stroke.top.width_px = 2 }"
                )
            )),
        ),
        (
            V::DrawnAsCycle,
            read("[widgets.button.icon]\ndrawn_as = { widget = \"button\", variants = [] }"),
        ),
        (
            V::TextScaleUnderDynamicType,
            read(
                "[tokens.text]\nscaling = \"dynamic_type\"\n[widgets.button.body]\n\
                  min_height_px = \"= mul(24px, text_scale)\"",
            ),
        ),
        (
            V::AbsentOperand,
            read(
                "[widgets.button.body]\nmargin.top_px = \"absent\"\n\
                  min_width_px = \"= add(part.margin.top, 4px)\"",
            ),
        ),
        (
            V::AutoOperand,
            read(
                "[widgets.button.body]\nmax_width_px = \"auto\"\n\
                  min_width_px = \"= add(part.max_width, 4px)\"",
            ),
        ),
        (
            V::ExpressionCycle,
            read(
                "[widgets.button.body]\nmin_width_px = \"= part.min_height\"\n\
                  min_height_px = \"= part.min_width\"",
            ),
        ),
        (V::ChainTooLong, chain),
        (
            V::PartNotDrawn,
            read(&format!(
                "[widgets.button.label]\npadding.top_px = \"= widget.menu_indicator.min_height\"\n\
                 [widgets.button.menu_indicator]\nabsent = true\n{}",
                ""
            )),
        ),
        (
            V::CornerAtLayout,
            read(
                "[widgets.button.body]\ncorners.top_left = { concentric = { minimum_px = 2 } }\n\
                  focus.width_px = \"= part.corners.top_left\"",
            ),
        ),
        (
            V::UnequalOffsets,
            read(
                "[widgets.button.body.focus]\ncorners.top_left = \"follow_part\"\n\
                  offset.top_px = 2\noffset.left_px = 3",
            ),
        ),
        (
            V::UnequalInsets,
            read(
                "[widgets.button.body]\ncorners.top_left = { concentric = { minimum_px = 2 } }\n\
                  margin.top_px = 2\nmargin.left_px = 3",
            ),
        ),
        (
            V::TextRoleShared,
            over(
                &toml(
                    "[tokens.text.roles]\nbody = \"= token.text.keyed.body\"\n\
                       caption = \"= token.text.keyed.body\"",
                ),
                &styles,
                &[],
            ),
        ),
        (
            V::TextRoleOrder,
            over(
                &toml(
                    "[tokens.text.roles]\ncaption = \"= token.text.keyed.body\"\n\
                       body = \"= token.text.roles.caption\"",
                ),
                &styles,
                &[],
            ),
        ),
        (V::ShapeOverCorners, shape),
        (
            V::EntryCreatedUnset,
            over(
                &toml("[tokens.text.keyed.caption]\nline_height = \"auto\""),
                &styles,
                &[],
            ),
        ),
        (
            V::ElementSizeInLayout,
            over(
                &toml("[widgets.button.body]\npadding.left_px = \"= mul(element.width, 0.1)\""),
                &data,
                &[],
            ),
        ),
        (V::LeftToPreset, left),
        (
            V::Unused,
            preset_problems("[declare]\nflavors = [\"widgets\", \"kirigami\"]"),
        ),
        (
            V::FromNoState,
            over(
                &toml(
                    "[widgets.button.body]\ntransitions = [{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = 0, timing = \"linear\", suspended_by_theme_change = true, from = [] }]",
                ),
                &Declarations::default(),
                &[],
            ),
        ),
        (
            V::StateLayerWithoutInteraction,
            over(
                &toml(
                    "[widgets.button.label]\ntext.fill = \"#000000\"\n[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.icon\"\nset = { transitions = [{ properties = [\"state_layer\"], kind = \"interpolate\", duration_ms = 15, delay_ms = 0, timing = \"linear\", suspended_by_theme_change = false }] }",
                ),
                &Declarations::default(),
                &[],
            ),
        ),
        (
            V::EllipseCorners,
            over(
                &toml(
                    "[widgets.button.body]\ncorners.top_left = \"ellipse\"\ncorners.top_right_px = 4\ncorners.bottom_right = \"ellipse\"\ncorners.bottom_left = \"ellipse\"\ncorners.curve = \"circular\"",
                ),
                &Declarations::default(),
                &[],
            ),
        ),
        (
            V::TextScalingNotImplemented,
            over(
                &toml("[tokens.text]\nscaling = \"dynamic_type\""),
                &Declarations::default(),
                &[],
            ),
        ),
        (
            V::RecencyTooSmall,
            preset_problems("[declare.widget.button]\nrecency = { glow = [\"hover\"] }"),
        ),
        (
            V::RecencyOrder,
            preset_problems(
                "[declare.widget.button]\nrecency = { glow = [\"hover\", \"pressed\"], ring = [\"pressed\", \"hover\"] }",
            ),
        ),
        (
            V::NotInRecencySet,
            over(
                &toml(
                    "[[rules]]\nwhen = { latest = { glow = \"pressed\" } }\ntarget = \"button.body\"\nset = { opacity = 0.5 }",
                ),
                &declare(
                    "[declare.widget.button]\nrecency = { glow = [\"hover\", \"focus_visible\"] }",
                ),
                &[],
            ),
        ),
    ];
    // Each variant is one `V`: two kinds the fixtures meet never share one.
    let mut met: Vec<(std::mem::Discriminant<K>, V)> = Vec::new();
    for (_, found) in &fixtures {
        for problem in found {
            let held = (std::mem::discriminant(&problem.kind), index(&problem.kind));
            assert!(
                met.iter().all(|(d, v)| (*d == held.0) == (*v == held.1)),
                "two variants indexed as {:?}",
                held.1
            );
            if !met.contains(&held) {
                met.push(held);
            }
        }
    }
    let mut covered = [false; V::COUNT];
    for (expected, found) in &fixtures {
        let kinds: Vec<V> = found.iter().map(|problem| index(&problem.kind)).collect();
        assert!(kinds.contains(expected), "variant {expected:?}: {found:?}");
        let slot = covered.get_mut(*expected as usize);
        assert!(slot.is_some(), "{expected:?} past `V::COUNT`");
        if let Some(slot) = slot {
            assert!(!*slot, "{expected:?} has two fixtures");
            *slot = true;
        }
    }
    let missing: Vec<usize> = (0..V::COUNT)
        .filter(|i| !covered.get(*i).copied().unwrap_or(false))
        .collect();
    assert_eq!(missing, Vec::<usize>::new());
    // `LayerError`'s every variant: `Invalid`, which every fixture above
    // returns, and the stack's refusals (S8, S9.2).
    every_layer_error();
}

/// Every `LayerError` variant, each triggered: `Invalid` by a layer's
/// problem, the others by the stack's refusals. Its own frame, so the
/// fixtures above keep theirs small.
#[inline(never)]
fn every_layer_error() {
    let none = Declarations::default();
    let invalid = check(
        &bare_layer_with_a_problem(),
        &Context {
            declarations: &none,
            preset: None,
            kind: LayerKind::Application,
            below: &[],
        },
    );
    let mut errors = crate::next::resolve::refusals();
    errors.extend(invalid.err());
    let mut seen = [false; 3];
    for error in &errors {
        let index = match error {
            LayerError::Invalid(found) => {
                assert!(!found.is_empty());
                0
            }
            LayerError::ReaderPresent => 1,
            LayerError::AccentInputsMissing(names) => {
                assert!(!names.is_empty());
                2
            }
        };
        seen[index] = true;
    }
    assert_eq!(seen, [true; 3], "{errors:?}");
}

/// A layer with one structural problem: an opacity outside `0..=1`.
fn bare_layer_with_a_problem() -> Layer {
    body(PartStylePatch {
        opacity: Some(Value::Stated(Fraction(2.0))),
        ..PartStylePatch::default()
    })
}

/// Recency (S7.2, S10, S12, ADD-A124): a set of two flags or more, none
/// twice, a keyed one declared, no two sets ordering two flags oppositely; a
/// `latest` term naming a declared set and one of its members; a declared
/// set no rule selects on is unused.
#[test]
fn refusal_recency() {
    let selecting = "[[rules]]\nwhen = { latest = { glow = \"hover\" } }\ntarget = \
                     \"button.body\"\nset = { opacity = 0.5 }";
    assert_eq!(
        preset(&format!(
            "[declare.widget.button]\nrecency = {{ glow = [\"hover\", \"focus_visible\"] }}\n\
             {selecting}"
        )),
        NONE
    );
    assert_eq!(
        preset("[declare.widget.button]\nrecency = { glow = [\"hover\", \"focus_visible\"] }"),
        ["declare.widget.button.recency: `glow` is declared but unused"]
    );
    assert_eq!(
        preset(&format!(
            "[declare.widget.button]\nrecency = {{ glow = [\"hover\", \"@spin\", \"hover\"] }}\n\
             {selecting}"
        )),
        [
            "declare.widget.button.recency.glow[1]: `@spin` is not declared in `declare.widget.button.flags`",
            "declare.widget.button.recency.glow[2]: `hover` listed twice",
        ]
    );
    let decl =
        declare("[declare.widget.button]\nrecency = { glow = [\"hover\", \"focus_visible\"] }");
    let rule = |latest: &str| {
        toml(&format!(
            "[[rules]]\nwhen = {{ latest = {latest} }}\ntarget = \"button.body\"\n\
             set = {{ opacity = 0.5 }}"
        ))
    };
    assert_eq!(found(&rule("{ glow = \"focus_visible\" }"), &decl), NONE);
    assert_eq!(
        found(&rule("{ ring = \"hover\" }"), &decl),
        ["rules[0].when.latest.ring: `ring` is not declared in `widget.button.recency`"]
    );
    assert_eq!(
        found(&rule("{ glow = \"pressed\" }"), &decl),
        ["rules[0].when.latest.glow: `pressed` is not a member of the recency set `glow`"]
    );
}

/// A keyed flag a rule selects on in a `latest` term alone is used (S12).
#[test]
fn refusal_unused_flag_in_a_latest_term() {
    let found = preset(
        "[declare.widget.button]\nflags = [\"@x\"]\nrecency = { s = [\"@x\", \"hover\"] }\n\
         [[rules]]\nwhen = { latest = { s = \"@x\" } }\ntarget = \"button.body\"\n\
         set = { opacity = 1 }",
    );
    assert!(
        !found
            .iter()
            .any(|problem| problem.contains("`@x` is declared but unused")),
        "{found:?}"
    );
}

/// S9.4's non-linear curves are refused until they are implemented; linear
/// scaling is not.
#[test]
fn refusal_text_scaling_not_implemented() {
    for kind in ["android_non_linear", "dynamic_type"] {
        assert_eq!(
            bare(&toml(&format!("[tokens.text]\nscaling = \"{kind}\""))),
            ["tokens.text.scaling: a text scaling whose curve is not implemented yet"],
            "{kind}"
        );
    }
    assert_eq!(bare(&toml("[tokens.text]\nscaling = \"linear\"")), NONE);
}
