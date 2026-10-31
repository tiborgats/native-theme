//! The gate's run (S17 *The gate*): every preset of a platform directory
//! resolved with a stand-in and no reader, each widget's combinations
//! queried and each key compared with the fact that applies; the perturbed
//! run; the declarations run; and the report.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::Path;
use std::time::{Duration, Instant};

use native_theme::next::gate::{
    Combination, Conditions, FactKey, FactValue, Outcome, Preset, Resolved, SetWhole, Tolerance,
    Unread, When, parse_key, parse_value, parse_when, set_whole, widgets,
};
use native_theme::next::{FontMetrics, Layer, Px, ResolvedFont, RootInputs, Size};
use toml::Value;

use super::facts::{self, Fact, Stated};
use super::model::{self, Mentioned, Model, Point, strings};

/// The gate's font measurement (S17 *The gate*, the design's "fixed,
/// documented stub"): every metric a fixed share of the font's size, so a
/// value reading a metric is compared over the same number on both sides.
/// Synthetic, no toolkit's.
pub struct StubMetrics;

fn size_of(font: &ResolvedFont) -> f32 {
    font.size.map_or(0.0, |Px(size)| size)
}

impl FontMetrics for StubMetrics {
    fn char_width(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) * 0.5)
    }
    fn max_char_width(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font))
    }
    fn x_height(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) * 0.5)
    }
    fn height(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) * 1.25)
    }
    fn line_spacing(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) * 1.5)
    }
    fn underline_offset(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) * 0.125)
    }
    fn underline_thickness(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) * 0.0625)
    }
    fn strikeout_offset(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) * 0.25)
    }
    fn strikeout_thickness(&self, font: &ResolvedFont) -> Px {
        Px(size_of(font) * 0.0625)
    }
}

/// The root every combination is queried in: a window and a screen of
/// fixed, synthetic sizes, left to right, no display radii.
pub fn root_inputs() -> RootInputs {
    let size = |width, height| Size {
        width: Px(width),
        height: Px(height),
    };
    RootInputs::new(
        size(1024.0, 768.0),
        size(1920.0, 1080.0),
        native_theme::next::Direction::Ltr,
    )
}

/// The perturbed run's colour for its `n`th operand (S17 *The perturbed
/// run*): `n` times an odd multiplier plus an offset, modulo 2^24, a map
/// that is one-to-one, so no two operands share a colour, and fixed, so the
/// run is reproducible; synthetic.
pub fn perturbed(n: usize) -> String {
    let rgb = (n as u64 * 0x9e_3779 + 0x13_579b) % (1 << 24);
    format!("#{rgb:06x}")
}

/// One of the gate's three runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pass {
    /// The no-reader state's conditions, the stand-in stating nothing.
    Normal,
    /// The stand-in stating a fixed colour for every declared input and
    /// every colour role a derivation names; only derivations compared.
    Perturbed,
    /// A measured derivation's evidence passes, each against its result.
    Measured,
}

/// What a run found (S17): the failures, the pending facts met (listed
/// with their register ids, apart from the failures), and what it skipped
/// while 0.8.0 is built.
#[derive(Debug, Default)]
pub struct Report {
    pub failures: Vec<String>,
    /// Each pending fact met: its place, its id, its key, and how many
    /// combinations it applied in.
    pub pending: BTreeMap<String, (String, String, u64)>,
    /// The values the model's reader refuses, listed: a failure only for a
    /// preset checked against them.
    pub unread: Vec<String>,
    /// Facts whose key the model does not hold yet, by key.
    pub keys_not_held: BTreeMap<String, usize>,
    /// Facts whose `when` names what the model does not hold yet, by what
    /// it names.
    pub whens_not_held: BTreeMap<String, usize>,
    /// Facts naming a preset no bundled preset is (decision 7).
    pub other_presets: usize,
    /// Facts of the accent run, which this gate does not run.
    pub accent_run: usize,
    /// Combinations skipped: a context or a condition the gate cannot set.
    pub combinations_skipped: u64,
    /// Every state model's combinations, by entry.
    pub combinations: BTreeMap<String, u64>,
    /// The distinct combinations queried, after the projection.
    pub queried: u64,
    /// The keys compared.
    pub compared: u64,
    /// The time the run took.
    pub elapsed: Duration,
    /// The failures recorded, each once.
    seen: HashSet<String>,
}

impl Report {
    fn fail(&mut self, failure: String) {
        if self.seen.insert(failure.clone()) {
            self.failures.push(failure);
        }
    }
}

/// A platform directory's facts and state models, read.
pub struct Platform {
    pub name: String,
    pub facts: Vec<Fact>,
    pub keys: Vec<(String, Result<FactKey, Unread>)>,
    pub models: Vec<Model>,
}

impl Platform {
    /// The directory `directory` of the facts root, read; a file the gate
    /// cannot read is a failure.
    pub fn read(directory: &Path, report: &mut Report) -> Platform {
        let name = directory
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let mut facts = Vec::new();
        for path in facts::files(directory) {
            let file = format!(
                "{name}/{}",
                path.file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default()
            );
            match std::fs::read_to_string(&path)
                .map_err(|error| error.to_string())
                .and_then(|text| facts::read(&file, &text))
            {
                Ok(read) => facts.extend(read),
                Err(error) => report.fail(error),
            }
        }
        let mut keys: Vec<(String, Result<FactKey, Unread>)> = Vec::new();
        let mut files: BTreeMap<&str, &str> = BTreeMap::new();
        for fact in &facts {
            if !keys.iter().any(|(key, _)| *key == fact.key) {
                keys.push((fact.key.clone(), parse_key(&fact.key)));
            }
            // S17 orders a key's facts within its file only; the accent
            // run's facts apply in that run alone.
            if fact.accent_run {
                continue;
            }
            match files.get(fact.key.as_str()) {
                Some(file) if *file != fact.file => report.fail(format!(
                    "`{}` has facts in {file} and {}: a key's facts are one file's, in its order",
                    fact.key, fact.file
                )),
                Some(_) => {}
                None => {
                    files.insert(&fact.key, &fact.file);
                }
            }
        }
        let models_path = directory.join("state-models.toml");
        let models = match std::fs::read_to_string(&models_path) {
            Ok(text) => {
                Model::read(&format!("{name}/state-models.toml"), &text).unwrap_or_else(|error| {
                    report.fail(error);
                    Vec::new()
                })
            }
            Err(_) => Vec::new(),
        };
        Platform {
            name,
            facts,
            keys,
            models,
        }
    }

    fn key(&self, key: &str) -> Option<&Result<FactKey, Unread>> {
        self.keys
            .iter()
            .find(|(held, _)| held == key)
            .map(|(_, parsed)| parsed)
    }
}

/// One fact prepared for a preset: its `when` read with the preset's
/// conditions decided, its value read for its key, its tolerance.
struct Prepared<'f> {
    fact: &'f Fact,
    when: When,
    value: Option<FactValue>,
    /// A set-whole block's leaf's value as the fact states it, composed
    /// with its siblings' per combination ([`composed`]).
    leaf: Option<Value>,
    tolerance: Tolerance,
    /// A measured derivation's passes, each with its result read for the
    /// key.
    passes: Vec<(facts::DerivationPass, FactValue)>,
}

/// A key's facts for a preset, in file order.
struct KeyFacts<'f> {
    name: String,
    key: FactKey,
    /// The set-whole block the key is a leaf of (S6.1, S17).
    set_whole: Option<SetWhole>,
    facts: Vec<Prepared<'f>>,
}

/// The negative lists of a `when` (S11): each holds where none of its
/// entries does.
const NEGATIVE: [&str; 6] = [
    "not_context",
    "not_variants",
    "not_states",
    "not_conditions",
    "not_sizes",
    "not_densities",
];

/// A fact's `when` as the gate reads it (S17): an entry of a negative list
/// naming what the model does not hold yet left out, as no element the gate
/// queries holds it (a `not_context`'s widget is no ancestor of one, a
/// variant, flag, condition, size or density is none it can hold); a
/// positive list naming one is not held, as no element the gate queries
/// holds what it names.
fn read_when(when: Option<&Value>) -> Result<When, Unread> {
    let mut when = when
        .cloned()
        .unwrap_or_else(|| Value::Table(toml::Table::new()));
    if let Some(table) = when.as_table_mut() {
        for list in NEGATIVE {
            if let Some(Value::Array(items)) = table.get_mut(list) {
                items.retain(|item| {
                    let mut probe = toml::Table::new();
                    probe.insert(list.to_owned(), Value::Array(vec![item.clone()]));
                    !matches!(parse_when(&Value::Table(probe)), Err(Unread::NotHeld(_)))
                });
            }
        }
    }
    parse_when(&when)
}

/// The keys of a `when` a token's, a colour role's or an input's fact may
/// state: they are read in the root scope, where no element's flag,
/// variant, size, context, direction or position holds (S17: "under its
/// `when`", its conditions).
const GLOBAL_WHEN: [&str; 2] = ["conditions", "not_conditions"];

/// Every fact of `platform` read as the model reads it, whatever preset it
/// is for (its `preset:` conditions left out): a key, a `when` or a value
/// the model holds but refuses is a failure; what it does not hold yet is
/// counted (S17: skipped while 0.8.0 is built).
pub fn read_all(platform: &Platform, report: &mut Report) {
    for fact in &platform.facts {
        if fact.accent_run {
            report.accent_run += 1;
            continue;
        }
        let key = match platform.key(&fact.key) {
            Some(Ok(key)) => key,
            Some(Err(Unread::NotHeld(_))) | None => {
                *report.keys_not_held.entry(fact.key.clone()).or_default() += 1;
                continue;
            }
            Some(Err(Unread::Refused(why))) => {
                report.fail(format!(
                    "{}: the key `{}` is refused: {why}",
                    fact.place(),
                    fact.key
                ));
                continue;
            }
        };
        let when = fact.when.clone().map(|mut when| {
            if let Some(table) = when.as_table_mut() {
                for list in ["conditions", "not_conditions"] {
                    if let Some(Value::Array(items)) = table.get_mut(list) {
                        items.retain(|item| {
                            !item
                                .as_str()
                                .is_some_and(|text| text.starts_with("preset:"))
                        });
                    }
                }
            }
            when
        });
        if key.widget().is_none()
            && let Some(named) = when
                .as_ref()
                .and_then(Value::as_table)
                .and_then(|table| table.keys().find(|k| !GLOBAL_WHEN.contains(&k.as_str())))
        {
            report.fail(format!(
                "{}: the `when` of `{}` states `{named}`, which a token, role or input fact read \
                 in the root scope cannot hold",
                fact.place(),
                fact.key
            ));
            continue;
        }
        match read_when(when.as_ref()) {
            Ok(_) => {}
            Err(Unread::NotHeld(why)) => {
                *report.whens_not_held.entry(why).or_default() += 1;
                continue;
            }
            Err(Unread::Refused(why)) => {
                report.fail(format!("{}: its `when` is refused: {why}", fact.place()));
                continue;
            }
        }
        if let Stated::Value(value) = &fact.stated
            && set_whole(&fact.key).is_none()
            && let Err(Unread::NotHeld(why) | Unread::Refused(why)) = parse_value(key, value)
        {
            report.unread.push(format!(
                "{}: the value of `{}` is refused: {why}",
                fact.place(),
                fact.key
            ));
        }
    }
}

/// The facts of `platform` that can apply to `preset`, by key, each read;
/// what [`read_all`] counted or failed left out.
fn prepare<'f>(
    platform: &'f Platform,
    preset: &str,
    repo: &Path,
    report: &mut Report,
) -> Vec<KeyFacts<'f>> {
    let mut out: Vec<KeyFacts<'f>> = Vec::new();
    for fact in &platform.facts {
        if fact.accent_run {
            continue;
        }
        let Some(Ok(key)) = platform.key(&fact.key) else {
            continue;
        };
        let Some((when, _)) = facts::for_preset(fact.when.as_ref(), preset) else {
            continue;
        };
        let Ok(when) = read_when(when.as_ref()) else {
            continue;
        };
        let block = set_whole(&fact.key);
        let leaf = match (&fact.stated, &block) {
            (Stated::Value(value), Some(_)) => Some(value.clone()),
            _ => None,
        };
        let value = match &fact.stated {
            Stated::Pending(_) => None,
            Stated::Value(_) if block.is_some() => None,
            Stated::Value(value) => match parse_value(key, value) {
                Ok(value) => Some(value),
                Err(Unread::NotHeld(why) | Unread::Refused(why)) => {
                    report.fail(format!(
                        "{}: the value of `{}`, which {preset} is checked against, is refused: {why}",
                        fact.place(),
                        fact.key
                    ));
                    continue;
                }
            },
        };
        let mut passes = Vec::new();
        match facts::derivation_passes(fact, repo, preset) {
            Ok(read) => {
                for pass in read {
                    match parse_value(key, &pass.result) {
                        Ok(result) => passes.push((pass, result)),
                        Err(Unread::NotHeld(why) | Unread::Refused(why)) => report.fail(format!(
                            "{}: a pass's result of `{}` is refused: {why}",
                            fact.place(),
                            fact.key
                        )),
                    }
                }
            }
            Err(why) => report.fail(why),
        }
        let prepared = Prepared {
            fact,
            when,
            value,
            leaf,
            tolerance: facts::tolerance(fact, repo, preset),
            passes,
        };
        match out.iter_mut().find(|held| held.name == fact.key) {
            Some(held) => held.facts.push(prepared),
            None => out.push(KeyFacts {
                name: fact.key.clone(),
                key: key.clone(),
                set_whole: block,
                facts: vec![prepared],
            }),
        }
    }
    out
}

/// The fact of `facts` that applies where `holds` says which hold: the
/// last in file order (S17 *Which fact applies*); a pending fact shadowed
/// by a later one where both hold is a failure.
fn applying<'p, 'f>(
    facts: &'p [Prepared<'f>],
    holds: impl Fn(&When) -> bool,
    report: &mut Report,
    combination: &dyn Fn() -> String,
) -> Option<&'p Prepared<'f>> {
    let held: Vec<&Prepared<'f>> = facts.iter().filter(|fact| holds(&fact.when)).collect();
    if let Some(last) = held.last() {
        for earlier in &held {
            if let Stated::Pending(id) = &earlier.fact.stated
                && !std::ptr::eq(*earlier, *last)
            {
                report.fail(format!(
                    "{}: the pending fact {id} of `{}` is shadowed by {} under {}",
                    earlier.fact.place(),
                    earlier.fact.key,
                    last.fact.place(),
                    combination()
                ));
            }
        }
    }
    held.last().copied()
}

/// What to do with the fact that applies to a key.
enum Verdict<'p> {
    Compare(&'p FactValue, &'p Tolerance),
    /// No fact of a whole part applies: the part is drawn.
    Drawn,
    Done,
}

fn verdict<'p>(
    key: &str,
    whole_part: bool,
    fact: Option<&'p Prepared<'_>>,
    composed: Option<&'p FactValue>,
    pass: Pass,
    report: &mut Report,
    combination: &dyn Fn() -> String,
) -> Verdict<'p> {
    match fact {
        None if whole_part => Verdict::Drawn,
        None => {
            if pass == Pass::Normal {
                report.fail(format!("`{key}` has no fact under {}", combination()));
            }
            Verdict::Done
        }
        Some(prepared) => match (&prepared.fact.stated, &prepared.value) {
            (Stated::Pending(id), _) => {
                if pass == Pass::Normal {
                    let entry = report
                        .pending
                        .entry(prepared.fact.place())
                        .or_insert_with(|| (id.clone(), key.to_owned(), 0));
                    entry.2 += 1;
                }
                Verdict::Done
            }
            (_, Some(value)) if pass == Pass::Normal || prepared.fact.derivation() => {
                Verdict::Compare(value, &prepared.tolerance)
            }
            (_, None) if pass == Pass::Normal || prepared.fact.derivation() => match composed {
                Some(value) => Verdict::Compare(value, &prepared.tolerance),
                None => Verdict::Done,
            },
            _ => Verdict::Done,
        },
    }
}

/// A set-whole block's value where `fact`, a leaf's, applies (S17): each
/// leaf's applying fact, as `applies` finds it, composed into one value; a
/// leaf with no fact fails as a key with no fact, a pending one composes
/// nothing (it is listed as pending).
fn composed(
    key: &KeyFacts<'_>,
    fact: Option<&Prepared<'_>>,
    applies: &dyn Fn(&str) -> Option<Option<Value>>,
    pass: Pass,
    report: &mut Report,
    combination: &dyn Fn() -> String,
) -> Option<FactValue> {
    let block = key.set_whole.as_ref()?;
    fact?.leaf.as_ref()?;
    let mut values = Vec::new();
    for (leaf, _, _) in &block.leaves {
        match applies(leaf) {
            None => {
                if pass == Pass::Normal {
                    report.fail(format!("`{leaf}` has no fact under {}", combination()));
                }
                return None;
            }
            Some(None) => return None,
            Some(Some(value)) => values.push(value),
        }
    }
    let refs: Vec<&Value> = values.iter().collect();
    match block.compose(&refs) {
        Ok(value) => Some(value),
        Err(Unread::NotHeld(why) | Unread::Refused(why)) => {
            report.fail(format!(
                "`{}` composed from its leaves under {} is refused: {why}",
                block.block,
                combination()
            ));
            None
        }
    }
}

/// The value as stated of the fact `holds` makes apply to the key `name`:
/// `None` where none applies, `Some(None)` where a pending one does.
fn leaf_value(
    keys: &[KeyFacts<'_>],
    name: &str,
    holds: &dyn Fn(&When, &FactKey) -> bool,
) -> Option<Option<Value>> {
    let key = keys.iter().find(|key| key.name == name)?;
    let mut probe = Report::default();
    let fact = applying(
        &key.facts,
        |when| holds(when, &key.key),
        &mut probe,
        &String::new,
    )?;
    Some(fact.leaf.clone())
}

fn record(
    outcome: Result<Outcome, Unread>,
    key: &str,
    place: &str,
    report: &mut Report,
    combination: &dyn Fn() -> String,
    pass: Pass,
) {
    report.compared += 1;
    match outcome {
        Ok(Outcome::Agrees) => {}
        Ok(Outcome::NotDrawn) => report.fail(format!(
            "{pass:?}: `{key}` under {}: {place} applies where its part is not drawn",
            combination()
        )),
        Ok(Outcome::Differs { at, theme, fact }) => report.fail(format!(
            "{pass:?}: `{key}` at `{at}` under {}: {} gives {theme}, {place} gives {fact}",
            combination(),
            match pass {
                Pass::Normal | Pass::Perturbed => "the preset",
                Pass::Measured => "the measured result",
            }
        )),
        Err(Unread::NotHeld(why) | Unread::Refused(why)) => {
            report.fail(format!("{pass:?}: `{key}` under {}: {why}", combination()))
        }
    }
}

/// The perturbed run's stand-in layer (S17): under each scheme, a fixed
/// colour for every declared input and every colour role a derivation fact
/// names as an operand, but a role whose own fact that applies under the
/// conditions `resolved` decided is a derivation: it follows its perturbed
/// operands on both sides, so only the derivation graph's leaves, the
/// preset's own base colours, are perturbed (S17's `read` facts "checked as
/// derivations").
fn stand_in(preset: &Preset, keys: &[KeyFacts<'_>], resolved: &Resolved) -> Result<Layer, String> {
    let mut roles: BTreeSet<String> = BTreeSet::new();
    let mut inputs: BTreeSet<String> = preset.inputs().into_iter().collect();
    for key in keys {
        for prepared in &key.facts {
            if let Stated::Value(value) = &prepared.fact.stated {
                let (named_roles, named_inputs) = preset.operands(value);
                roles.extend(named_roles);
                inputs.extend(
                    named_inputs
                        .into_iter()
                        .filter(|name| preset.inputs().contains(name)),
                );
            }
        }
    }
    roles.retain(|role| {
        let derived = keys
            .iter()
            .filter(|key| key.key.role().as_deref() == Some(role.as_str()))
            .filter_map(|key| {
                let mut probe = Report::default();
                applying(
                    &key.facts,
                    |when| resolved.conditions_hold(when),
                    &mut probe,
                    &String::new,
                )
            })
            .any(|prepared| prepared.fact.derivation());
        !derived
    });
    let mut text = String::new();
    let mut n = 0_usize;
    let mut next = || {
        let color = perturbed(n);
        n += 1;
        color
    };
    for input in &inputs {
        text.push_str(&format!(
            "[inputs.{input}]\nlight = \"{}\"\ndark = \"{}\"\n",
            next(),
            next()
        ));
    }
    for scheme in preset.schemes() {
        let mut by_table: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
        for role in &roles {
            let (table, leaf) = role.rsplit_once('.').unwrap_or(("", role));
            by_table
                .entry(table.to_owned())
                .or_default()
                .push((leaf.to_owned(), next()));
        }
        for (table, leaves) in by_table {
            text.push_str(&format!("[colors.{scheme}.{table}]\n"));
            for (leaf, color) in leaves {
                text.push_str(&format!("{leaf} = \"{color}\"\n"));
            }
        }
    }
    Layer::from_toml(&text).map_err(|error| error.to_string())
}

/// A condition vector's conditions set on the no-reader state, under the
/// flavour; `None` where one cannot be set.
fn conditions(vector: &[String], flavor: Option<&str>) -> Result<Conditions, Unread> {
    let scheme = vector
        .iter()
        .find_map(|condition| condition.strip_prefix("scheme:"))
        .unwrap_or("light");
    let mut out = Conditions::new(scheme).map_err(Unread::Refused)?;
    if let Some(flavor) = flavor {
        out.set(&format!("flavor:{flavor}"))?;
    }
    for condition in vector
        .iter()
        .filter(|condition| !condition.starts_with("scheme:"))
    {
        out.set(condition)?;
    }
    Ok(out)
}

/// A fact's own `when` as combinations (S17: "The gate also resolves each
/// fact's own `when`"): its conditions (each scheme where it names none),
/// its flags, its variants, its first size, density and context, its
/// direction, its `latest` flags each set last, and a position by
/// siblings of its part; `None` where it names what the gate cannot set
/// up (a `previous` or `next` sibling, a size class).
fn own_combination(
    when: &Value,
    widget: &str,
    schemes: &[String],
) -> Option<(Vec<Vec<String>>, Combination)> {
    let table = when.as_table()?;
    for key in table.keys() {
        if ![
            "conditions",
            "not_conditions",
            "states",
            "not_states",
            "variants",
            "not_variants",
            "sizes",
            "not_sizes",
            "densities",
            "not_densities",
            "context",
            "not_context",
            "direction",
            "latest",
            "position",
        ]
        .contains(&key.as_str())
        {
            return None;
        }
    }
    let named = strings(table.get("conditions"));
    let vectors: Vec<Vec<String>> = if named.iter().any(|c| c.starts_with("scheme:")) {
        vec![named]
    } else {
        schemes
            .iter()
            .map(|scheme| {
                std::iter::once(format!("scheme:{scheme}"))
                    .chain(named.iter().cloned())
                    .collect()
            })
            .collect()
    };
    let mut combination = Combination::new(widget);
    combination.states = strings(table.get("states"));
    combination.variants = strings(table.get("variants"));
    combination.size = strings(table.get("sizes")).into_iter().next();
    combination.density = strings(table.get("densities")).into_iter().next();
    combination.direction = table
        .get("direction")
        .and_then(Value::as_str)
        .map(str::to_owned);
    combination.context = table
        .get("context")
        .and_then(Value::as_array)
        .and_then(|list| list.first().cloned());
    if let Some(latest) = table.get("latest").and_then(Value::as_table) {
        let last: Vec<String> = latest
            .values()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect();
        let mut order: Vec<String> = combination
            .states
            .iter()
            .filter(|flag| !last.contains(flag))
            .cloned()
            .collect();
        for flag in &last {
            if !combination.states.contains(flag) {
                combination.states.push(flag.clone());
            }
            order.push(flag.clone());
        }
        combination.order = order;
    }
    if let Some(position) = table.get("position").and_then(Value::as_table) {
        if position.contains_key("previous") || position.contains_key("next") {
            return None;
        }
        if position
            .get("among")
            .is_some_and(|among| among.as_str() != Some("same"))
        {
            return None;
        }
        let flag = |key: &str| position.get(key).and_then(Value::as_bool);
        let even = position.get("parity").and_then(Value::as_str) == Some("even");
        let only = flag("only") == Some(true);
        combination.before = usize::from(flag("first") == Some(false) || even);
        combination.after = usize::from(
            flag("last") == Some(false) || (flag("only") == Some(false) && combination.before == 0),
        );
        if only {
            combination.before = 0;
            combination.after = 0;
        }
    }
    Some((vectors, combination))
}

/// A data combination as the model's.
fn combination_of(point: &Point, widget: &str, context: Option<&Value>) -> Combination {
    let mut combination = Combination::new(widget);
    combination.variants = point.variants.clone();
    combination.states = point.states.clone();
    combination.order = point.order.clone();
    combination.size = point.size.clone();
    combination.density = point.density.clone();
    combination.context = context.cloned();
    combination
}

/// The widget combinations of one preset and flavour, by condition vector.
type Planned = BTreeMap<Vec<String>, Vec<Combination>>;

/// Runs the gate over the facts root `root` for `presets`, each with the
/// platform directory it is checked against (S17 *Generations*).
pub fn run(root: &Path, presets: &[(String, Preset)]) -> Report {
    let started = Instant::now();
    let mut report = Report::default();
    let repo = root
        .parent()
        .and_then(Path::parent)
        .unwrap_or(root)
        .to_path_buf();
    let mut directories: Vec<_> = std::fs::read_dir(root)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path.is_dir())
                .collect()
        })
        .unwrap_or_default();
    directories.sort();
    let bundled: Vec<&str> = presets.iter().map(|(_, preset)| preset.id()).collect();
    for directory in directories {
        let name = directory
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if name == "imported" || name == "staged" || name == "palettes" {
            continue;
        }
        let platform = Platform::read(&directory, &mut report);
        read_all(&platform, &mut report);
        // Every pending fact is listed, with the combinations it applied
        // in, none where it applied in none (S17).
        for fact in &platform.facts {
            if let Stated::Pending(id) = &fact.stated {
                report
                    .pending
                    .entry(fact.place())
                    .or_insert_with(|| (id.clone(), fact.key.clone(), 0));
            }
        }
        report.other_presets += platform
            .facts
            .iter()
            .filter(|fact| {
                strings(fact.when.as_ref().and_then(|when| when.get("conditions")))
                    .iter()
                    .filter_map(|condition| condition.strip_prefix("preset:"))
                    .any(|id| !bundled.contains(&id))
            })
            .count();
        // Every entry's combinations, counted over its own conditions, as
        // the gate will query them once its preset is bundled.
        for entry in &platform.models {
            if !entry.available {
                continue;
            }
            let vectors = model::condition_vectors(
                &entry.axis_conditions,
                &["light".to_owned(), "dark".to_owned()],
            );
            let count = model::combinations(entry, &vectors, None, |_, _| {});
            report.combinations.insert(
                format!("{} {} {:?}", entry.file, entry.widget, entry.conditions),
                count,
            );
        }
        for (directory_of, preset) in presets {
            if *directory_of != platform.name {
                continue;
            }
            let flavors = preset.flavors();
            let flavors: Vec<Option<&str>> = if flavors.is_empty() {
                vec![None]
            } else {
                flavors.iter().map(|flavor| Some(flavor.as_str())).collect()
            };
            let keys = prepare(&platform, preset.id(), &repo, &mut report);
            for flavor in flavors {
                declarations_run(&platform, preset, flavor, &mut report);
                check(&platform, preset, flavor, &keys, &mut report);
                let entries = model::in_force(&platform.models, preset.id(), flavor);
                derivations(preset, flavor, &keys, &entries, &mut report);
            }
        }
    }
    report.elapsed = started.elapsed();
    report
}

/// The declarations run (S17, ADD-A116): each entry in force against the
/// preset's declarations of its widget under the flavour, each declaration
/// the entry leaves out stated as none.
pub fn declarations_run(
    platform: &Platform,
    preset: &Preset,
    flavor: Option<&str>,
    report: &mut Report,
) {
    let entries = model::in_force(&platform.models, preset.id(), flavor);
    for widget in widgets() {
        let available = preset
            .declared(&widget, flavor)
            .is_ok_and(|declared| declared.available);
        if available && !entries.iter().any(|entry| entry.widget == widget) {
            report.fail(format!(
                "declarations: `{widget}` is available in {} under {}, and no state model of {} \
                 states it",
                preset.id(),
                flavor.map_or_else(
                    || "the default flavour".to_owned(),
                    |f| format!("flavor:{f}")
                ),
                platform.name
            ));
        }
    }
    for entry in entries {
        let declared = match preset.declared(&entry.widget, flavor) {
            Ok(declared) => declared,
            Err(Unread::NotHeld(_)) => continue,
            Err(Unread::Refused(why)) => {
                report.fail(format!("{}: {why}", entry.file));
                continue;
            }
        };
        let set = |list: &[String]| list.iter().cloned().collect::<BTreeSet<_>>();
        let sets = |groups: &[Vec<String>]| {
            groups
                .iter()
                .map(|group| set(group))
                .collect::<BTreeSet<_>>()
        };
        // In written order: the query takes the first entry that holds (S10).
        let pairs = |pairs: &[(String, Vec<String>)]| {
            pairs
                .iter()
                .map(|(name, list)| (name.clone(), set(list)))
                .collect::<Vec<_>>()
        };
        let mut differ = |what: &str, preset_side: String, model_side: String| {
            if preset_side != model_side {
                report.fail(format!(
                    "declarations: `{}` {what} under {}: {} declares {preset_side}, {} states {model_side}",
                    entry.widget,
                    flavor.map_or_else(|| "the default flavour".to_owned(), |f| format!("flavor:{f}")),
                    preset.id(),
                    entry.file
                ));
            }
        };
        differ(
            "available",
            format!("{}", declared.available),
            format!("{}", entry.available),
        );
        if !entry.available {
            continue;
        }
        let flags: Vec<String> = entry.flags.clone();
        differ(
            "variants",
            format!("{:?}", set(&declared.variants)),
            format!("{:?}", set(&entry.variants)),
        );
        differ(
            "sizes",
            format!("{:?}", set(&declared.sizes)),
            format!("{:?}", set(&entry.sizes)),
        );
        differ(
            "default_size",
            format!("{:?}", declared.default_size),
            format!("{:?}", entry.default_size),
        );
        differ(
            "default_size_by_variant",
            format!(
                "{:?}",
                declared
                    .default_size_by_variant
                    .iter()
                    .cloned()
                    .collect::<BTreeMap<_, _>>()
            ),
            format!(
                "{:?}",
                entry
                    .default_size_by_variant
                    .iter()
                    .cloned()
                    .collect::<BTreeMap<_, _>>()
            ),
        );
        differ(
            "densities",
            format!("{:?}", set(&declared.densities)),
            format!("{:?}", set(&entry.densities)),
        );
        differ(
            "flags",
            format!("{:?}", set(&declared.flags)),
            format!("{:?}", set(&flags)),
        );
        differ(
            "exclusive",
            format!(
                "{:?}",
                declared.exclusive.iter().cloned().collect::<BTreeSet<_>>()
            ),
            format!(
                "{:?}",
                entry.exclusive.iter().cloned().collect::<BTreeSet<_>>()
            ),
        );
        differ(
            "exclusive_variants",
            format!("{:?}", sets(&declared.exclusive_variants)),
            format!("{:?}", sets(&entry.exclusive_variants)),
        );
        differ(
            "recency",
            format!(
                "{:?}",
                declared.recency.iter().cloned().collect::<BTreeMap<_, _>>()
            ),
            format!(
                "{:?}",
                entry.recency.iter().cloned().collect::<BTreeMap<_, _>>()
            ),
        );
        differ(
            "requires_variants",
            format!("{:?}", set(&declared.requires_variants)),
            format!("{:?}", set(&entry.requires_variants)),
        );
        differ(
            "sizes_by_variant",
            format!("{:?}", pairs(&declared.sizes_by_variant)),
            format!("{:?}", pairs(&entry.sizes_by_variant)),
        );
        differ(
            "sizes_by_context",
            format!("{:?}", pairs(&declared.sizes_by_context)),
            format!("{:?}", pairs(&entry.sizes_by_context)),
        );
    }
}

/// A fact's own combination completed to one the state model `entry`
/// draws (S17: the gate also resolves each fact's own `when`): where the
/// entry requires a variant the `when` names none of, the first required
/// variant its `when` does not exclude and no variant it holds shares an
/// `exclusive_variants` group with; `None` where none is left, as the fact
/// then applies to no element the platform draws.
fn complete(mut combination: Combination, when: &Value, entry: &Model) -> Option<Combination> {
    let required = &entry.requires_variants;
    if required.is_empty() || required.iter().any(|v| combination.variants.contains(v)) {
        return Some(combination);
    }
    let excluded = strings(when.get("not_variants"));
    let variant = required.iter().find(|variant| {
        !excluded.contains(variant)
            && !entry.exclusive_variants.iter().any(|group| {
                group.contains(variant)
                    && combination.variants.iter().any(|held| group.contains(held))
            })
    })?;
    combination.variants.push(variant.clone());
    Some(combination)
}

/// A state model's `not_drawn` entries read as `when`s (S17), for the
/// preset: an entry naming another preset left out; one naming what the
/// model does not hold yet holds on no element the gate queries, which
/// cannot hold the name either; one the model refuses is a failure.
fn not_drawn_whens(entry: &Model, preset: &str, report: &mut Report) -> Vec<When> {
    let mut out = Vec::new();
    for table in &entry.not_drawn {
        let item = Value::Table(table.clone());
        let Some((when, _)) = facts::for_preset(Some(&item), preset) else {
            continue;
        };
        match read_when(when.as_ref()) {
            Ok(when) => out.push(when),
            Err(Unread::NotHeld(_)) => {}
            Err(Unread::Refused(why)) => report.fail(format!(
                "{}: a `not_drawn` entry of `{}` is refused: {why}",
                entry.file, entry.widget
            )),
        }
    }
    out
}

/// The normal and the perturbed run of one preset and flavour, each
/// condition vector resolved for both: the perturbed run's stand-in is
/// decided by the facts that apply under the vector.
fn check(
    platform: &Platform,
    preset: &Preset,
    flavor: Option<&str>,
    keys: &[KeyFacts<'_>],
    report: &mut Report,
) {
    let schemes = preset.schemes();
    let entries = model::in_force(&platform.models, preset.id(), flavor);
    let mut planned: Planned = BTreeMap::new();
    let mut seen: HashSet<(Vec<String>, String)> = HashSet::new();
    let mut plan = |vector: Vec<String>, combination: Combination, planned: &mut Planned| {
        let identity = (vector.clone(), format!("{combination:?}"));
        if seen.insert(identity) {
            planned.entry(vector).or_default().push(combination);
        }
    };
    let mut global_vectors: BTreeSet<Vec<String>> = BTreeSet::new();
    let declared_variants = |widget: &str| {
        preset
            .declared(widget, flavor)
            .map(|d| d.variants)
            .unwrap_or_default()
    };
    let mut not_drawn: BTreeMap<String, Vec<When>> = BTreeMap::new();
    for entry in &entries {
        if !entry.available {
            continue;
        }
        not_drawn.insert(
            entry.widget.clone(),
            not_drawn_whens(entry, preset.id(), report),
        );
        let mut mentioned = Mentioned::of_model(entry);
        for key in keys
            .iter()
            .filter(|key| key.key.widget().as_deref() == Some(entry.widget.as_str()))
        {
            for prepared in &key.facts {
                if let Some(when) = &prepared.fact.when {
                    mentioned.add_when(when);
                }
            }
        }
        let (states, variants) = preset.mentions(&entry.widget);
        mentioned.states.extend(states);
        mentioned.variants.extend(variants);
        if let Ok(declared) = preset.declared(&entry.widget, flavor) {
            mentioned
                .states
                .extend(declared.exclusive.into_iter().flatten());
            mentioned
                .states
                .extend(declared.recency.into_iter().flat_map(|(_, set)| set));
            mentioned.variants.extend(declared.requires_variants);
            mentioned
                .variants
                .extend(declared.exclusive_variants.into_iter().flatten());
            mentioned.variants.extend(
                declared
                    .sizes_by_variant
                    .into_iter()
                    .map(|(variant, _)| variant),
            );
            mentioned.variants.extend(
                declared
                    .default_size_by_variant
                    .into_iter()
                    .map(|(variant, _)| variant),
            );
        }
        let declared = declared_variants(&entry.widget);
        mentioned.variants.extend(
            entry
                .variants
                .iter()
                .filter(|variant| !declared.contains(variant))
                .cloned(),
        );
        let vectors = model::condition_vectors(&entry.axis_conditions, &schemes);
        global_vectors.extend(vectors.iter().cloned());
        model::combinations(entry, &vectors, Some(&mentioned), |point, context| {
            plan(
                point.conditions.clone(),
                combination_of(point, &entry.widget, context),
                &mut planned,
            );
        });
    }
    // Each fact's own `when`.
    for key in keys {
        for prepared in &key.facts {
            let Some(when) = &prepared.fact.when else {
                continue;
            };
            let widget = key.key.widget();
            let Some((vectors, combination)) =
                own_combination(when, widget.as_deref().unwrap_or_default(), &schemes)
            else {
                report.combinations_skipped += 1;
                continue;
            };
            let combination = match entries
                .iter()
                .find(|entry| Some(&entry.widget) == widget.as_ref())
            {
                Some(entry) => complete(combination, when, entry),
                None => Some(combination),
            };
            let Some(combination) = combination else {
                continue;
            };
            for vector in vectors {
                match &widget {
                    Some(_) => plan(vector, combination.clone(), &mut planned),
                    None => {
                        global_vectors.insert(vector);
                    }
                }
            }
        }
    }
    for vector in &global_vectors {
        planned.entry(vector.clone()).or_default();
    }
    let root = root_inputs();
    for (vector, combinations) in &planned {
        let conditions = match conditions(vector, flavor) {
            Ok(conditions) => conditions,
            Err(Unread::NotHeld(_)) => {
                report.combinations_skipped += combinations.len() as u64;
                continue;
            }
            Err(Unread::Refused(why)) => {
                report.fail(format!("the conditions {vector:?} are refused: {why}"));
                continue;
            }
        };
        let resolve = |pass: Pass, layer: &Layer, report: &mut Report| match preset
            .resolve(&conditions, layer)
        {
            Ok(resolved) => Some(resolved),
            Err(why) => {
                report.fail(format!(
                    "{pass:?}: {} does not resolve under {vector:?}: {why}",
                    preset.id()
                ));
                None
            }
        };
        let Some(normal) = resolve(Pass::Normal, &Layer::default(), report) else {
            continue;
        };
        let layer = match stand_in(preset, keys, &normal) {
            Ok(layer) => layer,
            Err(why) => {
                report.fail(format!("Perturbed: the stand-in layer is refused: {why}"));
                continue;
            }
        };
        let perturbed = resolve(Pass::Perturbed, &layer, report);
        let passes = std::iter::once((Pass::Normal, Some(normal)))
            .chain(std::iter::once((Pass::Perturbed, perturbed)));
        for (pass, resolved) in passes {
            let Some(resolved) = resolved else {
                continue;
            };
            if global_vectors.contains(vector) {
                check_globals(&resolved, keys, vector, pass, &root, report);
            }
            let run = Run {
                resolved: &resolved,
                keys,
                vector,
                not_drawn: &not_drawn,
                pass,
                root: &root,
            };
            check_widgets(&run, combinations, report);
        }
    }
}

/// One resolution of a condition vector, for one pass.
struct Run<'a, 'f> {
    resolved: &'a Resolved,
    keys: &'a [KeyFacts<'f>],
    vector: &'a [String],
    /// Each widget's `not_drawn` entries.
    not_drawn: &'a BTreeMap<String, Vec<When>>,
    pass: Pass,
    root: &'a RootInputs,
}

/// Every token, colour-role and input key under one condition vector.
fn check_globals(
    resolved: &Resolved,
    keys: &[KeyFacts<'_>],
    vector: &[String],
    pass: Pass,
    root: &RootInputs,
    report: &mut Report,
) {
    let combination = || format!("{vector:?}");
    for key in keys.iter().filter(|key| key.key.widget().is_none()) {
        let fact = applying(
            &key.facts,
            |when| resolved.conditions_hold(when),
            report,
            &combination,
        );
        let place = fact.map(|fact| fact.fact.place()).unwrap_or_default();
        let holds = |when: &When, _: &FactKey| resolved.conditions_hold(when);
        let whole = composed(
            key,
            fact,
            &|leaf| leaf_value(keys, leaf, &holds),
            pass,
            report,
            &combination,
        );
        if let Verdict::Compare(value, tolerance) = verdict(
            &key.name,
            false,
            fact,
            whole.as_ref(),
            pass,
            report,
            &combination,
        ) {
            let outcome = resolved.compare_global(&key.key, value, root, tolerance);
            record(outcome, &key.name, &place, report, &combination, pass);
        }
    }
}

/// Every widget key over a condition vector's combinations, each distinct
/// signature compared once; a combination a `not_drawn` entry names, as
/// the query matches it, is not queried (S17).
fn check_widgets(run: &Run<'_, '_>, combinations: &[Combination], report: &mut Report) {
    let (resolved, keys, vector, pass) = (run.resolved, run.keys, run.vector, run.pass);
    let metrics = StubMetrics;
    let mut signatures: HashSet<(String, Vec<u32>, Vec<String>)> = HashSet::new();
    for combination in combinations {
        let describe = || format!("{vector:?} {combination:?}");
        let session = match resolved.session(combination, run.root, &metrics) {
            Ok(session) => session,
            Err(Unread::NotHeld(_)) => {
                report.combinations_skipped += 1;
                continue;
            }
            Err(Unread::Refused(why)) => {
                report.fail(format!("{pass:?}: {why} under {}", describe()));
                continue;
            }
        };
        if run
            .not_drawn
            .get(&combination.widget)
            .is_some_and(|whens| whens.iter().any(|when| session.root_holds(when)))
        {
            continue;
        }
        if let Some(why) = session.unavailable() {
            report.fail(format!(
                "{pass:?}: not available ({why}) under {}",
                describe()
            ));
            continue;
        }
        let widget_keys: Vec<&KeyFacts<'_>> = keys
            .iter()
            .filter(|key| key.key.widget().as_deref() == Some(combination.widget.as_str()))
            .collect();
        let mut probe = Report::default();
        let applying_facts: Vec<Option<&Prepared<'_>>> = widget_keys
            .iter()
            .map(|key| {
                applying(
                    &key.facts,
                    |when| session.holds(when, &key.key),
                    &mut probe,
                    &describe,
                )
            })
            .collect();
        for failure in probe.failures {
            report.fail(failure);
        }
        let indices: Vec<String> = applying_facts
            .iter()
            .map(|fact| fact.map(|fact| fact.fact.place()).unwrap_or_default())
            .collect();
        let element = format!(
            "{} {:?} {:?}",
            combination.widget, combination.context, combination.direction
        );
        if !signatures.insert((element, session.signature(), indices)) {
            continue;
        }
        report.queried += 1;
        for (key, fact) in widget_keys.iter().zip(applying_facts) {
            let place = fact.map(|fact| fact.fact.place()).unwrap_or_default();
            if !session.drawn(&key.key) {
                // A field of a part not drawn: no fact is asked for it, and
                // one that applies states a value the platform draws none
                // for (S17).
                if pass == Pass::Normal
                    && let Some(prepared) = fact
                    && matches!(prepared.fact.stated, Stated::Value(_))
                {
                    report.fail(format!(
                        "`{}` has a fact ({place}) under {}, where its part is {}",
                        key.name,
                        describe(),
                        session.answer(&key.key)
                    ));
                }
                continue;
            }
            let holds = |when: &When, key: &FactKey| session.holds(when, key);
            let whole = composed(
                key,
                fact,
                &|leaf| leaf_value(keys, leaf, &holds),
                pass,
                report,
                &describe,
            );
            match verdict(
                &key.name,
                key.key.whole_part(),
                fact,
                whole.as_ref(),
                pass,
                report,
                &describe,
            ) {
                Verdict::Compare(value, tolerance) => {
                    let outcome = session.compare(&key.key, Some(value), tolerance);
                    record(outcome, &key.name, &place, report, &describe, pass);
                }
                Verdict::Drawn if pass == Pass::Normal => {
                    let outcome = session.compare(&key.key, None, &Tolerance::quote());
                    record(
                        outcome,
                        &key.name,
                        "no fact of the whole part (drawn)",
                        report,
                        &describe,
                        pass,
                    );
                }
                Verdict::Drawn | Verdict::Done => {}
            }
        }
    }
}

/// Every measured derivation's passes (S17 *A measured derivation*): for
/// each pass, the preset resolved with the stand-in stating the varied
/// inputs under the fact's `when` and the pass's conditions, the fact's
/// expression evaluated in the key's place compared with the pass's result
/// within the citation's tolerance.
fn derivations(
    preset: &Preset,
    flavor: Option<&str>,
    keys: &[KeyFacts<'_>],
    entries: &[&Model],
    report: &mut Report,
) {
    let root = root_inputs();
    let metrics = StubMetrics;
    let schemes = preset.schemes();
    for key in keys {
        for prepared in &key.facts {
            let Some(value) = &prepared.value else {
                continue;
            };
            for (pass, result) in &prepared.passes {
                let describe =
                    || format!("the pass {:?} of {}", pass.inputs, prepared.fact.place());
                let empty = Value::Table(toml::Table::new());
                let when = facts::for_preset(prepared.fact.when.as_ref(), preset.id())
                    .and_then(|(when, _)| when)
                    .unwrap_or(empty);
                let widget = key.key.widget();
                let Some((vectors, combination)) =
                    own_combination(&when, widget.as_deref().unwrap_or_default(), &schemes)
                else {
                    report.combinations_skipped += 1;
                    continue;
                };
                let entry = entries
                    .iter()
                    .find(|entry| Some(&entry.widget) == widget.as_ref());
                let combination = match entry {
                    Some(entry) => complete(combination, &when, entry),
                    None => Some(combination),
                };
                let Some(combination) = combination else {
                    report.fail(format!(
                        "{}: its `when` names no combination the state model draws",
                        describe()
                    ));
                    continue;
                };
                let not_drawn = entry
                    .map(|entry| not_drawn_whens(entry, preset.id(), report))
                    .unwrap_or_default();
                let mut text = String::new();
                for (input, color) in &pass.inputs {
                    text.push_str(&format!(
                        "[inputs.{input}]\nlight = {color}\ndark = {color}\n"
                    ));
                }
                let layer = match Layer::from_toml(&text) {
                    Ok(layer) => layer,
                    Err(error) => {
                        report.fail(format!("{}: its inputs do not read: {error}", describe()));
                        continue;
                    }
                };
                let fixed = pass.conditions.iter().any(|c| c.starts_with("scheme:"));
                for vector in vectors {
                    let vector: Vec<String> = if fixed {
                        vector
                            .into_iter()
                            .filter(|c| !c.starts_with("scheme:"))
                            .chain(pass.conditions.iter().cloned())
                            .collect()
                    } else {
                        vector
                            .into_iter()
                            .chain(pass.conditions.iter().cloned())
                            .collect()
                    };
                    let resolved = match conditions(&vector, flavor)
                        .map_err(|unread| format!("{unread:?}"))
                        .and_then(|conditions| preset.resolve(&conditions, &layer))
                    {
                        Ok(resolved) => resolved,
                        Err(why) => {
                            report.fail(format!(
                                "{} does not resolve under {vector:?}: {why}",
                                describe()
                            ));
                            continue;
                        }
                    };
                    let outcome = match &widget {
                        None => resolved.compare_global_pass(
                            &key.key,
                            value,
                            result,
                            &root,
                            &pass.tolerance,
                        ),
                        Some(_) => match resolved.session(&combination, &root, &metrics) {
                            Ok(session) => {
                                let drawn = session.unavailable().map_or_else(
                                    || {
                                        (!not_drawn.iter().any(|when| session.root_holds(when)))
                                            .then_some(())
                                            .ok_or_else(|| "named by `not_drawn`".to_owned())
                                    },
                                    |why| Err(format!("not available ({why})")),
                                );
                                match drawn {
                                    Ok(()) => session.compare_pass(
                                        &key.key,
                                        value,
                                        result,
                                        &pass.tolerance,
                                    ),
                                    Err(why) => Err(Unread::Refused(format!(
                                        "the pass's combination {combination:?} is {why}"
                                    ))),
                                }
                            }
                            Err(Unread::NotHeld(_)) => {
                                report.combinations_skipped += 1;
                                continue;
                            }
                            Err(Unread::Refused(why)) => Err(Unread::Refused(why)),
                        },
                    };
                    let under = format!("{vector:?}");
                    record(
                        outcome,
                        &key.name,
                        &describe(),
                        report,
                        &|| under.clone(),
                        Pass::Measured,
                    );
                }
            }
        }
    }
}
