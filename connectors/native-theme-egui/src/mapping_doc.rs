//! The mapping document (spec §5.7, §5.9): `src/mapping.md`, rendered from `mapping.toml`,
//! so the rows, the totals and the base-owner table the rustdoc shows cannot drift from the
//! manifest. `mapping_md_is_current` fails when the committed file differs from the render;
//! `regenerate_mapping_md` (ignored) rewrites it.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::mapping_tests::{Manifest, Sink, Verdict, below, preset_reads, surface_by_key};

const CURRENT: &str = include_str!("mapping.md");
const REGENERATE: &str = "cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md";

/// §5's six groups, by a leaf's first path segment (§5.1–§5.6), in §5.7's order.
const GROUPS: &[(&str, &[&str])] = &[
    (
        "Foundation (`defaults`, `text_scale`, `layout`)",
        &["defaults", "text_scale", "layout"],
    ),
    (
        "Surfaces (window, dialog, popover, card, tooltip, menu)",
        &["window", "dialog", "popover", "card", "tooltip", "menu"],
    ),
    (
        "Buttons (button, link, switch, checkbox, segmented control)",
        &["button", "link", "switch", "checkbox", "segmented_control"],
    ),
    (
        "Inputs (input, text area, combo box, list)",
        &["input", "text_area", "combo_box", "list"],
    ),
    (
        "Indicators (scrollbar, slider, progress bar, splitter, separator, spinner)",
        &[
            "scrollbar",
            "slider",
            "progress_bar",
            "splitter",
            "separator",
            "spinner",
        ],
    ),
    (
        "Chrome (tab, sidebar, toolbar, status bar, expander)",
        &["tab", "sidebar", "toolbar", "status_bar", "expander"],
    ),
];
/// §2's four verdicts, in §5.7's column order.
const VERDICTS: [Verdict; 4] = [
    Verdict::Direct,
    Verdict::Scoped,
    Verdict::Derived,
    Verdict::Unmappable,
];

fn verdict_name(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Direct => "DIRECT",
        Verdict::Scoped => "SCOPED",
        Verdict::Derived => "DERIVED",
        Verdict::Unmappable => "UNMAPPABLE",
    }
}

fn group_of(leaf: &str) -> usize {
    let head = leaf.split('.').next().unwrap_or("");
    GROUPS
        .iter()
        .position(|(_, heads)| heads.contains(&head))
        .unwrap_or_else(|| panic!("leaf `{leaf}` belongs to no §5 group; extend GROUPS"))
}

/// Where a sink lives (§13.1): nothing for a base-style field, «scope» or «scope:variant» for a
/// role cell's field, \[surface\] — escaped, as rustdoc would read `[surface]` as a link — for a
/// `Frame`'s.
fn sink_location(sink: &Sink) -> String {
    match (&sink.scope, &sink.surface) {
        (Some(scope), _) => match &sink.variant {
            Some(variant) => format!("«{scope}:{variant}»"),
            None => format!("«{scope}»"),
        },
        (None, Some(surface)) => format!("\\[{surface}\\]"),
        (None, None) => String::new(),
    }
}

/// Free text from `mapping.toml` in a table cell: outside `code` spans, `[`, `]`, `<` and `>`
/// are escaped, since rustdoc would read a link or an HTML tag; `|` is escaped everywhere,
/// since a GFM table splits a row on it even inside a code span.
fn md_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_code = false;
    for c in s.chars() {
        match c {
            '`' => {
                in_code = !in_code;
                out.push(c);
            }
            '|' => out.push_str("\\|"),
            '[' | ']' | '<' | '>' if !in_code => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

/// One sink as the document spells it: its location, its path, and its `when`.
fn sink_text(sink: &Sink) -> String {
    let location = sink_location(sink);
    let mut out = if location.is_empty() {
        format!("`{}`", sink.path)
    } else {
        format!("{location} `{}`", sink.path)
    };
    if let Some(when) = &sink.when {
        let _ = write!(out, " (when: {})", md_text(when));
    }
    out
}

/// A row of the base-owner table: a `Style` field, or a `Frame` field no preset reads from
/// the `Style` (§3.2), keyed by its surface. `Style` rows sort first.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum FieldKey {
    Style(String),
    Frame(String, String),
}

/// Who writes one field. Owners: the base-style sinks with no `when`, and those whose `when`
/// is "stated; …" — the leaf holds the field whenever it is stated (§6.4); a DERIVED row's
/// sink beside a non-DERIVED owner is a fold into that owner's value (§6.13), so an input.
/// Inputs: every other `when` sink, a formula's other input or a fallback (§13.1). Forced:
/// `when = "never: …"`. Displaced: the scoped and frame sinks.
#[derive(Default)]
struct Field {
    /// The leaf, and whether its row is DERIVED.
    owners: Vec<(String, bool)>,
    inputs: Vec<String>,
    forced: Vec<String>,
    displaced: Vec<String>,
}

/// A `Frame` sink's row: the `Style` field its surface's preset reads for that frame field,
/// or the frame field itself.
fn frame_field(surface: &str, path: &str) -> FieldKey {
    preset_reads(surface_by_key(surface))
        .iter()
        .find_map(|(frame, style)| below(frame, path).map(|rest| format!("{style}{rest}")))
        .map_or_else(
            || FieldKey::Frame(surface.to_owned(), path.to_owned()),
            FieldKey::Style,
        )
}

pub(crate) fn render() -> String {
    let manifest = Manifest::load();

    let mut md = String::new();
    let _ = writeln!(md, "# The mapping\n");
    let _ = writeln!(
        md,
        "<!-- Generated from `mapping.toml` by `mapping_doc::render`; regenerate with `{REGENERATE}`. Do not edit. -->\n"
    );
    let _ = writeln!(
        md,
        "One row per native leaf (spec §5, §13.1). A sink is a base-style field; «scope» or «scope:variant» a role cell's field; \\[surface\\] a `Frame`'s.\n"
    );

    // ---- §5.7 totals, recomputed from the rows ---------------------------------------
    let mut totals = vec![[0usize; 5]; GROUPS.len() + 1]; // per group + TOTAL: leaves, then the four verdicts
    let mut tags: BTreeMap<&str, usize> = BTreeMap::new();
    for (leaf, row) in &manifest.rows {
        let v = VERDICTS
            .iter()
            .position(|x| *x == row.verdict)
            .expect("one of the four verdicts");
        let g = group_of(leaf);
        for t in [g, GROUPS.len()] {
            totals[t][0] += 1;
            totals[t][v + 1] += 1;
        }
        if row.verdict == Verdict::Unmappable {
            let tag = row
                .sub_tag
                .as_deref()
                .unwrap_or_else(|| panic!("row `{leaf}`: unmappable without sub_tag"));
            *tags.entry(tag).or_default() += 1;
        }
    }
    let _ = writeln!(
        md,
        "## Totals\n\n| group | leaves | DIRECT | SCOPED | DERIVED | UNMAPPABLE |\n|---|---:|---:|---:|---:|---:|"
    );
    for (g, (title, _)) in GROUPS.iter().enumerate() {
        let t = totals[g];
        let _ = writeln!(
            md,
            "| {title} | {} | {} | {} | {} | {} |",
            t[0], t[1], t[2], t[3], t[4]
        );
    }
    let t = totals[GROUPS.len()];
    let _ = writeln!(
        md,
        "| **TOTAL** | **{}** | **{}** | **{}** | **{}** | **{}** |\n",
        t[0], t[1], t[2], t[3], t[4]
    );
    let tag_text: Vec<String> = tags.iter().map(|(tag, n)| format!("`{tag}` {n}")).collect();
    let _ = writeln!(md, "UNMAPPABLE sub-tags (§2): {}.\n", tag_text.join(", "));

    // ---- the rows, by §5's groups --------------------------------------------------------
    let _ = writeln!(md, "## Rows\n");
    for (g, (title, _)) in GROUPS.iter().enumerate() {
        let _ = writeln!(
            md,
            "### {title}\n\n| leaf | verdict | sinks, or the test of its route | note |\n|---|---|---|---|"
        );
        for (leaf, row) in manifest.rows.iter().filter(|(leaf, _)| group_of(leaf) == g) {
            let mut route: Vec<String> = row.sinks.iter().map(sink_text).collect();
            if let Some(test) = &row.tested_by {
                route.push(format!("→ {test}"));
            }
            let mut note = Vec::new();
            if row.verdict == Verdict::Unmappable {
                let tag = row.sub_tag.as_deref().expect("checked above");
                let upstream = row
                    .upstream
                    .as_deref()
                    .unwrap_or_else(|| panic!("row `{leaf}`: unmappable without upstream"));
                note.push(format!("`{tag}`: {}", md_text(upstream)));
            }
            if let Some(probe) = &row.probe {
                note.push(format!("probe `{probe}`"));
            }
            for (preset, reason) in &row.exceptions {
                note.push(format!("exception on `{preset}`: {}", md_text(reason)));
            }
            let _ = writeln!(
                md,
                "| `{leaf}` | {} | {} | {} |",
                verdict_name(row.verdict),
                route.join("; "),
                note.join("; ")
            );
        }
        let _ = writeln!(md);
    }

    // ---- §5.9: base owners and what displaces them, from the sink locations ----------------
    let mut fields: BTreeMap<FieldKey, Field> = BTreeMap::new();
    for (leaf, row) in &manifest.rows {
        for sink in &row.sinks {
            match (&sink.scope, &sink.surface) {
                (None, None) => {
                    let field = fields
                        .entry(FieldKey::Style(sink.path.clone()))
                        .or_default();
                    let derived = row.verdict == Verdict::Derived;
                    match sink.when.as_deref() {
                        None => field.owners.push((format!("`{leaf}`"), derived)),
                        Some(when) if when.starts_with("stated; ") => {
                            field.owners.push((format!("`{leaf}`"), derived));
                        }
                        Some(when) => match when.strip_prefix("never: ") {
                            Some(why) => field
                                .forced
                                .push(format!("`{leaf}` forces it: {}", md_text(why))),
                            None => field.inputs.push(format!("`{leaf}`")),
                        },
                    }
                }
                (_, surface) => {
                    let key = match surface {
                        Some(surface) if sink.scope.is_none() => frame_field(surface, &sink.path),
                        _ => FieldKey::Style(sink.path.clone()),
                    };
                    fields
                        .entry(key)
                        .or_default()
                        .displaced
                        .push(format!("`{leaf}` {}", sink_location(sink)));
                }
            }
        }
    }
    let _ = writeln!(
        md,
        "## Base owners (§5.9)\n\nThe leaf that wins an egui field globally; the leaves that only feed its formula (a fold, a composite, a fallback), or force it to a constant; and the leaves that displace it inside a role scope or a `Surface` frame. A frame's field is listed under the `Style` field its egui preset reads (`egui/src/containers/frame.rs:178-221`), or by itself where the preset reads none.\n\n| egui field | base owner | formula inputs and forced constants | displaced to a scope or a surface |\n|---|---|---|---|"
    );
    for field in fields.values_mut() {
        if field.owners.iter().any(|(_, derived)| !derived) {
            let (owners, folds): (Vec<_>, Vec<_>) =
                field.owners.drain(..).partition(|(_, derived)| !derived);
            field.owners = owners;
            field.inputs.extend(folds.into_iter().map(|(leaf, _)| leaf));
            field.inputs.sort_unstable();
        }
    }
    for (key, field) in &fields {
        let (name, unowned) = match key {
            FieldKey::Style(path) => (format!("`{path}`"), "— (egui's own value stands)"),
            FieldKey::Frame(surface, path) => (
                format!("\\[{surface}\\] `{path}`"),
                "— (the `Frame` preset's own value)",
            ),
        };
        let owner = if !field.owners.is_empty() {
            let owners: Vec<&str> = field.owners.iter().map(|(leaf, _)| leaf.as_str()).collect();
            owners.join(", ")
        } else if !field.forced.is_empty() {
            "— (a constant this crate writes)".to_owned()
        } else if !field.inputs.is_empty() {
            "— (a formula's result)".to_owned()
        } else {
            unowned.to_owned()
        };
        let mut formula = field.inputs.clone();
        formula.extend(field.forced.iter().cloned());
        let _ = writeln!(
            md,
            "| {name} | {owner} | {} | {} |",
            formula.join(", "),
            field.displaced.join(", ")
        );
    }
    let _ = writeln!(
        md,
        "\n## Left to egui (`[unwritten]`)\n\n| egui field | why egui's own value stands |\n|---|---|"
    );
    for (field, why) in &manifest.unwritten {
        let _ = writeln!(md, "| `{field}` | {} |", md_text(why));
    }
    md
}

#[test]
fn mapping_md_is_current() {
    let want = render();
    assert!(
        CURRENT == want,
        "src/mapping.md differs from the render of mapping.toml; run `{REGENERATE}`"
    );
}

#[test]
#[ignore = "rewrites src/mapping.md; run on purpose"]
fn regenerate_mapping_md() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/mapping.md");
    std::fs::write(path, render()).expect("write src/mapping.md");
}
