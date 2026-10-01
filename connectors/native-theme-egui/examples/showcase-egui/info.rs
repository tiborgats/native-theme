//! Widget Info, generated from `mapping.toml` (spec §10.4, §13.1).

use std::str::FromStr;

use native_theme_egui::ThemeAtlas;

use crate::demo::{Seam, Shown};

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
    pub verdict: Verdict,
    pub sinks: Vec<Sink>,
    pub tested_by: Option<String>,
    pub sub_tag: Option<String>,
    pub upstream: Option<String>,
    pub exceptions: Vec<(String, String)>,
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
            rows.push(Row {
                leaf: key,
                verdict,
                sinks,
                tested_by: string("tested_by"),
                sub_tag: string("sub_tag"),
                upstream: string("upstream"),
                exceptions,
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

/// `serde_json::to_value` of the theme the atlas holds for `theme`, with the
/// four `LayoutTheme` leaves under `layout` as T3 injects them (§13.1): under the
/// manifest's names, since `LayoutTheme` serialises them as `*_px` and skips a `None`.
pub(crate) fn theme_json(
    atlas: &ThemeAtlas,
    theme: egui::Theme,
) -> Result<serde_json::Value, String> {
    let mut json = serde_json::to_value(atlas.resolved_for(theme)).map_err(|e| e.to_string())?;
    let layout = atlas.layout();
    if let Some(object) = json.as_object_mut() {
        object.insert(
            "layout".to_string(),
            serde_json::json!({
                "widget_gap": layout.widget_gap,
                "container_margin": layout.container_margin,
                "window_margin": layout.window_margin,
                "section_gap": layout.section_gap,
            }),
        );
        // The two icon leaves `docs/showcase-elements.toml` names, which the atlas holds beside
        // the resolved theme: the registry's `theme_variant.icon_set` and `defaults.icon_theme`.
        object.insert(
            "theme_variant".to_string(),
            serde_json::json!({ "icon_set": atlas.icon_set().to_string() }),
        );
        if let Some(defaults) = object
            .get_mut("defaults")
            .and_then(serde_json::Value::as_object_mut)
        {
            defaults.insert(
                "icon_theme".to_string(),
                serde_json::json!(atlas.icon_theme(theme)),
            );
        }
    }
    Ok(json)
}

/// `theme_json` for each scheme, made on first use and kept until the next install: Widget
/// Info and the Theme Map read it on every pass.
#[derive(Default)]
pub(crate) struct JsonCache {
    light: std::cell::OnceCell<Result<serde_json::Value, String>>,
    dark: std::cell::OnceCell<Result<serde_json::Value, String>>,
}

impl JsonCache {
    pub(crate) fn get(
        &self,
        atlas: &ThemeAtlas,
        theme: egui::Theme,
    ) -> &Result<serde_json::Value, String> {
        let cell = match theme {
            egui::Theme::Light => &self.light,
            egui::Theme::Dark => &self.dark,
        };
        cell.get_or_init(|| theme_json(atlas, theme))
    }
}

/// The value at a dotted leaf path, every segment walked; a `defined_size` leaf ends on the
/// tagged `FontSize` object (§13.1), which prints as its JSON text.
pub(crate) fn value_at(json: &serde_json::Value, leaf: &str) -> Option<serde_json::Value> {
    let mut current = json;
    for part in leaf.split('.') {
        current = current.get(part)?;
    }
    Some(current.clone())
}

/// How a value prints: a colour as its `#rrggbb` or `#rrggbbaa` text (native-theme's `Rgba`
/// `Display`, which the JSON carries), a number as itself, `null` as "not stated — egui's own
/// value stands", anything else as its JSON text.
pub(crate) fn value_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "not stated — egui's own value stands".to_string(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        other => other.to_string(),
    }
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

/// The value at a leaf as `docs/showcase-elements.toml` spells it, in `theme_json`'s tree: a
/// `*_px` segment is the serialised name without the suffix, and `padding_<side>_px` the side of
/// the border's `padding`.
pub(crate) fn listed_value(json: &serde_json::Value, leaf: &str) -> Option<serde_json::Value> {
    let mut current = json;
    for part in leaf.split('.') {
        current = listed_step(current, part)?;
    }
    Some(current.clone())
}

fn listed_step<'a>(value: &'a serde_json::Value, part: &str) -> Option<&'a serde_json::Value> {
    if let Some(next) = value.get(part) {
        return Some(next);
    }
    let bare = part.strip_suffix("_px")?;
    if let Some(next) = value.get(bare) {
        return Some(next);
    }
    let side = bare.strip_prefix("padding_")?;
    value.get("padding")?.get(side)
}

/// The `mapping.toml` key of a leaf as `docs/showcase-elements.toml` spells it: each `*_px`
/// segment without the suffix, `padding_<side>` as `padding.<side>`.
pub(crate) fn mapping_key(leaf: &str) -> String {
    leaf.split('.')
        .map(|part| {
            let bare = part.strip_suffix("_px").unwrap_or(part);
            match bare.strip_prefix("padding_") {
                Some(side) => format!("padding.{side}"),
                None => bare.to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join(".")
}

/// A number as Rust prints the `f32` the resolved theme holds.
fn number_text(number: &serde_json::Number) -> String {
    number
        .as_f64()
        .map_or_else(|| number.to_string(), |n| format!("{}", n as f32))
}

/// A whole font: `<family> <defined size> <pt|px> <weight>`, the size as its source stated it
/// (`size` in px where it stated none), the style after the weight when it is not normal.
fn font_text(font: &serde_json::Map<String, serde_json::Value>) -> String {
    let family = font
        .get("family")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default();
    let number = |value: Option<&serde_json::Value>| match value {
        Some(serde_json::Value::Number(n)) => number_text(n),
        _ => "?".to_string(),
    };
    let size = match font.get("defined_size") {
        Some(serde_json::Value::Object(defined)) => match (defined.get("Pt"), defined.get("Px")) {
            (Some(pt), _) => format!("{} pt", number(Some(pt))),
            (None, Some(px)) => format!("{} px", number(Some(px))),
            (None, None) => format!("{} px", number(font.get("size"))),
        },
        _ => format!("{} px", number(font.get("size"))),
    };
    let weight = number(font.get("weight"));
    let mut text = format!("{family} {size} {weight}");
    if let Some(style) = font
        .get("style")
        .and_then(serde_json::Value::as_str)
        .filter(|s| *s != "normal")
    {
        text.push(' ');
        text.push_str(style);
    }
    text
}

/// How a listed leaf's value prints (the three showcases print it alike): a colour as its
/// `#rrggbb[aa]`, a `*_px` number with " px", another number as itself, a whole font as
/// [`font_text`], a text-scale entry as `<size> px <weight>`, a flag as `true` or `false`, a
/// name as itself, and a value the theme leaves unstated as "not stated".
pub(crate) fn leaf_text(leaf: &str, value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "not stated".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) if leaf.ends_with("_px") => format!("{} px", number_text(n)),
        serde_json::Value::Number(n) => number_text(n),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Object(font) if font.contains_key("family") => font_text(font),
        serde_json::Value::Object(entry) if entry.contains_key("line_height") => {
            let number = |key: &str| match entry.get(key) {
                Some(serde_json::Value::Number(n)) => number_text(n),
                _ => "?".to_string(),
            };
            format!("{} px {}", number("size"), number("weight"))
        }
        other => other.to_string(),
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
    /// No `Style` or `Frame` field: the call site applies it, as the named test checks.
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
        if let Some(test) = &row.tested_by {
            return Route::PerCall(test.clone());
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
            Route::PerCall(test) => format!("egui: per call, no Style field ({test})"),
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

/// The line under a row: how egui applies `leaf` to an instance drawn with `seams`, from its
/// `mapping.toml` row; a whole font or text-scale entry by its size's route, then the parts whose
/// route differs; the icon leaves, which no `Style` field carries, by the showcase's loader.
pub(crate) fn how_line(manifest: &Manifest, leaf: &str, seams: &[Seam]) -> String {
    if matches!(leaf, "theme_variant.icon_set" | "defaults.icon_theme") {
        return "egui: the showcase loads its icons from it (demo::role_image)".to_string();
    }
    let key = mapping_key(leaf);
    if let Some(row) = manifest.rows.iter().find(|r| r.leaf == key) {
        return Route::of(row, seams).line();
    }
    let part = |name: &'static str| {
        let key = format!("{key}.{name}");
        manifest
            .rows
            .iter()
            .find(|r| r.leaf == key)
            .map(|row| (name, Route::of(row, seams)))
    };
    let mut parts = ["size", "family", "weight", "style", "line_height"]
        .into_iter()
        .filter_map(part);
    let Some((_, main)) = parts.next() else {
        return "not reachable: mapping.toml has no row for it".to_string();
    };
    let main_short = main.short();
    let mut rest: Vec<(String, Vec<&str>)> = Vec::new();
    for (name, route) in parts {
        let short = route.short();
        if short == main_short {
            continue;
        }
        match rest.iter_mut().find(|(s, _)| *s == short) {
            Some((_, names)) => names.push(name),
            None => rest.push((short, vec![name])),
        }
    }
    let mut line = main.line();
    for (short, names) in rest {
        line.push_str(&format!("; {}: {short}", names.join(", ")));
    }
    line
}

/// The row of a `mapping.toml` row for an instance drawn with `seams`: the leaf as the manifest
/// names it, its value in `json`, and its route.
pub(crate) fn manifest_row(row: &Row, json: &serde_json::Value, seams: &[Seam]) -> InfoRow {
    let value = value_at(json, &row.leaf);
    InfoRow {
        leaf: row.leaf.clone(),
        colour: value.as_ref().and_then(swatch_colour),
        value: value.map_or_else(|| "not stated".to_string(), |v| leaf_text(&row.leaf, &v)),
        how: Route::of(row, seams).line(),
    }
}

/// One row of Widget Info's "Theme" section: the leaf, its value, a colour value's colour, and
/// how egui applies it.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct InfoRow {
    pub leaf: String,
    pub value: String,
    pub colour: Option<egui::Color32>,
    pub how: String,
}

/// What Widget Info shows for the hovered instance (R11 §B): its title, a "Theme" row per leaf,
/// and what egui draws there that the theme states nothing for.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct InfoView {
    pub title: String,
    pub rows: Vec<InfoRow>,
    pub not_themeable: Vec<String>,
}

impl InfoView {
    /// The view of `shown`: for an element of `docs/showcase-elements.toml`, its name and state
    /// and a row per leaf of the list, in its order; for any other instance, its kind and a row
    /// per `mapping.toml` row of its seams.
    pub(crate) fn of(
        shown: &Shown,
        elements: &[crate::elements::ShowcaseElement],
        manifest: &Manifest,
        (json, preset): (&serde_json::Value, &str),
    ) -> Self {
        let seams = &shown.info.seams;
        // A row's route, with each exception `mapping.toml` records for the preset shown.
        let with_exceptions = |line: String, key: &str| {
            let prefix = format!("{key}.");
            manifest
                .rows
                .iter()
                .filter(|r| r.leaf == key || r.leaf.starts_with(&prefix))
                .flat_map(|r| r.exceptions.iter())
                .filter(|(p, _)| p == preset)
                .fold(line, |line, (_, why)| format!("{line}; on {preset}: {why}"))
        };
        let listed = shown
            .info
            .element
            .as_deref()
            .and_then(|id| elements.iter().find(|e| e.id == id));
        let (title, rows) = match listed {
            Some(element) => {
                let title = match element.states.first() {
                    Some(state) => format!("{} · {state}", element.name),
                    None => element.name.clone(),
                };
                let rows = element
                    .leaves
                    .iter()
                    .map(|leaf| {
                        let value = listed_value(json, leaf);
                        InfoRow {
                            leaf: leaf.clone(),
                            colour: value.as_ref().and_then(swatch_colour),
                            value: value
                                .map_or_else(|| "not stated".to_string(), |v| leaf_text(leaf, &v)),
                            how: with_exceptions(
                                how_line(manifest, leaf, seams),
                                &mapping_key(leaf),
                            ),
                        }
                    })
                    .collect();
                (title, rows)
            }
            None => {
                let mut rows: Vec<&Row> = seams.iter().flat_map(|s| manifest.rows_for(s)).collect();
                rows.sort_by(|a, b| a.leaf.cmp(&b.leaf));
                rows.dedup_by(|a, b| a.leaf == b.leaf);
                let rows = rows
                    .into_iter()
                    .map(|row| {
                        let mut info = manifest_row(row, json, seams);
                        info.how = with_exceptions(info.how, &row.leaf);
                        info
                    })
                    .collect();
                (shown.info.kind.to_string(), rows)
            }
        };
        let not_themeable = shown
            .info
            .notes
            .iter()
            .filter(|(what, note)| {
                note.contains("not stated") || (*what, note.as_str()) == crate::demo::GHOST_NOTE
            })
            .map(|(what, note)| format!("{what}: {note}"))
            .collect();
        InfoView {
            title,
            rows,
            not_themeable,
        }
    }

    /// The lines one row prints in Copy's text: `<leaf> <value>`, then its route, indented.
    pub(crate) fn row_lines(row: &InfoRow) -> [String; 2] {
        [
            format!("{} {}", row.leaf, row.value),
            format!("  {}", row.how),
        ]
    }

    /// What Copy puts on the clipboard: the text the panel shows, line by line.
    pub(crate) fn text(&self) -> String {
        let mut out = vec![self.title.clone(), "Theme".to_string()];
        for row in &self.rows {
            out.extend(Self::row_lines(row));
        }
        if !self.not_themeable.is_empty() {
            out.push("Not themeable".to_string());
            out.extend(self.not_themeable.iter().cloned());
        }
        out.join("\n")
    }
}

/// The Theme Map's four cells of one row (§10.4, the Theme Map row): the leaf, its value now
/// (`value_text`), the verdict, and the sinks with their scope or surface joined by "; " — for
/// an `unmappable` row its `sub_tag` and `upstream` instead, for a `tested_by` row its route.
pub(crate) fn row_cells(row: &Row, json: &serde_json::Value) -> [String; 4] {
    let value =
        value_at(json, &row.leaf).map_or_else(|| "(no value)".to_string(), |v| value_text(&v));
    let sinks = if row.verdict == Verdict::Unmappable {
        format!(
            "{}: {}",
            row.sub_tag.as_deref().unwrap_or_default(),
            row.upstream.as_deref().unwrap_or_default()
        )
    } else if let Some(test) = &row.tested_by {
        format!("per call; checked by {test}")
    } else {
        row.sinks
            .iter()
            .map(sink_text)
            .collect::<Vec<_>>()
            .join("; ")
    };
    [
        row.leaf.clone(),
        value,
        row.verdict.name().to_string(),
        sinks,
    ]
}

/// A colour value as egui's colour, for a swatch; `None` for any other value.
pub(crate) fn swatch_colour(value: &serde_json::Value) -> Option<egui::Color32> {
    let text = value.as_str()?;
    native_theme::color::Rgba::from_str(text)
        .ok()
        .map(native_theme_egui::convert::to_color32)
}

/// Text in the side panel's font (`sidebar.font`: `Body` in the sidebar scope the inspector is
/// drawn in), wrapping at the width it is given.
fn note(ui: &mut egui::Ui, text: impl Into<String>, weak: bool) -> egui::Response {
    let mut text = egui::RichText::new(text);
    if weak {
        text = text.weak();
    }
    ui.add(egui::Label::new(text).wrap())
}

/// A name over its value, one label, the gpui showcase's inspector `row` and note line
/// (`showcase-gpui/inspector.rs:343-351`, `:654-656`): the name in the weak text colour
/// (gpui's `muted_foreground` is `defaults.muted_color`, as egui's `weak_text_color` is, parity
/// rule R3), the value and each line after it in the text colour, all in the side panel's font
/// (`sidebar.font`, `Body` in the sidebar scope), wrapping.
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

/// A colour row, the gpui showcase's `swatch` line (`showcase-gpui/inspector.rs:364-371`,
/// `:644-648`): a `SWATCH_SIZE` square of the colour, framed as the gpui showcase's `demo_frame`
/// in `defaults.border` (`showcase-gpui/support.rs:209-216`), beside `label` in `style` — the
/// leaf and the colour's hex; the lines `under` it — the verdict and the details — weak, as
/// gpui's citation line is muted.
pub(crate) fn swatch_row(
    ui: &mut egui::Ui,
    t: &native_theme::theme::ResolvedTheme,
    colour: egui::Color32,
    (label, style): (&str, egui::TextStyle),
    under: &[String],
) -> egui::Response {
    let font = style.resolve(ui.style());
    let (weak, strong) = (ui.visuals().weak_text_color(), ui.visuals().text_color());
    let format = |color| egui::TextFormat {
        font_id: font.clone(),
        color,
        valign: egui::Align::Center,
        ..Default::default()
    };
    // One label: the first row leaves the swatch its room and is as tall as it, so the lines
    // under it follow with no gap, as gpui's `v_flex` of the two lines has none.
    let mut job = egui::text::LayoutJob {
        first_row_min_height: crate::SWATCH_SIZE,
        ..Default::default()
    };
    job.append(
        label,
        crate::SWATCH_SIZE + ui.spacing().item_spacing.x,
        format(strong),
    );
    for line in under {
        job.append("\n", 0.0, format(weak));
        job.append(line, 0.0, format(weak));
    }
    let response = ui.add(egui::Label::new(job).wrap());
    let square =
        egui::Rect::from_min_size(response.rect.min, egui::Vec2::splat(crate::SWATCH_SIZE));
    paint_swatch(ui, t, square, colour);
    response
}

/// `rect` filled with `colour` and framed in `defaults.border`'s width and radius and its
/// colour with `defaults.border.opacity` folded in (`native_theme_egui::border_color`), as the
/// gpui showcase's `demo_frame` (`showcase-gpui/support.rs:209-216`), which paints the colour
/// as stated: whether the opacity applies there is open (docs/todo.md, "Border opacity").
fn paint_swatch(
    ui: &egui::Ui,
    t: &native_theme::theme::ResolvedTheme,
    rect: egui::Rect,
    colour: egui::Color32,
) {
    ui.painter().rect(
        rect,
        native_theme_egui::border_radius(t),
        colour,
        egui::Stroke::new(
            t.defaults.border.line_width,
            native_theme_egui::border_color(t),
        ),
        egui::StrokeKind::Inside,
    );
}

/// The text Widget Info shows before anything is hovered, the three showcases' own.
pub(crate) const INFO_HINT: &str = "Hover any widget to see what the theme sets on it.";

/// The fonts and colours Widget Info draws with (R11 §B): `sidebar.font`'s size and family — the
/// `Body` style of the sidebar scope the inspector is drawn in — in `sidebar.font.color`, the
/// title and the section names at `SEMIBOLD_WEIGHT`, the lines under the rows in
/// `defaults.muted_color`; every line one line tall, `sidebar.font`'s size × `defaults.line_height`.
struct InfoText {
    regular: egui::FontId,
    semibold: egui::FontId,
    line: f32,
    colour: egui::Color32,
    muted: egui::Color32,
}

impl InfoText {
    fn of(ui: &egui::Ui, t: &native_theme::theme::ResolvedTheme) -> Self {
        let regular = egui::TextStyle::Body.resolve(ui.style());
        Self {
            semibold: crate::demo::semibold_font(ui, regular.size),
            line: regular.size * t.defaults.line_height,
            regular,
            colour: native_theme_egui::convert::to_color32(t.sidebar.font.color),
            muted: native_theme_egui::convert::to_color32(t.defaults.muted_color),
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
        response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
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
/// fraction of a pixel (`sidebar.font`'s size × `defaults.line_height`) paints the whole row
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
/// gap the theme leaves unstated is none. The content records nothing (§10.4); it places the
/// Widget Info elements of `docs/showcase-elements.toml`.
pub(crate) fn widget_tab(
    ui: &mut egui::Ui,
    t: &native_theme::theme::ResolvedTheme,
    layout: &native_theme::theme::LayoutTheme,
    view: Option<Result<InfoView, String>>,
    reg: &mut crate::demo::Registry,
) {
    let text = InfoText::of(ui, t);
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
    let widget_gap = layout.widget_gap.unwrap_or_default();
    let section_gap = layout.section_gap.unwrap_or_default();

    // The title row: the title and Copy, centred on each other.
    let title = text.galley(ui, &view.title, &text.semibold, text.colour);
    let copy_text = text.galley(ui, "Copy", &text.regular, text.colour);
    let padding = &t.button.border.padding;
    let copy_size = egui::vec2(
        copy_text.size().x + padding.left.unwrap_or_default() + padding.right.unwrap_or_default(),
        text.line + padding.top.unwrap_or_default() + padding.bottom.unwrap_or_default(),
    );
    let row_height = copy_size.y.max(text.line);
    let (row, title_row) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );
    title_row.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &view.title));
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
    let copy = ui.interact(copy_rect, ui.id().with("info copy"), egui::Sense::click());
    copy.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Copy"));
    let b = &t.button;
    // The pressed fill: `button.active_background`, or the hover fill where it is unstated (§6.4).
    let (fill, colour) = if copy.is_pointer_button_down_on() {
        (
            Some(b.active_background.unwrap_or(b.hover_background)),
            b.active_text_color,
        )
    } else if copy.hovered() {
        (Some(b.hover_background), b.hover_text_color)
    } else {
        (None, t.sidebar.font.color)
    };
    if let Some(fill) = fill {
        ui.painter().rect_filled(
            copy_rect,
            native_theme_egui::convert::to_corner_radius(
                egui::CornerRadius::ZERO,
                b.border.corner_radius,
            ),
            native_theme_egui::convert::to_color32(fill),
        );
    }
    let colour = native_theme_egui::convert::to_color32(colour);
    let copy_label = text.galley(ui, "Copy", &text.regular, colour);
    let at = egui::pos2(
        copy_rect.left() + padding.left.unwrap_or_default(),
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
        ui.add_space(widget_gap);
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), text.line + text.line),
            egui::Sense::hover(),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Label,
                true,
                InfoView::row_lines(row).join("\n"),
            )
        });
        let mut x = rect.left();
        let swatch = row.colour.map(|colour| {
            let square = egui::Rect::from_min_size(rect.min, egui::Vec2::splat(text.line));
            paint_swatch(ui, t, square, colour);
            x += text.line + widget_gap;
            square
        });
        let line = text.galley(
            ui,
            &format!("{} {}", row.leaf, row.value),
            &text.regular,
            text.colour,
        );
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

/// The Theme tab: `name()`, `os_mode()`, `accessibility()`, every `Note`, the
/// fonts in their defined units (§8.7), the window-frame facts of §10.4, and the
/// manifest's `[unwritten]` table.
pub(crate) fn theme_tab(
    ui: &mut egui::Ui,
    atlas: &ThemeAtlas,
    manifest: &Result<Manifest, String>,
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let section = |ui: &mut egui::Ui, title: &str| {
        let heading = crate::demo::heading_text(ui, title);
        ui.label(heading);
    };
    let row = |ui: &mut egui::Ui, key: &str, value: String| {
        key_value(ui, key, &[value]);
    };
    // Sections and rows as the gpui showcase's Theme tab (`showcase-gpui/inspector.rs:187-253`).
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
    if atlas.notes().is_empty() {
        note(ui, "none", true);
    }
    for note in atlas.notes() {
        let text = format!("{note:?}");
        let kind: String = text.chars().take_while(|c| c.is_alphanumeric()).collect();
        row(ui, &kind, text);
    }
    section(ui, "Fonts");
    let size = |font: &native_theme::theme::ResolvedFontSpec| match font.defined_size {
        Some(native_theme::theme::FontSize::Pt(v)) => format!("{v}pt"),
        Some(native_theme::theme::FontSize::Px(v)) => format!("{v}px"),
        None => "(size not stated)".to_string(),
    };
    row(ui, "font_family", t.defaults.font.family.to_string());
    row(ui, "font_size", size(&t.defaults.font));
    row(
        ui,
        "mono_font_family",
        t.defaults.mono_font.family.to_string(),
    );
    row(ui, "mono_font_size", size(&t.defaults.mono_font));
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
