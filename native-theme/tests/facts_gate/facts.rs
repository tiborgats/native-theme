//! The facts (S17 *Format*): each `[[fact]]` of a platform directory's
//! files, in file order, and the tolerance it is compared at for a preset.

use std::path::{Path, PathBuf};

use native_theme::next::gate::Tolerance;
use toml::{Table, Value};

/// What a fact states: a value, or a pending measurement's register id.
#[derive(Clone, Debug, PartialEq)]
pub enum Stated {
    Value(Value),
    Pending(String),
}

/// One `[[fact]]`.
#[derive(Clone, Debug)]
pub struct Fact {
    /// Its file and its place among the file's facts, for the report.
    pub file: String,
    pub index: usize,
    pub key: String,
    pub when: Option<Value>,
    pub stated: Stated,
    pub cites: Vec<Table>,
    /// It carries `scheme_file` and `accent`: the accent run's alone (S17).
    pub accent_run: bool,
}

impl Fact {
    /// Where the fact is written, for the report.
    pub fn place(&self) -> String {
        format!("{} fact {}", self.file, self.index + 1)
    }

    /// Whether its value is a derivation: an expression (S17).
    pub fn derivation(&self) -> bool {
        matches!(&self.stated, Stated::Value(Value::String(text)) if text.starts_with('='))
    }
}

/// The fact files of a platform directory, every `*.toml` but its state
/// models, by name.
pub fn files(directory: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(directory)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| {
                    path.extension().is_some_and(|ext| ext == "toml")
                        && path
                            .file_name()
                            .is_some_and(|name| name != "state-models.toml")
                })
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}

/// The facts of one file, in file order.
pub fn read(file: &str, text: &str) -> Result<Vec<Fact>, String> {
    let table: Table = toml::from_str(text).map_err(|error| format!("{file}: {error}"))?;
    let entries = table
        .get("fact")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut out = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        let Some(entry) = entry.as_table() else {
            return Err(format!("{file}: fact {} is not a table", index + 1));
        };
        let key = entry
            .get("key")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{file}: fact {} has no key", index + 1))?
            .to_owned();
        let stated = match (entry.get("value"), entry.get("pending")) {
            (Some(value), None) => Stated::Value(value.clone()),
            (None, Some(Value::String(id))) => Stated::Pending(id.clone()),
            _ => {
                return Err(format!(
                    "{file}: fact {} states neither one value nor one pending id",
                    index + 1
                ));
            }
        };
        let cites = entry
            .get("cite")
            .and_then(Value::as_array)
            .map(|cites| cites.iter().filter_map(Value::as_table).cloned().collect())
            .unwrap_or_default();
        out.push(Fact {
            file: file.to_owned(),
            index,
            key,
            when: entry.get("when").cloned(),
            stated,
            cites,
            accent_run: entry.contains_key("scheme_file") || entry.contains_key("accent"),
        });
    }
    Ok(out)
}

/// The generations a `measured` citation's evidence serves: its run's
/// `run.toml` maps the image folder to its preset (S17 *`evidence`*). The
/// evidence path is the repository's, `root` its root.
fn evidence_preset(root: &Path, evidence: &str) -> Option<String> {
    let file = evidence.split_once('#').map_or(evidence, |(file, _)| file);
    let parts: Vec<&str> = file.split('/').collect();
    // docs/evidence/<harness>/<date>-run-<id>/<image>/…
    let (run, image) = match parts.as_slice() {
        ["docs", "evidence", harness, run, image, ..] => {
            (format!("docs/evidence/{harness}/{run}"), *image)
        }
        _ => return None,
    };
    let text = std::fs::read_to_string(root.join(run).join("run.toml")).ok()?;
    let table: Table = toml::from_str(&text).ok()?;
    table
        .get("images")?
        .get(image)?
        .get("preset")?
        .as_str()
        .map(str::to_owned)
}

/// A citation's measured tolerance: one number, or one per leaf.
fn measured(cite: &Table) -> Option<Tolerance> {
    match cite.get("tolerance")? {
        Value::Integer(n) => Some(Tolerance::every(*n as f64)),
        Value::Float(x) => Some(Tolerance::every(*x)),
        Value::Table(leaves) => Some(Tolerance::per_leaf(
            leaves
                .iter()
                .filter_map(|(leaf, value)| {
                    let number = match value {
                        Value::Integer(n) => *n as f64,
                        Value::Float(x) => *x,
                        _ => return None,
                    };
                    Some((leaf.clone(), number))
                })
                .collect(),
        )),
        _ => None,
    }
}

/// The tolerance `fact` is compared at for `preset` (S17 *The gate's
/// tolerance*): the smallest of the measured citations serving the preset,
/// else the quote check's precision.
pub fn tolerance(fact: &Fact, root: &Path, preset: &str) -> Tolerance {
    fact.cites
        .iter()
        .filter(|cite| cite.get("kind").and_then(Value::as_str) == Some("measured"))
        .filter(|cite| {
            cite.get("evidence")
                .and_then(Value::as_str)
                .and_then(|evidence| evidence_preset(root, evidence))
                .is_some_and(|served| served == preset)
        })
        .filter_map(measured)
        .reduce(Tolerance::smallest)
        .unwrap_or_else(Tolerance::quote)
}

/// A fact's `when` for `preset` (S17, decision 7): its `preset:` conditions
/// decided here, as no other preset's hold, and left out. `None` where it
/// cannot hold for the preset; `Some(true)` with the `when` where it named
/// a preset.
pub fn for_preset(when: Option<&Value>, preset: &str) -> Option<(Option<Value>, bool)> {
    let Some(when) = when else {
        return Some((None, false));
    };
    let mut when = when.clone();
    let mut named = false;
    if let Some(table) = when.as_table_mut() {
        for (key, holds) in [("conditions", true), ("not_conditions", false)] {
            let Some(list) = table.get_mut(key).and_then(Value::as_array_mut) else {
                continue;
            };
            let mut kept = Vec::new();
            for item in list.drain(..) {
                match item.as_str().and_then(|text| text.strip_prefix("preset:")) {
                    Some(id) => {
                        named = true;
                        if (id == preset) != holds {
                            return None;
                        }
                    }
                    None => kept.push(item),
                }
            }
            *list = kept;
        }
    }
    Some((Some(when), named))
}

/// One pass of a measured derivation (S17 *A measured derivation*): the
/// varied inputs, each by its name below `input.`, the measured result, the
/// pass's conditions and the citation's tolerance.
#[derive(Clone, Debug)]
pub struct DerivationPass {
    pub inputs: Vec<(String, Value)>,
    pub result: Value,
    pub conditions: Vec<String>,
    pub tolerance: Tolerance,
}

/// The passes of `fact`'s measured citations with `inputs` that serve
/// `preset`, read from their evidence (a list of tables at the path); a
/// citation whose evidence the gate cannot read is an error.
pub fn derivation_passes(
    fact: &Fact,
    root: &Path,
    preset: &str,
) -> Result<Vec<DerivationPass>, String> {
    let mut out = Vec::new();
    for cite in &fact.cites {
        if cite.get("kind").and_then(Value::as_str) != Some("measured")
            || !cite.contains_key("inputs")
        {
            continue;
        }
        let Some(evidence) = cite.get("evidence").and_then(Value::as_str) else {
            continue;
        };
        if evidence_preset(root, evidence).as_deref() != Some(preset) {
            continue;
        }
        let (file, path) = evidence
            .split_once('#')
            .ok_or_else(|| format!("{}: `{evidence}` names no path in its file", fact.place()))?;
        let text = std::fs::read_to_string(root.join(file))
            .map_err(|error| format!("{}: {file}: {error}", fact.place()))?;
        let table: Table =
            toml::from_str(&text).map_err(|error| format!("{}: {file}: {error}", fact.place()))?;
        let mut at = Value::Table(table);
        for segment in path.split('.') {
            at = at
                .get(segment)
                .cloned()
                .ok_or_else(|| format!("{}: {file} holds no `{path}`", fact.place()))?;
        }
        let passes = at
            .as_array()
            .ok_or_else(|| format!("{}: {file}#{path} is not a list of passes", fact.place()))?;
        let tolerance = measured(cite).unwrap_or_else(Tolerance::quote);
        let names: Vec<String> = cite
            .get("inputs")
            .and_then(Value::as_array)
            .map(|names| {
                names
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default();
        for pass in passes {
            let inputs = names
                .iter()
                .filter_map(|name| {
                    let value = pass.get(name)?.clone();
                    Some((
                        name.strip_prefix("input.").unwrap_or(name).to_owned(),
                        value,
                    ))
                })
                .collect();
            let result = pass.get("result").cloned().ok_or_else(|| {
                format!("{}: a pass of {file}#{path} has no result", fact.place())
            })?;
            out.push(DerivationPass {
                inputs,
                result,
                conditions: super::model::strings(pass.get("conditions")),
                tolerance: tolerance.clone(),
            });
        }
    }
    Ok(out)
}
