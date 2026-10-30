//! S8's resolution, S9.2's accent layer and S9.4's conversion. Every
//! number and name here is synthetic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::float_cmp)]

use super::{
    Change, Lazily, Pos, ReaderState, Stack, WindowConditions, WindowState, font_dpi, lazily,
    logical_px, scaled_linear,
};
use crate::key;
use crate::next::blocks::part_style::PartStylePatch;
use crate::next::blocks::text::{
    Elision, Font, FontPatch, FontStyle, TextCase, TextStyle, TextStylePatch, Wrap,
};
use crate::next::blocks::{Apply, BlockPatch};
use crate::next::color::Color;
use crate::next::declare::Declarations;
use crate::next::error::LayerError;
use crate::next::expr::Expr;
use crate::next::inputs::{Accent, Contrast, InputName, InputValue};
use crate::next::layer::Layer;
use crate::next::selector::{Condition, InputKind, Rule, RuleBody, Scheme, Selector};
use crate::next::tokens::{TextRolesPatch, TextTokensPatch, TokensPatch};
use crate::next::toml::layer::read_preset;
use crate::next::units::{FontSize, Fraction};
use crate::next::value::{SchemeMap, Value};
use crate::next::widgets::{ButtonPart, ButtonPatch, PartPatch, PartSlot, TypedPart};

/// A stack of these layers, unvalidated: the resolution alone is tested.
fn stack(preset: Layer, reader: Option<(ReaderState, Layer)>, layers: Vec<Layer>) -> Stack {
    Stack {
        preset: None,
        declarations: Declarations::default(),
        preset_layer: Box::new(preset),
        reader: reader.map(|(state, layer)| super::Reader::StandIn {
            state,
            layer: Box::new(layer),
        }),
        layers,
        accent: None,
        accent_layer: Box::default(),
        picked: None,
        default_scheme: Scheme::Light,
        contrast: None,
        flavor: None,
    }
}

fn window() -> WindowConditions {
    WindowConditions::new(InputKind::Mouse)
}

fn opacity(n: f32) -> PartPatch {
    PartPatch::Style(PartStylePatch {
        opacity: Some(Value::Stated(Fraction(n))),
        ..PartStylePatch::default()
    })
}

/// A layer whose bases set the button body's opacity.
fn body(n: f32) -> Layer {
    let mut layer = Layer::default();
    layer.bases.widgets.button = Some(ButtonPatch {
        body: Some(opacity(n)),
        ..ButtonPatch::default()
    });
    layer
}

/// A rule setting the body's opacity under `condition`.
fn body_rule(condition: Condition, n: f32) -> Rule {
    let mut selector = Selector::default();
    selector.conditions.all_of.push(condition);
    Rule::new(
        selector,
        RuleBody::Part(
            crate::next::selector::PartRef::Typed(TypedPart::Button(ButtonPart::Body)),
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

fn style(size: f32) -> TextStyle {
    TextStyle {
        font: Font {
            family: Value::Stated("Synthetic".to_owned()),
            size: Value::Stated(FontSize::Px(size)),
            weight: Value::Stated(400),
            style: Value::Stated(FontStyle::Normal),
            width: Value::Absent,
            axes: Vec::new(),
            features: Vec::new(),
        },
        scaling_role: None,
        letter_spacing: Value::Absent,
        word_spacing: Value::Absent,
        line_height: Value::Auto,
        decoration: Value::Absent,
        mnemonic_decoration: Value::Absent,
        case: TextCase::Mixed,
        elision: Elision::End,
        wrap: Wrap::Word,
        max_lines: None,
        fill: Value::Absent,
    }
}

fn size(px: f32) -> BlockPatch<TextStyle> {
    BlockPatch::Fields(TextStylePatch {
        font: Some(FontPatch {
            size: Some(Value::Stated(FontSize::Px(px))),
            ..FontPatch::default()
        }),
        ..TextStylePatch::default()
    })
}

/// Tokens stating the keyed style `x` and the `body` slot.
fn text(keyed: Option<BlockPatch<TextStyle>>, body: Option<BlockPatch<TextStyle>>) -> TokensPatch {
    TokensPatch {
        text: Some(TextTokensPatch {
            keyed: keyed.map(|patch| vec![(key!("x"), patch)]),
            roles: body.map(|body| TextRolesPatch {
                body: Some(body),
                ..TextRolesPatch::default()
            }),
            ..TextTokensPatch::default()
        }),
        ..TokensPatch::default()
    }
}

fn tokens(patch: TokensPatch) -> Layer {
    let mut layer = Layer::default();
    layer.bases.tokens = patch;
    layer
}

/// The `body` slot's font size, evaluated as S2.2 composes a `Fields`
/// patch over a reference: the referenced style's steps and the slot's,
/// in `lazily`'s order, the slot's reference itself the place followed.
fn body_size(stack: &Stack) -> FontSize {
    let theme = stack.resolve(&mut WindowState::default(), &window()).theme;
    let keyed = |tokens: &TokensPatch| {
        let keyed = tokens.text.as_ref()?.keyed.as_ref()?;
        keyed.first().map(|(_, patch)| patch.clone())
    };
    let slot = |tokens: &TokensPatch| tokens.text.as_ref()?.roles.as_ref()?.body.clone();
    let mut value: Value<TextStyle> = Value::Absent;
    for item in lazily(&theme.tokens, &theme.tokens) {
        let patch = match item {
            Lazily::Referenced(step) => keyed(&step.patch),
            Lazily::Patch(step) => slot(&step.patch).filter(|patch| {
                // The slot's reference is the place followed, not a patch.
                !matches!(patch, BlockPatch::Set(Value::Expr(_)))
            }),
        };
        if let Some(patch) = patch {
            value.apply(patch).unwrap();
        }
    }
    match value {
        Value::Stated(style) => match style.font.size {
            Value::Stated(size) => size,
            other => panic!("{other:?}"),
        },
        other => panic!("{other:?}"),
    }
}

/// A `Fields` patch over a reference composes lazily (S2.2): the
/// referenced style's bases (the preset's and a later layer's), then the
/// patch stated in a base (a reader's font patch on the slot), before every
/// rule (a preset's `Tokens` rule on the referenced style still reaches
/// the slot).
#[test]
fn fields_patch_composes_lazily_before_rules() {
    let reference = BlockPatch::Set(Value::Expr(Expr::parse("token.text.keyed.x").unwrap()));
    let preset = tokens(text(
        Some(BlockPatch::Set(Value::Stated(style(10.0)))),
        Some(reference),
    ));
    let reader = tokens(text(None, Some(size(14.0))));
    let later = tokens(text(Some(size(16.0)), None));
    let state = ReaderState::new(Scheme::Light, 1.0);
    // The referenced style's later base comes before the slot's patch.
    let without_rule = stack(
        preset.clone(),
        Some((state.clone(), reader.clone())),
        vec![later.clone()],
    );
    assert_eq!(body_size(&without_rule), FontSize::Px(14.0));
    // A rule on the referenced style comes after every base patch.
    let mut ruled = preset;
    let mut selector = Selector::default();
    selector
        .conditions
        .all_of
        .push(Condition::Scheme(Scheme::Light));
    ruled.rules.push(Rule::new(
        selector,
        RuleBody::Tokens(text(Some(size(12.0)), None)),
    ));
    let with_rule = stack(ruled, Some((state, reader)), vec![later]);
    assert_eq!(body_size(&with_rule), FontSize::Px(12.0));
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
    stack.layers = vec![body(0.5)];
    assert_eq!(change(&stack, &mut state, &plain), Change::Unchanged);
    stack.pick(Some(Scheme::Dark)).unwrap();
    assert_eq!(change(&stack, &mut state, &plain), Change::ThemeChange);
    stack.set_contrast(Some(Contrast::High)).unwrap();
    assert_eq!(change(&stack, &mut state, &plain), Change::ThemeChange);
    stack.set_flavor(key!("other")).unwrap();
    assert_eq!(change(&stack, &mut state, &plain), Change::ConditionChange);
    // Both at once: the theme change wins.
    stack.layers.clear();
    assert_eq!(change(&stack, &mut state, &inactive), Change::ThemeChange);
    // The reader's conditions and values.
    let mut read = ReaderState::new(Scheme::Light, 1.0);
    stack.reader = Some(super::Reader::StandIn {
        state: read.clone(),
        layer: Box::default(),
    });
    assert_eq!(change(&stack, &mut state, &inactive), Change::ThemeChange);
    read.reduced_motion = true;
    stack.reader = Some(super::Reader::StandIn {
        state: read.clone(),
        layer: Box::default(),
    });
    assert_eq!(
        change(&stack, &mut state, &inactive),
        Change::ConditionChange
    );
    read.text_scaling_factor = 1.5;
    stack.reader = Some(super::Reader::StandIn {
        state: read.clone(),
        layer: Box::default(),
    });
    assert_eq!(change(&stack, &mut state, &inactive), Change::ThemeChange);
    // The reader's scheme under a picked one changes nothing in force.
    read.scheme = Scheme::Dark;
    stack.reader = Some(super::Reader::StandIn {
        state: read,
        layer: Box::default(),
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
            Scheme::Light
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

fn color(r: f32) -> Value<Color> {
    Value::Stated(Color {
        r,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    })
}

fn input(r: f32) -> InputValue {
    InputValue::new(SchemeMap::new(color(r), color(r)))
}

/// S9.2: the accent layer states the preset's accent inputs; an accent
/// lacking one is refused, naming it; `None` clears the layer. The accent
/// layer comes last, over the reader's input.
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
            (ramp, color(0.2)),
        ]
    );
    stack.set_accent(None).unwrap();
    assert_eq!(
        inputs(&stack),
        [(InputName::new("accent").unwrap(), color(0.9))]
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

/// S9.4: points become logical pixels by `dpi / 72`, the reader's DPI over
/// the preset's token; pixels stay; linear scaling multiplies after the
/// conversion.
#[test]
fn points_to_px_then_linear_scaling() {
    assert_eq!(font_dpi(Some(120.0), Some(96.0)), Some(120.0));
    assert_eq!(font_dpi(None, Some(96.0)), Some(96.0));
    assert_eq!(font_dpi(None, None), None);
    assert_eq!(logical_px(FontSize::Pt(9.0), 96.0), 12.0);
    assert_eq!(logical_px(FontSize::Px(9.0), 96.0), 9.0);
    // Converted, then scaled.
    let dpi = font_dpi(None, Some(96.0)).unwrap();
    assert_eq!(scaled_linear(logical_px(FontSize::Pt(9.0), dpi), 1.5), 18.0);
}

/// The stack's refusals, for `layer_error_every_variant_triggers`.
pub(super) fn refusals() -> Vec<LayerError> {
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
    vec![present, missing]
}
