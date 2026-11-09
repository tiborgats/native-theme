//! S16's mutations of the stack, each accepted and refused, and S12's
//! checks of a reader's or stand-in's state. Every number and name here is
//! synthetic.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::tests::{body, color, input, stack, window};
use super::{ReaderState, Stack, WindowState};
use crate::color::Color;
use crate::declare::Declarations;
use crate::error::LayerError;
use crate::inputs::{Accent, Contrast, InputName, InputValue};
use crate::key;
use crate::layer::{Layer, LayerId};
use crate::selector::{Scheme, WidgetRef};
use crate::toml::layer::read_preset;
use crate::value::{SchemeMap, Value};
use crate::widgets::WidgetKind;

/// A mutation of the stack.
type Mutation = dyn Fn(&mut Stack) -> Result<(), LayerError>;

/// Whether a refusal is the one a mutation expects.
type Refused = fn(&LayerError) -> bool;

/// A preset file's declarations, `declare` its `[declare]` table's body.
fn declarations(declare: &str) -> Declarations {
    read_preset(&format!(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n[declare]\n{declare}"
    ))
    .unwrap()
    .declarations
}

/// A layer stating each of `names` at both schemes, red `r`.
fn stating(names: &[&str], r: f32) -> Layer {
    let mut layer = Layer::default();
    for name in names {
        let entry = (InputName::new(name).unwrap(), input(r));
        layer.bases.inputs.entries.push(entry);
    }
    layer
}

/// A stack declaring the flavours `a` and `b`, and `accent` and
/// `accent_ramp.light1` as its accent inputs, its preset stating both.
fn declared() -> Stack {
    let mut stack = stack(
        stating(&["accent", "accent_ramp.light1"], 0.5),
        None,
        Vec::new(),
    );
    stack.declarations = declarations(
        "flavors = [\"a\", \"b\"]\n\
         inputs = [\"accent\", \"accent_ramp.light1\"]\n\
         accent_inputs = [\"accent\", \"accent_ramp.light1\"]\n",
    );
    stack
}

/// An accent stating both of [`declared`]'s accent inputs.
fn full_accent() -> Accent {
    let mut accent = Accent::new(input(0.1));
    accent
        .derived
        .push((InputName::new("accent_ramp.light1").unwrap(), input(0.2)));
    accent
}

/// The problems of a refusal, each as it displays.
fn problems(result: Result<(), LayerError>) -> Vec<String> {
    match result {
        Err(LayerError::Invalid(problems)) => problems.iter().map(ToString::to_string).collect(),
        other => panic!("not a problem list: {other:?}"),
    }
}

/// Every mutation S16 refuses is refused whole, the stack as it was (S8,
/// S12): a preset change, a stand-in (its state or its layer, or while a
/// reader exists), a flavour, a layer added, replaced or removed, an
/// accent; the stand-in's removal in
/// `stand_in_removal_refused_where_a_layer_needs_it`; a palette's, and
/// `pick` of a scheme it lacks, in `a_palette_s_refusals` (S14).
#[test]
fn mutation_refused_whole() {
    let mut stack = declared();
    stack
        .set_reader_stand_in(ReaderState::new(Scheme::Dark, 1.0), body(0.4))
        .unwrap();
    let id = stack.add_layer(body(0.5)).unwrap();
    stack.set_accent(Some(full_accent())).unwrap();
    stack.set_flavor(key!("b")).unwrap();
    let before = stack.clone();
    let mut bad_state = ReaderState::new(Scheme::Light, 1.0);
    bad_state.font_dpi = Some(0.0);
    let invalid: Refused = |e| matches!(e, LayerError::Invalid(_));
    let accent: Refused = |e| matches!(e, LayerError::AccentInputsMissing(_));
    let unknown: Refused = |e| matches!(e, LayerError::UnknownLayer(_));
    let refusals: Vec<(&str, Refused, Box<Mutation>)> = vec![
        (
            "a preset failing validation",
            invalid,
            Box::new(|stack| {
                stack.set_preset(None, Declarations::default(), Layer::default(), Vec::new())
            }),
        ),
        (
            "a preset whose accent inputs the accent lacks",
            accent,
            Box::new(|stack| {
                let more = declarations(
                    "inputs = [\"accent\", \"accent_ramp.light1\", \"accent_ramp.light2\"]\n\
                     accent_inputs = [\"accent\", \"accent_ramp.light1\", \"accent_ramp.light2\"]\n",
                );
                stack.set_preset(None, more, Layer::default(), Vec::new())
            }),
        ),
        (
            "a stand-in's state",
            invalid,
            Box::new(move |stack| stack.set_reader_stand_in(bad_state.clone(), Layer::default())),
        ),
        (
            "a stand-in's layer",
            invalid,
            Box::new(|stack| {
                stack.set_reader_stand_in(ReaderState::new(Scheme::Light, 1.0), body(2.0))
            }),
        ),
        (
            "an undeclared flavour",
            invalid,
            Box::new(|stack| stack.set_flavor(key!("c"))),
        ),
        (
            "a layer added",
            invalid,
            Box::new(|stack| stack.add_layer(body(2.0)).map(|_| ())),
        ),
        (
            "a layer replaced",
            invalid,
            Box::new(move |stack| stack.replace_layer(id, body(2.0))),
        ),
        (
            "an unknown layer replaced",
            unknown,
            Box::new(|stack| stack.replace_layer(LayerId::next(), body(0.6))),
        ),
        (
            "an unknown layer removed",
            unknown,
            Box::new(|stack| stack.remove_layer(LayerId::next())),
        ),
        (
            "an accent lacking an accent input",
            accent,
            Box::new(|stack| stack.set_accent(Some(Accent::new(input(0.3))))),
        ),
    ];
    for (what, reason, refusal) in refusals {
        match refusal(&mut stack) {
            Err(e) => assert!(reason(&e), "{what} refused for another reason: {e:?}"),
            Ok(()) => panic!("{what} accepted"),
        }
        assert_eq!(stack, before, "{what} changed the stack");
    }
    let mut read = declared();
    read.set_reader(ReaderState::new(Scheme::Light, 1.0), Layer::default(), None)
        .unwrap();
    let before = read.clone();
    assert_eq!(
        read.set_reader_stand_in(ReaderState::new(Scheme::Light, 1.0), Layer::default()),
        Err(LayerError::ReaderPresent)
    );
    assert_eq!(read, before);
}

/// A preset change (S16): refused where the preset fails validation, or
/// the accent set lacks one of its accent inputs, naming it; the flavour
/// set is kept where the new preset declares it, else the new preset's
/// default is in force, none where it declares none.
#[test]
fn preset_change_keeps_a_declared_flavor() {
    let mut stack = declared();
    assert!(matches!(
        stack.set_preset(None, Declarations::default(), Layer::default(), Vec::new()),
        Err(LayerError::Invalid(_))
    ));
    stack.set_accent(Some(Accent::new(input(0.1)))).unwrap_err();
    let mut one = declarations("inputs = [\"accent\"]\naccent_inputs = [\"accent\"]\n");
    stack.declarations = one.clone();
    stack.set_accent(Some(Accent::new(input(0.1)))).unwrap();
    let ramp = InputName::new("accent_ramp.light1").unwrap();
    assert_eq!(
        stack.set_preset(None, declared().declarations, Layer::default(), Vec::new()),
        Err(LayerError::AccentInputsMissing(vec![ramp]))
    );
    let mut stack = declared();
    stack.set_flavor(key!("b")).unwrap();
    let mut kept = stack.clone();
    kept.switch(
        None,
        declarations("flavors = [\"c\", \"b\"]\n"),
        body(0.1),
        Vec::new(),
    );
    assert_eq!(kept.flavor(), Some(&key!("b")));
    assert_eq!(*kept.preset_layer, body(0.1));
    let mut other = stack.clone();
    other.switch(
        None,
        declarations("flavors = [\"c\", \"d\"]\n"),
        Layer::default(),
        Vec::new(),
    );
    assert_eq!(other.flavor(), Some(&key!("c")));
    one.flavors.clear();
    let mut none = stack;
    none.switch(None, one, Layer::default(), Vec::new());
    assert_eq!(none.flavor(), None);
}

/// A flavour the preset declares is set and in force; an undeclared one is
/// refused, naming it and the declaration (S10, S16). Without one set, the
/// preset's first is in force.
#[test]
fn flavor_set_where_declared() {
    let mut flavored = declared();
    assert_eq!(flavored.flavor(), Some(&key!("a")));
    flavored.set_flavor(key!("b")).unwrap();
    assert_eq!(flavored.flavor(), Some(&key!("b")));
    assert_eq!(
        problems(flavored.set_flavor(key!("c"))),
        ["`c` is not declared in `flavors`"]
    );
    let mut none = stack(Layer::default(), None, Vec::new());
    assert_eq!(none.flavor(), None);
    assert_eq!(
        problems(none.set_flavor(key!("a"))),
        ["`a` is not declared in `flavors`"]
    );
}

/// Application layers by id (S16): added last, replaced in place, removed;
/// an id the stack does not hold, removed already or another stack's, is
/// `UnknownLayer`; a layer failing validation is refused.
#[test]
fn layers_added_replaced_removed_by_id() {
    let mut stack = declared();
    let first = stack.add_layer(body(0.1)).unwrap();
    let second = stack.add_layer(body(0.2)).unwrap();
    assert_ne!(first, second);
    stack.replace_layer(first, body(0.3)).unwrap();
    let held = |stack: &Stack| {
        stack
            .layers
            .iter()
            .map(|(id, layer)| (*id, (**layer).clone()))
            .collect::<Vec<_>>()
    };
    assert_eq!(held(&stack), [(first, body(0.3)), (second, body(0.2))]);
    assert!(matches!(
        stack.add_layer(body(2.0)),
        Err(LayerError::Invalid(_))
    ));
    stack.remove_layer(first).unwrap();
    assert_eq!(held(&stack), [(second, body(0.2))]);
    assert_eq!(
        stack.remove_layer(first),
        Err(LayerError::UnknownLayer(first))
    );
    assert_eq!(
        stack.replace_layer(first, body(0.4)),
        Err(LayerError::UnknownLayer(first))
    );
    let mut another = declared();
    let theirs = another.add_layer(body(0.5)).unwrap();
    assert_eq!(
        stack.remove_layer(theirs),
        Err(LayerError::UnknownLayer(theirs))
    );
}

/// The stand-in (S16): set, replaced, then removed back to the no-reader
/// state, whose scheme is the stack's own and which `system()` reads as
/// `None`; removing none is a no-op.
#[test]
fn stand_in_set_replaced_and_removed() {
    let mut stack = declared();
    assert_eq!(stack.remove_reader_stand_in(), Ok(()));
    assert_eq!(stack.reader_state(), None);
    stack
        .set_reader_stand_in(ReaderState::new(Scheme::Dark, 1.0), body(0.1))
        .unwrap();
    assert_eq!(stack.scheme(), Scheme::Dark);
    let mut state = ReaderState::new(Scheme::Dark, 1.25);
    state.font_dpi = Some(120.0);
    stack.set_reader_stand_in(state.clone(), body(0.2)).unwrap();
    assert_eq!(stack.reader_state(), Some(&state));
    stack.remove_reader_stand_in().unwrap();
    assert_eq!(stack.reader_state(), None);
    assert_eq!(stack.scheme(), Scheme::Light);
    let theme = stack.resolve(&mut WindowState::default(), &window()).theme;
    assert_eq!(theme.font_dpi, None);
}

/// `pick` and `set_contrast` (S16): a picked scheme or set contrast is in
/// force over the stand-in's; `None` follows the stand-in's again.
#[test]
fn pick_and_contrast_over_the_stand_in() {
    let mut stack = declared();
    let mut state = ReaderState::new(Scheme::Dark, 1.0);
    state.contrast = Some(Contrast::Increased);
    stack.set_reader_stand_in(state, Layer::default()).unwrap();
    stack.pick(Some(Scheme::Light)).unwrap();
    assert_eq!(stack.scheme(), Scheme::Light);
    stack.pick(None).unwrap();
    assert_eq!(stack.scheme(), Scheme::Dark);
    stack.set_contrast(Some(Contrast::High)).unwrap();
    assert_eq!(stack.contrast(), Some(Contrast::High));
    stack.set_contrast(None).unwrap();
    assert_eq!(stack.contrast(), Some(Contrast::Increased));
}

/// S12: a reader's or stand-in's text scaling factor, and its font DPI
/// where it reports one, is finite and above zero (S9.4).
#[test]
fn reader_state_factors_finite_and_above_zero() {
    let refused =
        |state: ReaderState| problems(declared().set_reader_stand_in(state, Layer::default()));
    let factor = |n: f32| ReaderState::new(Scheme::Light, n);
    assert_eq!(
        refused(factor(f32::NAN)),
        ["text_scaling_factor: not a finite number"]
    );
    assert_eq!(
        refused(factor(0.0)),
        ["text_scaling_factor: out of range: above zero"]
    );
    assert_eq!(
        refused(factor(-1.0)),
        ["text_scaling_factor: out of range: above zero"]
    );
    let dpi = |n: f32| {
        let mut state = factor(1.0);
        state.font_dpi = Some(n);
        state
    };
    assert_eq!(
        refused(dpi(f32::INFINITY)),
        ["font_dpi: not a finite number"]
    );
    assert_eq!(refused(dpi(0.0)), ["font_dpi: out of range: above zero"]);
    declared()
        .set_reader_stand_in(dpi(96.0), Layer::default())
        .unwrap();
    declared()
        .set_reader_stand_in(factor(0.5), Layer::default())
        .unwrap();
}

/// S12: a reader's or stand-in's `container_sizes` names each container
/// once, as it reports one size at a time.
#[test]
fn reader_state_names_a_container_once() {
    let button = WidgetRef::Typed(WidgetKind::Button);
    let mut state = ReaderState::new(Scheme::Light, 1.0);
    state
        .container_sizes
        .push((button.clone(), key!("compact")));
    state
        .container_sizes
        .push((WidgetRef::Component(key!("rail")), key!("compact")));
    declared()
        .set_reader_stand_in(state.clone(), Layer::default())
        .unwrap();
    state.container_sizes.push((button, key!("regular")));
    assert_eq!(
        problems(declared().set_reader_stand_in(state, Layer::default())),
        ["container_sizes[2]: `button` listed twice"]
    );
}

/// Removing the stand-in validates the stack again (S8, S12): an
/// application layer's call reading a touch target only the stand-in
/// states, over the preset's `absent`, is refused without it, and the
/// stand-in stays.
#[test]
fn stand_in_removal_refused_where_a_layer_needs_it() {
    let preset = Layer::from_toml("[widgets.button.body]\ntouch_target_px = \"absent\"").unwrap();
    let stand_in = Layer::from_toml("[widgets.button.body]\ntouch_target_px = 30").unwrap();
    let application = Layer::from_toml(
        "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
         set = { min_width_px = \"= add(part.touch_target, 4px)\" }",
    )
    .unwrap();
    let mut stack = stack(preset, None, Vec::new());
    stack
        .set_reader_stand_in(ReaderState::new(Scheme::Light, 1.0), stand_in)
        .unwrap();
    stack.add_layer(application).unwrap();
    let before = stack.clone();
    assert_eq!(
        problems(stack.remove_reader_stand_in()),
        ["rules[0].set.min_width_px: `part.touch_target` can be `absent` here"]
    );
    assert_eq!(stack, before);
}

/// The value of the input `name` in force on `stack` under `scheme`.
fn in_force(stack: &Stack, scheme: Scheme, name: &InputName) -> Option<Value<Color>> {
    let mut stack = stack.clone();
    stack.picked = Some(scheme);
    let theme = stack.resolve(&mut WindowState::default(), &window()).theme;
    theme
        .inputs
        .into_iter()
        .find(|(held, _)| held == name)
        .map(|(_, value)| value)
}

/// The preset's accent inputs as one group per scheme (S9.2, ADD-A132): an
/// `Accent` stating the group whole at one scheme is accepted, the other
/// scheme keeping the preset's accent and derived inputs together; one
/// stating a derived input at one scheme and the accent at both is
/// refused, naming the input once; a contrast base at a scheme with no
/// per-scheme value there needs every one's per-scheme value there, which
/// is all that is refused where the group is the `accent` alone.
#[test]
fn accent_inputs_one_group_per_scheme() {
    let accent = InputName::new("accent").unwrap();
    let ramp = InputName::new("accent_ramp.light1").unwrap();
    let light = |r: f32| InputValue::new(SchemeMap::one(Scheme::Light, color(r)));
    let with = |value: InputValue, derived: InputValue| {
        let mut out = Accent::new(value);
        out.derived.push((ramp.clone(), derived));
        out
    };
    let mut stack = declared();
    stack
        .set_accent(Some(with(light(0.1), light(0.2))))
        .unwrap();
    assert_eq!(in_force(&stack, Scheme::Light, &accent), Some(color(0.1)));
    assert_eq!(in_force(&stack, Scheme::Light, &ramp), Some(color(0.2)));
    assert_eq!(in_force(&stack, Scheme::Dark, &accent), Some(color(0.5)));
    assert_eq!(in_force(&stack, Scheme::Dark, &ramp), Some(color(0.5)));
    let before = stack.clone();
    assert_eq!(
        stack.set_accent(Some(with(input(0.1), light(0.2)))),
        Err(LayerError::AccentInputsMissing(vec![ramp.clone()]))
    );
    let mut based = light(0.1);
    based
        .per_contrast
        .push((Contrast::High, SchemeMap::one(Scheme::Dark, color(0.3))));
    assert_eq!(
        stack.set_accent(Some(with(based.clone(), light(0.2)))),
        Err(LayerError::AccentInputsMissing(vec![
            accent.clone(),
            ramp.clone()
        ]))
    );
    assert_eq!(stack, before);
    let mut alone = super::tests::stack(stating(&["accent"], 0.5), None, Vec::new());
    alone.declarations = declarations("inputs = [\"accent\"]\naccent_inputs = [\"accent\"]\n");
    alone.set_accent(Some(Accent::new(light(0.1)))).unwrap();
    assert_eq!(
        alone.set_accent(Some(Accent::new(based))),
        Err(LayerError::AccentInputsMissing(vec![accent]))
    );
}

/// A stand-in states the preset's accent inputs as one group (S9.2,
/// ADD-A132): one stating the `accent` alone is refused where the preset
/// derives others from it, the stack as it was, and accepted where the
/// group is the `accent` alone; one stating the group, or none of it, is
/// accepted; `set_preset` to a preset deriving more from the accent is
/// refused over that stand-in.
#[test]
fn stand_in_states_the_accent_group() {
    let ramp = InputName::new("accent_ramp.light1").unwrap();
    let state = || ReaderState::new(Scheme::Light, 1.0);
    let mut stack = declared();
    let before = stack.clone();
    assert_eq!(
        stack.set_reader_stand_in(state(), stating(&["accent"], 0.7)),
        Err(LayerError::AccentInputsMissing(vec![ramp.clone()]))
    );
    assert_eq!(stack, before);
    stack
        .set_reader_stand_in(state(), stating(&["accent", "accent_ramp.light1"], 0.7))
        .unwrap();
    stack
        .set_reader_stand_in(state(), Layer::default())
        .unwrap();
    let mut alone = super::tests::stack(stating(&["accent"], 0.5), None, Vec::new());
    alone.declarations = declarations("inputs = [\"accent\"]\naccent_inputs = [\"accent\"]\n");
    alone
        .set_reader_stand_in(state(), stating(&["accent"], 0.7))
        .unwrap();
    let derived = declared();
    assert_eq!(
        alone.set_preset(
            None,
            derived.declarations.clone(),
            (*derived.preset_layer).clone(),
            Vec::new()
        ),
        Err(LayerError::AccentInputsMissing(vec![ramp]))
    );
}
