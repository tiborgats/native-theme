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

/// One row as Widget Info shows it: the leaf, its value, the verdict, a colour value's colour,
/// and what follows the value — each sink it writes with its scope or surface, a `tested_by`
/// route, an exception for `preset`, and for an `unmappable` row "lost here", its sub_tag and
/// its upstream line. In words, with no glyph the proportional fonts may lack (parity item 7).
pub(crate) struct RowView {
    pub leaf: String,
    pub value: String,
    pub verdict: &'static str,
    pub colour: Option<egui::Color32>,
    pub details: Vec<String>,
}

pub(crate) fn row_view(row: &Row, json: &serde_json::Value, preset: &str) -> RowView {
    let value = value_at(json, &row.leaf);
    let mut details: Vec<String> = row
        .sinks
        .iter()
        .map(|s| format!("writes {}", sink_text(s)))
        .collect();
    if let Some(test) = &row.tested_by {
        details.push(format!(
            "no Style or Frame field carries it; its route is checked by {test}"
        ));
    }
    for (_, why) in row.exceptions.iter().filter(|(p, _)| p == preset) {
        details.push(format!("on {preset}: {why}"));
    }
    if row.verdict == Verdict::Unmappable {
        details.push(format!(
            "lost here — {}: {}",
            row.sub_tag.as_deref().unwrap_or_default(),
            row.upstream.as_deref().unwrap_or_default()
        ));
    }
    RowView {
        leaf: row.leaf.clone(),
        colour: value.as_ref().and_then(swatch_colour),
        value: value.map_or_else(|| "(no value)".to_string(), |v| value_text(&v)),
        verdict: row.verdict.name(),
        details,
    }
}

/// The lines one row prints in Copy's text: `leaf = value`, then the verdict and each of
/// `row_view`'s details, indented.
pub(crate) fn row_lines(row: &Row, json: &serde_json::Value, preset: &str) -> Vec<String> {
    let view = row_view(row, json, preset);
    let mut lines = vec![
        format!("{} = {}", view.leaf, view.value),
        format!("  {}", view.verdict),
    ];
    lines.extend(view.details.iter().map(|d| format!("  {d}")));
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
    for (what, note) in &shown.info.notes {
        out.push(format!("this instance: {what}: {note}"));
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

/// Text in `Small`, gpui's `text_sm` and `text_xs` (parity rule R1), wrapping at the width it
/// is given.
fn small(ui: &mut egui::Ui, text: impl Into<String>, weak: bool) -> egui::Response {
    let mut text = egui::RichText::new(text).small();
    if weak {
        text = text.weak();
    }
    ui.add(egui::Label::new(text).wrap())
}

/// A name over its value, one label, the gpui showcase's inspector `row` and note line
/// (`showcase-gpui/inspector.rs:321-329`, `:365-367`): the name in the weak text colour
/// (gpui's `muted_foreground` is `defaults.muted_color`, as egui's `weak_text_color` is, parity
/// rule R3), the value and each line after it in the text colour, all in `Small`, wrapping.
pub(crate) fn key_value(ui: &mut egui::Ui, key: &str, lines: &[String]) -> egui::Response {
    let font = egui::TextStyle::Small.resolve(ui.style());
    let (weak, strong) = (ui.visuals().weak_text_color(), ui.visuals().text_color());
    let mut job = egui::text::LayoutJob::default();
    job.append(key, 0.0, egui::TextFormat::simple(font.clone(), weak));
    for line in lines {
        job.append("\n", 0.0, egui::TextFormat::simple(font.clone(), strong));
        job.append(line, 0.0, egui::TextFormat::simple(font.clone(), strong));
    }
    ui.add(egui::Label::new(job).wrap())
}

/// A colour row, the gpui showcase's `swatch` line (`showcase-gpui/inspector.rs:342-360`): a
/// `SWATCH_SIZE` square of the colour, framed as the gpui showcase's `demo_frame` in
/// `defaults.border` (`showcase-gpui/support.rs:209-216`), beside the leaf and the colour's hex;
/// the verdict and the details under it, weak, as gpui's citation line is muted.
pub(crate) fn swatch_row(
    ui: &mut egui::Ui,
    t: &native_theme::theme::ResolvedTheme,
    colour: egui::Color32,
    label: &str,
    under: &[String],
) -> egui::Response {
    let font = egui::TextStyle::Small.resolve(ui.style());
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

/// `rect` filled with `colour` and framed in `defaults.border`'s colour, width and radius, as
/// the gpui showcase's `demo_frame` (`showcase-gpui/support.rs:209-216`).
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

/// The Widget tab, as the gpui showcase's (`showcase-gpui/inspector.rs:102-165`, `:352-378`):
/// the title and Copy on one row, then a section per seam — its heading, a swatch line per
/// colour row, a name-over-value row per other row — then "Theme config", the accessors and
/// leaves the helper read, and "This instance", its notes. The content records nothing (§10.4).
pub(crate) fn widget_tab(
    ui: &mut egui::Ui,
    t: &native_theme::theme::ResolvedTheme,
    shown: Option<&Shown>,
    manifest: &Result<Manifest, String>,
    json: &Result<serde_json::Value, String>,
    preset: &str,
) {
    let Some(shown) = shown else {
        small(
            ui,
            "Hover any widget to see what the theme sets on it.",
            true,
        );
        return;
    };
    let (manifest, json) = match (manifest, json) {
        (Ok(manifest), Ok(json)) => (manifest, json),
        (Err(error), _) | (_, Err(error)) => {
            small(ui, format!("Widget Info is unavailable: {error}"), true);
            return;
        }
    };
    // The title, and Copy flush right: a small Ghost button, frameless at rest (parity rule R4).
    ui.horizontal(|ui| {
        let title = crate::demo::section_text(ui, shown.info.kind);
        ui.label(title);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let copy = egui::Button::new("Copy").small().frame_when_inactive(false);
            if ui.add(copy).clicked() {
                ui.ctx().copy_text(info_text(shown, manifest, json, preset));
            }
        });
    });
    for seam in &shown.info.seams {
        let heading = crate::demo::section_text(ui, seam_text(seam));
        ui.label(heading);
        for row in manifest.rows_for(seam) {
            let view = row_view(row, json, preset);
            match view.colour {
                Some(colour) => {
                    let mut under = vec![view.verdict.to_string()];
                    under.extend(view.details);
                    swatch_row(
                        ui,
                        t,
                        colour,
                        &format!("{} {}", view.leaf, view.value),
                        &under,
                    );
                }
                None => {
                    let mut lines = vec![format!("{} ({})", view.value, view.verdict)];
                    lines.extend(view.details);
                    key_value(ui, &view.leaf, &lines);
                }
            }
        }
    }
    if !shown.info.read.is_empty() {
        let heading = crate::demo::section_text(ui, "Theme config");
        ui.label(heading);
        for (name, value) in &shown.info.read {
            key_value(ui, name, std::slice::from_ref(value));
        }
    }
    if !shown.info.notes.is_empty() {
        let heading = crate::demo::section_text(ui, "This instance");
        ui.label(heading);
        for (what, note) in &shown.info.notes {
            key_value(ui, what, std::slice::from_ref(note));
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
        let heading = crate::demo::section_text(ui, title);
        ui.label(heading);
    };
    let row = |ui: &mut egui::Ui, key: &str, value: String| {
        key_value(ui, key, &[value]);
    };
    // Sections and rows as the gpui showcase's Theme tab (`showcase-gpui/inspector.rs:166-232`).
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
        small(ui, "none", true);
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
            small(ui, format!("the manifest did not parse: {error}"), true);
        }
    }
}
