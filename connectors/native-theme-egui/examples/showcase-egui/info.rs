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

/// The lines one row prints: the leaf, its value, the verdict, the sinks with
/// their scope or surface, a `tested_by` route, an exception for `preset`, and
/// for an `unmappable` row "lost here", its sub_tag and its upstream line.
pub(crate) fn row_lines(row: &Row, json: &serde_json::Value, preset: &str) -> Vec<String> {
    let value =
        value_at(json, &row.leaf).map_or_else(|| "(no value)".to_string(), |v| value_text(&v));
    let mut lines = vec![
        format!("{} = {value}", row.leaf),
        format!("  {}", row.verdict.name()),
    ];
    lines.extend(row.sinks.iter().map(|s| format!("  → {}", sink_text(s))));
    if let Some(test) = &row.tested_by {
        lines.push(format!(
            "  no Style or Frame field carries it; its route is checked by {test}"
        ));
    }
    for (_, why) in row.exceptions.iter().filter(|(p, _)| p == preset) {
        lines.push(format!("  on {preset}: {why}"));
    }
    if row.verdict == Verdict::Unmappable {
        lines.push(format!(
            "  lost here — {}: {}",
            row.sub_tag.as_deref().unwrap_or_default(),
            row.upstream.as_deref().unwrap_or_default()
        ));
    }
    lines
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

fn seam_text(seam: &Seam) -> String {
    match seam {
        Seam::Base => "the base style".to_string(),
        Seam::Role(role, variant) => format!("Role::{role:?} ({})", variant.key()),
        Seam::Surface(surface) => format!("Surface::{surface:?}"),
    }
}

/// What Copy puts on the clipboard: the instance's kind, its seams, each seam's rows, the accessors and leaves read, the notes.
pub(crate) fn info_text(
    shown: &Shown,
    manifest: &Manifest,
    json: &serde_json::Value,
    preset: &str,
) -> String {
    let mut out = vec![shown.info.kind.to_string()];
    for seam in &shown.info.seams {
        out.push(format!("[{}]", seam_text(seam)));
        for row in manifest.rows_for(seam) {
            out.extend(row_lines(row, json, preset));
        }
    }
    for (name, value) in &shown.info.read {
        out.push(format!("read: {name} = {value}"));
    }
    for note in &shown.info.notes {
        out.push(format!("this instance: {note}"));
    }
    out.join("\n")
}

/// A colour value as egui's colour, for a swatch; `None` for any other value.
fn swatch_colour(value: &serde_json::Value) -> Option<egui::Color32> {
    let text = value.as_str()?;
    native_theme::color::Rgba::from_str(text)
        .ok()
        .map(native_theme_egui::convert::to_color32)
}

/// The Widget tab: `info_text`'s content as rows with colour swatches
/// (`ui.color_edit_button_srgba` is an editor; a swatch is a `Frame` filled with
/// the colour at `ui.spacing().interact_size.y` square), and the Copy button.
pub(crate) fn widget_tab(
    ui: &mut egui::Ui,
    shown: Option<&Shown>,
    manifest: &Result<Manifest, String>,
    json: &Result<serde_json::Value, String>,
    preset: &str,
) {
    let Some(shown) = shown else {
        ui.label("Hover any widget to see what the theme sets on it.");
        return;
    };
    let (manifest, json) = match (manifest, json) {
        (Ok(manifest), Ok(json)) => (manifest, json),
        (Err(error), _) | (_, Err(error)) => {
            ui.label(format!("Widget Info is unavailable: {error}"));
            return;
        }
    };
    ui.horizontal(|ui| {
        ui.strong(shown.info.kind);
        if ui.button("Copy").clicked() {
            ui.ctx().copy_text(info_text(shown, manifest, json, preset));
        }
    });
    let side = ui.spacing().interact_size.y;
    for seam in &shown.info.seams {
        ui.strong(seam_text(seam));
        for row in manifest.rows_for(seam) {
            let swatch = value_at(json, &row.leaf).as_ref().and_then(swatch_colour);
            let lines = row_lines(row, json, preset);
            ui.horizontal_top(|ui| {
                if let Some(colour) = swatch {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::Vec2::splat(side), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 0, colour);
                }
                ui.vertical(|ui| {
                    for line in lines {
                        ui.label(line);
                    }
                });
            });
        }
    }
    for (name, value) in &shown.info.read {
        ui.label(format!("read: {name} = {value}"));
    }
    for note in &shown.info.notes {
        ui.label(format!("This instance: {note}"));
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
    ui.label(format!("Theme: {}", atlas.name()));
    ui.label(format!("OS mode: {:?}", atlas.os_mode()));
    ui.label(format!("Accessibility: {:?}", atlas.accessibility()));
    ui.strong("Notes");
    if atlas.notes().is_empty() {
        ui.label("none");
    }
    for note in atlas.notes() {
        ui.label(format!("{note:?}"));
    }
    ui.strong("Fonts");
    for (name, font) in [
        ("defaults.font", &t.defaults.font),
        ("defaults.mono_font", &t.defaults.mono_font),
    ] {
        let size = match font.defined_size {
            Some(native_theme::theme::FontSize::Pt(v)) => format!("{v}pt"),
            Some(native_theme::theme::FontSize::Px(v)) => format!("{v}px"),
            None => "(size not stated)".to_string(),
        };
        ui.label(format!("{name}: {} {size}", font.family));
    }
    ui.strong("The window");
    ui.label("The OS draws the window's frame and title bar (decorations on); the showcase draws none (§10.4).");
    ui.strong("Fields left at egui's own value");
    match manifest {
        Ok(manifest) => {
            for (path, why) in &manifest.unwritten {
                ui.label(format!("{path}: {why}"));
            }
        }
        Err(error) => {
            ui.label(format!("the manifest did not parse: {error}"));
        }
    }
}
