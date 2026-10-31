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

/// The negative control's pair: facts stating one wrong value per key
/// class (a token, a colour role, an input, a widget value and a part's
/// field) fail at each, so `gate_fixture_passes` passing means each class
/// is compared.
#[test]
fn gate_fixture_wrong_value_per_key_class() {
    let report = fixture_report("wrong_values");
    for key in [
        "token.focus.width",
        "role.global.accent",
        "input.accent",
        "button.values.icon_text_gap",
        "button.body.fill",
    ] {
        assert!(
            !failing(&report, &format!("Normal: `{key}` at")).is_empty(),
            "{key}: {:?}",
            report.failures
        );
    }
}

/// The last fact in file order applies, not the most specific: a later,
/// unconditional 40px applies where an earlier pressed fact holds too, and
/// a preset stating 40px everywhere passes (S17 *Which fact applies*).
#[test]
fn gate_fixture_later_less_specific_fact_applies() {
    let report = fixture_report("file_order");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
}

/// Of several measured citations serving the preset, the smallest
/// tolerance holds; a citation serving another preset is not the
/// preset's (S17 *The gate's tolerance*).
#[test]
fn gate_fixture_tolerance_of_citations() {
    let report = fixture_report("tolerance_citations");
    assert!(!report.failures.is_empty(), "{report:?}");
    assert!(
        report
            .failures
            .iter()
            .all(|failure| failure.contains("button.body.min_width")),
        "{:?}",
        report.failures
    );
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

/// Each of S10's declarations the declarations run compares, one case
/// each: the preset declaring it otherwise than the state model fails at
/// that declaration (S17, ADD-A116), an exclusive group's precedence order
/// and the size lists' written order included; a widget available in the
/// preset that no state model states fails too.
#[test]
fn gate_declarations_each_compared() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/facts_gate");
    let base = std::fs::read_to_string(fixtures.join("base.toml")).unwrap();
    // A table's entries, one line each, every value inline.
    let inline = |table: &toml::Table| {
        table
            .iter()
            .map(|(key, value)| format!("{key} = {value}\n"))
            .collect::<String>()
    };
    // A TOML table, `extra`'s keys over `defaults`'.
    let merged = |defaults: &str, extra: &str| {
        let mut table: toml::Table = toml::from_str(defaults).unwrap();
        table.extend(toml::from_str::<toml::Table>(extra).unwrap());
        table
    };
    let model_of = |extra: &str| {
        let entry = merged(
            "widget = \"button\"\navailable = true\nvariants = [\"flat\", \"primary\"]\n\
             sizes = []\ndensities = []\nexclusive = []\nrows = []\n\
             axes = { states = [\"hover\", \"pressed\"], conditions = [], contexts = [] }\n",
            extra,
        );
        format!("[[model]]\n{}", inline(&entry))
    };
    let sized = "sizes = [\"small\", \"regular\"]\ndefault_size = \"regular\"\n";
    // (declaration, the preset's declarations, the state model's own lines)
    let cases: Vec<(&str, String, String)> = vec![
        ("available", String::new(), "available = false\n".to_owned()),
        (
            "variants",
            "variants = [\"flat\"]\n".to_owned(),
            String::new(),
        ),
        ("sizes", sized.to_owned(), "sizes = []\n".to_owned()),
        (
            "default_size",
            sized.to_owned(),
            "sizes = [\"small\", \"regular\"]\ndefault_size = \"small\"\n".to_owned(),
        ),
        (
            "default_size_by_variant",
            format!(
                "{sized}default_size_by_variant = [{{ variant = \"flat\", size = \"small\" }}]\n"
            ),
            sized.to_owned(),
        ),
        (
            "densities",
            "densities = [\"compact\"]\n".to_owned(),
            String::new(),
        ),
        ("flags", "flags = [\"@armed\"]\n".to_owned(), String::new()),
        (
            "exclusive",
            "exclusive = [[\"pressed\", \"hover\"]]\n".to_owned(),
            String::new(),
        ),
        (
            "exclusive",
            "exclusive = [[\"pressed\", \"hover\"]]\n".to_owned(),
            "exclusive = [[\"hover\", \"pressed\"]]\n".to_owned(),
        ),
        (
            "exclusive_variants",
            "exclusive_variants = [[\"flat\", \"primary\"]]\n".to_owned(),
            String::new(),
        ),
        (
            "recency",
            "recency = { lift = [\"pressed\", \"hover\"] }\n".to_owned(),
            String::new(),
        ),
        (
            "requires_variants",
            "requires_variants = [\"flat\", \"primary\"]\n".to_owned(),
            String::new(),
        ),
        (
            "sizes_by_variant",
            format!("{sized}sizes_by_variant = [{{ variant = \"flat\", sizes = [\"small\"] }}]\n"),
            sized.to_owned(),
        ),
        (
            "sizes_by_variant",
            format!(
                "{sized}sizes_by_variant = [{{ variant = \"flat\", sizes = [\"small\"] }}, \
                 {{ variant = \"primary\", sizes = [\"regular\"] }}]\n"
            ),
            format!(
                "{sized}sizes_by_variant = [{{ variant = \"primary\", sizes = [\"regular\"] }}, \
                 {{ variant = \"flat\", sizes = [\"small\"] }}]\n"
            ),
        ),
        (
            "sizes_by_context",
            format!(
                "{sized}sizes_by_context = [{{ context = \"button\", sizes = [\"small\"] }}]\n"
            ),
            sized.to_owned(),
        ),
    ];
    for (declaration, declared, stated) in &cases {
        let button = merged("variants = [\"flat\", \"primary\"]\n", declared);
        // A declared flag or recency set is one a rule selects on (S12).
        let rules = match *declaration {
            "flags" => {
                "[[rules]]\nwhen = { states = [\"@armed\"] }\ntarget = \"button.body\"\nset = { opacity = 1 }\n"
            }
            "recency" => {
                "[[rules]]\nwhen = { latest = { lift = \"hover\" } }\ntarget = \"button.body\"\nset = { opacity = 1 }\n"
            }
            _ => "",
        };
        let declare = format!("[declare.widget.button]\n{}{rules}", inline(&button));
        let preset = Preset::from_toml(&format!("{base}\n{declare}"))
            .unwrap_or_else(|error| panic!("{declaration}: {declare}: {error}"));
        let models = model::Model::read("gnome/state-models.toml", &model_of(stated)).unwrap();
        let platform = run::Platform {
            name: "gnome".to_owned(),
            facts: Vec::new(),
            keys: Vec::new(),
            models,
        };
        let mut report = run::Report::default();
        run::declarations_run(&platform, &preset, None, &mut report);
        let needle = format!("`button` {declaration} under");
        assert!(
            report
                .failures
                .iter()
                .any(|failure| failure.contains(&needle)),
            "{declaration}: {declare} against {stated}: {:?}",
            report.failures
        );
    }
    let preset = Preset::from_toml(&format!(
        "{base}\n[declare.widget.button]\nvariants = [\"flat\"]\n"
    ))
    .unwrap();
    let platform = run::Platform {
        name: "gnome".to_owned(),
        facts: Vec::new(),
        keys: Vec::new(),
        models: Vec::new(),
    };
    let mut report = run::Report::default();
    run::declarations_run(&platform, &preset, None, &mut report);
    assert!(
        report
            .failures
            .iter()
            .any(|failure| failure.contains("`button` is available")),
        "{:?}",
        report.failures
    );
}

/// The declarations run and the combinations are the state model's
/// projected onto what the enabled units' facts name (ADD-A128): a variant
/// only skipped facts name (their `when` names a toolbar, whose unit is not
/// enabled) is left out and counted, neither failed nor queried; a variant
/// no fact names is checked like any other, passing where the preset
/// declares it and failing where it does not.
#[test]
fn gate_fixture_variants_of_units_not_enabled() {
    let skipped = fixture_report("variant_skipped");
    assert_eq!(skipped.failures, Vec::<String>::new());
    assert_eq!(
        skipped.variants_not_held.iter().collect::<Vec<_>>(),
        ["gnome/state-models.toml button @raised"]
    );
    assert!(skipped.queried > 0, "{skipped:?}");
    let unnamed = fixture_report("variant_unnamed");
    assert_eq!(unnamed.failures, Vec::<String>::new());
    assert!(unnamed.variants_not_held.is_empty(), "{unnamed:?}");
    let undeclared = fixture_report("variant_undeclared");
    assert!(
        undeclared
            .failures
            .iter()
            .any(|failure| failure.contains("`button` variants under")),
        "{:?}",
        undeclared.failures
    );
    assert!(undeclared.variants_not_held.is_empty(), "{undeclared:?}");
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
    let names = |list: &[&str]| list.iter().map(|&n| n.to_owned()).collect::<Vec<_>>();
    assert_eq!(
        model::orders(
            &names(&["pressed", "hover"]),
            &[("lift".into(), names(&["pressed", "hover"]))]
        )
        .len(),
        2
    );
    // Three members of one set: one order per latest flag, the six
    // permutations counted three times.
    assert_eq!(
        model::orders(
            &names(&["a", "b", "c"]),
            &[("s".into(), names(&["a", "b", "c"]))]
        )
        .len(),
        3
    );
    // Two sets sharing `b`: one order per pair of latest flags, (b, c),
    // (b, b), (a, c) and (a, b).
    assert_eq!(
        model::orders(
            &names(&["a", "b", "c"]),
            &[
                ("s".into(), names(&["a", "b"])),
                ("t".into(), names(&["b", "c"]))
            ]
        )
        .len(),
        4
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
         {} accent-run facts, {} facts naming a preset not bundled, {} combinations, \
         {} state-model variants only skipped facts name",
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
        report.variants_not_held.len(),
    );
    for variant in &report.variants_not_held {
        eprintln!("  variants not held: {variant}");
    }
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
