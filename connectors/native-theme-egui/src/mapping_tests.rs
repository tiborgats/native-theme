//! §13 T3 (converse coverage), §5.7's totals and the typed reader of `mapping.toml`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]

use std::collections::{BTreeMap, BTreeSet};

use native_theme::theme::{ColorMode, Theme};

use crate::{ResolvedTheme, Role, RoleVariant, Surface};

/// The crate's one `resolved` (Task 11), re-exported so a module reading the manifest takes
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

/// `tested_by` names a clause of §13's table: `T1` to `T18`, a `b` only on the two tests that
/// have one (T1b, T10b), then an optional `(x)` clause letter.
fn is_clause(s: &str) -> bool {
    let Some(rest) = s.strip_prefix('T') else {
        return false;
    };
    let digits = rest.chars().take_while(char::is_ascii_digit).count();
    let Some(n) = rest.get(..digits).and_then(|d| d.parse::<u8>().ok()) else {
        return false;
    };
    if !(1..=18).contains(&n) {
        return false;
    }
    let rest = rest.get(digits..).unwrap_or("");
    let rest = match rest.strip_prefix('b') {
        Some(after) if n == 1 || n == 10 => after,
        Some(_) => return false,
        None => rest,
    };
    rest.is_empty()
        || (rest.len() == 3
            && rest.starts_with('(')
            && rest.ends_with(')')
            && rest.chars().nth(1).is_some_and(|c| c.is_ascii_lowercase()))
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
        ("buttons", [12, 44, 29, 14]),
        ("inputs", [4, 35, 28, 11]),
        ("indicators", [6, 17, 8, 9]),
        ("chrome", [0, 53, 22, 16]),
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
    assert_eq!(tags.get("egui-limited").copied().unwrap_or_default(), 54);
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
