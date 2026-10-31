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
/// wrong value only where its state model draws nothing, and passes.
#[test]
fn gate_fixture_not_drawn_skipped() {
    let report = fixture_report("not_drawn");
    assert_eq!(report.failures, Vec::<String>::new());
}

/// A measured fact compares within its measured tolerance, inclusive, and
/// fails beyond it (S17 *The gate's tolerance*): 40.5px holds against 40px
/// at 0.5, 4.6px does not against 4px.
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
