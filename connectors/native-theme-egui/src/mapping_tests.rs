//! §13 T3 (converse coverage), §5.7's totals and the typed reader of `mapping.toml`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use native_theme::theme::{ColorMode, Theme};

use crate::install_tests::pass;
use crate::style_diff::{Change, Location, all_frames, all_styles, atlas_diff, self_equal};
use crate::{LayoutTheme, Note, ThemeAtlas};
use crate::{ResolvedTheme, Role, RoleVariant, Surface};

/// The crate's one `resolved`, re-exported so a module reading the manifest takes
/// both from here.
pub(crate) use crate::install_tests::resolved;

/// The manifest, parsed. `include_str!` binds the test to the committed file.
pub(crate) fn manifest() -> toml::Table {
    toml::from_str(include_str!("../mapping.toml")).expect("mapping.toml parses")
}

/// A row's verdict (§2, §13.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Verdict {
    Direct,
    Scoped,
    Derived,
    Unmappable,
}

/// One sink of a row (§13.1): `{ path, scope?, variant?, surface?, when? }`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Sink {
    pub(crate) path: String,
    pub(crate) scope: Option<String>,
    pub(crate) variant: Option<String>,
    pub(crate) surface: Option<String>,
    pub(crate) when: Option<String>,
}

/// One row of the manifest, keyed by its leaf path in [`Manifest::rows`].
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Row {
    pub(crate) verdict: Verdict,
    pub(crate) sinks: Vec<Sink>,
    pub(crate) tested_by: Option<String>,
    pub(crate) probe: Option<toml::Value>,
    pub(crate) sub_tag: Option<String>,
    pub(crate) upstream: Option<String>,
    pub(crate) exceptions: Vec<(String, String)>,
}

/// `mapping.toml`, typed: the rows by leaf path and the `[unwritten]` table (§13.1).
pub(crate) struct Manifest {
    pub(crate) rows: BTreeMap<String, Row>,
    pub(crate) unwritten: BTreeMap<String, String>,
}

impl Manifest {
    /// The committed manifest, typed. A row it cannot type fails the calling test; T3 reports
    /// every such row with its reason.
    pub(crate) fn load() -> Manifest {
        fn text(table: &toml::Table, key: &str) -> Option<String> {
            table
                .get(key)
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
        }
        let mut rows = BTreeMap::new();
        let mut unwritten = BTreeMap::new();
        for (key, value) in &manifest() {
            let Some(row) = value.as_table() else {
                panic!("{key}: a row is a table")
            };
            if key == "unwritten" {
                for (path, reason) in row {
                    let reason = reason.as_str().expect("an [unwritten] reason is a string");
                    unwritten.insert(path.clone(), reason.to_owned());
                }
                continue;
            }
            let verdict = match row.get("verdict").and_then(toml::Value::as_str) {
                Some("direct") => Verdict::Direct,
                Some("scoped") => Verdict::Scoped,
                Some("derived") => Verdict::Derived,
                Some("unmappable") => Verdict::Unmappable,
                other => panic!("{key}: verdict {other:?}"),
            };
            let sinks = row
                .get("sinks")
                .and_then(toml::Value::as_array)
                .map_or_else(Vec::new, |sinks| {
                    sinks
                        .iter()
                        .map(|sink| {
                            let sink = sink.as_table().expect("a sink is an inline table");
                            Sink {
                                path: text(sink, "path").expect("a sink names its path"),
                                scope: text(sink, "scope"),
                                variant: text(sink, "variant"),
                                surface: text(sink, "surface"),
                                when: text(sink, "when"),
                            }
                        })
                        .collect()
                });
            let exceptions = row
                .get("exceptions")
                .and_then(toml::Value::as_array)
                .map_or_else(Vec::new, |pairs| {
                    pairs
                        .iter()
                        .map(|pair| match pair.as_array().map(Vec::as_slice) {
                            Some([preset, reason]) => (
                                preset
                                    .as_str()
                                    .expect("an exception's preset is a string")
                                    .to_owned(),
                                reason
                                    .as_str()
                                    .expect("an exception's reason is a string")
                                    .to_owned(),
                            ),
                            _ => panic!("{key}: an exception is [preset, reason]"),
                        })
                        .collect()
                });
            rows.insert(
                key.clone(),
                Row {
                    verdict,
                    sinks,
                    tested_by: text(row, "tested_by"),
                    probe: row.get("probe").cloned(),
                    sub_tag: text(row, "sub_tag"),
                    upstream: text(row, "upstream"),
                    exceptions,
                },
            );
        }
        Manifest { rows, unwritten }
    }
}

/// Every leaf path of `serde_json::to_value(resolved)` plus the four `layout.` leaves
/// (§13.1): the walk descends into objects, stops at a `defined_size` — an externally tagged
/// `FontSize`, `{"Pt": ..}` or `{"Px": ..}` (`native-theme/src/model/font.rs:78-89`) — and
/// counts a `null` as a leaf (an unstated padding side, a `None` soft option).
pub(crate) fn leaf_paths(resolved: &ResolvedTheme) -> BTreeSet<String> {
    fn walk(prefix: &str, value: &serde_json::Value, out: &mut BTreeSet<String>) {
        match value {
            serde_json::Value::Object(map) if !prefix.ends_with(".defined_size") => {
                for (key, child) in map {
                    let path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    };
                    walk(&path, child, out);
                }
            }
            _ => {
                out.insert(prefix.to_owned());
            }
        }
    }
    let value = serde_json::to_value(resolved).expect("ResolvedTheme serialises");
    let mut out = BTreeSet::new();
    walk("", &value, &mut out);
    for leaf in [
        "widget_gap",
        "container_margin",
        "window_margin",
        "section_gap",
    ] {
        out.insert(format!("layout.{leaf}"));
    }
    out
}

/// `tested_by` names a clause §13's table defines: a test `T1` to `T18`, T1b, T10's T10b and
/// T10c, or one of the lettered clauses a test states — T4 (a)–(c), T6 (a)–(e), T8 (a)–(c),
/// T11 (a)–(d), T13 (a)–(d), T14 (a)–(d), T15 (a)–(c) and T18 (a)–(h).
fn is_clause(s: &str) -> bool {
    const LETTERED: [(&str, &str); 8] = [
        ("T4", "abc"),
        ("T6", "abcde"),
        ("T8", "abc"),
        ("T11", "abcd"),
        ("T13", "abcd"),
        ("T14", "abcd"),
        ("T15", "abc"),
        ("T18", "abcdefgh"),
    ];
    let mut clauses: BTreeSet<String> = (1..=18).map(|n| format!("T{n}")).collect();
    clauses.extend(["T1b", "T10b", "T10c"].map(str::to_owned));
    for (test, letters) in LETTERED {
        clauses.extend(letters.chars().map(|c| format!("{test}({c})")));
    }
    clauses.contains(s)
}

#[test]
fn is_clause_accepts_only_the_clauses_section_13_defines() {
    for s in [
        "T1", "T1b", "T10b", "T10c", "T14(b)", "T6(d)", "T18(h)", "T18",
    ] {
        assert!(is_clause(s), "{s}");
    }
    for s in [
        "T3(z)", "T13(q)", "T2(a)", "T19", "T0", "T5b", "T18(i)", "t6(d)", "T6(d) ",
    ] {
        assert!(!is_clause(s), "{s}");
    }
}

fn rows(manifest: &toml::Table) -> impl Iterator<Item = (&String, &toml::Table)> {
    manifest
        .iter()
        .filter(|(key, _)| key.as_str() != "unwritten")
        .filter_map(|(key, row)| row.as_table().map(|row| (key, row)))
}

/// §13 T3: every leaf has exactly one row, every row is a leaf, and each row has the
/// schema's shape (§13.1).
#[test]
fn every_leaf_has_exactly_one_row_and_every_row_is_well_formed() {
    let manifest = manifest();
    let leaves = leaf_paths(&resolved("adwaita", ColorMode::Light));
    assert_eq!(
        leaves.len(),
        482,
        "478 ResolvedTheme leaves + 4 layout leaves (§5.7)"
    );

    let mut problems: Vec<String> = Vec::new();
    // A TOML table holds a key once, so presence is "exactly once".
    let keys: BTreeSet<&str> = manifest
        .keys()
        .map(String::as_str)
        .filter(|k| *k != "unwritten")
        .collect();
    for leaf in &leaves {
        if !keys.contains(leaf.as_str()) {
            problems.push(format!("leaf with no row: {leaf}"));
        }
    }
    for key in &keys {
        if !leaves.contains(*key) {
            problems.push(format!("row that is no leaf: {key}"));
        }
        if manifest.get(*key).and_then(toml::Value::as_table).is_none() {
            problems.push(format!("{key}: a row is a table"));
        }
    }

    let role_keys: BTreeSet<&str> = Role::all().iter().map(|r| r.key()).collect();
    let surface_keys: BTreeSet<&str> = Surface::all().iter().map(|s| s.key()).collect();
    let variant_keys: BTreeSet<&str> = [RoleVariant::Selected, RoleVariant::Disabled]
        .iter()
        .map(|v| v.key())
        .collect();
    let presets: BTreeSet<&str> = Theme::list_presets().iter().map(|p| p.key).collect();
    let sub_tags = [
        "egui-limited",
        "widgets-crate",
        "source-void",
        "source-side gap",
    ];
    let verdicts = ["direct", "scoped", "derived", "unmappable"];
    let sink_keys = ["path", "scope", "variant", "surface", "when"];
    let row_keys = [
        "verdict",
        "sinks",
        "tested_by",
        "probe",
        "exceptions",
        "sub_tag",
        "upstream",
    ];

    let mut declared_paths: BTreeSet<String> = BTreeSet::new();
    for (leaf, row) in rows(&manifest) {
        let Some(verdict) = row.get("verdict").and_then(toml::Value::as_str) else {
            problems.push(format!("{leaf}: no verdict"));
            continue;
        };
        if !verdicts.contains(&verdict) {
            problems.push(format!("{leaf}: verdict {verdict:?}"));
        }
        for key in row.keys() {
            if !row_keys.contains(&key.as_str()) {
                problems.push(format!("{leaf}: row key {key}"));
            }
        }
        let has = |key: &str| row.contains_key(key);
        if verdict == "unmappable" {
            match row.get("sub_tag").and_then(toml::Value::as_str) {
                Some(tag) if sub_tags.contains(&tag) => {}
                other => problems.push(format!("{leaf}: sub_tag {other:?}")),
            }
            if !row
                .get("upstream")
                .and_then(toml::Value::as_str)
                .is_some_and(|u| !u.trim().is_empty())
            {
                problems.push(format!(
                    "{leaf}: an unmappable row names its upstream change"
                ));
            }
            for key in ["sinks", "tested_by", "exceptions", "probe"] {
                if has(key) {
                    problems.push(format!("{leaf}: {key} on an unmappable row"));
                }
            }
            continue;
        }
        for key in ["sub_tag", "upstream"] {
            if has(key) {
                problems.push(format!("{leaf}: {key} on a {verdict} row"));
            }
        }
        if has("sinks") == has("tested_by") {
            problems.push(format!("{leaf}: exactly one of sinks and tested_by"));
        }
        if let Some(t) = row.get("tested_by") {
            match t.as_str() {
                Some(t) if is_clause(t) => {}
                other => problems.push(format!("{leaf}: tested_by {other:?} names no clause")),
            }
        }
        if let Some(sinks) = row.get("sinks") {
            let Some(sinks) = sinks.as_array() else {
                problems.push(format!("{leaf}: sinks is an array"));
                continue;
            };
            if sinks.is_empty() {
                problems.push(format!("{leaf}: no sink"));
            }
            for sink in sinks {
                let Some(sink) = sink.as_table() else {
                    problems.push(format!("{leaf}: a sink is an inline table"));
                    continue;
                };
                match sink.get("path").and_then(toml::Value::as_str) {
                    Some(path) if !path.is_empty() => {
                        declared_paths.insert(path.to_owned());
                    }
                    _ => problems.push(format!("{leaf}: a sink names its path")),
                }
                for key in sink.keys() {
                    if !sink_keys.contains(&key.as_str()) {
                        problems.push(format!("{leaf}: sink key {key}"));
                    }
                }
                let scope = sink.get("scope").and_then(toml::Value::as_str);
                if let Some(scope) = scope {
                    if !role_keys.contains(scope) {
                        problems.push(format!("{leaf}: scope {scope:?} is no Role::key()"));
                    }
                    if sink.contains_key("surface") {
                        problems.push(format!("{leaf}: scope beside surface"));
                    }
                }
                if let Some(variant) = sink.get("variant").and_then(toml::Value::as_str) {
                    if scope.is_none() {
                        problems.push(format!("{leaf}: variant without scope"));
                    }
                    if !variant_keys.contains(variant) {
                        problems.push(format!("{leaf}: variant {variant:?}"));
                    }
                }
                if let Some(surface) = sink.get("surface").and_then(toml::Value::as_str)
                    && !surface_keys.contains(surface)
                {
                    problems.push(format!("{leaf}: surface {surface:?} is no Surface::key()"));
                }
                if sink.contains_key("when")
                    && !sink
                        .get("when")
                        .and_then(toml::Value::as_str)
                        .is_some_and(|w| !w.trim().is_empty())
                {
                    problems.push(format!("{leaf}: when is a sentence"));
                }
            }
        }
        if let Some(exceptions) = row.get("exceptions") {
            let Some(exceptions) = exceptions.as_array() else {
                problems.push(format!("{leaf}: exceptions is an array"));
                continue;
            };
            for pair in exceptions {
                let pair = pair.as_array().map(|p| (p.first(), p.get(1), p.len()));
                match pair {
                    Some((Some(preset), Some(reason), 2)) => {
                        let preset = preset.as_str().unwrap_or("");
                        if !presets.contains(preset) {
                            problems.push(format!("{leaf}: exception names no preset: {preset:?}"));
                        }
                        if !reason.as_str().is_some_and(|r| !r.trim().is_empty()) {
                            problems.push(format!("{leaf}: an exception gives its reason"));
                        }
                    }
                    _ => problems.push(format!("{leaf}: an exception is [preset, reason]")),
                }
            }
        }
    }

    match manifest.get("unwritten").and_then(toml::Value::as_table) {
        Some(unwritten) => {
            for (path, reason) in unwritten {
                if !reason.as_str().is_some_and(|r| !r.trim().is_empty()) {
                    problems.push(format!("[unwritten] {path}: a reason"));
                }
                if declared_paths.contains(path) {
                    problems.push(format!("[unwritten] {path}: also a declared sink"));
                }
            }
        }
        None => problems.push("no [unwritten] table".to_owned()),
    }

    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// §5.7: the transcription carries the specification's verdicts, group by group, and its
/// sub-tags (15 + 5 + 54). Read through the typed reader.
#[test]
fn verdict_totals_match_the_specification() {
    let group_of = |leaf: &str| -> &'static str {
        match leaf.split('.').next().unwrap_or("") {
            "defaults" | "text_scale" | "layout" => "foundation",
            "window" | "dialog" | "popover" | "card" | "tooltip" | "menu" => "surfaces",
            "button" | "link" | "switch" | "checkbox" | "segmented_control" => "buttons",
            "input" | "combo_box" | "list" => "inputs",
            "scrollbar" | "slider" | "progress_bar" | "splitter" | "separator" | "spinner" => {
                "indicators"
            }
            "tab" | "sidebar" | "toolbar" | "status_bar" | "expander" => "chrome",
            _ => "unknown",
        }
    };
    let column = |v: Verdict| match v {
        Verdict::Direct => 0,
        Verdict::Scoped => 1,
        Verdict::Derived => 2,
        Verdict::Unmappable => 3,
    };
    let manifest = Manifest::load();
    let mut counts: BTreeMap<&str, [usize; 4]> = BTreeMap::new();
    let mut tags: BTreeMap<&str, usize> = BTreeMap::new();
    for (leaf, row) in &manifest.rows {
        counts.entry(group_of(leaf)).or_insert([0; 4])[column(row.verdict)] += 1;
        if let Some(tag) = &row.sub_tag {
            *tags.entry(tag.as_str()).or_default() += 1;
        }
    }
    // [direct, scoped, derived, unmappable] per group: §5.7's table
    let expected = [
        ("foundation", [18, 1, 37, 10]),
        ("surfaces", [8, 55, 31, 14]),
        ("buttons", [11, 45, 29, 14]),
        ("inputs", [4, 35, 28, 11]),
        ("indicators", [6, 17, 8, 9]),
        ("chrome", [0, 53, 23, 15]),
    ];
    for (group, want) in expected {
        let got = counts.get(group).copied().unwrap_or([0; 4]);
        assert_eq!(got, want, "{group}: [direct, scoped, derived, unmappable]");
    }
    assert!(
        !counts.contains_key("unknown"),
        "a leaf outside the six groups"
    );
    assert_eq!(tags.get("source-side gap").copied().unwrap_or_default(), 15);
    assert_eq!(tags.get("widgets-crate").copied().unwrap_or_default(), 5);
    assert_eq!(tags.get("egui-limited").copied().unwrap_or_default(), 53);
    assert_eq!(
        tags.get("source-void").copied().unwrap_or_default(),
        0,
        "no row is source-void (§5.7)"
    );
}

/// The typed reader carries every key of every row, as the raw table T3 validated states
/// it. The destructurings name every field, so a field added to `Row` or `Sink` without a
/// check fails to compile here.
#[test]
fn the_typed_reader_matches_the_raw_table() {
    let raw = manifest();
    let typed = Manifest::load();
    assert_eq!(
        typed.rows.len() + 1,
        raw.len(),
        "every key but [unwritten] is a row"
    );
    for (
        leaf,
        Row {
            verdict,
            sinks,
            tested_by,
            probe,
            sub_tag,
            upstream,
            exceptions,
        },
    ) in &typed.rows
    {
        let row = raw
            .get(leaf)
            .and_then(toml::Value::as_table)
            .expect("a raw row");
        let text = |key: &str| row.get(key).and_then(toml::Value::as_str);
        let spelled = match verdict {
            Verdict::Direct => "direct",
            Verdict::Scoped => "scoped",
            Verdict::Derived => "derived",
            Verdict::Unmappable => "unmappable",
        };
        assert_eq!(text("verdict"), Some(spelled), "{leaf}");
        assert_eq!(text("tested_by"), tested_by.as_deref(), "{leaf}");
        assert_eq!(text("sub_tag"), sub_tag.as_deref(), "{leaf}");
        assert_eq!(text("upstream"), upstream.as_deref(), "{leaf}");
        assert_eq!(row.get("probe"), probe.as_ref(), "{leaf}");
        let raw_exceptions = row
            .get("exceptions")
            .and_then(toml::Value::as_array)
            .map(Vec::len)
            .unwrap_or_default();
        assert_eq!(raw_exceptions, exceptions.len(), "{leaf}");
        let raw_sinks = row
            .get("sinks")
            .and_then(toml::Value::as_array)
            .map_or(&[][..], Vec::as_slice);
        assert_eq!(raw_sinks.len(), sinks.len(), "{leaf}");
        for (
            raw_sink,
            Sink {
                path,
                scope,
                variant,
                surface,
                when,
            },
        ) in raw_sinks.iter().zip(sinks)
        {
            let field = |key: &str| raw_sink.get(key).and_then(toml::Value::as_str);
            assert_eq!(
                (
                    field("path"),
                    field("scope"),
                    field("variant"),
                    field("surface"),
                    field("when")
                ),
                (
                    Some(path.as_str()),
                    scope.as_deref(),
                    variant.as_deref(),
                    surface.as_deref(),
                    when.as_deref()
                ),
                "{leaf}"
            );
        }
    }
    let unwritten = raw
        .get("unwritten")
        .and_then(toml::Value::as_table)
        .expect("an [unwritten] table");
    assert_eq!(unwritten.len(), typed.unwritten.len());
}

/// T4 (a)'s six values.
const HOSTILE: [f32; 6] = [
    f32::NAN,
    f32::INFINITY,
    f32::NEG_INFINITY,
    -0.0,
    1e30,
    -1e30,
];

/// Mutated leaf by leaf: the two presets that state the most optional leaves — kde-breeze's
/// paddings and sizes, adwaita's checkbox padding (§6.11).
const PER_LEAF: [&str; 2] = ["kde-breeze", "adwaita"];

/// Mutated all at once, and walked through `dnd_drop_zone`.
const PLATFORM: [&str; 4] = ["kde-breeze", "adwaita", "macos-sonoma", "windows-11"];

const MODES: [ColorMode; 2] = [ColorMode::Light, ColorMode::Dark];

fn as_table(t: &ResolvedTheme) -> toml::Table {
    toml::from_str(&toml::to_string(t).expect("serialises")).expect("parses back")
}

fn collect_floats(v: &toml::Value, path: &str, out: &mut Vec<String>) {
    match v {
        toml::Value::Table(t) => {
            for (k, v) in t {
                let p = if path.is_empty() {
                    k.clone()
                } else {
                    format!("{path}.{k}")
                };
                collect_floats(v, &p, out);
            }
        }
        toml::Value::Float(_) => out.push(path.to_owned()),
        _ => {}
    }
}

/// Every `f32` leaf of `t`, as a TOML path. A colour is a string
/// (`native-theme/src/color.rs:250-252`) and a weight an integer, so only the theme's `f32`s
/// are floats; an `Option<f32>` that is `None` is absent.
fn float_leaves(t: &ResolvedTheme) -> Vec<String> {
    let mut out = Vec::new();
    collect_floats(&toml::Value::Table(as_table(t)), "", &mut out);
    out
}

/// The `mapping.toml` leaf of a TOML path: the walk stops at `defined_size` (§13.1).
fn leaf_of(toml_path: &str) -> &str {
    toml_path
        .strip_suffix(".Pt")
        .or_else(|| toml_path.strip_suffix(".Px"))
        .unwrap_or(toml_path)
}

fn set_floats(v: &mut toml::Value, path: &str, only: Option<&str>, to: f64) {
    match v {
        toml::Value::Table(t) => {
            for (k, v) in t.iter_mut() {
                let p = if path.is_empty() {
                    k.clone()
                } else {
                    format!("{path}.{k}")
                };
                set_floats(v, &p, only, to);
            }
        }
        toml::Value::Float(f) if only.is_none_or(|o| o == path) => *f = to,
        _ => {}
    }
}

/// `t` with every unstated `Option<f32>` leaf — a `null` that deserialises from a number: the
/// padding sides and rationale §3.23's optional sizes — stated as `0.0`, so that the all-at-once
/// legs write their hostile `Some` into every one, including those no platform preset states.
fn with_every_option_stated(t: &ResolvedTheme) -> ResolvedTheme {
    fn nulls(pointer: &str, v: &serde_json::Value, out: &mut Vec<String>) {
        match v {
            serde_json::Value::Object(o) => {
                for (k, v) in o {
                    nulls(&format!("{pointer}/{k}"), v, out);
                }
            }
            serde_json::Value::Null => out.push(pointer.to_owned()),
            _ => {}
        }
    }
    let mut json = serde_json::to_value(t).expect("ResolvedTheme serialises");
    let mut pointers = Vec::new();
    nulls("", &json, &mut pointers);
    for p in pointers {
        let mut trial = json.clone();
        *trial.pointer_mut(&p).expect("the walk found it") = serde_json::json!(0.0);
        if serde_json::from_value::<ResolvedTheme>(trial.clone()).is_ok() {
            json = trial;
        }
    }
    serde_json::from_value(json).expect("a ResolvedTheme with every option stated")
}

/// Every padding side of every widget border is an `f32` leaf of the stated theme, and some were
/// unstated before (T4 (a)'s "every `Some`" reaches them only this way).
#[test]
fn every_option_stated_states_every_padding_side() {
    let mut newly = BTreeSet::new();
    for preset in PLATFORM {
        for mode in MODES {
            let t = resolved(preset, mode);
            let before: BTreeSet<String> = float_leaves(&t).into_iter().collect();
            let after: BTreeSet<String> = float_leaves(&with_every_option_stated(&t))
                .into_iter()
                .collect();
            assert!(before.is_subset(&after), "{preset} {mode:?}");
            let json = serde_json::to_value(&t).expect("ResolvedTheme serialises");
            let widgets = json.as_object().expect("an object");
            for (widget, v) in widgets {
                if v.pointer("/border/padding").is_some() {
                    for side in ["top", "right", "bottom", "left"] {
                        let leaf = format!("{widget}.border.padding.{side}");
                        assert!(after.contains(&leaf), "{preset} {mode:?} {leaf}");
                        if !before.contains(&leaf) {
                            newly.insert(leaf);
                        }
                    }
                }
            }
        }
    }
    assert!(
        !newly.is_empty(),
        "no platform preset leaves a padding side unstated"
    );
}

/// `t` with the float at `only` set to `v`, or every float when `only` is `None`.
fn with_hostile(t: &ResolvedTheme, only: Option<&str>, v: f32) -> ResolvedTheme {
    let mut root = toml::Value::Table(as_table(t));
    set_floats(&mut root, "", only, f64::from(v));
    root.try_into()
        .expect("a ResolvedTheme with hostile floats")
}

fn build(t: &ResolvedTheme) -> ThemeAtlas {
    ThemeAtlas::builder("t4", t, t).build()
}

/// The `tested_by` leaves `Builder::build` itself reads and sanitises (§7.2 does not exempt a
/// `tested_by` row): the focus ring's width and offset, which `build_focus_ring` reports when
/// non-finite (§6.18). `focus_ring_color` is a colour, not an `f32` leaf.
const BUILD_READ_TESTED_BY: &[&str] = &["defaults.focus_ring_width", "defaults.focus_ring_offset"];

/// What §7.2's emission rule predicts for one leaf set to `v`: `ValueSanitised` and
/// `ValueSaturated` only — the two kinds a hostile float causes.
fn predicted(m: &Manifest, leaf: &str, v: f32) -> Vec<Note> {
    let row = m
        .rows
        .get(leaf)
        .unwrap_or_else(|| panic!("no mapping row for {leaf}"));
    let path: &'static str = leaf.to_owned().leak();
    let read_by_build = !row.sinks.is_empty() || BUILD_READ_TESTED_BY.contains(&leaf);
    let unread = row.verdict == Verdict::Unmappable || !read_by_build;
    let text_size = row
        .sinks
        .iter()
        .any(|s| s.path.starts_with("text_styles[") || s.path.starts_with("override_font_id"));
    let i8_sink = row.sinks.iter().any(|s| {
        s.path.contains("inner_margin.")
            || s.path.starts_with("spacing.window_margin.")
            || s.path.starts_with("spacing.menu_margin.")
    });
    let mut out = Vec::new();
    if !unread {
        // §8.5: a text size that is not a positive normal `f32`; §6: any other non-finite leaf.
        let sanitised = if text_size {
            !(v.is_normal() && v > 0.0)
        } else {
            !v.is_finite()
        };
        if sanitised {
            out.push(Note::ValueSanitised { path });
        }
        if v.is_finite() && i8_sink && crate::style::saturates_i8(v) {
            out.push(Note::ValueSaturated { path });
        }
    }
    out
}

fn dedup(notes: &[Note]) -> Vec<&Note> {
    let mut out: Vec<&Note> = Vec::new();
    for n in notes {
        if !out.contains(&n) {
            out.push(n);
        }
    }
    out
}

/// `got` equals the unmutated atlas's notes (`baseline`) plus `predicted`, as sets, over every
/// note kind: a kind a hostile float does not cause — `TransparentFill` (windows-11's
/// `#f9f9f900` checkbox, `native-theme/src/presets/windows-11.toml:133`), the font notes — must
/// be exactly the baseline's.
fn assert_same_notes(got: &[Note], baseline: &[Note], predicted: Vec<Note>, ctx: &str) {
    let mut want = baseline.to_vec();
    want.extend(predicted);
    let (got, want) = (dedup(got), dedup(&want));
    for n in &got {
        assert!(want.contains(n), "{ctx}: unexpected {n:?}");
    }
    for n in &want {
        assert!(got.contains(n), "{ctx}: missing {n:?}");
    }
}

/// Every produced `Style` and `Frame` equals itself: a `NaN` in a scalar field makes a value
/// unequal to itself, and one in a `Vec2` panics inside `Style`'s derived `PartialEq`
/// (`egui/src/style.rs:241`) through `Vec2::eq`'s debug assert (`emath/src/vec2.rs:349-359`) —
/// either way the test fails, the panic naming the vectors rather than the location.
fn assert_nan_free(atlas: &ThemeAtlas, ctx: &str) {
    for (loc, s) in all_styles(atlas) {
        assert!(self_equal(&*s), "{ctx}: {loc:?} is not equal to itself");
    }
    for (loc, fr) in all_frames(atlas) {
        assert!(self_equal(&fr), "{ctx}: {loc:?} is not equal to itself");
    }
}

/// T4 (a): hostile values in every `f32` leaf — one at a time for `NaN` and `+∞`, all at once
/// for the six — build, produce `NaN`-free styles and emit exactly the predicted notes.
#[test]
fn t4a_hostile_leaves_build_nan_free_and_report_exactly_the_predicted_notes() {
    let m = Manifest::load();
    for preset in PER_LEAF {
        for mode in MODES {
            let t = resolved(preset, mode);
            let baseline = build(&t);
            for toml_path in float_leaves(&t) {
                let leaf = leaf_of(&toml_path);
                for v in [f32::NAN, f32::INFINITY] {
                    let ctx = format!("{preset} {mode:?} {leaf} = {v}");
                    let atlas = build(&with_hostile(&t, Some(&toml_path), v));
                    assert_nan_free(&atlas, &ctx);
                    assert_same_notes(
                        atlas.notes(),
                        baseline.notes(),
                        predicted(&m, leaf, v),
                        &ctx,
                    );
                }
            }
        }
    }
    for preset in PLATFORM {
        for mode in MODES {
            let t = with_every_option_stated(&resolved(preset, mode));
            let baseline = build(&t);
            for v in HOSTILE {
                let ctx = format!("{preset} {mode:?} every leaf = {v}");
                let atlas = build(&with_hostile(&t, None, v));
                assert_nan_free(&atlas, &ctx);
                let want: Vec<Note> = float_leaves(&t)
                    .iter()
                    .flat_map(|p| predicted(&m, leaf_of(p), v))
                    .collect();
                assert_same_notes(atlas.notes(), baseline.notes(), want, &ctx);
            }
        }
    }
}

/// T4 (b): every `Visuals` the all-at-once atlases produce goes through `Ui::dnd_drop_zone`
/// while a payload of another type is being dragged — the only path to `Visuals::disable`
/// (`egui/src/ui.rs:2726-2727`) and `Color32::gamma_multiply`'s
/// `debug_assert!(0.0 <= factor && factor.is_finite())` (`ecolor/src/color32.rs:270-273`).
/// `Ui::disable` never reaches it (§7.4).
#[test]
fn t4b_every_hostile_visuals_survives_egui_s_own_disable_assert() {
    let mut styles: Vec<(Location, Arc<egui::Style>)> = Vec::new();
    for preset in PLATFORM {
        for mode in MODES {
            let t = with_every_option_stated(&resolved(preset, mode));
            for v in HOSTILE {
                styles.extend(all_styles(&build(&with_hostile(&t, None, v))));
            }
        }
    }
    let ctx = egui::Context::default();
    let _ = pass(&ctx, egui::RawInput::default(), |ui| {
        // `DragAndDrop` is registered with every `Context` (`egui/src/context.rs:751`);
        // a `u8` payload is not the `String` the zone accepts, so it disables its colours.
        egui::DragAndDrop::set_payload(ui.ctx(), 1_u8);
        for (loc, s) in &styles {
            // The label's text sizes stay egui's: a finite size such as `1e30` outgrows the font
            // atlas, the stated residual of §8.5 and §14 item 44, which is not this test's.
            let mut style = (**s).clone();
            style.text_styles = egui::Style::default().text_styles;
            style.override_font_id = None;
            ui.scope_builder(egui::UiBuilder::new().style(style), |ui| {
                let _ = ui.dnd_drop_zone::<String, _>(egui::Frame::NONE, |ui| {
                    ui.label(format!("{loc:?}"));
                });
            });
        }
    });
}

/// The preset pairs T2 splices between, in both modes: the four platform presets pairwise,
/// and two community pairs.
const PAIRS: [(&str, &str); 8] = [
    ("kde-breeze", "adwaita"),
    ("kde-breeze", "macos-sonoma"),
    ("kde-breeze", "windows-11"),
    ("adwaita", "macos-sonoma"),
    ("adwaita", "windows-11"),
    ("macos-sonoma", "windows-11"),
    ("material", "catppuccin-mocha"),
    ("dracula", "solarized"),
];

const DISABLED_CELLS: [Role; 9] = [
    Role::Button,
    Role::ComboBox,
    Role::Checkbox,
    Role::Input,
    Role::Slider,
    Role::Switch,
    Role::Menu,
    Role::List,
    Role::Link,
];

/// What `menu_style` sets in the `Role::Menu` cells before the role's writes
/// (`egui/src/containers/menu.rs:22-29`, §3.4).
const MENU_STYLE_PATHS: [&str; 6] = [
    "spacing.button_padding",
    "visuals.widgets.active.bg_stroke",
    "visuals.widgets.open.bg_stroke",
    "visuals.widgets.hovered.bg_stroke",
    "visuals.widgets.inactive.weak_bg_fill",
    "visuals.widgets.inactive.bg_stroke",
];

/// Which frame field each surface's preset reads from which base-style field (§3.2;
/// `egui/src/containers/frame.rs:178-221`).
pub(crate) fn preset_reads(surface: Surface) -> &'static [(&'static str, &'static str)] {
    match surface {
        Surface::Window | Surface::WindowTitleBar => &[
            ("inner_margin", "spacing.window_margin"),
            ("corner_radius", "visuals.window_corner_radius"),
            ("shadow", "visuals.window_shadow"),
            ("fill", "visuals.window_fill"),
            ("stroke", "visuals.window_stroke"),
        ],
        Surface::Dialog | Surface::Popover | Surface::Tooltip => &[
            ("inner_margin", "spacing.menu_margin"),
            ("corner_radius", "visuals.menu_corner_radius"),
            ("shadow", "visuals.popup_shadow"),
            ("fill", "visuals.window_fill"),
            ("stroke", "visuals.window_stroke"),
        ],
        Surface::Card => &[
            (
                "corner_radius",
                "visuals.widgets.noninteractive.corner_radius",
            ),
            ("stroke", "visuals.widgets.noninteractive.bg_stroke"),
        ],
        Surface::Panel(_) | Surface::CentralPanel => &[("fill", "visuals.panel_fill")],
    }
}

/// `path` is `sink` itself or a field inside it.
pub(crate) fn covers(sink: &str, path: &str) -> bool {
    path == sink
        || path
            .strip_prefix(sink)
            .is_some_and(|rest| rest.starts_with('.'))
}

/// The rest of `path` below `prefix`: `""` when equal, `.x` when a field inside it.
pub(crate) fn below<'a>(prefix: &str, path: &'a str) -> Option<&'a str> {
    if path == prefix {
        Some("")
    } else {
        path.strip_prefix(prefix)
            .filter(|rest| rest.starts_with('.'))
    }
}

pub(crate) fn role_by_key(key: &str) -> Role {
    *Role::all()
        .iter()
        .find(|r| r.key() == key)
        .unwrap_or_else(|| panic!("no role {key}"))
}

pub(crate) fn variant_by_key(key: &str) -> RoleVariant {
    *RoleVariant::all()
        .iter()
        .find(|v| v.key() == key)
        .unwrap_or_else(|| panic!("no variant {key}"))
}

pub(crate) fn surface_by_key(key: &str) -> Surface {
    *Surface::all()
        .iter()
        .find(|s| s.key() == key)
        .unwrap_or_else(|| panic!("no surface {key}"))
}

/// One preset in one mode, as T2 splices into it.
struct Cached {
    json: serde_json::Value,
    theme: ResolvedTheme,
    layout: LayoutTheme,
    atlas: ThemeAtlas,
}

/// Whether `a` states `leaf`: a `null` is an unstated padding side or a `None` option.
fn stated(a: &Cached, leaf: &str) -> bool {
    match leaf.strip_prefix("layout.") {
        Some(key) => layout_field(&a.layout, key).is_some(),
        None => a.json.pointer(&pointer(leaf)).is_some_and(|v| !v.is_null()),
    }
}

/// How a location built from another one writes a field itself (§13.1, *Inheritance*).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Own {
    /// Nothing declared there, or only sinks whose leaf the theme leaves unstated: the location
    /// carries what it is built from.
    Carries,
    /// A `when` sink, or a stated and an unstated leaf together (§5's mean of two targets): the
    /// starting value can still show through, so a change there is allowed, not required.
    Partly,
    /// Every sink declared there without a `when` has its leaf stated, or §6 writes the field
    /// with no row: the location holds its own value.
    Overrides,
}

/// `Own` from the sinks `at` selects, read against the theme `a` spliced into.
fn own_write(m: &Manifest, a: &Cached, at: impl Fn(&Sink) -> bool) -> Own {
    let (mut any_stated, mut any_unstated, mut any_when) = (false, false, false);
    for (leaf, row) in &m.rows {
        for s in row.sinks.iter().filter(|s| at(s)) {
            if s.when.is_some() {
                any_when = true;
            } else if stated(a, leaf) {
                any_stated = true;
            } else {
                any_unstated = true;
            }
        }
    }
    match (any_stated, any_unstated, any_when) {
        (false, _, false) => Own::Carries,
        (true, false, _) => Own::Overrides,
        _ => Own::Partly,
    }
}

/// Whether §6 writes `path` in the cell (`role`, `variant`) with no manifest row.
pub(crate) fn no_row_write(role: Role, variant: RoleVariant, path: &str) -> bool {
    (variant == RoleVariant::Disabled
        && DISABLED_CELLS.contains(&role)
        && path == "visuals.disabled_alpha")
        || (role == Role::Expander
            && variant == RoleVariant::Normal
            && (path == "visuals.widgets.inactive.weak_bg_fill"
                || path == "visuals.widgets.open.weak_bg_fill"))
        || (role == Role::Menu
            && variant == RoleVariant::Normal
            && MENU_STYLE_PATHS.iter().any(|p| covers(p, path)))
        // §2 *Unstated sizes*: an unstated row height writes egui's own `interact_size.y`
        // (`style::roles::row_height`), so these cells never carry the base style's.
        || (matches!(role, Role::Menu | Role::List | Role::Toolbar)
            && variant == RoleVariant::Normal
            && path == "spacing.interact_size.y")
}

/// Whether the cell (`role`, `variant`) writes `path` itself: the sinks the manifest declares
/// at that cell, or one of the no-row writes §6 makes there.
fn cell_own(m: &Manifest, a: &Cached, role: Role, variant: RoleVariant, path: &str) -> Own {
    if no_row_write(role, variant, path) {
        return Own::Overrides;
    }
    own_write(m, a, |s| {
        s.scope.as_deref() == Some(role.key())
            && s.variant.as_deref().unwrap_or("normal") == variant.key()
            && covers(&s.path, path)
    })
}

/// Whether `surface`'s frame writes `field` itself.
fn frame_own(m: &Manifest, a: &Cached, surface: Surface, field: &str) -> Own {
    own_write(m, a, |s| {
        s.surface.as_deref() == Some(surface.key()) && covers(&s.path, field)
    })
}

/// Every location that carries one declared sink (§13.1, *Inheritance*) in one scheme of an
/// atlas built from `a`, and whether a change of the leaf must move it there.
fn carried_by(
    m: &Manifest,
    a: &Cached,
    s: &Sink,
    theme: egui::Theme,
) -> Vec<(Location, String, bool)> {
    let mut out = Vec::new();
    let required = s.when.is_none();
    match (s.scope.as_deref(), s.surface.as_deref()) {
        (None, None) => {
            out.push((Location::Base(theme), s.path.clone(), required));
            for role in Role::all() {
                let normal = cell_own(m, a, *role, RoleVariant::Normal, &s.path);
                if normal == Own::Overrides {
                    continue;
                }
                let at_normal = required && normal == Own::Carries;
                out.push((
                    Location::Cell(theme, *role, RoleVariant::Normal),
                    s.path.clone(),
                    at_normal,
                ));
                for v in [RoleVariant::Selected, RoleVariant::Disabled] {
                    let own = cell_own(m, a, *role, v, &s.path);
                    if own != Own::Overrides {
                        out.push((
                            Location::Cell(theme, *role, v),
                            s.path.clone(),
                            at_normal && own == Own::Carries,
                        ));
                    }
                }
            }
            for surface in Surface::all() {
                for (field, base_path) in preset_reads(*surface) {
                    if let Some(rest) = below(base_path, &s.path) {
                        let fp = format!("{field}{rest}");
                        let own = frame_own(m, a, *surface, &fp);
                        if own != Own::Overrides {
                            out.push((
                                Location::Frame(theme, *surface),
                                fp,
                                required && own == Own::Carries,
                            ));
                        }
                    }
                }
            }
        }
        (Some(scope), None) => {
            let role = role_by_key(scope);
            let variant = variant_by_key(s.variant.as_deref().unwrap_or("normal"));
            out.push((
                Location::Cell(theme, role, variant),
                s.path.clone(),
                required,
            ));
            if variant == RoleVariant::Normal {
                for v in [RoleVariant::Selected, RoleVariant::Disabled] {
                    let own = cell_own(m, a, role, v, &s.path);
                    if own != Own::Overrides {
                        out.push((
                            Location::Cell(theme, role, v),
                            s.path.clone(),
                            required && own == Own::Carries,
                        ));
                    }
                }
            }
        }
        (None, Some(surface)) => {
            out.push((
                Location::Frame(theme, surface_by_key(surface)),
                s.path.clone(),
                required,
            ));
        }
        (Some(scope), Some(surface)) => {
            panic!("a sink names both a scope ({scope}) and a surface ({surface})")
        }
    }
    out
}

fn json(t: &ResolvedTheme) -> serde_json::Value {
    serde_json::to_value(t).expect("serialises")
}

fn pointer(leaf: &str) -> String {
    format!("/{}", leaf.replace('.', "/"))
}

/// `a` with `leaf` replaced by `value`; `None` where they already agree.
fn spliced(a: &serde_json::Value, leaf: &str, value: &serde_json::Value) -> Option<ResolvedTheme> {
    let p = pointer(leaf);
    let current = a
        .pointer(&p)
        .unwrap_or_else(|| panic!("{leaf} is not a leaf of ResolvedTheme"));
    if current == value {
        return None;
    }
    let mut b = a.clone();
    *b.pointer_mut(&p).expect("the same leaf") = value.clone();
    Some(serde_json::from_value(b).expect("a ResolvedTheme with one leaf replaced"))
}

fn toml_to_json(v: &toml::Value) -> serde_json::Value {
    match v {
        toml::Value::String(s) => serde_json::json!(s),
        toml::Value::Integer(i) => serde_json::json!(i),
        toml::Value::Float(f) => serde_json::json!(f),
        toml::Value::Boolean(b) => serde_json::json!(b),
        toml::Value::Datetime(d) => serde_json::json!(d.to_string()),
        toml::Value::Array(a) => serde_json::Value::Array(a.iter().map(toml_to_json).collect()),
        toml::Value::Table(t) => serde_json::Value::Object(
            t.iter()
                .map(|(k, v)| (k.clone(), toml_to_json(v)))
                .collect(),
        ),
    }
}

/// The four `LayoutTheme` leaves are not in `ResolvedTheme` (§13.1); a `layout.*` row is
/// spliced on the builder's layout instead.
fn layout_field(l: &LayoutTheme, key: &str) -> Option<f32> {
    match key {
        "widget_gap" => l.widget_gap,
        "container_margin" => l.container_margin,
        "window_margin" => l.window_margin,
        "section_gap" => l.section_gap,
        _ => panic!("no layout leaf {key}"),
    }
}

fn with_layout_field(l: &LayoutTheme, key: &str, v: Option<f32>) -> LayoutTheme {
    let mut out = l.clone();
    match key {
        "widget_gap" => out.widget_gap = v,
        "container_margin" => out.container_margin = v,
        "window_margin" => out.window_margin = v,
        "section_gap" => out.section_gap = v,
        _ => panic!("no layout leaf {key}"),
    }
    out
}

fn build_with(t: &ResolvedTheme, layout: &LayoutTheme) -> ThemeAtlas {
    ThemeAtlas::builder("t2", t, t).layout(layout).build()
}

fn loc_name(loc: Location) -> String {
    match loc {
        Location::Base(t) => format!("base[{t:?}]"),
        Location::Cell(t, r, v) => format!("{}[{t:?},{}]", r.key(), v.key()),
        Location::Frame(t, s) => format!("{}[{t:?}]", s.key()),
    }
}

impl Change {
    fn location_name(&self) -> String {
        loc_name(self.location)
    }
}

/// One splice of one row into `a`: every change must be carried by a declared sink (a
/// `tested_by` row allows none), and the changes that are not are returned; each required
/// location, keyed by its name and sink, is marked moved or not in `moved`.
fn check_splice(
    m: &Manifest,
    leaf: &str,
    row: &Row,
    a: &Cached,
    changes: &[Change],
    ctx: &str,
    moved: &mut BTreeMap<(String, String), bool>,
) -> Vec<String> {
    let mut problems = Vec::new();
    if row.tested_by.is_some() {
        for c in changes {
            problems.push(format!(
                "{ctx}: {leaf} is tested_by and moved {}:{}",
                c.location_name(),
                c.path
            ));
        }
        return problems;
    }
    let mut carried: Vec<(Location, String, bool)> = Vec::new();
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        for s in &row.sinks {
            carried.extend(carried_by(m, a, s, theme));
        }
    }
    for c in changes {
        if !carried
            .iter()
            .any(|(loc, sink, _)| *loc == c.location && covers(sink, &c.path))
        {
            problems.push(format!(
                "{ctx}: {leaf} moved {}:{} which no declared sink carries",
                c.location_name(),
                c.path
            ));
        }
    }
    for (loc, sink, required) in carried {
        if required {
            let hit = changes
                .iter()
                .any(|c| c.location == loc && covers(&sink, &c.path));
            *moved.entry((loc_name(loc), sink)).or_default() |= hit;
        }
    }
    problems
}

/// T2: for every row that is not `unmappable`, splicing its leaf from preset B into preset A —
/// every pair that differentiates it, in both modes, and its `probe` — moves only sinks the
/// row declares, at a location that carries them, and moves each required one under at least
/// one splice (two presets can agree on a sink's value by chance); a row with neither a
/// differentiating pair nor a `probe` fails by name.
#[test]
fn t2_every_row_s_declared_sinks_are_exactly_what_its_leaf_moves() {
    let m = Manifest::load();
    let mut cache: Vec<((&str, ColorMode), Cached)> = Vec::new();
    for (a, b) in PAIRS {
        for name in [a, b] {
            for mode in MODES {
                if cache.iter().any(|(k, _)| *k == (name, mode)) {
                    continue;
                }
                let theme = resolved(name, mode);
                let layout = Theme::preset(name)
                    .expect("a bundled preset")
                    .layout
                    .clone();
                let atlas = build_with(&theme, &layout);
                cache.push((
                    (name, mode),
                    Cached {
                        json: json(&theme),
                        theme,
                        layout,
                        atlas,
                    },
                ));
            }
        }
    }
    let get = |name: &str, mode: ColorMode| -> &Cached {
        &cache
            .iter()
            .find(|(k, _)| *k == (name, mode))
            .expect("cached")
            .1
    };

    let mut problems: Vec<String> = Vec::new();
    let mut unexercised = Vec::new();
    for (leaf, row) in &m.rows {
        if row.verdict == Verdict::Unmappable {
            continue;
        }
        let mut moved: BTreeMap<(String, String), bool> = BTreeMap::new();
        let mut splices = 0_usize;
        for mode in MODES {
            for (a_name, b_name) in PAIRS {
                let (a, b) = (get(a_name, mode), get(b_name, mode));
                let ctx = format!("{a_name} <- {b_name} [{mode:?}]");
                let spliced_atlas = if let Some(key) = leaf.strip_prefix("layout.") {
                    let (va, vb) = (layout_field(&a.layout, key), layout_field(&b.layout, key));
                    if va == vb {
                        continue;
                    }
                    build_with(&a.theme, &with_layout_field(&a.layout, key, vb))
                } else {
                    let value = b.json.pointer(&pointer(leaf)).expect("a leaf T3 walked");
                    let Some(t) = spliced(&a.json, leaf, value) else {
                        continue;
                    };
                    build_with(&t, &a.layout)
                };
                let changes = atlas_diff(&a.atlas, &spliced_atlas);
                problems.extend(check_splice(&m, leaf, row, a, &changes, &ctx, &mut moved));
                splices += 1;
            }
        }
        if let Some(probe) = &row.probe {
            let &(a_name, _) = PAIRS.first().expect("PAIRS is not empty");
            let a = get(a_name, ColorMode::Light);
            let ctx = format!("{a_name} <- probe [Light]");
            let spliced_atlas = if let Some(key) = leaf.strip_prefix("layout.") {
                let v = probe
                    .as_float()
                    .map(|f| f as f32)
                    .expect("a float probe for a layout leaf");
                build_with(&a.theme, &with_layout_field(&a.layout, key, Some(v)))
            } else {
                let t = spliced(&a.json, leaf, &toml_to_json(probe))
                    .unwrap_or_else(|| panic!("{leaf}: the probe equals {a_name}'s value"));
                build_with(&t, &a.layout)
            };
            let changes = atlas_diff(&a.atlas, &spliced_atlas);
            problems.extend(check_splice(&m, leaf, row, a, &changes, &ctx, &mut moved));
            splices += 1;
        }
        if splices == 0 {
            unexercised.push(leaf.clone());
        }
        for ((loc, sink), hit) in moved {
            if !hit {
                problems.push(format!("{leaf} did not move its declared sink {loc}:{sink} under any of its {splices} splices"));
            }
        }
    }
    assert!(
        problems.is_empty(),
        "{} findings:\n{}",
        problems.len(),
        problems.join("\n")
    );
    assert!(
        unexercised.is_empty(),
        "no preset pair differentiates these rows and they have no `probe`; give each one in mapping.toml: {unexercised:?}"
    );
}
