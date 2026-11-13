//! The facts (S17 *Format*): each `[[fact]]` of a platform directory's
//! files, in file order, and the tolerance it is compared at for a preset.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use native_theme::gate::{Tolerance, is_derivation};
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
    /// The line of its `[[fact]]` header in its file, from 1.
    pub line: usize,
    pub key: String,
    pub when: Option<Value>,
    pub stated: Stated,
    pub cites: Vec<Table>,
    /// It carries `scheme_file` or `accent`: the accent run's alone (S17).
    pub accent_run: bool,
    /// The scheme file it was measured with, `<entry> <path>` (S17 *The
    /// accent run*).
    pub scheme_file: Option<String>,
    /// The accent it was measured with, a colour literal.
    pub accent: Option<String>,
    /// Every unit it belongs to is enabled (S17 *While 0.8.0 is built*);
    /// set as its platform is read.
    pub enabled: bool,
}

impl Fact {
    /// Where the fact is written, for the report.
    pub fn place(&self) -> String {
        format!("{} fact {}", self.file, self.index + 1)
    }

    /// Its file relative to the repository root and the line of its
    /// `[[fact]]` header, as `check_facts_trace.py --facts` names a fact.
    pub fn line_place(&self) -> String {
        format!("docs/facts/{}:{}", self.file, self.line)
    }

    /// Whether its value is a derivation: an expression, as the value or in
    /// a leaf of it (S17).
    pub fn derivation(&self) -> bool {
        matches!(&self.stated, Stated::Value(value) if is_derivation(value))
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

/// The facts of one file, in file order, each with the line of its
/// `[[fact]]` header; a file whose facts are not each one such line is
/// refused, as no fact's place could be named.
pub fn read(file: &str, text: &str) -> Result<Vec<Fact>, String> {
    let table: Table = toml::from_str(text).map_err(|error| format!("{file}: {error}"))?;
    let entries = table
        .get("fact")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let lines: Vec<usize> = text
        .lines()
        .zip(1_usize..)
        .filter(|(line, _)| line.trim_end() == "[[fact]]")
        .map(|(_, number)| number)
        .collect();
    if lines.len() != entries.len() {
        return Err(format!(
            "{file}: {} facts and {} `[[fact]]` lines: each fact is one `[[fact]]` table",
            entries.len(),
            lines.len()
        ));
    }
    let mut out = Vec::new();
    for ((index, entry), line) in entries.iter().enumerate().zip(lines) {
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
        let text = |field: &str| match entry.get(field) {
            None => Ok(None),
            Some(Value::String(text)) => Ok(Some(text.clone())),
            Some(_) => Err(format!(
                "{file}: fact {}'s `{field}` is no string",
                index + 1
            )),
        };
        let (scheme_file, accent) = (text("scheme_file")?, text("accent")?);
        out.push(Fact {
            file: file.to_owned(),
            index,
            line,
            key,
            when: entry.get("when").cloned(),
            stated,
            cites,
            accent_run: scheme_file.is_some() || accent.is_some(),
            scheme_file,
            accent,
            enabled: true,
        });
    }
    Ok(out)
}

/// The evidence files the citations read, each read and parsed once however
/// many citations name it: thousands of citations name a few files of
/// hundreds of kilobytes.
pub struct Evidence {
    /// The evidence root ([`evidence_root`], or a fixture's root), which the
    /// evidence paths (`docs/evidence/…`) are relative to.
    root: PathBuf,
    files: Mutex<HashMap<String, Arc<EvidenceFile>>>,
}

/// One evidence file: its text, or why it does not read; its parses made
/// when first asked for.
struct EvidenceFile {
    text: Result<String, String>,
    toml: OnceLock<Result<Table, String>>,
    json: OnceLock<Option<serde_json::Value>>,
}

impl Evidence {
    /// The evidence under the evidence root `root`, none read yet.
    pub fn new(root: &Path) -> Evidence {
        Evidence {
            root: root.to_path_buf(),
            files: Mutex::new(HashMap::new()),
        }
    }

    /// The file at `path`, below the root, read on the first call.
    fn file(&self, path: &str) -> Arc<EvidenceFile> {
        let mut files = self
            .files
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        Arc::clone(files.entry(path.to_owned()).or_insert_with(|| {
            Arc::new(EvidenceFile {
                text: std::fs::read_to_string(self.root.join(path))
                    .map_err(|error| error.to_string()),
                toml: OnceLock::new(),
                json: OnceLock::new(),
            })
        }))
    }
}

/// The variable naming a checkout of the evidence repository other than the
/// source cache's (spec S17 *The evidence root*).
pub const EVIDENCE_VARIABLE: &str = "NATIVE_THEME_EVIDENCE";

/// The variable naming the source cache, and its default: those of
/// `scripts/citations.py` (`cache_dir`).
const SOURCES_VARIABLE: &str = "NATIVE_THEME_SOURCES";
const DEFAULT_CACHE: &str = "/run/media/tibi/work/cargo-build/native-theme-sources";

/// The evidence repository's entry id in `sources.toml`, its folder in the
/// source cache.
const EVIDENCE_ID: &str = "native-theme-evidence";

/// The evidence root of the facts root `facts` outside a fixture (spec S17
/// *The evidence root*): the directory [`EVIDENCE_VARIABLE`] names, else
/// the pin's checkout in the source cache, as [`evidence_root_at`] checks
/// it.
pub fn evidence_root(facts: &Path) -> Result<PathBuf, String> {
    let set = |name: &str| std::env::var_os(name).filter(|value| !value.is_empty());
    let cache = set(SOURCES_VARIABLE).map_or_else(|| PathBuf::from(DEFAULT_CACHE), PathBuf::from);
    evidence_root_at(facts, set(EVIDENCE_VARIABLE).map(PathBuf::from), &cache)
}

/// The commit `sources.toml` (its text) pins the evidence repository at:
/// its one `git` entry with `evidence = true`.
pub fn evidence_pin(sources: &str) -> Result<String, String> {
    let table: Table = toml::from_str(sources).map_err(|error| format!("sources.toml: {error}"))?;
    let pins: Vec<&Value> = table
        .get("source")
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter(|entry| {
                    entry.get("kind").and_then(Value::as_str) == Some("git")
                        && entry.get("evidence").and_then(Value::as_bool) == Some(true)
                })
                .collect()
        })
        .unwrap_or_default();
    match pins.as_slice() {
        [pin] => pin
            .get("commit")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| "sources.toml: the evidence entry has no commit".to_owned()),
        _ => Err(format!(
            "sources.toml: {} evidence entries (a git [[source]] with `evidence = true`), not one",
            pins.len()
        )),
    }
}

/// The evidence root of the facts root `facts`: `named` (the variable's
/// directory), else `<cache>/native-theme-evidence`. It must be the top of
/// a git checkout whose HEAD is the commit `sources.toml` pins
/// ([`evidence_pin`]) and which is clean, `git status --porcelain --ignored
/// --untracked-files=all` printing nothing (ignored files count: the gate
/// reads files on disk); otherwise an error naming the root and the pin, so
/// the gate never runs without its evidence.
pub fn evidence_root_at(
    facts: &Path,
    named: Option<PathBuf>,
    cache: &Path,
) -> Result<PathBuf, String> {
    let sources = std::fs::read_to_string(facts.join("sources.toml"))
        .map_err(|error| format!("{}: {error}", facts.join("sources.toml").display()))?;
    let commit = evidence_pin(&sources)?;
    let (root, place, fix) = match named {
        Some(root) => {
            let place = format!("{EVIDENCE_VARIABLE}={}", root.display());
            (
                root,
                place,
                format!("check {EVIDENCE_ID} out there at {commit}"),
            )
        }
        None => {
            let root = cache.join(EVIDENCE_ID);
            let place = format!(
                "{} (the source cache's checkout of {EVIDENCE_ID})",
                root.display()
            );
            (
                root,
                place,
                format!("fetch it with scripts/update_sources.sh --only {EVIDENCE_ID}"),
            )
        }
    };
    if !root.is_dir() {
        return Err(format!(
            "no evidence root: {place} is no directory; the pin is {commit} ({fix})"
        ));
    }
    let git = |args: &[&str]| -> Result<Option<String>, String> {
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(&root)
            .args(args)
            .output()
            .map_err(|error| format!("the evidence root {place}: git: {error}"))?;
        Ok(output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).into_owned()))
    };
    if git(&["rev-parse", "--show-prefix"])?.as_deref() != Some("\n") {
        return Err(format!(
            "the evidence root {place} is not the top of a git checkout; the pin is {commit} ({fix})"
        ));
    }
    let head = git(&["rev-parse", "--verify", "-q", "HEAD"])?.unwrap_or_default();
    let head = head.trim();
    if head != commit {
        return Err(format!(
            "the evidence root {place} is at {}, not at the pin {commit} ({fix})",
            if head.is_empty() { "no commit" } else { head }
        ));
    }
    let status = git(&[
        "status",
        "--porcelain",
        "--ignored",
        "--untracked-files=all",
    ])?;
    match status.as_deref().map(str::trim) {
        Some("") => Ok(root),
        status => {
            let changed: Vec<&str> = status.map(str::lines).into_iter().flatten().collect();
            let first = changed
                .first()
                .map(|line| format!("; first: {}", line.trim()))
                .unwrap_or_default();
            Err(format!(
                "the evidence root {place} at the pin {commit} is not clean: {} paths changed, \
                 untracked or ignored{first}",
                changed.len()
            ))
        }
    }
}

/// Where the accent run reads a scheme file that is not the preset's own
/// (S17 *The scheme files read*).
#[derive(Clone, Debug)]
pub enum Sources {
    /// The source cache: each entry's checkout at `<cache>/<entry>`, at the
    /// commit `sources.toml` pins, the file tracked and unchanged.
    Cache(PathBuf),
    /// A fixture's own files, at `<folder>/<entry>/<path>`: no checkout and
    /// no pin, each written for the fixture.
    Fixture(PathBuf),
}

/// The source cache the gate reads the scheme files from outside a fixture:
/// the folder `NATIVE_THEME_SOURCES` names, else `scripts/citations.py`'s
/// default, the cache [`evidence_root`] reads.
pub fn sources() -> Sources {
    let cache = std::env::var_os(SOURCES_VARIABLE).filter(|value| !value.is_empty());
    Sources::Cache(cache.map_or_else(|| PathBuf::from(DEFAULT_CACHE), PathBuf::from))
}

/// A `scheme_file`'s entry and path (`<entry> <path>`, S17 *Fields*).
pub fn scheme_file_parts(scheme_file: &str) -> Option<(&str, &str)> {
    scheme_file
        .split_once(' ')
        .filter(|(entry, path)| !entry.is_empty() && !path.is_empty() && !path.contains(' '))
}

/// The commit `sources.toml` (its text) pins the git entry `id` at.
fn entry_pin(sources: &str, id: &str) -> Result<String, String> {
    let table: Table = toml::from_str(sources).map_err(|error| format!("sources.toml: {error}"))?;
    let entries: Vec<&Value> = table
        .get("source")
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter(|entry| entry.get("id").and_then(Value::as_str) == Some(id))
                .collect()
        })
        .unwrap_or_default();
    match entries.as_slice() {
        [entry] if entry.get("kind").and_then(Value::as_str) == Some("git") => entry
            .get("commit")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| format!("sources.toml: the entry {id} has no commit")),
        [_] => Err(format!("sources.toml: the entry {id} is no git entry")),
        _ => Err(format!(
            "sources.toml: {} entries {id}, not one",
            entries.len()
        )),
    }
}

impl Sources {
    /// The bytes of the scheme file `scheme_file` (`<entry> <path>`) for the
    /// facts root `facts`. From the cache, the entry's checkout must be the
    /// top of a git checkout at the commit `sources.toml` pins, with the
    /// file tracked and unchanged, which the gate checks itself through git
    /// as it checks the evidence root; otherwise an error naming the entry,
    /// so the run never compares nothing.
    pub fn read(&self, facts: &Path, scheme_file: &str) -> Result<Vec<u8>, String> {
        let (entry, path) = scheme_file_parts(scheme_file)
            .ok_or_else(|| format!("the scheme file `{scheme_file}` is no `<entry> <path>`"))?;
        let cache = match self {
            Sources::Fixture(folder) => {
                let file = folder.join(entry).join(path);
                return std::fs::read(&file)
                    .map_err(|error| format!("{}: {error}", file.display()));
            }
            Sources::Cache(cache) => cache,
        };
        let text = std::fs::read_to_string(facts.join("sources.toml"))
            .map_err(|error| format!("{}: {error}", facts.join("sources.toml").display()))?;
        let commit = entry_pin(&text, entry)?;
        let root = cache.join(entry);
        let fix = format!("fetch it with scripts/update_sources.sh --only {entry}");
        let place = format!(
            "the scheme file {scheme_file}: {} (the source cache's checkout of {entry})",
            root.display()
        );
        if !root.is_dir() {
            return Err(format!(
                "{place} is no directory; the pin is {commit} ({fix})"
            ));
        }
        let git = |args: &[&str]| -> Result<Option<String>, String> {
            let output = std::process::Command::new("git")
                .arg("-C")
                .arg(&root)
                .args(args)
                .output()
                .map_err(|error| format!("{place}: git: {error}"))?;
            Ok(output
                .status
                .success()
                .then(|| String::from_utf8_lossy(&output.stdout).into_owned()))
        };
        if git(&["rev-parse", "--show-prefix"])?.as_deref() != Some("\n") {
            return Err(format!(
                "{place} is not the top of a git checkout; the pin is {commit} ({fix})"
            ));
        }
        let head = git(&["rev-parse", "--verify", "-q", "HEAD"])?.unwrap_or_default();
        let head = head.trim();
        if head != commit {
            return Err(format!(
                "{place} is at {}, not at the pin {commit} ({fix})",
                if head.is_empty() { "no commit" } else { head }
            ));
        }
        if git(&["ls-files", "--error-unmatch", "--", path])?.is_none() {
            return Err(format!("{place} does not track {path} at the pin {commit}"));
        }
        match git(&["status", "--porcelain", "--", path])?
            .as_deref()
            .map(str::trim)
        {
            Some("") => {}
            _ => {
                return Err(format!(
                    "{place}: {path} is changed at the pin {commit} ({fix})"
                ));
            }
        }
        let file = root.join(path);
        std::fs::read(&file).map_err(|error| format!("{}: {error}", file.display()))
    }
}

impl EvidenceFile {
    /// Its text parsed as TOML, or why it does not read or parse.
    fn toml(&self) -> Result<&Table, String> {
        self.toml
            .get_or_init(|| {
                let text = self.text.as_ref().map_err(String::clone)?;
                toml::from_str(text).map_err(|error| error.to_string())
            })
            .as_ref()
            .map_err(String::clone)
    }

    /// Its text parsed as JSON, where it reads and parses.
    fn json(&self) -> Option<&serde_json::Value> {
        self.json
            .get_or_init(|| {
                let text = self.text.as_ref().ok()?;
                serde_json::from_str(text).ok()
            })
            .as_ref()
    }
}

/// The generations a `measured` citation's evidence serves: its run's
/// `run.toml` maps the image folder to its preset (S17 *`evidence`*). The
/// evidence path is a `docs/evidence/…` path under the evidence root.
fn evidence_preset(files: &Evidence, evidence: &str) -> Option<String> {
    let file = evidence.split_once('#').map_or(evidence, |(file, _)| file);
    let parts: Vec<&str> = file.split('/').collect();
    // docs/evidence/<harness>/<date>-run-<id>/<image>/…
    let (run, image) = match parts.as_slice() {
        ["docs", "evidence", harness, run, image, ..] => {
            (format!("docs/evidence/{harness}/{run}"), *image)
        }
        _ => return None,
    };
    let file = files.file(&format!("{run}/run.toml"));
    file.toml()
        .ok()?
        .get("images")?
        .get(image)?
        .get("preset")?
        .as_str()
        .map(str::to_owned)
}

/// The extensions of an image an analysis reads pixels from.
const IMAGE_EXTENSIONS: [&str; 9] = [
    "png", "jpg", "jpeg", "bmp", "gif", "tif", "tiff", "webp", "ppm",
];

/// Whether a `measured` citation's evidence was read from drawn pixels (S17
/// *`evidence`*): an analysis output whose top-level `input_image` names an
/// image (one of IMAGE_EXTENSIONS). A value an API read (a probe's own
/// value, a kdeglobals key, an analysis over such reads, whose
/// `input_image` names those files) is not, and compares as read; nor is a
/// GNOME render-node file (`.node`) or an analysis of one, which holds the
/// colour's OKLab coordinates unrounded, converted as the colour function
/// converts them (S17), no pixel drawn. The evidence path is a
/// `docs/evidence/…` path under the evidence root.
fn drawn_pixels(files: &Evidence, evidence: &str) -> bool {
    let file = evidence.split_once('#').map_or(evidence, |(file, _)| file);
    let drawn = |name: &str| {
        Path::new(name)
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                IMAGE_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str())
            })
    };
    let read = files.file(file);
    if read.text.is_err() {
        return false;
    }
    let names: Vec<String> = if file.ends_with(".json") {
        let Some(document) = read.json() else {
            return false;
        };
        match document.get("input_image") {
            Some(serde_json::Value::String(name)) => vec![name.clone()],
            Some(serde_json::Value::Array(names)) => names
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_owned)
                .collect(),
            _ => Vec::new(),
        }
    } else if file.ends_with(".toml") {
        let Ok(table) = read.toml() else {
            return false;
        };
        match table.get("input_image") {
            Some(Value::String(name)) => vec![name.clone()],
            Some(Value::Array(names)) => names
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect(),
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    };
    names.iter().any(|name| drawn(name))
}

/// A citation's measured tolerance: one number, or one per leaf; its
/// colours compared as drawn where its evidence was read from drawn pixels
/// ([`drawn_pixels`]).
fn measured(files: &Evidence, cite: &Table) -> Option<Tolerance> {
    let tolerance = measured_tolerance(cite)?;
    let drawn = cite
        .get("evidence")
        .and_then(Value::as_str)
        .is_some_and(|evidence| drawn_pixels(files, evidence));
    Some(if drawn {
        tolerance.from_drawn_pixels()
    } else {
        tolerance
    })
}

/// A citation's stated tolerance: one number, or one per leaf.
fn measured_tolerance(cite: &Table) -> Option<Tolerance> {
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

/// The paths, as S11 spells them in `value`, of its leaves in points
/// (S9.4): `""` for a value `"<n>pt"`, a table's `_pt`-suffixed keys.
pub fn points(value: &Value) -> Vec<String> {
    fn walk(value: &Value, at: &str, out: &mut Vec<String>) {
        let below = |key: &str| {
            if at.is_empty() {
                key.to_owned()
            } else {
                format!("{at}.{key}")
            }
        };
        match value {
            Value::String(text)
                if text
                    .strip_suffix("pt")
                    .is_some_and(|number| number.trim().parse::<f64>().is_ok()) =>
            {
                out.push(at.to_owned());
            }
            Value::Table(table) => {
                for (key, item) in table {
                    if key.ends_with("_pt") && !item.is_table() && !item.is_array() {
                        out.push(below(key));
                    } else {
                        walk(item, &below(key), out);
                    }
                }
            }
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    walk(item, &below(&index.to_string()), out);
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(value, "", &mut out);
    out
}

/// The tolerance `fact` is compared at for `preset` (S17 *The gate's
/// tolerance*): the smallest of the measured citations serving the preset,
/// else the quote check's precision; its leaves in points marked so, as
/// the value is compared in px (S9.4).
pub fn tolerance(fact: &Fact, files: &Evidence, preset: &str) -> Tolerance {
    let points = match &fact.stated {
        Stated::Value(value) => points(value),
        Stated::Pending(_) => Vec::new(),
    };
    fact.cites
        .iter()
        .filter(|cite| cite.get("kind").and_then(Value::as_str) == Some("measured"))
        .filter(|cite| {
            cite.get("evidence")
                .and_then(Value::as_str)
                .and_then(|evidence| evidence_preset(files, evidence))
                .is_some_and(|served| served == preset)
        })
        .filter_map(|cite| measured(files, cite))
        .reduce(Tolerance::smallest)
        .unwrap_or_else(Tolerance::quote)
        .in_points(&points)
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
    files: &Evidence,
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
        if evidence_preset(files, evidence).as_deref() != Some(preset) {
            continue;
        }
        let (file, path) = evidence
            .split_once('#')
            .ok_or_else(|| format!("{}: `{evidence}` names no path in its file", fact.place()))?;
        let read = files.file(file);
        let table = read
            .toml()
            .map_err(|error| format!("{}: {file}: {error}", fact.place()))?;
        let mut segments = path.split('.');
        let mut at = segments
            .next()
            .and_then(|segment| table.get(segment))
            .ok_or_else(|| format!("{}: {file} holds no `{path}`", fact.place()))?;
        for segment in segments {
            at = at
                .get(segment)
                .ok_or_else(|| format!("{}: {file} holds no `{path}`", fact.place()))?;
        }
        let passes = at
            .as_array()
            .ok_or_else(|| format!("{}: {file}#{path} is not a list of passes", fact.place()))?;
        let tolerance = measured(files, cite).unwrap_or_else(Tolerance::quote);
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
            let tolerance = tolerance.clone().in_points(&points(&result));
            out.push(DerivationPass {
                inputs,
                result,
                conditions: super::model::strings(pass.get("conditions")),
                tolerance,
            });
        }
    }
    Ok(out)
}
