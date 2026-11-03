//! The enabled units (S17 *While 0.8.0 is built*): `scripts/widget_units.toml`
//! read as `scripts/check_facts_trace.py` reads it (its `read_units`), a path
//! in a unit by that script's `in_unit` rule, so the gate and the trace check
//! enable the same units from one source.

use std::path::Path;

use toml::{Table, Value};

/// The units file's tables that are no unit.
const TABLES: [&str; 4] = ["deferred", "hand_written", "prefixes", "staged"];

/// The unit suffixes a path's segment drops before it is compared (S1).
const SUFFIXES: [&str; 4] = ["_px", "_ms", "_deg", "_pt"];

/// One enabled unit: its catalogue entries and prefixes, and the paths it
/// holds none of (the values a unit defers, the paths it stages), each by
/// its segments.
#[derive(Debug)]
struct Unit {
    entries: Vec<Vec<String>>,
    deferred: Vec<Vec<String>>,
}

/// The enabled units.
#[derive(Debug)]
pub struct Units(Vec<Unit>);

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

impl Units {
    /// No unit enabled.
    pub fn none() -> Units {
        Units(Vec::new())
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
        for (widget, values) in &table("deferred")? {
            let values = values
                .as_table()
                .ok_or_else(|| format!("{path:?}: `deferred.{widget}` is not a table"))?;
            deferred.extend(
                values
                    .keys()
                    .map(|value| segments(&format!("{widget}.values.{value}"))),
            );
        }
        let prefixes = table("prefixes")?;
        let staged = table("staged")?;
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
            units.push(Unit {
                entries: held.iter().map(|entry| segments(entry)).collect(),
                deferred: out,
            });
        }
        for name in prefixes.keys().chain(staged.keys()) {
            if !data.contains_key(name) || TABLES.contains(&name.as_str()) {
                return Err(format!("{path:?}: `{name}` names no enabled unit"));
            }
        }
        Ok(Units(units))
    }

    /// Whether `path` lies in an enabled unit (check_facts_trace.py's
    /// `in_unit`): it starts with one of a unit's entries segment by
    /// segment, or is itself a leading part of one, and lies under no path
    /// the unit defers or stages.
    pub fn hold(&self, path: &str) -> bool {
        let path = segments(path);
        self.0.iter().any(|unit| {
            !unit
                .deferred
                .iter()
                .any(|deferred| path.starts_with(deferred))
                && unit.entries.iter().any(|entry| {
                    let n = path.len().min(entry.len());
                    path.get(..n) == entry.get(..n)
                })
        })
    }
}
