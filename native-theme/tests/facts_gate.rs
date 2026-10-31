//! The facts gate (S17 *The gate*, M1a task 14): each bundled preset
//! resolved with a stand-in and no reader, every widget's state-model
//! combinations and every token, colour-role and input fact compared with
//! the fact that applies; the perturbed run for derivations; the
//! declarations run. `facts_gate` runs it over `docs/facts` and the bundled
//! presets, the `gate_fixture_*` tests over fixtures, where every value is
//! synthetic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "facts_gate/facts.rs"]
mod facts;
#[path = "facts_gate/model.rs"]
mod model;
#[path = "facts_gate/run.rs"]
mod run;

use std::path::{Path, PathBuf};

use native_theme::next::PresetId;
use native_theme::next::gate::Preset;

/// The platform directory each preset's facts are in (S17 *Generations*).
const DIRECTORIES: [(&str, &str); 8] = [
    ("adwaita", "gnome"),
    ("kde-breeze", "kde"),
    ("macos-monterey", "macos"),
    ("macos-tahoe", "macos"),
    ("ios-13", "ios"),
    ("ios-26", "ios"),
    ("windows-11", "windows"),
    ("material-3", "material"),
];

/// A fixture's facts root and its preset (the fixtures' base with the
/// case's declarations and rules appended), checked against its `gnome`
/// directory.
fn fixture(name: &str) -> (PathBuf, Vec<(String, Preset)>) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/facts_gate");
    let base = std::fs::read_to_string(fixtures.join("base.toml")).unwrap();
    let case = fixtures.join(name);
    let own = std::fs::read_to_string(case.join("preset.toml")).unwrap();
    let preset = Preset::from_toml(&format!("{base}\n{own}"))
        .unwrap_or_else(|error| panic!("{name}: {error}"));
    (case.join("docs/facts"), vec![("gnome".to_owned(), preset)])
}

fn fixture_report(name: &str) -> run::Report {
    let (root, presets) = fixture(name);
    run::run(&root, &presets)
}

/// The report's failures that mention `needle`.
fn failing<'r>(report: &'r run::Report, needle: &str) -> Vec<&'r String> {
    report
        .failures
        .iter()
        .filter(|failure| failure.contains(needle))
        .collect()
}

/// `gate_fixture_passes`: a preset stating every fact of its state model's
/// combinations passes the normal, perturbed and declarations runs; its
/// pending fact is listed with its register id, not failed.
#[test]
fn gate_fixture_passes() {
    let report = fixture_report("passes");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
    assert!(report.queried > 0, "{report:?}");
}

/// `gate_fixture_missing_fact_fails`: a key with no fact on a combination
/// the state model draws fails (R4).
#[test]
fn gate_fixture_missing_fact_fails() {
    let report = fixture_report("missing_fact");
    let failures = failing(&report, "has no fact under");
    assert!(!failures.is_empty(), "{:?}", report.failures);
    assert!(
        failures
            .iter()
            .all(|failure| failure.contains("button.body.opacity")),
        "{failures:?}"
    );
    assert!(
        failures.iter().all(|failure| failure.contains("\"hover\"")),
        "{failures:?}"
    );
}

/// `gate_fixture_literal_where_derived_fails_perturbed`: a preset stating
/// the literal the derivation gives under its own colours passes the normal
/// run and fails the perturbed one.
#[test]
fn gate_fixture_literal_where_derived_fails_perturbed() {
    let report = fixture_report("literal_where_derived");
    assert!(
        failing(&report, "Normal:").is_empty(),
        "{:?}",
        report.failures
    );
    let perturbed = failing(&report, "Perturbed:");
    assert!(!perturbed.is_empty(), "{:?}", report.failures);
    assert!(
        perturbed
            .iter()
            .all(|failure| failure.contains("button.body.fill")),
        "{perturbed:?}"
    );
}

/// A literal where the platform derives from `surface.<role>` fails the
/// perturbed run: the role is perturbed in every surface set the preset
/// declares (S17, R4's GN-1.29 form).
#[test]
fn gate_fixture_surface_operand_perturbed() {
    let report = fixture_report("surface_operand");
    assert!(
        failing(&report, "Normal:").is_empty(),
        "{:?}",
        report.failures
    );
    let perturbed = failing(&report, "Perturbed:");
    assert!(!perturbed.is_empty(), "{:?}", report.failures);
    assert!(
        perturbed
            .iter()
            .all(|failure| failure.contains("button.body.fill")),
        "{perturbed:?}"
    );
}

/// A derivation nested in a table value (a paint's layer's colour) is a
/// derivation: a literal there fails the perturbed run (S17).
#[test]
fn gate_fixture_nested_derivation_perturbed() {
    let report = fixture_report("nested_derivation");
    assert!(
        failing(&report, "Normal:").is_empty(),
        "{:?}",
        report.failures
    );
    let perturbed = failing(&report, "Perturbed:");
    assert!(!perturbed.is_empty(), "{:?}", report.failures);
    assert!(
        perturbed
            .iter()
            .all(|failure| failure.contains("button.body.fill")),
        "{perturbed:?}"
    );
}

/// A role a derivation reads, whose own fact is a derivation too
/// (`accent_standalone = "= role.global.accent"`), is not perturbed: it
/// follows its perturbed operand on both sides, so a preset deriving as the
/// facts do passes, and the role's own derivation is checked (S17).
#[test]
fn gate_fixture_derived_operand_passes() {
    let report = fixture_report("derived_operand");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
}

/// A fact's own `when` on a widget that requires a variant is completed
/// to a combination the state model draws, so a correct preset passes
/// rather than failing "not available" (S17).
#[test]
fn gate_fixture_requires_variant_own_when() {
    let report = fixture_report("requires_variant");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.queried > 0, "{report:?}");
}

/// A field of a part the query does not draw asks no fact, and a fact
/// that applies there fails, as it states a value the platform draws none
/// for (S17): the icon's size fact applies only to the flat button.
#[test]
fn gate_fixture_part_not_drawn() {
    let report = fixture_report("part_not_drawn");
    assert!(
        failing(&report, "has no fact").is_empty(),
        "{:?}",
        report.failures
    );
    let failures = failing(&report, "where its part is NotDrawn");
    assert!(!failures.is_empty(), "{:?}", report.failures);
    assert_eq!(
        failures.len(),
        report.failures.len(),
        "{:?}",
        report.failures
    );
    assert!(
        failures
            .iter()
            .all(|failure| failure.contains("button.icon.icon.size")
                && failure.contains("variants: [\"flat\"]")),
        "{failures:?}"
    );
}

/// `gate_fixture_declarations_mismatch_fails`: a preset declaring the
/// widget's variants or exclusive groups otherwise than its state model
/// fails the declarations run.
#[test]
fn gate_fixture_declarations_mismatch_fails() {
    let report = fixture_report("declarations_mismatch");
    let failures = failing(&report, "declarations:");
    assert!(
        failures
            .iter()
            .any(|failure| failure.contains("`button` variants")),
        "{failures:?}"
    );
    assert!(
        failures
            .iter()
            .any(|failure| failure.contains("`button` exclusive")),
        "{failures:?}"
    );
}

/// A pending fact is listed with its register id and its key, apart from
/// the failures; one a later fact shadows where both hold fails.
#[test]
fn gate_fixture_pending_counted_and_shadowed() {
    let report = fixture_report("passes");
    let pending: Vec<_> = report.pending.values().collect();
    assert_eq!(pending.len(), 1, "{:?}", report.pending);
    let (id, key, count) = pending[0];
    assert_eq!((id.as_str(), key.as_str()), ("U1", "button.body.opacity"));
    assert!(*count > 0);
    let shadowed = fixture_report("pending_shadowed");
    assert!(
        !failing(
            &shadowed,
            "the pending fact U1 of `button.body.opacity` is shadowed"
        )
        .is_empty(),
        "{:?}",
        shadowed.failures
    );
}

/// A recency set's orders are combinations of their own (S17, ADD-A124):
/// a preset that states the `latest` hover fact's value under every order
/// fails where pressed is the latest.
#[test]
fn gate_fixture_recency_order() {
    assert_eq!(
        model::orders(
            &["pressed".into(), "hover".into()],
            &[("lift".into(), vec!["pressed".into(), "hover".into()])]
        )
        .len(),
        2
    );
    let report = fixture_report("recency_order");
    let failures = failing(&report, "button.body.min_width");
    assert!(!failures.is_empty(), "{:?}", report.failures);
    assert!(
        failures
            .iter()
            .all(|failure| failure.contains("order: [\"hover\", \"pressed\"]")),
        "{failures:?}"
    );
}

/// A `not_drawn` combination is not queried: the fixture's preset states a
/// wrong value where its state model draws nothing (pressed), which passes,
/// and one where it draws (hover), which fails; a pending fact that applies
/// on no combination is still listed, on none (S17).
#[test]
fn gate_fixture_not_drawn_skipped() {
    let report = fixture_report("not_drawn");
    assert!(report.queried > 0, "{report:?}");
    assert!(!report.failures.is_empty(), "{report:?}");
    assert!(
        report
            .failures
            .iter()
            .all(|failure| failure.contains("button.body.min_width")
                && failure.contains("\"hover\"")
                && !failure.contains("\"pressed\"")),
        "{:?}",
        report.failures
    );
    let pending: Vec<_> = report.pending.values().collect();
    assert_eq!(pending.len(), 1, "{:?}", report.pending);
    let (id, key, count) = pending[0];
    assert_eq!(
        (id.as_str(), key.as_str(), *count),
        ("U2", "button.body.min_width", 0)
    );
}

/// A `not_drawn` entry's context is matched as the query matches one (S7),
/// not by equality: `{ widget = "button" }` names the axis' flat button,
/// inside which the preset's wrong value is never queried.
#[test]
fn gate_fixture_not_drawn_context() {
    let report = fixture_report("not_drawn_context");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.queried > 0, "{report:?}");
}

/// A `not_drawn` entry with a key the count does not evaluate, and an
/// entry with more flags than the gate enumerates, are refused, never
/// evaluated without it (S17).
#[test]
fn state_model_refuses_what_the_gate_cannot_enumerate() {
    let entry = |extra: &str| {
        format!(
            "[[model]]\nwidget = \"button\"\navailable = true\nvariants = []\nsizes = []\n\
             densities = []\nexclusive = []\nrows = []\n{extra}"
        )
    };
    let sizes = entry("not_drawn = [{ states = [\"pressed\"], sizes = [\"mini\"] }]\n");
    let error = model::Model::read("probe", &sizes).unwrap_err();
    assert!(error.contains("`sizes`"), "{error}");
    let flags: Vec<String> = (0..=model::MAX_FLAGS)
        .map(|n| format!("\"@flag{n}\""))
        .collect();
    let many = entry(&format!(
        "axes = {{ states = [{}], conditions = [], contexts = [] }}\n",
        flags.join(", ")
    ));
    let error = model::Model::read("probe", &many).unwrap_err();
    assert!(error.contains("flags"), "{error}");
    let fits =
        entry("not_drawn = [{ states = [\"pressed\"], context = [{ widget = \"button\" }] }]\n");
    assert!(model::Model::read("probe", &fits).is_ok());
}

/// An entry of a negative list naming what the model does not hold leaves
/// the fact holding: the pressed fact still applies where pressed (S17).
#[test]
fn gate_fixture_negative_list_unknown_name() {
    let report = fixture_report("negative_unknown");
    assert_eq!(report.failures, Vec::<String>::new());
}

/// A token's fact is read in the root scope: a `when` stating a flag
/// there is refused.
#[test]
fn gate_fixture_global_when_refused() {
    let report = fixture_report("global_when");
    let failures = failing(&report, "cannot hold");
    assert_eq!(failures.len(), 1, "{:?}", report.failures);
    assert!(failures[0].contains("token.focus.width"), "{failures:?}");
}

/// A key whose facts are in two files fails: S17 orders a key's facts
/// within its file.
#[test]
fn gate_fixture_key_in_two_files() {
    let report = fixture_report("two_files");
    let failures = failing(&report, "has facts in");
    assert_eq!(failures.len(), 1, "{:?}", report.failures);
    assert!(
        failures[0].contains("button.values.icon_text_gap"),
        "{failures:?}"
    );
}

/// A measured fact compares within its measured tolerance and fails beyond
/// it (S17 *The gate's tolerance*): 40.5px holds against 40px at 0.5, 4.6px
/// does not against 4px; a point size's tolerance is in points, compared in
/// px as the value is (S9.4). The edge and the f32 slack beside it are
/// `measured_tolerance_edge_and_its_slack`'s.
#[test]
fn gate_fixture_tolerance_edge() {
    let report = fixture_report("tolerance");
    let failures = failing(&report, "Normal:");
    assert!(!failures.is_empty(), "{:?}", report.failures);
    assert!(
        failures
            .iter()
            .all(|failure| failure.contains("button.values.icon_text_gap")),
        "{failures:?}"
    );
}

/// A block S6.1 sets whole (`Option<KeyRepeat>`) has its facts stated per
/// leaf (S17): the gate composes the leaves' applying facts into one value
/// per combination. The preset stating both leaves passes; one stating
/// another interval fails at that leaf; a leaf with no fact fails as a key
/// with no fact.
#[test]
fn gate_fixture_set_whole_composed() {
    let report = fixture_report("set_whole");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.unread.is_empty(), "{:?}", report.unread);
    let differs = fixture_report("set_whole_differs");
    let failures = failing(&differs, "Normal:");
    assert!(!failures.is_empty(), "{:?}", differs.failures);
    assert!(
        failures
            .iter()
            .all(|failure| failure.contains("token.behavior.key_repeat.interval")),
        "{failures:?}"
    );
    let missing = fixture_report("set_whole_missing");
    let failures = failing(&missing, "has no fact under");
    assert!(
        failures
            .iter()
            .any(|failure| failure.contains("`token.behavior.key_repeat.interval`")),
        "{:?}",
        missing.failures
    );
}

/// A measured derivation is evaluated over each evidence pass, its varied
/// input stated by the stand-in, against the pass's result (S17 *A
/// measured derivation*): the pass whose result is one 8-bit step off the
/// accent, beyond the 0.5 tolerance, fails; the other holds.
#[test]
fn gate_fixture_measured_derivation() {
    let report = fixture_report("derivation_passes");
    assert!(!report.failures.is_empty(), "{report:?}");
    assert!(
        report
            .failures
            .iter()
            .all(|failure| failure.contains("the pass")
                && failure.contains("#00ff00")
                && failure.contains("button.body.fill")),
        "{:?}",
        report.failures
    );
}

/// `facts_gate`: the gate over `docs/facts` and every bundled preset; none
/// is bundled until M1a task 15, so it checks that every fact file and
/// state model reads, counts each entry's combinations, and runs no
/// preset.
#[test]
fn facts_gate() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/facts");
    let presets: Vec<(String, Preset)> = PresetId::all()
        .iter()
        .map(|id| {
            let directory = DIRECTORIES
                .iter()
                .find(|(key, _)| *key == id.key())
                .map(|(_, directory)| (*directory).to_owned())
                .unwrap_or_else(|| panic!("{}: no facts directory", id.key()));
            (directory, Preset::bundled(*id).unwrap())
        })
        .collect();
    let report = run::run(&root, &presets);
    eprintln!(
        "facts gate: {} presets, {} keys compared over {} distinct combinations in {:?}; \
         {} pending facts; skipped: {} facts of {} keys the model does not hold, {} whens, \
         {} accent-run facts, {} facts naming a preset not bundled, {} combinations",
        presets.len(),
        report.compared,
        report.queried,
        report.elapsed,
        report.pending.len(),
        report.keys_not_held.values().sum::<usize>(),
        report.keys_not_held.len(),
        report.whens_not_held.values().sum::<usize>(),
        report.accent_run,
        report.other_presets,
        report.combinations_skipped,
    );
    for (why, count) in &report.whens_not_held {
        eprintln!("  whens not held: {count}: {why}");
    }
    for (key, count) in &report.keys_not_held {
        eprintln!("  keys not held: {count}: {key}");
    }
    for (entry, count) in &report.combinations {
        eprintln!("  combinations: {entry}: {count}");
    }
    eprintln!(
        "  {} values the model's reader refuses (a failure for a preset checked against them):",
        report.unread.len()
    );
    for unread in &report.unread {
        eprintln!("    {unread}");
    }
    for (place, (id, key, count)) in &report.pending {
        eprintln!("  pending: {place}: {id} `{key}` on {count} combinations");
    }
    assert_eq!(report.failures, Vec::<String>::new());
}
