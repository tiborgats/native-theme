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

impl Route {
    fn of(row: &Row, seams: &[Seam]) -> Self {
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
        // The instance's own cells and frames first; a role's cell carries what the role does
        // not write from the base style (§3.4).
        let mut on: Vec<(&Seam, &Sink)> = seams
            .iter()
            .flat_map(|seam| {
                row.sinks
                    .iter()
                    .filter(move |s| sink_on(s, seam))
                    .map(move |s| (seam, s))
            })
            .collect();
        if on.is_empty() {
            on = row
                .sinks
                .iter()
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
            None => match row.sinks.first() {
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
        return Route::of(row, seams).line();
    }
    let part = |name: &'static str| {
        let field = format!("{home}.{name}");
        if let Some((_, how)) = per_call.iter().find(|(h, _)| *h == field) {
            let short = format!("per call, {how}");
            return Some((name, format!("egui: {short}"), short));
        }
        manifest.rows.iter().find(|r| r.holds(&field)).map(|row| {
            let route = Route::of(row, seams);
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

/// The homes a page applies per call where `mapping.toml`'s row names a style field the
/// connector writes, or none, by element id (a prefix, for the rows) and home: the List's and
/// the table's frames, which `pages::basic::list_frame` paints, the rows whose fills
/// `pages::basic::inner_corners` rounds where they meet a frame, the selected rows' unfocused
/// look and a hovered list row's outline, which `pages::basic::RowLook::of` takes, the hover
/// outline the Basic table's rows never take, and the menu bar's fill, which
/// `chrome::menu_panel` paints.
const PER_CALL_ROUTES: &[(&str, &str, &str)] = &[
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
];

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

/// The line under a listed element's row: the call that applies it where a page or the chrome
/// paints it per call ([`PER_CALL_ROUTES`], whose `row_` ids name every row by prefix and whose
/// other ids name one element each; [`TAB_PER_CALL_ROUTES`] on [`TAB_ELEMENTS`]), else
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
    PER_CALL_ROUTES
        .iter()
        .find(|(id, h, _)| names(id) && *h == home)
        .map(|(_, _, line)| *line)
        .or(tab_route)
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
    let how = Route::of(row, seams).line();
    row.homes_on(preset)
        .iter()
        .map(|home| {
            let (value, colour) = shown.value(home, family);
            InfoRow {
                leaf: home.clone(),
                value,
                colour,
                how: how.clone(),
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

/// The Theme tab: `name()`, `os_mode()`, `accessibility()`, every `Note` and each face the
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
    if atlas.notes().is_empty() && semibold.invalid().is_empty() {
        note(ui, "none", true);
    }
    for note in atlas.notes() {
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
