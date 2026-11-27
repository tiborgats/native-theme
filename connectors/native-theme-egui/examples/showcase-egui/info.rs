//! Widget Info, generated from `mapping.toml` (spec §10.4, §13.1), on the 0.8.0 model: a row
//! per home, its value read from the installed 0.8.0 theme, its route from the `mapping.toml`
//! row that holds the home (`homes`).

use crate::app::OldAtlas;
use native_theme::next::{Color, ResolvedTheme, WidgetState};
use native_theme_showcase_support::info::ListedInfo;
use native_theme_showcase_support::next::values::{StatedFont, home_value};
use native_theme_showcase_support::next::{
    info as next_info, status as next_status, values as next_values,
};

use crate::demo::{Seam, Shown};

/// One row of Widget Info's "Theme" section, the shared crate's: the leaf, its value, a colour
/// value's colour, and how egui applies it.
pub(crate) use native_theme_showcase_support::info::InfoRow;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Verdict {
    Direct,
    Scoped,
    Derived,
    Unmappable,
}

impl Verdict {
    fn name(self) -> &'static str {
        match self {
            Verdict::Direct => "direct",
            Verdict::Scoped => "scoped",
            Verdict::Derived => "derived",
            Verdict::Unmappable => "unmappable",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Sink {
    pub path: String,
    pub scope: Option<String>,
    pub variant: Option<String>,
    pub surface: Option<String>,
    pub when: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Row {
    pub leaf: String,
    /// Where the leaf's value lives on the 0.8.0 model, the row's `homes`, in its order.
    pub homes: Vec<String>,
    /// The row's `homes_by_preset`: a preset and the homes that stand for [`Row::homes`] on it.
    pub homes_by_preset: Vec<(String, Vec<String>)>,
    pub verdict: Verdict,
    pub sinks: Vec<Sink>,
    pub tested_by: Option<String>,
    /// What checks the route a `tested_by` row names, in words a reader can follow
    /// ([`checked_by`]).
    pub checked_by: Option<String>,
    pub sub_tag: Option<String>,
    pub upstream: Option<String>,
    pub exceptions: Vec<(String, String)>,
    /// A fact about the leaf's route its sinks cannot carry (an egui limit, a reader, egui's own
    /// value where the theme states none), which Widget Info shows after the route.
    pub note: Option<String>,
}

impl Row {
    /// The row's homes on `preset`: the ones its `homes_by_preset` names for it, else
    /// [`Row::homes`].
    pub(crate) fn homes_on(&self, preset: &str) -> &[String] {
        self.homes_by_preset
            .iter()
            .find(|(p, _)| p == preset)
            .map_or(&self.homes, |(_, homes)| homes)
    }

    /// Whether the row holds `home`, on any preset.
    fn holds(&self, home: &str) -> bool {
        self.homes.iter().any(|h| h == home)
            || self
                .homes_by_preset
                .iter()
                .any(|(_, homes)| homes.iter().any(|h| h == home))
    }

    /// Whether the row holds `home` or a field of it, a text style's (`button.label(NONE).size_px`
    /// of `button.label(NONE)`).
    fn holds_within(&self, home: &str) -> bool {
        let fields = format!("{home}.");
        self.holds(home)
            || self
                .homes
                .iter()
                .chain(self.homes_by_preset.iter().flat_map(|(_, homes)| homes))
                .any(|h| h.starts_with(&fields))
    }
}

/// The connector's test file whose `#[test]` functions hold the §13 clauses `mapping.toml`'s
/// `tested_by` names, and its path in the connector.
const CONNECTOR_TESTS: (&str, &str) = (
    include_str!("../../src/install_tests.rs"),
    "src/install_tests.rs",
);

/// What checks a per-call route whose `mapping.toml` row names `clause` in `tested_by`, a test
/// clause of the archived egui connector spec's §13 table (`T18(a)`;
/// `docs/archive/todo_v0.6.0_egui-connector-spec.md`): "checked by native-theme-egui's test
/// <name> (src/install_tests.rs)", naming the `#[test]` functions of the connector's
/// `src/install_tests.rs` that hold the clause, found by it as the file names them, a function
/// name that begins with the clause in lower case (`t18_a_`) or a doc comment whose first line
/// begins with it (`T14 (b)`). Where the file holds none, the clause and the spec that
/// defines it.
pub(crate) fn checked_by(clause: &str) -> String {
    let (source, path) = CONNECTOR_TESTS;
    let parts = clause
        .strip_prefix('T')
        .and_then(|rest| rest.strip_suffix(')'))
        .and_then(|rest| rest.split_once('('));
    let names = parts.map_or_else(Vec::new, |(number, letter)| {
        clause_tests(
            source,
            &format!("t{number}_{letter}_"),
            &format!("T{number} ({letter})"),
        )
    });
    match names.as_slice() {
        [] => format!(
            "checked by the egui connector spec's §13 test {clause} \
             (docs/archive/todo_v0.6.0_egui-connector-spec.md)"
        ),
        [one] => format!("checked by native-theme-egui's test {one} ({path})"),
        [first @ .., last] => format!(
            "checked by native-theme-egui's tests {} and {last} ({path})",
            first.join(", ")
        ),
    }
}

/// The `#[test]` functions of `source` whose name begins with `prefix` or whose doc comment's
/// first line begins with `doc`, in the file's order.
fn clause_tests<'a>(source: &'a str, prefix: &str, doc: &str) -> Vec<&'a str> {
    let mut names = Vec::new();
    let (mut first_doc, mut in_doc, mut test) = (None, false, false);
    for line in source.lines().map(str::trim) {
        if let Some(text) = line.strip_prefix("///") {
            if !in_doc {
                first_doc = Some(text.trim_start());
            }
            in_doc = true;
            continue;
        }
        in_doc = false;
        if line == "#[test]" {
            test = true;
        } else if line.starts_with("#[") {
        } else if let Some(rest) = line.strip_prefix("fn ") {
            let name = rest.split(['(', '<']).next().unwrap_or_default();
            if test && (name.starts_with(prefix) || first_doc.is_some_and(|d| d.starts_with(doc))) {
                names.push(name);
            }
            (first_doc, test) = (None, false);
        } else {
            (first_doc, test) = (None, false);
        }
    }
    names
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Manifest {
    pub rows: Vec<Row>,
    pub unwritten: Vec<(String, String)>,
}

impl Manifest {
    /// The embedded `mapping.toml`; a parse failure is shown in the inspector, never a panic.
    pub(crate) fn parse(text: &str) -> Result<Manifest, String> {
        let table: toml::Table = toml::from_str(text).map_err(|e| e.to_string())?;
        let mut rows = Vec::new();
        let mut unwritten = Vec::new();
        for (key, value) in table {
            let Some(entry) = value.as_table() else {
                return Err(format!("{key}: not a table"));
            };
            if key == "unwritten" {
                for (path, reason) in entry {
                    unwritten.push((
                        path.clone(),
                        reason.as_str().unwrap_or_default().to_string(),
                    ));
                }
                continue;
            }
            let verdict = match entry.get("verdict").and_then(toml::Value::as_str) {
                Some("direct") => Verdict::Direct,
                Some("scoped") => Verdict::Scoped,
                Some("derived") => Verdict::Derived,
                Some("unmappable") => Verdict::Unmappable,
                other => return Err(format!("{key}: verdict {other:?}")),
            };
            let string = |name: &str| {
                entry
                    .get(name)
                    .and_then(toml::Value::as_str)
                    .map(str::to_string)
            };
            let strings = |value: Option<&toml::Value>| -> Vec<String> {
                value
                    .and_then(toml::Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(toml::Value::as_str)
                    .map(str::to_string)
                    .collect()
            };
            let sinks = entry
                .get("sinks")
                .and_then(toml::Value::as_array)
                .map(|sinks| {
                    sinks
                        .iter()
                        .filter_map(toml::Value::as_table)
                        .map(|s| {
                            let field = |name: &str| {
                                s.get(name)
                                    .and_then(toml::Value::as_str)
                                    .map(str::to_string)
                            };
                            Sink {
                                path: field("path").unwrap_or_default(),
                                scope: field("scope"),
                                variant: field("variant"),
                                surface: field("surface"),
                                when: field("when"),
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();
            let exceptions = entry
                .get("exceptions")
                .and_then(toml::Value::as_array)
                .map(|list| {
                    list.iter()
                        .filter_map(toml::Value::as_array)
                        .map(|pair| {
                            (
                                pair.first()
                                    .and_then(toml::Value::as_str)
                                    .unwrap_or_default()
                                    .to_string(),
                                pair.get(1)
                                    .and_then(toml::Value::as_str)
                                    .unwrap_or_default()
                                    .to_string(),
                            )
                        })
                        .collect()
                })
                .unwrap_or_default();
            let tested_by = string("tested_by");
            let homes_by_preset = entry
                .get("homes_by_preset")
                .and_then(toml::Value::as_table)
                .map(|by| {
                    by.iter()
                        .map(|(preset, homes)| (preset.clone(), strings(Some(homes))))
                        .collect()
                })
                .unwrap_or_default();
            rows.push(Row {
                leaf: key,
                homes: strings(entry.get("homes")),
                homes_by_preset,
                verdict,
                sinks,
                checked_by: tested_by.as_deref().map(checked_by),
                tested_by,
                sub_tag: string("sub_tag"),
                upstream: string("upstream"),
                exceptions,
                note: string("note"),
            });
        }
        rows.sort_by(|a, b| a.leaf.cmp(&b.leaf));
        Ok(Manifest { rows, unwritten })
    }

    fn base_rows(&self) -> impl Iterator<Item = &Row> {
        self.rows.iter().filter(|r| {
            r.sinks
                .iter()
                .any(|s| s.scope.is_none() && s.surface.is_none())
                || (r.sinks.is_empty()
                    && ["defaults.", "text_scale.", "layout."]
                        .iter()
                        .any(|p| r.leaf.starts_with(p)))
        })
    }

    /// §10.4's rule: a role lists its own leaves, every row scoped to it, and the
    /// base style's rows (a cell carries what its role does not write, §3.4); a
    /// surface lists the rows with a sink on it and every row of the native
    /// widgets those leaves belong to; the base style lists the base owners and
    /// the sinkless `defaults.`, `text_scale.` and `layout.` rows.
    pub(crate) fn rows_for(&self, seam: &Seam) -> Vec<&Row> {
        let mut out: Vec<&Row> = match seam {
            Seam::Base => self.base_rows().collect(),
            Seam::Role(role, _) => {
                let key = role.key();
                let prefix = format!("{key}.");
                self.rows
                    .iter()
                    .filter(|r| {
                        r.leaf.starts_with(&prefix)
                            || r.sinks.iter().any(|s| s.scope.as_deref() == Some(key))
                    })
                    .chain(self.base_rows())
                    .collect()
            }
            Seam::Surface(surface) => {
                let key = surface.key();
                let widgets: Vec<&str> = self
                    .rows
                    .iter()
                    .filter(|r| r.sinks.iter().any(|s| s.surface.as_deref() == Some(key)))
                    .filter_map(|r| r.leaf.split('.').next())
                    .collect();
                self.rows
                    .iter()
                    .filter(|r| {
                        r.leaf
                            .split('.')
                            .next()
                            .is_some_and(|w| widgets.contains(&w))
                    })
                    .collect()
            }
        };
        out.sort_by(|a, b| a.leaf.cmp(&b.leaf));
        out.dedup_by(|a, b| a.leaf == b.leaf);
        out
    }
}

/// A 0.8.0 resolved theme as Widget Info and the Theme Map read it: the theme, and its
/// serialised form (`next::values::theme_json`), which `next::values::home_value` walks beside
/// it.
#[derive(Clone, Debug)]
pub(crate) struct InfoTheme {
    pub theme: ResolvedTheme,
    pub json: serde_json::Value,
}

impl InfoTheme {
    /// `theme` with its serialised form.
    pub(crate) fn new(theme: ResolvedTheme) -> Result<Self, String> {
        let json = next_values::theme_json(&theme)?;
        Ok(Self { theme, json })
    }

    /// The value of `home` as Widget Info shows it, each text style's family as `family` labels
    /// it (`next::values::home_value`); a home of no known name as the error that says so.
    pub(crate) fn value(
        &self,
        home: &str,
        family: &dyn Fn(&str, &StatedFont) -> String,
    ) -> (String, Option<Color>) {
        match home_value(home, &self.theme, &self.json, family) {
            Ok(value) => (value.text, value.colour),
            Err(error) => (format!("error: {error}"), None),
        }
    }
}

/// The installed 0.8.0 theme the pages draw in `ctx`, the one Widget Info shows
/// (`OldAtlas::next_shown`: the scheme egui draws, for an inactive window while the viewport is
/// unfocused), with its serialised form; an error where the atlas holds none.
pub(crate) fn info_theme(atlas: &OldAtlas, ctx: &egui::Context) -> Result<InfoTheme, String> {
    let resolved = atlas
        .next_shown(ctx)
        .ok_or("the atlas holds no 0.8.0 theme, which Widget Info reads")?;
    InfoTheme::new(resolved.clone())
}

/// [`info_theme`] for each scheme, for an active and an inactive window, made on first use and
/// kept until the next install: Widget Info and the Theme Map read it on every pass.
#[derive(Default)]
pub(crate) struct JsonCache {
    /// `[light, dark]`, each `[active, inactive]`.
    cells: [[std::cell::OnceCell<Result<InfoTheme, String>>; 2]; 2],
}

impl JsonCache {
    /// [`info_theme`] of the theme the pages draw in `ctx`.
    pub(crate) fn get(&self, atlas: &OldAtlas, ctx: &egui::Context) -> &Result<InfoTheme, String> {
        let [light, dark] = &self.cells;
        let [active, inactive] = match ctx.theme() {
            egui::Theme::Light => light,
            egui::Theme::Dark => dark,
        };
        let cell = if OldAtlas::shows_inactive(ctx) {
            inactive
        } else {
            active
        };
        cell.get_or_init(|| info_theme(atlas, ctx))
    }
}

/// A text style's family as the theme states it, the Theme Map's label of it.
pub(crate) fn as_stated(_: &str, font: &StatedFont) -> String {
    font.family.clone()
}

/// A sink as it prints: `path @scope[/variant]`, `path @surface`, or `path @base style`,
/// its `when` after it in parentheses.
fn sink_text(sink: &Sink) -> String {
    let at = match (&sink.scope, &sink.variant, &sink.surface) {
        (Some(scope), Some(variant), _) => format!("@{scope}/{variant}"),
        (Some(scope), None, _) => format!("@{scope}"),
        (None, _, Some(surface)) => format!("@{surface}"),
        (None, _, None) => "@base style".to_string(),
    };
    match &sink.when {
        Some(when) => format!("{} {at} ({when})", sink.path),
        None => format!("{} {at}", sink.path),
    }
}

/// Whether `sink` writes into the cell or frame `seam` is.
fn sink_on(sink: &Sink, seam: &Seam) -> bool {
    match seam {
        Seam::Base => sink.scope.is_none() && sink.surface.is_none(),
        Seam::Role(role, variant) => {
            sink.scope.as_deref() == Some(role.key())
                && sink.variant.as_deref().unwrap_or("normal") == variant.key()
        }
        Seam::Surface(surface) => sink.surface.as_deref() == Some(surface.key()),
    }
}

/// Where a sink is, in words: the role's scope, the surface's frame, or the base style.
fn seam_place(seam: &Seam) -> String {
    match seam {
        Seam::Base => "base style".to_string(),
        Seam::Role(role, variant) => match variant.key() {
            "normal" => format!("Role::{role:?} scope"),
            key => format!("Role::{role:?} ({key}) scope"),
        },
        Seam::Surface(surface) => format!("Surface::{surface:?} frame"),
    }
}

/// How egui applies one `mapping.toml` row to an instance drawn with `seams`.
enum Route {
    /// A style or frame field: where, and the field.
    Sink(String),
    /// No `Style` or `Frame` field: the call site applies it, as the named tests check
    /// ([`checked_by`]).
    PerCall(String),
    /// The companion crate paints it.
    Widgets(String),
    /// Not reachable: why.
    Lost(String),
}

/// The `visuals.widgets` entry egui draws a home's state in, where its state names one: an open
/// state the `open` entry, a press the `active` one, a hover the `hovered` one, in that order
/// (an open combo box under the pointer is drawn open); `None` for a home of no such state.
fn widget_entry(home: &str) -> Option<&'static str> {
    let (_, query) = home.split_once('(')?;
    let state = query.split([')', ';']).next()?;
    let flags: Vec<&str> = state.split('|').collect();
    [
        ("OPEN", "open"),
        ("PRESSED", "active"),
        ("HOVER", "hovered"),
    ]
    .into_iter()
    .find(|(flag, _)| flags.contains(flag))
    .map(|(_, entry)| entry)
}

impl Route {
    /// How egui applies `row` to `home`, one of its homes, on an instance drawn with `seams`.
    /// Where the row's homes are drawn in different `visuals.widgets` entries
    /// ([`widget_entry`]), only the sinks of `home`'s entry are its route.
    fn of(row: &Row, home: &str, seams: &[Seam]) -> Self {
        if row.verdict == Verdict::Unmappable {
            let upstream = row.upstream.as_deref().unwrap_or_default();
            if let Some(widget) = upstream.strip_prefix("native-theme-egui-widgets: ") {
                let widget = widget.split(';').next().unwrap_or(widget);
                return Route::Widgets(widget.to_string());
            }
            let why = if let Some(rest) = upstream.strip_prefix("none: ") {
                rest.to_string()
            } else if let Some(rest) = upstream.strip_prefix("egui: ") {
                format!("egui lacks {rest}")
            } else if let Some(rest) = upstream.strip_prefix("native-theme: ") {
                format!("native-theme lacks {rest}")
            } else {
                format!("egui lacks {upstream}")
            };
            return Route::Lost(why);
        }
        if let Some(check) = &row.checked_by {
            return Route::PerCall(check.clone());
        }
        let mut entries: Vec<&str> = row.homes.iter().filter_map(|h| widget_entry(h)).collect();
        entries.sort_unstable();
        entries.dedup();
        let sinks: Vec<&Sink> = match widget_entry(home) {
            Some(entry) if entries.len() > 1 => {
                let infix = format!(".widgets.{entry}.");
                let own: Vec<&Sink> = row
                    .sinks
                    .iter()
                    .filter(|s| s.path.contains(&infix))
                    .collect();
                if own.is_empty() {
                    row.sinks.iter().collect()
                } else {
                    own
                }
            }
            _ => row.sinks.iter().collect(),
        };
        // The instance's own cells and frames first; a role's cell carries what the role does
        // not write from the base style (§3.4).
        let mut on: Vec<(&Seam, &Sink)> = seams
            .iter()
            .flat_map(|seam| {
                sinks
                    .iter()
                    .copied()
                    .filter(move |s| sink_on(s, seam))
                    .map(move |s| (seam, s))
            })
            .collect();
        if on.is_empty() {
            on = sinks
                .iter()
                .copied()
                .filter(|s| sink_on(s, &Seam::Base))
                .map(|s| (&Seam::Base, s))
                .collect();
        }
        // An unconditional sink before one written only under a condition.
        on.sort_by_key(|(_, s)| s.when.is_some());
        match on.first() {
            Some((seam, sink)) => {
                let more = match on.len().saturating_sub(1) {
                    0 => String::new(),
                    n => format!(" (+{n} more)"),
                };
                Route::Sink(format!("{}, {}{more}", seam_place(seam), sink.path))
            }
            None => match sinks.first() {
                Some(sink) => Route::Sink(sink_text(sink)),
                None => Route::PerCall("no test named".to_string()),
            },
        }
    }

    /// The route in full, as the line under a row reads.
    fn line(&self) -> String {
        match self {
            Route::Sink(sink) => format!("egui: {sink}"),
            Route::PerCall(check) => format!("egui: per call, no Style field; {check}"),
            Route::Widgets(widget) => format!("egui: native-theme-egui-widgets {widget} paints it"),
            Route::Lost(why) => format!("not reachable: {why}"),
        }
    }

    /// The route in a word or two, for a part of a whole font or text-scale entry.
    fn short(&self) -> String {
        match self {
            Route::Sink(sink) => sink.clone(),
            Route::PerCall(_) => "per call".to_string(),
            Route::Widgets(_) => "native-theme-egui-widgets".to_string(),
            Route::Lost(_) => "not reachable".to_string(),
        }
    }
}

/// The line under a row: how egui applies `home` to an instance drawn with `seams`, from the
/// first `mapping.toml` row that holds it (`homes`, in the rows' leaf order); a text style named
/// whole by its size's route, then the fields whose route differs; the icon homes, which no
/// `Style` field carries, by the showcase's loader.
pub(crate) fn how_line(manifest: &Manifest, home: &str, seams: &[Seam]) -> String {
    how_line_with(manifest, home, seams, &[])
}

/// [`how_line`], where a text style named whole has fields the showcase applies per call
/// (`per_call`: the field's home, and its route after "per call, "), each such field by that
/// route in place of its `mapping.toml` row's.
fn how_line_with(
    manifest: &Manifest,
    home: &str,
    seams: &[Seam],
    per_call: &[(&str, &str)],
) -> String {
    if matches!(home, "icons.set" | "icons.theme") {
        return "egui: the showcase loads its icons from it (demo::role_image)".to_string();
    }
    if let Some(row) = manifest.rows.iter().find(|r| r.holds(home)) {
        return Route::of(row, home, seams).line();
    }
    let part = |name: &'static str| {
        let field = format!("{home}.{name}");
        if let Some((_, how)) = per_call.iter().find(|(h, _)| *h == field) {
            let short = format!("per call, {how}");
            return Some((name, format!("egui: {short}"), short));
        }
        manifest.rows.iter().find(|r| r.holds(&field)).map(|row| {
            let route = Route::of(row, &field, seams);
            (name, route.line(), route.short())
        })
    };
    let mut parts = ["size_px", "family", "weight", "style", "line_height"]
        .into_iter()
        .filter_map(part);
    let Some((_, main, main_short)) = parts.next() else {
        return "not reachable: no mapping.toml row holds it".to_string();
    };
    let mut rest: Vec<(String, Vec<&str>)> = Vec::new();
    for (name, _, short) in parts {
        if short == main_short {
            continue;
        }
        match rest.iter_mut().find(|(s, _)| *s == short) {
            Some((_, names)) => names.push(name),
            None => rest.push((short, vec![name])),
        }
    }
    let mut line = main;
    for (short, names) in rest {
        line.push_str(&format!("; {}: {short}", names.join(", ")));
    }
    line
}

/// The route of `layout.border_width` on a list row or frame: the hovered row's outline takes
/// its width from the highlight, per call in `pages::basic::RowLook::of`.
const HOVERED_ROW_EDGE: &str = "egui: per call, not this: the hovered row's outline is \
     list.highlight(HOVER).stroke, its width included (pages::basic::RowLook::of)";
/// The route of a Basic table home that only a hovered row would draw.
const TABLE_ROW_NO_HOVER: &str =
    "not reachable: the Basic table's rows take no hover (pages::basic::table)";

/// The route of a selected list or table row's `colors.selection.unfocused_background`, per call
/// in `pages::basic::RowLook::of`.
const SELECTED_ROW_UNFOCUSED_FILL: &str = "egui: per call, the selected row's highlight while \
     the window is not focused, list.highlight's query with UNFOCUSED_VIEW where a rule states \
     it for that state (pages::basic::RowLook::of)";
/// The route of a selected list or table row's `colors.selection.unfocused_foreground`, as
/// [`SELECTED_ROW_UNFOCUSED_FILL`].
const SELECTED_ROW_UNFOCUSED_TEXT: &str = "egui: per call, the selected row's text colour while \
     the window is not focused, list.item_label's query with UNFOCUSED_VIEW where a rule states \
     it for that state (pages::basic::RowLook::of)";

/// The route of the primary button's own outline, which the Basic page hands its button per call.
pub(crate) const PRIMARY_OUTLINE: &str = "egui: per call, Button::stroke, the primary button's outline, \
     button.body(<state>; variant=Primary).stroke, its width and colour, at rest where stated, \
     hovered and pressed where a rule states it for the state, a translucent one laid inside \
     composited over the button's fill; else the button scope's own entry for the state; \
     focused, the scope's own (pages::basic, primary_pen)";
/// The route of the dialog's default button's outline: its own look where the theme gives it
/// one, else the primary button's outline, both per call.
pub(crate) const DIALOG_DEFAULT_OUTLINE: &str = "egui: per call, Button::stroke, in the state of the \
     button's previous response: where a rule states button.body(DEFAULT).fill, the default \
     button's own frame (native-theme-egui next::default_button: button.body(DEFAULT).stroke, \
     hovered and pressed button.body(HOVER).stroke, focused button.body(FOCUS_VISIBLE).stroke \
     where a rule states each, else egui's own); else it is the primary button, filled as it is and outlined in \
     button.body(<state>; variant=Primary).stroke at rest where stated, hovered and pressed \
     where a rule states it for the state, else in the button scope's own entry for the state, \
     which this home gives; focused, the scope's own (pages::basic, primary_pen)";
/// The route of the primary button's own corners: egui reads them nowhere.
pub(crate) const PRIMARY_CORNERS_UNREAD: &str = "not drawn: the primary button is egui's selected Button in \
     the button scope (pages::basic, primary), rounded as the push button, by \
     button.body(NONE).corners";

/// The route of the scroll bar thumb's corners, which the atlas writes as the scroll bar scope's
/// radius, which no `mapping.toml` row holds.
const THUMB_CORNERS: &str = "egui: the scroll bar scope's corner_radius, the thumb's and its \
     trough's, where stated; else layout.corner_radius's (native-theme-egui next/build.rs, \
     thumb_radius; style/derived.rs, scrollbar_radii)";
/// The route of the thumb's corners in a view without the focus.
const UNFOCUSED_THUMB_CORNERS: &str = "not reachable: egui rounds the thumb alike whether its view \
     has the focus or not, by scrollbar.thumb(NONE).corners (native-theme-egui next/build.rs, \
     thumb_radius)";

/// The route of the segmented control's separator colour on a divider: the companion crate's
/// control paints its dividers in it.
const SEGMENT_DIVIDER: &str = "egui: native-theme-egui-widgets SegmentedControl paints its \
     dividers in it where stated, else in its outline's colour, the scope's \
     segmented_control.track(NONE).stroke.color (native-theme-egui-widgets segmented_control.rs, \
     Look::next_of, Outline::paint_lines)";

/// The route of a flat button's hovered outline (the Pages menu button, the Copy box, the
/// side-panel toggle while closed), which `next::flat_button` lays over the scope's hovered
/// entry per call, not the atlas's push button row.
pub(crate) const FLAT_HOVER_EDGE: &str = "egui: per call, the flat button's outline under the pointer, \
     button.body(HOVER; variant=Flat).stroke.color where a rule states it for HOVER, else egui's \
     own treatment of the hover (native-theme-egui next::flat_button; demo::flat)";
/// The route of a flat button's pressed outline, as [`FLAT_HOVER_EDGE`]: egui draws pressed and
/// focused alike, in its one `active` entry.
pub(crate) const FLAT_PRESSED_EDGE: &str = "egui: per call, the flat button's outline pressed or focused, \
     egui's one active entry: button.body(FOCUS_VISIBLE; variant=Flat).stroke.color, else \
     button.body(PRESSED; variant=Flat).stroke.color, where a rule states it for the state, else \
     egui's own treatment of the press (native-theme-egui next::flat_button; demo::flat)";
/// The route of the side-panel toggle's hovered outline: the flat button's while the panel is
/// closed ([`FLAT_HOVER_EDGE`]), the checked state's while it is open.
pub(crate) const TOGGLE_HOVER_EDGE: &str = "egui: per call, the toggle's outline under the pointer: \
     closed, the flat button's, button.body(HOVER; variant=Flat).stroke.color where a rule \
     states it for HOVER, else egui's own treatment of the hover (native-theme-egui \
     next::flat_button; demo::flat); open, button.body(CHECKED|HOVER; variant=Flat).stroke \
     where a rule states button.body(CHECKED; variant=Flat).stroke, or one for \
     CHECKED|HOVER states this one (the query asking CHECKED), else the closed one's \
     (chrome::toggle_checked_pens)";
/// The route of the side-panel toggle's pressed outline, as [`TOGGLE_HOVER_EDGE`]: egui draws
/// pressed and focused alike, in its one `active` entry.
pub(crate) const TOGGLE_PRESSED_EDGE: &str = "egui: per call, the toggle's outline pressed or focused, \
     egui's one active entry: closed, the flat button's, button.body(FOCUS_VISIBLE; \
     variant=Flat).stroke.color, else button.body(PRESSED; variant=Flat).stroke.color, where a \
     rule states it for the state, else egui's own treatment of the press (native-theme-egui \
     next::flat_button; demo::flat); open, button.body(CHECKED|FOCUS_VISIBLE; \
     variant=Flat).stroke where a rule states the flat button's FOCUS_VISIBLE stroke, else \
     button.body(CHECKED|PRESSED; variant=Flat).stroke, where a rule states \
     button.body(CHECKED; variant=Flat).stroke, or one for CHECKED with that state states this \
     one (the query asking CHECKED), else the closed one's (chrome::toggle_checked_pens)";
/// The route of a flat button's hovered outline width, as [`FLAT_HOVER_EDGE`].
pub(crate) const FLAT_HOVER_WIDTH: &str = "egui: per call, the flat button's outline width under the \
     pointer, button.body(HOVER; variant=Flat).stroke.width where a rule states the stroke for \
     HOVER, else egui's own treatment of the hover (native-theme-egui next::flat_button; \
     demo::flat)";
/// The route of a flat button's outline width pressed or focused, as [`FLAT_PRESSED_EDGE`].
pub(crate) const FLAT_PRESSED_WIDTH: &str = "egui: per call, the flat button's outline width \
     pressed or focused, egui's one active entry: button.body(PRESSED; variant=Flat).stroke.width \
     where a rule states the stroke for PRESSED and none states it for FOCUS_VISIBLE, else egui's \
     own treatment of the press (native-theme-egui next::flat_button; demo::flat)";
/// The route of a flat button's focused outline, egui's one `active` entry with the pressed one.
pub(crate) const FLAT_FOCUS_PEN: &str = "egui: per call, the flat button's outline pressed or \
     focused, egui's one active entry: button.body(FOCUS_VISIBLE; variant=Flat).stroke, its \
     colour and width, where a rule states it for FOCUS_VISIBLE, else button.body(PRESSED; \
     variant=Flat).stroke's where one states it, else egui's own treatment of the press \
     (native-theme-egui next::flat_button; demo::flat)";
/// The route of a flat button's fill under the pointer.
pub(crate) const FLAT_HOVER_FILL: &str = "egui: per call, the flat button's fill under the \
     pointer, visuals.widgets.hovered.weak_bg_fill, button.body(HOVER; variant=Flat).fill where a \
     rule states it for HOVER, else egui's own treatment of the hover (native-theme-egui \
     next::flat_button; demo::flat)";
/// The route of a flat button's fill pressed or focused, egui's one `active` entry.
pub(crate) const FLAT_PRESSED_FILL: &str = "egui: per call, the flat button's fill pressed or \
     focused, egui's one active entry, visuals.widgets.active.weak_bg_fill, button.body(PRESSED; \
     variant=Flat).fill where a rule states it for PRESSED, else egui's own treatment of the \
     press (native-theme-egui next::flat_button; demo::flat)";
/// The route of a flat button's label colour under the pointer.
pub(crate) const FLAT_HOVER_LABEL: &str = "egui: per call, the flat button's label colour under \
     the pointer, visuals.widgets.hovered.fg_stroke, button.label(HOVER; variant=Flat).color \
     where a rule states it for HOVER, else egui's own treatment of the hover \
     (native-theme-egui next::flat_button; demo::flat)";
/// The route of a flat button's label colour pressed or focused, egui's one `active` entry.
pub(crate) const FLAT_PRESSED_LABEL: &str = "egui: per call, the flat button's label colour \
     pressed or focused, egui's one active entry, visuals.widgets.active.fg_stroke, \
     button.label(PRESSED; variant=Flat).color where a rule states it for PRESSED, else egui's \
     own treatment of the press (native-theme-egui next::flat_button; demo::flat)";

/// The route of a disabled checked box's own label colour.
const CHECKED_DISABLED_LABEL: &str = "egui: the check box's SelectedDisabled cell's text, \
     visuals.override_text_color, checkbox.label(DISABLED|CHECKED).color where a rule states it \
     for DISABLED by spec §2's two queries, else the Disabled cell's, \
     checkbox.label(DISABLED).color where stated; on a stated disabled fill with no disabled mark \
     stated, the mark too, visuals.widgets.inactive.fg_stroke; faded by the scope's \
     disabled_alpha (native-theme-egui next/build.rs, variant_source; style/variants.rs, \
     selected_disabled_cell; egui-0.37.0 src/ui.rs:523-529, Ui::disable)";

/// The route of a flat button's own label colour at rest, which egui does not read.
pub(crate) const FLAT_REST_LABEL: &str = "not drawn: next::flat_button lays only the hovered, \
     pressed and focused states over the scope; at rest the flat button is lettered in the \
     scope's own text (native-theme-egui next::flat_button; demo::flat)";

/// The route of a flat button's own corners, which egui does not read.
pub(crate) const FLAT_REST_CORNERS: &str = "not drawn: next::flat_button writes no corner \
     radius; the flat button is rounded by the scope's own (native-theme-egui next::flat_button; \
     demo::flat)";

/// The route of the side-panel toggle's own label colour at rest, which egui does not read.
pub(crate) const TOGGLE_REST_LABEL: &str = "not drawn: closed, next::flat_button lays only the \
     hovered, pressed and focused states over the scope, and at rest the toggle is lettered in \
     the scope's own text (native-theme-egui next::flat_button; demo::flat); open, it is egui's \
     selected button (chrome::ghost_button)";

/// The route of the side-panel toggle's own corners, which egui does not read.
pub(crate) const TOGGLE_REST_CORNERS: &str = "not drawn: next::flat_button writes no corner \
     radius; the toggle, closed or open, is rounded by the scope's own (native-theme-egui \
     next::flat_button; demo::flat; chrome::ghost_button)";

/// The homes of a flat button's own states, which `next::flat_button` lays over the scope per
/// call from the installed 0.8.0 theme and no `mapping.toml` row holds, each with its route. Only
/// the flat buttons ([`FLAT_BUTTONS`]) take them; the side-panel toggle's are
/// [`TOGGLE_ROUTES`]'.
pub(crate) const FLAT_BUTTON_ROUTES: [(&str, &str); 12] = [
    ("button.label(NONE; variant=Flat).color", FLAT_REST_LABEL),
    ("button.body(NONE; variant=Flat).corners", FLAT_REST_CORNERS),
    ("button.body(HOVER; variant=Flat).fill", FLAT_HOVER_FILL),
    ("button.body(PRESSED; variant=Flat).fill", FLAT_PRESSED_FILL),
    ("button.label(HOVER; variant=Flat).color", FLAT_HOVER_LABEL),
    (
        "button.label(PRESSED; variant=Flat).color",
        FLAT_PRESSED_LABEL,
    ),
    (
        "button.body(HOVER; variant=Flat).stroke.color",
        FLAT_HOVER_EDGE,
    ),
    (
        "button.body(PRESSED; variant=Flat).stroke.color",
        FLAT_PRESSED_EDGE,
    ),
    (
        "button.body(HOVER; variant=Flat).stroke.width",
        FLAT_HOVER_WIDTH,
    ),
    (
        "button.body(PRESSED; variant=Flat).stroke.width",
        FLAT_PRESSED_WIDTH,
    ),
    (
        "button.body(FOCUS_VISIBLE; variant=Flat).stroke.color",
        FLAT_FOCUS_PEN,
    ),
    (
        "button.body(FOCUS_VISIBLE; variant=Flat).stroke.width",
        FLAT_FOCUS_PEN,
    ),
];

/// The flat buttons drawn in `demo::flat` alone, by id, whose elements take
/// [`FLAT_BUTTON_ROUTES`]: the Pages menu button and Widget Info's Copy.
const FLAT_BUTTONS: [&str; 2] = ["chrome.page_tabs.menu", "chrome.info.copy"];

/// The tool buttons, by id, whose elements and icons' elements take [`TOOL_BUTTON_ROUTES`].
const TOOL_BUTTONS: [&str; 8] = [
    "chrome.toolbar.command_palette",
    "chrome.toolbar.reload_theme",
    "chrome.toolbar.preferences",
    "basic.icons.copy",
    "basic.icons.paste",
    "basic.icons.delete",
    "basic.icons.edit",
    "basic.icons.save",
];

/// The side-panel toggle, whose element and its icon's take [`TOGGLE_ROUTES`].
const TOGGLE: &str = "chrome.status_bar.toggle";

/// Whether `element` is the button `id` or its icon.
fn button_or_icon(element: &str, id: &str) -> bool {
    element == id || element.strip_suffix(".icon") == Some(id)
}

/// The route of a tool button's hovered and pressed outline width, from the toolbar scope.
pub(crate) const TOOL_HOVER_WIDTH: &str = "egui: the toolbar scope's hovered entry, \
     visuals.widgets.hovered.bg_stroke's width, where a rule states toolbar.button(HOVER).stroke \
     for HOVER, else egui's own treatment of the hover (native-theme-egui next/build.rs, \
     ToolbarCell button_border; style/roles.rs, toolbar)";
/// As [`TOOL_HOVER_WIDTH`], pressed: egui draws pressed and focused alike, in its `active` entry.
pub(crate) const TOOL_PRESSED_WIDTH: &str = "egui: the toolbar scope's active entry, pressed or \
     keyboard-focused, visuals.widgets.active.bg_stroke's width, where a rule states \
     toolbar.button(PRESSED).stroke for PRESSED, else egui's own treatment of the press \
     (native-theme-egui next/build.rs, ToolbarCell button_border; style/roles.rs, toolbar)";
/// The route of a tool button's hovered label, from the toolbar scope.
pub(crate) const TOOL_HOVER_LABEL: &str = "egui: the toolbar scope's hovered entry, \
     visuals.widgets.hovered.fg_stroke, where a rule states it for HOVER, else egui's own \
     treatment of the hover (native-theme-egui next/build.rs, ToolbarCell button_text; \
     style/roles.rs, panel): the label's colour where the set has no icon for the button; an \
     icon keeps the colour it loads in";
/// As [`TOOL_HOVER_LABEL`], pressed.
pub(crate) const TOOL_PRESSED_LABEL: &str = "egui: the toolbar scope's active entry, pressed or \
     keyboard-focused, visuals.widgets.active.fg_stroke, where a rule states it for PRESSED, else \
     egui's own treatment of the press (native-theme-egui next/build.rs, ToolbarCell \
     button_text; style/roles.rs, panel): the label's colour where the set has no icon for the \
     button; an icon keeps the colour it loads in";
/// The route of a tool button's own outline at rest, which egui's toolbar scope reads none of.
pub(crate) const TOOL_REST_EDGE: &str = "not drawn: egui's toolbar scope reads no outline of a tool button \
     at rest, only its rest capsule's fill and corners (native-theme-egui next/build.rs, \
     ToolbarCell rest); a stated hovered or pressed outline is drawn at its own width";

/// The homes of a toolbar button the toolbar scope or the showcase applies from the installed
/// 0.8.0 theme, which no `mapping.toml` row holds, each with its route: the rest capsule, the
/// hovered and pressed outline widths and labels, the edit switch's checked looks
/// (`pages::basic::edit_look_at`) and the icon's colour (`demo::tool_icon_color`). Only the
/// tool buttons and their icons ([`TOOL_BUTTONS`]) take them.
pub(crate) const TOOL_BUTTON_ROUTES: [(&str, &str); 19] = [
    ("toolbar.button(NONE).stroke.color", TOOL_REST_EDGE),
    ("toolbar.button(NONE).stroke.width", TOOL_REST_EDGE),
    ("toolbar.button(HOVER).stroke.width", TOOL_HOVER_WIDTH),
    ("toolbar.button(PRESSED).stroke.width", TOOL_PRESSED_WIDTH),
    ("toolbar.button_label(HOVER).color", TOOL_HOVER_LABEL),
    ("toolbar.button_label(PRESSED).color", TOOL_PRESSED_LABEL),
    ("toolbar.button(NONE).fill", TOOL_REST_FILL),
    ("toolbar.button(NONE).corners", TOOL_REST_CORNERS),
    ("toolbar.button(HOVER|CHECKED).fill", EDIT_HOVER_FILL),
    ("toolbar.button(PRESSED|CHECKED).fill", EDIT_PRESSED_FILL),
    ("toolbar.button_label(CHECKED).color", EDIT_LABEL),
    (
        "toolbar.button_label(HOVER|CHECKED).color",
        EDIT_HOVER_LABEL,
    ),
    (
        "toolbar.button_label(PRESSED|CHECKED).color",
        EDIT_PRESSED_LABEL,
    ),
    ("toolbar.button_icon(NONE).color", TOOL_ICON_REST),
    ("toolbar.button_icon(HOVER).color", TOOL_ICON_HOVER),
    ("toolbar.button_icon(PRESSED).color", TOOL_ICON_PRESSED),
    ("toolbar.button_icon(CHECKED).color", EDIT_ICON_CHECKED),
    (
        "toolbar.button_icon(HOVER|CHECKED).color",
        EDIT_ICON_HOVER_CHECKED,
    ),
    (
        "toolbar.button_icon(PRESSED|CHECKED).color",
        EDIT_ICON_PRESSED_CHECKED,
    ),
];

/// The route of a tool button's fill at rest, the toolbar scope's rest capsule.
pub(crate) const TOOL_REST_FILL: &str = "egui: the toolbar scope's resting fill, \
     visuals.widgets.inactive.weak_bg_fill, where stated (native-theme-egui style/roles.rs, the \
     toolbar's rest capsule); per call, demo::next_toolbar_button frames the button at rest in it";
/// The route of a tool button's corners, the toolbar scope's rest capsule.
pub(crate) const TOOL_REST_CORNERS: &str = "egui: the toolbar scope's corner_radius in every \
     state, where stated (native-theme-egui style/roles.rs, the toolbar's rest capsule)";
/// The route of the edit switch's fill while on under the pointer.
pub(crate) const EDIT_HOVER_FILL: &str = "egui: per call, Button::fill, the edit switch's fill \
     while on under the pointer, in the state of its previous response, where a rule states \
     toolbar.button(CHECKED).fill for CHECKED, or one for HOVER|CHECKED states this one (the \
     query asking CHECKED); else egui's own selected button's, visuals.selection.bg_fill \
     (pages::basic::edit_look_at, native-theme-egui next::toolbar_button_checked)";
/// The route of the edit switch's fill while on and pressed.
pub(crate) const EDIT_PRESSED_FILL: &str = "egui: per call, Button::fill, the edit switch's fill \
     while on and pressed, in the state of its previous response, where a rule states \
     toolbar.button(CHECKED).fill for CHECKED, or one for PRESSED|CHECKED states this one (the \
     query asking CHECKED); else egui's own selected button's, visuals.selection.bg_fill \
     (pages::basic::edit_look_at, native-theme-egui next::toolbar_button_checked)";
/// The route of the edit switch's label while on, at rest.
pub(crate) const EDIT_LABEL: &str = "egui: per call, the edit switch's label colour while on, at \
     rest, where a rule states it for CHECKED, before button.label(CHECKED).color; else egui's \
     own selected button's, visuals.selection.stroke (pages::basic::edit_look_at, \
     native-theme-egui next::toolbar_button_checked)";
/// The route of the edit switch's label while on under the pointer.
pub(crate) const EDIT_HOVER_LABEL: &str = "egui: per call, the edit switch's label colour while \
     on under the pointer, in the state of its previous response, where a rule states \
     toolbar.button_label(CHECKED).color for CHECKED, or one for HOVER|CHECKED states this one \
     (the query asking CHECKED); else as at rest \
     (pages::basic::edit_look_at, native-theme-egui next::toolbar_button_checked)";
/// The route of the edit switch's label while on and pressed.
pub(crate) const EDIT_PRESSED_LABEL: &str = "egui: per call, the edit switch's label colour while \
     on and pressed, in the state of its previous response, where a rule states \
     toolbar.button_label(CHECKED).color for CHECKED, or one for PRESSED|CHECKED states this \
     one (the query asking CHECKED); else as at rest \
     (pages::basic::edit_look_at, native-theme-egui next::toolbar_button_checked)";
/// The route of a tool button's icon colour at rest.
pub(crate) const TOOL_ICON_REST: &str = "egui: per call, the colour the icon loads in at rest, \
     where stated, a colour stated at rest holding under the pointer and pressed, as egui changes \
     an image in no state; else the colour it loads in (demo::tool_icon_color, demo::loaded_in)";
/// The route of a tool button's icon colour under the pointer.
pub(crate) const TOOL_ICON_HOVER: &str = "egui: per call, the colour the icon loads in under the \
     pointer, in the state of the button's previous response, where stated \
     (demo::tool_icon_color, demo::loaded_in)";
/// The route of a tool button's icon colour while pressed.
pub(crate) const TOOL_ICON_PRESSED: &str = "egui: per call, the colour the icon loads in while \
     pressed, in the state of the button's previous response, where stated \
     (demo::tool_icon_color, demo::loaded_in)";
/// The route of the edit switch's icon colour while on, at rest.
pub(crate) const EDIT_ICON_CHECKED: &str = "egui: per call, the colour the edit switch's icon \
     loads in while on, at rest, where stated, a colour stated at rest holding while on, as \
     egui draws a selected button's image as it loads; else the switch's label colour \
     (demo::tool_icon_color, demo::loaded_in)";
/// The route of the edit switch's icon colour while on under the pointer.
pub(crate) const EDIT_ICON_HOVER_CHECKED: &str = "egui: per call, the colour the edit switch's \
     icon loads in while on under the pointer, in the state of its previous response, where \
     stated, as egui has no treatment of CHECKED for a button's image \
     (demo::tool_icon_color, demo::loaded_in)";
/// The route of the edit switch's icon colour while on and pressed.
pub(crate) const EDIT_ICON_PRESSED_CHECKED: &str = "egui: per call, the colour the edit switch's \
     icon loads in while on and pressed, in the state of its previous response, where stated, \
     as egui has no treatment of CHECKED for a button's image \
     (demo::tool_icon_color, demo::loaded_in)";

/// The homes of the side-panel toggle's checked outline, fill and label under the pointer and
/// pressed, its flat fill, label and outline under the pointer and pressed and its focused pen,
/// and of its icon's colour, which the chrome lays over the scope per call from the installed
/// 0.8.0 theme and no `mapping.toml` row holds, each with its route. Only the toggle and its
/// icon ([`TOGGLE`]) take them.
pub(crate) const TOGGLE_ROUTES: [(&str, &str); 30] = [
    ("button.label(NONE; variant=Flat).color", TOGGLE_REST_LABEL),
    (
        "button.body(NONE; variant=Flat).corners",
        TOGGLE_REST_CORNERS,
    ),
    (
        "button.body(HOVER; variant=Flat).stroke.width",
        TOGGLE_HOVER_WIDTH,
    ),
    (
        "button.body(PRESSED; variant=Flat).stroke.width",
        TOGGLE_PRESSED_WIDTH,
    ),
    (
        "button.body(HOVER; variant=Flat).stroke.color",
        TOGGLE_HOVER_EDGE,
    ),
    (
        "button.body(PRESSED; variant=Flat).stroke.color",
        TOGGLE_PRESSED_EDGE,
    ),
    (
        "button.body(HOVER; variant=Flat).fill",
        TOGGLE_CLOSED_HOVER_FILL,
    ),
    (
        "button.body(PRESSED; variant=Flat).fill",
        TOGGLE_CLOSED_PRESSED_FILL,
    ),
    (
        "button.label(HOVER; variant=Flat).color",
        TOGGLE_CLOSED_HOVER_LABEL,
    ),
    (
        "button.label(PRESSED; variant=Flat).color",
        TOGGLE_CLOSED_PRESSED_LABEL,
    ),
    (
        "button.body(FOCUS_VISIBLE; variant=Flat).stroke.color",
        TOGGLE_FOCUS_PEN,
    ),
    (
        "button.body(FOCUS_VISIBLE; variant=Flat).stroke.width",
        TOGGLE_FOCUS_PEN,
    ),
    (
        "button.body(HOVER|CHECKED; variant=Flat).fill",
        TOGGLE_HOVER_FILL,
    ),
    (
        "button.body(PRESSED|CHECKED; variant=Flat).fill",
        TOGGLE_PRESSED_FILL,
    ),
    (
        "button.label(HOVER|CHECKED; variant=Flat).color",
        TOGGLE_HOVER_LABEL,
    ),
    (
        "button.label(PRESSED|CHECKED; variant=Flat).color",
        TOGGLE_PRESSED_LABEL,
    ),
    (
        "button.icon(HOVER|CHECKED; variant=Flat).color",
        TOGGLE_ICON_HOVER,
    ),
    (
        "button.icon(PRESSED|CHECKED; variant=Flat).color",
        TOGGLE_ICON_PRESSED,
    ),
    (
        "button.icon(HOVER; variant=Flat).color",
        TOGGLE_ICON_CLOSED_HOVER,
    ),
    (
        "button.icon(PRESSED; variant=Flat).color",
        TOGGLE_ICON_CLOSED_PRESSED,
    ),
    (
        "button.body(CHECKED; variant=Flat).stroke.color",
        TOGGLE_CHECKED_PEN,
    ),
    (
        "button.body(CHECKED; variant=Flat).stroke.width",
        TOGGLE_CHECKED_PEN,
    ),
    (
        "button.body(HOVER|CHECKED; variant=Flat).stroke.color",
        TOGGLE_CHECKED_HOVER_PEN,
    ),
    (
        "button.body(HOVER|CHECKED; variant=Flat).stroke.width",
        TOGGLE_CHECKED_HOVER_PEN,
    ),
    (
        "button.body(FOCUS_VISIBLE|CHECKED; variant=Flat).stroke.color",
        TOGGLE_CHECKED_ACTIVE_PEN,
    ),
    (
        "button.body(FOCUS_VISIBLE|CHECKED; variant=Flat).stroke.width",
        TOGGLE_CHECKED_ACTIVE_PEN,
    ),
    (
        "button.body(PRESSED|CHECKED; variant=Flat).stroke.color",
        TOGGLE_CHECKED_ACTIVE_PEN,
    ),
    (
        "button.body(PRESSED|CHECKED; variant=Flat).stroke.width",
        TOGGLE_CHECKED_ACTIVE_PEN,
    ),
    ("button.icon(NONE; variant=Flat).color", TOGGLE_ICON_REST),
    ("button.icon(CHECKED; variant=Flat).color", TOGGLE_ICON_OPEN),
];
/// The route of the toggle's outline width under the pointer.
pub(crate) const TOGGLE_HOVER_WIDTH: &str = "egui: per call, the toggle's outline width under \
     the pointer while the side panel is closed, button.body(HOVER; variant=Flat).stroke.width \
     where a rule states the stroke for HOVER, else egui's own treatment of the hover \
     (native-theme-egui next::flat_button; demo::flat); open, where a rule states \
     button.body(CHECKED; variant=Flat).stroke, or one for CHECKED|HOVER states this one (the \
     query asking CHECKED), the checked pens' (chrome::toggle_checked_pens)";
/// The route of the toggle's outline width pressed or focused, as [`TOGGLE_HOVER_WIDTH`].
pub(crate) const TOGGLE_PRESSED_WIDTH: &str = "egui: per call, the toggle's outline width pressed \
     or focused while the side panel is closed, egui's one active entry, button.body(PRESSED; \
     variant=Flat).stroke.width where a rule states the stroke for PRESSED and none states it for \
     FOCUS_VISIBLE, else egui's own treatment of the press (native-theme-egui \
     next::flat_button; demo::flat); open, where a rule states button.body(CHECKED; \
     variant=Flat).stroke, or one for CHECKED with the state drawn states it (the query asking \
     CHECKED), the checked pens' (chrome::toggle_checked_pens)";
/// The route of the closed toggle's fill under the pointer.
pub(crate) const TOGGLE_CLOSED_HOVER_FILL: &str = "egui: per call, the toggle's fill under the \
     pointer while the side panel is closed, visuals.widgets.hovered.weak_bg_fill, \
     button.body(HOVER; variant=Flat).fill where a rule states it for HOVER, else egui's own \
     treatment of the hover (native-theme-egui next::flat_button; demo::flat); open, not drawn: \
     the toggle is egui's selected button, filled in its checked fill of the state \
     (chrome::toggle_checked_in)";
/// The route of the closed toggle's fill pressed or focused.
pub(crate) const TOGGLE_CLOSED_PRESSED_FILL: &str = "egui: per call, the toggle's fill pressed or \
     focused while the side panel is closed, egui's one active entry, \
     visuals.widgets.active.weak_bg_fill, button.body(PRESSED; variant=Flat).fill where a rule \
     states it for PRESSED, else egui's own treatment of the press (native-theme-egui \
     next::flat_button; demo::flat); open, not drawn: the toggle is egui's selected button, \
     filled in its checked fill of the state (chrome::toggle_checked_in)";
/// The route of the closed toggle's label colour under the pointer.
pub(crate) const TOGGLE_CLOSED_HOVER_LABEL: &str = "egui: per call, the toggle's label colour \
     under the pointer while the side panel is closed, visuals.widgets.hovered.fg_stroke, \
     button.label(HOVER; variant=Flat).color where a rule states it for HOVER, else egui's own \
     treatment of the hover (native-theme-egui next::flat_button; demo::flat); open, not drawn: \
     the toggle is egui's selected button, lettered in its checked label of the state \
     (chrome::toggle_checked_in); an icon shown in the label's place keeps the colour it loads \
     in (chrome::toggle_icon_colour)";
/// The route of the closed toggle's label colour pressed or focused.
pub(crate) const TOGGLE_CLOSED_PRESSED_LABEL: &str = "egui: per call, the toggle's label colour \
     pressed or focused while the side panel is closed, egui's one active entry, \
     visuals.widgets.active.fg_stroke, button.label(PRESSED; variant=Flat).color where a rule \
     states it for PRESSED, else egui's own treatment of the press (native-theme-egui \
     next::flat_button; demo::flat); open, not drawn: the toggle is egui's selected button, \
     lettered in its checked label of the state (chrome::toggle_checked_in); an icon shown in \
     the label's place keeps the colour it loads in (chrome::toggle_icon_colour)";
/// The route of the open toggle's fill under the pointer.
pub(crate) const TOGGLE_HOVER_FILL: &str = "egui: per call, Button::fill, the toggle's fill under \
     the pointer while the side panel is open, in the state of its previous response, where a \
     rule states button.body(CHECKED; variant=Flat).fill for CHECKED, or one for HOVER|CHECKED \
     states this one (the query asking CHECKED); else egui's own selected button's, \
     visuals.selection.bg_fill (chrome::ghost_button, chrome::toggle_checked_in)";
/// The route of the open toggle's fill pressed.
pub(crate) const TOGGLE_PRESSED_FILL: &str = "egui: per call, Button::fill, the toggle's fill \
     pressed while the side panel is open, in the state of its previous response, where a rule \
     states button.body(CHECKED; variant=Flat).fill for CHECKED, or one for PRESSED|CHECKED \
     states this one (the query asking CHECKED); else egui's own selected button's, \
     visuals.selection.bg_fill (chrome::ghost_button, chrome::toggle_checked_in)";
/// The route of the open toggle's label under the pointer.
pub(crate) const TOGGLE_HOVER_LABEL: &str = "egui: per call, the toggle's label colour under the \
     pointer while the side panel is open, in the state of its previous response, where a rule \
     states button.label(CHECKED; variant=Flat).color for CHECKED, or one for HOVER|CHECKED \
     states this one (the query asking CHECKED); else egui's own selected button's, \
     visuals.selection.stroke; an icon shown in the label's place keeps the colour it loads in \
     (chrome::ghost_button, chrome::toggle_checked_in)";
/// The route of the open toggle's label pressed.
pub(crate) const TOGGLE_PRESSED_LABEL: &str = "egui: per call, the toggle's label colour pressed \
     while the side panel is open, in the state of its previous response, where a rule states \
     button.label(CHECKED; variant=Flat).color for CHECKED, or one for PRESSED|CHECKED states \
     this one (the query asking CHECKED); else egui's own selected button's, \
     visuals.selection.stroke; an icon shown in the label's place keeps the colour it loads in \
     (chrome::ghost_button, chrome::toggle_checked_in)";
/// The route of the open toggle's icon colour under the pointer.
pub(crate) const TOGGLE_ICON_HOVER: &str = "egui: per call, the colour the toggle's icon loads in \
     under the pointer while the side panel is open, in the state of its previous response, \
     where stated, as egui has no treatment of CHECKED for a button's image \
     (chrome::toggle_icon_colour)";
/// The route of the open toggle's icon colour pressed.
pub(crate) const TOGGLE_ICON_PRESSED: &str = "egui: per call, the colour the toggle's icon loads \
     in pressed while the side panel is open, in the state of its previous response, where \
     stated, as egui has no treatment of CHECKED for a button's image \
     (chrome::toggle_icon_colour)";
/// The route of the closed toggle's icon colour.
pub(crate) const TOGGLE_ICON_REST: &str = "egui: per call, the colour the toggle's icon loads in \
     while the side panel is closed, where stated, a colour stated at rest holding under the \
     pointer and pressed, as egui changes an image in no state; else the colour it loads in \
     (chrome::toggle_icon_colour)";
/// The route of the closed toggle's icon colour under the pointer.
pub(crate) const TOGGLE_ICON_CLOSED_HOVER: &str = "egui: per call, the colour the toggle's icon \
     loads in under the pointer while the side panel is closed, in the state of its previous \
     response, where stated (chrome::toggle_icon_colour, as demo::tool_icon_color the tool \
     buttons')";
/// The route of the closed toggle's icon colour pressed.
pub(crate) const TOGGLE_ICON_CLOSED_PRESSED: &str = "egui: per call, the colour the toggle's icon \
     loads in pressed while the side panel is closed, in the state of its previous response, \
     where stated (chrome::toggle_icon_colour, as demo::tool_icon_color the tool buttons')";
/// The route of the open toggle's icon colour.
pub(crate) const TOGGLE_ICON_OPEN: &str = "egui: per call, the colour the toggle's icon loads in \
     while the side panel is open, where stated, a colour stated at rest holding there too; \
     in the state of the toggle's previous response, under the pointer \
     button.icon(HOVER|CHECKED; variant=Flat).color and pressed \
     button.icon(PRESSED|CHECKED; variant=Flat).color, where stated; else the colour it \
     loads in \
     (chrome::toggle_icon_colour)";
/// The route of the open toggle's outline at rest.
pub(crate) const TOGGLE_CHECKED_PEN: &str = "egui: per call, the toggle's outline while the side panel is \
     open, at rest, egui's inactive entry, where a rule states it for CHECKED, a translucent pen \
     laid inside composited over the fill; else the scope's own (chrome::toggle_checked_pens)";
/// The route of the open toggle's outline under the pointer.
pub(crate) const TOGGLE_CHECKED_HOVER_PEN: &str = "egui: per call, the toggle's outline under the pointer \
     while the side panel is open, egui's hovered entry, the theme's answer for this state \
     where a rule states button.body(CHECKED; variant=Flat).stroke for CHECKED, or one for \
     CHECKED|HOVER states this one (the query asking CHECKED); else the scope's own \
     (chrome::toggle_checked_pens)";
/// The route of the open toggle's outline pressed or focused, egui's one `active` entry.
pub(crate) const TOGGLE_CHECKED_ACTIVE_PEN: &str = "egui: per call, the toggle's outline pressed or focused \
     while the side panel is open, egui's one active entry, the theme's answer for \
     (CHECKED|FOCUS_VISIBLE) where a rule states the flat button's FOCUS_VISIBLE stroke, else \
     for (CHECKED|PRESSED), where a rule states button.body(CHECKED; variant=Flat).stroke for \
     CHECKED, or one for CHECKED with that state states this one (the query asking CHECKED); \
     else the scope's own (chrome::toggle_checked_pens)";
/// The route of the flat button's focused pen on the toggle: egui's one `active` entry, press and
/// keyboard focus alike.
pub(crate) const TOGGLE_FOCUS_PEN: &str = "egui: per call, the toggle's outline pressed or focused, egui's \
     one active entry: closed, button.body(FOCUS_VISIBLE; variant=Flat).stroke, its colour and \
     width, where a rule states it for FOCUS_VISIBLE, else button.body(PRESSED; \
     variant=Flat).stroke's where one states it, else egui's own treatment of the press \
     (native-theme-egui next::flat_button; demo::flat); open, where a rule states \
     button.body(CHECKED; variant=Flat).stroke, or one for CHECKED with the state drawn states \
     it (the query asking CHECKED), whether one states this pen picks \
     (CHECKED|FOCUS_VISIBLE)'s over (CHECKED|PRESSED)'s (chrome::toggle_checked_pens)";

/// The homes a page applies per call where `mapping.toml`'s row names a style field the
/// connector writes, or none, by element id (a prefix, for the rows) and home: the List's and
/// the table's frames, which `pages::basic::list_frame` paints, the rows whose fills
/// `pages::basic::inner_corners` rounds where they meet a frame, the selected rows' unfocused
/// look and a hovered list row's outline, which `pages::basic::RowLook::of` takes, the hover
/// outline the Basic table's rows never take, the menu bar's fill, which
/// `chrome::menu_panel` paints, the side-panel toggle's checked look, which `chrome::ghost_button`
/// lays over egui's selected button, the Basic page's edit switch's checked fill and label, which
/// `pages::basic::edit_look_at` hands it per state, and its checked pens, which
/// `pages::basic::tool_pen` hands it, the On toggle's checked look per state, which
/// `pages::basic::toggle_on_in` hands it, and its checked pens and the push button's outline they
/// cover, which `pages::basic::toggle_on_pen` hands it, the flat buttons' hovered and pressed outlines, which
/// `next::flat_button` lays over their scope, the scroll bar thumb's corners, which the atlas writes, the
/// primary button's own outline and the dialog's default button's, which `pages::basic` hands
/// each button, the segmented control's
/// separator colour, which the companion crate's control paints its dividers in, the primary
/// button's own corners, which egui reads none of, and
/// the text selection the Theme row's `ComboBox` has none of.
const PER_CALL_ROUTES: &[(&str, &str, &str)] = &[
    // The disabled checked check box, in the check box's SelectedDisabled cell.
    (
        "basic.checkboxes.disabled",
        "checkbox.label(DISABLED|CHECKED).color",
        CHECKED_DISABLED_LABEL,
    ),
    (
        "basic.checkboxes.disabled.label",
        "checkbox.label(DISABLED|CHECKED).color",
        CHECKED_DISABLED_LABEL,
    ),
    (
        "basic.checkboxes.disabled.mark",
        "checkbox.label(DISABLED|CHECKED).color",
        CHECKED_DISABLED_LABEL,
    ),
    // Save, the disabled tool button (pages::basic::icon_buttons), in the toolbar's Disabled cell.
    (
        "basic.icons.save",
        "toolbar.button(DISABLED).fill",
        SAVE_DISABLED_FILL,
    ),
    (
        "basic.icons.save",
        "toolbar.button(DISABLED).corners",
        SAVE_DISABLED_CORNERS,
    ),
    (
        "basic.icons.save",
        "toolbar.button(NONE).fill",
        SAVE_REST_FILL,
    ),
    (
        "basic.icons.save",
        "toolbar.button(NONE).corners",
        SAVE_REST_CORNERS,
    ),
    (
        "basic.icons.save",
        "toolbar.button(HOVER).stroke.color",
        "not drawn: egui draws a disabled button in its resting entry alone, never hovered \
         (egui-0.37.0 src/ui.rs:1713-1724, Ui::add_enabled)",
    ),
    (
        "basic.icons.save",
        "toolbar.button_label(DISABLED).color",
        SAVE_DISABLED_LABEL,
    ),
    (
        "basic.icons.save",
        "toolbar.button_label(NONE).color",
        SAVE_REST_LABEL,
    ),
    (
        "basic.icons.save.icon",
        "toolbar.button_label(NONE).color",
        SAVE_REST_LABEL,
    ),
    (
        "basic.icons.save.icon",
        "toolbar.button_label(DISABLED).color",
        SAVE_DISABLED_LABEL,
    ),
    (
        "basic.icons.save",
        "colors.disabled_opacity",
        "egui: the toolbar scope's visuals.disabled_alpha, the base style's, by which egui \
         multiplies the disabled button's painter opacity, its fill, label and icon \
         (native-theme-egui style/base.rs; egui-0.37.0 src/ui.rs:523-529, Ui::disable)",
    ),
    (
        "basic.icons.save",
        "button.disabled_opacity",
        "not drawn: the toolbar scope is no button scope, and its disabled_alpha is the base \
         style's, colors.disabled_opacity (native-theme-egui style/variants.rs, disabled_cell)",
    ),
    (
        "basic.icons.save",
        "colors.window.foreground_muted",
        "not drawn: egui letters a disabled tool button in the scope's label and loads its icon \
         in the colour a rule states for DISABLED, else in that label, each faded by \
         disabled_alpha, not in a muted colour (demo::disabled_tool_icon_color)",
    ),
    (
        "basic.icons.save",
        "toolbar.button_icon(DISABLED).color",
        SAVE_ICON_DISABLED,
    ),
    (
        "basic.icons.save.icon",
        "toolbar.button_icon(DISABLED).color",
        SAVE_ICON_DISABLED,
    ),
    (
        "basic.list.frame",
        "list.container(NONE).corners",
        "egui: per call, the frame's corner radius (pages::basic::list_frame)",
    ),
    (
        "basic.list.frame",
        "list.container(NONE).stroke.width",
        "egui: per call, the frame's line width (pages::basic::list_frame)",
    ),
    (
        "basic.table.frame",
        "list.container(NONE).corners",
        "egui: per call, the frame's corner radius (pages::basic::list_frame)",
    ),
    (
        "basic.table.frame",
        "list.container(NONE).stroke.width",
        "egui: per call, the frame's line width (pages::basic::list_frame)",
    ),
    (
        "basic.table.header",
        "list.container(NONE).corners",
        "egui: per call, the fill's top corners at the frame's inner radius, this less \
         list.container(NONE).stroke.width (pages::basic::inner_corners)",
    ),
    (
        "basic.table.header",
        "list.container(NONE).stroke.width",
        "egui: per call, taken off the frame's radius for the fill's top corners \
         (pages::basic::inner_corners)",
    ),
    (
        "basic.table.row_",
        "list.container(NONE).corners",
        "egui: per call, the last row's bottom corners at the frame's inner radius, this less \
         list.container(NONE).stroke.width (pages::basic::inner_corners)",
    ),
    (
        "basic.table.row_",
        "list.container(NONE).stroke.width",
        "egui: per call, taken off the frame's radius for the last row's bottom corners \
         (pages::basic::inner_corners)",
    ),
    (
        "basic.list.row_",
        "list.container(NONE).corners",
        "egui: per call, a fill's corners where it meets the frame at the frame's inner \
         radius, this less list.container(NONE).stroke.width (pages::basic::inner_corners)",
    ),
    (
        "basic.list.row_",
        "list.container(NONE).stroke.width",
        "egui: per call, taken off the frame's radius for a fill's corners where it meets the \
         frame (pages::basic::inner_corners)",
    ),
    (
        "basic.table.row_",
        "colors.selection.unfocused_background",
        SELECTED_ROW_UNFOCUSED_FILL,
    ),
    (
        "basic.table.row_",
        "colors.selection.unfocused_foreground",
        SELECTED_ROW_UNFOCUSED_TEXT,
    ),
    (
        "basic.list.row_",
        "colors.selection.unfocused_background",
        SELECTED_ROW_UNFOCUSED_FILL,
    ),
    (
        "basic.list.row_",
        "colors.selection.unfocused_foreground",
        SELECTED_ROW_UNFOCUSED_TEXT,
    ),
    ("basic.list.row_", "layout.border_width", HOVERED_ROW_EDGE),
    ("basic.list.frame", "layout.border_width", HOVERED_ROW_EDGE),
    (
        "basic.table.frame",
        "list.highlight(HOVER).stroke.color",
        TABLE_ROW_NO_HOVER,
    ),
    (
        "basic.table.frame",
        "layout.border_width",
        TABLE_ROW_NO_HOVER,
    ),
    (
        "basic.table.row_",
        "list.highlight(HOVER).stroke.color",
        TABLE_ROW_NO_HOVER,
    ),
    (
        "basic.table.row_",
        "layout.border_width",
        TABLE_ROW_NO_HOVER,
    ),
    (
        "chrome.menu_bar",
        "menu_bar.bar(NONE).fill",
        "egui: per call, the menu bar panel's fill (chrome::menu_panel)",
    ),
    (
        "basic.buttons.primary",
        "button.body(NONE; variant=Primary).stroke.color",
        PRIMARY_OUTLINE,
    ),
    (
        "basic.buttons.primary",
        "button.body(HOVER; variant=Primary).stroke.color",
        PRIMARY_OUTLINE,
    ),
    (
        "basic.buttons.primary",
        "button.body(PRESSED; variant=Primary).stroke.color",
        PRIMARY_OUTLINE,
    ),
    (
        "basic.buttons.primary",
        "button.body(NONE; variant=Primary).corners",
        PRIMARY_CORNERS_UNREAD,
    ),
    (
        "basic.buttons.primary",
        "button.body(NONE; variant=Primary).stroke.width",
        PRIMARY_OUTLINE,
    ),
    (
        "basic.buttons.primary",
        "button.body(HOVER; variant=Primary).stroke.width",
        PRIMARY_OUTLINE,
    ),
    (
        "basic.buttons.primary",
        "button.body(PRESSED; variant=Primary).stroke.width",
        PRIMARY_OUTLINE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(NONE; variant=Primary).stroke.color",
        DIALOG_DEFAULT_OUTLINE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(NONE; variant=Primary).stroke.width",
        DIALOG_DEFAULT_OUTLINE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(HOVER; variant=Primary).stroke.color",
        DIALOG_DEFAULT_OUTLINE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(HOVER; variant=Primary).stroke.width",
        DIALOG_DEFAULT_OUTLINE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(PRESSED; variant=Primary).stroke.color",
        DIALOG_DEFAULT_OUTLINE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(PRESSED; variant=Primary).stroke.width",
        DIALOG_DEFAULT_OUTLINE,
    ),
    (
        "chrome.status_bar.toggle",
        "button.body(CHECKED).fill",
        "egui: per call, Button::fill, the toggle's fill while the side panel is open, in the \
         state of its previous response: where a rule states button.body(CHECKED; \
         variant=Flat).fill for CHECKED, at rest this, under the pointer \
         button.body(HOVER|CHECKED; variant=Flat).fill and pressed button.body(PRESSED|CHECKED; \
         variant=Flat).fill, each also where a rule for that state with CHECKED states it (the \
         query asking CHECKED); else egui's own selected button's in that state, \
         visuals.selection.bg_fill (chrome::ghost_button, chrome::toggle_checked_in)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.label(CHECKED).color",
        "egui: per call, the toggle's label colour while the side panel is open, in the state of \
         its previous response: where a rule states button.label(CHECKED; variant=Flat).color \
         for CHECKED, at rest this, under the pointer button.label(HOVER|CHECKED; \
         variant=Flat).color and pressed button.label(PRESSED|CHECKED; variant=Flat).color, each \
         also where a rule for that state with CHECKED states it (the query asking CHECKED); \
         else egui's own selected button's in that state, visuals.selection.stroke; an icon \
         shown in the label's place keeps the colour it loads in (chrome::ghost_button, \
         chrome::toggle_checked_in)",
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(CHECKED).fill",
        "egui: per call, Button::fill, the toggle's fill while on, in the state of its previous \
         response: where a rule states it for CHECKED, at rest this, under the pointer \
         button.body(HOVER|CHECKED).fill and pressed button.body(PRESSED|CHECKED).fill, each \
         also where a rule for that state with CHECKED states it (the query asking CHECKED); \
         else egui's own selected button's in that state, visuals.selection.bg_fill \
         (pages::basic::toggle_on_in)",
    ),
    (
        "basic.buttons.toggle_on",
        "button.label(CHECKED).color",
        "egui: per call, RichText::color, the toggle's label colour while on, in the state of its \
         previous response: where a rule states it for CHECKED, at rest this, under the pointer \
         button.label(HOVER|CHECKED).color and pressed button.label(PRESSED|CHECKED).color, each \
         also where a rule for that state with CHECKED states it (the query asking CHECKED); \
         else egui's own selected button's in that state, visuals.selection.stroke \
         (pages::basic::toggle_on_in)",
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(HOVER|CHECKED).fill",
        TOGGLE_ON_HOVER_FILL,
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(PRESSED|CHECKED).fill",
        TOGGLE_ON_PRESSED_FILL,
    ),
    (
        "basic.buttons.toggle_on",
        "button.label(HOVER|CHECKED).color",
        TOGGLE_ON_HOVER_LABEL,
    ),
    (
        "basic.buttons.toggle_on",
        "button.label(PRESSED|CHECKED).color",
        TOGGLE_ON_PRESSED_LABEL,
    ),
    (
        "basic.buttons.toggle_on.label",
        "button.label(HOVER|CHECKED).color",
        TOGGLE_ON_HOVER_LABEL,
    ),
    (
        "basic.buttons.toggle_on.label",
        "button.label(PRESSED|CHECKED).color",
        TOGGLE_ON_PRESSED_LABEL,
    ),
    (
        "basic.buttons.toggle_on",
        "button.label(PRESSED).color",
        TOGGLE_ON_PRESSED_RULE_LABEL,
    ),
    (
        "basic.buttons.toggle_on.label",
        "button.label(PRESSED).color",
        TOGGLE_ON_PRESSED_RULE_LABEL,
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(CHECKED).stroke.color",
        TOGGLE_ON_PEN,
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(CHECKED).stroke.width",
        TOGGLE_ON_PEN,
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(HOVER|CHECKED).stroke.color",
        TOGGLE_ON_HOVER_PEN,
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(HOVER|CHECKED).stroke.width",
        TOGGLE_ON_HOVER_PEN,
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(PRESSED|CHECKED).stroke.color",
        TOGGLE_ON_PRESSED_PEN,
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(PRESSED|CHECKED).stroke.width",
        TOGGLE_ON_PRESSED_PEN,
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(NONE).stroke.color",
        TOGGLE_ON_REST_EDGE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(NONE; variant=Primary).corners",
        DIALOG_DEFAULT_CORNERS,
    ),
    (
        "basic.icons.edit",
        "toolbar.button(HOVER).stroke.width",
        EDIT_HOVER_WIDTH,
    ),
    (
        "basic.icons.edit",
        "toolbar.button(PRESSED).stroke.width",
        EDIT_PRESSED_WIDTH,
    ),
    (
        "basic.icons.edit",
        "toolbar.button_label(HOVER).color",
        EDIT_OFF_HOVER_LABEL,
    ),
    (
        "basic.icons.edit",
        "toolbar.button_label(PRESSED).color",
        EDIT_OFF_PRESSED_LABEL,
    ),
    (
        "basic.icons.edit",
        "toolbar.button(CHECKED).stroke.color",
        EDIT_CHECKED_PEN,
    ),
    (
        "basic.icons.edit",
        "toolbar.button(CHECKED).stroke.width",
        EDIT_CHECKED_WIDTH,
    ),
    (
        "basic.icons.edit",
        "toolbar.button(HOVER|CHECKED).stroke.width",
        EDIT_HOVER_CHECKED_WIDTH,
    ),
    (
        "basic.icons.edit",
        "toolbar.button(PRESSED|CHECKED).stroke.color",
        EDIT_PRESSED_PEN,
    ),
    (
        "basic.icons.edit",
        "toolbar.button(PRESSED|CHECKED).stroke.width",
        EDIT_PRESSED_PEN,
    ),
    (
        "basic.icons.edit",
        "toolbar.button(PRESSED|HOVER|CHECKED).stroke.color",
        EDIT_PRESSED_UNDER_POINTER,
    ),
    (
        "basic.icons.edit",
        "toolbar.button(PRESSED|HOVER|CHECKED).stroke.width",
        EDIT_PRESSED_UNDER_POINTER,
    ),
    (
        "basic.buttons.toggle_on",
        "button.body(PRESSED).fill",
        "egui: not drawn: the toggle is on, egui's selected button, which egui fills in \
         visuals.selection.bg_fill in every state, the active entry's fill too \
         (egui-0.37.0 src/theme/default_style.rs, StyleProvider<ButtonStyle>); pressed it is \
         filled in button.body(PRESSED|CHECKED).fill where a rule states \
         button.body(CHECKED).fill for CHECKED, or one for PRESSED|CHECKED states it (the query \
         asking CHECKED) (pages::basic::toggle_on_in)",
    ),
    (
        "basic.icons.edit",
        "toolbar.button(CHECKED).fill",
        "egui: per call, Button::fill, the edit switch's fill while on, in the state of its \
         previous response: where a rule states it for CHECKED, at rest this, under the pointer \
         toolbar.button(HOVER|CHECKED).fill and pressed toolbar.button(PRESSED|CHECKED).fill, \
         each also where a rule for that state with CHECKED states it (the query asking \
         CHECKED); else egui's own selected button's in that state, visuals.selection.bg_fill \
         (pages::basic::edit_look_at, native-theme-egui next::toolbar_button_checked)",
    ),
    (
        "basic.icons.edit",
        "button.label(CHECKED).color",
        "egui: per call, the edit switch's label colour while on, where no rule states \
         toolbar.button_label(CHECKED).color, nor one for the state drawn with CHECKED (the \
         query asking CHECKED); where one does, that one at rest, \
         toolbar.button_label(HOVER|CHECKED).color under the pointer and \
         toolbar.button_label(PRESSED|CHECKED).color pressed; else egui's own selected button's, \
         visuals.selection.stroke. Its icon loads in it where \
         toolbar.button_icon(<state>|CHECKED).color is unstated; where stated, in that \
         (pages::basic::edit_look_at, \
         demo::tool_icon_color)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.body(HOVER).stroke.color",
        TOGGLE_HOVER_EDGE,
    ),
    (
        "chrome.status_bar.toggle",
        "button.body(PRESSED).stroke.color",
        TOGGLE_PRESSED_EDGE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(NONE).stroke.color",
        DIALOG_DEFAULT_OUTLINE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(NONE).stroke.width",
        DIALOG_DEFAULT_OUTLINE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(HOVER).stroke.color",
        DIALOG_DEFAULT_OUTLINE,
    ),
    (
        "basic.buttons.dialog_default",
        "button.body(PRESSED).stroke.color",
        DIALOG_DEFAULT_OUTLINE,
    ),
    (
        "chrome.page_tabs.menu",
        "button.body(HOVER).stroke.color",
        FLAT_HOVER_EDGE,
    ),
    (
        "chrome.page_tabs.menu",
        "button.body(PRESSED).stroke.color",
        FLAT_PRESSED_EDGE,
    ),
    (
        "chrome.info.copy",
        "button.body(HOVER).stroke.color",
        FLAT_HOVER_EDGE,
    ),
    (
        "chrome.info.copy",
        "button.body(PRESSED).stroke.color",
        FLAT_PRESSED_EDGE,
    ),
    (
        "chrome.info.copy",
        "button.body(HOVER).stroke.width",
        FLAT_HOVER_WIDTH,
    ),
    (
        "basic.list.scrollbar",
        "scrollbar.thumb(NONE).corners",
        THUMB_CORNERS,
    ),
    (
        "basic.list.scrollbar.thumb",
        "scrollbar.thumb(NONE).corners",
        THUMB_CORNERS,
    ),
    (
        "basic.list.scrollbar",
        "scrollbar.thumb(UNFOCUSED_VIEW).corners",
        UNFOCUSED_THUMB_CORNERS,
    ),
    (
        "basic.list.scrollbar.thumb",
        "scrollbar.thumb(UNFOCUSED_VIEW).corners",
        UNFOCUSED_THUMB_CORNERS,
    ),
    (
        "basic.segmented.divider_1",
        "segmented_control.separator(NONE).fill",
        SEGMENT_DIVIDER,
    ),
    (
        "basic.segmented.divider_2",
        "segmented_control.separator(NONE).fill",
        SEGMENT_DIVIDER,
    ),
    (
        "chrome.side_panel.settings.theme",
        "colors.text_selection.background",
        "not reachable: the Theme row is a ComboBox, which has no text field to select text in \
         (chrome::settings_rows)",
    ),
    (
        "basic.icons.edit",
        "toolbar.button(NONE).fill",
        EDIT_REST_FILL,
    ),
    (
        "basic.icons.edit",
        "toolbar.button_icon(NONE).color",
        EDIT_ICON_REST,
    ),
    (
        "basic.icons.edit",
        "toolbar.button_icon(HOVER).color",
        EDIT_ICON_HOVER,
    ),
    (
        "basic.icons.edit",
        "toolbar.button_icon(PRESSED).color",
        EDIT_ICON_PRESSED,
    ),
    (
        "basic.icons.edit.icon",
        "toolbar.button_icon(NONE).color",
        EDIT_ICON_REST,
    ),
    (
        "basic.icons.edit.icon",
        "toolbar.button_icon(HOVER).color",
        EDIT_ICON_HOVER,
    ),
    (
        "basic.icons.edit.icon",
        "toolbar.button_icon(PRESSED).color",
        EDIT_ICON_PRESSED,
    ),
];

/// The dialog's default button's own corners, where it is the primary button.
pub(crate) const DIALOG_DEFAULT_CORNERS: &str = "not drawn: where the theme gives the dialog's \
     default button no look of its own it is the primary button, egui's selected Button in the \
     button scope (pages::basic, primary_at), rounded as the push button, by \
     button.body(NONE).corners";

/// The edit switch's hovered outline width: [`TOOL_HOVER_WIDTH`]'s while it is off.
pub(crate) const EDIT_HOVER_WIDTH: &str = "egui: while the edit switch is off, and while it is on \
     where pages::basic::tool_pen finds no rule stating toolbar.button(HOVER|CHECKED).stroke, as \
     the other tool buttons: the toolbar scope's hovered entry, \
     visuals.widgets.hovered.bg_stroke's width, where a rule states toolbar.button(HOVER).stroke \
     for HOVER, else egui's own treatment of the hover (native-theme-egui next/build.rs, \
     ToolbarCell button_border)";

/// The edit switch's pressed outline width: [`TOOL_PRESSED_WIDTH`]'s while it is off.
pub(crate) const EDIT_PRESSED_WIDTH: &str = "egui: while the edit switch is off, and while it is \
     on where pages::basic::tool_pen finds no rule stating toolbar.button(PRESSED|CHECKED).stroke, \
     as the other tool buttons: the toolbar scope's active entry, \
     visuals.widgets.active.bg_stroke's width, where a rule states \
     toolbar.button(PRESSED).stroke for PRESSED, else egui's own treatment of the press \
     (native-theme-egui next/build.rs, ToolbarCell button_border)";

/// The edit switch's hovered label: [`TOOL_HOVER_LABEL`]'s while it is off.
pub(crate) const EDIT_OFF_HOVER_LABEL: &str = "egui: while the edit switch is off, as the other \
     tool buttons: the toolbar scope's hovered entry, visuals.widgets.hovered.fg_stroke, where a \
     rule states it for HOVER (native-theme-egui next/build.rs, ToolbarCell button_text), the \
     label's colour where the set has no icon; while it is on (it opens on), not drawn: its text \
     is the checked label in every state, override_text_color (pages::basic::icon_buttons)";

/// The edit switch's pressed label: [`TOOL_PRESSED_LABEL`]'s while it is off.
pub(crate) const EDIT_OFF_PRESSED_LABEL: &str = "egui: while the edit switch is off, as the other \
     tool buttons: the toolbar scope's active entry, visuals.widgets.active.fg_stroke, where a \
     rule states it for PRESSED (native-theme-egui next/build.rs, ToolbarCell button_text), the \
     label's colour where the set has no icon; while it is on (it opens on), not drawn: its text \
     is the checked label in every state, override_text_color (pages::basic::icon_buttons)";

/// The edit switch's outline width while on, at rest.
pub(crate) const EDIT_CHECKED_WIDTH: &str = "egui: per call, the edit switch's outline width \
     while on at rest, egui's inactive entry, visuals.widgets.inactive.bg_stroke, where a rule \
     states toolbar.button(CHECKED).stroke for the state, else the scope's own \
     (pages::basic::tool_pen, demo::state_pen)";

/// The edit switch's outline colour while on, at rest.
pub(crate) const EDIT_CHECKED_PEN: &str = "egui: per call, the edit switch's outline colour \
     while on at rest, egui's inactive entry, visuals.widgets.inactive.bg_stroke, where a rule \
     states toolbar.button(CHECKED).stroke for the state, else the scope's own \
     (pages::basic::tool_pen, demo::state_pen)";

/// The edit switch's outline width while on under the pointer.
pub(crate) const EDIT_HOVER_CHECKED_WIDTH: &str = "egui: per call, the edit switch's outline \
     width while on under the pointer, egui's hovered entry, visuals.widgets.hovered.bg_stroke, \
     where a rule states toolbar.button(HOVER|CHECKED).stroke for the state, else the scope's \
     own (pages::basic::tool_pen, demo::state_pen)";

/// The edit switch's fill at rest: [`TOOL_REST_FILL`]'s while it is off.
pub(crate) const EDIT_REST_FILL: &str = "egui: while the edit switch is off, as the other tool \
     buttons: the toolbar scope's resting fill, where stated (native-theme-egui style/roles.rs, \
     the toolbar's rest capsule), demo::next_toolbar_button framing the button at rest in it; \
     while it is on (it opens on), not drawn: it is egui's selected Button, filled in \
     toolbar.button(CHECKED).fill or visuals.selection.bg_fill, and the rest frame is skipped \
     (pages::basic::icon_buttons)";

/// The edit switch's icon at rest: [`TOOL_BUTTON_ROUTES`]' line holds only while the switch is
/// off; while on, `demo::tool_icon_color` reads `CHECKED` and its forms instead.
pub(crate) const EDIT_ICON_REST: &str = "egui: while the edit switch is off, as the other tool buttons: the \
     colour the icon loads in at rest, where stated (demo::tool_icon_color, demo::loaded_in); \
     while it is on (it opens on), toolbar.button_icon(CHECKED)'s colour, the colour stated at \
     rest where no rule states one for CHECKED, egui drawing a selected button's image as it \
     loads; where none is stated, the switch's label colour";

/// The edit switch's icon under the pointer, as [`EDIT_ICON_REST`].
pub(crate) const EDIT_ICON_HOVER: &str = "egui: while the edit switch is off, the colour the icon loads in \
     under the pointer, in the state of the button's previous response, where stated \
     (demo::tool_icon_color, demo::loaded_in); while it is on (it opens on), not drawn: \
     toolbar.button_icon(HOVER|CHECKED)'s colour is read instead";

/// The edit switch's icon while pressed, as [`EDIT_ICON_REST`].
pub(crate) const EDIT_ICON_PRESSED: &str = "egui: while the edit switch is off, the colour the icon loads in \
     while pressed, in the state of the button's previous response, where stated \
     (demo::tool_icon_color, demo::loaded_in); while it is on (it opens on), not drawn: \
     toolbar.button_icon(PRESSED|CHECKED)'s colour is read instead";

/// The edit switch's outline while on, pressed or keyboard-focused, egui's one `active` entry.
pub(crate) const EDIT_PRESSED_PEN: &str = "egui: per call, the edit switch's outline while on, pressed or \
     keyboard-focused, egui's one active entry, visuals.widgets.active.bg_stroke, its colour and \
     width, where a rule states toolbar.button(PRESSED|CHECKED).stroke for the state, else the \
     scope's own (pages::basic::tool_pen, demo::state_pen)";

/// The edit switch's outline pressed under the pointer, iced's query, which egui does not ask.
pub(crate) const EDIT_PRESSED_UNDER_POINTER: &str = "not drawn: egui asks the edit switch's pressed outline \
     as toolbar.button(PRESSED|CHECKED), a pressed response being egui's active entry whether \
     the pointer is over it or not (pages::basic::tool_pen)";

/// Save's fill, the disabled tool button's, in the toolbar's Disabled cell.
pub(crate) const SAVE_DISABLED_FILL: &str = "egui: the toolbar scope's Disabled cell's resting \
     fill, visuals.widgets.inactive.weak_bg_fill, the entry egui draws a disabled button in, where \
     a rule states it for DISABLED (native-theme-egui style/variants.rs, disabled_cell); per call, \
     demo::next_disabled_toolbar_button keeps the cell's fill where the theme states one for \
     DISABLED, by a rule for the state or at rest, else transparent, as a Ghost; faded by the \
     scope's disabled_alpha (egui-0.37.0 src/ui.rs:523-529, Ui::disable)";

/// Save's corners, the Disabled cell's.
pub(crate) const SAVE_DISABLED_CORNERS: &str = "egui: the toolbar scope's Disabled cell's \
     corner_radius, where a rule states it for DISABLED; else the Normal cell's, \
     toolbar.button(NONE).corners where stated (native-theme-egui style/variants.rs, \
     disabled_cell)";

/// Save's fill where no rule states the disabled one: the rest capsule's, kept by the cell.
pub(crate) const SAVE_REST_FILL: &str = "egui: where no rule states \
     toolbar.button(DISABLED).fill, the disabled button's fill: the Disabled cell keeps the Normal \
     cell's resting fill, and demo::next_disabled_toolbar_button keeps it, the theme's answer \
     for DISABLED being the fill at rest (native-theme-egui style/variants.rs, disabled_cell)";

/// Save's corners where no rule states the disabled ones, as [`SAVE_REST_FILL`].
pub(crate) const SAVE_REST_CORNERS: &str = "egui: where no rule states \
     toolbar.button(DISABLED).corners, the Disabled cell's corner_radius, which it keeps from the \
     Normal cell's rest capsule (native-theme-egui style/variants.rs, disabled_cell; \
     style/roles.rs, the toolbar's rest capsule)";

/// Save's label disabled, which the Disabled cell writes where a rule states it.
pub(crate) const SAVE_DISABLED_LABEL: &str = "egui: the toolbar scope's Disabled cell's label \
     colour, visuals.widgets.inactive.fg_stroke and noninteractive.fg_stroke, where a rule \
     states it for DISABLED, faded by disabled_alpha: the label's where the set has no icon for \
     the button, and the colour the icon loads in where no rule states \
     toolbar.button_icon(DISABLED).color, the scope's text colour (native-theme-egui \
     style/variants.rs, disabled_cell; demo::disabled_tool_icon_color, demo::role_image; \
     egui-0.37.0 src/ui.rs:523-529, Ui::disable)";

/// Save's icon colour, per call.
pub(crate) const SAVE_ICON_DISABLED: &str = "egui: per call, the colour the icon loads in, where \
     a rule states it for DISABLED; else the icon loads in the scope's text colour, \
     toolbar.button_label(DISABLED).color where a rule states it, else \
     toolbar.button_label(NONE).color where stated; faded with the button by the scope's \
     disabled_alpha (demo::disabled_tool_icon_color, demo::loaded_in; egui-0.37.0 \
     src/ui.rs:523-529, Ui::disable)";

/// Save's label colour at rest, which the Disabled cell keeps where no rule states the disabled
/// one: its label's, and its icon's where no rule states the disabled icon colour.
pub(crate) const SAVE_REST_LABEL: &str = "egui: where no rule states \
     toolbar.button_label(DISABLED).color, the toolbar scope's label colour, which the Disabled \
     cell keeps, faded by its disabled_alpha: the label's where the set has no icon for the \
     button, and the colour the icon loads in where no rule states \
     toolbar.button_icon(DISABLED).color (native-theme-egui next/build.rs, ToolbarCell panel; \
     style/variants.rs, disabled_cell; demo::disabled_tool_icon_color, demo::role_image; \
     egui-0.37.0 src/ui.rs:523-529, Ui::disable)";

/// The On toggle's fill under the pointer, per call in `pages::basic::toggle_on_in`.
pub(crate) const TOGGLE_ON_HOVER_FILL: &str = "egui: per call, Button::fill, the toggle's fill while on \
     under the pointer, in the state of its previous response, where a rule states \
     button.body(CHECKED).fill for CHECKED, or one for HOVER|CHECKED states this one (the query \
     asking CHECKED); else egui's own selected button's, visuals.selection.bg_fill \
     (pages::basic::toggle_on_in)";

/// The On toggle's fill pressed, as [`TOGGLE_ON_HOVER_FILL`].
pub(crate) const TOGGLE_ON_PRESSED_FILL: &str = "egui: per call, Button::fill, the toggle's fill while on \
     and pressed, in the state of its previous response, where a rule states \
     button.body(CHECKED).fill for CHECKED, or one for PRESSED|CHECKED states this one (the \
     query asking CHECKED); else egui's own selected button's, visuals.selection.bg_fill \
     (pages::basic::toggle_on_in)";

/// The On toggle's label under the pointer, as [`TOGGLE_ON_HOVER_FILL`].
pub(crate) const TOGGLE_ON_HOVER_LABEL: &str = "egui: per call, RichText::color, the toggle's label colour \
     while on under the pointer, in the state of its previous response, where a rule states \
     button.label(CHECKED).color for CHECKED, or one for HOVER|CHECKED states this one (the \
     query asking CHECKED); else egui's own selected button's, visuals.selection.stroke \
     (pages::basic::toggle_on_in)";

/// The On toggle's label pressed, as [`TOGGLE_ON_HOVER_FILL`].
pub(crate) const TOGGLE_ON_PRESSED_LABEL: &str = "egui: per call, RichText::color, the toggle's label colour \
     while on and pressed, in the state of its previous response, where a rule states \
     button.label(CHECKED).color for CHECKED, or one for PRESSED|CHECKED states this one (the \
     query asking CHECKED); else egui's own selected button's, visuals.selection.stroke \
     (pages::basic::toggle_on_in)";

/// The pressed label on the On toggle, which the theme's answer for `PRESSED|CHECKED` takes where
/// the rule stating it follows every rule for `CHECKED`, as [`TOGGLE_ON_PRESSED_LABEL`].
pub(crate) const TOGGLE_ON_PRESSED_RULE_LABEL: &str = "egui: per call, RichText::color, the toggle's label \
     colour while on and pressed, in the state of its previous response, where a rule states \
     button.label(CHECKED).color for CHECKED, or one for PRESSED|CHECKED states it (the query \
     asking CHECKED): the theme's answer for \
     button.label(PRESSED|CHECKED).color, which this gives where its rule follows every rule for \
     CHECKED; else egui's own selected button's, visuals.selection.stroke \
     (pages::basic::toggle_on_in)";

/// The On toggle's outline at rest, its colour and width, per call in
/// `pages::basic::toggle_on_pen`.
pub(crate) const TOGGLE_ON_PEN: &str = "egui: per call, Button::stroke, the toggle's outline while \
     on at rest, where a rule states button.body(CHECKED).stroke for CHECKED, its colour laid over \
     the fill egui paints the button in (demo::pen_over); else egui's own selected button's pen, \
     which egui keeps from the widget state, visuals.widgets.inactive.bg_stroke (egui 0.37.0 \
     src/style.rs, Style::interact_selectable; pages::basic::toggle_on_pen, demo::state_pen)";

/// The On toggle's outline under the pointer, its colour and width, as [`TOGGLE_ON_PEN`].
pub(crate) const TOGGLE_ON_HOVER_PEN: &str = "egui: per call, Button::stroke, the toggle's outline \
     while on under the pointer, in the state of its previous response, where a rule states \
     button.body(CHECKED).stroke for CHECKED, or one for HOVER|CHECKED states this one (the query \
     asking CHECKED); else, or where it is stated in a form egui cannot draw, egui's own selected \
     button's pen, visuals.widgets.hovered.bg_stroke, never another state's \
     (pages::basic::toggle_on_pen, demo::stated_pen)";

/// The On toggle's outline pressed, its colour and width, as [`TOGGLE_ON_PEN`].
pub(crate) const TOGGLE_ON_PRESSED_PEN: &str = "egui: per call, Button::stroke, the toggle's outline \
     while on and pressed, in the state of its previous response, where a rule states \
     button.body(CHECKED).stroke for CHECKED, or one for PRESSED|CHECKED states this one (the \
     query asking CHECKED), none where its width is not positive; else, or where it is stated in \
     a form egui cannot draw, egui's own selected button's pen, visuals.widgets.active.bg_stroke, \
     never another state's (pages::basic::toggle_on_pen, demo::stated_pen)";

/// The push button's outline at rest on the On toggle, which egui's selected button keeps where
/// the theme states no checked stroke, and which the checked pen covers where it does.
pub(crate) const TOGGLE_ON_REST_EDGE: &str = "egui: the toggle while on is egui's selected button, \
     which keeps the widget state's pen, visuals.widgets.inactive.bg_stroke at rest, which \
     next::apply sets from this (mapping.toml, button.border.color; egui 0.37.0 src/style.rs, \
     Style::interact_selectable), where no rule states button.body(CHECKED).stroke for CHECKED; \
     where one does, Button::stroke draws that pen instead, so this colour is not drawn on it: \
     windows-11 states it, ToggleButtonBorderBrushChecked's three sides, \
     ControlStrokeColorOnAccentDefault #14FFFFFF, the same under the pointer and none pressed \
     (pages::basic::toggle_on_pen)";

/// The rows of tabs `demo::tab_bar` draws, each bar's element and its tabs': the Basic page's,
/// the page tabs and the inspector's tabs.
pub(crate) const TAB_ELEMENTS: [&str; 9] = [
    "basic.tabs.bar",
    "basic.tabs.one",
    "basic.tabs.two",
    "chrome.page_tabs",
    "chrome.page_tabs.basic",
    "chrome.page_tabs.buttons",
    "chrome.side_panel.inspector_tabs",
    "chrome.side_panel.inspector_tabs.widget",
    "chrome.side_panel.inspector_tabs.theme",
];

/// The homes a row of tabs draws per call from the installed 0.8.0 theme
/// (`demo::TabLook::next`), each with its route: what egui's `Button` and the tab scope's style
/// carry none of.
pub(crate) const TAB_PER_CALL_ROUTES: [(&str, &str); 6] = [
    (
        "tab.tab(NONE).min_size[0]",
        "egui: per call, the least tab width, which egui's Button never reads from the style \
         (demo::TabLook::next, demo::tab_bar)",
    ),
    (
        "tab.indicator(SELECTED).fill",
        "egui: per call, the colour of the selected tab's line (demo::next_indicator_line, \
         painted by demo::tab_bar)",
    ),
    (
        "tab.indicator(SELECTED).min_size[1]",
        "egui: per call, the thickness of the selected tab's line, the indicator's exact height \
         (demo::next_indicator_line)",
    ),
    (
        "tab.indicator(SELECTED).max_size[1]",
        "egui: per call, not read: the selected tab's line is min_size[1] thick, the \
         indicator's exact height (demo::next_indicator_line)",
    ),
    (
        "tab.indicator_side",
        "egui: per call, the edge of the selected tab its line lies along \
         (demo::next_indicator_line)",
    ),
    (
        "tab.active_outward_corner_radius",
        "egui: per call, the selected tab's outward corners, in its fill and outline \
         (demo::next_outward_radius, painted by demo::tab_bar)",
    ),
];

/// The fields of a tab's label a row of tabs applies per call (`demo::next_tab_label`), each with
/// its route after "per call, ": its weight, which no `TextStyle` carries.
pub(crate) const TAB_LABEL_PER_CALL: [(&str, &str); 1] = [(
    "tab.label(NONE).weight",
    "the face the showcase registers at the stated weight where the fonts hold one, else the \
     body's (demo::next_tab_label)",
)];

/// The homes of a switch's label and of its place the companion crate's switch reads from the
/// installed 0.8.0 theme at its home (`pages::basic::switches`), which no `mapping.toml` row
/// holds, each with its route.
pub(crate) const SWITCH_LABEL_ROUTES: [(&str, &str); 5] = [
    (
        "switch.label_gap",
        "egui: native-theme-egui-widgets Switch (docs/archive/todo_egui-widgets-spec.md §4.1) \
         paints it: the space between the track and the label where stated, else egui's own \
         spacing.icon_spacing; the two at the row's ends where switch.label_gap_stretches is \
         true (native-theme-egui-widgets switch.rs, Geometry::next_of, switch_ui)",
    ),
    (
        "switch.label(NONE)",
        "egui: native-theme-egui-widgets Switch (docs/archive/todo_egui-widgets-spec.md §4.1) \
         paints it: the label at its size_px where stated, lowered to the atlas's \
         text_size_cap, else egui's own, in egui's \
         Proportional family, which holds the body text's face; a stated family, weight or \
         slant is not drawn (native-theme-egui-widgets switch.rs, Geometry::next_of, Switch::ui)",
    ),
    (
        "switch.label(NONE).color",
        "egui: native-theme-egui-widgets Switch (docs/archive/todo_egui-widgets-spec.md §4.1) \
         paints it: the off label's colour where stated, else the body text's, \
         typography.body.color (native-theme-egui-widgets switch.rs, Look::next_of)",
    ),
    (
        "switch.label(CHECKED).color",
        "egui: native-theme-egui-widgets Switch (docs/archive/todo_egui-widgets-spec.md §4.1) \
         paints it: the on label's colour where a rule for CHECKED states it, else the off \
         label's, switch.label(NONE).color where stated, else the body text's, \
         typography.body.color (native-theme-egui-widgets switch.rs, Look::next_of)",
    ),
    (
        "switch.label(DISABLED|CHECKED).color",
        "egui: native-theme-egui-widgets Switch (docs/archive/todo_egui-widgets-spec.md §4.1) \
         paints it, a disabled on switch's label, where the theme states it for DISABLED \
         (switch.label(DISABLED), or switch.label(DISABLED|CHECKED) asking DISABLED); else \
         colors.disabled_text (native-theme-egui-widgets switch.rs, Look::next_of)",
    ),
];

/// The line under a listed element's row: the call that applies it where a page or the chrome
/// paints it per call ([`PER_CALL_ROUTES`], whose `row_` ids name every row by prefix and whose
/// other ids name one element each; [`TAB_PER_CALL_ROUTES`] on [`TAB_ELEMENTS`]), the companion
/// crate's switch where it reads a home of its label ([`SWITCH_LABEL_ROUTES`]), the toolbar
/// scope or the showcase where it applies a tool button's own ([`TOOL_BUTTON_ROUTES`], on a
/// tool button or its icon, [`TOOL_BUTTONS`]), the chrome where it lays the side-panel toggle's
/// own looks or icon colour ([`TOGGLE_ROUTES`], on the toggle or its icon, [`TOGGLE`]), the
/// flat button's own states on the other flat buttons ([`FLAT_BUTTON_ROUTES`], [`FLAT_BUTTONS`]), else
/// [`how_line`]'s, from the `mapping.toml` row that holds the home, a tab's label with its
/// fields [`TAB_LABEL_PER_CALL`] names by their call.
pub(crate) fn listed_how(manifest: &Manifest, element: &str, home: &str, seams: &[Seam]) -> String {
    let names = |id: &str| {
        if id.ends_with("row_") {
            element.starts_with(id)
        } else {
            element == id
        }
    };
    let tab = TAB_ELEMENTS.contains(&element);
    let tab_route = tab
        .then(|| TAB_PER_CALL_ROUTES.iter().find(|(h, _)| *h == home))
        .flatten()
        .map(|(_, line)| *line);
    let switch_route = SWITCH_LABEL_ROUTES
        .iter()
        .find(|(h, _)| *h == home)
        .map(|(_, line)| *line);
    let tool = TOOL_BUTTONS.iter().any(|id| button_or_icon(element, id));
    let tool_route = tool
        .then(|| TOOL_BUTTON_ROUTES.iter().find(|(h, _)| *h == home))
        .flatten()
        .map(|(_, line)| *line);
    let toggle_route = button_or_icon(element, TOGGLE)
        .then(|| TOGGLE_ROUTES.iter().find(|(h, _)| *h == home))
        .flatten()
        .map(|(_, line)| *line);
    let flat_route = FLAT_BUTTONS
        .contains(&element)
        .then(|| FLAT_BUTTON_ROUTES.iter().find(|(h, _)| *h == home))
        .flatten()
        .map(|(_, line)| *line);
    PER_CALL_ROUTES
        .iter()
        .find(|(id, h, _)| names(id) && *h == home)
        .map(|(_, _, line)| *line)
        .or(tab_route)
        .or(switch_route)
        .or(tool_route)
        .or(toggle_route)
        .or(flat_route)
        .map_or_else(
            || {
                if tab {
                    how_line_with(manifest, home, seams, &TAB_LABEL_PER_CALL)
                } else {
                    how_line(manifest, home, seams)
                }
            },
            str::to_string,
        )
}

/// The rows of a `mapping.toml` row for an instance drawn with `seams` under `preset`: one per
/// home the row has on it ([`Row::homes_on`]), the home, its value in `shown` in the listed
/// rows' text, a text style's family as `family` labels it, and the row's route.
pub(crate) fn manifest_rows(
    row: &Row,
    (shown, preset): (&InfoTheme, &str),
    family: &dyn Fn(&str, &StatedFont) -> String,
    seams: &[Seam],
) -> Vec<InfoRow<Color>> {
    row.homes_on(preset)
        .iter()
        .map(|home| {
            let (value, colour) = shown.value(home, family);
            InfoRow {
                leaf: home.clone(),
                value,
                colour,
                how: Route::of(row, home, seams).line(),
            }
        })
        .collect()
}

/// What Widget Info shows for the hovered instance (R11 §B), in the shared crate's form
/// (`info::ListedInfo`): its title, a "Theme" row per home, and what egui draws there that the
/// theme states nothing for.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct InfoView(ListedInfo<Color>);

impl std::ops::Deref for InfoView {
    type Target = ListedInfo<Color>;

    fn deref(&self) -> &ListedInfo<Color> {
        &self.0
    }
}

impl InfoView {
    /// The view of `shown`: for an element of `docs/showcase-elements.toml`, its name and state
    /// and a row per home of the list, in its order, each value read from the installed 0.8.0
    /// theme `info` as the three showcases read it (`next::info::listed_info`); for any other
    /// instance, its kind and a row per home of each `mapping.toml` row of its seams
    /// ([`manifest_rows`]). Each text style's family is named by the family egui draws
    /// (`demo::DrawnFamilies::home_label`, the fonts read from `ctx`, the atlas's notes from
    /// `atlas`).
    pub(crate) fn of(
        shown: &Shown,
        elements: &[crate::elements::ShowcaseElement],
        manifest: &Manifest,
        (info, preset): (&InfoTheme, &str),
        (atlas, ctx): (&OldAtlas, &egui::Context),
    ) -> Self {
        let families = crate::demo::DrawnFamilies::of(ctx, atlas);
        let family = |path: &str, font: &StatedFont| {
            families.as_ref().map_or_else(
                || "error: the atlas holds no 0.8.0 light theme, whose families egui draws".into(),
                |families| families.home_label(path, font),
            )
        };
        let seams = &shown.info.seams;
        // A row's route, with each note `mapping.toml` records on a row that holds its home or
        // a field of it, once, and each exception such a row records for the preset shown.
        let with_exceptions = |line: String, home: &str| {
            let rows = || manifest.rows.iter().filter(|r| r.holds_within(home));
            let mut notes: Vec<&str> = rows().filter_map(|r| r.note.as_deref()).collect();
            notes.dedup();
            let line = notes
                .into_iter()
                .fold(line, |line, note| format!("{line}; {note}"));
            rows()
                .flat_map(|r| r.exceptions.iter())
                .filter(|(p, _)| p == preset)
                .fold(line, |line, (_, why)| format!("{line}; on {preset}: {why}"))
        };
        let listed = shown
            .info
            .element
            .as_deref()
            .and_then(|id| elements.iter().find(|e| e.id == id));
        let not_themeable = shown
            .info
            .not_themeable
            .iter()
            .map(|(what, note)| format!("{what}: {note}"))
            .collect();
        if let Some(element) = listed {
            return InfoView(next_info::listed_info(
                element,
                &info.theme,
                &info.json,
                family,
                |home| {
                    let how = listed_how(manifest, &element.id, home, seams);
                    with_exceptions(how, home)
                },
                not_themeable,
            ));
        }
        let mut rows: Vec<&Row> = seams.iter().flat_map(|s| manifest.rows_for(s)).collect();
        rows.sort_by(|a, b| a.leaf.cmp(&b.leaf));
        rows.dedup_by(|a, b| a.leaf == b.leaf);
        let mut shown_rows: Vec<InfoRow<Color>> = Vec::new();
        for row in rows {
            for mut home in manifest_rows(row, (info, preset), &family, seams) {
                if shown_rows.iter().any(|r| r.leaf == home.leaf) {
                    continue;
                }
                home.how = with_exceptions(home.how, &home.leaf);
                shown_rows.push(home);
            }
        }
        let rows = shown_rows;
        InfoView(ListedInfo {
            title: shown.info.kind.to_string(),
            rows,
            not_themeable,
        })
    }

    /// What Copy puts on the clipboard: the text the panel shows, in the three showcases'
    /// form (`ListedInfo::copy_text`).
    pub(crate) fn text(&self) -> String {
        self.0.copy_text()
    }
}

/// The values of a row's homes on `preset` in `shown` ([`Row::homes_on`]), each family as the
/// theme states it: the value alone for one home, else each `<home> <value>`, joined by "; ";
/// and the first colour among them.
fn homes_text(row: &Row, (shown, preset): (&InfoTheme, &str)) -> (String, Option<Color>) {
    let values: Vec<(&String, (String, Option<Color>))> = row
        .homes_on(preset)
        .iter()
        .map(|home| (home, shown.value(home, &as_stated)))
        .collect();
    let colour = values.iter().find_map(|(_, (_, colour))| *colour);
    let text = match values.as_slice() {
        [] => "(no home)".to_string(),
        [(_, (value, _))] => value.clone(),
        many => many
            .iter()
            .map(|(home, (value, _))| format!("{home} {value}"))
            .collect::<Vec<_>>()
            .join("; "),
    };
    (text, colour)
}

/// The Theme Map's four cells of one row (§10.4, the Theme Map row): the leaf, the values of
/// its homes on the 0.8.0 model now (`homes_text`), the verdict, and the sinks with their scope
/// or surface joined by "; " — for an `unmappable` row its `sub_tag` and `upstream` instead, for
/// a `tested_by` row its route; and beside them the first colour among the homes' values on
/// `preset` in `shown`, as egui's colour, for the row's swatch, `None` where no home holds a
/// colour.
pub(crate) fn row_cells(
    row: &Row,
    shown: (&InfoTheme, &str),
) -> ([String; 4], Option<egui::Color32>) {
    let (value, colour) = homes_text(row, shown);
    let sinks = if row.verdict == Verdict::Unmappable {
        format!(
            "{}: {}",
            row.sub_tag.as_deref().unwrap_or_default(),
            row.upstream.as_deref().unwrap_or_default()
        )
    } else if let Some(check) = &row.checked_by {
        format!("per call; {check}")
    } else {
        row.sinks
            .iter()
            .map(sink_text)
            .collect::<Vec<_>>()
            .join("; ")
    };
    (
        [
            row.leaf.clone(),
            value,
            row.verdict.name().to_string(),
            sinks,
        ],
        colour.map(native_theme_egui::next::to_color32),
    )
}

/// Text in the side panel's font (`sidebar.item_label(NONE)`: `Body` in the sidebar scope the inspector is
/// drawn in), wrapping at the width it is given.
fn note(ui: &mut egui::Ui, text: impl Into<String>, weak: bool) -> egui::Response {
    let mut text = egui::RichText::new(text);
    if weak {
        text = text.weak();
    }
    ui.add(egui::Label::new(text).wrap())
}

/// A name over its value, one label, the gpui showcase's inspector `row` and note line
/// (`showcase-gpui/inspector.rs:546-554`, `:930-931`): the name in the weak text colour
/// (gpui's `muted_foreground` is `defaults.muted_color`, as egui's `weak_text_color` is, parity
/// rule R3), the value and each line after it in the text colour, all in the side panel's font
/// (`sidebar.item_label(NONE)`, `Body` in the sidebar scope), wrapping.
pub(crate) fn key_value(ui: &mut egui::Ui, key: &str, lines: &[String]) -> egui::Response {
    let font = egui::TextStyle::Body.resolve(ui.style());
    let (weak, strong) = (ui.visuals().weak_text_color(), ui.visuals().text_color());
    let mut job = egui::text::LayoutJob::default();
    job.append(key, 0.0, egui::TextFormat::simple(font.clone(), weak));
    for line in lines {
        job.append("\n", 0.0, egui::TextFormat::simple(font.clone(), strong));
        job.append(line, 0.0, egui::TextFormat::simple(font.clone(), strong));
    }
    ui.add(egui::Label::new(job).wrap())
}

/// A colour's cell in a one-line table column, the Theme Map's Value: a `SWATCH_SIZE` square of
/// the colour ([`paint_swatch`]), then `text` beside it in the cell's own wrap mode, so a column
/// narrower than the two cuts the text short with egui's ellipsis, its whole text on hover
/// (`pages::data::one_line_cells`), where a wrapping label with the swatch as its first row's
/// leading space moved a text with no room on that row onto rows the one-line cell clips away
/// (epaint 0.36.2 `src/text/text_layout.rs:668-684`, `line_break`). The two are laid out in the
/// cell's whole height, centred down it as a plain cell's text is
/// (`pages::data::centred_cells`), where `ui.horizontal` laid them in a row `interact_size.y`
/// tall (egui 0.36.2 `src/ui.rs:2376-2379`), at some text scales taller than the table's row,
/// which put the swatch and its text below the row's centre.
pub(crate) fn swatch_cell(
    ui: &mut egui::Ui,
    r: Option<&ResolvedTheme>,
    colour: egui::Color32,
    text: egui::RichText,
) -> egui::Response {
    ui.with_layout(crate::pages::data::centred_cells(), |ui| {
        let size = egui::Vec2::splat(crate::SWATCH_SIZE);
        let (square, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        paint_swatch(ui, r, square, colour);
        ui.label(text);
    })
    .response
}

/// `rect` filled with `colour` and framed as the theme frames a plain box, of the installed
/// 0.8.0 theme `r`: `layout.border_width` wide, `layout.corner_radius` round, in
/// `colors.window.border`, as the gpui showcase's `demo_frame` frames it; where the theme states
/// no border colour, or without an installed 0.8.0 theme, egui's own line of a plain box,
/// `widgets.noninteractive.bg_stroke` (spec §10, rule 0), and no rounding without a theme.
fn paint_swatch(ui: &egui::Ui, r: Option<&ResolvedTheme>, rect: egui::Rect, colour: egui::Color32) {
    let own = ui.visuals().widgets.noninteractive.bg_stroke;
    let (radius, stroke) = r.map_or((egui::CornerRadius::ZERO, own), |r| {
        let line = r
            .colors
            .window
            .border
            .map_or(own.color, native_theme_egui::next::to_color32);
        (
            native_theme_egui::convert::to_corner_radius(
                egui::CornerRadius::ZERO,
                r.layout.corner_radius,
            ),
            egui::Stroke::new(r.layout.border_width, line),
        )
    });
    ui.painter()
        .rect(rect, radius, colour, stroke, egui::StrokeKind::Inside);
}

/// The text Widget Info shows before anything is hovered, the three showcases' own.
pub(crate) const INFO_HINT: &str = "Hover any widget to see what the theme sets on it.";

/// The fonts and colours Widget Info draws with (R11 §B), of the installed 0.8.0 theme: the
/// `Body` style of the sidebar scope the inspector is drawn in, the side panel's item text, in
/// `sidebar.item_label(NONE).color`, the title and the section names at `SEMIBOLD_WEIGHT`, the
/// lines under the rows in `colors.window.foreground_muted`; every line one line tall, the
/// text's size × `typography.line_height_multiplier`. Where the theme states no colour, or
/// without an installed 0.8.0 theme, egui's own text and weak text colours of the scope (spec
/// §10, rule 0); without one, egui's own line, the font's row height.
struct InfoText {
    regular: egui::FontId,
    semibold: egui::FontId,
    line: f32,
    colour: egui::Color32,
    muted: egui::Color32,
}

impl InfoText {
    fn of(ui: &egui::Ui, r: Option<&ResolvedTheme>) -> Self {
        let regular = egui::TextStyle::Body.resolve(ui.style());
        let visuals = ui.visuals();
        let (own_colour, own_muted) = (visuals.text_color(), visuals.weak_text_color());
        let line = r.map_or_else(
            || ui.fonts_mut(|f| f.row_height(&regular)),
            |r| regular.size * r.typography.line_height_multiplier,
        );
        let colour = r
            .and_then(|r| {
                native_theme_egui::next::text_color_of(&r.sidebar.item_label(WidgetState::NONE))
            })
            .unwrap_or(own_colour);
        let muted = r
            .and_then(|r| r.colors.window.foreground_muted)
            .map_or(own_muted, native_theme_egui::next::to_color32);
        Self {
            semibold: crate::demo::semibold_font(ui, regular.size),
            line,
            regular,
            colour,
            muted,
        }
    }

    /// `text` laid out on one line, never wrapped: a line wider than the panel is clipped.
    fn galley(
        &self,
        ui: &egui::Ui,
        text: &str,
        font: &egui::FontId,
        colour: egui::Color32,
    ) -> std::sync::Arc<egui::Galley> {
        let mut job =
            egui::text::LayoutJob::simple_singleline(text.to_string(), font.clone(), colour);
        if let Some(section) = job.sections.first_mut() {
            section.format.line_height = Some(self.line);
        }
        ui.fonts_mut(|f| f.layout_job(job))
    }

    /// `text` at the cursor, one line tall and as wide as it is, clipped at the room there is;
    /// the rectangle it paints (`painted`).
    fn line(
        &self,
        ui: &mut egui::Ui,
        text: &str,
        font: &egui::FontId,
        colour: egui::Color32,
    ) -> egui::Rect {
        let galley = self.galley(ui, text, font, colour);
        let size = egui::vec2(galley.size().x.min(ui.available_width()), self.line);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
        response.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Label, true, text));
        let shown = painted(ui, rect, &galley);
        ui.painter()
            .with_clip_rect(rect)
            .galley(rect.min, galley, colour);
        shown
    }
}

/// The rectangle `galley` paints when drawn at `rect`'s corner and clipped to its width: epaint
/// draws a galley on the pixel its corner rounds to (`round_text_to_pixels`, on by default,
/// `epaint/src/tessellator.rs:2024-2025`) and lays each row out a whole number of pixels tall
/// (`epaint/src/text/text_layout.rs:971`), so a line one line box tall where that box is a
/// fraction of a pixel (`sidebar.item_label(NONE)`'s size × `typography.line_height_multiplier`) paints the whole row
/// from the rounded corner.
fn painted(ui: &egui::Ui, rect: egui::Rect, galley: &egui::Galley) -> egui::Rect {
    use egui::emath::GuiRounding as _;
    egui::Rect::from_min_size(
        rect.min.round_to_pixels(ui.pixels_per_point()),
        egui::vec2(rect.width(), galley.size().y),
    )
}

/// The Widget tab (R11 §B), laid out as the three showcases lay it out: the title flush left
/// and Copy flush right on one row, `layout.section_gap` below it the "Theme" section's name,
/// then each row `layout.widget_gap` below the one before — a colour's swatch, a square one line
/// tall, `layout.widget_gap` before the text `<leaf> <value>`, and under the text the muted line
/// of how egui applies it — and, `layout.section_gap` below, "Not themeable" and its lines. A
/// gap the theme leaves unstated is none. Copy is a flat button, padded by
/// `button.body(NONE).padding` and rounded by its corners, unframed at rest, and under the
/// pointer and while pressed or focused filled, outlined and lettered as
/// `button.body(HOVER|PRESSED; variant=Flat)` and its label state, the outline focused
/// `FOCUS_VISIBLE`'s, where a rule states them for the state, else egui's own
/// (`next::flat_button`; spec §10, rule 0). All of the installed 0.8.0 theme `r`. The content records nothing (§10.4); it
/// places the Widget Info elements of `docs/showcase-elements.toml`.
pub(crate) fn widget_tab(
    ui: &mut egui::Ui,
    r: Option<&ResolvedTheme>,
    view: Option<Result<InfoView, String>>,
    reg: &mut crate::demo::Registry,
) {
    use native_theme_egui::next as nx;
    let text = InfoText::of(ui, r);
    ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
    let view = match view {
        None => {
            let rect = text.line(ui, INFO_HINT, &text.regular, text.muted);
            reg.place(ui, "chrome.info.hint", rect);
            return;
        }
        Some(Err(error)) => {
            text.line(
                ui,
                &format!("Widget Info is unavailable: {error}"),
                &text.regular,
                text.muted,
            );
            return;
        }
        Some(Ok(view)) => view,
    };
    let widget_gap = r.and_then(|r| r.layout.widget_gap).unwrap_or_default();
    let section_gap = r.and_then(|r| r.layout.section_gap).unwrap_or_default();

    // The title row: the title and Copy, centred on each other.
    let title = text.galley(ui, &view.title, &text.semibold, text.colour);
    let copy_text = text.galley(ui, "Copy", &text.regular, text.colour);
    let [top, right, bottom, left] = r
        .map_or([None; 4], |r| {
            nx::padding_of(&r.button.body(WidgetState::NONE))
        })
        .map(Option::unwrap_or_default);
    let copy_size = egui::vec2(copy_text.size().x + left + right, text.line + top + bottom);
    let row_height = copy_size.y.max(text.line);
    let (row, title_row) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );
    title_row.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Label, true, &view.title));
    let title_rect = egui::Rect::from_min_size(
        egui::pos2(row.left(), row.center().y - 0.5 * text.line),
        egui::vec2(title.size().x, text.line),
    );
    let title_shown = painted(ui, title_rect, &title);
    ui.painter().galley(title_rect.min, title, text.colour);
    reg.place(ui, "chrome.info.title", title_shown);
    let copy_rect = egui::Rect::from_min_size(
        egui::pos2(
            row.right() - copy_size.x,
            row.center().y - 0.5 * copy_size.y,
        ),
        copy_size,
    );
    let copy = ui.interact(
        copy_rect,
        ui.scope_id().with("info copy"),
        egui::Sense::click(),
    );
    copy.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, "Copy"));
    // A flat button, unframed at rest: under the pointer and pressed or focused, the flat
    // button's states laid over the scope's (`next::flat_button`, as `demo::flat` lays them),
    // egui's `active` entry for a press and a keyboard focus alike. The notes are the flat
    // button's homes', which the recorded flat buttons list; the Widget tab records nothing.
    let mut visuals = ui.visuals().clone();
    if let Some(r) = r {
        let _notes = nx::flat_button(r, ui.ctx().theme(), &mut visuals);
    }
    let widgets = &visuals.widgets;
    let entry = if copy.is_pointer_button_down_on() || copy.has_focus() {
        Some(&widgets.active)
    } else if copy.hovered() {
        Some(&widgets.hovered)
    } else {
        None
    };
    let colour = entry.map_or(text.colour, egui::style::WidgetVisuals::text_color);
    if let Some(entry) = entry {
        let radius = r
            .and_then(|r| nx::radius_of(&r.button.body(WidgetState::NONE)))
            .map_or(widgets.inactive.corner_radius, |radius| {
                native_theme_egui::convert::to_corner_radius(egui::CornerRadius::ZERO, radius)
            });
        ui.painter().rect(
            copy_rect,
            radius,
            entry.weak_bg_fill,
            entry.bg_stroke,
            egui::StrokeKind::Inside,
        );
    }
    let copy_label = text.galley(ui, "Copy", &text.regular, colour);
    let at = egui::pos2(
        copy_rect.left() + left,
        copy_rect.center().y - 0.5 * text.line,
    );
    ui.painter().galley(at, copy_label, colour);
    reg.place(ui, "chrome.info.copy", copy_rect);
    if copy.clicked() {
        ui.ctx().copy_text(view.text());
    }

    ui.add_space(section_gap);
    let section = text.line(ui, "Theme", &text.semibold, text.colour);
    reg.place(ui, "chrome.info.section.theme", section);
    for (index, row) in view.rows.iter().enumerate() {
        let [value_line, _] = row.lines();
        ui.add_space(widget_gap);
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), text.line + text.line),
            egui::Sense::hover(),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::Label, true, row.lines().join("\n"))
        });
        let mut x = rect.left();
        let swatch = row.colour.map(|colour| {
            let square = egui::Rect::from_min_size(rect.min, egui::Vec2::splat(text.line));
            paint_swatch(ui, r, square, nx::to_color32(colour));
            x += text.line + widget_gap;
            square
        });
        let line = text.galley(ui, &value_line, &text.regular, text.colour);
        // A line wider than the row is clipped at its end.
        let room = (rect.right() - x).max(0.0);
        let line_rect = egui::Rect::from_min_size(
            egui::pos2(x, rect.top()),
            egui::vec2(line.size().x.min(room), text.line),
        );
        let line_shown = painted(ui, line_rect, &line);
        ui.painter()
            .with_clip_rect(line_rect)
            .galley(line_rect.min, line, text.colour);
        let how = text.galley(ui, &row.how, &text.regular, text.muted);
        let how_rect = egui::Rect::from_min_size(
            egui::pos2(x, line_rect.bottom()),
            egui::vec2(how.size().x.min(room), text.line),
        );
        let how_shown = painted(ui, how_rect, &how);
        ui.painter()
            .with_clip_rect(how_rect)
            .galley(how_rect.min, how, text.muted);
        if index == 0 {
            // The row across the panel's text column, from the top of what it paints (its
            // swatch and its first line) to the foot of its second line.
            let top = swatch.map_or(line_shown.top(), |square| {
                square.top().min(line_shown.top())
            });
            let shown = egui::Rect::from_x_y_ranges(rect.x_range(), top..=how_shown.bottom());
            reg.place(ui, "chrome.info.row_1", shown);
            if let Some(square) = swatch {
                reg.place(ui, "chrome.info.row_1.swatch", square);
            }
            reg.place(ui, "chrome.info.row_1.text", line_shown);
            reg.place(ui, "chrome.info.row_1.how", how_shown);
        }
    }
    if !view.not_themeable.is_empty() {
        ui.add_space(section_gap);
        text.line(ui, "Not themeable", &text.semibold, text.colour);
        for line in &view.not_themeable {
            ui.add_space(widget_gap);
            text.line(ui, line, &text.regular, text.muted);
        }
    }
}

/// The Theme tab: `name()`, `os_mode()`, `accessibility()`, every `Note`, the atlas's, the
/// companion crate's widgets' (`native_theme_egui_widgets::next_fills_unmapped`) and the
/// showcase's drawing sites' (`demo::drawn_fills_unmapped`), and each face the
/// showcase did not add at another weight (`demo::Semibold::invalid`), the
/// fonts of the installed 0.8.0 theme, `typography.body` and `typography.monospace`, by the
/// families egui draws (`demo::DrawnFamilies`) in their defined units (§8.7), the window-frame
/// facts of §10.4, and the manifest's `[unwritten]` table; the theme is the one the pages draw
/// (`OldAtlas::next_shown`).
pub(crate) fn theme_tab(
    ui: &mut egui::Ui,
    atlas: &OldAtlas,
    semibold: &crate::demo::Semibold,
    manifest: &Result<Manifest, String>,
) {
    let r = atlas.next_shown(ui.ctx());
    // The section names as the Widget tab draws its own (`widget_tab`): one heading style in
    // the panel, whichever tab is shown.
    let text = InfoText::of(ui, r);
    let section = |ui: &mut egui::Ui, title: &str| {
        text.line(ui, title, &text.semibold, text.colour);
    };
    let row = |ui: &mut egui::Ui, key: &str, value: String| {
        key_value(ui, key, &[value]);
    };
    // Sections and rows as the gpui showcase's Theme tab (`showcase-gpui/inspector.rs:198-242`).
    section(ui, "Theme");
    row(ui, "name", atlas.name().to_string());
    row(ui, "os_mode", format!("{:?}", atlas.os_mode()));
    let prefs = atlas.accessibility();
    row(
        ui,
        "text_scaling_factor",
        prefs.text_scaling_factor.to_string(),
    );
    row(ui, "reduce_motion", prefs.reduce_motion.to_string());
    row(ui, "high_contrast", prefs.high_contrast.to_string());
    row(
        ui,
        "reduce_transparency",
        prefs.reduce_transparency.to_string(),
    );
    section(ui, "Notes");
    // The companion crate's widgets and the showcase's own drawing sites report a fill they
    // draw egui's own in place of on the atlas's channel
    // (`native_theme_egui_widgets::next_fills_unmapped`, `demo::drawn_fills_unmapped`); one
    // the atlas holds too, a fill its own styles take, is listed once.
    let widget_notes = r.map(|r| {
        let mut notes = native_theme_egui_widgets::next_fills_unmapped(r);
        notes.extend(crate::demo::drawn_fills_unmapped(r));
        notes.retain(|n| !atlas.notes().contains(n));
        notes
    });
    let widget_notes = widget_notes.as_deref().unwrap_or_default();
    if atlas.notes().is_empty() && widget_notes.is_empty() && semibold.invalid().is_empty() {
        note(ui, "none", true);
    }
    for note in atlas.notes().iter().chain(widget_notes) {
        let kind: String = format!("{note:?}")
            .chars()
            .take_while(|c| c.is_alphanumeric())
            .collect();
        row(ui, &kind, crate::demo::note_text(note));
    }
    // The showcase's own faces at other weights that epaint cannot load (`Semibold::add_face`).
    for (family, weight) in semibold.invalid() {
        row(
            ui,
            "FontDataInvalid",
            format!(
                "the showcase's face of {family:?} at weight {weight} failed epaint's parse and \
                 is not added; text at weight {weight} is drawn in the body's face"
            ),
        );
    }
    section(ui, "Fonts");
    match r.zip(crate::demo::DrawnFamilies::of(ui.ctx(), atlas)) {
        Some((r, families)) => {
            let (body, mono) = (&r.typography.body, &r.typography.monospace);
            row(ui, "font_family", families.label(&body.family, false));
            row(ui, "font_size", next_status::defined_size(body));
            row(ui, "mono_font_family", families.label(&mono.family, true));
            row(ui, "mono_font_size", next_status::defined_size(mono));
        }
        None => {
            note(ui, "the atlas holds no 0.8.0 theme", true);
        }
    }
    section(ui, "Window");
    row(
        ui,
        "decorations",
        "on: the showcase asks for the OS's frame (ViewportBuilder::with_decorations(true))"
            .to_string(),
    );
    row(
        ui,
        "frame",
        "the OS's, or winit's Adwaita-styled one where the compositor leaves the frame to the application; the showcase draws no title bar (§10.4)"
            .to_string(),
    );
    section(ui, "Fields left at egui's own value");
    match manifest {
        Ok(manifest) => {
            for (path, why) in &manifest.unwritten {
                row(ui, path, why.clone());
            }
        }
        Err(error) => {
            note(ui, format!("the manifest did not parse: {error}"), true);
        }
    }
}
