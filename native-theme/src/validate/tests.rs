//! S12's structural checks, one `refusal_<item>` test per item: a layer
//! that triggers exactly that problem, and a near miss that does not. Every
//! number and name here is synthetic.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::walk::Walk;
use super::{Context, LayerKind, Reused, check, check_preset};
use crate::blocks::BlockPatch;
use crate::blocks::focus::FocusIndicatorPatch;
use crate::blocks::icon::IconStylePatch;
use crate::blocks::interaction::{IndicationBox, InteractionPatch, RippleStylePatch};
use crate::blocks::motion::{
    Keyframe, Loop, PropertyGroup, StepJump, Timing, Transition, TransitionEngine, TransitionKind,
};
use crate::blocks::outline::{CornersPatch, Radius};
use crate::blocks::paint::{BlendMode, Paint, PaintKind, PaintLayer, PaintPatch, Stop};
use crate::blocks::part_style::PartStylePatch;
use crate::blocks::shadow::Shadow;
use crate::blocks::text::{AxisTag, AxisValue, FontPatch, TextStylePatch};
use crate::color::Color;
use crate::declare::Declarations;
use crate::error::{LayerError, Problem};
use crate::expr::{Expr, Func, MAX_DEPTH, Ref};
use crate::key;
use crate::layer::Layer;
use crate::roles::{ColorRolesPatch, GlobalRolesPatch};
use crate::selector::PartRef;
use crate::tokens::{MotionTokensPatch, TokensPatch};
use crate::toml::layer::read_preset;
use crate::toml::read::{Spelling, read_in};
use crate::units::{Fraction, Ms, Px};
use crate::value::Value;
use crate::widgets::{ButtonPart, ButtonPatch, PartPatch, TypedPart, WidgetKind};

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
        reader_accent_left_out: false,
        below: &[],
    };
    strings(check(&[layer], &cx, &Reused::default()))
}

/// The problems of an application layer over `decl`, `below` the preset's
/// layer beneath it.
fn found_over(layer: &Layer, decl: &Declarations, below: &Layer) -> Vec<String> {
    let below = [super::Stacked {
        layer: below,
        kind: LayerKind::Preset,
        left_out: None,
    }];
    let cx = Context {
        declarations: decl,
        preset: None,
        kind: LayerKind::Application,
        reader_accent_left_out: false,
        below: &below,
    };
    strings(check(&[layer], &cx, &Reused::default()))
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
    layer.bases.widgets.button = Some(Box::new(ButtonPatch {
        body: Some(PartPatch::of_style(style)),
        ..ButtonPatch::default()
    }));
    layer
}

/// A layer whose bases state `tokens`.
fn tokens(tokens: TokensPatch) -> Layer {
    let mut layer = Layer::default();
    layer.bases.tokens = Box::new(tokens);
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
    // An engine that computes the timing (S2.7, S12): libadwaita's
    // animation a spring, GTK's CSS transitions any other.
    let engine = if matches!(timing, Timing::Spring { .. }) {
        TransitionEngine::AdwAnimation { replays: false }
    } else {
        TransitionEngine::GtkCss
    };
    Transition {
        properties,
        kind: TransitionKind::Interpolate,
        duration: Value::Stated(Ms(100.0)),
        delay: Value::Stated(Ms(0.0)),
        timing: Value::Stated(timing),
        engine,
        from: None,
    }
}

/// A `WinUiAnimatedIcon` transition of the icon, the one engine that runs a
/// `Keyed` kind (S2.7, S12): its visual `accept` times it, so it states no
/// duration, timing or delay.
fn keyed_icon(kind: TransitionKind) -> Transition {
    Transition {
        properties: vec![PropertyGroup::Icon],
        kind,
        duration: Value::Absent,
        delay: Value::Stated(Ms(0.0)),
        timing: Value::Absent,
        engine: TransitionEngine::WinUiAnimatedIcon {
            visual: key!("accept"),
        },
        from: None,
    }
}

fn transitions(items: Vec<Transition>) -> Layer {
    body(PartStylePatch {
        transitions: Some(items),
        ..PartStylePatch::default()
    })
}

/// The problems `block`'s own structural checks find (S12), walked as the
/// field `field` of `button`'s part `part`, its places spelled by TOML's
/// keys where `toml`, over `decl`: the check called on the block with no
/// widget in between, as no part lists `transitions` and no icon its
/// loop until the motion unit (D6 outcome 3, S6.1).
fn on_part<T: Walk + ?Sized>(
    block: &T,
    part: ButtonPart,
    field: &str,
    toml: bool,
    decl: &Declarations,
) -> Vec<String> {
    let name = crate::widgets::TypedPart::Button(part).name();
    let at = format!("widgets.button.{name}.{field}");
    let problems = super::checks::triggers::walked_block(
        block,
        &at,
        Some(TypedPart::Button(part)),
        toml,
        decl,
    );
    problems.iter().map(Problem::to_string).collect()
}

/// The problems of `items` as `button.body`'s transitions ([`on_part`]).
fn body_transitions(items: Vec<Transition>) -> Vec<String> {
    on_part(
        &items,
        ButtonPart::Body,
        "transitions",
        false,
        &Declarations::default(),
    )
}

/// The problems of `text`, a TOML list of transitions, read by the list's
/// own reader and checked as the transitions of `button`'s part `part`
/// over `decl` ([`on_part`]): the reader's problems where it refuses it.
fn toml_transitions(part: ButtonPart, text: &str, decl: &Declarations) -> Vec<String> {
    let name = crate::widgets::TypedPart::Button(part).name();
    let at = format!("widgets.button.{name}.transitions");
    match read_in::<Vec<Transition>>(text, Spelling::Bare, &at, Some(WidgetKind::Button)) {
        Ok(items) => on_part(&items, part, "transitions", true, decl),
        Err(problems) => problems,
    }
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
    assert_eq!(
        bare(&layer("= token.behavior.mnemonic_when_disabled")),
        NONE
    );
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
    // A transition's ranges, its own checks (S2.7, S12).
    assert_eq!(
        body_transitions(vec![
            transition(vec![PropertyGroup::Fill], spring(0.0)),
            transition(
                vec![PropertyGroup::Shadow],
                Timing::CubicBezier(1.5, 0.0, 0.5, 1.0)
            ),
            transition(
                vec![PropertyGroup::Corners],
                Timing::Steps {
                    count: 1,
                    jump: StepJump::JumpNone
                }
            ),
        ]),
        [
            "widgets.button.body.transitions[0].timing.spring.mass: out of range: above zero",
            "widgets.button.body.transitions[1].timing.cubic_bezier[0]: out of range: 0..=1",
            "widgets.button.body.transitions[2].timing.steps.count: out of range: at least two \
             steps under `jump-none`",
        ]
    );
    assert_eq!(
        body_transitions(vec![
            transition(vec![PropertyGroup::Fill], spring(1.0)),
            transition(
                vec![PropertyGroup::Stroke],
                Timing::Steps {
                    count: 1,
                    jump: StepJump::JumpEnd
                }
            ),
        ]),
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
            angle: crate::units::Deg(0.0),
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
        body_transitions(vec![transition(Vec::new(), Timing::Linear)]),
        ["widgets.button.body.transitions[0].properties: a transition of no property group"]
    );
    assert_eq!(
        body_transitions(vec![transition(vec![PropertyGroup::Fill], Timing::Linear)]),
        NONE
    );
}

/// A transition's `from` (S2.7, S12): an empty list applies from no
/// state, a flag listed twice, a keyed flag the widget does not declare.
#[test]
fn refusal_transition_from() {
    let from = |from: &str, decl: &Declarations| {
        toml_transitions(
            ButtonPart::Body,
            &format!(
                "[{{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, \
                 delay_ms = 0, timing = \"linear\", engine = \"gtk_css\", from = {from} }}]"
            ),
            decl,
        )
    };
    let none = Declarations::default();
    assert_eq!(
        from("[]", &none),
        ["widgets.button.body.transitions[0].from: a transition applying from no state"]
    );
    assert_eq!(
        from("[\"hover\", \"pressed\", \"hover\"]", &none),
        ["widgets.button.body.transitions[0].from[2]: `hover` listed twice"]
    );
    assert_eq!(
        from("[\"@spin\"]", &none),
        [
            "widgets.button.body.transitions[0].from[0]: `@spin` is not declared in `widget.button.flags`"
        ]
    );
    let declared = declare("[declare.widget.button]\nflags = [\"@spin\"]");
    assert_eq!(from("[\"@spin\", \"hover\"]", &declared), NONE);
}

/// `SurfaceRoles.border` may be absent in the `Selection` set alone, a
/// mark of that place (S6.3, the catalogue's role record; ADD-A130): in a
/// base, a rule and an entry an application layer creates.
#[test]
fn refusal_border_absent_outside_selection() {
    let decl = declare("surface_sets = [\"selection\", \"window\", \"view\"]");
    // The entries' other fields, which this layer creates unset, are the
    // entry check's, below.
    let absence = |layer: &Layer| -> Vec<String> {
        found(layer, &decl)
            .into_iter()
            .filter(|problem| !problem.contains("can create the entry"))
            .collect()
    };
    let border = |set: &str| {
        toml(&format!(
            "[colors.light.surfaces.{set}]\nborder = \"absent\""
        ))
    };
    assert_eq!(absence(&border("selection")), NONE);
    assert_eq!(
        absence(&border("window")),
        ["colors.light.surfaces.window.border: may not be absent"]
    );
    let rule = |set: &str| {
        toml(&format!(
            "[[rules]]\nwhen = {{ conditions = [\"scheme:dark\"] }}\ntarget = \"colors\"\n\
             set = {{ surfaces.{set}.border = \"absent\" }}"
        ))
    };
    assert_eq!(absence(&rule("selection")), NONE);
    assert_eq!(
        absence(&rule("view")),
        ["rules[0].set.surfaces.view.border: may not be absent"]
    );
    // An application layer creating an entry leaves `border` unset only
    // where it may be absent.
    let created = |set: &str| {
        let layer = toml(&format!(
            "[colors.light.surfaces.{set}]\nbackground = \"#000000\""
        ));
        found(&layer, &decl)
            .iter()
            .any(|problem| problem.ends_with("with `border` unset"))
    };
    assert!(!created("selection"));
    assert!(created("window"));
}

/// A transition list's one item on `button.body`, its `duration_ms`,
/// `timing` and `engine` as written.
fn timed(duration: &str, timing: &str, engine: &str) -> Layer {
    toml(&format!(
        "[widgets.button.body]\ntransitions = {}",
        timed_list(duration, timing, engine)
    ))
}

/// [`timed`]'s list of one transition, as TOML.
fn timed_list(duration: &str, timing: &str, engine: &str) -> String {
    format!(
        "[{{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = {duration}, delay_ms \
         = 0, timing = {timing}, engine = {engine} }}]"
    )
}

/// The problems of [`timed_list`]'s transition's own checks, as
/// `button.body`'s ([`toml_transitions`]).
fn timed_checked(duration: &str, timing: &str, engine: &str) -> Vec<String> {
    toml_transitions(
        ButtonPart::Body,
        &timed_list(duration, timing, engine),
        &Declarations::default(),
    )
}

/// A stated zero of `fill` on `button.body`, its `timing` and `engine` as
/// written, after a 100 ms fade it overrides (S12: a zero overriding
/// nothing is refused), so it is the list's second item.
fn zeroed(timing: &str, engine: &str) -> Layer {
    toml(&format!(
        "[widgets.button.body]\ntransitions = [{{ properties = [\"fill\"], kind = \
         \"interpolate\", duration_ms = 100, delay_ms = 0, timing = \"linear\", engine = \
         \"gtk_css\", from = [\"hover\"] }}, {{ properties = [\"fill\"], kind = \"interpolate\", \
         duration_ms = 0, delay_ms = 0, timing = {timing}, engine = {engine} }}]"
    ))
}

/// A focus indicator's transition on `button.body` stated `focus` in the
/// bases, and a rule under `hover` patching it by `rule`.
fn focus_timed(duration: &str, timing: &str, rule: &str) -> Layer {
    toml(&format!(
        "[widgets.button.body.focus]\ntransition = {{ properties = [], kind = \
         \"interpolate\", duration_ms = {duration}, delay_ms = 0, timing = {timing}, engine = \
         \"gtk_css\", from = \"none\" }}\n[[rules]]\nwhen = {{ states = [\"hover\"] }}\n\
         target = \"button.body\"\nset = {{ {rule} }}"
    ))
}

/// A widget's `appear` presentation of one step, its `kind`,
/// `duration_ms` and `timing` as written.
fn presented(kind: &str, duration: &str, timing: &str) -> Layer {
    toml(&format!(
        "[widgets.button.values]\npresentations = [{{ event = \"appear\", steps = [{{ kind = \
         {kind}, duration_ms = {duration}, delay_ms = 0, timing = {timing}, stagger_ms = 0 }}] \
         }}]"
    ))
}

/// A transition's or a presentation step's timing is `Absent` exactly
/// where its duration is a stated zero, the literal zero in the field
/// (S2.7, S6.3, S12; ADD-A125): a reference resolving to zero is none, and
/// a rule's patch of the duration is read merged with the base's timing.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn refusal_timing_absent() {
    let refusal = "a timing absent where the duration is not a stated zero";
    assert_eq!(
        bare(&timed("100", "\"absent\"", "\"gtk_css\"")),
        [format!(
            "widgets.button.body.transitions[0].timing: {refusal}"
        )]
    );
    assert_eq!(bare(&zeroed("\"absent\"", "\"gtk_css\"")), NONE);
    let quick = declare("durations = [\"quick\"]");
    let referenced = toml(&format!(
        "[tokens.motion]\ndurations.quick_ms = 0\n{}",
        "[widgets.button.body]\ntransitions = [{ properties = [\"fill\"], kind = \
         \"interpolate\", duration_ms = \"= token.motion.durations.quick\", delay_ms = 0, \
         timing = \"absent\", engine = \"gtk_css\" }]"
    ));
    assert_eq!(
        found(&referenced, &quick),
        [format!(
            "widgets.button.body.transitions[0].timing: {refusal}"
        )]
    );
    assert_eq!(
        bare(&focus_timed(
            "100",
            "\"linear\"",
            "focus.transition.timing = \"absent\""
        )),
        [format!("rules[0].set.focus.transition.timing: {refusal}")]
    );
    assert_eq!(
        bare(&focus_timed(
            "100",
            "\"linear\"",
            "focus.transition.timing = \"absent\", focus.transition.duration_ms = 0"
        )),
        NONE
    );
    assert_eq!(
        bare(&presented("\"fade\"", "100", "\"absent\"")),
        [format!(
            "widgets.button.values.presentations[0].steps[0].timing: {refusal}"
        )]
    );
}

/// A stated zero duration runs no curve: its timing is `Absent` (S2.7,
/// S12; ADD-A125), a step's alike; a reference resolving to zero keeps its
/// timing. A rule's zero is read merged with the base's timing.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn refusal_timing_on_zero() {
    let refusal = "a timing stated where the duration is a stated zero";
    assert_eq!(
        bare(&zeroed("\"linear\"", "\"gtk_css\"")),
        [format!(
            "widgets.button.body.transitions[1].timing: {refusal}"
        )]
    );
    assert_eq!(bare(&timed("100", "\"linear\"", "\"gtk_css\"")), NONE);
    let quick = declare("durations = [\"quick\"]");
    let referenced = toml(&format!(
        "[tokens.motion]\ndurations.quick_ms = 0\n{}",
        "[widgets.button.body]\ntransitions = [{ properties = [\"fill\"], kind = \
         \"interpolate\", duration_ms = \"= token.motion.durations.quick\", delay_ms = 0, \
         timing = \"linear\", engine = \"gtk_css\" }]"
    ));
    assert_eq!(found(&referenced, &quick), NONE);
    assert_eq!(
        bare(&focus_timed(
            "100",
            "\"linear\"",
            "focus.transition.duration_ms = 0"
        )),
        [format!(
            "widgets.button.body.focus.transition.timing: {refusal}"
        )]
    );
    assert_eq!(
        bare(&focus_timed(
            "100",
            "\"linear\"",
            "focus.transition = { replace = { properties = [], kind = \"interpolate\", \
             duration_ms = 0, delay_ms = 0, timing = \"absent\", engine = \"gtk_css\", from = \
             \"none\" } }"
        )),
        NONE
    );
    assert_eq!(
        bare(&presented("\"instant\"", "0", "\"linear\"")),
        [format!(
            "widgets.button.values.presentations[0].steps[0].timing: {refusal}"
        )]
    );
    assert_eq!(bare(&presented("\"instant\"", "0", "\"absent\"")), NONE);
}

/// A duration is `Absent` only where its timing, followed through
/// references, is a spring, whose time is its own (S2.7, S6.3, S12), a
/// step's alike; a transition's under `WinUiAnimatedIcon` too, which the
/// visual times.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn refusal_duration_absent() {
    let refusal = "a duration absent where the timing is no spring";
    assert_eq!(
        bare(&timed("\"absent\"", "\"linear\"", "\"gtk_css\"")),
        [format!(
            "widgets.button.body.transitions[0].duration_ms: {refusal}"
        )]
    );
    let spring = "{ spring = { mass = 1, stiffness = 100, damping_ratio = 1, initial_velocity \
                  = 0, rest_epsilon = 0.001, clamp = false, time_scale = 1 } }";
    let adw = "{ adw_animation = { replays = false } }";
    assert_eq!(bare(&timed("\"absent\"", spring, adw)), NONE);
    let bounce = declare("timings = [\"bounce\"]");
    let referenced = |timing: &str| {
        toml(&format!(
            "[tokens.motion]\ntimings.bounce = {timing}\n[widgets.button.body]\ntransitions = \
             [{{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = \"absent\", \
             delay_ms = 0, timing = \"= token.motion.timings.bounce\", engine = {adw} }}]"
        ))
    };
    assert_eq!(found(&referenced(spring), &bounce), NONE);
    assert_eq!(
        found(&referenced("\"linear\""), &bounce),
        [format!(
            "widgets.button.body.transitions[0].duration_ms: {refusal}"
        )]
    );
    assert_eq!(
        bare(&presented("\"fade\"", "\"absent\"", "\"linear\"")),
        [format!(
            "widgets.button.values.presentations[0].steps[0].duration_ms: {refusal}"
        )]
    );
    assert_eq!(bare(&presented("\"fade\"", "\"absent\"", spring)), NONE);
}

/// An `Instant` step states a zero duration and an `Absent` timing, its
/// one form (S2.7, S12; ADD-A125).
#[test]
#[ignore = "motion unit: no widget lists `presentations` until it is enabled (D6 outcome 3)"]
fn refusal_instant_not_zero() {
    assert_eq!(
        bare(&presented("\"instant\"", "100", "\"linear\"")),
        [
            "widgets.button.values.presentations[0].steps[0].duration_ms: an `instant` step whose \
          duration is not a stated zero"
        ]
    );
    assert_eq!(bare(&presented("\"instant\"", "0", "\"absent\"")), NONE);
}

/// A step of another kind than `Instant` states no zero duration: an
/// instant step is `Instant`, its one spelling (S2.7, S12; ADD-A125).
#[test]
#[ignore = "motion unit: no widget lists `presentations` until it is enabled (D6 outcome 3)"]
fn refusal_zero_not_instant() {
    assert_eq!(
        bare(&presented("\"fade\"", "0", "\"absent\"")),
        [
            "widgets.button.values.presentations[0].steps[0].duration_ms: a stated zero duration \
          on a step that is not `instant`"
        ]
    );
    assert_eq!(bare(&presented("\"fade\"", "100", "\"linear\"")), NONE);
}

/// A transition's stated timing, followed through references, is one its
/// engine's code computes (S2.7, S12); a stated zero's, which is refused,
/// is not read by the lists.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn refusal_timing_not_computed() {
    let spring = "{ spring = { mass = 1, stiffness = 100, damping_ratio = 1, initial_velocity \
                  = 0, rest_epsilon = 0.001, clamp = false, time_scale = 1 } }";
    assert_eq!(
        bare(&timed("100", spring, "\"gtk_css\"")),
        [
            "widgets.button.body.transitions[0].timing: a timing the `gtk_css` engine does not \
          compute"
        ]
    );
    assert_eq!(
        bare(&timed(
            "100",
            "{ cubic_bezier = [0.25, 0.1, 0.25, 1] }",
            "\"gtk_css\""
        )),
        NONE
    );
    // Under `CssAnimation` the curves CSS states, as under `GtkCss`.
    let css = |timing: &str| {
        toml(&format!(
            "[widgets.button.body]\ntransitions = [{{ properties = [\"fill\"], kind = {}, \
             duration_ms = 100, delay_ms = 0, timing = {timing}, engine = \"css_animation\" }}]",
            keyframes_kind(&[("0", ", patch = { fill = \"#000000\" }")])
        ))
    };
    assert_eq!(
        bare(&css(spring)),
        [
            "widgets.button.body.transitions[0].timing: a timing the `css_animation` engine does \
             not compute"
        ]
    );
    assert_eq!(bare(&css("{ cubic_bezier = [0.2, 0, 0, 1] }")), NONE);
    assert_eq!(
        bare(&zeroed(spring, "\"gtk_css\"")),
        [
            "widgets.button.body.transitions[1].timing: a timing stated where the duration is a \
          stated zero"
        ]
    );
    let curve = declare("timings = [\"curve\"]");
    let referenced = |timing: &str, engine: &str| {
        toml(&format!(
            "[tokens.motion]\ntimings.curve = {timing}\n[widgets.button.body]\ntransitions = \
             [{{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = \
             0, timing = \"= token.motion.timings.curve\", engine = {engine} }}]"
        ))
    };
    let adw = "{ platform = { adw = \"ease_out_cubic\" } }";
    assert_eq!(
        found(&referenced(adw, "\"qt_quick\""), &curve),
        [
            "widgets.button.body.transitions[0].timing: a timing the `qt_quick` engine does not \
          compute"
        ]
    );
    assert_eq!(
        found(
            &referenced(adw, "{ adw_animation = { replays = false } }"),
            &curve
        ),
        NONE
    );
    assert_eq!(
        found(
            &referenced("{ platform = \"kirigami_velocity\" }", "\"qt_quick\""),
            &curve
        ),
        NONE
    );
    assert_eq!(
        found(
            &referenced(
                "{ platform = { qt = \"out_cubic\" } }",
                "\"compose_transition\""
            ),
            &curve
        ),
        [
            "widgets.button.body.transitions[0].timing: a timing the `compose_transition` engine \
          does not compute"
        ]
    );
    // The lists of the engines the presets have not used yet: one timing
    // computed, one not, each. `GtkSwitch` and `ComposeHeld` take no part
    // of the catalogue yet: the timing's refusal shows beside that one.
    let breeze = "{ breeze = { any = [\"hover\"], none = [], conditions = [] } }";
    let ease = "{ cubic_bezier = [0.25, 0.1, 0.25, 1] }";
    let not_computed = |engine: &str| {
        format!(
            "widgets.button.body.transitions[0].timing: a timing the `{engine}` engine does not \
             compute"
        )
    };
    let place = |engine: &str, takes: &str| {
        format!("widgets.button.body.transitions[0].engine: the `{engine}` engine takes {takes}")
    };
    assert_eq!(
        bare(&timed(
            "100",
            "{ platform = { qt = \"out_cubic\" } }",
            breeze
        )),
        NONE
    );
    assert_eq!(bare(&timed("100", ease, breeze)), [not_computed("breeze")]);
    assert_eq!(
        bare(&timed(
            "100",
            "{ steps = { count = 3, jump = \"jump_end\" } }",
            "\"win_ui\""
        )),
        NONE
    );
    assert_eq!(
        bare(&timed("100", spring, "\"win_ui\"")),
        [not_computed("win_ui")]
    );
    let segment = place(
        "compose_held",
        "a segmented control's segment's `geometry` transition only",
    );
    assert_eq!(
        bare(&timed("\"absent\"", spring, "\"compose_held\"")),
        [segment.as_str()]
    );
    assert_eq!(
        bare(&timed("100", "\"linear\"", "\"compose_held\"")),
        [segment, not_computed("compose_held")]
    );
    let thumb = place(
        "gtk_switch",
        "the switch thumb's `geometry` transition only",
    );
    assert_eq!(bare(&timed("100", adw, "\"gtk_switch\"")), [thumb.as_str()]);
    assert_eq!(
        bare(&timed(
            "100",
            "{ platform = { adw = \"ease_in_cubic\" } }",
            "\"gtk_switch\""
        )),
        [thumb, not_computed("gtk_switch")]
    );
    // A tokens rule can change the referenced curve where the transition
    // applies.
    let ruled = toml(&format!(
        "[tokens.motion]\ntimings.curve = \"linear\"\n[[rules]]\nwhen = {{ conditions = \
         [\"reduced_motion\"] }}\ntarget = \"tokens\"\nset = {{ motion.timings.curve = {spring} \
         }}\n[widgets.button.body]\ntransitions = [{{ properties = [\"fill\"], kind = \
         \"interpolate\", duration_ms = 100, delay_ms = 0, timing = \
         \"= token.motion.timings.curve\", engine = \"gtk_css\" }}]"
    ));
    assert_eq!(
        found(&ruled, &curve),
        [
            "widgets.button.body.transitions[0].timing: a timing the `gtk_css` engine does not \
          compute"
        ]
    );
}

/// A transition its engine does not run (S2.7, S12; ADD-A126):
/// `WinUiAnimatedIcon` takes a `Keyed` transition with no duration, timing
/// or delay; a Breeze engine an `Interpolate` one with a duration and no
/// delay; `GtkSwitch` the switch thumb's `Geometry` transition and
/// `ComposeHeld` a segmented control's segment's, none of the catalogue's
/// parts yet; and no value engine a `Keyed` one. The transition's own
/// checks, called on the list as `button.body`'s: no part lists
/// `transitions` until the motion unit (D6 outcome 3); the visual's
/// source, staged with it, is no structural check's.
#[test]
fn refusal_engine_mismatch() {
    let kinds = declare("transition_kinds = { normal_off = {} }");
    let checked = |list: String| toml_transitions(ButtonPart::Body, &list, &kinds);
    let keyed = "{ keyed = { key = \"normal_off\", params = {} } }";
    let icon = |kind: &str, duration: &str, delay: &str, timing: &str| {
        checked(format!(
            "[{{ properties = [\"icon\"], kind = {kind}, duration_ms = {duration}, delay_ms = \
             {delay}, timing = {timing}, engine = {{ win_ui_animated_icon = {{ visual = \
             \"accept\" }} }} }}]"
        ))
    };
    let at = "widgets.button.body.transitions[0]";
    let icon_takes = "the `win_ui_animated_icon` engine takes";
    assert_eq!(icon(keyed, "\"absent\"", "0", "\"absent\""), NONE);
    assert_eq!(
        icon(keyed, "100", "10", "\"linear\""),
        [
            format!("{at}.duration_ms: {icon_takes} no duration: its visual times it"),
            format!("{at}.timing: {icon_takes} no timing: its visual times it"),
            format!("{at}.delay_ms: {icon_takes} no delay"),
        ]
    );
    assert_eq!(
        icon("\"interpolate\"", "\"absent\"", "0", "\"absent\""),
        [format!("{at}.kind: {icon_takes} a `keyed` transition only")]
    );
    let breeze = |kind: &str, duration: &str, delay: &str| {
        checked(format!(
            "[{{ properties = [\"fill\"], kind = {kind}, duration_ms = {duration}, delay_ms = \
             {delay}, timing = \"linear\", engine = {{ breeze = {{ any = [\"hover\"], none = [], \
             conditions = [] }} }} }}]"
        ))
    };
    assert_eq!(breeze("\"interpolate\"", "100", "0"), NONE);
    let breeze_takes = "the `breeze` engine takes";
    assert_eq!(
        breeze(keyed, "\"absent\"", "10"),
        [
            format!("{at}.duration_ms: {breeze_takes} a duration, on which its time runs"),
            format!("{at}.delay_ms: {breeze_takes} no delay"),
            format!("{at}.kind: {breeze_takes} an `interpolate` transition only"),
        ]
    );
    assert_eq!(
        timed_checked(
            "100",
            "{ platform = { adw = \"ease_out_cubic\" } }",
            "\"gtk_switch\""
        ),
        [format!(
            "{at}.engine: the `gtk_switch` engine takes the switch thumb's `geometry` \
             transition only"
        )]
    );
    let spring = "{ spring = { mass = 1, stiffness = 100, damping_ratio = 1, initial_velocity \
                  = 0, rest_epsilon = 0.001, clamp = false, time_scale = 1 } }";
    assert_eq!(
        timed_checked("\"absent\"", spring, "\"compose_held\""),
        [format!(
            "{at}.engine: the `compose_held` engine takes a segmented control's segment's \
             `geometry` transition only"
        )]
    );
    assert_eq!(
        checked(format!(
            "[{{ properties = [\"icon\"], kind = {keyed}, duration_ms = 100, delay_ms = 0, timing \
             = \"linear\", engine = \"gtk_css\" }}]"
        )),
        [format!(
            "{at}.kind: the `gtk_css` engine takes no `keyed` transition"
        )]
    );
    let one = |kind: &str, duration: &str, delay: &str, timing: &str, engine: &str| {
        checked(format!(
            "[{{ properties = [\"fill\"], kind = {kind}, duration_ms = {duration}, delay_ms = \
             {delay}, timing = {timing}, engine = {engine} }}]"
        ))
    };
    // `GtkSwitch` and `ComposeHeld` take no part of the catalogue yet: the
    // checks of their other fields show beside that refusal.
    let adw = "{ platform = { adw = \"ease_out_cubic\" } }";
    let switch_takes = "the `gtk_switch` engine takes";
    let thumb =
        format!("{at}.engine: {switch_takes} the switch thumb's `geometry` transition only");
    assert_eq!(
        one("\"interpolate\"", "100", "0", adw, "\"gtk_switch\""),
        [thumb.as_str()]
    );
    assert_eq!(
        one(keyed, "100", "10", adw, "\"gtk_switch\""),
        [
            thumb,
            format!("{at}.delay_ms: {switch_takes} no delay"),
            format!("{at}.kind: {switch_takes} an `interpolate` transition only"),
        ]
    );
    let segment = format!(
        "{at}.engine: the `compose_held` engine takes a segmented control's segment's \
         `geometry` transition only"
    );
    assert_eq!(
        one(
            "\"interpolate\"",
            "\"absent\"",
            "0",
            spring,
            "\"compose_held\""
        ),
        [segment.as_str()]
    );
    assert_eq!(
        one(keyed, "\"absent\"", "0", spring, "\"compose_held\""),
        [
            segment,
            format!("{at}.kind: the `compose_held` engine takes no `keyed` transition"),
        ]
    );
    assert_eq!(
        one("\"interpolate\"", "100", "0", "\"linear\"", "\"win_ui\""),
        NONE
    );
    assert_eq!(
        one(keyed, "100", "0", "\"linear\"", "\"win_ui\""),
        [format!(
            "{at}.kind: the `win_ui` engine takes no `keyed` transition"
        )]
    );
    let tab_bar = "{ breeze_tab_bar = { any = [\"hover\"], none = [], conditions = [] } }";
    assert_eq!(
        one("\"interpolate\"", "100", "0", "\"linear\"", tab_bar),
        NONE
    );
    let tab_bar_takes = "the `breeze_tab_bar` engine takes";
    assert_eq!(
        one(keyed, "\"absent\"", "10", "\"linear\"", tab_bar),
        [
            format!("{at}.duration_ms: {tab_bar_takes} a duration, on which its time runs"),
            format!("{at}.delay_ms: {tab_bar_takes} no delay"),
            format!("{at}.kind: {tab_bar_takes} an `interpolate` transition only"),
        ]
    );
    // `CssAnimation` runs a `Keyframes` transition, and only it (ADD-A134).
    let frames = keyframes_kind(&[("0", ", patch = { fill = \"#000000\" }")]);
    let css = "\"css_animation\"";
    assert_eq!(one(&frames, "100", "0", "\"linear\"", css), NONE);
    assert_eq!(
        one("\"interpolate\"", "100", "0", "\"linear\"", css),
        [format!(
            "{at}.kind: the `css_animation` engine takes a `keyframes` transition only"
        )]
    );
    assert_eq!(
        one(&frames, "100", "0", "\"linear\"", "\"gtk_css\""),
        [format!(
            "{at}.kind: the `gtk_css` engine takes no `keyframes` transition"
        )]
    );
}

/// A Breeze test holds somewhere and reads what the engine can (S2.7,
/// S12; ADD-A126): a flag in `any` or a condition on a transition that is
/// no stated zero, every list empty on a stated zero, no flag twice or in
/// both `any` and `none`, and the window's conditions only. The
/// transition's own checks, called on the list as `button.body`'s: no part
/// lists `transitions` until the motion unit (D6 outcome 3). A focus
/// indicator's `focus_visible` in `any` is its own transition's rule,
/// staged with that unit (S6.1).
#[test]
fn refusal_breeze_test_invalid() {
    let fade = "{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = \
                0, timing = \"linear\", engine = { breeze = { any = [\"pressed\"], none = [], \
                conditions = [] } } }";
    let none = Declarations::default();
    let breeze = |duration: &str, timing: &str, test: &str| {
        toml_transitions(
            ButtonPart::Body,
            &format!(
                "[{fade}, {{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = \
                 {duration}, delay_ms = 0, timing = {timing}, engine = {{ breeze = {{ {test} }} \
                 }} }}]"
            ),
            &none,
        )
    };
    let at = "widgets.button.body.transitions[1].engine.breeze";
    let empty = "any = [], none = [], conditions = []";
    assert_eq!(
        breeze("100", "\"linear\"", empty),
        [format!(
            "{at}: a Breeze test names a flag in `any` or a condition: it holds nowhere"
        )]
    );
    assert_eq!(
        breeze(
            "100",
            "\"linear\"",
            "any = [], none = [], conditions = [\"window_inactive\"]"
        ),
        NONE
    );
    assert_eq!(breeze("0", "\"absent\"", empty), NONE);
    for (list, test) in [
        ("any", "any = [\"hover\"], none = [], conditions = []"),
        ("none", "any = [], none = [\"hover\"], conditions = []"),
        (
            "conditions",
            "any = [], none = [], conditions = [\"window_inactive\"]",
        ),
    ] {
        assert_eq!(
            breeze("0", "\"absent\"", test),
            [format!(
                "{at}.{list}: a Breeze test of a stated zero is empty: a zero reads none"
            )]
        );
    }
    let tab_bar = toml_transitions(
        ButtonPart::Body,
        &format!(
            "[{fade}, {{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, \
             delay_ms = 0, timing = \"linear\", engine = {{ breeze_tab_bar = {{ {empty} }} }} }}]"
        ),
        &none,
    );
    assert_eq!(
        tab_bar,
        [
            "widgets.button.body.transitions[1].engine.breeze_tab_bar: a Breeze test names a flag \
          in `any` or a condition: it holds nowhere"
        ]
    );
    assert_eq!(
        breeze(
            "100",
            "\"linear\"",
            "any = [\"hover\", \"hover\"], none = [\"hover\"], conditions = [\"reduced_motion\"]"
        ),
        [
            format!("{at}.any[1]: `hover` listed twice"),
            format!("{at}.none[0]: a Breeze test names no flag in both `any` and `none`"),
            format!("{at}.conditions[0]: a Breeze test names the window's conditions only"),
        ]
    );
}

/// A delay is read by its value (S2.7, S12): one written as a reference
/// is refused where it resolves to another than zero, a literal where it
/// is stated, a list's item's and a focus indicator's transition's alike.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn delay_read_by_its_resolved_value() {
    let quick = declare("durations = [\"quick\"]");
    let delay = "\"= token.motion.durations.quick\"";
    let breeze = "engine = { breeze = { any = [\"focus_visible\"], none = [], conditions = [] } }";
    let item = |token: &str| {
        toml(&format!(
            "[tokens.motion]\ndurations.quick_ms = {token}\n[widgets.button.body]\n\
             transitions = [{{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = \
             100, delay_ms = {delay}, timing = \"linear\", {breeze} }}]"
        ))
    };
    assert_eq!(found(&item("0"), &quick), NONE);
    assert_eq!(
        found(&item("50"), &quick),
        ["widgets.button.body.transitions[0].delay_ms: the `breeze` engine takes no delay"]
    );
    let focus = |token: &str| {
        toml(&format!(
            "[tokens.motion]\ndurations.quick_ms = {token}\n[widgets.button.body.focus]\n\
             transition = {{ properties = [], kind = \"interpolate\", duration_ms = 100, \
             delay_ms = {delay}, timing = \"linear\", {breeze}, from = \"none\" }}"
        ))
    };
    assert_eq!(found(&focus("0"), &quick), NONE);
    assert_eq!(
        found(&focus("50"), &quick),
        ["widgets.button.body.focus.transition.delay_ms: the `breeze` engine takes no delay"]
    );
}

/// The problems of a `css_animation` transition of `kind` timing `groups`
/// on the button's body, synthetic, read by the list's own reader and
/// checked by the transition's own checks.
fn css_transition(groups: &str, kind: &str) -> Vec<Problem> {
    let at = "widgets.button.body.transitions";
    let list = format!(
        "[{{ properties = [{groups}], kind = {kind}, duration_ms = 100, delay_ms = 0, timing = \
         \"linear\", engine = \"css_animation\" }}]"
    );
    match read_in::<Vec<Transition>>(&list, Spelling::Bare, at, Some(WidgetKind::Button)) {
        Ok(items) => super::checks::triggers::walked_block(
            &items,
            at,
            Some(TypedPart::Button(ButtonPart::Body)),
            true,
            &Declarations::default(),
        ),
        Err(problems) => problems
            .into_iter()
            .map(|problem| Problem::new(at, crate::error::ProblemKind::Toml(problem)))
            .collect(),
    }
}

/// [`css_transition`]'s problems, as `path: kind` lines: the transition's
/// own checks, called on the list as `button.body`'s, as no part lists
/// `transitions` until the motion unit (D6 outcome 3).
fn css_checked(groups: &str, kind: &str) -> Vec<String> {
    css_transition(groups, kind)
        .iter()
        .map(Problem::to_string)
        .collect()
}

/// `Keyframes` with `keyframes`, each `(at, its line's rest)`.
fn keyframes_kind(keyframes: &[(&str, &str)]) -> String {
    let keyframes: Vec<String> = keyframes
        .iter()
        .map(|(at, rest)| format!("{{ at = {at}, timing = \"absent\"{rest} }}"))
        .collect();
    format!(
        "{{ keyframes = {{ keyframes = [{}] }} }}",
        keyframes.join(", ")
    )
}

/// A `Keyframes` transition has a keyframe (S2.7, S12; ADD-A134).
#[test]
fn refusal_no_keyframe() {
    let at = "widgets.button.body.transitions[0].kind.keyframes.keyframes";
    assert_eq!(
        css_checked("\"opacity\"", &keyframes_kind(&[])),
        [format!("{at}: a `keyframes` transition with no keyframe")]
    );
    let one = keyframes_kind(&[("0", ", patch = { opacity = 0 }")]);
    assert_eq!(css_checked("\"opacity\"", &one), NONE);
}

/// A `Keyframes` transition's keyframes name no part, their `at` within
/// `0..=1` and increasing (S2.7, S12; ADD-A134).
#[test]
fn refusal_keyframe_names_part() {
    let at = "widgets.button.body.transitions[0].kind.keyframes.keyframes";
    let named = keyframes_kind(&[("0", ", part = \"body\", patch = { opacity = 0 }")]);
    assert_eq!(
        css_checked("\"opacity\"", &named),
        [format!(
            "{at}[0].part: a `keyframes` transition's keyframe names a part"
        )]
    );
    let repeated = keyframes_kind(&[
        ("0.25", ", patch = { opacity = 0 }"),
        ("0.25", ", patch = { opacity = 1 }"),
    ]);
    assert_eq!(
        css_checked("\"opacity\"", &repeated),
        [format!("{at}: the keyframes' `at` do not increase")]
    );
    let beyond = keyframes_kind(&[("1.5", ", patch = { opacity = 0 }")]);
    assert_eq!(
        css_checked("\"opacity\"", &beyond),
        [format!("{at}[0].at: out of range: 0..=1")]
    );
}

/// A `Keyframes` transition's keyframes state only the fields its groups
/// time (S2.7, S12; ADD-A134). A focus indicator's, which lists none and
/// states `focus`'s fields alone, is its own transition's, staged with the
/// motion unit (S6.1).
#[test]
fn refusal_keyframe_outside_groups() {
    let at = "widgets.button.body.transitions[0].kind.keyframes.keyframes";
    let outside = "a field the keyframes' transition does not time";
    let opacity = keyframes_kind(&[("0", ", patch = { opacity = 0 }")]);
    assert_eq!(css_checked("\"opacity\"", &opacity), NONE);
    assert_eq!(
        css_checked("\"fill\"", &opacity),
        [format!("{at}[0].patch.opacity: {outside}")]
    );
    // `StrokeWidth` times the edges' widths alone, `Stroke` the whole stroke.
    let width = keyframes_kind(&[("0", ", patch = { stroke.top.width_px = 0 }")]);
    assert_eq!(css_checked("\"stroke_width\"", &width), NONE);
    assert_eq!(css_checked("\"stroke\"", &width), NONE);
    let paint = keyframes_kind(&[("0", ", patch = { stroke.top.paint = \"#000000\" }")]);
    assert_eq!(
        css_checked("\"stroke_width\"", &paint),
        [format!("{at}[0].patch.stroke.top.paint: {outside}")]
    );
    let padding = keyframes_kind(&[("0", ", patch = { padding.top_px = 0 }")]);
    assert_eq!(css_checked("\"geometry\"", &padding), NONE);
}

/// A `FocusMove` transition is refused: no engine runs one yet, as an
/// engine runs per element part and the move crosses two (S2.7, S2.10,
/// S12; ADD-A126). The transition's own check, called on the list as
/// `button.body`'s: no part lists `transitions` until the motion unit (D6
/// outcome 3).
#[test]
fn refusal_focus_move_not_run() {
    let kind = |kind: &str| {
        toml_transitions(
            ButtonPart::Body,
            &format!(
                "[{{ properties = [\"fill\"], kind = \"{kind}\", duration_ms = 100, delay_ms = 0, \
                 timing = \"linear\", engine = \"gtk_css\" }}]"
            ),
            &Declarations::default(),
        )
    };
    assert_eq!(
        kind("focus_move"),
        [
            "widgets.button.body.transitions[0].kind: a `focus_move` transition, which no engine \
          runs yet"
        ]
    );
    assert_eq!(kind("interpolate"), NONE);
}

/// A `GtkCss` part's transitions that can apply together name one curve,
/// followed through references, but on `Shadow` and `Corners`, the groups
/// GTK's engine takes a curve for by property id (S2.7, S12; ADD-A131).
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn refusal_several_curves() {
    let css = |groups: &str, timing: &str| {
        format!(
            "{{ properties = [{groups}], kind = \"interpolate\", duration_ms = 100, delay_ms = \
             0, timing = {timing}, engine = \"gtk_css\" }}"
        )
    };
    let list = |items: &[String]| {
        toml(&format!(
            "[widgets.button.body]\ntransitions = [{}]",
            items.join(", ")
        ))
    };
    let ease = "{ cubic_bezier = [0.25, 0.1, 0.25, 1] }";
    let refusal = "a `gtk_css` style's transitions name several curves on a group other than \
                   `shadow` and `corners`";
    assert_eq!(
        bare(&list(&[
            css("\"fill\"", "\"linear\""),
            css("\"shadow\"", ease)
        ])),
        [format!(
            "widgets.button.body.transitions[0].timing: {refusal}"
        )]
    );
    assert_eq!(
        bare(&list(&[
            css("\"shadow\"", "\"linear\""),
            css("\"corners\"", ease)
        ])),
        NONE
    );
    assert_eq!(
        bare(&list(&[css("\"fill\"", ease), css("\"stroke\"", ease)])),
        NONE
    );
    // One curve through references; another a rule adds where it applies.
    let curve = declare("timings = [\"ease\"]");
    let referenced = format!(
        "[tokens.motion]\ntimings.ease = {ease}\n[widgets.button.body]\ntransitions = [{}, {}]\n\
         [[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.body\"\nset = {{ \
         transitions = [{}] }}",
        css("\"fill\"", "\"= token.motion.timings.ease\""),
        css("\"stroke\"", ease),
        css("\"opacity\"", "\"linear\""),
    );
    assert_eq!(
        found(&toml(&referenced), &curve),
        [
            format!("widgets.button.body.transitions[0].timing: {refusal}"),
            format!("widgets.button.body.transitions[1].timing: {refusal}"),
            format!("rules[0].set.transitions[0].timing: {refusal}"),
        ]
    );
    // The focus indicator's transition is one of the node's CSS list
    // (libadwaita's `transition: $focus_transition, $transition;`).
    let focused = |timing: &str| {
        toml(&format!(
            "[widgets.button.body]\ntransitions = [{}]\n[widgets.button.body.focus]\n\
             transition = {{ properties = [], kind = \"interpolate\", duration_ms = 100, \
             delay_ms = 0, timing = {timing}, engine = \"gtk_css\", from = \"none\" }}",
            css("\"fill\"", ease)
        ))
    };
    assert_eq!(
        bare(&focused("\"linear\"")),
        [format!(
            "widgets.button.body.transitions[0].timing: {refusal}"
        )]
    );
    assert_eq!(bare(&focused(ease)), NONE);
}

/// A stated zero with no delay overrides a transition of one of its groups
/// that can apply before it, earlier in its list, in the base or an
/// earlier rule of its part; where none can, no transition is stated
/// (S2.7, S12; ADD-A126). A focus indicator's zero overrides the
/// indicator's transition below it.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn refusal_overrides_nothing() {
    let item = |groups: &str, duration: &str, delay: &str, timing: &str| {
        format!(
            "{{ properties = [{groups}], kind = \"interpolate\", duration_ms = {duration}, \
             delay_ms = {delay}, timing = {timing}, engine = \"gtk_css\" }}"
        )
    };
    let zero = |groups: &str| item(groups, "0", "0", "\"absent\"");
    let fade = |groups: &str| item(groups, "100", "0", "\"linear\"");
    let refusal = "a stated zero with no delay that overrides no transition";
    let base = |items: &[String], rules: &str| {
        toml(&format!(
            "[widgets.button.body]\ntransitions = [{}]\n{rules}",
            items.join(", ")
        ))
    };
    assert_eq!(
        bare(&base(&[zero("\"fill\"")], "")),
        [format!(
            "widgets.button.body.transitions[0].duration_ms: {refusal}"
        )]
    );
    assert_eq!(
        bare(&base(&[item("\"fill\"", "0", "50", "\"absent\"")], "")),
        NONE
    );
    assert_eq!(bare(&base(&[fade("\"fill\""), zero("\"fill\"")], "")), NONE);
    assert_eq!(
        bare(&base(&[fade("\"stroke\""), zero("\"stroke_width\"")], "")),
        NONE
    );
    let disabled = |groups: &str| {
        format!(
            "[[rules]]\nwhen = {{ states = [\"disabled\"] }}\ntarget = \"button.body\"\nset = \
             {{ transitions = [{}] }}",
            zero(groups)
        )
    };
    assert_eq!(
        bare(&base(&[fade("\"fill\"")], &disabled("\"fill\""))),
        NONE
    );
    assert_eq!(
        bare(&base(&[fade("\"fill\"")], &disabled("\"stroke\""))),
        [format!(
            "rules[0].set.transitions[0].duration_ms: {refusal}"
        )]
    );
    // A rule's transition the zero's rule cannot apply beside.
    let hovered = format!(
        "[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.body\"\nset = {{ \
         transitions = [{}] }}\n[[rules]]\nwhen = {{ not_states = [\"hover\"] }}\ntarget = \
         \"button.body\"\nset = {{ transitions = [{}] }}",
        fade("\"opacity\""),
        zero("\"opacity\"")
    );
    assert_eq!(
        bare(&toml(&hovered)),
        [format!(
            "rules[1].set.transitions[0].duration_ms: {refusal}"
        )]
    );
    let focus = |rules: &str| {
        toml(&format!(
            "[widgets.button.body.focus]\ntransition = {{ properties = [], kind = \
             \"interpolate\", duration_ms = 0, delay_ms = 0, timing = \"absent\", engine = \
             \"gtk_css\", from = \"none\" }}\n{rules}"
        ))
    };
    assert_eq!(
        bare(&focus("")),
        [format!(
            "widgets.button.body.focus.transition.duration_ms: {refusal}"
        )]
    );
    assert_eq!(
        bare(&focus(
            "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\nset = { \
             focus.transition.delay_ms = 10, focus.transition.duration_ms = 0 }"
        )),
        [format!(
            "widgets.button.body.focus.transition.duration_ms: {refusal}"
        )]
    );
    let over = toml(
        "[widgets.button.body.focus]\ntransition = { properties = [], kind = \"interpolate\", \
         duration_ms = 100, delay_ms = 0, timing = \"linear\", engine = \"gtk_css\", from = \
         \"none\" }\n[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\nset \
         = { focus.transition.duration_ms = 0, focus.transition.timing = \"absent\" }",
    );
    assert_eq!(bare(&over), NONE);
}

/// A stated zero's delay is read by its value (S2.7, S12): one written as a
/// reference that resolves to zero is no delay, so the zero is refused
/// where it overrides nothing; one resolving to another than zero holds
/// its start. A list's item's and a focus indicator's transition's alike.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn overrides_nothing_reads_the_delay_by_its_value() {
    let quick = declare("durations = [\"quick\"]");
    let delay = "\"= token.motion.durations.quick\"";
    let refusal = "a stated zero with no delay that overrides no transition";
    let item = |token: &str| {
        toml(&format!(
            "[tokens.motion]\ndurations.quick_ms = {token}\n[widgets.button.body]\n\
             transitions = [{{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = \
             0, delay_ms = {delay}, timing = \"absent\", engine = \"gtk_css\" }}]"
        ))
    };
    assert_eq!(
        found(&item("0"), &quick),
        [format!(
            "widgets.button.body.transitions[0].duration_ms: {refusal}"
        )]
    );
    assert_eq!(found(&item("50"), &quick), NONE);
    let focus = |token: &str| {
        toml(&format!(
            "[tokens.motion]\ndurations.quick_ms = {token}\n[widgets.button.body.focus]\n\
             transition = {{ properties = [], kind = \"interpolate\", duration_ms = 0, \
             delay_ms = {delay}, timing = \"absent\", engine = \"gtk_css\", from = \"none\" }}"
        ))
    };
    assert_eq!(
        found(&focus("0"), &quick),
        [format!(
            "widgets.button.body.focus.transition.duration_ms: {refusal}"
        )]
    );
    assert_eq!(found(&focus("50"), &quick), NONE);
}

/// A stated zero's delay and what it overrides are read in one world
/// (S2.7, S12): a delay resolving to zero under a world where no earlier
/// transition applies refuses the zero there, though the delay is another
/// elsewhere and an earlier transition applies elsewhere; where the earlier
/// one applies in every world of a zero delay, it is accepted.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn overrides_nothing_reads_delay_and_override_per_world() {
    let quick = declare("durations = [\"quick\"]");
    let refusal = "a stated zero with no delay that overrides no transition";
    let layer = |earlier: &str| {
        toml(&format!(
            "[tokens.motion]\ndurations.quick_ms = 50\n[widgets.button.body]\ntransitions = \
             []\n[[rules]]\nwhen = {{ conditions = [\"scheme:dark\"] }}\ntarget = \"tokens\"\n\
             set = {{ motion.durations.quick_ms = 0 }}\n[[rules]]\nwhen = {{ conditions = \
             [\"scheme:{earlier}\"] }}\ntarget = \"button.body\"\nset = {{ transitions = [{{ \
             properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = 0, \
             timing = \"linear\", engine = \"gtk_css\" }}] }}\n[[rules]]\nwhen = {{ states = \
             [\"hover\"] }}\ntarget = \"button.body\"\nset = {{ transitions = [{{ properties = \
             [\"fill\"], kind = \"interpolate\", duration_ms = 0, delay_ms = \"= \
             token.motion.durations.quick\", timing = \"absent\", engine = \"gtk_css\" }}] }}"
        ))
    };
    assert_eq!(
        found(&layer("light"), &quick),
        [format!(
            "rules[2].set.transitions[0].duration_ms: {refusal}"
        )]
    );
    assert_eq!(found(&layer("dark"), &quick), NONE);
    let focus = |earlier: &str| {
        toml(&format!(
            "[tokens.motion]\ndurations.quick_ms = 50\n[[rules]]\nwhen = {{ conditions = \
             [\"scheme:dark\"] }}\ntarget = \"tokens\"\nset = {{ motion.durations.quick_ms = 0 \
             }}\n[[rules]]\nwhen = {{ conditions = [\"scheme:{earlier}\"] }}\ntarget = \
             \"button.body\"\nset = {{ focus.transition = {{ properties = [], kind = \
             \"interpolate\", duration_ms = 100, delay_ms = 0, timing = \"linear\", engine = \
             \"gtk_css\", from = \"none\" }} }}\n[[rules]]\nwhen = {{ states = [\"hover\"] }}\n\
             target = \"button.body\"\nset = {{ focus.transition = {{ properties = [], kind = \
             \"interpolate\", duration_ms = 0, delay_ms = \"= token.motion.durations.quick\", \
             timing = \"absent\", engine = \"gtk_css\", from = \"none\" }} }}"
        ))
    };
    assert_eq!(
        found(&focus("light"), &quick),
        [format!(
            "rules[2].set.focus.transition.duration_ms: {refusal}"
        )]
    );
    assert_eq!(found(&focus("dark"), &quick), NONE);
}

/// A Breeze test's flags and conditions are read as a `when` writes them
/// and walked as a rule's are: a keyed flag the widget declares, a keyed
/// condition the preset declares (S2.7, S10, S11, S12; ADD-A126). Read by
/// the list's own reader and checked by the transition's own checks, as
/// `button.body`'s: no part lists `transitions` until the motion unit (D6
/// outcome 3).
#[test]
fn breeze_test_reads_and_walks() {
    let breeze = |test: &str| {
        format!(
            "[{{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = \
             0, timing = \"linear\", engine = {{ breeze = {{ {test} }} }} }}]"
        )
    };
    let none = Declarations::default();
    let at = "widgets.button.body.transitions[0].engine.breeze";
    assert_eq!(
        toml_transitions(
            ButtonPart::Body,
            &breeze("any = [\"hover\"], none = [], conditions = [\"shiny\"]"),
            &none
        ),
        [format!("{at}.conditions[0]: unknown name \"shiny\"")]
    );
    let keyed = breeze(
        "any = [\"@spin\"], none = [\"disabled\"], conditions = [\"system_setting:glow\", \
         \"window_inactive\"]",
    );
    // A system setting is no window's condition, which a Breeze test names
    // alone (S12): declared, it is that one problem.
    let window = format!("{at}.conditions[0]: a Breeze test names the window's conditions only");
    assert_eq!(
        toml_transitions(ButtonPart::Body, &keyed, &none),
        [
            window.clone(),
            format!("{at}.any[0]: `@spin` is not declared in `widget.button.flags`"),
            format!("{at}.conditions[0]: `glow` is not declared in `conditions`"),
        ]
    );
    let declared =
        declare("conditions = [\"@glow\"]\n[declare.widget.button]\nflags = [\"@spin\"]");
    assert_eq!(
        toml_transitions(ButtonPart::Body, &keyed, &declared),
        [window]
    );
    let tab_bar = breeze("any = [\"hover\"], none = [\"@spin\"], conditions = []")
        .replace("{ breeze =", "{ breeze_tab_bar =");
    assert_eq!(
        toml_transitions(ButtonPart::Body, &tab_bar, &none),
        [
            "widgets.button.body.transitions[0].engine.breeze_tab_bar.none[0]: `@spin` is not declared \
          in `widget.button.flags`"
        ]
    );
}

/// A `StateLayer` transition times the part's `interaction`, so a part
/// without it may not state one (S2.7, S2.12, S12). The transition's own
/// check, called on the list as the part's: no part lists `transitions`
/// until the motion unit (D6 outcome 3).
#[test]
fn refusal_state_layer_without_interaction() {
    let on = |part: ButtonPart| {
        toml_transitions(
            part,
            "[{ properties = [\"state_layer\"], kind = \"interpolate\", duration_ms = 15, \
             delay_ms = 0, timing = \"linear\", engine = \"gtk_css\" }]",
            &Declarations::default(),
        )
    };
    assert_eq!(
        on(ButtonPart::Icon),
        [
            "widgets.button.icon.transitions[0].properties: a `state_layer` transition on a part without `interaction`"
        ]
    );
    assert_eq!(on(ButtonPart::Body), NONE);
}

/// A ripple's indication box, `button.body`'s, as a TOML table.
const RIPPLE: &str = "[widgets.button.body.interaction]\n\
                      state_layer = { color = \"#000000\", alpha = 0.1 }\n\
                      ripple = { effect = \"android_patterned\", color = \"#000000\", \
                      alpha = 0.1, bounded = true, max_radius_px = \"absent\", padding_px = 0, \
                      indication_box = INDICATION }";

/// [`RIPPLE`] with its indication box `indication`.
fn ripple(indication: &str) -> Layer {
    toml(&RIPPLE.replace("INDICATION", indication))
}

/// A layer whose bases give `button.body`'s ripple the indication box of
/// `part`, built in Rust.
fn indication_on(part: PartRef) -> Layer {
    body(PartStylePatch {
        interaction: Some(BlockPatch::Fields(InteractionPatch {
            ripple: Some(RippleStylePatch {
                indication_box: Some(Some(IndicationBox::new(part, Value::Stated(Px(0.0))))),
                ..RippleStylePatch::default()
            }),
            ..InteractionPatch::default()
        })),
        ..PartStylePatch::default()
    })
}

/// A keyed component's part, another widget than `button`.
fn component_part() -> PartRef {
    PartRef::Component {
        key: key!("probe"),
        part: key!("thumb"),
    }
}

/// An indication box names a part of the ripple's own widget (S2.12, S12):
/// another widget's part is refused. `button` is the one typed widget, so
/// the other is a keyed component's part, built in Rust as no TOML names
/// one while no unit enables a component.
#[test]
fn refusal_indication_box_other_widget() {
    let indication = indication_on;
    assert_eq!(
        bare(&indication(component_part())),
        [
            "widgets.button.body.interaction.ripple.indication_box.part: the indication's box is \
             `component.probe.thumb`, another widget's part"
        ]
    );
    let own = PartRef::Typed(TypedPart::Button(ButtonPart::Icon));
    assert_eq!(bare(&indication(own)), NONE);
    assert_eq!(
        bare(&ripple("{ part = \"button.icon\", outset_px = 2 }")),
        NONE
    );
    assert_eq!(bare(&ripple("{ part = \"icon\", outset_px = 2 }")), NONE);
    assert_eq!(bare(&ripple("\"none\"")), NONE);
}

/// An indication box's `outset` is never negative (S2.12, S12).
#[test]
fn refusal_indication_box_outset_negative() {
    assert_eq!(
        bare(&ripple("{ part = \"icon\", outset_px = -1 }")),
        [
            "widgets.button.body.interaction.ripple.indication_box.outset_px: out of range: at least zero"
        ]
    );
    assert_eq!(bare(&ripple("{ part = \"icon\", outset_px = 0 }")), NONE);
}

/// An indication box names no part that can be `Absent` where the ripple
/// can be drawn (S2.12, S12): the part made absent under a state while the
/// ripple's effect is stated is refused; where another box is in effect
/// there, or the ripple has no effect, it is not.
#[test]
fn refusal_indication_box_part_absent() {
    let icon_box = ripple("{ part = \"icon\", outset_px = 2 }");
    let no_effect = toml(
        "[widgets.button.body.interaction]\n\
         state_layer = { color = \"#000000\", alpha = 0.1 }\n\
         ripple = { effect = \"absent\", color = \"absent\", alpha = \"absent\", \
         bounded = true, max_radius_px = \"absent\", padding_px = \"absent\", \
         indication_box = { part = \"icon\", outset_px = 2 } }",
    );
    let rule = |target: &str, set: &str| {
        toml(&format!(
            "[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.{target}\"\n\
             set = {set}"
        ))
    };
    let icon_absent = rule("icon", "{ absent = true }");
    assert_eq!(
        found_over(&icon_absent, &Declarations::default(), &icon_box),
        [
            "widgets.button.body.interaction.ripple.indication_box.part: the indication's box \
             is `icon`, which can be absent where the ripple is drawn"
        ]
    );
    assert_eq!(
        found_over(&icon_absent, &Declarations::default(), &no_effect),
        NONE
    );
    let mut other_box = rule(
        "body",
        "{ interaction.ripple.indication_box = { part = \"body\", outset_px = 0 } }",
    );
    other_box.rules.extend(icon_absent.rules.iter().cloned());
    assert_eq!(
        found_over(&other_box, &Declarations::default(), &icon_box),
        NONE
    );
}

/// A loop's keyframes' `at` increase (S2.7, S12): the loop's own check,
/// called on the block as `button.icon`'s icon animation, which is staged
/// with the motion unit (S6.1).
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
        on_part(
            &animation,
            ButtonPart::Icon,
            "icon.animation",
            false,
            &Declarations::default(),
        )
    };
    assert_eq!(
        animated([0.5, 0.5]),
        ["widgets.button.icon.icon.animation.keyframes: the keyframes' `at` do not increase"]
    );
    assert_eq!(animated([0.0, 0.5]), NONE);
}

/// A keyframe's patch sets only fields its part's catalogue entry lists
/// (S2.7, S2.11): the part it names, or the part the loop is on where it
/// names none; `shown` on no part that does not list it. The loop's own
/// check, called on the block read by its own reader as `button.icon`'s
/// icon animation, which is staged with the motion unit (S6.1).
#[test]
fn refusal_keyframe_field_its_part_does_not_list() {
    let at = "widgets.button.icon.icon.animation";
    let looped = |part: &str, patch: &str| {
        let text = format!(
            "{{ period_ms = 100, timing = \"linear\", keyframes = [{{ at = 0, {part}timing = \
             \"absent\", patch = {{ {patch} }} }}] }}"
        );
        match read_in::<Loop>(&text, Spelling::Bare, at, Some(WidgetKind::Button)) {
            Ok(looped) => on_part(
                &looped,
                ButtonPart::Icon,
                "icon.animation",
                true,
                &Declarations::default(),
            ),
            Err(problems) => problems,
        }
    };
    let at = format!("{at}.keyframes[0].patch");
    assert_eq!(
        looped("", "shown = false"),
        [format!("{at}.shown: a field `button.icon` does not have")]
    );
    assert_eq!(
        looped("part = \"label\", ", "opacity = 0.5"),
        [format!(
            "{at}.opacity: a field `button.label` does not have"
        )]
    );
    assert_eq!(looped("part = \"body\", ", "opacity = 0.5"), NONE);
    assert_eq!(looped("", "glyph = \"absent\""), NONE);
}

/// A `KirigamiSmoothstep` shadow layer has no spread and is never inset
/// (S2.5, S12; ADD-A134): a spread other than a stated zero, an expression
/// included, and `inset = true` are refused, a part's layer and an
/// elevation entry's alike; a `Gaussian` layer takes both.
#[test]
fn refusal_smoothstep_spread_or_inset() {
    let layer = |spread: &str, inset: bool, falloff: &str| {
        format!(
            "{{ x_px = 0, y_px = 1, blur_px = 8, spread_px = {spread}, color = \"#00000040\", \
             inset = {inset}, falloff = \"{falloff}\" }}"
        )
    };
    let shadow = |layer: &str| {
        toml(&format!(
            "[widgets.button.body]\nshadow = {{ layers = [{layer}] }}"
        ))
    };
    let refused = "a `kirigami_smoothstep` shadow layer's spread or inset";
    let at = "widgets.button.body.shadow.layers[0]";
    let smoothstep = "kirigami_smoothstep";
    assert_eq!(bare(&shadow(&layer("0", false, smoothstep))), NONE);
    assert_eq!(
        bare(&shadow(&layer("2", false, smoothstep))),
        [format!("{at}.spread_px: {refused}")]
    );
    assert_eq!(
        bare(&shadow(&layer(
            "\"= token.params.layout.gap\"",
            false,
            smoothstep
        )))
        .iter()
        .filter(|problem| problem.ends_with(refused))
        .collect::<Vec<_>>(),
        [&format!("{at}.spread_px: {refused}")]
    );
    assert_eq!(
        bare(&shadow(&layer("0", true, smoothstep))),
        [format!("{at}.inset: {refused}")]
    );
    assert_eq!(
        bare(&shadow(&layer("1", true, smoothstep))),
        [
            format!("{at}.spread_px: {refused}"),
            format!("{at}.inset: {refused}")
        ]
    );
    assert_eq!(bare(&shadow(&layer("2", true, "gaussian"))), NONE);
    let elevation = toml(&format!(
        "[tokens.elevation]\nraised = [{}]",
        layer("0", true, smoothstep)
    ));
    assert_eq!(
        found(&elevation, &declare("elevation = [\"raised\"]")),
        [format!("tokens.elevation.raised[0].inset: {refused}")]
    );
}

/// A shadow layer's `spread` may be negative (S2.5, S12; ADD-A135), as
/// CSS's spread distance may: a part's layer and an elevation entry's take
/// it, its blur still at least zero.
#[test]
fn shadow_spread_may_be_negative() {
    let layer = |blur: &str, spread: &str| {
        format!(
            "{{ x_px = 0, y_px = 4, blur_px = {blur}, spread_px = {spread}, \
             color = \"#00000040\", inset = false, falloff = \"gaussian\" }}"
        )
    };
    let shadow = |layer: &str| {
        toml(&format!(
            "[widgets.button.body]\nshadow = {{ layers = [{layer}] }}"
        ))
    };
    assert_eq!(bare(&shadow(&layer("16", "-3"))), NONE);
    assert_eq!(
        bare(&shadow(&layer("-1", "-3"))),
        ["widgets.button.body.shadow.layers[0].blur_px: out of range: at least zero"]
    );
    let elevation = toml(&format!(
        "[tokens.elevation]\nwindow = [{}]",
        layer("16", "-3")
    ));
    assert_eq!(
        found(&elevation, &declare("elevation = [\"window\"]")),
        NONE
    );
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

/// `round_pt` is stated only in an expression on a `Font.size`, on a
/// reference to a font size (S2.2, S12).
#[test]
fn refusal_round_pt_misused() {
    let label = |key: &str, expr: &str| {
        bare(&toml(&format!(
            "[widgets.button.label]\n{key} = \"= {expr}\""
        )))
    };
    let misused = |path: &str| {
        vec![format!(
            "{path}: `round_pt` takes a reference to a font size, in an expression on a \
             `Font.size`"
        )]
    };
    assert_eq!(
        label(
            "text.font.size",
            "mul(round_pt(token.text.roles.body.font.size), 1.2)"
        ),
        NONE
    );
    assert_eq!(
        label(
            "padding.top_px",
            "round_pt(token.text.roles.body.font.size)"
        ),
        misused("widgets.button.label.padding.top_px")
    );
    assert_eq!(
        label("text.font.size", "round_pt(10pt)"),
        misused("widgets.button.label.text.font.size")
    );
    assert_eq!(
        label(
            "text.font.size",
            "round_pt(add(token.text.roles.body.font.size, 1px))"
        ),
        misused("widgets.button.label.text.font.size")
    );
    assert_eq!(
        label("text.font.size", "round_pt(token.text.font_dpi)"),
        misused("widgets.button.label.text.font.size")
    );
    let style = toml(
        "[tokens.text.roles.caption]\nfont.size = \"= round_pt(token.text.roles.body.font.size)\"",
    );
    assert_eq!(bare(&style), NONE);
}

#[test]
fn refusal_route_duplicated() {
    let axis = |tag: &str| {
        let mut layer = Layer::default();
        layer.bases.widgets.button = Some(Box::new(ButtonPatch {
            label: Some(PartPatch::of_style(PartStylePatch {
                text: Some(BlockPatch::Fields(TextStylePatch {
                    font: Some(FontPatch {
                        axes: Some(Value::Stated(vec![(
                            AxisTag::new(tag).unwrap(),
                            AxisValue::Fixed(1.0),
                        )])),
                        ..FontPatch::default()
                    }),
                    ..TextStylePatch::default()
                })),
                ..PartStylePatch::default()
            })),
            ..ButtonPatch::default()
        }));
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
    // The near miss types: `kde_accent` is implemented (S2.2).
    assert_eq!(
        found(
            &accent("= kde_accent(window, background_normal, inactive)"),
            &kde_accent_decl()
        ),
        NONE
    );
}

/// The window set and the inputs `kde_accent(window, background_normal, …)`,
/// `kde_accent(header, background_normal, …)` and
/// `kde_accent_wm(active_background)` read themselves (S2.2).
fn kde_accent_decl() -> Declarations {
    declare(
        "surface_sets = [\"window\"]\ninputs = [\"accent\", \
         \"kde_scheme.window.background_normal\", \
         \"kde_scheme.window.inactive.background_normal\", \
         \"kde_scheme.header.background_normal\", \
         \"kde_scheme.header.inactive.background_normal\", \
         \"kde_scheme.view.background_normal\", \"kde_scheme.wm.active_background\"]",
    )
}

#[test]
fn kde_accent_set_is_declared() {
    // One of KDE's sets, declared by the preset (S2.2, S12).
    let decl = kde_accent_decl();
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
        NONE
    );
}

/// The inputs a KDE accent call reads itself are its operands, each
/// declared (S2.2, S12), each reported once; KDE's three lists are S9.1's
/// own, declared by none.
#[test]
fn kde_accent_reads_declared_inputs() {
    let decl = declare("surface_sets = [\"window\"]");
    let undeclared =
        |name: &str| format!("colors.light.global.accent: `{name}` is not declared in `inputs`");
    assert_eq!(
        found(
            &accent("= kde_accent(window, background_normal, inactive)"),
            &decl
        ),
        [
            undeclared("accent"),
            undeclared("kde_scheme.window.background_normal"),
            undeclared("kde_scheme.window.inactive.background_normal"),
            undeclared("kde_scheme.view.background_normal"),
        ]
    );
    assert_eq!(
        found(&accent("= kde_accent_wm(active_background)"), &decl),
        [
            undeclared("accent"),
            undeclared("kde_scheme.wm.active_background"),
            undeclared("kde_scheme.window.background_normal"),
        ]
    );
    assert_eq!(
        found(
            &accent("= kde_accent_wm(active_background)"),
            &kde_accent_decl()
        ),
        NONE
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

/// A reference to a `Value<Option<T>>` field is an option of `T`'s type
/// (S2.2, S6.1): a part's text takes its style's `scaling_role` and
/// `max_lines` by reference, and an option is refused where another type
/// is, as another type is where the option is; a literal or a call on an
/// option is refused, its expression a reference only.
#[test]
fn scaling_role_reference_types() {
    let label = |set: &str| toml(&format!("[widgets.button.label.text]\n{set}"));
    for set in [
        "scaling_role = \"= token.text.roles.body.scaling_role\"",
        "max_lines = \"= token.text.roles.caption.max_lines\"",
        "case = \"= token.text.roles.body.case\"",
        "elision = \"= token.text.roles.body.elision\"",
        "wrap = \"= token.text.roles.body.wrap\"",
    ] {
        assert_eq!(bare(&label(set)), NONE, "{set}");
    }
    assert_eq!(
        bare(&label(
            "scaling_role = \"= token.text.roles.body.max_lines\""
        )),
        [
            "widgets.button.label.text.scaling_role: the expression does not type: an \
             option<number> where an option<enum<TextRole>> is expected"
        ]
    );
    assert_eq!(
        bare(&label("case = \"= token.text.roles.body.scaling_role\"")),
        [
            "widgets.button.label.text.case: the expression does not type: an \
             option<enum<TextRole>> where an enum<TextCase> is expected"
        ]
    );
    assert_eq!(
        bare(&label("max_lines = \"= token.motion.speed_factor\"")),
        [
            "widgets.button.label.text.max_lines: the expression does not type: a number \
             where an option<number> is expected"
        ]
    );
    assert_eq!(
        bare(&label("max_lines = \"= 2\"")),
        [
            "widgets.button.label.text.max_lines: the expression does not type: an \
             option<number> field's expression is a reference only"
        ]
    );
    assert_eq!(
        bare(&label("max_lines = \"= max(1, 2)\"")),
        [
            "widgets.button.label.text.max_lines: the expression does not type: an option<number> field's expression is a \
         reference only"
        ]
    );
}

/// A reference to a `Value` of a map is that map's type (S2.2, S6.1): a
/// part's text takes its style's `font.axes` and `font.features` by
/// reference; one map is refused where the other is, as any other type is;
/// a literal or a call on a map is refused, its expression a reference
/// only.
#[test]
fn font_axes_reference_types() {
    let label = |set: &str| toml(&format!("[widgets.button.label.text]\n{set}"));
    for set in [
        "font.axes = \"= token.text.roles.body.font.axes\"",
        "font.features = \"= token.text.roles.caption.font.features\"",
        "font.axes = {}\nfont.features = {}",
    ] {
        assert_eq!(bare(&label(set)), NONE, "{set}");
    }
    assert_eq!(
        bare(&label(
            "font.axes = \"= token.text.roles.body.font.features\""
        )),
        [
            "widgets.button.label.text.font.axes: the expression does not type: a \
             map<FeatureTag, u32> where a map<AxisTag, AxisValue> is expected"
        ]
    );
    assert_eq!(
        bare(&label("font.features = \"= token.motion.speed_factor\"")),
        [
            "widgets.button.label.text.font.features: the expression does not type: a number \
             where a map<FeatureTag, u32> is expected"
        ]
    );
    assert_eq!(
        bare(&label("font.axes = \"= 2\"")),
        [
            "widgets.button.label.text.font.axes: the expression does not type: a \
             map<AxisTag, AxisValue> field's expression is a reference only"
        ]
    );
    assert_eq!(
        bare(&label("font.features = \"= max(1, 2)\"")),
        [
            "widgets.button.label.text.font.features: the expression does not type: a map<FeatureTag, u32> field's expression is a \
         reference only"
        ]
    );
}

#[test]
fn refusal_too_deep() {
    let nested = |calls: usize| {
        let inner = Expr::Lit(crate::expr::Literal::Number(1.0));
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
    rule.rules.push(crate::selector::Rule::new(
        crate::selector::Selector::default(),
        crate::selector::RuleBody::Part(
            crate::selector::PartRef::Typed(crate::widgets::TypedPart::Button(
                crate::widgets::ButtonPart::Body,
            )),
            PartPatch::of_style(append),
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
        crate::InputName::new("accent").unwrap(),
        crate::InputValue::new(crate::SchemeMap::one(
            crate::Scheme::Light,
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

/// `present_with_nothing_beneath_is_missing` (S6.3, S12, ADD-A126): under
/// each combination in which `absent = false` draws a part again, every
/// field of it no patch states is missing, the fields beside the absence
/// stated beneath it; a patch of fields alone draws nothing, so nothing is
/// missing; a preset's own absent part needs no fields, those beside its
/// absence kept for a later `absent = false`.
#[test]
fn present_with_nothing_beneath_is_missing() {
    let decl = Declarations::default();
    let redraw = |set: &str| {
        toml(&format!(
            "[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.menu_indicator\"\n\
             set = {set}"
        ))
    };
    let missing = |fields: &[&str]| -> Vec<String> {
        fields
            .iter()
            .map(|field| format!("widgets.button.menu_indicator.{field}: missing"))
            .collect()
    };
    let bare = toml("[widgets.button.menu_indicator]\nabsent = true");
    assert_eq!(
        found_over(&redraw("{ absent = false }"), &decl, &bare),
        missing(&["glyph", "min_width", "min_height", "margin", "transform"])
    );
    assert_eq!(
        found_over(&redraw("{ min_width_px = 4 }"), &decl, &bare),
        NONE
    );
    let beneath = toml("[widgets.button.menu_indicator]\nabsent = true\nmin_width_px = 4");
    assert_eq!(
        found_over(
            &redraw("{ absent = false, min_height_px = 4 }"),
            &decl,
            &beneath
        ),
        missing(&["glyph", "margin", "transform"])
    );
    // Rule A states one leaf of the glyph, rule B redraws the part with
    // every other field: the glyph's other leaves are missing.
    let mut leaves = redraw("{ glyph = { size_px = 4 } }");
    leaves.rules.extend(
        redraw(
            "{ absent = false, min_width_px = 4, min_height_px = 4, margin = { top_px = 0, \
             right_px = 0, bottom_px = 0 } }",
        )
        .rules,
    );
    assert_eq!(
        found_over(&leaves, &decl, &bare),
        missing(&[
            "glyph.source",
            "glyph.fill",
            "glyph.stroke",
            "glyph.axes",
            "glyph.draw_extent",
            "glyph.draw_start",
            "glyph.mirroring",
            "margin.left",
            "transform"
        ])
    );
    let file = read_preset(&format!(
        "{PRESET_HEAD}[widgets.button.menu_indicator]\nabsent = true\nmin_width_px = 4\n\
         margin = {{ top_px = 4 }}\nglyph = {{ size_px = 4 }}"
    ))
    .unwrap();
    let own = strings(check_preset(
        &file.declarations,
        &file.layer,
        None,
        &[],
        None,
        &Reused::default(),
    ));
    assert!(
        own.iter()
            .any(|problem| problem.contains("widgets.button.body"))
    );
    assert!(
        !own.iter().any(|problem| problem.contains("menu_indicator")),
        "{own:?}"
    );
}

/// The problems of a preset stating `body` after its header.
fn preset(body: &str) -> Vec<String> {
    let file = read_preset(&format!("{PRESET_HEAD}{body}")).unwrap();
    strings(check_preset(
        &file.declarations,
        &file.layer,
        None,
        &[],
        None,
        &Reused::default(),
    ))
    .into_iter()
    .filter(|problem| !problem.ends_with(": missing"))
    .collect()
}

/// The problems of a preset stating `body` after its header on the
/// synthetic route, which waives a pending default size (S17, ADD-A131).
fn synthetic_preset(body: &str) -> Vec<String> {
    let file = read_preset(&format!("{PRESET_HEAD}{body}")).unwrap();
    strings(check_preset(
        &file.declarations,
        &file.layer,
        None,
        &[],
        Some(&[]),
        &Reused::default(),
    ))
    .into_iter()
    .filter(|problem| !problem.ends_with(": missing"))
    .collect()
}

/// A default size is one of the sizes it goes with: the widget's, a
/// variant entry's, and the one in force in a context (S10, S12).
#[test]
fn refusal_size_not_taken() {
    let widget = |default: &str| {
        preset(&format!(
            "[declare.widget.button]\nsizes = [\"small\", \"regular\"]\ndefault_size = \"{default}\""
        ))
    };
    assert_eq!(
        widget("large"),
        [
            "declare.widget.button.default_size: the default size `large` is not one of the sizes \
          taken there"
        ]
    );
    assert_eq!(widget("regular"), NONE);
    assert_eq!(
        preset("[declare.widget.button]\ndefault_size = \"regular\""),
        [
            "declare.widget.button.default_size: the default size `regular` is not one of the sizes \
          taken there"
        ]
    );
    let entry = |default: &str| {
        preset(&format!(
            "[declare.widget.button]\nvariants = [\"primary\"]\nsizes = [\"small\", \"regular\"]\n\
             default_size = \"regular\"\nsizes_by_variant = [{{ variant = \"primary\", sizes = \
             [\"small\", \"large\"], default_size = \"{default}\" }}]"
        ))
    };
    assert_eq!(
        entry("regular"),
        [
            "declare.widget.button.sizes_by_variant[0].default_size: the default size `regular` is \
          not one of the sizes taken there"
        ]
    );
    assert_eq!(entry("large"), NONE);
    // In a context: the widget's default left out there, and the context's
    // own default outside the sizes a variant keeps there.
    let context = |entry: &str| {
        preset(&format!(
            "[declare.widget.button]\nvariants = [\"primary\"]\nsizes = [\"small\", \"regular\"]\n\
             default_size = \"regular\"\nsizes_by_variant = [{{ variant = \"primary\", sizes = \
             [\"large\", \"small\"], default_size = \"large\" }}]\nsizes_by_context = [{entry}]"
        ))
    };
    assert_eq!(
        context("{ context = { widget = \"button\", part = \"body\" }, sizes = [\"small\"] }"),
        [
            "declare.widget.button.sizes_by_context[0]: the default size `regular` is not one of \
             the sizes taken there",
            "declare.widget.button.sizes_by_context[0]: the default size `large` is not one of \
             the sizes taken there",
        ]
    );
    assert_eq!(
        context(
            "{ context = { widget = \"button\", part = \"body\" }, sizes = [\"small\"], \
             default_size = \"small\" }"
        ),
        NONE
    );
}

/// `default_size` is stated wherever sizes are taken, with them under each
/// flavour, and in a variant entry that leaves the widget's out (S10,
/// S12).
#[test]
fn refusal_default_size_missing() {
    let missing = |path: &str| format!("{path}: no default size where sizes are taken");
    assert_eq!(
        preset("[declare.widget.button]\nsizes = [\"small\", \"regular\"]"),
        [missing("declare.widget.button.default_size")]
    );
    assert_eq!(preset("[declare.widget.button]\nsizes = []"), NONE);
    let entry = |sizes: &str| {
        preset(&format!(
            "[declare.widget.button]\nvariants = [\"primary\"]\nsizes = [\"regular\"]\n\
             default_size = \"regular\"\nsizes_by_variant = [{{ variant = \"primary\", sizes = \
             {sizes} }}]"
        ))
    };
    assert_eq!(
        entry("[\"small\"]"),
        [missing(
            "declare.widget.button.sizes_by_variant[0].default_size"
        )]
    );
    assert_eq!(entry("[\"small\", \"regular\"]"), NONE);
    assert_eq!(entry("[]"), NONE);
    // A variant's entry where the widget takes no size states its own.
    assert_eq!(
        preset(
            "[declare.widget.button]\nvariants = [\"primary\"]\nsizes_by_variant = [{ variant = \
             \"primary\", sizes = [\"small\"] }]"
        ),
        [missing(
            "declare.widget.button.sizes_by_variant[0].default_size"
        )]
    );
    // A flavour's section states `default_size` with its `sizes`: the
    // widget's default goes with the widget's sizes.
    let flavored = |section: &str| {
        preset(&format!(
            "[declare]\nflavors = [\"widgets\", \"kirigami\"]\n[declare.widget.button]\n\
             sizes = [\"small\", \"regular\"]\ndefault_size = \"small\"\n\
             [declare.flavor.kirigami.widget.button]\n{section}"
        ))
        .into_iter()
        .filter(|problem| !problem.starts_with("declare.flavors:"))
        .collect::<Vec<_>>()
    };
    assert_eq!(
        flavored("sizes = [\"regular\"]"),
        [missing(
            "declare.flavor.kirigami.widget.button.default_size"
        )]
    );
    assert_eq!(
        flavored("sizes = [\"regular\"]\ndefault_size = \"regular\""),
        NONE
    );
    assert_eq!(flavored("sizes = []"), NONE);
    assert_eq!(
        flavored("default_size = \"large\""),
        [
            "declare.flavor.kirigami.widget.button.default_size: the default size `large` is not \
          one of the sizes taken there"
        ]
    );
}

/// A pending default size, listed in the preset's `[pending]` table, is
/// the missing default size on every route but the synthetic one, which
/// waives the checks that read it (S10's *A pending default*, S17).
#[test]
fn pending_default_size_refused_but_on_the_synthetic_route() {
    let body = "[declare.widget.button]\nvariants = [\"primary\"]\nsizes = [\"small\", \"regular\"]\n\
                sizes_by_variant = [{ variant = \"primary\", sizes = [\"small\", \"large\"] }]\n\
                sizes_by_context = [{ context = { widget = \"button\", part = \"body\" }, sizes = \
                [\"small\"] }]\n\
                [pending]\n\"declare.widget.button.default_size\" = \"U1\"\n\
                \"declare.widget.button.sizes_by_variant.primary.default_size\" = \"U2\"\n\
                \"declare.widget.button.sizes_by_context.0.default_size\" = \"U3\"";
    let missing = |path: &str| format!("{path}: no default size where sizes are taken");
    assert_eq!(
        preset(body),
        [
            missing("declare.widget.button.default_size"),
            missing("declare.widget.button.sizes_by_variant[0].default_size"),
            missing("declare.widget.button.sizes_by_context[0].default_size"),
        ]
    );
    assert_eq!(synthetic_preset(body), NONE);
    // The waiver covers the pending default alone: a stated default the
    // synthetic route checks as any.
    let stated = body.replace(
        "sizes = [\"small\", \"large\"] }",
        "sizes = [\"small\", \"large\"], default_size = \"regular\" }",
    );
    assert_eq!(
        synthetic_preset(&stated),
        [
            "declare.widget.button.sizes_by_variant[0].default_size: the default size `regular` is \
          not one of the sizes taken there"
        ]
    );
    // A widget stating no sizes: its pending default, which a variant
    // entry's sizes fall back on, is missing all the same.
    let sizeless = "[declare.widget.button]\nvariants = [\"primary\"]\n\
                   sizes_by_variant = [{ variant = \"primary\", sizes = [\"small\"] }]\n\
                   [pending]\n\"declare.widget.button.default_size\" = \"U1\"";
    assert_eq!(
        preset(sizeless),
        [missing("declare.widget.button.default_size")]
    );
    assert_eq!(synthetic_preset(sizeless), NONE);
}

/// A size only a `sizes_by_context` entry names is taken by no element,
/// as a context entry only leaves sizes out: a rule selecting it selects an
/// undeclared size; a variant entry's size is declared (S10, ADD-A131).
#[test]
fn size_only_a_context_names_is_undeclared() {
    let decl = declare(
        "[declare.widget.button]\nvariants = [\"primary\"]\nsizes = [\"regular\"]\n\
         default_size = \"regular\"\nsizes_by_variant = [{ variant = \"primary\", sizes = \
         [\"small\"], default_size = \"small\" }]\nsizes_by_context = [{ context = { widget = \
         \"button\", part = \"body\" }, sizes = [\"regular\", \"large\"] }]",
    );
    let rule = |size: &str| {
        toml(&format!(
            "[[rules]]\nwhen = {{ sizes = [\"{size}\"] }}\ntarget = \"button.body\"\n\
             set = {{ opacity = 0.5 }}"
        ))
    };
    assert_eq!(
        found(&rule("large"), &decl),
        ["rules[0].when.sizes[0]: `large` is not declared in `widget.button.sizes`"]
    );
    assert_eq!(found(&rule("small"), &decl), NONE);
}

/// An element taking sizes outside a context and none inside it is
/// refused, for each variant entry and for none (S10, S12).
#[test]
fn refusal_no_size_in_context() {
    let context = |kept: &str| {
        preset(&format!(
            "[declare.widget.button]\nvariants = [\"primary\"]\nsizes = [\"small\", \"regular\"]\n\
             default_size = \"regular\"\nsizes_by_variant = [{{ variant = \"primary\", sizes = \
             [\"large\"], default_size = \"large\" }}]\nsizes_by_context = [{{ context = {{ widget = \
             \"button\", part = \"body\" }}, sizes = {kept}, default_size = \"regular\" }}]"
        ))
    };
    assert_eq!(
        context("[\"regular\"]"),
        [
            "declare.widget.button.sizes_by_context[0]: an element holding `primary` takes sizes \
          outside the context and none in it"
        ]
    );
    assert_eq!(
        context("[\"large\"]"),
        [
            "declare.widget.button.sizes_by_context[0]: an element holding no variant of an entry \
             takes sizes outside the context and none in it",
            "declare.widget.button.sizes_by_context[0].default_size: the default size `regular` is \
             not one of the sizes taken there",
        ]
    );
}

/// A `sizes_by_context` entry's selector states only `scope`, `widget`,
/// `part`, `variants` and `not_variants` (S10, S12).
#[test]
fn refusal_size_context_key() {
    let selector = |keys: &str| {
        preset(&format!(
            "[declare.widget.button]\nsizes = [\"regular\"]\ndefault_size = \"regular\"\n\
             sizes_by_context = [{{ context = {{ widget = \"button\", {keys} }}, sizes = \
             [\"regular\"] }}]"
        ))
    };
    let refused = |key: &str| {
        format!(
            "declare.widget.button.sizes_by_context[0].context.{key}: a `sizes_by_context` \
             selector may not state `{key}`"
        )
    };
    assert_eq!(
        selector("states = [\"hover\"], not_sizes = [\"small\"], surface = \"window\""),
        [refused("states"), refused("not_sizes"), refused("surface")]
    );
    // A key is refused as stated, an empty list's too.
    assert_eq!(
        selector("states = [], sizes = []"),
        [refused("states"), refused("sizes")]
    );
    assert_eq!(
        selector("scope = \"any\", part = \"body\", variants = [\"primary\"]"),
        NONE
    );
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
            crate::Key::new(group).unwrap(),
            vec![(crate::Key::new(key).unwrap(), Value::Absent)],
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
/// leaves out is the widget's own (S10). The flavour a rule names is the
/// default, the one no later unit states (S12; ADD-A147).
#[test]
fn flavour_sections_inherit_the_widgets_keys() {
    let entry = "[declare]\nflavors = [\"kirigami\", \"widgets\"]\n[declare.widget.button]\n\
                 variants = [\"@fab\"]\nsizes = [\"regular\"]\ndefault_size = \"regular\"\n\
                 [declare.flavor.kirigami.widget.button]\n\
                 sizes_by_variant = [{ variant = \"@fab\", sizes = [\"regular\"], default_size = \
                \"regular\" }]";
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
            "rules[1].when.variants[0]: `@fab` is not declared in \
             `flavor.kirigami.widget.button.variants`",
            "rules[0].when.variants[0]: `@pill` is not declared in \
             `flavor.widgets.widget.button.variants`",
            "declare.flavor.kirigami.widget.button.sizes_by_variant[0].variant: `@fab` is not \
             declared in `declare.flavor.kirigami.widget.button.variants`"
        ]
    );
}

/// Task 11's review: a flavour's section replacing the flags, the variants
/// or the sizes keeps the widget's own recency sets and default sizes,
/// which are checked against what it replaces (S10).
#[test]
fn flavour_sections_check_the_keys_they_keep() {
    let entry = "[declare]\nflavors = [\"widgets\", \"kirigami\"]\n[declare.widget.button]\n\
                 variants = [\"@fab\"]\nsizes = [\"small\", \"regular\"]\nflags = [\"@armed\"]\n\
                 recency = { lift = [\"@armed\", \"hover\"] }\ndefault_size = \"regular\"\n\
                 sizes_by_variant = [{ variant = \"@fab\", sizes = [\"small\", \"regular\"], \
                 default_size = \"small\" }]\n\
                 [declare.flavor.kirigami.widget.button]\n";
    let rules = "\n[[rules]]\nwhen = { variants = [\"@fab\"], latest = { lift = \"@armed\" } }\n\
                 target = \"button.body\"\nset = { opacity = 0.5 }";
    let declared = |body: &str| {
        preset(&format!("{entry}{body}{rules}"))
            .into_iter()
            .filter(|problem| {
                problem.starts_with("declare.widget.") || problem.starts_with("declare.flavor.")
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(declared("available = true"), NONE);
    assert_eq!(
        declared(
            "flags = []\nvariants = [\"@pill\"]\nsizes = [\"regular\"]\ndefault_size = \"regular\""
        ),
        [
            "declare.flavor.kirigami.widget.button.variants: `@fab` is not declared in \
             `declare.flavor.kirigami.widget.button.variants`",
            "declare.flavor.kirigami.widget.button.flags: `@armed` is not declared in \
             `declare.flavor.kirigami.widget.button.flags`",
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
    let gap = |scalar: crate::units::Scalar| {
        let mut layer = Layer::default();
        layer.bases.tokens.params = Some(vec![(
            key!("layout"),
            vec![(key!("gap"), Value::Stated(scalar))],
        )]);
        layer
    };
    assert_eq!(
        found(&gap(crate::units::Scalar::Ms(Ms(4.0))), &decl),
        ["tokens.params.layout.gap: declared in `px`"]
    );
    assert_eq!(found(&gap(crate::units::Scalar::Px(Px(4.0))), &decl), NONE);
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

/// A declared flavour but the default is a flavour of a unit not enabled
/// while the flavours unit is not: exempt from the unused check, and a
/// rule naming it refused (S12; ADD-A147).
#[test]
fn refusal_unused() {
    let flavors = |rules: &str| {
        preset(&format!(
            "[declare]\nflavors = [\"widgets\", \"kirigami\"]\n{rules}"
        ))
    };
    assert_eq!(flavors(""), NONE);
    assert_eq!(
        flavors(
            "[[rules]]\nwhen = { conditions = [\"flavor:kirigami\"] }\ntarget = \"tokens\"\nset = {}"
        ),
        [
            "rules[0].when.conditions[0]: `kirigami` is declared in `flavors`, and a later unit \
             states it"
        ]
    );
}

/// Device models no rule selects on, and keyed icon contexts and status
/// colours no base states, are unused (S12); a declared variant no rule
/// selects on is not (ADD-A128: it draws as the base, and the facts gate's
/// declarations run checks it).
#[test]
fn refusal_unused_variants_models_and_maps() {
    let rule = "[[rules]]\nwhen = { variants = [\"@pill\"], conditions = \
                [\"device_model:phone_x\"] }\ntarget = \"button.body\"\nset = { opacity = 0.5 }";
    let head = "[declare]\ndevice_models = [\"phone_x\"]\nicon_contexts = [\"menu\"]\n\
                symbolic_colors = [\"error\"]\n[declare.widget.button]\nvariants = [\"@pill\", \
                \"@with_arrow\", \"@tall\", \"@fab\"]\nrequires_variants = [\"@tall\"]\n\
                sizes_by_variant = [{ variant = \"@fab\", sizes = [\"regular\"], default_size = \
                \"regular\" }]";
    assert_eq!(
        preset(head),
        [
            "declare.device_models: `phone_x` is declared but unused",
            "declare.symbolic_colors: `error` is declared but unused",
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
    layer.bases.tokens.icons = Some(Box::new(crate::tokens::IconTokensPatch {
        sizes: Some(crate::tokens::IconSizeTokensPatch {
            keyed: Some(vec![(key!("menu"), BlockPatch::Fields(size))]),
            ..crate::tokens::IconSizeTokensPatch::default()
        }),
        ..crate::tokens::IconTokensPatch::default()
    }));
    let error = PaintPatch::Set(Value::Stated(Paint {
        layers: vec![layer_of(BlendMode::Normal)],
    }));
    layer.bases.widgets.button = Some(Box::new(ButtonPatch {
        icon: Some(PartPatch::of_style(PartStylePatch {
            icon: Some(BlockPatch::Fields(IconStylePatch {
                symbolic_colors: Some(vec![(key!("error"), error)]),
                ..IconStylePatch::default()
            })),
            ..PartStylePatch::default()
        })),
        ..ButtonPatch::default()
    }));
    let problems: Vec<String> = strings(check_preset(
        &file.declarations,
        &layer,
        None,
        &[],
        None,
        &Reused::default(),
    ))
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
                    .map(|key| (crate::Key::new(key).unwrap(), Value::Stated(color())))
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
        layer.bases.widgets.button = Some(Box::new(ButtonPatch {
            label: Some(PartPatch::of_style(style)),
            ..ButtonPatch::default()
        }));
        layer
    };
    assert_eq!(
        bare(&label(fill(vec![layer_of(BlendMode::Normal)]))),
        ["widgets.button.label.fill: a field `button.label` does not have"]
    );
    assert_eq!(
        bare(&label(PartStylePatch {
            padding: Some(crate::units::Sides {
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
fn refusal_missing() -> Result<(), LayerError> {
    // The preset's bases, the bottom of the stack, state every field: here
    // the tokens' text block states only its scaling and its keyed styles.
    let file = read_preset(&format!(
        "{PRESET_HEAD}[tokens.text]\nscaling = \"linear\"\nkeyed = {{}}"
    ))?;
    let found = strings(check_preset(
        &file.declarations,
        &file.layer,
        None,
        &[],
        None,
        &Reused::default(),
    ));
    assert!(
        found.contains(&"tokens.text.font_dpi: missing".to_owned()),
        "{found:?}"
    );
    assert!(
        !found.iter().any(|p| p.starts_with("tokens.text.keyed")),
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
        }),
        ..TokensPatch::default()
    });
    assert_eq!(bare(&motion), NONE);
    Ok(())
}

#[test]
fn accent_ramp_entries_are_declared() {
    let ramp = |key: &str| {
        let mut layer = Layer::default();
        layer.bases.colors.light = Some(ColorRolesPatch {
            global: Some(GlobalRolesPatch {
                accent_ramp: Some(vec![(
                    crate::Key::new(key).unwrap(),
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
    // A `DrawnAs`'s target: the model's field, TOML's key.
    let unavailable = declare("[declare.widget.button]\navailable = false");
    let mut drawn = Layer::default();
    drawn.bases.widgets.button = Some(Box::new(ButtonPatch {
        icon: Some(PartPatch::of_presence(
            crate::widgets::PartPresence::DrawnAs {
                target: crate::selector::WidgetOrPart::Part(crate::selector::PartRef::Typed(
                    crate::widgets::TypedPart::Button(crate::widgets::ButtonPart::Body),
                )),
                variants: Vec::new(),
            },
        )),
        ..ButtonPatch::default()
    }));
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
    let mut selector = crate::selector::Selector::default();
    selector
        .conditions
        .none_of
        .push(crate::selector::Condition::Flavor(key!("kirigami")));
    let mut appended = Layer::default();
    appended.rules.push(crate::selector::Rule::new(
        selector,
        crate::selector::RuleBody::Part(
            crate::selector::PartRef::Typed(crate::widgets::TypedPart::Button(
                crate::widgets::ButtonPart::Body,
            )),
            PartPatch::of_style(PartStylePatch {
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
    // A transition's own checks, called on the list as `button.body`'s.
    assert_eq!(
        body_transitions(vec![
            transition(vec![PropertyGroup::Fill], spring),
            transition(
                vec![PropertyGroup::Opacity],
                Timing::CubicBezier(f32::NAN, 0.0, 0.5, 1.0)
            ),
        ]),
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
            Value::Stated(crate::units::Scalar::Number(f32::NAN)),
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

/// A keyed text style's and a slot's drawn-text leaves, `fill`, `elision`,
/// `wrap` and `max_lines`, may be `Absent`, a mark of those places only
/// (S2.8, S6.3): no other leaf there, and none of the four on a part.
/// `letter_spacing` and `word_spacing` are never `Absent`, on a part, a
/// slot or a keyed style (S6.3, ADD-A141): a platform whose text has no
/// such spacing states `0px`.
#[test]
fn a_spacing_is_never_absent() {
    for field in ["letter_spacing", "word_spacing"] {
        for table in ["widgets.button.label.text", "tokens.text.roles.body"] {
            assert_eq!(
                bare(&toml(&format!("[{table}]\n{field} = \"absent\""))),
                [format!("{table}.{field}: may not be absent")]
            );
            assert_eq!(bare(&toml(&format!("[{table}]\n{field}_px = 0"))), NONE);
        }
        let decl = declare("text_styles = [\"title\"]");
        let preset = toml("[tokens.text.keyed.title]\nfont.family = \"Probe Sans\"");
        let keyed = toml(&format!("[tokens.text.keyed.title]\n{field} = \"absent\""));
        assert_eq!(
            found_over(&keyed, &decl, &preset),
            [format!(
                "tokens.text.keyed.title.{field}: may not be absent"
            )]
        );
    }
}

#[test]
fn keyed_style_drawn_text_leaves_may_be_absent() {
    let decl = declare("text_styles = [\"title\"]");
    let leaves = "fill = \"absent\"\nelision = \"absent\"\nwrap = \"absent\"\n\
                  max_lines = \"absent\"";
    // The preset beneath states the style, so the patch creates no entry.
    let preset = toml("[tokens.text.keyed.title]\nfont.family = \"Probe Sans\"");
    let keyed = |text: &str| toml(&format!("[tokens.text.keyed.title]\n{text}"));
    assert_eq!(found_over(&keyed(leaves), &decl, &preset), NONE);
    assert_eq!(
        found_over(&keyed("case = \"absent\""), &decl, &preset),
        ["tokens.text.keyed.title.case: may not be absent"]
    );
    let slot = |text: &str| toml(&format!("[tokens.text.roles.body]\n{text}"));
    assert_eq!(bare(&slot(leaves)), NONE);
    assert_eq!(
        bare(&slot("case = \"absent\"")),
        ["tokens.text.roles.body.case: may not be absent"]
    );
    assert_eq!(
        bare(&toml(&format!("[widgets.button.label.text]\n{leaves}"))),
        [
            "widgets.button.label.text.elision: may not be absent",
            "widgets.button.label.text.wrap: may not be absent",
            "widgets.button.label.text.max_lines: may not be absent",
            "widgets.button.label.text.fill: may not be absent",
        ]
    );
}

/// A mark of one place does not travel with a reference (S12): a part's
/// text taking a style's absent leaf, by a leaf's reference, the whole
/// style's or a slot's referencing it, is refused; a stated leaf is not.
#[test]
fn part_text_reaching_absent_style_leaf_refused() {
    let decl = declare("text_styles = [\"title\"]");
    let preset = |elision: &str| {
        toml(&format!(
            "[tokens.text.keyed.title]\nfont.family = \"Probe Sans\"\nelision = \"{elision}\"\n\
             [tokens.text.roles]\nbody = \"= token.text.keyed.title\""
        ))
    };
    let label = |text: &str| toml(&format!("[widgets.button.label]\n{text}"));
    for text in [
        "text.elision = \"= token.text.keyed.title.elision\"",
        "text.elision = \"= token.text.roles.body.elision\"",
        "text = \"= token.text.keyed.title\"",
        "text = \"= token.text.roles.body\"",
    ] {
        assert_eq!(
            found_over(&label(text), &decl, &preset("absent")),
            ["widgets.button.label.text.elision: may not be absent"],
            "{text}"
        );
        assert_eq!(
            found_over(&label(text), &decl, &preset("end")),
            NONE,
            "{text}"
        );
    }
}

/// The T7 review's SF3: a block a plain reference brings absent whole has
/// no fields (S2.2), so none of its fields is refused again below it: a
/// part's icon, which may be absent, referencing an icon context stated
/// `absent` is accepted, though the context's size may not be absent where
/// the context is stated. No block field that may not be absent can
/// reference a block that may be: each block type a reference reaches
/// absent (a part's `text` and `icon`, an icon context) is marked at every
/// place a reference can read it from.
#[test]
fn block_referenced_absent_whole_refuses_no_field() {
    let preset = toml("[tokens.icons.sizes]\ntoolbar = \"absent\"");
    let icon = toml("[widgets.button.icon]\nicon = \"= token.icons.sizes.toolbar\"");
    assert_eq!(found_over(&icon, &Declarations::default(), &preset), NONE);
}

/// The T7 review's SF4: a length's may-be-absent mark does not travel with
/// a reference (S4.2, S12): a part's length reading it where it is
/// `absent` is refused, and accepted where it is stated. `token.caret` is
/// staged with the fields unit (S6.1) and the catalogue has no caret part
/// yet, so the button's subtitle gap stands in for `token.caret.width`
/// and the body's length for a caret part's width.
#[test]
fn part_length_reaching_absent_length_refused() {
    let gap = |width: &str| {
        toml(&format!(
            "[widgets.button.values]\nsubtitle_gap_px = {width}"
        ))
    };
    let body = toml("[widgets.button.body]\nmin_width_px = \"= values.subtitle_gap\"");
    let decl = Declarations::default();
    assert_eq!(
        found_over(&body, &decl, &gap("\"absent\"")),
        ["widgets.button.body.min_width_px: may not be absent"]
    );
    assert_eq!(found_over(&body, &decl, &gap("1")), NONE);
}

/// A slot referencing a style takes its absence with the mark (S12): the
/// slot's own place allows the drawn-text leaves absent, whole or leaf by
/// leaf.
#[test]
fn slot_takes_style_absence() {
    let decl = declare("text_styles = [\"title\"]");
    let preset = toml(
        "[tokens.text.keyed.title]\nfont.family = \"Probe Sans\"\nfill = \"absent\"\n\
         elision = \"absent\"\nwrap = \"absent\"\nmax_lines = \"absent\"",
    );
    let roles = |text: &str| toml(&format!("[tokens.text.roles]\n{text}"));
    assert_eq!(
        found_over(
            &roles("body = \"= token.text.keyed.title\""),
            &decl,
            &preset
        ),
        NONE
    );
    assert_eq!(
        found_over(
            &roles("body.fill = \"= token.text.keyed.title.fill\""),
            &decl,
            &preset
        ),
        NONE
    );
}

/// A part's icon taking an icon-size context's absent `rendering_mode`,
/// which only that context may leave absent (S4.5), is refused (S12); a
/// stated mode is not.
#[test]
fn part_icon_reaching_absent_context_mode_refused() {
    let preset = |mode: &str| {
        toml(&format!(
            "[tokens.icons.sizes.toolbar]\nsize_px = 16\nrendering_mode = \"{mode}\""
        ))
    };
    let decl = Declarations::default();
    let icon = |text: &str| toml(&format!("[widgets.button.icon]\n{text}"));
    for text in [
        "icon.rendering_mode = \"= token.icons.sizes.toolbar.rendering_mode\"",
        "icon = \"= token.icons.sizes.toolbar\"",
    ] {
        assert_eq!(
            found_over(&icon(text), &decl, &preset("absent")),
            ["widgets.button.icon.icon.rendering_mode: may not be absent"],
            "{text}"
        );
        assert_eq!(
            found_over(&icon(text), &decl, &preset("template")),
            NONE,
            "{text}"
        );
    }
}

/// A text-keyed map's key stated twice is one problem at the key as TOML
/// writes it, quoted (S11, S12).
#[test]
fn a_text_keyed_map_names_its_key_quoted() {
    // The font fallbacks' map, walked by its own checks: the fallbacks are
    // staged with the text rendering parking unit (S6.1), and no map at
    // depth is keyed by text.
    let fallbacks = crate::tokens::FontFallbacksPatch {
        per_family: Some(vec![
            ("Segoe UI".to_owned(), Vec::new()),
            ("Segoe UI".to_owned(), Vec::new()),
        ]),
        ..crate::tokens::FontFallbacksPatch::default()
    };
    let problems = super::checks::triggers::walked_block(
        &fallbacks,
        "tokens.text.fallbacks",
        None,
        false,
        &Declarations::default(),
    );
    assert_eq!(
        problems.iter().map(Problem::to_string).collect::<Vec<_>>(),
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

/// A glyph's drawn extent is at least zero (S2.9, S12), and may exceed one:
/// it is drawn clamped to the whole path (a pen width's dash).
#[test]
fn refusal_draw_extent_negative() {
    let extent = |n: f32| toml(&format!("[widgets.button.icon]\nglyph.draw_extent = {n}"));
    assert_eq!(
        bare(&extent(-0.5)),
        ["widgets.button.icon.glyph.draw_extent: out of range: at least zero"]
    );
    assert_eq!(bare(&extent(0.0)), NONE);
    assert_eq!(bare(&extent(2.5)), NONE);
}

/// A reference naming a whole map, no key, names nothing (S2.2, S12): a
/// reference names one entry's value.
#[test]
fn refusal_whole_map_reference() {
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
    // A call on a block-mode enum's field: a transition's own check, called
    // on the list as `button.body`'s (no part lists `transitions` until the
    // motion unit, D6 outcome 3).
    let mut called = transition(vec![PropertyGroup::Fill], Timing::Linear);
    called.timing = Value::Expr(Expr::Call(
        Func::Max,
        vec![
            Expr::Lit(crate::expr::Literal::Number(1.0)),
            Expr::Lit(crate::expr::Literal::Number(2.0)),
        ],
    ));
    assert_eq!(
        body_transitions(vec![called]),
        [
            "widgets.button.body.transitions[0].timing: the expression does not type: a Timing field's expression is a \
         reference only"
        ]
    );
    // Near misses: a call on a number, and on a length a token states. (A
    // reference of a timing's type is a keyed timing's, staged with the
    // motion unit, S6.1.)
    assert_eq!(bare(&body("opacity = \"= mul(0.5, 2)\"")), NONE);
    assert_eq!(
        bare(&body("min_width_px = \"= add(token.focus.width, 2px)\"")),
        NONE
    );
}

/// A layer whose bases style `button.icon`'s icon by `icon`.
fn icon(icon: IconStylePatch) -> Layer {
    let mut layer = Layer::default();
    layer.bases.widgets.button = Some(Box::new(ButtonPatch {
        icon: Some(PartPatch::of_style(PartStylePatch {
            icon: Some(BlockPatch::Fields(icon)),
            ..PartStylePatch::default()
        })),
        ..ButtonPatch::default()
    }));
    layer
}

/// A keyed transition's parameter is typed by its declared unit, as any
/// other keyed scalar (S2.7, S4.6, S12). (A presentation step's left with
/// `button`'s `presentations`, the motion unit's.)
#[test]
fn keyed_kind_params_are_typed() -> Result<(), crate::expr::ParseError> {
    let decl = declare("transition_kinds = { bounce = { amplitude = \"px\" } }");
    // The transition's own checks, called on the list as `button.body`'s:
    // no part lists `transitions` until the motion unit (D6 outcome 3).
    let keyed = |text: &str| -> Result<Vec<String>, crate::expr::ParseError> {
        let item = keyed_icon(TransitionKind::Keyed {
            key: key!("bounce"),
            params: vec![(key!("amplitude"), Value::Expr(Expr::parse(text)?))],
        });
        Ok(on_part(
            &vec![item],
            ButtonPart::Body,
            "transitions",
            false,
            &decl,
        ))
    };
    assert_eq!(
        keyed("token.nowhere")?,
        [
            "widgets.button.body.transitions[0].kind.keyed.params.amplitude: the expression does \
             not type: `token.nowhere` names nothing"
        ]
    );
    assert_eq!(
        keyed("#ff0000")?,
        [
            "widgets.button.body.transitions[0].kind.keyed.params.amplitude: the expression does \
             not type: a colour where a length is expected"
        ]
    );
    assert_eq!(keyed("4px")?, NONE);
    Ok(())
}

/// Every kind of name S10 declares, each refused undeclared and passed
/// declared (S10, S12).
#[test]
fn refusal_undeclared_every_kind() {
    use crate::blocks::focus::{FocusIndicatorPatch, FocusKind};
    use crate::blocks::paint::Material;
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
    let keyed_transition = keyed_icon(TransitionKind::Keyed {
        key: key!("bounce"),
        params: Vec::new(),
    });
    let rows: Vec<(Layer, &str, &str)> = vec![
        (
            rule("{ states = [\"@spin\"] }"),
            "rules[0].when.states[0]: `@spin` is not declared in `widget.button.flags`",
            "[declare.widget.button]\nflags = [\"@spin\"]",
        ),
        // Declared as the default in force: a size or a flavour of a unit
        // not enabled is refused as `Later` (S12; ADD-A147), below.
        (
            rule("{ sizes = [\"large\"] }"),
            "rules[0].when.sizes[0]: `large` is not declared in `widget.button.sizes`",
            "[declare.widget.button]\nsizes = [\"large\"]\ndefault_size = \"large\"",
        ),
        (
            rule("{ conditions = [\"flavor:kirigami\"] }"),
            "rules[0].when.conditions[0]: `kirigami` is not declared in `flavors`",
            "flavors = [\"kirigami\", \"widgets\"]",
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
    ];
    // The near miss over a preset stating the same, so no map entry is
    // created (S12).
    for (layer, problem, declaration) in rows {
        assert_eq!(bare(&layer), [problem], "{problem}");
        let decl = declare(declaration);
        assert_eq!(found_over(&layer, &decl, &layer), NONE, "{declaration}");
    }
    // A transition's keyed kind: the transition's own check, called on the
    // list as `button.body`'s (no part lists `transitions` until the motion
    // unit, D6 outcome 3).
    let kind = |decl: &Declarations| {
        on_part(
            &vec![keyed_transition.clone()],
            ButtonPart::Body,
            "transitions",
            false,
            decl,
        )
    };
    assert_eq!(
        kind(&Declarations::default()),
        [
            "widgets.button.body.transitions[0].kind.keyed.key: `@bounce` is not declared in \
             `transition_kinds`"
        ]
    );
    assert_eq!(kind(&declare("transition_kinds = { bounce = {} }")), NONE);
    // A density undeclared, and declared: every declared density is the
    // sizes unit's, refused while it is not enabled (S10's *Later names*,
    // S12; ADD-A147), so it has no near miss at depth.
    let density = rule("{ not_densities = [\"compact\"] }");
    assert_eq!(
        bare(&density),
        ["rules[0].when.not_densities[0]: `compact` is not declared in `widget.button.densities`"]
    );
    assert_eq!(
        found_over(
            &density,
            &declare("[declare.widget.button]\ndensities = [\"compact\"]"),
            &density
        ),
        ["rules[0].when.not_densities[0]: `compact` is declared in \
             `widget.button.densities`, and a later unit states it"]
    );
    // A component: none is declared until components exist (M3), and
    // TOML names none yet.
    let mut component = Layer::default();
    component.rules.push(crate::selector::Rule::new(
        crate::selector::Selector::default(),
        crate::selector::RuleBody::Part(
            crate::selector::PartRef::Component {
                key: key!("sidebar"),
                part: key!("body"),
            },
            PartPatch::default(),
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
        layer.bases.widgets.button = Some(Box::new(ButtonPatch {
            label: Some(PartPatch::of_style(PartStylePatch {
                text: Some(BlockPatch::Fields(TextStylePatch {
                    font: Some(FontPatch {
                        features: Some(Value::Stated(vec![(
                            crate::blocks::text::FeatureTag::new(tag).unwrap(),
                            1,
                        )])),
                        ..FontPatch::default()
                    }),
                    ..TextStylePatch::default()
                })),
                ..PartStylePatch::default()
            })),
            ..ButtonPatch::default()
        }));
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
        let material = crate::blocks::paint::Material {
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
    // A transition's own checks, called on the list as `button.body`'s.
    assert_eq!(
        body_transitions(vec![transition(
            vec![PropertyGroup::Fill],
            spring(0.0, -0.5, 0.0)
        )]),
        [
            "widgets.button.body.transitions[0].timing.spring.stiffness: out of range: above zero",
            "widgets.button.body.transitions[0].timing.spring.damping_ratio: out of range: at \
             least zero",
            "widgets.button.body.transitions[0].timing.spring.rest_epsilon: out of range: above \
             zero",
        ]
    );
    assert_eq!(
        body_transitions(vec![transition(
            vec![PropertyGroup::Fill],
            spring(100.0, 0.0, 0.001)
        )]),
        NONE
    );
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
/// naming a variant twice; a `sizes_by_variant` entry naming an
/// undeclared variant, or a variant an earlier entry names.
#[test]
fn refusal_declarations() {
    assert_eq!(
        preset(
            "[declare.widget.button]\nvariants = [\"primary\", \"tool\"]\n\
             exclusive_variants = [[\"primary\", \"tool\", \"primary\"]]\n\
             [[rules]]\nwhen = { variants = [\"primary\", \"tool\"] }\ntarget = \"button.body\"\n\
             set = { opacity = 0.5 }"
        ),
        ["declare.widget.button.exclusive_variants[0][2]: `primary` listed twice"]
    );
    let entries = |entries: &str| {
        preset(&format!(
            "[declare.widget.button]\nvariants = [\"primary\"]\nsizes = [\"regular\"]\n\
             default_size = \"regular\"\nsizes_by_variant = {entries}"
        ))
    };
    assert_eq!(
        entries(
            "[{ variant = \"primary\", sizes = [\"regular\"] }, { variant = \"primary\", sizes = \
             [\"regular\"] }, { variant = \"tool\", sizes = [\"regular\"] }]"
        ),
        [
            "declare.widget.button.sizes_by_variant[1]: `primary` listed twice",
            "declare.widget.button.sizes_by_variant[2].variant: `tool` is not declared in \
             `declare.widget.button.variants`",
        ]
    );
    assert_eq!(
        entries("[{ variant = \"primary\", sizes = [\"regular\"] }]"),
        NONE
    );
}

/// `later.variants` lists declared variants (S10's *Later names*, S12;
/// ADD-A147): one the variants in force do not declare is undeclared, one
/// listed twice is refused; and a rule, a context or a patch naming a
/// `later` variant, in a positive or a negative list, is refused with
/// `Later`, the declaration named.
#[test]
fn refusal_later_names() {
    let head = "[declare.widget.button]\nvariants = [\"primary\", \"@solo\"]\n";
    assert_eq!(
        preset(&format!(
            "{head}later = {{ variants = [\"@solo\", \"@duo\", \"@solo\"] }}"
        )),
        [
            "declare.widget.button.later.variants[1]: `@duo` is not declared in \
             `declare.widget.button.variants`",
            "declare.widget.button.later.variants[2]: `@solo` listed twice",
        ]
    );
    assert_eq!(
        preset(&format!("{head}later = {{ variants = [\"@solo\"] }}")),
        NONE
    );
    let later = |rules: &str| {
        preset(&format!(
            "{head}later = {{ variants = [\"@solo\"] }}\n{rules}"
        ))
    };
    let refused = |at: &str| {
        format!("{at}: `@solo` is declared in `widget.button.later`, and a later unit states it")
    };
    assert_eq!(
        later(
            "[[rules]]\nwhen = { not_variants = [\"@solo\"] }\ntarget = \"button.body\"\n\
             set = { opacity = 0.5 }"
        ),
        [refused("rules[0].when.not_variants[0]")]
    );
    assert_eq!(
        later(
            "[[rules]]\nwhen = { context = [{ widget = \"button\", variants = [\"@solo\"] }] }\n\
             target = \"button.label\"\nset = { padding.top_px = 1 }"
        ),
        [refused("rules[0].when.context[0].variants[0]")]
    );
    assert_eq!(
        later(
            "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.icon\"\n\
             set = { drawn_as = { widget = \"button\", part = \"body\", variants = [\"@solo\"] } }"
        ),
        [refused("rules[0].set.drawn_as.variants[0]")]
    );
    assert_eq!(
        later(
            "[[rules]]\nwhen = { variants = [\"primary\"] }\ntarget = \"button.body\"\n\
             set = { opacity = 0.5 }"
        ),
        NONE
    );
}

/// The default flavour's sections are checked for unused names as the
/// widget's own are; another flavour's, of a unit not enabled, are exempt
/// (S12; ADD-A147).
#[test]
fn refusal_unused_in_the_default_flavours_section() {
    let flavors = |default: &str, other: &str| {
        preset(&format!(
            "[declare]\nflavors = [\"{default}\", \"{other}\"]\n\
             [declare.flavor.kirigami.widget.button]\nflags = [\"@spin\"]"
        ))
    };
    assert_eq!(
        flavors("kirigami", "widgets"),
        ["declare.flavor.kirigami.widget.button.flags: `@spin` is declared but unused"]
    );
    assert_eq!(flavors("widgets", "kirigami"), NONE);
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

/// One fieldless variant per `ProblemKind` variant, of the same name, and
/// `index`, which maps each kind to its own: both written from one list,
/// whose `match` names every kind without a wildcard, so a new kind does not
/// compile until it is listed, no kind can be indexed as another's variant,
/// and `COUNT` is the list's length.
macro_rules! kinds {
    ($($name:ident $(($($tuple:tt)*))? $({ $($fields:tt)* })?),* $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum V {
            $($name,)*
        }

        impl V {
            /// How many there are.
            const COUNT: usize = [$(V::$name),*].len();
        }

        /// The kind's own variant of `V`.
        fn index(kind: &crate::error::ProblemKind) -> V {
            use crate::error::ProblemKind as K;
            match kind {
                $(K::$name $(($($tuple)*))? $({ $($fields)* })? => V::$name,)*
            }
        }
    };
}

kinds! {
    Toml(_),
    UnknownKey,
    Mistyped { .. },
    UnknownName { .. },
    NotAKey(_),
    Expression(_),
    Missing,
    DuplicateKey { .. },
    NotFinite,
    PartWithoutWidget,
    NotAPartField { .. },
    AppendInBases,
    AbsentNotAllowed,
    AutoNotAllowed,
    CallNotAllowed,
    OutOfRange { .. },
    NoLayer,
    MaskAtBottom,
    TooFewStops,
    NoProperties,
    FromNoState,
    TimingAbsent,
    TimingOnZero,
    DurationAbsent,
    InstantNotZero,
    ZeroNotInstant,
    TimingNotComputed { .. },
    EngineMismatch { .. },
    BreezeTestInvalid { .. },
    FocusMoveNotRun,
    SeveralCurves,
    OverridesNothing,
    RecencyTooSmall,
    RecencyOrder { .. },
    NotInRecencySet { .. },
    StateLayerWithoutInteraction,
    IndicationBoxOtherWidget { .. },
    IndicationBoxPartAbsent { .. },
    KeyframesNotIncreasing,
    KeyframeWithoutPart,
    NoKeyframe,
    KeyframeNamesPart,
    KeyframeOutsideGroups,
    SmoothstepSpreadOrInset,
    FollowPartOutsideFocus,
    RoundPtMisused,
    RouteDuplicated { .. },
    KdeAccentArgument { .. },
    PickWithoutColors,
    TooDeep,
    BaseInBases,
    ElementOperand { .. },
    SurfaceInRoles,
    IllTyped { .. },
    OtherElementsPart { .. },
    UndeclaredData { .. },
    Undeclared { .. },
    Later { .. },
    NotConditions,
    WidgetUnavailable { .. },
    AccentInput,
    ListedTwice { .. },
    SizeNotTaken { .. },
    DefaultSizeMissing,
    NoSizeInContext { .. },
    SizeContextKey { .. },
    NotABlock,
    UnitMismatch { .. },
    DrawnAsCycle,
    TextScaleUnderDynamicType,
    AbsentOperand { .. },
    AutoOperand { .. },
    ExpressionCycle,
    ChainTooLong,
    PartNotDrawn { .. },
    CornerAtLayout { .. },
    UnequalOffsets { .. },
    UnequalInsets { .. },
    TextRoleShared { .. },
    TextRoleOrder { .. },
    ShapeOverCorners,
    EllipseCorners,
    EntryCreatedUnset { .. },
    ElementSizeInLayout { .. },
    LeftToPreset { .. },
    SetStatedWhole { .. },
    Unused { .. },
    SyntheticExpression,
    SyntheticOverlap,
}

/// Every `LayerError` and `ProblemKind` variant, each triggered by a
/// fixture: S11's reader's kinds by text it refuses, S12's by a layer or a
/// stack it refuses. `index` matches every variant without a wildcard,
/// each to its own `V` (`kinds!`), so a new variant does not compile until
/// it is listed, and fails here until a fixture triggers it.
#[test]
fn layer_error_every_variant_triggers() {
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
                left_out: None,
            })
            .collect();
        let cx = Context {
            declarations: decl,
            preset: None,
            kind: LayerKind::Application,
            reader_accent_left_out: false,
            below: &below,
        };
        problems(check(&[layer], &cx, &Reused::default()))
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
        problems(check_preset(
            &file.declarations,
            &file.layer,
            None,
            &[],
            None,
            &Reused::default(),
        ))
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
            angle: crate::units::Deg(0.0),
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
    // A loop's own check, called on the block as `button.icon`'s icon
    // animation, which is staged with the motion unit (S6.1).
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
        let looped = Loop {
            period: Value::Stated(Ms(1000.0)),
            timing: Value::Stated(Timing::Linear),
            keyframes,
        };
        super::checks::triggers::walked_block(
            &looped,
            "widgets.button.icon.icon.animation",
            Some(TypedPart::Button(ButtonPart::Icon)),
            false,
            &none,
        )
    };
    // A part's transitions: no part lists `transitions` until the motion
    // unit (D6 outcome 3), so their own checks are called on the list, read
    // by its own reader as `button.body`'s, and the checks over
    // combinations on a stack holding the list built in Rust (as `shape`'s
    // below).
    let body_list = |text: &str| -> Vec<Transition> {
        read_in::<Vec<Transition>>(
            text,
            Spelling::Bare,
            "widgets.button.body.transitions",
            Some(WidgetKind::Button),
        )
        .unwrap_or_default()
    };
    let walked = |text: &str, decl: &Declarations| {
        super::checks::triggers::walked_block(
            &body_list(text),
            "widgets.button.body.transitions",
            Some(TypedPart::Button(ButtonPart::Body)),
            true,
            decl,
        )
    };
    let combined = |text: &str| {
        let layer = transitions(body_list(text));
        let layers = [super::Stacked {
            layer: &layer,
            kind: LayerKind::Application,
            left_out: None,
        }];
        let cx = Context {
            declarations: &none,
            preset: None,
            kind: LayerKind::Application,
            reader_accent_left_out: false,
            below: &[],
        };
        super::combos::check(&super::stack::Stack::new(&layers, cx), None)
            .into_iter()
            .map(|(_, problem)| problem)
            .collect::<Vec<Problem>>()
    };
    let route = {
        let mut layer = Layer::default();
        layer.bases.widgets.button = Some(Box::new(ButtonPatch {
            label: Some(PartPatch::of_style(PartStylePatch {
                text: Some(BlockPatch::Fields(TextStylePatch {
                    font: Some(FontPatch {
                        axes: Some(Value::Stated(vec![(
                            AxisTag::new("wght").unwrap(),
                            AxisValue::Fixed(1.0),
                        )])),
                        ..FontPatch::default()
                    }),
                    ..TextStylePatch::default()
                })),
                ..PartStylePatch::default()
            })),
            ..ButtonPatch::default()
        }));
        layer
    };
    let deep = body(PartStylePatch {
        opacity: Some(Value::Expr(
            (0..=MAX_DEPTH).fold(Expr::Lit(crate::expr::Literal::Number(1.0)), |expr, _| {
                Expr::Call(Func::Floor, vec![expr])
            }),
        )),
        ..PartStylePatch::default()
    });
    let mismatch = {
        let mut layer = Layer::default();
        layer.bases.tokens.params = Some(vec![(
            key!("layout"),
            vec![(
                key!("gap"),
                Value::Stated(crate::units::Scalar::Ms(Ms(4.0))),
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
        layer.rules.push(crate::selector::Rule::new(
            crate::selector::Selector::default(),
            crate::selector::RuleBody::Part(
                crate::selector::PartRef::Typed(crate::widgets::TypedPart::Button(
                    crate::widgets::ButtonPart::Body,
                )),
                PartPatch::of_style(PartStylePatch {
                    shape: Some(Value::Expr(Expr::Ref(Ref::Base))),
                    corners: Some(BlockPatch::Fields(CornersPatch {
                        curve: Some(crate::blocks::outline::CornerCurve::Continuous),
                        ..CornersPatch::default()
                    })),
                    ..PartStylePatch::default()
                }),
            ),
        ));
        let layers = [super::Stacked {
            layer: &layer,
            kind: LayerKind::Application,
            left_out: None,
        }];
        let cx = Context {
            declarations: &none,
            preset: None,
            kind: LayerKind::Application,
            reader_accent_left_out: false,
            below: &[],
        };
        super::combos::check(&super::stack::Stack::new(&layers, cx), None)
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
                left_out: None,
            },
            super::Stacked {
                layer: &palette,
                kind: LayerKind::Palette,
                left_out: None,
            },
        ];
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Palette,
            reader_accent_left_out: false,
            below: &[],
        };
        super::combos::mapping_palette(&super::stack::Stack::new(&layers, cx), 1)
    };
    let whole = {
        use crate::roles::SurfaceSet;
        let preset = read_preset(crate::presets::PresetId::KdeBreeze.source()).ok();
        let mut stand_in = Layer::default();
        stand_in.bases.colors.light = preset.and_then(|preset| preset.layer.bases.colors.light);
        let left_out = crate::resolve::LeftOut::new(
            vec![crate::selector::Scheme::Light],
            &[super::AccentRole::Surface(
                SurfaceSet::View,
                crate::inputs::KdeSchemeKey::ForegroundLink,
            )],
            Vec::new(),
        );
        super::sets_stated_whole(
            &super::Stacked {
                layer: &stand_in,
                kind: LayerKind::Reader,
                left_out: Some(&left_out),
            },
            None,
        )
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
            walked(
                "[{ properties = [], kind = \"interpolate\", duration_ms = 100, delay_ms = 0, \
                 timing = \"linear\", engine = \"gtk_css\" }]",
                &none,
            ),
        ),
        (V::KeyframesNotIncreasing, keyframes),
        (
            V::KeyframeWithoutPart,
            super::checks::triggers::keyframe_without_part(),
        ),
        (
            V::NoKeyframe,
            css_transition("\"opacity\"", &keyframes_kind(&[])),
        ),
        (
            V::KeyframeNamesPart,
            css_transition(
                "\"opacity\"",
                &keyframes_kind(&[("0", ", part = \"body\", patch = { opacity = 0 }")]),
            ),
        ),
        (
            V::KeyframeOutsideGroups,
            css_transition(
                "\"fill\"",
                &keyframes_kind(&[("0", ", patch = { opacity = 0 }")]),
            ),
        ),
        (
            V::SmoothstepSpreadOrInset,
            read(
                "[widgets.button.body]\nshadow = { layers = [{ x_px = 0, y_px = 1, blur_px = 8, spread_px = 0, color = \"#00000040\", inset = true, falloff = \"kirigami_smoothstep\" }] }",
            ),
        ),
        (
            V::FollowPartOutsideFocus,
            read("[widgets.button.body]\ncorners.top_left = \"follow_part\""),
        ),
        (
            V::RoundPtMisused,
            read(
                "[widgets.button.label]\npadding.top_px = \"= round_pt(token.text.roles.body.font.size)\"",
            ),
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
            V::Later,
            preset_problems(&format!(
                "[declare.widget.button]\nvariants = [\"@solo\"]\nlater = {{ variants = [\"@solo\"] }}\n{}",
                rule(
                    "{ variants = [\"@solo\"] }",
                    "button.body",
                    "{ opacity = 0.5 }"
                )
            )),
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
                "[declare.widget.button]\nsizes = [\"regular\"]\ndefault_size = \"large\"",
            ),
        ),
        (
            V::DefaultSizeMissing,
            preset_problems("[declare.widget.button]\nsizes = [\"regular\"]"),
        ),
        (
            V::NoSizeInContext,
            preset_problems(
                "[declare.widget.button]\nsizes = [\"regular\"]\ndefault_size = \"regular\"\n\
                 sizes_by_context = [{ context = { widget = \"button\" }, sizes = [\"small\"] }]",
            ),
        ),
        (
            V::SizeContextKey,
            preset_problems(
                "[declare.widget.button]\nsizes = [\"regular\"]\ndefault_size = \"regular\"\n\
                 sizes_by_context = [{ context = { widget = \"button\", states = [\"hover\"] }, \
                 sizes = [\"regular\"] }]",
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
        (V::SetStatedWhole, whole),
        (
            V::SyntheticExpression,
            super::synthetic_shape(
                &[toml("[tokens.focus]\nwidth_px = \"= token.focus.width\"")],
                &none,
                None,
            ),
        ),
        (
            V::SyntheticOverlap,
            super::synthetic_shape(
                &[
                    toml("[tokens.focus]\nwidth_px = 1"),
                    toml("[tokens]\nfocus.width_px = 2"),
                ],
                &none,
                None,
            ),
        ),
        (
            V::Unused,
            preset_problems("[declare.widget.button]\nflags = [\"@spin\"]"),
        ),
        (
            V::TimingAbsent,
            combined(
                "[{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = 0, timing = \"absent\", engine = \"gtk_css\" }]",
            ),
        ),
        (
            V::TimingOnZero,
            combined(
                "[{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 0, delay_ms = 0, timing = \"linear\", engine = \"gtk_css\" }]",
            ),
        ),
        (
            V::DurationAbsent,
            combined(
                "[{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = \"absent\", delay_ms = 0, timing = \"linear\", engine = \"gtk_css\" }]",
            ),
        ),
        (
            V::EngineMismatch,
            walked(
                "[{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = 10, timing = \"linear\", engine = { breeze = { any = [\"hover\"], none = [], conditions = [] } } }]",
                &none,
            ),
        ),
        (
            V::BreezeTestInvalid,
            walked(
                "[{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = 0, timing = \"linear\", engine = { breeze = { any = [], none = [], conditions = [] } } }]",
                &none,
            ),
        ),
        (
            V::FocusMoveNotRun,
            walked(
                "[{ properties = [\"fill\"], kind = \"focus_move\", duration_ms = 100, delay_ms = 0, timing = \"linear\", engine = \"gtk_css\" }]",
                &none,
            ),
        ),
        (
            V::SeveralCurves,
            combined(
                "[{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = 0, timing = \"linear\", engine = \"gtk_css\" }, { properties = [\"stroke\"], kind = \"interpolate\", duration_ms = 100, delay_ms = 0, timing = { cubic_bezier = [0.25, 0.1, 0.25, 1] }, engine = \"gtk_css\" }]",
            ),
        ),
        (
            V::OverridesNothing,
            combined(
                "[{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 0, delay_ms = 0, timing = \"absent\", engine = \"gtk_css\" }]",
            ),
        ),
        (
            V::TimingNotComputed,
            combined(
                "[{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = 0, timing = { spring = { mass = 1, stiffness = 100, damping_ratio = 1, initial_velocity = 0, rest_epsilon = 0.001, clamp = false, time_scale = 1 } }, engine = \"gtk_css\" }]",
            ),
        ),
        (
            V::FromNoState,
            walked(
                "[{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = 0, timing = \"linear\", engine = \"gtk_css\", from = [] }]",
                &none,
            ),
        ),
        (
            V::IndicationBoxOtherWidget,
            over(&indication_on(component_part()), &none, &[]),
        ),
        (
            V::IndicationBoxPartAbsent,
            over(
                &toml(
                    "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.icon\"\nset = { absent = true }",
                ),
                &Declarations::default(),
                &[ripple("{ part = \"icon\", outset_px = 0 }")],
            ),
        ),
        (
            V::StateLayerWithoutInteraction,
            super::checks::triggers::walked_block(
                &body_list(
                    "[{ properties = [\"state_layer\"], kind = \"interpolate\", duration_ms = 15, delay_ms = 0, timing = \"linear\", engine = \"gtk_css\" }]",
                ),
                "widgets.button.icon.transitions",
                Some(TypedPart::Button(ButtonPart::Icon)),
                true,
                &none,
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
    // A presentation step's kinds: no enabled widget's values list
    // `presentations` (the button's are the motion unit's, D6 outcome 3,
    // S6.1), and no stack holds a step, so no fixture can reach them.
    let unreachable = [V::InstantNotZero as usize, V::ZeroNotInstant as usize];
    let missing: Vec<usize> = (0..V::COUNT)
        .filter(|i| !covered.get(*i).copied().unwrap_or(false) && !unreachable.contains(i))
        .collect();
    assert_eq!(missing, Vec::<usize>::new());
    // `LayerError`'s every variant: `Invalid`, which every fixture above
    // returns, and the stack's refusals (S8, S9.2, S16).
    every_layer_error();
}

/// Every `LayerError` variant, each triggered: `Invalid` by a layer's
/// problem, the others by the stack's refusals. Its own frame, so the
/// fixtures above keep theirs small.
#[inline(never)]
fn every_layer_error() {
    let none = Declarations::default();
    let invalid = check(
        &[&bare_layer_with_a_problem()],
        &Context {
            declarations: &none,
            preset: None,
            kind: LayerKind::Application,
            reader_accent_left_out: false,
            below: &[],
        },
        &Reused::default(),
    );
    let mut errors = crate::resolve::refusals();
    errors.extend(invalid.err());
    let mut seen = [false; 6];
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
            LayerError::UnknownLayer(_) => 3,
            LayerError::SchemeNotInPalette(_) => 4,
            LayerError::PaletteNotForPreset(_) => 5,
        };
        seen[index] = true;
    }
    assert_eq!(seen, [true; 6], "{errors:?}");
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

/// S9.4: every text scaling kind is implemented, so validation refuses
/// none.
#[test]
fn every_text_scaling_kind_accepted() {
    for kind in ["linear", "android_non_linear", "dynamic_type"] {
        assert_eq!(
            bare(&toml(&format!("[tokens.text]\nscaling = \"{kind}\""))),
            NONE,
            "{kind}"
        );
    }
}

/// Task 11's review: a keyed flag in a token's transition `from` is one the
/// element of any widget reading the token may hold, so every widget
/// declares it (S10, S12). The transition's own check, called on the block
/// read by its own reader as the tokens' focus indicator's, which is
/// staged with the motion unit (S6.1).
#[test]
fn token_transition_keyed_flag_declared() {
    let decl = declare("[declare.widget.button]\nflags = [\"@armed\"]");
    let at = "tokens.focus.transition";
    let checked = |flag: &str| {
        let text = format!(
            "{{ properties = [\"fill\"], kind = \"interpolate\", duration_ms = 100, delay_ms = \
             0, timing = \"linear\", engine = \"gtk_css\", from = [\"{flag}\"] }}"
        );
        match read_in::<Transition>(&text, Spelling::Bare, at, None) {
            Ok(transition) => {
                super::checks::triggers::walked_block(&transition, at, None, true, &decl)
                    .iter()
                    .map(Problem::to_string)
                    .collect()
            }
            Err(problems) => problems,
        }
    };
    assert_eq!(checked("@armed"), NONE);
    assert_eq!(
        checked("@nope"),
        ["tokens.focus.transition.from[0]: `@nope` is not declared in `widget.button.flags`"]
    );
}
