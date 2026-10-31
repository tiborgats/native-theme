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
/// case's declarations and rules, its `file`, appended), checked against
/// its `gnome` directory.
fn fixture(name: &str, file: &str) -> (PathBuf, Vec<(String, Preset)>) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/facts_gate");
    let base = std::fs::read_to_string(fixtures.join("base.toml")).unwrap();
    let case = fixtures.join(name);
    let preset = Preset::from_toml(&fixture_text(&case, &base, file))
        .unwrap_or_else(|error| panic!("{name}/{file}: {error}"));
    (case.join("docs/facts"), vec![("gnome".to_owned(), preset)])
}

/// A case's preset text: the base, less each line a `# base without: <line>`
/// comment of the case's `file` names (a listed path's own line, S17), with
/// the case's file appended.
fn fixture_text(case: &Path, base: &str, file: &str) -> String {
    let own = std::fs::read_to_string(case.join(file)).unwrap();
    let mut base = base.to_owned();
    for line in own.lines() {
        if let Some(dropped) = line.strip_prefix("# base without: ") {
            let dropped = format!("\n{dropped}\n");
            assert!(
                base.contains(&dropped),
                "{file}: the base has no {dropped:?}"
            );
            base = base.replacen(&dropped, "\n", 1);
        }
    }
    format!("{base}\n{own}")
}

fn fixture_report(name: &str) -> run::Report {
    fixture_report_with(name, "preset.toml")
}

/// The report of a fixture's facts checked against the case's preset
/// `file`, a twin of its `preset.toml`.
fn fixture_report_with(name: &str, file: &str) -> run::Report {
    let (root, presets) = fixture(name, file);
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

/// A part the preset leaves out where no part fact applies fails at the
/// part's own key, which with no part fact is expected drawn; its field's
/// fact, which applies only where the button is flat, compares nothing
/// where the part is not drawn and fails nothing (S17, ADD-A130).
#[test]
fn gate_fixture_part_not_drawn() {
    let report = fixture_report("part_not_drawn");
    assert!(
        failing(&report, "button.icon.icon.size").is_empty(),
        "{:?}",
        report.failures
    );
    let failures = failing(&report, "`button.icon` at");
    assert!(!failures.is_empty(), "{:?}", report.failures);
    assert_eq!(
        failures.len(),
        report.failures.len(),
        "{:?}",
        report.failures
    );
    assert!(
        failures.iter().all(|failure| failure.contains("NotDrawn")
            && failure.contains("no fact of the whole part (drawn)")),
        "{failures:?}"
    );
    assert!(report.fields_not_drawn > 0, "{report:?}");
}

/// libadwaita's menu indicator pattern (S17, ADD-A130): the part's own
/// facts state where it is absent (`not_variants = ["flat"]`, and `flat`
/// without `@with_arrow`), its field's fact holds without a `when`. A
/// preset drawing the part exactly where the facts do passes, its field
/// compared where drawn and nothing compared where not; one leaving the
/// part out where the facts draw it fails at the part's key alone.
#[test]
fn gate_fixture_part_absent_by_its_own_facts() {
    let report = fixture_report("part_absent_fields");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.fields_not_drawn > 0, "{report:?}");
    let dropped = fixture_report("part_absent_dropped");
    assert!(!dropped.failures.is_empty(), "{dropped:?}");
    assert!(
        dropped
            .failures
            .iter()
            .all(|failure| failure.contains("`button.subtitle` at")
                && failure.contains("[\"@with_arrow\", \"flat\"]")),
        "{:?}",
        dropped.failures
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

/// A fact whose only list naming a widget of a unit not enabled is
/// negative (`not_context` a toolbar) belongs to its key's unit alone: the
/// negation holds, so the variant it names is not projected out, and the
/// fact applies (S17, ADD-A128's unit membership). A preset stating its
/// value passes; one leaving it unstated fails there.
#[test]
fn gate_fixture_negative_list_adds_no_unit() {
    let report = fixture_report("variant_negated");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.variants_not_held.is_empty(), "{report:?}");
    let unstated = fixture_report_with("variant_negated", "unstated.toml");
    let failures = failing(&unstated, "`button.body.fill` at");
    assert!(!failures.is_empty(), "{:?}", unstated.failures);
    assert_eq!(
        failures.len(),
        unstated.failures.len(),
        "{:?}",
        unstated.failures
    );
    assert!(
        failures
            .iter()
            .all(|failure| failure.contains("variants: [\"@raised\"]")),
        "{failures:?}"
    );
}

/// A variant named only by a fact whose key the model refuses stays in
/// the state model: the refusal fails the run, and the declarations run
/// compares the variant as declared, its messages not hiding it (S17,
/// ADD-A128: only a key the model does not hold is skipped).
#[test]
fn gate_fixture_refused_key_skips_no_variant() {
    let report = fixture_report("variant_refused");
    assert!(
        !failing(&report, "the key `button..fill` is refused").is_empty(),
        "{:?}",
        report.failures
    );
    assert!(
        failing(&report, "`button` variants under").is_empty(),
        "{:?}",
        report.failures
    );
    assert!(report.variants_not_held.is_empty(), "{report:?}");
}

/// Each rule of the projection (ADD-A128), one variant each: a variant
/// only skipped facts name goes, out of its three-member exclusive group
/// too, which keeps its other two; one a kept fact names as well stays;
/// one only a fact of a key the model does not hold names goes; a content
/// variant S12 exempts stays. A preset declaring what is left passes.
#[test]
fn gate_fixture_projection_rules() {
    let report = fixture_report("variant_projection");
    assert_eq!(report.failures, Vec::<String>::new());
    assert_eq!(
        report.variants_not_held.iter().collect::<Vec<_>>(),
        [
            "gnome/state-models.toml button @raised",
            "gnome/state-models.toml button @unheld"
        ]
    );
    assert!(report.queried > 0, "{report:?}");
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

/// A set-whole block's leaves compose per combination (ADD-A128): a delay
/// under high contrast beside an unconditional interval gives two values,
/// so a preset stating each class passes and one stating the base class's
/// delay under high contrast fails there; a preset stating the field
/// `"none"` where the facts state both leaves fails as a resolved `None`.
#[test]
fn gate_fixture_set_whole_per_combination() {
    let report = fixture_report("set_whole_classes");
    assert_eq!(report.failures, Vec::<String>::new());
    let one = fixture_report_with("set_whole_classes", "one_class.toml");
    let failures = failing(&one, "Normal:");
    assert!(!failures.is_empty(), "{:?}", one.failures);
    assert_eq!(failures.len(), one.failures.len(), "{:?}", one.failures);
    assert!(
        failures
            .iter()
            .all(|failure| failure.contains("contrast_high")
                && failure.contains("token.behavior.key_repeat.delay")),
        "{failures:?}"
    );
    let none = fixture_report_with("set_whole", "none.toml");
    assert!(
        !failing(&none, "Normal: `token.behavior.key_repeat.").is_empty(),
        "{:?}",
        none.failures
    );
    assert!(
        none.failures
            .iter()
            .all(|failure| failure.contains("`token.behavior.key_repeat.")),
        "{:?}",
        none.failures
    );
}

/// A pending leaf makes the set-whole field pending (ADD-A128): nothing is
/// compared for it, the pending fact is listed with its id, and the other
/// leaf's fact is listed unchecked beside it.
#[test]
fn gate_fixture_set_whole_pending_leaf() {
    let report = fixture_report("set_whole_pending");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(
        report.pending.values().any(|(id, key, count)| id == "U3"
            && key == "token.behavior.key_repeat.interval"
            && *count > 0),
        "{:?}",
        report.pending
    );
    let unchecked: Vec<_> = report.unchecked.values().collect();
    assert_eq!(unchecked.len(), 1, "{:?}", report.unchecked);
    let (key, pending, count) = unchecked[0];
    assert_eq!(key, "token.behavior.key_repeat.delay");
    assert!(pending.starts_with("U3 at "), "{pending}");
    assert!(*count > 0);
}

/// A preset whose `[pending]` table lists a path (S17, ADD-A128), marked
/// synthetic: the gate loads it through the synthetic route, finds the
/// listed path where its pending fact applies with nothing beneath,
/// compares the path where the preset's rule states it (high contrast),
/// and skips and reports a fact whose evaluation reaches the synthetic
/// value (the long press reading the delay), comparing it where the rule
/// gives the value.
#[test]
fn gate_fixture_pending_listed() {
    let report = fixture_report("pending_listed");
    assert_eq!(report.failures, Vec::<String>::new());
    assert_eq!(
        report.listed.keys().collect::<Vec<_>>(),
        ["probe token.behavior.touch.delay"]
    );
    assert_eq!(
        report.listed.values().next().map(|(id, _)| id.as_str()),
        Some("U5")
    );
    let skipped: Vec<&String> = report.synthetic.values().map(|(key, _)| key).collect();
    assert_eq!(
        skipped,
        ["token.behavior.long_press"],
        "{:?}",
        report.synthetic
    );
    assert!(report.compared > 0, "{report:?}");
}

/// The table holds exactly the listed paths (S17, ADD-A128): a listed path
/// the table lacks is S12's missing field, which loading the preset
/// refuses; an entry no combination leaves to a pending fact fails, and so
/// does one under another register id than its pending fact's.
#[test]
fn gate_fixture_pending_table_both_ways() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/facts_gate");
    let base = std::fs::read_to_string(fixtures.join("base.toml")).unwrap();
    let case = fixtures.join("pending_listed");
    let error = Preset::from_toml(&fixture_text(&case, &base, "unlisted.toml")).unwrap_err();
    assert!(
        error.contains("tokens.behavior.touch.delay: missing"),
        "{error}"
    );
    let extra = fixture_report_with("pending_listed", "extra.toml");
    assert_eq!(
        failing(&extra, "[pending]"),
        [
            &"probe: `tokens.behavior.drag_time_ms` (U6) is in its [pending] table, and no \
           combination leaves it to a pending fact with nothing beneath"
                .to_owned()
        ],
        "{:?}",
        extra.failures
    );
    let wrong = fixture_report_with("pending_listed", "wrong_id.toml");
    assert_eq!(
        failing(&wrong, "[pending]"),
        [
            &"probe: `tokens.behavior.touch.delay_ms` is listed as U7 in its [pending] table, and \
           its pending fact is U5"
                .to_owned()
        ],
        "{:?}",
        wrong.failures
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
    for (key, (id, count)) in &report.listed {
        eprintln!("  listed: {key}: {id} on {count} combinations");
    }
    for (place, (key, count)) in &report.synthetic {
        eprintln!("  reaches a synthetic value: {place}: `{key}` on {count} combinations");
    }
    for (place, (key, pending, count)) in &report.unchecked {
        eprintln!(
            "  unchecked: {place}: `{key}` beside the pending {pending} on {count} combinations"
        );
    }
    eprintln!(
        "  {} field facts where their part is not drawn, {} facts whose own when draws nothing",
        report.fields_not_drawn, report.whens_not_drawn
    );
    assert_eq!(report.failures, Vec::<String>::new());
    // The release's gate (S17, R21): 0.8.0 is released with nothing skipped,
    // no pending fact and every `[pending]` table empty, which the plan's
    // final gate runs this test with `FACTS_GATE_RELEASE=1` to require.
    if std::env::var_os("FACTS_GATE_RELEASE").is_some() {
        assert_eq!(
            (
                report.pending.len(),
                report.keys_not_held.len(),
                report.whens_not_held.len(),
                report.other_presets,
                report.combinations_skipped,
                report.variants_not_held.len(),
                report.unread.len(),
                report.listed.len() + report.synthetic.len(),
            ),
            (0, 0, 0, 0, 0, 0, 0, 0),
            "the release's gate: no pending fact, nothing skipped, no value refused"
        );
    }
}
