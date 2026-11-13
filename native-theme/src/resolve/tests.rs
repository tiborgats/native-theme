//! S8's resolution and S9.2's accent layer. Every
//! number and name here is synthetic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

use std::sync::Arc;

use super::{Change, Pos, ReaderState, Stack, WindowConditions, WindowState};
use crate::blocks::Apply;
use crate::blocks::part_style::PartStylePatch;
use crate::color::Color;
use crate::declare::Declarations;
use crate::error::LayerError;
use crate::inputs::{Accent, Contrast, InputName, InputValue};
use crate::key;
use crate::key::Key;
use crate::layer::{Layer, LayerId};
use crate::presets::PresetId;
use crate::selector::{Condition, InputKind, Rule, RuleBody, Scheme, Selector, WidgetRef};
use crate::toml::layer::read_preset;
use crate::units::Fraction;
use crate::value::{SchemeMap, Value};
use crate::widgets::{ButtonPart, ButtonPatch, PartPatch, PartSlot, TypedPart, WidgetKind};

/// A stack of these layers, unvalidated: the resolution alone is tested.
pub(super) fn stack(
    preset: Layer,
    reader: Option<(ReaderState, Layer)>,
    layers: Vec<Layer>,
) -> Stack {
    Stack {
        preset: None,
        declarations: Declarations::default(),
        preset_layer: Arc::new(preset),
        derivation: None,
        reader_left_out: None,
        reader: reader.map(|(state, layer)| super::Reader::StandIn {
            state,
            layer: Arc::new(layer),
        }),
        layers: layers
            .into_iter()
            .map(|layer| (LayerId::next(), Arc::new(layer)))
            .collect(),
        palette: None,
        port_left_out: None,
        accent: None,
        accent_layer: Arc::default(),
        picked: None,
        default_scheme: Scheme::Light,
        contrast: None,
        flavor: None,
        pending: Vec::new(),
        synthetic: None,
    }
}

pub(super) fn window() -> WindowConditions {
    WindowConditions::new(InputKind::Mouse)
}

fn opacity(n: f32) -> PartPatch {
    PartPatch::of_style(PartStylePatch {
        opacity: Some(Value::Stated(Fraction(n))),
        ..PartStylePatch::default()
    })
}

/// A layer whose bases set the button body's opacity.
pub(super) fn body(n: f32) -> Layer {
    let mut layer = Layer::default();
    layer.bases.widgets.button = Some(Box::new(ButtonPatch {
        body: Some(opacity(n)),
        ..ButtonPatch::default()
    }));
    layer
}

/// A rule setting the body's opacity under `condition`.
fn body_rule(condition: Condition, n: f32) -> Rule {
    let mut selector = Selector::default();
    selector.conditions.all_of.push(condition);
    Rule::new(
        selector,
        RuleBody::Part(
            crate::selector::PartRef::Typed(TypedPart::Button(ButtonPart::Body)),
            opacity(n),
        ),
    )
}

fn stated_opacity(slot: &PartSlot) -> Option<f32> {
    match slot.style.as_ref()?.opacity.as_ref()? {
        Value::Stated(Fraction(n)) => Some(*n),
        _ => None,
    }
}

/// A rule whose `Preset` condition names another preset than the current
/// one is not applied, and one naming the current preset is; a
/// `not_conditions` entry naming the current preset excludes its rule
/// (S7.1, S12): `adwaita`'s rules under no bundled preset and under
/// `adwaita`.
#[test]
fn a_rule_naming_another_preset_is_not_applied() {
    let opacities = |preset: Option<PresetId>, rule: Rule| {
        let mut application = body(0.2);
        application.rules.push(rule);
        let mut stack = stack(Layer::default(), None, vec![application]);
        stack.preset = preset;
        let theme = stack.resolve(&mut WindowState::default(), &window()).theme;
        let mut slot = PartSlot::default();
        theme
            .part_steps(TypedPart::Button(ButtonPart::Body))
            .iter()
            .map(|step| {
                slot.apply(step.patch.clone()).unwrap();
                stated_opacity(&slot).unwrap()
            })
            .collect::<Vec<_>>()
    };
    let named = || body_rule(Condition::Preset(PresetId::Adwaita), 0.4);
    assert_eq!(opacities(None, named()), [0.2]);
    assert_eq!(opacities(Some(PresetId::Adwaita), named()), [0.2, 0.4]);
    let mut unless = named();
    unless.selector.conditions.none_of = std::mem::take(&mut unless.selector.conditions.all_of);
    assert_eq!(opacities(Some(PresetId::Adwaita), unless), [0.2]);
}

/// S8's order: every layer's bases, in order (preset, reader, the
/// application's), then every layer's rules whose conditions hold, in
/// order; a rule whose conditions do not hold is not kept.
#[test]
fn bases_then_rules_order() {
    let mut preset = body(0.1);
    preset.rules.push(body_rule(Condition::ReducedMotion, 0.3));
    preset.rules.push(body_rule(Condition::BoldText, 0.9));
    let mut state = ReaderState::new(Scheme::Light, 1.0);
    state.reduced_motion = true;
    let mut application = body(0.2);
    application
        .rules
        .push(body_rule(Condition::ReducedMotion, 0.4));
    let stack = stack(preset, Some((state, body(0.15))), vec![application]);
    let theme = stack.resolve(&mut WindowState::default(), &window()).theme;
    let steps = theme.part_steps(TypedPart::Button(ButtonPart::Body));
    let base = |layer| Pos {
        rule: false,
        layer,
        index: 0,
    };
    let rule = |layer, index| Pos {
        rule: true,
        layer,
        index,
    };
    assert_eq!(
        steps.iter().map(|step| step.pos).collect::<Vec<_>>(),
        [base(0), base(1), base(2), rule(0, 0), rule(2, 0)]
    );
    let mut slot = PartSlot::default();
    let mut seen = Vec::new();
    for step in steps {
        slot.apply(step.patch.clone()).unwrap();
        seen.push(stated_opacity(&slot).unwrap());
    }
    assert_eq!(seen, [0.1, 0.15, 0.2, 0.3, 0.4]);
    // A base stated after a rule in the application's layer still comes
    // before every rule.
    assert!(steps.windows(2).all(|pair| pair[0].pos < pair[1].pos));
}

/// S8's change classification: the first resolution, none, a condition
/// change, and a theme change (a layer, the scheme, the contrast, a reader
/// value), which wins over a condition change.
#[test]
fn change_classification() {
    let mut stack = stack(Layer::default(), None, Vec::new());
    let mut state = WindowState::default();
    let plain = window();
    let change = |stack: &Stack, state: &mut WindowState, conditions: &WindowConditions| {
        stack.resolve(state, conditions).change
    };
    assert_eq!(change(&stack, &mut state, &plain), Change::First);
    assert_eq!(change(&stack, &mut state, &plain), Change::Unchanged);
    let mut inactive = window();
    inactive.inactive = true;
    assert_eq!(
        change(&stack, &mut state, &inactive),
        Change::ConditionChange
    );
    assert_eq!(change(&stack, &mut state, &plain), Change::ConditionChange);
    stack.add_layer(body(0.5)).unwrap();
    assert_eq!(change(&stack, &mut state, &plain), Change::ThemeChange);
    // An equal stack is no change, however it was reached.
    stack.layers = vec![(LayerId::next(), Arc::new(body(0.5)))];
    assert_eq!(change(&stack, &mut state, &plain), Change::Unchanged);
    stack.pick(Some(Scheme::Dark)).unwrap();
    assert_eq!(change(&stack, &mut state, &plain), Change::ThemeChange);
    stack.set_contrast(Some(Contrast::High)).unwrap();
    assert_eq!(change(&stack, &mut state, &plain), Change::ThemeChange);
    stack.declarations.flavors = vec![key!("first"), key!("other")];
    // The declared default comes in force: a condition change (S8).
    assert_eq!(change(&stack, &mut state, &plain), Change::ConditionChange);
    // `set_flavor` refuses a flavour but the default while the flavours
    // unit is not enabled (S6.1, S16; ADD-A147), which changes nothing; a
    // flavour switch, set here directly, is a condition change (S8).
    assert!(stack.set_flavor(key!("other")).is_err());
    assert_eq!(change(&stack, &mut state, &plain), Change::Unchanged);
    stack.flavor = Some(key!("other"));
    assert_eq!(change(&stack, &mut state, &plain), Change::ConditionChange);
    // Both at once: the theme change wins.
    stack.layers.clear();
    assert_eq!(change(&stack, &mut state, &inactive), Change::ThemeChange);
    // The reader's conditions and values.
    let mut read = ReaderState::new(Scheme::Light, 1.0);
    stack.reader = Some(super::Reader::StandIn {
        state: read.clone(),
        layer: Arc::default(),
    });
    assert_eq!(change(&stack, &mut state, &inactive), Change::ThemeChange);
    read.reduced_motion = true;
    stack.reader = Some(super::Reader::StandIn {
        state: read.clone(),
        layer: Arc::default(),
    });
    assert_eq!(
        change(&stack, &mut state, &inactive),
        Change::ConditionChange
    );
    read.text_scaling_factor = 1.5;
    stack.reader = Some(super::Reader::StandIn {
        state: read.clone(),
        layer: Arc::default(),
    });
    assert_eq!(change(&stack, &mut state, &inactive), Change::ThemeChange);
    // The reader's scheme under a picked one changes nothing in force.
    read.scheme = Scheme::Dark;
    stack.reader = Some(super::Reader::StandIn {
        state: read,
        layer: Arc::default(),
    });
    assert_eq!(change(&stack, &mut state, &inactive), Change::Unchanged);
}

/// A stand-in is refused while a reader exists (S8, R2.6); without one it
/// is set, and replaced.
#[test]
fn reader_stand_in_refused_with_reader() {
    let state = || ReaderState::new(Scheme::Dark, 1.0);
    // A preset stating nothing is refused: a preset states every field.
    assert!(matches!(
        Stack::new(
            None,
            Declarations::default(),
            Layer::default(),
            Scheme::Light,
            Vec::new()
        ),
        Err(LayerError::Invalid(_))
    ));
    let mut stack = stack(Layer::default(), None, Vec::new());
    stack
        .set_reader_stand_in(state(), Layer::default())
        .unwrap();
    assert_eq!(stack.scheme(), Scheme::Dark);
    stack.set_reader_stand_in(state(), body(0.5)).unwrap();
    let mut read = stack.clone();
    read.set_reader(state(), Layer::default(), None).unwrap();
    assert_eq!(
        read.set_reader_stand_in(state(), Layer::default()),
        Err(LayerError::ReaderPresent)
    );
    // The refused change leaves the stack as it was.
    assert!(matches!(read.reader, Some(super::Reader::Read { .. })));
    // A stand-in's layer is validated as a reader's: refused whole.
    let mut invalid = Layer::default();
    invalid.bases.inputs.entries.push((
        InputName::new("nowhere").unwrap(),
        InputValue::new(SchemeMap::one(Scheme::Light, Value::Absent)),
    ));
    assert!(matches!(
        stack.set_reader_stand_in(state(), invalid),
        Err(LayerError::Invalid(_))
    ));
}

pub(super) fn color(r: f32) -> Value<Color> {
    Value::Stated(Color {
        r,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    })
}

pub(super) fn input(r: f32) -> InputValue {
    InputValue::new(SchemeMap::new(color(r), color(r)))
}

/// S9.2: the accent layer states the preset's accent inputs; an accent
/// lacking one is refused, naming it; `None` clears the layer. The accent
/// layer comes last, over the stand-in's inputs.
#[test]
fn accent_layer_states_the_accent_inputs() {
    let declarations = read_preset(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n[declare]\n\
         inputs = [\"accent\", \"accent_ramp.light1\"]\n\
         accent_inputs = [\"accent\", \"accent_ramp.light1\"]\n",
    )
    .unwrap()
    .declarations;
    let mut stack = stack(Layer::default(), None, Vec::new());
    stack.declarations = declarations;
    let ramp = InputName::new("accent_ramp.light1").unwrap();
    assert_eq!(
        stack.set_accent(Some(Accent::new(input(0.1)))),
        Err(LayerError::AccentInputsMissing(vec![ramp.clone()]))
    );
    let mut reader = Layer::default();
    reader
        .bases
        .inputs
        .entries
        .push((InputName::new("accent").unwrap(), input(0.9)));
    reader.bases.inputs.entries.push((ramp.clone(), input(0.8)));
    stack
        .set_reader_stand_in(ReaderState::new(Scheme::Light, 1.0), reader)
        .unwrap();
    let mut accent = Accent::new(input(0.1));
    accent.derived.push((ramp.clone(), input(0.2)));
    stack.set_accent(Some(accent)).unwrap();
    let inputs = |stack: &Stack| {
        stack
            .resolve(&mut WindowState::default(), &window())
            .theme
            .inputs
    };
    assert_eq!(
        inputs(&stack),
        [
            (InputName::new("accent").unwrap(), color(0.1)),
            (ramp.clone(), color(0.2)),
        ]
    );
    stack.set_accent(None).unwrap();
    assert_eq!(
        inputs(&stack),
        [
            (InputName::new("accent").unwrap(), color(0.9)),
            (ramp, color(0.8)),
        ]
    );
}

/// An input's per-contrast base applies where its contrast holds; where it
/// states none, its per-scheme value applies under contrast too (S9).
#[test]
fn inputs_take_their_contrast_base() {
    let mut reader = Layer::default();
    let mut value = input(0.3);
    value
        .per_contrast
        .push((Contrast::High, SchemeMap::one(Scheme::Light, color(0.7))));
    let accent = InputName::new("accent").unwrap();
    reader.bases.inputs.entries.push((accent.clone(), value));
    let mut state = ReaderState::new(Scheme::Light, 1.0);
    let resolved = |state: &ReaderState| {
        stack(
            Layer::default(),
            Some((state.clone(), reader.clone())),
            Vec::new(),
        )
        .resolve(&mut WindowState::default(), &window())
        .theme
        .inputs
    };
    assert_eq!(resolved(&state), [(accent.clone(), color(0.3))]);
    state.contrast = Some(Contrast::High);
    assert_eq!(resolved(&state), [(accent.clone(), color(0.7))]);
    state.contrast = Some(Contrast::Increased);
    assert_eq!(resolved(&state), [(accent, color(0.3))]);
}

/// An input is replaced slot by slot, never whole (S9, ADD-A132): at each
/// of the six slots the last layer giving the input a value there wins,
/// its contrast base for that contrast, else its per-scheme value, so a
/// stand-in's one-scheme value stands under every contrast at that scheme,
/// over the preset's contrast base, and leaves the other scheme's slots to
/// the preset but where it states a contrast base.
#[test]
fn an_input_is_replaced_slot_by_slot() {
    let accent = InputName::new("accent").unwrap();
    let mut preset = Layer::default();
    let mut stated = InputValue::new(SchemeMap::new(color(0.1), color(0.2)));
    stated
        .per_contrast
        .push((Contrast::High, SchemeMap::new(color(0.3), color(0.4))));
    preset.bases.inputs.entries.push((accent.clone(), stated));
    let mut reader = Layer::default();
    let mut one = InputValue::new(SchemeMap::one(Scheme::Dark, color(0.9)));
    one.per_contrast.push((
        Contrast::Increased,
        SchemeMap::one(Scheme::Light, color(0.8)),
    ));
    reader.bases.inputs.entries.push((accent.clone(), one));
    let at = |scheme: Scheme, contrast: Option<Contrast>| {
        let mut state = ReaderState::new(scheme, 1.0);
        state.contrast = contrast;
        stack(preset.clone(), Some((state, reader.clone())), Vec::new())
            .resolve(&mut WindowState::default(), &window())
            .theme
            .inputs
    };
    let slots = [
        (Scheme::Light, None, 0.1),
        (Scheme::Light, Some(Contrast::Increased), 0.8),
        (Scheme::Light, Some(Contrast::High), 0.3),
        (Scheme::Dark, None, 0.9),
        (Scheme::Dark, Some(Contrast::Increased), 0.9),
        (Scheme::Dark, Some(Contrast::High), 0.9),
    ];
    for (scheme, contrast, r) in slots {
        assert_eq!(
            at(scheme, contrast),
            [(accent.clone(), color(r))],
            "{scheme:?} {contrast:?}"
        );
    }
}

/// The stack's refusals, for `layer_error_every_variant_triggers`: a
/// stand-in while a reader exists, an accent lacking an accent input, an
/// id the stack does not hold, a palette under a scheme it lacks and one
/// with no source for the preset (S14). A palette that does not convert
/// gives no refusal, which that test then reports as missing.
pub(super) fn refusals() -> Vec<LayerError> {
    let mut refused = stack_refusals();
    // A port of a synthetic file, for the dark scheme alone.
    let port =
        || crate::Palette::from_port(b"[Colors:Window]\nBackgroundNormal=1,2,3\n", Scheme::Dark);
    let mut light = stack(Layer::default(), None, Vec::new());
    refused.extend(port().ok().and_then(|p| light.apply_palette(Some(p)).err()));
    let mut adwaita = stack(Layer::default(), None, Vec::new());
    adwaita.preset = Some(PresetId::Adwaita);
    adwaita.picked = Some(Scheme::Dark);
    refused.extend(
        port()
            .ok()
            .and_then(|p| adwaita.apply_palette(Some(p)).err()),
    );
    refused
}

/// [`refusals`]' first three.
fn stack_refusals() -> Vec<LayerError> {
    let mut read = stack(Layer::default(), None, Vec::new());
    read.set_reader(ReaderState::new(Scheme::Light, 1.0), Layer::default(), None)
        .unwrap();
    let present = read
        .set_reader_stand_in(ReaderState::new(Scheme::Light, 1.0), Layer::default())
        .unwrap_err();
    let mut accent = stack(Layer::default(), None, Vec::new());
    accent.declarations.accent_inputs = vec![
        InputName::new("accent").unwrap(),
        InputName::new("accent_ramp.light1").unwrap(),
    ];
    let missing = accent
        .set_accent(Some(Accent::new(input(0.1))))
        .unwrap_err();
    let unknown = accent.remove_layer(LayerId::next()).unwrap_err();
    vec![present, missing, unknown]
}

/// S9.3: a reader stating an input under one scheme only leaves the
/// earlier layers' value under the other.
#[test]
fn input_stated_for_one_scheme_keeps_the_other() {
    let accent = InputName::new("accent").unwrap();
    let mut preset = Layer::default();
    preset.bases.inputs.entries.push((
        accent.clone(),
        InputValue::new(SchemeMap::new(color(0.1), color(0.2))),
    ));
    let mut reader = Layer::default();
    reader.bases.inputs.entries.push((
        accent.clone(),
        InputValue::new(SchemeMap::one(Scheme::Dark, color(0.9))),
    ));
    let resolved = |scheme| {
        stack(
            preset.clone(),
            Some((ReaderState::new(scheme, 1.0), reader.clone())),
            Vec::new(),
        )
        .resolve(&mut WindowState::default(), &window())
        .theme
        .inputs
    };
    assert_eq!(resolved(Scheme::Light), [(accent.clone(), color(0.1))]);
    assert_eq!(resolved(Scheme::Dark), [(accent, color(0.9))]);
}

/// S9.2: the accent layer states the preset's accent inputs only: an
/// accent's derived input the preset does not declare, or declares as an
/// input but not as an accent input, is left out (S9.5: an accent may carry
/// more for another preset).
#[test]
fn accent_layer_states_only_the_accent_inputs() {
    let declarations = |inputs: &str| {
        read_preset(&format!(
            "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n[declare]\n\
             inputs = [{inputs}]\naccent_inputs = [\"accent\"]\n"
        ))
        .unwrap()
        .declarations
    };
    let ramp = InputName::new("accent_ramp.light1").unwrap();
    let mut accent = Accent::new(input(0.1));
    accent.derived.push((ramp.clone(), input(0.2)));
    // Not declared at all: the accent is still accepted.
    let mut undeclared = stack(Layer::default(), None, Vec::new());
    undeclared.declarations = declarations("\"accent\"");
    undeclared.set_accent(Some(accent.clone())).unwrap();
    let inputs = |stack: &Stack| {
        stack
            .resolve(&mut WindowState::default(), &window())
            .theme
            .inputs
    };
    assert_eq!(
        inputs(&undeclared),
        [(InputName::new("accent").unwrap(), color(0.1))]
    );
    // Declared as an input, not an accent input: the reader's stays.
    let mut reader = Layer::default();
    reader.bases.inputs.entries.push((ramp.clone(), input(0.7)));
    let mut preset = Layer::default();
    let entry = (InputName::new("accent").unwrap(), input(0.5));
    preset.bases.inputs.entries.push(entry);
    let mut declared = stack(preset, None, Vec::new());
    declared.declarations = declarations("\"accent\", \"accent_ramp.light1\"");
    declared
        .set_reader_stand_in(ReaderState::new(Scheme::Light, 1.0), reader)
        .unwrap();
    declared.set_accent(Some(accent)).unwrap();
    assert_eq!(
        inputs(&declared),
        [
            (ramp, color(0.7)),
            (InputName::new("accent").unwrap(), color(0.1)),
        ]
    );
}

/// S16's no-reader state: a text-scaling factor of 1; a stand-in's own
/// factor where one is set.
#[test]
fn no_reader_scales_text_by_one() {
    let mut stack = stack(Layer::default(), None, Vec::new());
    let factor = |stack: &Stack| {
        stack
            .resolve(&mut WindowState::default(), &window())
            .theme
            .text_scaling_factor
    };
    assert_eq!(factor(&stack), 1.0);
    stack
        .set_reader_stand_in(ReaderState::new(Scheme::Light, 1.5), Layer::default())
        .unwrap();
    assert_eq!(factor(&stack), 1.5);
}

/// S7.1: exactly one declared size of a container holds; a reported size
/// the preset does not declare leaves the declared default.
#[test]
fn undeclared_container_size_takes_the_default() {
    let button = WidgetRef::Typed(WidgetKind::Button);
    let size = |s| Condition::ContainerSize {
        container: button.clone(),
        size: Key::new(s).unwrap(),
    };
    let mut preset = body(0.1);
    preset.rules.push(body_rule(size("compact"), 0.3));
    preset.rules.push(body_rule(size("regular"), 0.5));
    let opacities = |reported: &str| {
        let mut state = ReaderState::new(Scheme::Light, 1.0);
        state
            .container_sizes
            .push((button.clone(), Key::new(reported).unwrap()));
        let mut stack = stack(preset.clone(), Some((state, Layer::default())), Vec::new());
        stack.declarations.container_sizes =
            vec![(button.clone(), vec![key!("compact"), key!("regular")])];
        let theme = stack.resolve(&mut WindowState::default(), &window()).theme;
        let mut slot = PartSlot::default();
        theme
            .part_steps(TypedPart::Button(ButtonPart::Body))
            .iter()
            .map(|step| {
                slot.apply(step.patch.clone()).unwrap();
                stated_opacity(&slot).unwrap()
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(opacities("regular"), [0.1, 0.5]);
    assert_eq!(opacities("huge"), [0.1, 0.3]);
}

/// A window's record shares the stack's layers, copying none, and a
/// reader's accent left out or applied (its own preset, a stand-in) is a
/// theme change (S8, S9.5).
#[test]
fn window_record_shares_the_layers() {
    let stack = stack(body(0.1), None, vec![body(0.2)]);
    let mut state = WindowState::default();
    let _ = stack.resolve(&mut state, &window());
    let last = state.last.as_ref().unwrap();
    assert!(Arc::ptr_eq(&last.layers[0], &stack.preset_layer));
    assert!(Arc::ptr_eq(&last.layers[1], &stack.layers[0].1));
    let mut now = last.clone();
    now.accent_left_out = !last.accent_left_out;
    assert_eq!(super::classify(last, &now), Change::ThemeChange);
}

/// The layers' places in the stack's order (S8; ADD-A132): on `kde-breeze`
/// the accent-derivation layer lies directly above the preset, beneath the
/// reader or stand-in, so every layer above it lies one place higher, with
/// a reader, a stand-in or neither; on every other preset there is none.
/// The synthetic route's places, a window's record and the resolution's
/// synthetic places follow them.
#[test]
fn accent_derivation_layer_places() -> Result<(), Box<dyn std::error::Error>> {
    use crate::validate::LayerKind as K;
    let kinds = |stack: &Stack| stack.stacked().iter().map(|s| s.kind).collect::<Vec<_>>();
    // The synthetic route's stack of the bundled preset `id`, with the
    // number of layers it adds.
    let synthetic = |id: PresetId| -> Result<(Stack, usize), Box<dyn std::error::Error>> {
        let file = read_preset(id.source())?;
        let layers = crate::gate::synthetic_layers(&file.pending, &file.layer)?;
        let count = layers.len();
        let stack = Stack::synthetic(
            Some(id),
            file.declarations,
            file.layer,
            Scheme::Light,
            file.pending,
            layers,
        )?;
        Ok((stack, count))
    };
    // The stack's order as `stacked`, a window's record and the resolution
    // see it, with the synthetic route's places `places`.
    let pinned = |stack: &Stack, expected: &[K], places: &[usize]| {
        assert_eq!(kinds(stack), expected);
        assert_eq!(stack.synthetic_places(), places);
        let recorded = stack.recorded(&window()).layers;
        let stacked = stack.stacked();
        assert_eq!(recorded.len(), stacked.len());
        for (shared, stacked) in recorded.iter().zip(&stacked) {
            assert!(std::ptr::eq(Arc::as_ptr(shared), stacked.layer));
        }
        assert_eq!(stack.resolved(&window()).synthetic, places);
    };
    let applications = |count: usize| vec![K::Application; count];
    let stand_in = || ReaderState::new(Scheme::Dark, 1.0);

    let (mut breeze, count) = synthetic(PresetId::KdeBreeze)?;
    assert!(count > 0);
    let after = |first: usize| (first..first + count).collect::<Vec<_>>();
    let order = |below: &[K], above: usize| {
        let mut order = below.to_vec();
        order.extend(applications(count + above));
        order.push(K::Accent);
        order
    };
    pinned(
        &breeze,
        &order(&[K::Preset, K::AccentDerivation], 0),
        &after(2),
    );
    let mut read = breeze.clone();
    read.set_reader(stand_in(), Layer::default(), Some(PresetId::KdeBreeze))?;
    pinned(
        &read,
        &order(&[K::Preset, K::AccentDerivation, K::Reader], 0),
        &after(3),
    );
    breeze.set_reader_stand_in(stand_in(), Layer::default())?;
    assert_eq!(breeze.reader_place(), 2);
    breeze.add_layer(Layer::default())?;
    pinned(
        &breeze,
        &order(&[K::Preset, K::AccentDerivation, K::Reader], 1),
        &after(3),
    );

    let (mut adwaita, count) = synthetic(PresetId::Adwaita)?;
    let after = |first: usize| (first..first + count).collect::<Vec<_>>();
    let order = |below: &[K]| {
        let mut order = below.to_vec();
        order.extend(applications(count));
        order.push(K::Accent);
        order
    };
    pinned(&adwaita, &order(&[K::Preset]), &after(1));
    adwaita.set_reader_stand_in(stand_in(), Layer::default())?;
    assert_eq!(adwaita.reader_place(), 1);
    pinned(&adwaita, &order(&[K::Preset, K::Reader]), &after(2));

    // Off the synthetic route and with no preset id: no derivation.
    let mut plain = stack(body(0.1), None, vec![body(0.2)]);
    pinned(&plain, &[K::Preset, K::Application, K::Accent], &[]);
    plain.set_reader_stand_in(stand_in(), Layer::default())?;
    pinned(
        &plain,
        &[K::Preset, K::Reader, K::Application, K::Accent],
        &[],
    );
    Ok(())
}

/// Colours resolved, each by its path.
type Resolved = Vec<(String, Option<Color>)>;

/// What `stack` resolves to at `scheme` at a window's root (the scheme
/// picked, the accent-derivation layer derived for it): every key's role of
/// every declared set and each declared `[WM]` keyed role (S9.1's list as
/// under a tinting scheme), then every inactive input role, each by its
/// path.
fn derived_values(stack: &Stack, scheme: Scheme) -> Result<Resolved, Box<dyn std::error::Error>> {
    use crate::blocks::part_style::RoleRef;
    use crate::units::{Px, Size};
    use crate::validate::AccentRole;
    let mut stack = stack.clone();
    stack.picked = Some(scheme);
    stack.derive_derivation();
    let resolved = stack.resolved(&window());
    let size = Size {
        width: Px(800.0),
        height: Px(600.0),
    };
    let root = crate::query::RootInputs::new(size, size, crate::selector::Direction::Ltr);
    let scope = resolved.root_scope(&root);
    let mut out = Vec::new();
    for role in crate::validate::accent_dependent_roles(stack.declarations(), true) {
        let reference = match role {
            AccentRole::Surface(set, key) => RoleRef::Set(set, key.role()),
            AccentRole::Wm(key) => RoleRef::Keyed(Key::new(&format!("wm_{}", key.name()))?),
        };
        out.push((role.path(), resolved.role(&scope, reference)));
    }
    for (name, _, _) in super::inactive_inputs(stack.declarations()) {
        let input = crate::expr::Ref::Input(name.segments().to_vec());
        let value = resolved.evaluate_role(&scope, &input).value;
        out.push((format!("inputs.{name}"), value));
    }
    Ok(out)
}

/// The paths at which two lists of [`derived_values`] differ, in order.
fn differ(a: &[(String, Option<Color>)], b: &[(String, Option<Color>)]) -> Vec<String> {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b)
        .filter(|(a, b)| a != b)
        .map(|((path, _), _)| path.clone())
        .collect()
}

/// `kde-breeze`'s accent-derivation layer (S8, S9.2; ADD-A132): with no
/// stated accent it states nothing; under `set_accent` the `Accent`'s
/// derivation shows at both schemes; an accent for one scheme leaves the
/// other as without it; a stand-in's accent equals `set_accent` of it but
/// for the roles the stand-in states; a reader-shaped stand-in (its accent
/// common, its results in its part for the preset at its scheme) shows its
/// results at its scheme and the derivation at the other; under
/// `set_accent` a stand-in's own accent-dependent roles and inactive input
/// roles are left out at both schemes, its other roles kept; and
/// validation reads that leave-out (S12): a stand-in whose left-out link
/// and input read themselves is refused alone and kept under the
/// `Accent`, which then cannot be cleared. Every colour but the accent,
/// the accent run's `#e5a50a`, is synthetic.
#[test]
fn accent_derivation_layer() -> Result<(), Box<dyn std::error::Error>> {
    use Scheme::{Dark, Light};
    let file = read_preset(PresetId::KdeBreeze.source())?;
    let layers = crate::gate::synthetic_layers(&file.pending, &file.layer)?;
    let plain = Stack::synthetic(
        Some(PresetId::KdeBreeze),
        file.declarations,
        file.layer,
        Light,
        file.pending,
        layers,
    )?;
    let accent = |schemes: &[Scheme], hex: &str| -> Result<Accent, Box<dyn std::error::Error>> {
        let mut per_scheme = SchemeMap::default();
        for scheme in schemes {
            *per_scheme.slot(*scheme) =
                Some(Value::Stated(Color::parse(hex).ok_or("not a colour")?));
        }
        Ok(Accent::new(InputValue::new(per_scheme)))
    };
    let at_light = || ReaderState::new(Light, 1.0);
    let both = [Light, Dark];
    let x = "#e5a50a";

    assert_eq!(plain.derivation.as_deref(), Some(&Layer::default()));
    assert_eq!(plain.reader_left_out, None);
    let mut quiet = plain.clone();
    quiet.set_reader_stand_in(
        at_light(),
        Layer::from_toml("[colors.light.surfaces.view]\nlink = \"#102030\"\n")?,
    )?;
    assert_eq!(quiet.derivation.as_deref(), Some(&Layer::default()));
    assert_eq!(quiet.reader_left_out, None);

    let plain_light = derived_values(&plain, Light)?;
    let plain_dark = derived_values(&plain, Dark)?;
    let mut set = plain.clone();
    set.set_accent(Some(accent(&both, x)?))?;
    let set_light = derived_values(&set, Light)?;
    let set_dark = derived_values(&set, Dark)?;
    let link = "surfaces.view.link".to_owned();
    assert!(differ(&set_light, &plain_light).contains(&link));
    assert!(differ(&set_dark, &plain_dark).contains(&link));

    let mut one = plain.clone();
    one.set_accent(Some(accent(&[Light], x)?))?;
    assert_eq!(derived_values(&one, Light)?, set_light);
    assert_eq!(derived_values(&one, Dark)?, plain_dark);

    let mut stood = plain.clone();
    stood.set_reader_stand_in(
        at_light(),
        Layer::from_toml(&format!(
            "[inputs.accent]\nlight = \"{x}\"\ndark = \"{x}\"\n\
             [colors.light.surfaces.view]\nlink = \"#102030\"\n\
             [colors.dark.keyed]\nwm_active_background = \"#203040\"\n"
        ))?,
    )?;
    assert_eq!(stood.reader_left_out, None);
    assert_eq!(differ(&derived_values(&stood, Light)?, &set_light), [link]);
    assert_eq!(
        differ(&derived_values(&stood, Dark)?, &set_dark),
        ["keyed.wm_active_background"]
    );

    let mut shaped = plain.clone();
    shaped.set_reader_stand_in(
        at_light(),
        Layer::from_toml(&format!(
            "[inputs.accent]\nlight = \"{x}\"\ndark = \"{x}\"\n\
             [for_preset.kde-breeze.colors.light.surfaces.selection]\n\
             background = \"#304050\"\n\
             [for_preset.kde-breeze.inputs.inactive.view.foreground_link]\n\
             light = \"#405060\"\n"
        ))?,
    )?;
    assert_eq!(
        differ(&derived_values(&shaped, Light)?, &set_light),
        [
            "surfaces.selection.background",
            "inputs.inactive.view.foreground_link"
        ]
    );
    assert_eq!(derived_values(&shaped, Dark)?, set_dark);

    let mut over = plain.clone();
    over.set_reader_stand_in(
        at_light(),
        Layer::from_toml(
            "[inputs.accent]\nlight = \"#1d99f3\"\ndark = \"#1d99f3\"\n\
             [colors.light.surfaces.view]\nlink = \"#102030\"\nbackground = \"#506070\"\n\
             [colors.dark.keyed]\nwm_active_background = \"#203040\"\n\
             [inputs.inactive.view.foreground_link]\n\
             light = \"#405060\"\ndark = \"#405060\"\n",
        )?,
    )?;
    assert_eq!(over.reader_left_out, None);
    over.set_accent(Some(accent(&both, x)?))?;
    assert!(over.reader_left_out.is_some());
    assert_eq!(
        differ(&derived_values(&over, Light)?, &set_light),
        ["surfaces.view.background"]
    );
    assert_eq!(derived_values(&over, Dark)?, set_dark);

    let cycle = Layer::from_toml(
        "[colors.light.surfaces.view]\n\
         link = \"= role.view.link_visited\"\nlink_visited = \"= role.view.link\"\n\
         [inputs.inactive.view.foreground_link]\n\
         light = \"= input.inactive.view.foreground_link\"\n",
    )?;
    let mut alone = plain.clone();
    assert!(
        alone
            .set_reader_stand_in(at_light(), cycle.clone())
            .is_err()
    );
    let mut kept = set.clone();
    kept.set_reader_stand_in(at_light(), cycle)?;
    assert!(kept.set_accent(None).is_err());
    Ok(())
}

/// A stand-in stating a surface set whole on `kde-breeze`, a block and not
/// field by field (here the preset's own `view` block at light), which S8's
/// per-role leave-out cannot reach (S8, S9.2, S12): kept while no layer
/// above it gives the accent, refused under an `Accent`, so `set_accent`
/// is refused over it and the stand-in under it, each leaving the stack as
/// it was.
#[test]
fn a_set_stated_whole_beneath_the_derivation() -> Result<(), Box<dyn std::error::Error>> {
    use crate::blocks::BlockPatch;
    use crate::roles::{ColorRolesPatch, SurfaceSet};
    let file = read_preset(PresetId::KdeBreeze.source())?;
    let view = file
        .layer
        .bases
        .colors
        .light
        .as_ref()
        .and_then(|roles| roles.surfaces.as_ref())
        .and_then(|surfaces| surfaces.get(SurfaceSet::View))
        .cloned()
        .ok_or("the preset states its view set")?;
    assert!(matches!(view, BlockPatch::Set(Value::Stated(_))));
    let mut surfaces = crate::value::SurfaceMap::default();
    surfaces.insert(SurfaceSet::View, view);
    let mut whole = Layer::default();
    whole.bases.colors.light = Some(ColorRolesPatch {
        surfaces: Some(surfaces),
        ..ColorRolesPatch::default()
    });
    let layers = crate::gate::synthetic_layers(&file.pending, &file.layer)?;
    let plain = Stack::synthetic(
        Some(PresetId::KdeBreeze),
        file.declarations,
        file.layer,
        Scheme::Light,
        file.pending,
        layers,
    )?;
    let accent = Color::parse("#e5a50a").ok_or("not a colour")?;
    let accent = Accent::new(InputValue::new(SchemeMap::new(
        Value::Stated(accent),
        Value::Stated(accent),
    )));
    let named = |error: LayerError| match error {
        LayerError::Invalid(problems) => problems.iter().map(ToString::to_string).collect(),
        other => vec![other.to_string()],
    };
    let refusal = [
        "colors.light.surfaces.view: the surface set `view` stated whole where its \
         accent-dependent roles are left out; state its roles one by one",
    ];

    let mut stood = plain.clone();
    stood.set_reader_stand_in(ReaderState::new(Scheme::Light, 1.0), whole.clone())?;
    let before = stood.clone();
    let refused = stood
        .set_accent(Some(accent.clone()))
        .err()
        .ok_or("set_accent kept")?;
    assert_eq!(named(refused), refusal);
    assert_eq!(stood, before);

    let mut accented = plain;
    accented.set_accent(Some(accent))?;
    let before = accented.clone();
    let refused = accented
        .set_reader_stand_in(ReaderState::new(Scheme::Light, 1.0), whole)
        .err()
        .ok_or("the stand-in kept")?;
    assert_eq!(named(refused), refusal);
    assert_eq!(accented, before);
    Ok(())
}

// === Palettes (S8, S14) ===

/// Colours resolved, each by its path, as 8-bit sRGB.
type Colours = Vec<(String, Option<[u8; 4]>)>;

/// What `stack` resolves to at a window's root, the window `inactive` or
/// not: every role of every declared set, every global role, every
/// declared keyed role, then every declared input, each by its path, as
/// 8-bit sRGB (a port's file values and the preset's TOML hex are one
/// 8-bit colour each, computed ones compared at that precision).
fn every_colour(stack: &Stack, inactive: bool) -> Result<Colours, Box<dyn std::error::Error>> {
    use crate::blocks::part_style::RoleRef;
    use crate::roles::{GlobalRole, SurfaceRole};
    use crate::units::{Px, Size};
    let mut conditions = window();
    conditions.inactive = inactive;
    let resolved = stack.resolved(&conditions);
    let size = Size {
        width: Px(800.0),
        height: Px(600.0),
    };
    let root = crate::query::RootInputs::new(size, size, crate::selector::Direction::Ltr);
    let scope = resolved.root_scope(&root);
    let decl = stack.declarations();
    let mut roles: Vec<(String, RoleRef)> = Vec::new();
    for set in &decl.surface_sets {
        for role in SurfaceRole::ALL {
            let path = format!("surfaces.{}.{}", set.name(), role.path());
            roles.push((path, RoleRef::Set(*set, *role)));
        }
    }
    for role in GlobalRole::ALL {
        roles.push((
            format!("global.{}", role.path()),
            RoleRef::Global(role.clone()),
        ));
    }
    for key in &decl.color_roles {
        roles.push((format!("keyed.{key}"), RoleRef::Keyed(key.clone())));
    }
    let mut out: Colours = roles
        .into_iter()
        .map(|(path, role)| {
            let color = resolved.role(&scope, role);
            (path, color.map(Color::to_srgb8_clamped))
        })
        .collect();
    for name in &decl.inputs {
        let input = crate::expr::Ref::Input(name.segments().to_vec());
        let color = resolved.evaluate_role(&scope, &input).value;
        out.push((format!("inputs.{name}"), color.map(Color::to_srgb8_clamped)));
    }
    Ok(out)
}

/// The paths at which two lists of [`every_colour`] differ, in order, each
/// with both colours.
fn colours_differ(a: &Colours, b: &Colours) -> Vec<String> {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b)
        .filter(|(a, b)| a != b)
        .map(|((path, a), (_, b))| format!("{path}: {a:?} / {b:?}"))
        .collect()
}

/// The paths alone of [`colours_differ`].
fn paths_differ(a: &Colours, b: &Colours) -> Vec<String> {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b)
        .filter(|(a, b)| a != b)
        .map(|((path, _), _)| path.clone())
        .collect()
}

/// `kde-breeze` on the synthetic route at `scheme`, with no reader.
fn breeze_at(scheme: Scheme) -> Result<Stack, Box<dyn std::error::Error>> {
    let file = read_preset(PresetId::KdeBreeze.source())?;
    let layers = crate::gate::synthetic_layers(&file.pending, &file.layer)?;
    Ok(Stack::synthetic(
        Some(PresetId::KdeBreeze),
        file.declarations,
        file.layer,
        scheme,
        file.pending,
        layers,
    )?)
}

/// `from_port` of Breeze's file `name` at `scheme`, from the source cache
/// (`palette::tests::breeze_file`, which fails naming the entry where the
/// cache lacks it).
fn breeze_port(name: &str, scheme: Scheme) -> Result<crate::Palette, Box<dyn std::error::Error>> {
    let file = crate::palette::tests::breeze_file(name);
    Ok(crate::Palette::from_port(&file, scheme)?)
}

/// The place in the stack's order of the layer the input `path` in force
/// is taken from at the scheme in force (S8 step 1).
fn input_place(stack: &Stack, path: &str) -> Result<Option<usize>, Box<dyn std::error::Error>> {
    let name = InputName::new(path)?;
    let resolved = stack.resolved(&window());
    Ok(resolved
        .input_layers
        .iter()
        .find(|(held, _)| *held == name)
        .map(|(_, layer)| *layer))
}

/// The place of the palette's entry in the stack's order.
fn palette_place(stack: &Stack) -> Option<usize> {
    stack
        .stacked()
        .iter()
        .position(|stacked| stacked.kind == crate::validate::LayerKind::Palette)
}

/// `kde-breeze` with `from_port` of Breeze's own file at its scheme applied
/// (S14) resolves as the preset there: every colour role and input, in an
/// active and an inactive window, the port's values in force (its scheme
/// inputs, lists and inactive input roles, its `= active(…)` ones accepted
/// by the cycle check, ADD-A144), so the conversion with no accent is
/// Plasma's look of the file.
#[test]
fn a_breeze_port_resolves_as_kde_breeze() -> Result<(), Box<dyn std::error::Error>> {
    for (name, scheme) in [
        ("BreezeLight.colors", Scheme::Light),
        ("BreezeDark.colors", Scheme::Dark),
    ] {
        let plain = breeze_at(scheme)?;
        let mut ported = plain.clone();
        ported.apply_palette(Some(breeze_port(name, scheme)?))?;
        let place = palette_place(&ported).ok_or("no palette entry")?;
        assert_eq!(
            input_place(&ported, "kde_scheme.view.background_normal")?,
            Some(place),
            "{name}"
        );
        assert_eq!(
            input_place(&ported, "inactive.view.foreground_link")?,
            Some(place),
            "{name}"
        );
        for inactive in [false, true] {
            let differ = colours_differ(
                &every_colour(&plain, inactive)?,
                &every_colour(&ported, inactive)?,
            );
            assert_eq!(differ, Vec::<String>::new(), "{name}, inactive: {inactive}");
        }
    }
    Ok(())
}

/// The palette sits among the application layers where it was first
/// added (S8): a layer added after it goes above it, a palette replacing it
/// keeps its place, `apply_palette(None)` removes it and is a no-op
/// without one.
#[test]
fn a_palette_keeps_the_place_it_was_first_added_at() -> Result<(), Box<dyn std::error::Error>> {
    use crate::validate::LayerKind as K;
    let kinds = |stack: &Stack| stack.stacked().iter().map(|s| s.kind).collect::<Vec<_>>();
    let mut stack = breeze_at(Scheme::Light)?;
    let below = kinds(&stack);
    stack.add_layer(Layer::default())?;
    stack.apply_palette(Some(breeze_port("BreezeLight.colors", Scheme::Light)?))?;
    stack.add_layer(Layer::default())?;
    let place = palette_place(&stack).ok_or("no palette entry")?;
    let mut expected = below.clone();
    let accent = expected.pop();
    expected.extend([K::Application, K::Palette, K::Application]);
    expected.extend(accent);
    assert_eq!(kinds(&stack), expected);
    stack.apply_palette(Some(breeze_port("BreezeLight.colors", Scheme::Light)?))?;
    assert_eq!(palette_place(&stack), Some(place));
    stack.apply_palette(None)?;
    assert_eq!(palette_place(&stack), None);
    assert_eq!(kinds(&stack).len(), expected.len() - 1);
    let before = stack.clone();
    stack.apply_palette(None)?;
    assert_eq!(stack, before);
    Ok(())
}

/// S14's refusals, each leaving the stack as it was: a palette under a
/// scheme it lacks and `pick(Some(s))` of one (`SchemeNotInPalette`); a
/// palette with no source for the preset, and `set_preset` to one under it
/// (`PaletteNotForPreset`). `pick(None)` is no refusal: following a scheme
/// the palette lacks suspends it.
#[test]
fn a_palette_s_refusals() -> Result<(), Box<dyn std::error::Error>> {
    let light = || breeze_port("BreezeLight.colors", Scheme::Light);
    let mut dark = breeze_at(Scheme::Dark)?;
    let before = dark.clone();
    assert_eq!(
        dark.apply_palette(Some(light()?)),
        Err(LayerError::SchemeNotInPalette(Scheme::Dark))
    );
    assert_eq!(dark, before);

    let mut ported = breeze_at(Scheme::Light)?;
    ported.apply_palette(Some(light()?))?;
    let before = ported.clone();
    assert_eq!(
        ported.pick(Some(Scheme::Dark)),
        Err(LayerError::SchemeNotInPalette(Scheme::Dark))
    );
    assert_eq!(ported, before);
    ported.pick(Some(Scheme::Light))?;
    ported.pick(None)?;

    let adwaita = read_preset(PresetId::Adwaita.source())?;
    let before = ported.clone();
    assert_eq!(
        ported.set_preset(
            Some(PresetId::Adwaita),
            adwaita.declarations,
            adwaita.layer,
            adwaita.pending
        ),
        Err(LayerError::PaletteNotForPreset(PresetId::Adwaita))
    );
    assert_eq!(ported, before);

    let file = read_preset(PresetId::Adwaita.source())?;
    let layers = crate::gate::synthetic_layers(&file.pending, &file.layer)?;
    let mut on_adwaita = Stack::synthetic(
        Some(PresetId::Adwaita),
        file.declarations,
        file.layer,
        Scheme::Light,
        file.pending,
        layers,
    )?;
    let before = on_adwaita.clone();
    assert_eq!(
        on_adwaita.apply_palette(Some(light()?)),
        Err(LayerError::PaletteNotForPreset(PresetId::Adwaita))
    );
    assert_eq!(on_adwaita, before);
    Ok(())
}

/// S8's leave-out beneath a KDE port, and the user's accent under it
/// (S8, S14, R2.6; ADD-A142), with a reader-shaped stand-in (its accent
/// common, its scheme inputs, a list, roles and `token.kde` in its part for
/// the preset): at the port's scheme its colours, inputs, lists and
/// `token.kde` are left out, its accent kept in force (A7's palette case:
/// a port states no accent, so the reader's or stand-in's is not left
/// out), and the accent-derivation layer's roles over it show in the
/// port's place, so the stack resolves as `set_accent` of that accent over
/// the port; the port's own accent-dependent roles are left out, and
/// differ from the port's no-accent look. With no accent the stand-in's
/// colours are left out whole, the port's look shown. At a scheme the
/// palette lacks it is suspended, and under a contrast it states no
/// colours for it is skipped: the stand-in's colours apply as without it.
/// Every colour but the accent, the accent run's `#e5a50a`, is synthetic.
#[test]
fn a_port_leaves_the_stand_in_out_but_its_accent() -> Result<(), Box<dyn std::error::Error>> {
    use Scheme::{Dark, Light};
    let x = "#e5a50a";
    let light = || breeze_port("BreezeLight.colors", Light);
    let shaped = |accent: bool| -> Result<Layer, Box<dyn std::error::Error>> {
        let accent = if accent {
            format!("[inputs.accent]\nlight = \"{x}\"\ndark = \"{x}\"\n")
        } else {
            String::new()
        };
        Ok(Layer::from_toml(&format!(
            "{accent}\
             [for_preset.kde-breeze.inputs.kde_scheme.view.background_normal]\n\
             light = \"#102030\"\ndark = \"#102030\"\n\
             [for_preset.kde-breeze.inputs.kde_scheme.groups]\n\
             light = [\"view\"]\ndark = [\"view\"]\n\
             [for_preset.kde-breeze.inputs.inactive.view.foreground_link]\n\
             light = \"#203040\"\ndark = \"#203040\"\n\
             [for_preset.kde-breeze.colors.light.surfaces.view]\n\
             background = \"#304050\"\nlink = \"#405060\"\n\
             [for_preset.kde-breeze.colors.dark.surfaces.view]\n\
             background = \"#304050\"\nlink = \"#405060\"\n\
             [for_preset.kde-breeze.tokens.kde]\n\
             inactive_enabled = true\n"
        ))?)
    };
    let accent = Color::parse(x).ok_or("not a colour")?;
    let accent = Accent::new(InputValue::new(SchemeMap::new(
        Value::Stated(accent),
        Value::Stated(accent),
    )));

    let mut port = breeze_at(Light)?;
    port.apply_palette(Some(light()?))?;
    let mut set = port.clone();
    set.set_accent(Some(accent))?;
    let mut stood = breeze_at(Light)?;
    stood.set_reader_stand_in(ReaderState::new(Light, 1.0), shaped(true)?)?;
    let alone = stood.clone();
    stood.apply_palette(Some(light()?))?;
    assert_eq!(
        input_place(&stood, "accent")?,
        Some(stood.reader_place()),
        "the stand-in's accent in force"
    );
    assert!(stood.port_left_out.is_some());
    for inactive in [false, true] {
        let stood = every_colour(&stood, inactive)?;
        assert_eq!(
            colours_differ(&stood, &every_colour(&set, inactive)?),
            Vec::<String>::new(),
            "inactive: {inactive}"
        );
        let differ = paths_differ(&stood, &every_colour(&port, inactive)?);
        assert!(
            differ.contains(&"surfaces.view.link".to_owned()),
            "{differ:?}"
        );
        assert!(differ.contains(&"inputs.accent".to_owned()), "{differ:?}");
    }

    let mut quiet = breeze_at(Light)?;
    quiet.set_reader_stand_in(ReaderState::new(Light, 1.0), shaped(false)?)?;
    let quiet_alone = quiet.clone();
    quiet.apply_palette(Some(light()?))?;
    assert_eq!(quiet.port_left_out, None);
    for inactive in [false, true] {
        assert_eq!(
            colours_differ(
                &every_colour(&quiet, inactive)?,
                &every_colour(&port, inactive)?
            ),
            Vec::<String>::new(),
            "inactive: {inactive}"
        );
    }

    let mut suspended = stood.clone();
    suspended.set_reader_stand_in(ReaderState::new(Dark, 1.0), shaped(true)?)?;
    let mut unported = alone.clone();
    unported.set_reader_stand_in(ReaderState::new(Dark, 1.0), shaped(true)?)?;
    assert_eq!(
        colours_differ(
            &every_colour(&suspended, false)?,
            &every_colour(&unported, false)?
        ),
        Vec::<String>::new()
    );

    for contrast in [Contrast::Increased, Contrast::High] {
        let mut skipped = quiet.clone();
        skipped.set_contrast(Some(contrast))?;
        let mut unported = quiet_alone.clone();
        unported.set_contrast(Some(contrast))?;
        assert_eq!(
            colours_differ(
                &every_colour(&skipped, false)?,
                &every_colour(&unported, false)?
            ),
            Vec::<String>::new(),
            "{contrast:?}"
        );
    }
    Ok(())
}
