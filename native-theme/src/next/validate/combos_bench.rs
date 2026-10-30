//! The cost of S12's checks over combinations (decision 5) at the sizes
//! M3's presets reach: one place's worlds under 96 rules over 20 atoms, and
//! a whole Material-sized widget, 400 rules over 40 atoms with exclusive
//! groups, checked end to end. Timed in an ignored test, run in the release
//! profile (`cargo test --release -p native-theme --lib combos_bench --
//! --ignored --nocapture`); the outcomes' counts are checked in every run.
//! Every name and number here is synthetic.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, Instant};

use super::super::{Context, LayerKind, Stacked};
use super::{Seed, Stack, Target};
use crate::next::blocks::part_style::PartStylePatch;
use crate::next::declare::Declarations;
use crate::next::expr::Expr;
use crate::next::layer::Layer;
use crate::next::selector::{
    Condition, Density, Direction, PartRef, PositionMatch, Rule, RuleBody, Scheme, Selector,
    StateFlag, VariantRef,
};
use crate::next::toml::layer::read_preset;
use crate::next::units::{Fraction, Px};
use crate::next::value::Value;
use crate::next::widgets::{ButtonPart, PartPatch, TypedPart};

/// The declarations: the states and variants the rules name, the
/// interaction states and the check states exclusive, each family of
/// variants exclusive.
fn declarations() -> Declarations {
    read_preset(
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n[declare.widget.button]\n\
         flags = [\"hover\", \"pressed\", \"focused\", \"focus_visible\", \"disabled\", \
         \"checked\", \"selected\", \"indeterminate\", \"dragged\", \"invalid\"]\n\
         variants = [\"@s1\", \"@s2\", \"@s3\", \"@s4\", \"@s5\", \"@z1\", \"@z2\", \"@z3\", \
         \"@z4\", \"@h1\", \"@h2\", \"@toggle\", \"@icon\"]\n\
         densities = [\"default\", \"compact\"]\n\
         exclusive = [[\"hover\", \"pressed\", \"focused\", \"dragged\"], [\"checked\", \
         \"indeterminate\"]]\n\
         exclusive_variants = [[\"@s1\", \"@s2\", \"@s3\", \"@s4\", \"@s5\"], [\"@z1\", \"@z2\", \
         \"@z3\", \"@z4\"], [\"@h1\", \"@h2\"]]\n",
    )
    .unwrap()
    .declarations
}

/// One selector part, naming one atom (the scheme and the direction name
/// their pair: S12 completes them).
#[derive(Clone)]
enum Part {
    Cond(Condition),
    State(StateFlag),
    Variant(&'static str),
    Dark,
    Rtl,
    Compact,
    First,
    Last,
}

/// The parts the rules draw from: 40 atoms with the scheme's and the
/// direction's pairs.
fn parts() -> Vec<Part> {
    let mut out: Vec<Part> = [
        Condition::ContrastIncreased,
        Condition::ContrastHigh,
        Condition::ReducedMotion,
        Condition::ReducedTransparency,
        Condition::BoldText,
        Condition::AnimationsOff,
        Condition::ButtonShapes,
        Condition::DifferentiateWithoutColor,
        Condition::TabletMode,
        Condition::FullKeyboardAccess,
    ]
    .into_iter()
    .map(Part::Cond)
    .collect();
    out.extend(
        [
            StateFlag::Hover,
            StateFlag::Pressed,
            StateFlag::Focused,
            StateFlag::FocusVisible,
            StateFlag::Disabled,
            StateFlag::Checked,
            StateFlag::Selected,
            StateFlag::Indeterminate,
            StateFlag::Dragged,
            StateFlag::Invalid,
        ]
        .map(Part::State),
    );
    out.extend(
        [
            "s1", "s2", "s3", "s4", "s5", "z1", "z2", "z3", "z4", "h1", "h2", "toggle", "icon",
        ]
        .map(Part::Variant),
    );
    out.extend([
        Part::Dark,
        Part::Rtl,
        Part::Compact,
        Part::First,
        Part::Last,
    ]);
    out
}

fn add(selector: &mut Selector, part: &Part, holds: bool) {
    fn set<T>(set: &mut crate::next::selector::SetMatch<T>, item: T, holds: bool) {
        if holds {
            set.all_of.push(item);
        } else {
            set.none_of.push(item);
        }
    }
    match part {
        Part::Cond(condition) => set(&mut selector.conditions, condition.clone(), holds),
        Part::State(flag) => set(&mut selector.states, flag.clone(), holds),
        Part::Variant(name) => set(
            &mut selector.variants,
            VariantRef::Keyed(crate::next::key::Key::new(name).unwrap()),
            holds,
        ),
        Part::Dark => set(
            &mut selector.conditions,
            Condition::Scheme(if holds { Scheme::Dark } else { Scheme::Light }),
            true,
        ),
        Part::Rtl => {
            selector.direction = Some(if holds {
                Direction::Rtl
            } else {
                Direction::Ltr
            });
        }
        Part::Compact => set(&mut selector.size.densities, Density::Compact, holds),
        Part::First | Part::Last => {
            let position = selector.position.get_or_insert_with(PositionMatch::default);
            if matches!(part, Part::First) {
                position.first = Some(holds);
            } else {
                position.last = Some(holds);
            }
        }
    }
}

/// A seeded xorshift.
struct Rng(u64);

impl Rng {
    fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        usize::try_from(self.0 % u64::try_from(n.max(1)).unwrap()).unwrap()
    }

    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }
}

/// A rule on `part`'s style setting `patch`.
fn rule(selector: Selector, part: ButtonPart, patch: PartStylePatch) -> Rule {
    Rule::new(
        selector,
        RuleBody::Part(
            PartRef::Typed(TypedPart::Button(part)),
            PartPatch::Style(patch),
        ),
    )
}

fn opacity(n: f32) -> PartStylePatch {
    PartStylePatch {
        opacity: Some(Value::Stated(Fraction(n))),
        ..PartStylePatch::default()
    }
}

/// `rules` rules on the body's opacity, each needing one to three of the
/// first `atoms` parts, held or not: task 9's measure.
fn one_place(rules: usize, atoms: usize) -> Layer {
    let parts = parts();
    let parts = parts.get(..atoms).unwrap();
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let mut layer = Layer::default();
    for index in 0..rules {
        let mut selector = Selector::default();
        for _ in 0..=rng.below(3) {
            let part = parts.get(rng.below(parts.len())).unwrap();
            let holds = rng.chance(75);
            add(&mut selector, part, holds);
        }
        let n = f32::from(u8::try_from(index % 100).unwrap()) / 100.0;
        layer
            .rules
            .push(rule(selector, ButtonPart::Body, opacity(n)));
    }
    layer
}

/// A Material-sized widget: `rules` rules, each a style variant (most), an
/// interaction state (often) and a few other parts at Material's rates,
/// spread over the button's places, a quarter of the lengths an expression
/// over another place.
fn material(rules: usize) -> Layer {
    let parts = parts();
    let pick = |name: &str| {
        parts
            .iter()
            .find(|part| matches!(part, Part::Variant(v) if *v == name))
            .cloned()
            .unwrap()
    };
    let styles: Vec<Part> = ["s1", "s2", "s3", "s4", "s5"].map(pick).to_vec();
    let sizes: Vec<Part> = ["z1", "z2", "z3", "z4"].map(pick).to_vec();
    let interaction = [
        StateFlag::Hover,
        StateFlag::Pressed,
        StateFlag::Focused,
        StateFlag::Dragged,
    ]
    .map(Part::State);
    let others: Vec<Part> = parts
        .iter()
        .filter(|part| match part {
            Part::Variant(name) => !name.starts_with('s') && !name.starts_with('z'),
            Part::State(flag) => !interaction
                .iter()
                .any(|i| matches!(i, Part::State(f) if f == flag)),
            _ => true,
        })
        .cloned()
        .collect();
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    let mut layer = Layer::default();
    for index in 0..rules {
        let mut selector = Selector::default();
        if rng.chance(85) {
            add(&mut selector, styles.get(rng.below(5)).unwrap(), true);
        }
        if rng.chance(60) {
            add(&mut selector, interaction.get(rng.below(4)).unwrap(), true);
        }
        if rng.chance(20) {
            add(&mut selector, sizes.get(rng.below(4)).unwrap(), true);
        }
        if rng.chance(30) {
            let part = others.get(rng.below(others.len())).unwrap();
            let holds = rng.chance(80);
            add(&mut selector, part, holds);
        }
        let n = u8::try_from(index % 50).unwrap();
        let mut length = |field: &str| -> Value<Px> {
            if rng.chance(25) {
                Value::Expr(Expr::parse(&format!("part.{field}")).unwrap())
            } else {
                Value::Stated(Px(f32::from(n)))
            }
        };
        let (part, patch) = match index % 6 {
            0 => (ButtonPart::Body, opacity(f32::from(n) / 50.0)),
            1 => (
                ButtonPart::Body,
                PartStylePatch {
                    min_width: Some(Value::Stated(Px(f32::from(n)))),
                    ..PartStylePatch::default()
                },
            ),
            2 => (
                ButtonPart::Body,
                PartStylePatch {
                    min_height: Some(length("min_width")),
                    ..PartStylePatch::default()
                },
            ),
            3 => (
                ButtonPart::Body,
                PartStylePatch {
                    max_width: Some(length("min_width")),
                    ..PartStylePatch::default()
                },
            ),
            4 => (
                ButtonPart::MenuIndicator,
                PartStylePatch {
                    min_width: Some(Value::Stated(Px(f32::from(n)))),
                    ..PartStylePatch::default()
                },
            ),
            _ => (
                ButtonPart::MenuIndicator,
                PartStylePatch {
                    min_height: Some(length("min_width")),
                    ..PartStylePatch::default()
                },
            ),
        };
        layer.rules.push(rule(selector, part, patch));
    }
    layer
}

/// The worlds of the body's opacity under `layer`, and the time taken.
fn opacity_worlds(layer: &Layer, decl: &Declarations) -> (usize, Duration) {
    let below = [Stacked {
        layer,
        kind: LayerKind::Application,
    }];
    let cx = Context {
        declarations: decl,
        preset: None,
        kind: LayerKind::Application,
        below: &[],
    };
    let stack = Stack::new(&below, cx);
    let started = Instant::now();
    let worlds = stack.worlds(vec![Seed::Field(
        Target::Part(TypedPart::Button(ButtonPart::Body)),
        "opacity".to_owned(),
        false,
    )]);
    (worlds.len(), started.elapsed())
}

/// The whole check of `layer`, and the time taken.
fn checked(layer: &Layer, decl: &Declarations) -> (usize, Duration) {
    let cx = Context {
        declarations: decl,
        preset: None,
        kind: LayerKind::Application,
        below: &[],
    };
    let started = Instant::now();
    let problems = match super::super::check(layer, &cx) {
        Ok(()) => 0,
        Err(crate::next::error::LayerError::Invalid(problems)) => problems.len(),
        Err(other) => panic!("{other}"),
    };
    (problems, started.elapsed())
}

/// The outcomes' counts, deterministic: the per-place enumeration's work
/// grows with them.
#[test]
fn combos_bench_counts() {
    let decl = declarations();
    assert_eq!(opacity_worlds(&one_place(48, 14), &decl).0, 8912);
    assert_eq!(opacity_worlds(&material(400), &decl).0, 1280);
}

/// The timings decision 5 asks for: 96 rules over 20 atoms at one place in
/// under 1 s, a Material-sized widget checked whole in under 5 s.
#[test]
#[ignore = "a timing: run in the release profile, `--ignored`"]
fn combos_bench_timings() {
    let decl = declarations();
    let (worlds, place) = opacity_worlds(&one_place(96, 20), &decl);
    eprintln!("one place, 96 rules over 20 atoms: {worlds} worlds in {place:?}");
    let layer = material(400);
    let (worlds, material_place) = opacity_worlds(&layer, &decl);
    eprintln!("a Material-sized widget, its body's opacity: {worlds} worlds in {material_place:?}");
    let (problems, whole) = checked(&layer, &decl);
    eprintln!("a Material-sized widget, 400 rules: {problems} problems in {whole:?}");
    assert!(place < Duration::from_secs(1), "{place:?}");
    assert!(whole < Duration::from_secs(5), "{whole:?}");
}
