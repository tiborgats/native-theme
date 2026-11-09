//! The facts gate (S17 *The gate*, M1a task 14): each bundled preset
//! resolved with a stand-in and no reader, every widget's state-model
//! combinations and every token, colour-role and input fact compared with
//! the fact that applies; the perturbed run for derivations; the
//! declarations run. `facts_gate` runs it over `docs/facts` and the bundled
//! presets, the `gate_fixture_*` tests over fixtures, where every value is
//! synthetic.
//!
//! During development `facts_gate` can be limited to some presets:
//! `NT_FACTS_GATE_PRESETS=adwaita,macos-tahoe cargo test -p native-theme
//! --test facts_gate` gates those two alone, its report saying it ran a
//! subset. An id no bundled preset has, an empty name or an id named twice
//! fails the test; `none` runs no preset, the facts' own checks alone;
//! unset, every bundled preset runs. A subset is no gate:
//! `scripts/check_release.sh` gates every preset in every mode (`--commit`
//! and `--quick` one preset per step, beside `none`'s step, the facts' own
//! checks every step also runs), and the release's run
//! (`FACTS_GATE_RELEASE=1`) refuses a subset.
//!
//! `NT_FACTS_GATE_SHARD=k/N` runs the `k`th of `N` shards of the work
//! (`run::Shard`): each condition vector checked by one shard alone, the
//! rest by every shard as far as the vectors need it and recorded by the
//! first; its report names the shard. A preset's listed paths are found
//! over all its vectors, so no shard checks its `[pending]` table: each
//! writes the listed paths it found to the directory
//! `NT_FACTS_GATE_LISTED` names, and `facts_gate` with
//! `NT_FACTS_GATE_MERGE` naming that directory checks every preset's table
//! against all the shards' findings instead of gating. The shards and that
//! check find what the whole run finds (`gate_shards_add_up`). CI runs the
//! gate so, a job per shard, on one target with one feature set: no `cfg`
//! reaches the gate, so its outcome is one for all. A shard is no gate
//! alone, and the release's run refuses one.
//!
//! The gate's parallel steps run on as many threads as the machine has;
//! `NT_FACTS_GATE_THREADS=8` runs them on eight, and a value that is no
//! number above zero fails. The count sets the speed and the memory: as many
//! condition vectors are checked at once as there are threads, each one's
//! combinations enumerated as they are checked, a chunk at a time, never all
//! held, so each thread adds about 120 MB and the six presets peak near
//! 4 GB on 16 threads, 6 GB on 32 (`run::thread_count`).
//! `NT_FACTS_GATE_TIMING=1` prints, after each preset's and flavour's
//! widget checks, the time their steps took over every thread
//! (`timing::Step`); the report holds none of it.
//!
//! The citations' evidence is read under the evidence root (S17 *The
//! evidence root*, `facts::evidence_root`): the evidence repository checked
//! out where `NATIVE_THEME_EVIDENCE` names, else in the source cache
//! (`NATIVE_THEME_SOURCES`, else `scripts/citations.py`'s default) at
//! `native-theme-evidence/`, clean and at the commit `docs/facts/sources.toml`
//! pins (`scripts/update_sources.sh` fetches it); `facts_gate` fails without
//! one. A fixture's evidence lies under its own root.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "facts_gate/facts.rs"]
mod facts;
#[path = "facts_gate/model.rs"]
mod model;
#[path = "facts_gate/run.rs"]
mod run;
#[path = "facts_gate/timing.rs"]
mod timing;
#[path = "facts_gate/units.rs"]
mod units;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use native_theme::PresetId;
use native_theme::gate::{Combination, Preset, Query};

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

/// The environment variable that limits `facts_gate` to the presets it
/// names, comma-separated (the module's docs).
const PRESETS_VARIABLE: &str = "NT_FACTS_GATE_PRESETS";

/// The presets `facts_gate` runs, in [`PresetId::all`]'s order: every
/// bundled preset where `value`, [`PRESETS_VARIABLE`]'s value, is `None`,
/// none where it is `none` (the facts' own checks alone), else those its
/// comma-separated list names. An id no bundled preset has, an empty name
/// or an id named twice is an error, never an empty run.
fn selected_presets(value: Option<&str>) -> Result<Vec<PresetId>, String> {
    let Some(value) = value else {
        return Ok(PresetId::all().to_vec());
    };
    if value.trim() == "none" {
        return Ok(Vec::new());
    }
    let mut named = Vec::new();
    for name in value.split(',').map(str::trim) {
        let known = || {
            PresetId::all()
                .iter()
                .map(|id| id.key())
                .collect::<Vec<_>>()
                .join(", ")
        };
        let id = PresetId::from_key(name).ok_or_else(|| {
            format!(
                "`{name}` is no bundled preset's id (the bundled: {})",
                known()
            )
        })?;
        if named.contains(&id) {
            return Err(format!("`{name}` is named twice"));
        }
        named.push(id);
    }
    Ok(PresetId::all()
        .iter()
        .copied()
        .filter(|id| named.contains(id))
        .collect())
}

/// The environment variable naming the directory a shard of `facts_gate`
/// writes the listed paths it found to ([`write_listed`]).
const LISTED_VARIABLE: &str = "NT_FACTS_GATE_LISTED";

/// The environment variable naming the directory of every shard's listed
/// paths, which `facts_gate` then checks instead of gating ([`read_listed`],
/// [`check_listed`]).
const MERGE_VARIABLE: &str = "NT_FACTS_GATE_MERGE";

/// The directory the variable `name` names; `None` where it is unset.
fn directory_variable(name: &str) -> Option<PathBuf> {
    std::env::var_os(name).map(PathBuf::from)
}

/// The listed paths `report`, the shard `shard`'s report of a run over
/// `presets`, found (`Report::listed`), written to `directory` as
/// `listed-<k>-of-<N>.tsv`: a line naming the presets, then one per path,
/// its key, its register ids and its combinations, tab-separated.
fn write_listed(
    directory: &Path,
    shard: run::Shard,
    presets: &[(String, Preset)],
    report: &run::Report,
) {
    let ids: Vec<&str> = presets.iter().map(|(_, preset)| preset.id()).collect();
    let mut text = format!("presets\t{}\n", ids.join(","));
    for (key, (found, count)) in &report.listed {
        assert!(
            !key.contains(['\t', '\n']) && found.iter().all(|id| !id.contains(['\t', '\n', ' '])),
            "the listed path {key:?} ({found:?}) does not write as one line"
        );
        let found: Vec<&str> = found.iter().map(String::as_str).collect();
        text.push_str(&format!("{key}\t{}\t{count}\n", found.join(" ")));
    }
    std::fs::create_dir_all(directory).unwrap();
    let file = directory.join(format!("listed-{}-of-{}.tsv", shard.index + 1, shard.count));
    std::fs::write(&file, text).unwrap_or_else(|error| panic!("{}: {error}", file.display()));
}

/// The listed paths a run found (`Report::listed`): by key, the register
/// ids and the combinations.
type Listed = BTreeMap<String, (BTreeSet<String>, u64)>;

/// Every shard's listed paths in `directory` ([`write_listed`]), merged as
/// one run's: each path's ids joined and its combinations added. An error
/// where a file is no shard's, the shards name different counts, one of
/// them is missing, a file names other presets than `presets`, or a line
/// does not read.
fn read_listed(directory: &Path, presets: &[(String, Preset)]) -> Result<Listed, String> {
    let ids: Vec<&str> = presets.iter().map(|(_, preset)| preset.id()).collect();
    let header = format!("presets\t{}", ids.join(","));
    let mut shards = BTreeMap::new();
    let mut count = None;
    let entries = std::fs::read_dir(directory)
        .map_err(|error| format!("{}: {error}", directory.display()))?;
    for entry in entries {
        let path = entry.map_err(|error| error.to_string())?.path();
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let shard = name
            .strip_prefix("listed-")
            .and_then(|rest| rest.strip_suffix(".tsv"))
            .and_then(|rest| rest.split_once("-of-"))
            .and_then(|(k, n)| run::shard_of(Some(&format!("{k}/{n}"))).ok())
            .ok_or_else(|| format!("{name}: no shard's listed paths"))?;
        if *count.get_or_insert(shard.count) != shard.count {
            return Err(format!(
                "{name}: another count of shards than its neighbours'"
            ));
        }
        shards.insert(shard.index, path);
    }
    let count = count.ok_or_else(|| format!("{}: no shard's listed paths", directory.display()))?;
    if shards.len() != count {
        let missing: Vec<String> = (1..=count)
            .filter(|k| !shards.contains_key(&(k - 1)))
            .map(|k| k.to_string())
            .collect();
        return Err(format!(
            "the shards {} of {count} are missing",
            missing.join(", ")
        ));
    }
    let mut listed = Listed::new();
    for path in shards.values() {
        let text = std::fs::read_to_string(path)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        let mut lines = text.lines();
        if lines.next() != Some(header.as_str()) {
            return Err(format!(
                "{}: not a run over {}",
                path.display(),
                ids.join(", ")
            ));
        }
        for line in lines {
            let fields: Vec<&str> = line.split('\t').collect();
            let (Some(key), Some(found), Some(combinations), 3) =
                (fields.first(), fields.get(1), fields.get(2), fields.len())
            else {
                return Err(format!("{}: `{line}` does not read", path.display()));
            };
            let combinations: u64 = combinations
                .parse()
                .map_err(|_| format!("{}: `{line}` does not read", path.display()))?;
            let entry = listed.entry((*key).to_owned()).or_default();
            entry.0.extend(
                found
                    .split(' ')
                    .filter(|id| !id.is_empty())
                    .map(str::to_owned),
            );
            entry.1 += combinations;
        }
    }
    Ok(listed)
}

/// Each of `presets`' `[pending]` tables against the listed paths
/// `report` holds, the shards' findings merged (`run::listed_paths`).
fn check_listed(presets: &[(String, Preset)], report: &mut run::Report) {
    for (_, preset) in presets {
        run::listed_paths(preset, report);
    }
}

/// [`PRESETS_VARIABLE`]'s value; `None` where it is unset. A value that is
/// not Unicode fails.
fn presets_variable() -> Option<String> {
    match std::env::var(PRESETS_VARIABLE) {
        Ok(value) => Some(value),
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(value)) => {
            panic!("{PRESETS_VARIABLE} is not Unicode: {value:?}")
        }
    }
}

/// A fixture's evidence root: the case's own root, holding its own
/// `docs/evidence/` where it has one (S17 *The evidence root*), its facts
/// root being `<case>/docs/facts`.
fn fixture_evidence(root: &Path) -> &Path {
    root.parent().and_then(Path::parent).unwrap()
}

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
    run::run(&root, fixture_evidence(&root), &presets)
}

/// A listed path's register ids found, in order.
fn ids_of(ids: &BTreeSet<String>) -> Vec<&str> {
    ids.iter().map(String::as_str).collect()
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

/// A map fact is the value, not a patch (S17, the T6 review's S1): stated
/// whole (`Font.axes`, a `Value` of a map), it is compared whole, entry by
/// entry by key, never merged over the theme's map. The fact `{}` fails a
/// preset whose axes hold an extra axis; a fact of two axes agrees with a
/// preset stating them in another order and fails one stating fewer or
/// more.
#[test]
fn gate_value_map_fact_whole() {
    let axes = |report: &run::Report| failing(report, "font.axes").len();
    let empty = fixture_report("map_whole");
    assert_eq!(empty.failures, Vec::<String>::new());
    let extra = fixture_report_with("map_whole", "preset_extra.toml");
    assert!(axes(&extra) > 0, "{:?}", extra.failures);
    let order = fixture_report("map_whole_order");
    assert_eq!(order.failures, Vec::<String>::new());
    for file in ["preset_fewer.toml", "preset_more.toml"] {
        let report = fixture_report_with("map_whole_order", file);
        assert!(axes(&report) > 0, "{file}: {:?}", report.failures);
    }
}

/// A colour role's map fact (`role.global.accent_ramp`, stated whole) is
/// the value too (S17), read at the role's own place: the fact `{}` agrees
/// with a preset whose ramp is empty and fails one whose ramp holds an
/// entry.
#[test]
fn gate_role_map_fact_whole() {
    let ramp = |report: &run::Report| failing(report, "accent_ramp").len();
    let empty = fixture_report("map_whole_role");
    assert_eq!(empty.failures, Vec::<String>::new());
    let extra = fixture_report_with("map_whole_role", "preset_extra.toml");
    assert!(ramp(&extra) > 0, "{:?}", extra.failures);
}

/// A plain map's fact (`token.icons.mirroring`, a `Vec<(K, V)>` stated
/// whole) is the value too (S17, the T6 review's S1): the fact `{}` fails
/// a preset whose map holds an entry and agrees with one holding none; a
/// fact of that entry agrees with the preset holding it and fails one
/// holding none.
#[test]
fn gate_plain_map_fact_whole() {
    let mirroring = |report: &run::Report| failing(report, "icons.mirroring").len();
    let empty = fixture_report("map_whole_plain");
    assert_eq!(empty.failures, Vec::<String>::new());
    let extra = fixture_report_with("map_whole_plain", "preset_extra.toml");
    assert!(mirroring(&extra) > 0, "{:?}", extra.failures);
    let entry = fixture_report("map_whole_plain_entry");
    assert_eq!(entry.failures, Vec::<String>::new());
    let none = fixture_report_with("map_whole_plain_entry", "preset_empty.toml");
    assert!(mirroring(&none) > 0, "{:?}", none.failures);
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

/// A block's fact and its leaf's later fact (S17 *Which fact applies*):
/// the focus ring stated whole by a reference, then its width where the
/// button is not focused. The leaf's fact applies to the leaf there, the
/// block's to the rest of the block and to the leaf elsewhere, so a preset
/// patching the width over the reference passes, with no leaf lacking a
/// fact; its twin leaving the token's width fails at the leaf alone.
#[test]
fn gate_fixture_leaf_facts_over_whole_block() {
    let report = fixture_report("leaf_over_block");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
    let wrong = fixture_report_with("leaf_over_block", "wrong_width.toml");
    assert!(
        !failing(&wrong, "Normal: `button.body.focus.width` at").is_empty(),
        "{:?}",
        wrong.failures
    );
    assert!(
        wrong
            .failures
            .iter()
            .all(|failure| failure.contains("`button.body.focus.width` at")),
        "{:?}",
        wrong.failures
    );
}

/// A whole-block fact after its leaf's (S17 *Which fact applies*: the last
/// applying fact in file order, the block's counting for each of its
/// leaves): the block's later fact applies to the width wherever the
/// leaf's holds, so the token's width passes everywhere and a preset
/// stating the leaf's zero where the button is not focused fails at the
/// block's width; neither a leaf skipped only where no leaf fact applies
/// nor every leaf with an applying fact grafted out of the block passes it.
#[test]
fn gate_fixture_whole_block_over_leaf_facts() {
    let report = fixture_report("block_over_leaf");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
    let wrong = fixture_report_with("block_over_leaf", "wrong_width.toml");
    let width = failing(&wrong, "Normal: `button.body.focus` at");
    assert!(!width.is_empty(), "{:?}", wrong.failures);
    assert!(
        wrong
            .failures
            .iter()
            .all(|failure| failure.contains("`button.body.focus` at")),
        "{:?}",
        wrong.failures
    );
}

/// A block stated per leaf and whole after its leaves under a narrower
/// `when` (S17 *Which fact applies*): where the block's fact does not hold,
/// its leaves' facts state it, so the block is no key without a fact
/// there; on a flat button the block's `absent` applies.
#[test]
fn gate_fixture_block_beside_its_leaves() {
    let report = fixture_report("block_beside_leaves");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
}

/// A token's block stated whole and its leaf after it (S17 *Which fact
/// applies*, for every key): the block's fact compares the block but the
/// leaf where the leaf's later fact applies, which compares the leaf; a
/// preset patching the leaf there passes, its twin stating the block's
/// value there fails at the leaf alone.
#[test]
fn gate_fixture_token_leaf_over_whole_block() {
    let report = fixture_report("token_leaf_over_block");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
    let wrong = fixture_report_with("token_leaf_over_block", "wrong_top.toml");
    assert!(
        !failing(&wrong, "Normal: `token.focus.offset.top` at").is_empty(),
        "{:?}",
        wrong.failures
    );
    assert!(
        wrong
            .failures
            .iter()
            .all(|failure| failure.contains("`token.focus.offset.top` at")),
        "{:?}",
        wrong.failures
    );
}

/// A block stated whole and its leaf pending after it, listed (S17 *Which
/// fact applies*, the synthetic route), a token's and a part field's: the
/// leaf's synthetic value is left out of the block's comparison with the
/// leaf, so the block's other leaves are still compared; a preset stating
/// a wrong top fails the block there, and nothing else.
#[test]
fn gate_fixture_block_beside_pending_leaf() {
    let report = fixture_report("block_pending_leaf");
    assert_eq!(report.failures, Vec::<String>::new());
    assert_eq!(
        report.listed.keys().collect::<Vec<_>>(),
        [
            "probe button.body.margin.left",
            "probe token.focus.offset.left"
        ]
    );
    for (file, key) in [
        ("wrong_top.toml", "`token.focus.offset` at"),
        ("wrong_margin.toml", "`button.body.margin` at"),
    ] {
        let wrong = fixture_report_with("block_pending_leaf", file);
        assert!(
            !failing(&wrong, &format!("Normal: {key}")).is_empty(),
            "{file}: {:?}",
            wrong.failures
        );
        assert!(
            wrong.failures.iter().all(|failure| failure.contains(key)),
            "{file}: {:?}",
            wrong.failures
        );
    }
}

/// A text style's fact reading its own font size (S2.2: `font_size` in a
/// text style is its font's size; the query evaluates it so): the fact's
/// expression, evaluated in the key's place, gives the preset's line
/// height, and a preset stating another multiple fails it.
#[test]
fn gate_fixture_text_style_element_operand() {
    let report = fixture_report("text_style_operand");
    assert_eq!(report.failures, Vec::<String>::new());
    let wrong = fixture_report_with("text_style_operand", "wrong.toml");
    let line_height = failing(&wrong, "`token.text.keyed.body.line_height` at");
    assert!(!line_height.is_empty(), "{:?}", wrong.failures);
    assert!(
        line_height
            .iter()
            .all(|failure| failure.contains("gives Px(16.8)")),
        "{line_height:?}"
    );
}

/// An icon-size context's leaves stated per leaf (`token.icons.sizes.keyed.
/// status_page.size`): each fact states its leaf below the keyed entry, the
/// entry's other leaves the preset's, so the gate reads the fact's value
/// there and a preset stating another size fails at the size alone.
#[test]
fn gate_fixture_icon_size_keyed() {
    let report = fixture_report("icon_size_keyed");
    assert_eq!(report.failures, Vec::<String>::new());
    let wrong = fixture_report_with("icon_size_keyed", "wrong.toml");
    assert!(
        !failing(&wrong, "`token.icons.sizes.keyed.status_page.size` at").is_empty(),
        "{:?}",
        wrong.failures
    );
    assert!(
        wrong
            .failures
            .iter()
            .all(|failure| failure.contains("`token.icons.sizes.keyed.status_page.size` at")),
        "{:?}",
        wrong.failures
    );
}

/// A token key of a unit not enabled (S17 *While 0.8.0 is built*: "the
/// gate skips it while any of those units is not enabled"): the units are
/// `scripts/widget_units.toml`'s, as check_facts_trace.py reads them, whose
/// slice holds `token.params.pointer_repeat`, `.color` and `.opacity` and
/// no other parameter group, so `token.params.caret`'s fact is skipped and
/// counted, never compared, though the model holds the keyed map.
#[test]
fn gate_fixture_token_of_unit_not_enabled() {
    let report = fixture_report("unit_not_enabled");
    assert_eq!(report.failures, Vec::<String>::new());
    assert_eq!(
        report
            .units_not_enabled
            .get("token.params.caret.aspect_ratio"),
        Some(&1),
        "{:?}",
        report.units_not_enabled
    );
    let units = units::Units::repository().unwrap();
    assert!(units.hold("token.params.pointer_repeat.delay"));
    assert!(units.hold("token.params"));
    assert!(!units.hold("token.params.caret.aspect_ratio"));
    assert!(!units.hold("token.behavior.scroll.lines_per_step"));
    assert!(!units.hold("button.values.menu_placement"));
    assert!(units.hold("button.body.min_width_px"));
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

/// A preset of the case `name`'s `preset.toml` over the base whose text
/// `edit` changes (the base's `[preset]` and `[declare]` keys, which a
/// case's appended file cannot restate), checked against its `gnome`
/// directory.
fn fixture_edited(name: &str, edit: impl Fn(String) -> String) -> (PathBuf, Vec<(String, Preset)>) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/facts_gate");
    let base = edit(std::fs::read_to_string(fixtures.join("base.toml")).unwrap());
    let case = fixtures.join(name);
    let preset = Preset::from_toml(&fixture_text(&case, &base, "preset.toml"))
        .unwrap_or_else(|error| panic!("{name}: {error}"));
    (case.join("docs/facts"), vec![("gnome".to_owned(), preset)])
}

/// The base declaring the input `tint` beside the `accent`.
fn with_tint(base: String) -> String {
    let edited = base.replacen(
        "inputs = [\"accent\"]",
        "inputs = [\"accent\", \"tint\"]",
        1,
    );
    assert_ne!(edited, base);
    edited
}

/// The perturbed stand-in's inputs for the preset of `root` and `presets`
/// under the light scheme with no other condition (S17).
fn stand_in_inputs(root: &Path, presets: &[(String, Preset)]) -> Vec<String> {
    let mut report = run::Report::default();
    let units = units::Units::repository().unwrap();
    let platform = run::Platform::read(&root.join("gnome"), &units, &mut report);
    let preset = &presets[0].1;
    let evidence = facts::Evidence::new(fixture_evidence(root));
    let keys = run::prepare(&platform, preset.id(), &evidence, &mut report);
    let conditions = run::conditions(&["scheme:light".to_owned()], None).unwrap();
    let resolved = preset
        .resolve(&conditions, &native_theme::Layer::default())
        .unwrap();
    let layer = run::stand_in(preset, &keys, &resolved).unwrap();
    layer
        .bases
        .inputs
        .entries
        .iter()
        .map(|(name, _)| name.to_string())
        .collect()
}

/// The perturbed stand-in states every declared input but one whose fact
/// that applies is a derivation (S17, ADD-A132): `tint`, derived from the
/// accent, follows the perturbed accent and is compared, where stating it
/// would compare the stand-in's own colour with the derivation and fail.
#[test]
fn gate_fixture_derived_input_perturbed() {
    let (root, presets) = fixture_edited("derived_input", with_tint);
    assert_eq!(stand_in_inputs(&root, &presets), ["accent"]);
    let report = run::run(&root, fixture_evidence(&root), &presets);
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
}

/// The perturbed pass replays the normal pass's probes where its
/// resolution reads alike (S17: "reusing what the set's rules share"): the
/// normal and the perturbed resolution of a scheme are alike, as the
/// stand-in states colours and inputs alone; two schemes whose rules
/// differ are not, where the perturbed pass would probe its own
/// combinations.
#[test]
fn gate_probes_alike_reads_the_steps() -> Result<(), String> {
    let (root, presets) = fixture("surface_operand", "preset.toml");
    let mut report = run::Report::default();
    let units = units::Units::repository().map_err(failed)?;
    let platform = run::Platform::read(&root.join("gnome"), &units, &mut report);
    let preset = &presets.first().ok_or("the fixture has no preset")?.1;
    let evidence = facts::Evidence::new(fixture_evidence(&root));
    let keys = run::prepare(&platform, preset.id(), &evidence, &mut report);
    let resolve = |scheme: &str, layer: &native_theme::Layer| {
        let conditions = run::conditions(&[format!("scheme:{scheme}")], None).map_err(failed)?;
        preset.resolve(&conditions, layer)
    };
    let light = resolve("light", &native_theme::Layer::default())?;
    let layer = run::stand_in(preset, &keys, &light)?;
    assert!(!layer.bases.inputs.entries.is_empty());
    let inputs = run::root_inputs();
    assert!(light.probes_alike(&resolve("light", &layer)?, &inputs));
    assert!(!light.probes_alike(&resolve("dark", &native_theme::Layer::default())?, &inputs));
    Ok(())
}

/// A test's error, written as its value's debug form.
fn failed(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

/// A session of shapes alone (`Query::Shapes`, which the gate's probes
/// query) answers each part as a session of styles does and matches the
/// same steps (S17: the probe reads each part's answer and the scope it
/// passes on, never a value): on every fixture's preset, under each
/// scheme, for each set of the button's flags, with no variant or one the
/// preset declares, at the default size and each one it declares, the
/// two sessions' signatures agree, and the shapes' `probed` gives what
/// the styles' `holds_each` and `signature` give.
#[test]
fn gate_shapes_probe_as_styles() -> Result<(), String> {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/facts_gate");
    let base = std::fs::read_to_string(fixtures.join("base.toml")).map_err(failed)?;
    let mut cases: Vec<PathBuf> = std::fs::read_dir(&fixtures)
        .map_err(failed)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|case| case.join("preset.toml").is_file())
        .collect();
    cases.sort();
    let flags = ["hover", "pressed", "disabled", "checked", "focus_visible"];
    // `when`s with no conditions, so each holds in every resolution, on a
    // part's field, a whole part and a token (`Session::probed`).
    let read = |(when, key): (&str, &str)| {
        let when = toml::from_str::<toml::Value>(when).map_err(failed)?;
        Ok::<_, String>((
            native_theme::gate::parse_when(&when).map_err(failed)?,
            native_theme::gate::parse_key(key).map_err(failed)?,
        ))
    };
    let parsed: Vec<(native_theme::gate::When, native_theme::gate::FactKey)> = [
        ("states = [\"hover\"]", "button.body.fill"),
        (
            "states = [\"pressed\"]\nnot_states = [\"disabled\"]",
            "button.body.min_width",
        ),
        ("variants = [\"flat\"]", "button.body.fill"),
        ("sizes = [\"small\"]", "button.label"),
        ("context = [{ widget = \"button\" }]", "button.body.fill"),
        ("states = [\"focus_visible\"]", "button.icon"),
        ("states = [\"checked\"]", "token.focus.width"),
    ]
    .into_iter()
    .map(read)
    .collect::<Result<_, _>>()?;
    let whens: Vec<(&native_theme::gate::When, &native_theme::gate::FactKey)> =
        parsed.iter().map(|(when, key)| (when, key)).collect();
    let root = run::root_inputs();
    let metrics = run::StubMetrics;
    let mut sessions = 0_usize;
    for case in &cases {
        let Ok(preset) = Preset::from_toml(&fixture_text(case, &base, "preset.toml")) else {
            continue;
        };
        let Ok(declared) = preset.declared("button", None) else {
            continue;
        };
        let variants: Vec<Vec<String>> = std::iter::once(Vec::new())
            .chain(
                declared
                    .variants
                    .iter()
                    .map(|variant| vec![variant.clone()]),
            )
            .collect();
        let sizes: Vec<Option<String>> = std::iter::once(None)
            .chain(declared.sizes.iter().cloned().map(Some))
            .collect();
        for scheme in preset.schemes() {
            let conditions =
                run::conditions(&[format!("scheme:{scheme}")], None).map_err(failed)?;
            let resolved = preset.resolve(&conditions, &native_theme::Layer::default())?;
            for set in 0..1_usize << flags.len() {
                for variant in &variants {
                    for size in &sizes {
                        let mut combination = Combination::new("button");
                        combination.states = flags
                            .iter()
                            .enumerate()
                            .filter(|(at, _)| set & (1 << at) != 0)
                            .map(|(_, flag)| (*flag).to_owned())
                            .collect();
                        combination.variants.clone_from(variant);
                        combination.size.clone_from(size);
                        let session = |query| {
                            resolved
                                .session_drawn(&combination, &root, &metrics, &[], query)
                                .map_err(failed)?
                                .ok_or_else(|| format!("{combination:?} is named not drawn"))
                        };
                        let (styles, shapes) = (session(Query::Styles)?, session(Query::Shapes)?);
                        assert_eq!(
                            (styles.unavailable(), styles.signature()),
                            (shapes.unavailable(), shapes.signature()),
                            "{case:?} {scheme} {combination:?}"
                        );
                        assert_eq!(
                            shapes.probed(&whens),
                            (styles.holds_each(&whens), styles.signature()),
                            "{case:?} {scheme} {combination:?}"
                        );
                        sessions += 1;
                    }
                }
            }
        }
    }
    assert!(sessions > 1000, "{sessions}");
    Ok(())
}

/// A probe's failure is recorded under each pass's label, the perturbed
/// pass's in the normal pass's order, where it replays the normal pass's
/// probes as where it probes its own: the small size, which the preset
/// does not declare, is not available in either.
#[test]
fn gate_fixture_perturbed_probe_failures_replayed() {
    let report = fixture_report("not_drawn_unavailable");
    let unavailable = |label: &str| -> Vec<String> {
        report
            .failures
            .iter()
            .filter_map(|failure| failure.strip_prefix(label))
            .filter(|failure| failure.starts_with("not available"))
            .map(str::to_owned)
            .collect()
    };
    let normal = unavailable("Normal: ");
    assert!(!normal.is_empty(), "{:?}", report.failures);
    assert_eq!(unavailable("Perturbed: "), normal);
}

/// On `kde-breeze` the perturbed stand-in states no `accent` (S9.2, S17,
/// ADD-A132): no accent-derivation layer is derived, the preset's
/// no-accent derivations are compared, and `kde_accent` is the accent
/// run's; on another preset it states it.
#[test]
fn gate_perturbed_stand_in_states_no_accent_on_kde_breeze() {
    let (root, presets) = fixture("passes", "preset.toml");
    assert_eq!(stand_in_inputs(&root, &presets), ["accent"]);
    let (root, presets) = fixture_edited("passes", |base| {
        let edited = base.replacen("id = \"probe\"", "id = \"kde-breeze\"", 1);
        assert_ne!(edited, base);
        edited
    });
    assert_eq!(presets[0].1.id(), "kde-breeze");
    assert!(stand_in_inputs(&root, &presets).is_empty());
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

/// `gate_fixture_present` (S6.3, S17, ADD-A126): a whole part's
/// `value = "present"`, after the fact that makes it absent, is compared
/// with any style the query answers. A preset drawing the part again by
/// `absent = false` where the facts do passes, its field compared there,
/// as its twin drawing it there with another text fails at that field
/// alone; its twin patching the part's field alone there leaves it absent
/// and fails at the part's key alone, under the flat button.
#[test]
fn gate_fixture_present() {
    let report = fixture_report("present");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.fields_not_drawn > 0, "{report:?}");
    let text = fixture_report_with("present", "text.toml");
    assert!(!text.failures.is_empty(), "{text:?}");
    assert!(
        text.failures
            .iter()
            .all(|failure| failure.contains("`button.subtitle.text` at")
                && failure.contains("[\"flat\"]")),
        "{:?}",
        text.failures
    );
    let fields = fixture_report_with("present", "fields.toml");
    assert!(!fields.failures.is_empty(), "{fields:?}");
    assert!(
        fields
            .failures
            .iter()
            .all(|failure| failure.contains("`button.subtitle` at")
                && failure.contains("[\"flat\"]")
                && failure.contains("NotDrawn")),
        "{:?}",
        fields.failures
    );
}

/// A fact's table at a `Value` struct block is the whole block (S17: "by a
/// table (the whole block, as the preset layer's bases spell it, S2.2)"),
/// so where the preset leaves the block absent it fails, as a `Fields`
/// patch setting nothing over `Absent` would not (S2.2, S6.1); the preset
/// whose rule states it by `{ replace = { … } }` passes.
#[test]
fn gate_fixture_whole_table_over_absent_base() {
    let report = fixture_report("whole_table");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
    let absent = fixture_report_with("whole_table", "absent.toml");
    assert!(!absent.failures.is_empty(), "{absent:?}");
    assert!(
        absent
            .failures
            .iter()
            .all(|failure| failure.contains("`button.icon.glyph` at")
                && failure.contains("[\"flat\"]")),
        "{:?}",
        absent.failures
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
            "sizes_by_variant",
            format!(
                "{sized}sizes_by_variant = [{{ variant = \"flat\", sizes = [\"small\", \"regular\"], \
                 default_size = \"small\" }}]\n"
            ),
            format!(
                "{sized}sizes_by_variant = [{{ variant = \"flat\", sizes = [\"small\", \"regular\"] }}]\n"
            ),
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
            "recency",
            "recency = { lift = [\"pressed\", \"hover\"] }\n".to_owned(),
            "recency = { lift = [\"hover\", \"pressed\"] }\n".to_owned(),
        ),
        (
            "requires_variants",
            "requires_variants = [\"flat\", \"primary\"]\n".to_owned(),
            String::new(),
        ),
        (
            "sizes_by_variant",
            format!(
                "{sized}sizes_by_variant = [{{ variant = \"flat\", sizes = [\"small\"], \
                 default_size = \"small\" }}]\n"
            ),
            sized.to_owned(),
        ),
        (
            "sizes_by_variant",
            format!(
                "{sized}sizes_by_variant = [{{ variant = \"flat\", sizes = [\"small\"], \
                 default_size = \"small\" }}, {{ variant = \"primary\", sizes = [\"regular\"] }}]\n"
            ),
            format!(
                "{sized}sizes_by_variant = [{{ variant = \"primary\", sizes = [\"regular\"] }}, \
                 {{ variant = \"flat\", sizes = [\"small\"], default_size = \"small\" }}]\n"
            ),
        ),
        (
            "sizes_by_context",
            format!(
                "{sized}sizes_by_context = [{{ context = {{ widget = \"button\" }}, sizes = \
                 [\"small\"], default_size = \"small\" }}]\n"
            ),
            sized.to_owned(),
        ),
        (
            "sizes_by_context",
            format!(
                "{sized}sizes_by_context = [{{ context = {{ widget = \"button\" }}, sizes = \
                 [\"small\"], default_size = \"small\" }}]\n"
            ),
            format!(
                "{sized}sizes_by_context = [{{ context = {{ widget = \"button\", part = \
                 \"body\" }}, sizes = [\"small\"], default_size = \"small\" }}]\n"
            ),
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
    // A pending default size: the preset lists its declaration, the state
    // model states it pending, compared with its ids (S10's *A pending
    // default*, ADD-A131).
    let pending = |listed: &str, stated: &str| {
        let declare = format!(
            "[declare.widget.button]\nvariants = [\"flat\", \"primary\"]\n\
             sizes = [\"small\", \"regular\"]\n\
             sizes_by_variant = [{{ variant = \"flat\", sizes = [\"small\", \"regular\"] }}]\n\
             [pending]\n\"declare.widget.button.sizes_by_variant.flat.default_size\" = \"{listed}\"\n\
             \"declare.widget.button.default_size\" = \"U1\"\n"
        );
        let preset = Preset::from_toml(&format!("{base}\n{declare}"))
            .unwrap_or_else(|error| panic!("{declare}: {error}"));
        let stated = format!(
            "sizes = [\"small\", \"regular\"]\ndefault_size = {{ pending = \"U1\" }}\n\
             sizes_by_variant = [{{ variant = \"flat\", sizes = [\"small\", \"regular\"], \
             default_size = {stated} }}]\n"
        );
        let platform = run::Platform {
            name: "gnome".to_owned(),
            facts: Vec::new(),
            keys: Vec::new(),
            models: model::Model::read("gnome/state-models.toml", &model_of(&stated)).unwrap(),
        };
        let mut report = run::Report::default();
        run::declarations_run(&platform, &preset, None, &mut report);
        report.failures
    };
    assert_eq!(pending("U2", "{ pending = \"U2\" }"), Vec::<String>::new());
    // A default both stated and listed: the declarations run agrees with a
    // model stating it, and the [pending] table's entry fails; listed
    // alone, it is recorded with its ids (S17, ADD-A131).
    let listed = |stated: &str| {
        let preset = Preset::from_toml(&format!(
            "{base}\n[declare.widget.button]\nvariants = [\"flat\", \"primary\"]\nsizes = [\"small\"]\n\
             {stated}[pending]\n\"declare.widget.button.default_size\" = \"U4\"\n"
        ))
        .unwrap();
        let platform = run::Platform {
            name: "gnome".to_owned(),
            facts: Vec::new(),
            keys: Vec::new(),
            models: model::Model::read(
                "gnome/state-models.toml",
                &model_of("sizes = [\"small\"]\ndefault_size = \"small\"\n"),
            )
            .unwrap(),
        };
        let mut report = run::Report::default();
        run::declarations_run(&platform, &preset, None, &mut report);
        let declared = report.failures.clone();
        report.failures.clear();
        run::listed_paths(&preset, &mut report);
        (declared, report)
    };
    let (declared, stated) = listed("default_size = \"small\"\n");
    assert_eq!(declared, Vec::<String>::new());
    assert_eq!(
        stated.failures,
        [
            "probe: `declare.widget.button.default_size` (U4) is in its [pending] table, and the \
             preset states it"
        ]
    );
    assert!(stated.listed.is_empty(), "{:?}", stated.listed);
    let (_, alone) = listed("");
    assert_eq!(alone.failures, Vec::<String>::new());
    assert_eq!(
        alone
            .listed
            .iter()
            .map(|(key, (ids, _))| (key.as_str(), ids_of(ids)))
            .collect::<Vec<_>>(),
        [("probe declare.widget.button.default_size", vec!["U4"])]
    );
    for stated in ["{ pending = \"U3\" }", "\"small\""] {
        let failures = pending("U2", stated);
        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("`button` sizes_by_variant under")),
            "{stated}: {failures:?}"
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

/// A state model's context selector stating an empty list spells as the
/// one without the key, as the crate's selector holds none (the A131a
/// review's M1): `states = []` and `variants = []` select as no key does.
#[test]
fn gate_selector_empty_list_spelled_as_none() {
    let spelled = |text: &str| {
        let table: toml::Table = toml::from_str(&format!("context = {text}")).unwrap();
        model::selector_spelled(&table["context"])
    };
    let bare = spelled("{ widget = \"toolbar\" }");
    assert_eq!(spelled("{ widget = \"toolbar\", states = [] }"), bare);
    assert_eq!(spelled("{ widget = \"toolbar\", variants = [] }"), bare);
    assert_ne!(
        spelled("{ widget = \"toolbar\", states = [\"hover\"] }"),
        bare
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

/// A fact whose `when` excludes a flavour (`not_conditions` naming it, or
/// `conditions` naming another) is no combination of that flavour's run
/// (S10, S17): its variant `@main_only`, the default flavour's alone, is
/// checked in the default flavour's run, where the preset states it, and
/// never queried under `flavor:alt`, whose declarations lack it.
#[test]
fn gate_fixture_flavor_excluded_fact() {
    let report = fixture_report("flavor_excluded");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
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

/// A pending fact a later pending fact shadows where both hold is exempt
/// (S17 *Which fact applies*, ADD-A135): no failure, the combination
/// pending under the later fact, both listed.
#[test]
fn gate_fixture_pending_shadowed_by_pending() {
    let report = fixture_report("pending_later_pending");
    assert_eq!(report.failures, Vec::<String>::new());
    let ids: BTreeSet<&str> = report
        .pending
        .values()
        .map(|(id, _, _)| id.as_str())
        .collect();
    assert_eq!(ids, BTreeSet::from(["U1", "U2"]), "{:?}", report.pending);
    assert!(
        report.pending.values().all(|(_, _, count)| *count > 0),
        "{:?}",
        report.pending
    );
}

/// The exemption is a later pending fact's alone: a pending fact a later
/// stated fact shadows fails, shadowed by that fact, though a pending fact
/// later still shadows both there.
#[test]
fn gate_fixture_pending_shadowed_by_stated_before_pending() {
    let report = fixture_report("pending_stated_between");
    assert!(!report.failures.is_empty(), "{report:?}");
    assert!(
        report.failures.iter().all(|failure| failure.contains(
            "gnome/button.toml fact 6: the pending fact U1 of `button.body.opacity` is shadowed \
             by gnome/button.toml fact 8 under"
        )),
        "{:?}",
        report.failures
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

/// An `axes.contexts` entry that lists selectors is a chain of ancestors,
/// outermost first, each the element inside the one before (S17): the
/// preset's rule naming both ancestors holds in it, and the fact naming the
/// nearest button the inner, flat one; a value wrong there alone fails, in
/// the chain's combinations, on the key and never on the context's reading.
#[test]
fn gate_fixture_context_chain() {
    let report = fixture_report("context_chain");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.queried > 0, "{report:?}");
    let wrong = fixture_report_with("context_chain", "wrong.toml");
    assert!(!wrong.failures.is_empty(), "{wrong:?}");
    assert!(
        wrong
            .failures
            .iter()
            .all(|failure| failure.contains("button.body.min_width")
                && failure.contains("context: Some(Array(")),
        "{:?}",
        wrong.failures
    );
}

/// A `not_drawn` entry naming a size is evaluated by the count and by the
/// run (S17: `not_drawn` entries are `when`s, which state `sizes`, S11):
/// the preset's wrong value at the small size is never queried where the
/// entry names pressed, and fails where hover is drawn; the count leaves
/// out the small size's pressed combinations alone, and a named density's
/// likewise.
#[test]
fn gate_fixture_not_drawn_sizes() {
    let report = fixture_report("not_drawn_sizes");
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
    let state_model = |not_drawn: &str| {
        format!(
            "[[model]]\nwidget = \"button\"\navailable = true\nvariants = []\n\
             sizes = [\"small\", \"regular\"]\ndensities = []\nexclusive = []\nrows = []\n\
             axes = {{ states = [\"pressed\"], conditions = [], contexts = [] }}\n{not_drawn}"
        )
    };
    let count = |text: &str| {
        let models = model::Model::read("probe", text).unwrap();
        model::count(&models[0], &[Vec::new()])
    };
    assert_eq!(count(&state_model("")), 4);
    assert_eq!(
        count(&state_model(
            "not_drawn = [{ states = [\"pressed\"], sizes = [\"small\"] }]\n"
        )),
        3
    );
    assert_eq!(
        count(&state_model(
            "not_drawn = [{ states = [\"pressed\"], not_sizes = [\"small\"] }]\n"
        )),
        3
    );
    // Densities likewise: none and each declared one, an entry naming one
    // leaving out its pressed combination, one naming it in
    // `not_densities` every other density's.
    let densities = |not_drawn: &str| {
        state_model(not_drawn).replacen(
            "sizes = [\"small\", \"regular\"]\ndensities = []",
            "sizes = []\ndensities = [\"compact\", \"spacious\"]",
            1,
        )
    };
    assert_eq!(count(&densities("")), 6);
    assert_eq!(
        count(&densities(
            "not_drawn = [{ states = [\"pressed\"], densities = [\"compact\"] }]\n"
        )),
        5
    );
    assert_eq!(
        count(&densities(
            "not_drawn = [{ states = [\"pressed\"], not_densities = [\"compact\"] }]\n"
        )),
        4
    );
}

/// A `not_drawn` entry naming a combination that is not available leaves
/// it failed as not available (S8, S17): the entry is matched against the
/// root's element as the query reads it, which a size the preset does not
/// declare never is, so asking the entries before the parts are queried
/// changes no verdict.
#[test]
fn gate_fixture_not_drawn_unavailable() {
    let report = fixture_report("not_drawn_unavailable");
    assert!(report.queried > 0, "{report:?}");
    let unavailable = failing(&report, "not available (Size)");
    assert!(!unavailable.is_empty(), "{:?}", report.failures);
    assert!(
        unavailable
            .iter()
            .all(|failure| failure.contains("size: Some(\"small\")")),
        "{unavailable:?}"
    );
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
    let direction = entry("not_drawn = [{ states = [\"pressed\"], direction = \"rtl\" }]\n");
    let error = model::Model::read("probe", &direction).unwrap_err();
    assert!(error.contains("`direction`"), "{error}");
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

/// A fact whose `when` names the preset checked (`preset:adwaita`, as the
/// facts of a directory two presets share do, S17 *Generations*) is
/// planned under its `when` with that condition decided, never as a
/// condition vector naming the preset, which no stand-in sets.
#[test]
fn gate_fixture_preset_condition_decided() {
    let report = fixture_report("preset_condition");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
}

/// A measured derivation's pass varies the accent alone on a preset whose
/// accent inputs are two (S9.1): every other operand resolves from the
/// preset (S17 *A measured derivation*), so the pass's stand-in states the
/// preset's own `highlight` beside the varied accent, the accent inputs as
/// one group (S9.2), and each pass resolves and agrees, its result mixing
/// in the preset's `highlight` at the pass's scheme.
#[test]
fn gate_fixture_derivation_pass_states_the_accent_group() {
    let report = fixture_report("derivation_accent_group");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(report.compared > 0, "{report:?}");
}

/// Only a measured derivation's pass completes the accent group with the
/// preset's own values, as every other operand of its fact resolves from
/// the preset (S17 *A measured derivation*); any other stand-in, the
/// perturbed run's among them, is validated as `Theme` validates it, one
/// stating some of the group at a scheme refused (S9.2): a stand-in stating
/// the accent alone, where `highlight` is an accent input too, is refused
/// by `resolve` and resolved by `resolve_pass` with the preset's own
/// `highlight`.
#[test]
fn gate_accent_group_completed_for_a_pass_alone() {
    let (_, presets) = fixture("derivation_accent_group", "preset.toml");
    let preset = &presets[0].1;
    let conditions = run::conditions(&["scheme:light".to_owned()], None).unwrap();
    let stand_in =
        native_theme::Layer::from_toml("[inputs.accent]\nlight = \"#ff0000\"\ndark = \"#ff0000\"")
            .unwrap();
    let refused = preset
        .resolve(&conditions, &stand_in)
        .err()
        .unwrap_or_default();
    assert!(refused.contains("highlight"), "{refused:?}");
    assert!(preset.resolve_pass(&conditions, &stand_in).is_ok());
}

/// The same pass with the preset's `highlight` listed in the light scheme
/// (S17, ADD-A132): the stand-in carries its folded synthetic value as the
/// accent group's, still marked, so the light passes reading it are
/// skipped, never compared with the synthetic value; the dark ones, reading
/// the preset's stated value, are compared and agree.
#[test]
fn gate_fixture_derivation_pass_carries_a_listed_input() {
    let report = fixture_report("derivation_listed_accent");
    assert_eq!(report.failures, Vec::<String>::new());
    assert_eq!(
        report.listed.keys().collect::<Vec<_>>(),
        ["probe input.highlight light"]
    );
    let skipped: Vec<&String> = report.synthetic.values().map(|(key, _)| key).collect();
    assert_eq!(skipped, ["button.body.fill"], "{:?}", report.synthetic);
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
        report.listed.values().next().map(|(ids, _)| ids_of(ids)),
        Some(vec!["U5"])
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

/// A field S6.1 sets whole listed with a list of ids, its leaves pending
/// under different ones (S11: `"tokens.behavior.key_repeat" = ["<id>",
/// "<id>"]`): found with both; listed with one of them, it fails, naming
/// both.
#[test]
fn gate_fixture_pending_ids() {
    let report = fixture_report("pending_ids");
    assert_eq!(report.failures, Vec::<String>::new());
    assert_eq!(
        report.listed.keys().collect::<Vec<_>>(),
        ["probe token.behavior.key_repeat"]
    );
    assert_eq!(
        report.listed.values().next().map(|(ids, _)| ids_of(ids)),
        Some(vec!["U3", "U4"])
    );
    let one = fixture_report_with("pending_ids", "one_id.toml");
    assert_eq!(
        failing(&one, "[pending]"),
        [
            &"probe: `tokens.behavior.key_repeat` is listed as U3 in its [pending] table, and \
           its pending facts are U3, U4"
                .to_owned()
        ],
        "{:?}",
        one.failures
    );
}

/// An input listed as pending is found per scheme (S17: "a scheme at which
/// its per-scheme value has only pending facts and nothing beneath them is
/// listed"; S11 writes it `inputs.<name>.<scheme>`, ADD-A132); listed
/// while the preset states it, it fails at each scheme. Its pending
/// high-contrast fact, a contrast base never listed, is a pending fact
/// met, its id no listed input's (the A131b review's SF2). A role read from
/// the folded input is skipped by the input's mark, its literal fact never
/// compared with the synthetic value (the review's M4).
#[test]
fn gate_fixture_pending_input() {
    let report = fixture_report("pending_input");
    assert_eq!(report.failures, Vec::<String>::new());
    assert!(
        report
            .pending
            .values()
            .any(|(id, key, applied)| id == "U10" && key == "input.accent" && *applied > 0),
        "{:?}",
        report.pending
    );
    let skipped: Vec<&String> = report.synthetic.values().map(|(key, _)| key).collect();
    assert_eq!(
        skipped,
        ["role.global.drop_highlight"],
        "{:?}",
        report.synthetic
    );
    assert_eq!(
        report.listed.keys().collect::<Vec<_>>(),
        ["probe input.accent dark", "probe input.accent light"]
    );
    assert!(
        report.listed.values().all(|(ids, _)| ids_of(ids) == ["U9"]),
        "{:?}",
        report.listed
    );
    let stated = fixture_report_with("pending_input", "stated.toml");
    assert_eq!(
        failing(&stated, "[pending]"),
        [
            &"probe: `inputs.accent.light` (U9) is in its [pending] table, and the preset states \
              it"
            .to_owned(),
            &"probe: `inputs.accent.dark` (U9) is in its [pending] table, and the preset states it"
                .to_owned()
        ],
        "{:?}",
        stated.failures
    );
}

/// A pending presence with no presence beneath is listed by its part's
/// path (S17, ADD-A131: `"widgets.button.label" = "U122"`): found with its
/// id, the part drawn on the synthetic route, its fields compared there
/// (a subtitle cased against its fact fails, the review's M5); listed
/// while the preset states the presence, it fails.
#[test]
fn gate_fixture_pending_presence() {
    let report = fixture_report("pending_presence");
    assert_eq!(report.failures, Vec::<String>::new());
    assert_eq!(
        report.listed.keys().collect::<Vec<_>>(),
        ["probe button.subtitle"]
    );
    assert_eq!(
        report.listed.values().next().map(|(ids, _)| ids_of(ids)),
        Some(vec!["U7"])
    );
    assert!(report.compared > 0, "{report:?}");
    let upper = fixture_report_with("pending_presence", "upper.toml");
    let compared = failing(&upper, "button.subtitle.text.case");
    assert!(!compared.is_empty(), "{:?}", upper.failures);
    assert_eq!(compared.len(), upper.failures.len(), "{:?}", upper.failures);
    let stated = fixture_report_with("pending_presence", "stated.toml");
    assert_eq!(
        failing(&stated, "[pending]"),
        [
            &"probe: `widgets.button.subtitle` (U7) is in its [pending] table, and the preset \
              states it"
                .to_owned()
        ],
        "{:?}",
        stated.failures
    );
}

/// A value the specification defines is compared with its definition
/// (S17, ADD-A125, ADD-A131): the base's `font_dpi` of 96 fails on an
/// Apple preset, which defines 72, and its empty `animated_visuals` agrees
/// on every preset but `windows-11`, which states its own.
#[test]
fn gate_definitions_compared() {
    let (_, presets) = fixture("passes", "preset.toml");
    let (_, preset) = &presets[0];
    let conditions = native_theme::gate::Conditions::new("light").unwrap();
    let resolved = preset
        .resolve(&conditions, &native_theme::Layer::default())
        .unwrap();
    let root = run::root_inputs();
    let failures = |id: &str| {
        let mut report = run::Report::default();
        run::definitions(id, &resolved, &root, &mut report, &|| "[]".to_owned());
        (report.failures, report.compared)
    };
    assert_eq!(failures("probe"), (Vec::new(), 1));
    assert_eq!(failures("windows-11"), (Vec::new(), 0));
    let (apple, compared) = failures("ios-26");
    assert_eq!(compared, 2);
    assert_eq!(apple.len(), 1, "{apple:?}");
    assert!(
        apple[0].contains("token.text.font_dpi") && apple[0].contains("the definition of S9.4"),
        "{apple:?}"
    );
}

/// An Apple preset's `font_dpi` is compared with S9.4's definition, 72,
/// which no fact states, in the gate's run over a preset (S17, ADD-A125):
/// the preset stating 72 passes, its twin stating 96 fails naming the key
/// and the definition.
#[test]
fn gate_fixture_apple_font_dpi() {
    let report = fixture_report("apple_font_dpi");
    assert_eq!(report.failures, Vec::<String>::new());
    let other = fixture_report_with("apple_font_dpi", "dpi_96.toml");
    assert!(!other.failures.is_empty(), "{other:?}");
    assert!(
        other
            .failures
            .iter()
            .all(|failure| failure.contains("token.text.font_dpi")
                && failure.contains("the definition of S9.4")),
        "{:?}",
        other.failures
    );
}

/// A colour measured from drawn pixels is compared with the resolved colour
/// as the measured platform's renderer draws it (S17, S2.1's hand-over;
/// ADD-A131 Q-B4): GTK's and QPainter's premultiply-then-clamp, Compose's
/// straight clamp; out of the gamut where no source states the drawing
/// (Apple's, WinUI's, Qt Quick's) the comparison fails, naming the key.
#[test]
fn gate_measured_colour_drawn_per_renderer() {
    use native_theme::gate::Drawing;
    for (platform, flavor, drawing) in [
        ("gnome", None, Drawing::Premultiplied),
        ("kde", None, Drawing::Premultiplied),
        ("kde", Some("widgets"), Drawing::Premultiplied),
        ("kde", Some("kirigami"), Drawing::Unread),
        ("material", None, Drawing::Clamped),
        ("macos", None, Drawing::Unread),
        ("ios", None, Drawing::Unread),
        ("windows", None, Drawing::Unread),
    ] {
        assert_eq!(
            run::drawing(platform, flavor),
            drawing,
            "{platform} {flavor:?}"
        );
    }
    let report = |platform: &str| {
        let (root, presets) = fixture("measured_drawn", "preset.toml");
        let presets: Vec<(String, Preset)> = presets
            .into_iter()
            .map(|(_, preset)| (platform.to_owned(), preset))
            .collect();
        run::run(&root, fixture_evidence(&root), &presets)
    };
    for platform in ["gnome", "kde", "material"] {
        let drawn = report(platform);
        assert_eq!(drawn.failures, Vec::<String>::new(), "{platform}");
        assert!(drawn.compared > 0, "{platform}: {drawn:?}");
    }
    let apple = report("macos");
    assert!(!apple.failures.is_empty(), "{apple:?}");
    assert!(
        apple
            .failures
            .iter()
            .all(|failure| failure.contains("`button.body.fill`")
                && failure.contains("out of the gamut")),
        "{:?}",
        apple.failures
    );
}

/// Only a colour measured from drawn pixels is drawn before it is compared
/// (S17: "a colour measured from drawn pixels is compared with the resolved
/// colour as the platform draws it"): `measured_drawn`'s evidence is an
/// image's analysis (its `input_image` a PNG), `measured_read`'s a value an
/// API read, `measured_node`'s a render-node file and
/// `measured_node_analysis`'s an analysis of one (S17: a `.node` file holds
/// the colour's OKLab coordinates unrounded, which an analysis converts as
/// the function does), each of the three compared unclamped on every
/// platform, so its out-of-gamut colour passes where no source states the
/// renderer's drawing too.
#[test]
fn gate_measured_colour_drawn_only_from_pixels() {
    let report = |name: &str, platform: &str| {
        let (root, presets) = fixture(name, "preset.toml");
        let presets: Vec<(String, Preset)> = presets
            .into_iter()
            .map(|(_, preset)| (platform.to_owned(), preset))
            .collect();
        run::run(&root, fixture_evidence(&root), &presets)
    };
    for name in ["measured_read", "measured_node", "measured_node_analysis"] {
        for platform in ["gnome", "kde", "material", "macos"] {
            let read = report(name, platform);
            assert_eq!(read.failures, Vec::<String>::new(), "{name} {platform}");
            assert!(read.compared > 0, "{name} {platform}: {read:?}");
        }
    }
    let drawn = report("measured_drawn", "macos");
    assert!(
        drawn
            .failures
            .iter()
            .any(|failure| failure.contains("out of the gamut")),
        "{:?}",
        drawn.failures
    );
}

/// A whole map listed as pending is found (S17: "U150's
/// `token.text.fallbacks.per_language` … on `adwaita`"): the map's
/// synthetic value is empty, so reading it reads the synthetic layer's step
/// though it states no entry.
#[test]
fn gate_fixture_pending_map() {
    let report = fixture_report("pending_map");
    assert_eq!(report.failures, Vec::<String>::new());
    assert_eq!(
        report.listed.keys().collect::<Vec<_>>(),
        ["probe token.text.fallbacks.per_language"]
    );
    assert_eq!(
        report.listed.values().next().map(|(ids, _)| ids_of(ids)),
        Some(vec!["U150"])
    );
}

/// A listed field of a part skips no fact of a sibling field (S17: only
/// "every fact whose evaluation reaches a synthetic value"): where no
/// preset rule on the part applies, the synthetic layer's step is the last
/// kept one, yet it changes nothing of whether the part is drawn, so the
/// opacity, other than its fact, fails there too.
#[test]
fn gate_fixture_pending_sibling_field() {
    let report = fixture_report("pending_sibling");
    let failures = failing(&report, "button.body.opacity");
    assert!(
        failures
            .iter()
            .any(|failure| failure.contains("states: []") && failure.contains("variants: []")),
        "{:?}",
        report.failures
    );
    assert!(report.synthetic.is_empty(), "{:?}", report.synthetic);
    assert_eq!(
        report.listed.keys().collect::<Vec<_>>(),
        ["probe button.body.min_width"]
    );
}

/// A fact reading a listed path through a function that maps many values
/// to one (`max`, `min`, `floor`, `round`, a `mix` all the way to its
/// second colour, a `pick`) is skipped and reported (S17, ADD-A128): its
/// evaluation reaches the synthetic value, whatever value it gives; so is
/// an input whose value reads a listed token; each listed path, a colour
/// role's per scheme among them, is found.
#[test]
fn gate_fixture_pending_reached_through_functions() {
    let report = fixture_report("pending_reached");
    assert_eq!(report.failures, Vec::<String>::new());
    assert_eq!(
        report.listed.keys().collect::<Vec<_>>(),
        [
            "probe role.global.selection_text_background",
            "probe token.caret.blink_on",
            "probe token.caret.color",
            "probe token.motion.speed_factor",
        ]
    );
    let mut skipped: Vec<&String> = report.synthetic.values().map(|(key, _)| key).collect();
    skipped.sort();
    assert_eq!(
        skipped,
        [
            "input.accent",
            "role.global.drop_highlight",
            "role.global.scrim",
            "token.behavior.hover_time",
            "token.behavior.long_press",
            "token.text.rendering.contrast",
            "token.text.rendering.gamma",
        ],
        "{:?}",
        report.synthetic
    );
    assert!(report.compared > 0, "{report:?}");
}

/// A part's field and a widget value listed (S17, ADD-A128): each found
/// where its pending fact applies with nothing beneath; a fact reading one
/// through `max` or `min` skipped where it reaches the synthetic value, and
/// compared where the preset's rule states the field (pressed).
#[test]
fn gate_fixture_pending_widget_paths() {
    let report = fixture_report("pending_widget");
    assert_eq!(report.failures, Vec::<String>::new());
    assert_eq!(
        report.listed.keys().collect::<Vec<_>>(),
        [
            "probe button.body.min_width",
            "probe button.values.menu_indicator_gap",
        ]
    );
    let mut skipped: Vec<&String> = report.synthetic.values().map(|(key, _)| key).collect();
    skipped.sort();
    assert_eq!(
        skipped,
        ["button.body.max_width", "button.values.subtitle_gap"],
        "{:?}",
        report.synthetic
    );
    let pressed = fixture_report_with("pending_widget", "unpressed.toml");
    assert!(
        pressed
            .failures
            .iter()
            .any(|failure| failure.contains("button.body.min_width")),
        "the rule's value is compared where it applies: {:?}",
        pressed.failures
    );
}

/// The table holds exactly the listed paths (S17, ADD-A128): a listed path
/// the table lacks is S12's missing field, which loading the preset
/// refuses; an entry no combination leaves to a pending fact fails, and so
/// do one the preset states, which the reader reads, and one under another
/// register id than its pending fact's.
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
    let stated = fixture_report_with("pending_listed", "stated.toml");
    assert_eq!(
        failing(&stated, "[pending]"),
        [&"probe: `tokens.behavior.drag_time_ms` (U6) is in its [pending] table, and the preset \
           states it"
            .to_owned()],
        "{:?}",
        stated.failures
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

/// The derivations run refuses a pass it cannot evaluate where its fact
/// says (S17 *A measured derivation*, task 15): a fact whose `when` names
/// no combination the state model draws, and one whose combination the
/// state model's `not_drawn` names.
#[test]
fn gate_fixture_measured_derivation_refused() {
    let report = fixture_report("derivation_refused");
    assert!(
        report.failures.iter().any(
            |failure| failure.contains("its `when` names no combination the state model draws")
        ),
        "{:?}",
        report.failures
    );
    assert!(
        report
            .failures
            .iter()
            .any(|failure| failure.contains("is named by `not_drawn`")),
        "{:?}",
        report.failures
    );
}

/// A fact's own combination completed to one the state model draws (S17):
/// the first required variant its `when` does not exclude by
/// `not_variants`, and none sharing an `exclusive_variants` group with a
/// variant it holds; `None` where none is left.
#[test]
fn own_combination_completed_by_the_filters() {
    let models = model::Model::read(
        "probe",
        "[[model]]\nwidget = \"button\"\navailable = true\n\
         variants = [\"flat\", \"primary\", \"@x\"]\nsizes = []\ndensities = []\nexclusive = []\n\
         exclusive_variants = [[\"flat\", \"@x\"]]\nrequires_variants = [\"flat\", \"primary\"]\n\
         rows = []\n",
    )
    .unwrap();
    let entry = &models[0];
    let when = |text: &str| -> toml::Value { toml::from_str(text).unwrap() };
    let completed = |variants: &[&str], when: &toml::Value| {
        let mut combination = Combination::new("button");
        combination.variants = variants.iter().map(|v| (*v).to_owned()).collect();
        run::complete(combination, when, entry).map(|c| c.variants)
    };
    let plain = when("");
    assert_eq!(completed(&[], &plain), Some(vec!["flat".to_owned()]));
    let not_flat = when("not_variants = [\"flat\"]");
    assert_eq!(completed(&[], &not_flat), Some(vec!["primary".to_owned()]));
    assert_eq!(
        completed(&["@x"], &plain),
        Some(vec!["@x".to_owned(), "primary".to_owned()])
    );
    let neither = when("not_variants = [\"primary\"]");
    assert_eq!(completed(&["@x"], &neither), None);
}

/// The perturbed run's colours are one-to-one over many operands (S17): no
/// two of the first thousand share a colour, beyond the eight an earlier
/// map cycled through.
#[test]
fn perturbed_colours_are_distinct() {
    let colours: std::collections::BTreeSet<String> = (0..1000).map(run::perturbed).collect();
    assert_eq!(colours.len(), 1000);
}

/// `facts_gate`: the gate over `docs/facts` and every bundled preset, each
/// against its platform's directory: every fact file and state model
/// read, each entry's combinations counted, and each preset's normal,
/// perturbed and declarations runs.
/// [`PRESETS_VARIABLE`] unset: `facts_gate` runs every bundled preset.
#[test]
fn facts_gate_presets_unset_runs_all() {
    assert_eq!(selected_presets(None), Ok(PresetId::all().to_vec()));
}

/// [`PRESETS_VARIABLE`] set: `facts_gate` runs the presets it names alone,
/// in [`PresetId::all`]'s order, spaces around a name ignored.
#[test]
fn facts_gate_presets_filter() {
    assert_eq!(
        selected_presets(Some("macos-tahoe, adwaita")),
        Ok(vec![PresetId::Adwaita, PresetId::MacosTahoe])
    );
    assert_eq!(selected_presets(Some("ios-13")), Ok(vec![PresetId::Ios13]));
}

/// [`PRESETS_VARIABLE`] naming an unknown id, nothing or an id twice fails,
/// never runs an empty or silently narrowed gate.
#[test]
fn facts_gate_presets_unknown_id_fails() {
    let unknown = selected_presets(Some("adwaita,no-such-preset")).unwrap_err();
    assert!(
        unknown.contains("`no-such-preset` is no bundled preset's id"),
        "{unknown}"
    );
    assert!(selected_presets(Some("")).is_err());
    assert!(selected_presets(Some("adwaita,")).is_err());
    let twice = selected_presets(Some("adwaita,adwaita")).unwrap_err();
    assert!(twice.contains("named twice"), "{twice}");
}

/// [`PRESETS_VARIABLE`] `none`: `facts_gate` runs no preset, the facts' own
/// checks alone; `none` beside an id is no bundled preset's id.
#[test]
fn facts_gate_presets_none() {
    assert_eq!(selected_presets(Some("none")), Ok(Vec::new()));
    assert_eq!(selected_presets(Some(" none ")), Ok(Vec::new()));
    let beside = selected_presets(Some("none,adwaita")).unwrap_err();
    assert!(
        beside.contains("`none` is no bundled preset's id"),
        "{beside}"
    );
}

/// [`run::SHARD_VARIABLE`] names one shard of the gate's work, `k/N`;
/// unset, the whole run; any other value fails.
#[test]
fn facts_gate_shard_variable() {
    assert_eq!(run::shard_of(None), Ok(run::Shard::WHOLE));
    assert_eq!(
        run::shard_of(Some("1/1")),
        Ok(run::Shard { index: 0, count: 1 })
    );
    assert_eq!(
        run::shard_of(Some(" 3 / 8 ")),
        Ok(run::Shard { index: 2, count: 8 })
    );
    for refused in [
        "", "0/4", "5/4", "1/0", "2", "1/2/3", "a/4", "-1/4", "1.5/4",
    ] {
        let error = run::shard_of(Some(refused)).unwrap_err();
        assert!(error.contains("is no shard"), "{refused}: {error}");
    }
}

/// The shards add up to the whole run ([`run::Shard`]): over every fixture
/// case whose preset reads, each split into two, three and five shards,
/// the shards' reports merged hold exactly the whole run's failures, groups,
/// pending, listed and skipped facts and counts; and some shard but the
/// first checks a condition vector, so the split is no first-shard run.
#[test]
fn gate_shards_add_up() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/facts_gate");
    let base = std::fs::read_to_string(fixtures.join("base.toml")).unwrap();
    let mut cases: Vec<PathBuf> = std::fs::read_dir(&fixtures)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|case| case.join("preset.toml").is_file())
        .collect();
    cases.sort();
    let listed =
        std::env::temp_dir().join(format!("native-theme-shards-listed-{}", std::process::id()));
    let mut split = 0;
    let mut gated = 0;
    for case in &cases {
        let Ok(preset) = Preset::from_toml(&fixture_text(case, &base, "preset.toml")) else {
            continue;
        };
        gated += 1;
        let root = case.join("docs/facts");
        let presets = vec![("gnome".to_owned(), preset)];
        let whole = run::run(&root, fixture_evidence(&root), &presets);
        for count in [2, 3, 5] {
            let _ = std::fs::remove_dir_all(&listed);
            let mut merged = run::Report::default();
            for index in 0..count {
                let shard = run::Shard { index, count };
                let part = run::run_shard(&root, fixture_evidence(&root), &presets, shard);
                if index > 0 && part.compared > 0 {
                    split += 1;
                }
                write_listed(&listed, shard, &presets, &part);
                merged.merge(part);
            }
            assert_eq!(read_listed(&listed, &presets), Ok(merged.listed.clone()));
            check_listed(&presets, &mut merged);
            assert_eq!(
                merged.findings(),
                whole.findings(),
                "{}: {count} shards",
                case.display()
            );
        }
    }
    let _ = std::fs::remove_dir_all(&listed);
    assert!(gated > 10, "{gated} fixture cases gated");
    assert!(split > 0, "no shard but the first checked a vector");
}

/// The shards' listed paths are refused unless every shard of one count
/// wrote them over the same presets ([`read_listed`]).
#[test]
fn facts_gate_listed_refused() {
    let base = std::env::temp_dir().join(format!(
        "native-theme-listed-refused-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&base);
    let (_, presets) = fixture("passes", "preset.toml");
    let report = run::Report::default();
    let write = |k: usize, n: usize| {
        write_listed(
            &base,
            run::Shard {
                index: k - 1,
                count: n,
            },
            &presets,
            &report,
        );
    };
    let refused = |want: &str| {
        let error = read_listed(&base, &presets).unwrap_err();
        assert!(error.contains(want), "{error}");
    };
    std::fs::create_dir_all(&base).unwrap();
    refused("no shard's listed paths");
    write(1, 3);
    write(3, 3);
    refused("the shards 2 of 3 are missing");
    write(2, 3);
    assert_eq!(read_listed(&base, &presets), Ok(Default::default()));
    write(1, 2);
    refused("another count of shards");
    std::fs::remove_file(base.join("listed-1-of-2.tsv")).unwrap();
    std::fs::write(base.join("listed-2-of-3.tsv"), "presets\tother\n").unwrap();
    refused("not a run over");
    let header = format!("presets\t{}", presets.first().unwrap().1.id());
    std::fs::write(
        base.join("listed-2-of-3.tsv"),
        format!("{header}\nkey\tU1\n"),
    )
    .unwrap();
    refused("does not read");
    std::fs::write(base.join("notes.txt"), "").unwrap();
    refused("notes.txt: no shard's listed paths");
    std::fs::remove_dir_all(&base).unwrap();
}

/// [`run::THREADS_VARIABLE`] sets the gate's threads; unset, the machine's
/// parallelism; a value that is no number above zero fails.
#[test]
fn facts_gate_threads_variable() {
    assert_eq!(run::thread_count(Some("4")), Ok(4));
    assert_eq!(run::thread_count(Some(" 12 ")), Ok(12));
    assert_eq!(
        run::thread_count(None),
        Ok(std::thread::available_parallelism().map_or(1, std::num::NonZero::get))
    );
    for refused in ["0", "", "four", "-2", "1.5"] {
        let error = run::thread_count(Some(refused)).unwrap_err();
        assert!(error.contains("no number above zero"), "{error}");
    }
}

/// The evidence root outside a fixture (S17 *The evidence root*,
/// [`facts::evidence_root_at`]): a clean checkout at the pin, the variable's
/// directory before the source cache's; refused at another commit, with a
/// changed, an untracked or an ignored file, as a subfolder, without a
/// folder, and where `sources.toml` has no evidence entry or two.
#[test]
fn facts_gate_evidence_root() {
    let base =
        std::env::temp_dir().join(format!("native-theme-evidence-root-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let cache = base.join("cache");
    let repo = cache.join("native-theme-evidence");
    std::fs::create_dir_all(repo.join("docs/evidence")).unwrap();
    std::fs::write(repo.join("docs/evidence/a.toml"), "a = 1\n").unwrap();
    let git = |args: &[&str]| {
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(&repo)
            .args([
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .output()
            .unwrap();
        assert!(output.status.success(), "git {args:?}: {output:?}");
        String::from_utf8(output.stdout).unwrap()
    };
    git(&["init", "-q"]);
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "c"]);
    let head = git(&["rev-parse", "HEAD"]).trim().to_owned();
    let facts = base.join("facts");
    std::fs::create_dir_all(&facts).unwrap();
    let pins = |commits: &[&str]| {
        let entries: String = commits
            .iter()
            .map(|commit| {
                format!("[[source]]\nid = \"native-theme-evidence\"\nkind = \"git\"\nevidence = true\ncommit = \"{commit}\"\n\n")
            })
            .collect();
        std::fs::write(
            facts.join("sources.toml"),
            format!(
                "{entries}[[source]]\nid = \"x@1\"\nkind = \"git\"\ncommit = \"{}\"\n",
                "1".repeat(40)
            ),
        )
        .unwrap();
    };
    let refused = |named: Option<PathBuf>, cache: &Path, want: &str| {
        let error = facts::evidence_root_at(&facts, named, cache).unwrap_err();
        assert!(error.contains(want), "{error}");
    };
    pins(&[&head]);
    assert_eq!(
        facts::evidence_root_at(&facts, None, &cache),
        Ok(repo.clone())
    );
    let elsewhere = base.join("elsewhere");
    assert_eq!(
        facts::evidence_root_at(&facts, Some(repo.clone()), &elsewhere),
        Ok(repo.clone())
    );
    refused(None, &elsewhere, "is no directory");
    refused(
        Some(repo.join("docs")),
        &cache,
        "is not the top of a git checkout",
    );
    for (path, ignored) in [
        ("docs/evidence/b.toml", false),
        ("docs/evidence/c.toml", true),
    ] {
        std::fs::write(repo.join(path), "b = 1\n").unwrap();
        if ignored {
            std::fs::write(repo.join(".git/info/exclude"), "c.toml\n").unwrap();
        }
        refused(
            None,
            &cache,
            "is not clean: 1 paths changed, untracked or ignored",
        );
        std::fs::remove_file(repo.join(path)).unwrap();
    }
    std::fs::write(repo.join("docs/evidence/a.toml"), "a = 2\n").unwrap();
    refused(None, &cache, "is not clean");
    std::fs::write(repo.join("docs/evidence/a.toml"), "a = 1\n").unwrap();
    let other = "0".repeat(40);
    pins(&[&other]);
    refused(
        None,
        &cache,
        &format!("is at {head}, not at the pin {other}"),
    );
    pins(&[]);
    refused(None, &cache, "0 evidence entries");
    pins(&[&head, &head]);
    refused(None, &cache, "2 evidence entries");
    std::fs::remove_dir_all(&base).unwrap();
}

#[test]
fn facts_gate() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/facts");
    let variable = presets_variable();
    let ids = selected_presets(variable.as_deref())
        .unwrap_or_else(|error| panic!("{PRESETS_VARIABLE}: {error}"));
    let subset = ids.len() < PresetId::all().len();
    assert!(
        !(subset && std::env::var_os("FACTS_GATE_RELEASE").is_some()),
        "the release's gate runs every bundled preset: unset {PRESETS_VARIABLE}"
    );
    let shard = run::shard();
    assert!(
        !(shard != run::Shard::WHOLE && std::env::var_os("FACTS_GATE_RELEASE").is_some()),
        "the release's gate runs every shard at once: unset {}",
        run::SHARD_VARIABLE
    );
    let presets: Vec<(String, Preset)> = ids
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
    if let Some(directory) = directory_variable(MERGE_VARIABLE) {
        assert!(
            shard == run::Shard::WHOLE && std::env::var_os("FACTS_GATE_RELEASE").is_none(),
            "{MERGE_VARIABLE} checks the shards' listed paths: unset {} and FACTS_GATE_RELEASE",
            run::SHARD_VARIABLE
        );
        let mut report = run::Report::default();
        report.listed = read_listed(&directory, &presets)
            .unwrap_or_else(|error| panic!("{MERGE_VARIABLE}: {error}"));
        check_listed(&presets, &mut report);
        eprintln!(
            "facts gate: the listed paths of {} presets over the shards' findings in {}, {} \
             found; {} failures:",
            presets.len(),
            directory.display(),
            report.listed.len(),
            report.failures.len()
        );
        for failure in &report.failures {
            eprintln!("    {failure}");
        }
        assert!(
            report.failures.is_empty(),
            "{} failures of the listed paths (listed above)",
            report.failures.len()
        );
        return;
    }
    let evidence = facts::evidence_root(&root).unwrap_or_else(|error| panic!("{error}"));
    let report = run::run_shard(&root, &evidence, &presets, shard);
    if shard != run::Shard::WHOLE
        && let Some(directory) = directory_variable(LISTED_VARIABLE)
    {
        write_listed(&directory, shard, &presets, &report);
    }
    let mut scope = if ids.is_empty() {
        format!(
            "no preset, the facts' own checks alone, as {PRESETS_VARIABLE} names (unset it to \
             gate every preset)"
        )
    } else if subset {
        format!(
            "a subset, {} of {} bundled presets ({}) as {PRESETS_VARIABLE} names (unset it to \
             gate every preset)",
            ids.len(),
            PresetId::all().len(),
            ids.iter().map(|id| id.key()).collect::<Vec<_>>().join(", "),
        )
    } else {
        format!("{} presets", presets.len())
    };
    if shard != run::Shard::WHOLE {
        scope.push_str(&format!(
            ", shard {} of {} as {} names (unset it to gate every shard), its listed paths \
             checked over every shard's ({LISTED_VARIABLE}, then {MERGE_VARIABLE})",
            shard.index + 1,
            shard.count,
            run::SHARD_VARIABLE
        ));
    }
    eprintln!(
        "facts gate: {scope}, {} keys compared over {} distinct combinations in {:?}; \
         {} pending facts; skipped: {} facts of {} keys the model does not hold, {} facts of a unit \
         not enabled, {} whens, \
         {} accent-run facts, {} facts naming a preset not bundled, {} combinations, \
         {} state-model variants only skipped facts name",
        report.compared,
        report.queried,
        report.elapsed,
        report.pending.len(),
        report.keys_not_held.values().sum::<usize>(),
        report.keys_not_held.len(),
        report.units_not_enabled.values().sum::<usize>(),
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
    for (key, count) in &report.units_not_enabled {
        eprintln!("  unit not enabled: {count}: {key}");
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
    for (key, (ids, count)) in &report.listed {
        eprintln!(
            "  listed: {key}: {} on {count} combinations",
            ids_of(ids).join(", ")
        );
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
    eprintln!(
        "  {} failures in {} groups by run, key and fact{}:",
        report.failures.len(),
        report.groups.len(),
        if report.groups.len() > 200 {
            ", the first 200"
        } else {
            ""
        }
    );
    for group in report.groups.iter().take(200) {
        eprintln!(
            "    {} × {} `{}` {}: first: {}",
            group.count, group.run, group.key, group.fact, group.first
        );
    }
    assert!(
        report.failures.is_empty(),
        "{} failures in {} groups (listed above); gated: {scope}",
        report.failures.len(),
        report.groups.len()
    );
    // The release's gate (S17, R21): 0.8.0 is released with nothing skipped,
    // no pending fact and every `[pending]` table empty, which the plan's
    // final gate runs this test with `FACTS_GATE_RELEASE=1` to require.
    if std::env::var_os("FACTS_GATE_RELEASE").is_some() {
        assert_eq!(
            (
                report.pending.len(),
                report.keys_not_held.len(),
                report.units_not_enabled.len(),
                report.whens_not_held.len(),
                report.other_presets,
                report.combinations_skipped,
                report.variants_not_held.len(),
                report.unread.len(),
                report.listed.len() + report.synthetic.len(),
            ),
            (0, 0, 0, 0, 0, 0, 0, 0, 0),
            "the release's gate: no pending fact, nothing skipped, no value refused"
        );
    }
}
