//! The enabled units (S17 *While 0.8.0 is built*): `scripts/widget_units.toml`
//! read as `scripts/check_facts_trace.py` reads it (its `read_units` and
//! `read_later`), a path in a unit by that script's `in_unit` rule and a name
//! at depth by its `Depth` class's (ADD-A147), so the gate and the scripts
//! enable the same units and names from one source.

use std::path::Path;

use toml::{Table, Value};

/// The units file's table of unit names listed before their units are
/// (M3a's widget units).
const NAMED: &str = "unit_names";

/// The units file's tables that are no unit.
const TABLES: [&str; 8] = [
    "deferred",
    "hand_written",
    "prefixes",
    "staged",
    "kept",
    "at_depth",
    "later",
    NAMED,
];

/// The widget catalogue, relative to the repository root, whose entries a
/// `[staged]` or `[deferred]` value may name.
const CATALOGUE: &str = "docs/todo_v0.7.1_model-completeness-spec-widgets.md";

/// The catalogue's entry names (scripts/spec_catalogue.py's `entries`): each
/// `### <name>` heading of a typed section (one of M3a's six, or one whose
/// title starts `Typed widgets`) and each `#### component.<key>` heading of
/// a `Keyed components` section.
fn catalogue_entries(text: &str) -> Vec<String> {
    const TYPED: [&str; 6] = [
        "Window and chrome",
        "Overlays",
        "Buttons and toggles",
        "Fields",
        "Indicators",
        "Containers",
    ];
    let mut out = Vec::new();
    let mut section: Option<&str> = None;
    for line in text.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            section = Some(title.trim());
        } else if let Some(name) = line.strip_prefix("### ") {
            if section.is_some_and(|s| TYPED.contains(&s) || s.starts_with("Typed widgets")) {
                out.push(name.trim().to_owned());
            }
        } else if let Some(rest) = line.strip_prefix("#### component.")
            && section.is_some_and(|s| s.starts_with("Keyed components"))
        {
            let key: String = rest
                .chars()
                .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '_')
                .collect();
            if rest.starts_with(|c: char| c.is_ascii_lowercase() || c == '_') {
                out.push(format!("component.{key}"));
            }
        }
    }
    out
}

/// Each `[staged.<unit>]` path and `[deferred.<widget>]` value or part's
/// field, as the units file spells its key, with the unit it names, in the
/// file's order.
fn waiting_values(data: &Table) -> Vec<(String, String)> {
    let table = |name: &str| data.get(name).and_then(Value::as_table);
    let mut out = Vec::new();
    for (unit, paths) in table("staged").into_iter().flatten() {
        for (staged, value) in paths.as_table().into_iter().flatten() {
            let value = value.as_str().unwrap_or_default().to_owned();
            out.push((format!("staged.{unit}.\"{staged}\""), value));
        }
    }
    for (widget, values) in table("deferred").into_iter().flatten() {
        for (name, value) in values.as_table().into_iter().flatten() {
            let value = value.as_str().unwrap_or_default().to_owned();
            let key = if name.contains('.') {
                format!("deferred.{widget}.\"{name}\"")
            } else {
                format!("deferred.{widget}.{name}")
            };
            out.push((key, value));
        }
    }
    out
}

/// Why a `[staged]` or `[deferred]` value of `data` (the units file at
/// `path`) names none of the units a path may wait for (S6.1, ADD-A147;
/// check_facts_trace.py's `check_waiting`): a `[[later]]` unit, a
/// `[hand_written]` unit, a unit `[unit_names]` lists or a catalogue entry,
/// the catalogue (at [`CATALOGUE`] beside the file's `scripts/` folder)
/// read only for a value none of the others names; and why a
/// `[unit_names]` is no table of lists of names.
fn check_waiting(data: &Table, path: &Path, later: &[Later]) -> Result<(), String> {
    let mut known: Vec<String> = later.iter().map(|unit| unit.name.clone()).collect();
    if let Some(hand) = data.get("hand_written").and_then(Value::as_table) {
        known.extend(hand.keys().cloned());
    }
    match data.get(NAMED) {
        None => {}
        Some(Value::Table(named)) => {
            for (milestone, names) in named {
                known.extend(list(names, &format!("{path:?}: `{NAMED}.{milestone}`"))?);
            }
        }
        Some(_) => return Err(format!("{path:?}: `{NAMED}` is not a table")),
    }
    let unknown: Vec<(String, String)> = waiting_values(data)
        .into_iter()
        .filter(|(_, value)| !known.contains(value))
        .collect();
    let Some((first_key, first_value)) = unknown.first() else {
        return Ok(());
    };
    let catalogue = path
        .parent()
        .and_then(Path::parent)
        .map(|root| root.join(CATALOGUE))
        .ok_or_else(|| format!("{path:?}: no repository root above it"))?;
    let text = std::fs::read_to_string(&catalogue).map_err(|error| {
        format!(
            "{path:?}: `{first_key}` = {first_value:?} names no later, hand-written or listed \
             unit, and the catalogue that may name it an entry cannot be read: {error}"
        )
    })?;
    let entries = catalogue_entries(&text);
    match unknown.iter().find(|(_, value)| !entries.contains(value)) {
        Some((key, value)) => Err(format!(
            "{path:?}: `{key}` = {value:?} names no later unit, no hand-written unit, no unit \
             `{NAMED}` lists and no catalogue entry"
        )),
        None => Ok(()),
    }
}

/// The unit suffixes a path's segment drops before it is compared (S1).
const SUFFIXES: [&str; 4] = ["_px", "_ms", "_deg", "_pt"];

/// The keys of a `[[later]]` table that name no catalogue entry.
const LATER_KEYS: [&str; 6] = [
    "name",
    "kind",
    "enabled",
    "conditions",
    "directions",
    "sizes",
];

/// One enabled unit: its catalogue entries and prefixes, the paths it holds
/// none of (the values and part fields a unit defers, the paths it stages,
/// a `*` segment standing for any one, a `@*` for any keyed one), and per
/// preset the entries of a staged keyed map it keeps (`[kept]`), each by its
/// segments.
#[derive(Clone, Debug)]
struct Unit {
    entries: Vec<Vec<String>>,
    deferred: Vec<Vec<String>>,
    kept: Vec<(String, Vec<Vec<String>>)>,
}

/// A group of names a later unit holds for a catalogue entry: on the
/// presets it names, on every preset where it names none.
#[derive(Clone, Debug)]
struct Group {
    entry: String,
    presets: Option<Vec<String>>,
    variants: Vec<String>,
    states: Vec<String>,
}

/// A later unit (`[[later]]`, ADD-A147): whether it is enabled and what it
/// holds.
#[derive(Clone, Debug)]
struct Later {
    name: String,
    enabled: bool,
    conditions: Vec<String>,
    directions: Vec<String>,
    sizes: bool,
    groups: Vec<Group>,
}

/// The enabled units, and the later units with the names they hold.
#[derive(Clone, Debug)]
pub struct Units {
    units: Vec<Unit>,
    later: Vec<Later>,
    /// `[at_depth]`: a key prefix's segments and the conditions at depth for
    /// its keys that a later unit holds for every other key.
    at_depth: Vec<(Vec<String>, Vec<String>)>,
    /// Each path `[staged]` stages or `[deferred]` defers, by its segments,
    /// with the unit it waits for, in the file's order.
    waiting_paths: Vec<(Vec<String>, String)>,
}

/// The catalogue entry a key, a path or a state model's `widget` names
/// (check_facts_trace.py's `entry_of`): `component.<key>` under
/// `component.`, else its first segment (a widget, or `role`, `token`,
/// `input`).
pub fn entry_of(key: &str) -> &str {
    let mut ends = key.match_indices('.').map(|(at, _)| at);
    let first = ends.next().unwrap_or(key.len());
    let entry = key.get(..first).unwrap_or(key);
    if entry == "component"
        && first < key.len()
        && let Some(end) = ends.next().or(Some(key.len()))
    {
        return key.get(..end).unwrap_or(key);
    }
    entry
}

/// A path's segments, each without its unit suffix.
fn segments(path: &str) -> Vec<String> {
    path.split('.')
        .map(|segment| {
            SUFFIXES
                .iter()
                .find_map(|suffix| segment.strip_suffix(suffix))
                .unwrap_or(segment)
                .to_owned()
        })
        .collect()
}

/// Whether `path` starts with `prefix`, segment by segment, a `*` segment
/// of the prefix standing for any one, a `@*` for any keyed one.
fn under(path: &[String], prefix: &[String]) -> bool {
    path.len() >= prefix.len()
        && prefix.iter().zip(path).all(|(held, segment)| {
            held == "*" || held == segment || (held == "@*" && segment.starts_with('@'))
        })
}

/// The path a `[deferred.<widget>]` key defers: a part's field where the key
/// is `<part>.<field>`, else a widget value.
fn deferred_path(widget: &str, name: &str) -> String {
    if name.contains('.') {
        format!("{widget}.{name}")
    } else {
        format!("{widget}.values.{name}")
    }
}

/// The strings of a list, or why it is none.
fn list(value: &Value, what: &str) -> Result<Vec<String>, String> {
    let items = value
        .as_array()
        .ok_or_else(|| format!("{what} is not a list"))?;
    items
        .iter()
        .map(|item| {
            item.as_str()
                .filter(|text| !text.is_empty())
                .map(str::to_owned)
                .ok_or_else(|| format!("{what} holds what is no name"))
        })
        .collect()
}

/// The strings of an optional list of a table.
fn listed(table: &Table, key: &str, what: &str) -> Result<Vec<String>, String> {
    table
        .get(key)
        .map_or_else(|| Ok(Vec::new()), |value| list(value, what))
}

/// The later units of `data`, in the file's order.
fn read_later(data: &Table, path: &Path) -> Result<Vec<Later>, String> {
    let Some(tables) = data.get("later") else {
        return Ok(Vec::new());
    };
    let tables = tables
        .as_array()
        .ok_or_else(|| format!("{path:?}: `later` is not an array of tables"))?;
    let mut out = Vec::new();
    for table in tables {
        let table = table
            .as_table()
            .ok_or_else(|| format!("{path:?}: a `later` item is not a table"))?;
        let name = table
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{path:?}: a [[later]] table has no `name`"))?;
        let what = |key: &str| format!("{path:?}: later unit `{name}`'s `{key}`");
        let enabled = table
            .get("enabled")
            .and_then(Value::as_bool)
            .ok_or_else(|| format!("{} is no boolean", what("enabled")))?;
        let sizes = match table.get("sizes") {
            None => false,
            Some(value) => value
                .as_bool()
                .ok_or_else(|| format!("{} is no boolean", what("sizes")))?,
        };
        let mut groups = Vec::new();
        for (entry, value) in table {
            if LATER_KEYS.contains(&entry.as_str()) {
                continue;
            }
            let items = value
                .as_array()
                .ok_or_else(|| format!("{} is not a list of groups", what(entry)))?;
            for item in items {
                let group = item
                    .as_table()
                    .ok_or_else(|| format!("{} holds what is no table", what(entry)))?;
                let presets = match group.get("presets") {
                    None => None,
                    Some(value) => Some(list(value, &what(entry))?),
                };
                groups.push(Group {
                    entry: entry.clone(),
                    presets,
                    variants: listed(group, "variants", &what(entry))?,
                    states: listed(group, "states", &what(entry))?,
                });
            }
        }
        out.push(Later {
            name: name.to_owned(),
            enabled,
            conditions: listed(table, "conditions", &what("conditions"))?,
            directions: listed(table, "directions", &what("directions"))?,
            sizes,
            groups,
        });
    }
    Ok(out)
}

impl Units {
    /// No unit enabled.
    pub fn none() -> Units {
        Units {
            units: Vec::new(),
            later: Vec::new(),
            at_depth: Vec::new(),
            waiting_paths: Vec::new(),
        }
    }

    /// The repository's units, `scripts/widget_units.toml`'s.
    pub fn repository() -> Result<Units, String> {
        Units::read(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../scripts/widget_units.toml"))
    }

    /// The units `path` states, or why it cannot be read.
    pub fn read(path: &Path) -> Result<Units, String> {
        let text = std::fs::read_to_string(path).map_err(|error| format!("{path:?}: {error}"))?;
        let data: Table = toml::from_str(&text).map_err(|error| format!("{path:?}: {error}"))?;
        let table = |name: &str| -> Result<Table, String> {
            match data.get(name) {
                None => Ok(Table::new()),
                Some(Value::Table(table)) => Ok(table.clone()),
                Some(_) => Err(format!("{path:?}: `{name}` is not a table")),
            }
        };
        let mut deferred = Vec::new();
        let mut waiting_paths = Vec::new();
        for (widget, values) in &table("deferred")? {
            let values = values
                .as_table()
                .ok_or_else(|| format!("{path:?}: `deferred.{widget}` is not a table"))?;
            deferred.extend(
                values
                    .keys()
                    .map(|value| segments(&deferred_path(widget, value))),
            );
            for (value, unit) in values {
                let unit = unit.as_str().unwrap_or_default().to_owned();
                waiting_paths.push((segments(&deferred_path(widget, value)), unit));
            }
        }
        for (_, paths) in &table("staged")? {
            for (staged, unit) in paths.as_table().into_iter().flatten() {
                let unit = unit.as_str().unwrap_or_default().to_owned();
                waiting_paths.push((segments(staged), unit));
            }
        }
        let prefixes = table("prefixes")?;
        let staged = table("staged")?;
        let kept = table("kept")?;
        let mut units = Vec::new();
        for (name, entries) in &data {
            if TABLES.contains(&name.as_str()) {
                continue;
            }
            let mut held = list(entries, &format!("{path:?}: unit `{name}`"))?;
            if let Some(more) = prefixes.get(name) {
                held.extend(list(more, &format!("{path:?}: `prefixes.{name}`"))?);
            }
            let mut out = deferred.clone();
            if let Some(paths) = staged.get(name) {
                let paths = paths
                    .as_table()
                    .ok_or_else(|| format!("{path:?}: `staged.{name}` is not a table"))?;
                out.extend(paths.keys().map(|staged| segments(staged)));
            }
            let mut keeps = Vec::new();
            if let Some(presets) = kept.get(name) {
                let presets = presets
                    .as_table()
                    .ok_or_else(|| format!("{path:?}: `kept.{name}` is not a table"))?;
                for (preset, paths) in presets {
                    let paths = list(paths, &format!("{path:?}: `kept.{name}.{preset}`"))?;
                    keeps.push((
                        preset.clone(),
                        paths.iter().map(|held| segments(held)).collect(),
                    ));
                }
            }
            units.push(Unit {
                entries: held.iter().map(|entry| segments(entry)).collect(),
                deferred: out,
                kept: keeps,
            });
        }
        for name in prefixes.keys().chain(staged.keys()).chain(kept.keys()) {
            if !data.contains_key(name) || TABLES.contains(&name.as_str()) {
                return Err(format!("{path:?}: `{name}` names no enabled unit"));
            }
        }
        let mut at_depth = Vec::new();
        for (prefix, conditions) in &table("at_depth")? {
            at_depth.push((
                segments(prefix),
                list(conditions, &format!("{path:?}: `at_depth.{prefix}`"))?,
            ));
        }
        let later = read_later(&data, path)?;
        check_waiting(&data, path, &later)?;
        Ok(Units {
            units,
            later,
            at_depth,
            waiting_paths,
        })
    }

    /// Whether `path` lies in an enabled unit on some preset
    /// ([`Units::hold_on`] with every preset's kept entries).
    pub fn hold(&self, path: &str) -> bool {
        self.hold_on(path, None)
    }

    /// Whether `path` lies in an enabled unit on `preset` (check_facts_trace.py's
    /// `in_unit`): it starts with one of a unit's entries segment by
    /// segment, or is itself a leading part of one, and lies under no path
    /// the unit defers or stages unless the preset keeps it (`[kept]`; with
    /// no preset, any preset).
    pub fn hold_on(&self, path: &str, preset: Option<&str>) -> bool {
        let path = segments(path);
        self.units.iter().any(|unit| {
            let kept = |deferred: &[String]| {
                unit.kept
                    .iter()
                    .filter(|(held, _)| preset.is_none_or(|preset| held == preset))
                    .any(|(_, paths)| {
                        paths
                            .iter()
                            .any(|kept| under(&path, kept) && under(kept, deferred))
                    })
            };
            !unit
                .deferred
                .iter()
                .any(|deferred| under(&path, deferred) && !kept(deferred))
                && unit.entries.iter().any(|entry| {
                    let n = path.len().min(entry.len());
                    path.get(..n) == entry.get(..n)
                })
        })
    }

    /// The first later unit not enabled for which `holds`, with its place
    /// in the file's order.
    fn waiting_one(&self, holds: impl Fn(&Later) -> bool) -> Option<(usize, &str)> {
        self.later
            .iter()
            .enumerate()
            .find(|(_, unit)| !unit.enabled && holds(unit))
            .map(|(at, unit)| (at, unit.name.as_str()))
    }

    /// The later unit `name` while it is not enabled, with its place.
    pub fn waiting(&self, name: &str) -> Option<(usize, &str)> {
        self.waiting_one(|unit| unit.name == name)
    }

    /// The unit a key no enabled unit holds waits for: the one the longest
    /// `[staged]` or `[deferred]` path above it names, else its entry
    /// ([`entry_of`]: an entry of a unit not enabled; check_facts_trace.py's
    /// `key_unit`).
    pub fn key_unit(&self, key: &str) -> String {
        let path = segments(key);
        self.waiting_paths
            .iter()
            .filter(|(prefix, _)| under(&path, prefix))
            .max_by_key(|(prefix, _)| prefix.len())
            .map_or_else(|| entry_of(key).to_owned(), |(_, unit)| unit.clone())
    }

    /// The later unit not enabled that holds a variant (`variant`) or a
    /// state flag of the catalogue entry `entry` on `preset`, with its
    /// place; `None` where the name is at depth (the class `Depth` of
    /// check_facts_trace.py).
    pub fn holder(
        &self,
        entry: &str,
        preset: &str,
        name: &str,
        variant: bool,
    ) -> Option<(usize, &str)> {
        self.waiting_one(|unit| {
            unit.groups.iter().any(|group| {
                group.entry == entry
                    && group
                        .presets
                        .as_ref()
                        .is_none_or(|presets| presets.iter().any(|held| held == preset))
                    && if variant {
                        group.variants.iter().any(|held| held == name)
                    } else {
                        group.states.iter().any(|held| held == name)
                    }
            })
        })
    }

    /// Whether a variant (`variant`) or a state flag of `entry` is at depth
    /// on `preset` ([`Units::holder`]).
    pub fn at_depth(&self, entry: &str, preset: &str, name: &str, variant: bool) -> bool {
        self.holder(entry, preset, name, variant).is_none()
    }

    /// The later unit not enabled that holds a condition for the key `key`,
    /// with its place: none for a scheme or a preset, nor where
    /// `[at_depth]` names the condition for a prefix of the key.
    pub fn condition_holder(&self, condition: &str, key: &str) -> Option<(usize, &str)> {
        if condition.starts_with("scheme:") || condition.starts_with("preset:") {
            return None;
        }
        let key = segments(key);
        if self.at_depth.iter().any(|(prefix, conditions)| {
            under(&key, prefix) && conditions.iter().any(|held| held == condition)
        }) {
            return None;
        }
        self.waiting_one(|unit| unit.conditions.iter().any(|held| held == condition))
    }

    /// Whether a condition is at depth for the key `key`
    /// ([`Units::condition_holder`]).
    pub fn condition_at_depth(&self, condition: &str, key: &str) -> bool {
        self.condition_holder(condition, key).is_none()
    }

    /// The later unit not enabled that holds a direction, with its place.
    pub fn direction_holder(&self, direction: &str) -> Option<(usize, &str)> {
        self.waiting_one(|unit| unit.directions.iter().any(|held| held == direction))
    }

    /// The sizes unit (the later unit holding the sizes) while it is not
    /// enabled, with its place: an element then takes its default size in
    /// force alone and no density (S10).
    pub fn sizes_holder(&self) -> Option<(usize, &str)> {
        self.waiting_one(|unit| unit.sizes)
    }

    /// Whether the sizes unit is enabled ([`Units::sizes_holder`]).
    pub fn sizes_enabled(&self) -> bool {
        self.sizes_holder().is_none()
    }

    /// Whether the flavours unit (the later unit holding the `flavor:`
    /// conditions) is enabled: while it is not, the default flavour's state
    /// models alone are in force (S10).
    pub fn flavours_enabled(&self) -> bool {
        self.waiting_one(|unit| {
            unit.conditions
                .iter()
                .any(|condition| condition.starts_with("flavor:"))
        })
        .is_none()
    }
}
