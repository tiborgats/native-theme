//! The load budget's probe, its part both binaries share (the 0.8.0 plan's
//! *The load budget*; ADD-A147): the cases the probe can measure, each
//! bundled preset's file as the synthetic route needs it, the themes the
//! cases measure, and a pass run in a process of its own.
//!
//! A case is named `<line>.<preset>.<route>[.<what>]`, the route `read`,
//! `synthetic`, `plain` or `file`. Lines 1 to 3 are `read` on every preset:
//! the read of its text alone (`native_theme::gate::read_cost`, a lower
//! bound, which leaves out the trusted construction's own part), its peak
//! and the bytes its result holds, since until load task 1 (trust) a whole
//! load runs the checks that task removes; on a preset whose `[pending]`
//! table lists no path, lines 1 and 2 are also `plain`, `Theme::from_preset`
//! whole. Lines 3 to 8, 11 and 12 are measured on the synthetic theme where
//! the table lists paths, else on the plain one, and say which. Every value
//! a case states (an accent colour, a reference layer's values) is a
//! synthetic fixture value, not a platform's.

use std::error::Error;
use std::process::Command;

use native_theme::{Layer, PresetId, Scheme, Theme};

/// A probe's result.
pub type Res<T> = Result<T, Box<dyn Error>>;

/// The scheme every case loads its theme under: the one the D6 profile
/// measured (`<preset>-light-…`).
pub const SCHEME: Scheme = Scheme::Light;

/// Line 12's reference layers, application layers that state structure
/// (*The load budget*, line 12: an expression, `absent`, `auto`, a rule on
/// a part, a presence), each with its name in the case. Synthetic fixtures:
/// their values are no platform's.
pub const REFERENCE_LAYERS: [(&str, &str); 5] = [
    (
        "expression",
        "[widgets.button.body]\nmin_width_px = \"= add(part.padding.left, part.padding.right)\"\n",
    ),
    ("absent", "[widgets.button.body]\ncursor = \"absent\"\n"),
    ("auto", "[widgets.button.body]\nmax_width_px = \"auto\"\n"),
    (
        "rule",
        "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\n[rules.set]\nopacity = 1\n",
    ),
    ("presence", "[widgets.button.subtitle]\nabsent = true\n"),
];

/// How a case's figure is counted (*The load budget*, rule 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A time, the median of a pass's samples.
    Time,
    /// Bytes a counting allocator counts.
    Bytes,
    /// Bytes counted exactly: a preset file's size (line 9).
    Exact,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Time => "time",
            Kind::Bytes => "bytes",
            Kind::Exact => "exact",
        }
    }
}

/// The theme a case is measured on, or the read alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// The read of the preset's text alone (`gate::read_cost`).
    Read,
    /// `Theme::from_preset_synthetic` over the preset's synthetic layers.
    Synthetic,
    /// `Theme::from_preset`.
    Plain,
}

impl Route {
    fn name(self) -> &'static str {
        match self {
            Route::Read => "read",
            Route::Synthetic => "synthetic",
            Route::Plain => "plain",
        }
    }
}

/// A change every application makes (line 4) that the probe can make.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    /// `pick(Some(Dark))` on a light theme.
    Pick,
    /// `set_contrast(Some(High))`.
    Contrast,
    /// `set_accent` of a synthetic accent stating every accent input.
    Accent,
    // `set_flavor` has no case: at depth it refuses every flavour but the
    // preset's default (`ProblemKind::Later`); the flavours unit brings back
    // `4.<preset>.<route>.set_flavor` on a preset that declares two.
    /// `set_preset` to the preset itself, which a second live theme holds.
    SetPreset,
}

impl Change {
    fn name(self) -> &'static str {
        match self {
            Change::Pick => "pick",
            Change::Contrast => "set_contrast",
            Change::Accent => "set_accent",
            Change::SetPreset => "set_preset",
        }
    }
}

/// The button's body as a query asks for it: at rest or hovered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Body {
    /// At rest.
    Rest,
    /// Hovered.
    Hover,
}

impl Body {
    fn name(self) -> &'static str {
        match self {
            Body::Rest => "rest",
            Body::Hover => "hover",
        }
    }
}

/// What a case measures, by budget line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Measure {
    /// Line 1: the first load in the process, `read` or `plain`.
    Load(Route),
    /// Line 2: the peak during the first load, above the level before it.
    LoadPeak(Route),
    /// Line 3: the bytes kept: the read's result (`read`, the theme's part
    /// alone), or the theme and one window's resolution.
    Kept(Route),
    /// Line 4: one change.
    Change(Route, Change),
    /// Line 5: `resolve` for a new window.
    ResolveChanged(Route),
    /// Line 6: `resolve` again, nothing changed.
    ResolveUnchanged(Route),
    /// Line 7: a styled query on a resolution not asked before.
    QueryComputed(Route, Body),
    /// Line 8: a styled query asked again in one resolution.
    QueryRepeated(Route, Body),
    /// Line 9: the preset file's size, which `include_str!` embeds.
    File,
    /// Line 11: the peak during `resolve`, above the level before it.
    ResolvePeak(Route),
    /// Line 12: adding a reference layer, `REFERENCE_LAYERS`' index.
    AddLayer(Route, usize),
}

/// A case the probe can measure.
#[derive(Clone, Debug)]
pub struct Case {
    /// Its name, `<line>.<preset>.<route>[.<what>]`.
    pub name: String,
    /// Its preset.
    pub preset: PresetId,
    /// What it measures.
    pub measure: Measure,
}

impl Case {
    /// Its budget line.
    pub fn line(&self) -> u8 {
        match self.measure {
            Measure::Load(_) => 1,
            Measure::LoadPeak(_) => 2,
            Measure::Kept(_) => 3,
            Measure::Change(..) => 4,
            Measure::ResolveChanged(_) => 5,
            Measure::ResolveUnchanged(_) => 6,
            Measure::QueryComputed(..) => 7,
            Measure::QueryRepeated(..) => 8,
            Measure::File => 9,
            Measure::ResolvePeak(_) => 11,
            Measure::AddLayer(..) => 12,
        }
    }

    /// How its figure is counted.
    pub fn kind(&self) -> Kind {
        match self.measure {
            Measure::LoadPeak(_) | Measure::Kept(_) | Measure::ResolvePeak(_) => Kind::Bytes,
            Measure::File => Kind::Exact,
            _ => Kind::Time,
        }
    }

    /// Its line in the probe's list.
    pub fn listed(&self) -> String {
        format!(
            "cost-case {} line={} kind={}",
            self.name,
            self.line(),
            self.kind().name()
        )
    }
}

/// Every case the probe can measure, preset by preset in
/// `PresetId::all()`'s order, each preset's by line.
pub fn cases() -> Res<Vec<Case>> {
    let mut cases = Vec::new();
    for &id in PresetId::all() {
        let file = PresetFile::read(id)?;
        let route = file.route();
        let mut measures = vec![
            Measure::Load(Route::Read),
            Measure::LoadPeak(Route::Read),
            Measure::Kept(Route::Read),
        ];
        if route == Route::Plain {
            measures.push(Measure::Load(Route::Plain));
            measures.push(Measure::LoadPeak(Route::Plain));
        }
        measures.push(Measure::Kept(route));
        let mut changes = vec![Change::Pick, Change::Contrast, Change::Accent];
        if route == Route::Plain {
            changes.push(Change::SetPreset);
        }
        measures.extend(
            changes
                .into_iter()
                .map(|change| Measure::Change(route, change)),
        );
        measures.push(Measure::ResolveChanged(route));
        measures.push(Measure::ResolveUnchanged(route));
        for body in [Body::Rest, Body::Hover] {
            measures.push(Measure::QueryComputed(route, body));
        }
        for body in [Body::Rest, Body::Hover] {
            measures.push(Measure::QueryRepeated(route, body));
        }
        measures.push(Measure::File);
        measures.push(Measure::ResolvePeak(route));
        measures.extend((0..REFERENCE_LAYERS.len()).map(|at| Measure::AddLayer(route, at)));
        for measure in measures {
            let case = Case {
                name: String::new(),
                preset: id,
                measure,
            };
            let name = format!("{}.{}.{}", case.line(), id.key(), what(measure));
            cases.push(Case { name, ..case });
        }
    }
    Ok(cases)
}

/// A case's name after its preset: its route and what it measures.
fn what(measure: Measure) -> String {
    match measure {
        Measure::Load(route)
        | Measure::LoadPeak(route)
        | Measure::Kept(route)
        | Measure::ResolveChanged(route)
        | Measure::ResolveUnchanged(route)
        | Measure::ResolvePeak(route) => route.name().to_owned(),
        Measure::Change(route, change) => format!("{}.{}", route.name(), change.name()),
        Measure::QueryComputed(route, body) | Measure::QueryRepeated(route, body) => {
            format!("{}.{}", route.name(), body.name())
        }
        Measure::File => "file".to_owned(),
        Measure::AddLayer(route, at) => format!(
            "{}.{}",
            route.name(),
            REFERENCE_LAYERS.get(at).map_or("unknown", |(name, _)| name)
        ),
    }
}

/// The cases among `all` that `names` asks for, in its order; each name
/// that is none of them is printed `cost-error <name> not a case this
/// binary can measure`, and the others are measured all the same (the
/// design's situation A13: a case a probe change no longer produces fails
/// alone); with the number of such names.
pub fn asked(names: &str, all: &[Case]) -> (Vec<Case>, usize) {
    let mut unknown = 0_usize;
    let cases = names
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .filter_map(|name| {
            let case = all.iter().find(|case| case.name == name).cloned();
            if case.is_none() {
                unknown = unknown.saturating_add(1);
                println!("cost-error {name} not a case this binary can measure");
            }
            case
        })
        .collect();
    (cases, unknown)
}

/// A bundled preset's file as the probe needs it from outside the crate:
/// its `[pending]` table, its own layer (the file without its `[preset]`,
/// `[declare]` and `[pending]` tables, read as a layer: the bases and rules
/// `synthetic_layers` looks a keyed entry up in) and its declarations.
pub struct PresetFile {
    /// The preset.
    id: PresetId,
    /// `[pending]`: each listed path with its register ids.
    pub pending: Vec<(String, Vec<String>)>,
    /// The file's tables.
    table: toml::Table,
}

/// The bundled preset `id`'s file, which `include_str!` puts in a binary.
pub fn preset_path(id: PresetId) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/presets")
        .join(format!("{}.toml", id.key()))
}

impl PresetFile {
    /// The bundled preset `id`'s file.
    pub fn read(id: PresetId) -> Res<PresetFile> {
        let text = std::fs::read_to_string(preset_path(id))?;
        let table: toml::Table = text.parse()?;
        let mut pending = Vec::new();
        if let Some(entries) = table.get("pending").and_then(toml::Value::as_table) {
            for (path, ids) in entries {
                let ids = match ids {
                    toml::Value::String(id) => vec![id.clone()],
                    toml::Value::Array(ids) => strings(ids)
                        .ok_or_else(|| format!("{}: pending {path} = {ids:?}", id.key()))?,
                    other => return Err(format!("{}: pending {path} = {other}", id.key()).into()),
                };
                pending.push((path.clone(), ids));
            }
        }
        Ok(PresetFile { id, pending, table })
    }

    /// The names `[declare] <name>` lists, none where it lists none.
    #[allow(
        dead_code,
        reason = "the timings binary's alone (line 4's set_accent); the cost_bytes binary shares this module"
    )]
    pub fn declared(&self, name: &str) -> Res<Vec<String>> {
        let key = self.id.key();
        match self
            .table
            .get("declare")
            .and_then(|declare| declare.get(name))
        {
            None => Ok(Vec::new()),
            Some(toml::Value::Array(names)) => {
                Ok(strings(names).ok_or_else(|| format!("{key}: declare {name} = {names:?}"))?)
            }
            Some(other) => Err(format!("{key}: declare {name} = {other}").into()),
        }
    }

    /// The route its theme takes: synthetic while its `[pending]` table
    /// lists paths, else plain.
    pub fn route(&self) -> Route {
        if self.pending.is_empty() {
            Route::Plain
        } else {
            Route::Synthetic
        }
    }

    /// Its own layer: the file without its `[preset]`, `[declare]` and
    /// `[pending]` tables.
    pub fn own(&self) -> Res<Layer> {
        let mut table = self.table.clone();
        for header in ["preset", "declare", "pending"] {
            table.remove(header);
        }
        Ok(Layer::from_toml(&toml::to_string(&table)?)?)
    }
}

fn strings(values: &[toml::Value]) -> Option<Vec<String>> {
    values
        .iter()
        .map(|value| value.as_str().map(str::to_owned))
        .collect()
}

/// The theme of the bundled preset `id` on `route` (synthetic or plain),
/// under [`SCHEME`]; for the synthetic route its layers are built from
/// `file`'s `[pending]` table over `own`, its own layer.
pub fn theme(id: PresetId, route: Route, file: &PresetFile, own: &Layer) -> Res<Theme> {
    match route {
        Route::Plain => Ok(Theme::from_preset(id, SCHEME)?),
        Route::Synthetic => {
            let layers = native_theme::gate::synthetic_layers(&file.pending, own)?;
            Ok(Theme::from_preset_synthetic(id, SCHEME, layers)?)
        }
        Route::Read => Err("the read builds no theme".into()),
    }
}

/// Runs this test binary's `test` again in a process of its own with
/// `NT_COST_CHILD` set to `child` and `NT_COST_CASES` to `cases`, and
/// prints the lines of its output the probe prints (`cost-…`); a process
/// that fails prints `cost-error` with its reason for each of `cases` it
/// did not report.
pub fn run_child(test: &str, child: &str, cases: &[&str]) -> Res<()> {
    let output = Command::new(std::env::current_exe()?)
        .args([
            "--ignored",
            "--exact",
            test,
            "--nocapture",
            "--test-threads=1",
        ])
        .env("NT_COST_CHILD", child)
        .env("NT_COST_CASES", cases.join(","))
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    // The first may follow libtest's `test <name> ... ` on its line.
    let printed: Vec<&str> = stdout
        .lines()
        .filter_map(|line| line.find("cost-").and_then(|at| line.get(at..)))
        .collect();
    for line in &printed {
        println!("{line}");
    }
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let why = stderr
            .lines()
            .chain(stdout.lines())
            .rfind(|line| line.contains("Error") || line.contains("panicked"))
            .unwrap_or("the process failed")
            .to_owned();
        let reported = |case: &str| {
            printed
                .iter()
                .any(|line| line.split_whitespace().nth(1) == Some(case))
        };
        for case in cases.iter().filter(|case| !reported(case)) {
            println!("cost-error {case} {why}");
        }
    }
    Ok(())
}
