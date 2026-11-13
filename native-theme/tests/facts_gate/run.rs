//! The gate's run (S17 *The gate*): every preset of a platform directory
//! resolved with a stand-in and no reader, each widget's combinations
//! queried and each key compared with the fact that applies; the perturbed
//! run; the declarations run; and the report.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use native_theme::gate::{
    Combination, Conditions, Drawing, FactKey, FactValue, Outcome, Preset, Query, Resolved,
    SetWhole, Tolerance, Unread, When, context_held, is_listed_default, listed_key, names_read,
    parse_key, parse_value, parse_when, parts, set_whole, widgets,
};
use native_theme::{FontMetrics, Layer, Px, ResolvedFont, RootInputs, Size};
use toml::Value;

use super::depth::{Depth, Place};
use super::facts::{self, Fact, Stated};
use super::model::{self, Block, Mentioned, Model, Point, Space, strings};
use super::timing::{Step, timed};
use super::units::Units;

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
        native_theme::Direction::Ltr,
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
    /// The accent run (S17 *The accent run*): `kde-breeze` over a scheme
    /// file, with an accent set or none, against the facts measured so.
    Accent,
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
    /// Each fact skipped as its evaluation reaches a synthetic value, the
    /// synthetic route's stand-in for a listed path (S17, ADD-A128): its
    /// place, its key, and how many combinations.
    pub synthetic: BTreeMap<String, (String, u64)>,
    /// Each listed path found: a pending fact applying where the preset
    /// leaves the path to the synthetic value, by `<preset> <key>`: the
    /// pending facts' register ids (a set-whole block's, its pending
    /// leaves') and how many combinations; and each pending default size
    /// listed, by `<preset> <declaration path>`, with its listed ids, once.
    pub listed: BTreeMap<String, (BTreeSet<String>, u64)>,
    /// Each set-whole leaf's fact left unchecked as a sibling leaf's
    /// applying fact is pending (ADD-A128): its place, its key, the pending
    /// fact, and how many combinations.
    pub unchecked: BTreeMap<String, (String, String, u64)>,
    /// The values the model's reader refuses, listed: a failure only for a
    /// preset checked against them.
    pub unread: Vec<String>,
    /// Facts whose key the model does not hold yet, by key.
    pub keys_not_held: BTreeMap<String, usize>,
    /// Facts a unit they belong to, their key's or a widget's their `when`
    /// names, is not enabled for (S17 *While 0.8.0 is built*), by key.
    pub units_not_enabled: BTreeMap<String, usize>,
    /// Facts a preset's run skips by S17's predicate (ADD-A147) that a unit
    /// they wait for places (`depth::Place::Later`), by `<preset>\t<the
    /// unit>`.
    pub later_units: BTreeMap<String, usize>,
    /// Facts a preset's run skips by S17's predicate that no unit's names
    /// place (`depth::Place::Unplaced`), by preset.
    pub unplaced: BTreeMap<String, usize>,
    /// Per preset, the facts of the enabled units its run prepares and how
    /// many of them are pending (S17's predicate, ADD-A147).
    pub at_depth: BTreeMap<String, (usize, usize)>,
    /// Facts whose `when` names what the model does not hold yet, by what
    /// it names.
    pub whens_not_held: BTreeMap<String, usize>,
    /// Facts naming a preset no bundled preset is (decision 7).
    pub other_presets: usize,
    /// State-model variants only facts the gate skips name, each left out
    /// of its entry's declarations and combinations (ADD-A128), by
    /// `<file> <widget> <variant>`.
    pub variants_not_held: BTreeSet<String>,
    /// The accent run's facts compared (S17 *The accent run*), each once
    /// however many of its resolutions compare it.
    pub accent_run: usize,
    /// Field facts that apply where their part is not drawn, so compare
    /// nothing: the part's own key is compared there (S17, ADD-A130).
    pub fields_not_drawn: u64,
    /// Facts whose own `when` names no combination the state model draws
    /// (its required variants all excluded), so it applies to no element
    /// the platform draws; a measured derivation's such pass fails.
    pub whens_not_drawn: u64,
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
    /// The failures by run, fact and key, in the order each group's first
    /// failure was recorded: its run, its key, its fact, how many
    /// failures and its first; a failure of no comparison is its own group.
    pub groups: Vec<Group>,
    /// Each failure's group, by index into `groups`, in order.
    of_group: Vec<usize>,
    /// The groups by run, key and fact.
    group_index: HashMap<(String, String, String), usize>,
    /// The failures recorded, each once: their places in `failures` by a
    /// hash of their text, so each text is held once ([`text_hash`]).
    seen: HashMap<u64, Vec<usize>>,
}

/// A hash of a failure's text, by which [`Report`] finds whether it holds
/// the failure already.
fn text_hash(text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

/// The failures of one run, fact and key (S17's report, grouped).
#[derive(Debug)]
pub struct Group {
    pub run: String,
    pub key: String,
    pub fact: String,
    pub count: u64,
    pub first: String,
}

impl Report {
    /// A failure of no comparison, its own group.
    pub fn fail(&mut self, failure: String) {
        let group = (String::new(), String::new(), failure.clone());
        self.fail_in(group, failure);
    }

    /// A failure of the run, key and fact `group` (its run, key and fact).
    fn fail_in(&mut self, group: (String, String, String), failure: String) {
        let hash = text_hash(&failure);
        let held = self.seen.get(&hash).is_some_and(|places| {
            places
                .iter()
                .any(|at| self.failures.get(*at) == Some(&failure))
        });
        if held {
            return;
        }
        self.seen.entry(hash).or_default().push(self.failures.len());
        let index = match self.group_index.get(&group) {
            Some(index) => *index,
            None => {
                let index = self.groups.len();
                let (run, key, fact) = group.clone();
                self.groups.push(Group {
                    run,
                    key,
                    fact,
                    count: 0,
                    first: failure.clone(),
                });
                self.group_index.insert(group, index);
                index
            }
        };
        if let Some(group) = self.groups.get_mut(index) {
            group.count += 1;
        }
        self.of_group.push(index);
        self.failures.push(failure);
    }

    /// `other`'s findings added after this report's, as one run recording
    /// both in turn records them.
    pub fn merge(&mut self, mut other: Report) {
        let failures = std::mem::take(&mut other.failures);
        for (failure, group) in failures.into_iter().zip(&other.of_group) {
            match other.groups.get(*group) {
                Some(group) => {
                    let group = (group.run.clone(), group.key.clone(), group.fact.clone());
                    self.fail_in(group, failure);
                }
                None => self.fail(failure),
            }
        }
        for (place, (id, key, count)) in other.pending {
            self.pending.entry(place).or_insert((id, key, 0)).2 += count;
        }
        for (place, (key, count)) in other.synthetic {
            self.synthetic.entry(place).or_insert((key, 0)).1 += count;
        }
        for (key, (ids, count)) in other.listed {
            let entry = self.listed.entry(key).or_default();
            entry.0.extend(ids);
            entry.1 += count;
        }
        for (place, (key, pending, count)) in other.unchecked {
            self.unchecked.entry(place).or_insert((key, pending, 0)).2 += count;
        }
        self.unread.extend(other.unread);
        for (key, count) in other.keys_not_held {
            *self.keys_not_held.entry(key).or_default() += count;
        }
        for (key, count) in other.units_not_enabled {
            *self.units_not_enabled.entry(key).or_default() += count;
        }
        for (key, count) in other.later_units {
            *self.later_units.entry(key).or_default() += count;
        }
        for (preset, count) in other.unplaced {
            *self.unplaced.entry(preset).or_default() += count;
        }
        for (preset, counts) in other.at_depth {
            self.at_depth.entry(preset).or_insert(counts);
        }
        for (why, count) in other.whens_not_held {
            *self.whens_not_held.entry(why).or_default() += count;
        }
        self.other_presets += other.other_presets;
        self.variants_not_held.extend(other.variants_not_held);
        self.accent_run += other.accent_run;
        self.fields_not_drawn += other.fields_not_drawn;
        self.whens_not_drawn += other.whens_not_drawn;
        self.combinations_skipped += other.combinations_skipped;
        for (entry, count) in other.combinations {
            *self.combinations.entry(entry).or_default() += count;
        }
        self.queried += other.queried;
        self.compared += other.compared;
    }

    /// Its findings and counts, all but the time, in an order of their own:
    /// the failures sorted, and each group by its run, key and fact with its
    /// count; two runs that found the same, in any order, give the same.
    pub fn findings(&self) -> String {
        let mut failures = self.failures.clone();
        failures.sort();
        let groups: BTreeMap<(&str, &str, &str), u64> = self
            .groups
            .iter()
            .map(|group| {
                (
                    (group.run.as_str(), group.key.as_str(), group.fact.as_str()),
                    group.count,
                )
            })
            .collect();
        let mut unread = self.unread.clone();
        unread.sort();
        format!(
            "failures {failures:?}\ngroups {groups:?}\npending {:?}\nsynthetic {:?}\n\
             listed {:?}\nunchecked {:?}\nunread {unread:?}\nkeys not held {:?}\n\
             units not enabled {:?}\nlater units {:?}\nunplaced {:?}\nat depth {:?}\nwhens not held {:?}\n\
             other presets {}\n\
             variants not held {:?}\naccent run {}\nfields not drawn {}\n\
             whens not drawn {}\ncombinations skipped {}\ncombinations {:?}\nqueried {}\n\
             compared {}",
            self.pending,
            self.synthetic,
            self.listed,
            self.unchecked,
            self.keys_not_held,
            self.units_not_enabled,
            self.later_units,
            self.unplaced,
            self.at_depth,
            self.whens_not_held,
            self.other_presets,
            self.variants_not_held,
            self.accent_run,
            self.fields_not_drawn,
            self.whens_not_drawn,
            self.combinations_skipped,
            self.combinations,
            self.queried,
            self.compared,
        )
    }
}

/// A platform directory's facts and state models, read.
pub struct Platform {
    pub name: String,
    pub facts: Vec<Fact>,
    pub keys: Vec<(String, Result<FactKey, Unread>)>,
    pub models: Vec<Model>,
    /// The units it was read with, whose names at depth each preset's run
    /// reads (S17, ADD-A147).
    pub units: Units,
}

impl Platform {
    /// The directory `directory` of the facts root, read; a file the gate
    /// cannot read is a failure.
    pub fn read(directory: &Path, units: &Units, report: &mut Report) -> Platform {
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
        for fact in &mut facts {
            fact.enabled = units_enabled(units, fact);
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
            units: units.clone(),
        }
    }

    fn key(&self, key: &str) -> Option<&Result<FactKey, Unread>> {
        self.keys
            .iter()
            .find(|(held, _)| held == key)
            .map(|(_, parsed)| parsed)
    }
}

/// Whether every unit `fact` belongs to is enabled (S17 *While 0.8.0 is
/// built*): its key's, and each widget's its `when`'s positive lists name
/// ([`when_widgets`]), as check_facts_trace.py's `in_unit` places them. An
/// S9 input's key (`input.…`) belongs to no unit: the inputs are the
/// model's core (the plan's M1a: S9.1, S9.2, S9.4), whatever unit is
/// enabled.
fn units_enabled(units: &Units, fact: &Fact) -> bool {
    (fact.key.starts_with("input.") || units.hold(&fact.key))
        && fact
            .when
            .as_ref()
            .is_none_or(|when| when_widgets(when).iter().all(|widget| units.hold(widget)))
}

/// The entry's state model in force on `preset` (S10's *Later names*, S17,
/// ADD-A147): its variants, flags and axis conditions at depth (`units`),
/// each exclusive group and recency set trimmed to the flags at depth (one
/// left with fewer than two dropped), a set holding a variant beyond depth
/// no element's ([`Model::without`]); and, while the sizes unit is not
/// enabled, the default size in force alone (the widget's, and each
/// `sizes_by_variant` entry's, else the widget's where the entry takes it)
/// and no density. Its declarations stay whole in the preset.
pub fn in_force_at_depth(entry: &Model, units: &Units, preset: &str) -> Model {
    let widget = entry.widget.as_str();
    let mut off: Vec<String> = entry.skipped_variants.clone();
    off.extend(
        entry
            .variants
            .iter()
            .filter(|variant| !units.at_depth(widget, preset, variant, true))
            .cloned(),
    );
    let mut out = entry.without(&off);
    let flag = |flag: &String| units.at_depth(widget, preset, flag, false);
    out.states.retain(flag);
    out.flags.retain(flag);
    out.exclusive = out
        .exclusive
        .iter()
        .map(|group| {
            group
                .iter()
                .filter(|held| flag(held))
                .cloned()
                .collect::<Vec<_>>()
        })
        .filter(|group| group.len() >= 2)
        .collect();
    out.recency = out
        .recency
        .iter()
        .map(|(name, set)| {
            let set: Vec<String> = set.iter().filter(|held| flag(held)).cloned().collect();
            (name.clone(), set)
        })
        .filter(|(_, set)| set.len() >= 2)
        .collect();
    out.axis_conditions
        .retain(|condition| units.condition_at_depth(condition, widget));
    if !units.sizes_enabled() {
        if let Some(default) = entry
            .default_size
            .as_ref()
            .filter(|d| entry.sizes.contains(d))
        {
            out.sizes = vec![default.clone()];
        }
        out.sizes_by_variant = entry
            .sizes_by_variant
            .iter()
            .filter(|(variant, _, _)| !off.contains(variant))
            .map(|(variant, sizes, default)| {
                let in_force = default
                    .as_ref()
                    .or(entry.default_size.as_ref())
                    .filter(|default| sizes.contains(default));
                let sizes = in_force.map_or_else(|| sizes.clone(), |size| vec![size.clone()]);
                (variant.clone(), sizes, default.clone())
            })
            .collect();
        out.densities.clear();
    }
    out
}

/// One fact prepared for a preset: its `when` read with the preset's
/// conditions decided, its value read for its key, its tolerance.
pub struct Prepared<'f> {
    fact: &'f Fact,
    /// Its `when` as written, its `preset:` conditions decided and left
    /// out ([`facts::for_preset`]).
    written: Option<Value>,
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
pub struct KeyFacts<'f> {
    name: String,
    key: FactKey,
    /// The widget whose element the key is queried with
    /// ([`FactKey::widget`]), held as every combination asks for it.
    widget: Option<String>,
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
        // The accent run reads its own facts ([`accent_run`]).
        if fact.accent_run {
            continue;
        }
        if !fact.enabled {
            *report
                .units_not_enabled
                .entry(fact.key.clone())
                .or_default() += 1;
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

/// Counts where a fact of `preset` (its `when` as written for it) stands
/// ([`Depth::unit`], S17, ADD-A147): under the unit that places it, apart
/// where none does, and, of the enabled units, in `counts` (its facts, its
/// pending facts); whether it is of the enabled units.
fn count_place(
    depth: &Depth,
    fact: &Fact,
    written: Option<&Value>,
    preset: &str,
    counts: &mut (usize, usize),
    report: &mut Report,
) -> bool {
    match depth.unit(&fact.key, written) {
        Place::Enabled => {
            counts.0 = counts.0.saturating_add(1);
            if matches!(fact.stated, Stated::Pending(_)) {
                counts.1 = counts.1.saturating_add(1);
            }
            true
        }
        Place::Later(unit) => {
            *report
                .later_units
                .entry(format!("{preset}\t{unit}"))
                .or_default() += 1;
            false
        }
        Place::Unplaced => {
            *report.unplaced.entry(preset.to_owned()).or_default() += 1;
            false
        }
    }
}

/// Where every fact of `platform` that can apply to `preset` stands, counted
/// in the report as [`prepare`] counts it, with no preset loaded
/// (`facts_units`).
pub fn place_facts(platform: &Platform, preset: &str, report: &mut Report) {
    let depth = Depth::new(platform, preset);
    let mut counts = (0_usize, 0_usize);
    for fact in &platform.facts {
        if fact.accent_run {
            continue;
        }
        if let Some((written, _)) = facts::for_preset(fact.when.as_ref(), preset) {
            count_place(&depth, fact, written.as_ref(), preset, &mut counts, report);
        }
    }
    report.at_depth.insert(preset.to_owned(), counts);
}

/// The facts of `platform` that can apply to `preset`, by key, each read;
/// what [`read_all`] counted or failed left out.
pub fn prepare<'f>(
    platform: &'f Platform,
    preset: &str,
    evidence: &facts::Evidence,
    report: &mut Report,
) -> Vec<KeyFacts<'f>> {
    let mut out: Vec<KeyFacts<'f>> = Vec::new();
    let depth = Depth::new(platform, preset);
    let mut counts = (0_usize, 0_usize);
    for fact in &platform.facts {
        if fact.accent_run {
            continue;
        }
        let Some((written, _)) = facts::for_preset(fact.when.as_ref(), preset) else {
            continue;
        };
        if !count_place(&depth, fact, written.as_ref(), preset, &mut counts, report) {
            continue;
        }
        let Some(Ok(key)) = platform.key(&fact.key) else {
            continue;
        };
        let Ok(when) = read_when(written.as_ref()) else {
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
        match facts::derivation_passes(fact, evidence, preset) {
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
            written,
            when,
            value,
            leaf,
            tolerance: facts::tolerance(fact, evidence, preset),
            passes,
        };
        match out.iter_mut().find(|held| held.name == fact.key) {
            Some(held) => held.facts.push(prepared),
            None => out.push(KeyFacts {
                name: fact.key.clone(),
                key: key.clone(),
                widget: key.widget(),
                set_whole: block,
                facts: vec![prepared],
            }),
        }
    }
    report.at_depth.insert(preset.to_owned(), counts);
    // A part's own key is compared under every combination, drawn where no
    // part fact applies (S17, ADD-A130), so a part with no fact has its key
    // too.
    for widget in widgets() {
        for part in parts(&widget) {
            let name = format!("{widget}.{part}");
            if out.iter().any(|held| held.name == name) {
                continue;
            }
            match parse_key(&name) {
                Ok(key) => out.push(KeyFacts {
                    name,
                    widget: key.widget(),
                    key,
                    set_whole: None,
                    facts: Vec::new(),
                }),
                Err(unread) => {
                    report.fail(format!(
                        "the part's own key `{name}` is refused: {unread:?}"
                    ));
                }
            }
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
    let held: Vec<bool> = facts.iter().map(|fact| holds(&fact.when)).collect();
    applying_of(facts, &held, report, combination)
}

/// [`applying`], whether each fact's `when` holds given in `held`.
fn applying_of<'p, 'f>(
    facts: &'p [Prepared<'f>],
    held: &[bool],
    report: &mut Report,
    combination: &dyn Fn() -> String,
) -> Option<&'p Prepared<'f>> {
    applying_at(facts, held, report, combination).and_then(|at| facts.get(at))
}

/// [`applying_of`], the fact that applies by its place among `facts`.
fn applying_at(
    facts: &[Prepared<'_>],
    held: &[bool],
    report: &mut Report,
    combination: &dyn Fn() -> String,
) -> Option<usize> {
    let holding = || {
        facts
            .iter()
            .zip(held)
            .enumerate()
            .filter(|(_, (_, holds))| **holds)
            .map(|(at, (fact, _))| (at, fact))
    };
    let (at, _) = holding().next_back()?;
    // A pending fact a later stated fact shadows fails; a later pending
    // fact is exempt, the combination pending under it (S17 *Which fact
    // applies*, ADD-A135), as `check_facts.py`'s `pending_problems` reads it.
    let Some((stated_at, stated)) = holding()
        .rev()
        .find(|(_, fact)| !matches!(fact.fact.stated, Stated::Pending(_)))
    else {
        return Some(at);
    };
    for (_, earlier) in holding().take_while(|(earlier_at, _)| *earlier_at < stated_at) {
        if let Stated::Pending(id) = &earlier.fact.stated {
            report.fail(format!(
                "{}: the pending fact {id} of `{}` is shadowed by {} under {}",
                earlier.fact.place(),
                earlier.fact.key,
                stated.fact.place(),
                combination()
            ));
        }
    }
    Some(at)
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
                report.fail_in(
                    ("Normal".to_owned(), key.to_owned(), String::new()),
                    format!("`{key}` has no fact under {}", combination()),
                );
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

/// A set-whole block's leaf as [`leaf_value`] finds it under a
/// combination.
enum LeafFact {
    /// The leaf's key has no fact at all, which no verdict of its own
    /// reports.
    NoKey,
    /// Its key has facts, none of which applies: its own verdict reports
    /// it.
    NoneApplies,
    /// A pending fact applies: its register id and place.
    Pending(String, String),
    /// A fact applies, its value as stated.
    Value(Value),
}

/// A set-whole block's value where `fact`, a leaf's, applies (S17): each
/// leaf's applying fact, as `applies` finds it, composed into one value; a
/// leaf with no fact fails as a key with no fact (reported once, by the
/// leaf's own verdict where its key has facts), and a pending one makes
/// the field pending there: nothing is composed, and `fact` is listed
/// unchecked beside the pending fact (ADD-A128).
fn composed(
    key: &KeyFacts<'_>,
    fact: Option<&Prepared<'_>>,
    applies: &dyn Fn(&str) -> LeafFact,
    pass: Pass,
    report: &mut Report,
    combination: &dyn Fn() -> String,
) -> Option<FactValue> {
    let block = key.set_whole.as_ref()?;
    let prepared = fact?;
    prepared.leaf.as_ref()?;
    let mut values = Vec::new();
    for (leaf, _, _) in &block.leaves {
        match applies(leaf) {
            LeafFact::NoKey => {
                if pass == Pass::Normal {
                    report.fail_in(
                        ("Normal".to_owned(), leaf.clone(), String::new()),
                        format!("`{leaf}` has no fact under {}", combination()),
                    );
                }
                return None;
            }
            LeafFact::NoneApplies => return None,
            LeafFact::Pending(id, place) => {
                if pass == Pass::Normal {
                    let entry = report
                        .unchecked
                        .entry(prepared.fact.place())
                        .or_insert_with(|| (key.name.clone(), format!("{id} at {place}"), 0));
                    entry.2 += 1;
                }
                return None;
            }
            LeafFact::Value(value) => values.push(value),
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

/// The fact `holds` makes apply to the key `name`, as a set-whole block's
/// leaf.
fn leaf_value(
    keys: &[KeyFacts<'_>],
    name: &str,
    holds: &dyn Fn(&When, &FactKey) -> bool,
) -> LeafFact {
    let Some(key) = keys.iter().find(|key| key.name == name) else {
        return LeafFact::NoKey;
    };
    let held: Vec<bool> = key
        .facts
        .iter()
        .map(|fact| holds(&fact.when, &key.key))
        .collect();
    leaf_of(key, &held)
}

/// The leaf `key`'s fact as [`leaf_value`] finds it, whether each of its
/// facts' `when` holds given in `held`.
fn leaf_of(key: &KeyFacts<'_>, held: &[bool]) -> LeafFact {
    let mut probe = Report::default();
    let fact = applying_of(&key.facts, held, &mut probe, &String::new);
    match fact {
        None => LeafFact::NoneApplies,
        Some(prepared) => match (&prepared.fact.stated, &prepared.leaf) {
            (Stated::Pending(id), _) => LeafFact::Pending(id.clone(), prepared.fact.place()),
            (Stated::Value(_), Some(value)) => LeafFact::Value(value.clone()),
            (Stated::Value(_), None) => LeafFact::NoneApplies,
        },
    }
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
    let group = (format!("{pass:?}"), key.to_owned(), place.to_owned());
    match outcome {
        Ok(Outcome::Agrees) => {}
        Ok(Outcome::NotDrawn) => report.fail_in(
            group,
            format!(
                "{pass:?}: `{key}` under {}: {place} applies where its part is not drawn",
                combination()
            ),
        ),
        Ok(Outcome::Differs { at, theme, fact }) => report.fail_in(
            group,
            format!(
                "{pass:?}: `{key}` at `{at}` under {}: {} gives {theme}, {place} gives {fact}",
                combination(),
                match pass {
                    Pass::Normal | Pass::Perturbed | Pass::Accent => "the preset",
                    Pass::Measured => "the measured result",
                }
            ),
        ),
        Err(Unread::NotHeld(why) | Unread::Refused(why)) => report.fail_in(
            group,
            format!("{pass:?}: `{key}` under {}: {why}", combination()),
        ),
    }
}

/// The widgets a `when`'s positive lists name as an ancestor or a sibling
/// (S11), the units a fact belongs to beside its key's (S17, ADD-A128):
/// its contexts' (`scope.nearest`'s too) and its position's. A negative
/// list never adds a unit: where its widget's unit is not enabled no such
/// widget exists, so the negation holds ([`read_when`]).
fn when_widgets(when: &Value) -> Vec<String> {
    let mut out = Vec::new();
    let widget = |entry: &Value| {
        entry
            .get("widget")
            .and_then(Value::as_str)
            .map(str::to_owned)
    };
    for entry in when
        .get("context")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        out.extend(widget(entry));
        out.extend(strings(
            entry.get("scope").and_then(|scope| scope.get("nearest")),
        ));
    }
    if let Some(position) = when.get("position") {
        out.extend(strings(position.get("among")));
        for side in ["previous", "next"] {
            out.extend(position.get(side).and_then(widget));
        }
    }
    out
}

/// The variants of `entry` the model does not hold: its typed variants with
/// no member of `Variant`, which a later unit of depth holds (S6.1, S10
/// *Later names*, ADD-A147). They have no spelling, so the declarations
/// leave them out and no combination holds one; every other variant stays
/// in the declarations whole, the state model in force at depth
/// ([`in_force_at_depth`]) deciding which the combinations draw. Each
/// recorded in the report.
fn unspelled_variants(entry: &Model, report: &mut Report) -> Vec<String> {
    let unspelled = |variant: &String| {
        let mut probe = toml::Table::new();
        probe.insert(
            "variants".to_owned(),
            Value::Array(vec![Value::String(variant.clone())]),
        );
        matches!(parse_when(&Value::Table(probe)), Err(Unread::NotHeld(_)))
    };
    let out: Vec<String> = entry
        .variants
        .iter()
        .filter(|variant| unspelled(variant))
        .cloned()
        .collect();
    for variant in &out {
        report
            .variants_not_held
            .insert(format!("{} {} {variant}", entry.file, entry.widget));
    }
    out
}

/// The entries in force for `preset` and `flavor`, each without the
/// variants the model does not hold ([`unspelled_variants`]) and the typed
/// variants at depth no combination at depth draws (the preset's `later`,
/// S10 *Later names*: no element holds one), and each its state model in
/// force at depth ([`in_force_at_depth`]): the combinations the gate draws
/// hold only names at depth (S17, ADD-A147).
fn projected(
    platform: &Platform,
    preset: &Preset,
    flavor: Option<&str>,
    report: &mut Report,
) -> Vec<Model> {
    let depth = Depth::new(platform, preset.id());
    model::in_force(&platform.models, preset.id(), flavor)
        .into_iter()
        .map(|entry| {
            let mut off = unspelled_variants(entry, report);
            let drawn = depth.drawn_variants(&entry.widget);
            let nowhere: Vec<String> = entry
                .variants
                .iter()
                .filter(|variant| {
                    !variant.starts_with('@') && !drawn.contains(*variant) && !off.contains(variant)
                })
                .cloned()
                .collect();
            off.extend(nowhere);
            in_force_at_depth(&entry.without(&off), &platform.units, preset.id())
        })
        .collect()
}

/// The entries in force for `preset` and `flavor` as its declarations state
/// them (S10 *Later names*, S17, ADD-A147), as scripts/generate_preset.py's
/// `declared()` writes them: whole, but for the typed variants the model
/// does not hold ([`unspelled_variants`]: they have no spelling); their
/// exclusive groups trimmed to the flags at depth (a group left with fewer
/// than two dropped); their recency sets trimmed to the flags at depth, a
/// set left with fewer than two, or that no rule's `latest` on the widget
/// names, dropped; and `flags` to the flags at depth a rule of the preset
/// on the widget names ([`Preset::mentions`]).
fn declared_entries(
    platform: &Platform,
    preset: &Preset,
    flavor: Option<&str>,
    report: &mut Report,
) -> Vec<Model> {
    let units = &platform.units;
    model::in_force(&platform.models, preset.id(), flavor)
        .into_iter()
        .map(|entry| {
            let unspelled = unspelled_variants(entry, report);
            let mut out = entry.without(&unspelled);
            let widget = out.widget.clone();
            let (named, _) = preset.mentions(&widget);
            let latest = preset.latest_sets(&widget);
            let flag = |flag: &String| units.at_depth(&widget, preset.id(), flag, false);
            out.flags.retain(|held| flag(held) && named.contains(held));
            out.exclusive = out
                .exclusive
                .iter()
                .map(|group| {
                    group
                        .iter()
                        .filter(|held| flag(held))
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .filter(|group| group.len() >= 2)
                .collect();
            out.recency = out
                .recency
                .iter()
                .map(|(name, set)| {
                    let set: Vec<String> = set.iter().filter(|held| flag(held)).cloned().collect();
                    (name.clone(), set)
                })
                .filter(|(name, set)| set.len() >= 2 && latest.contains(name))
                .collect();
            out
        })
        .collect()
}

/// The variants a preset's declarations list under `later` for an entry
/// whose typed variants the model holds (S10 *Later names*, ADD-A147), as
/// scripts/generate_preset.py's `later_variants()` gives them: each a later
/// unit not enabled holds for the entry on the preset (a keyed variant
/// beyond depth, a typed variant the model holds through another entry's
/// Depth line), and each typed variant at depth that no combination at
/// depth draws (`drawn`, [`Depth::drawn_variants`]), sorted.
fn later_variants(
    entry: &Model,
    units: &Units,
    preset: &str,
    drawn: &BTreeSet<String>,
) -> Vec<String> {
    let mut out: Vec<String> = entry
        .variants
        .iter()
        .filter(|variant| {
            !units.at_depth(&entry.widget, preset, variant, true)
                || (!variant.starts_with('@') && !drawn.contains(*variant))
        })
        .cloned()
        .collect();
    out.sort();
    out.dedup();
    out
}

/// The perturbed run's stand-in layer (S17): under each scheme, a fixed
/// colour for every declared input and every colour role a derivation fact
/// names as an operand, but an input or role whose own fact that applies
/// under the conditions `resolved` decided is a derivation: it follows its
/// perturbed operands on both sides, so only the derivation graph's leaves,
/// the preset's own base colours, are perturbed (S17's `read` facts
/// "checked as derivations"); an input with no fact yet is stated. On
/// `kde-breeze` it states no `accent`, so no accent-derivation layer is
/// derived and the preset's no-accent derivations are compared, `kde_accent`
/// being the accent run's (S9.2, S17, ADD-A132).
pub fn stand_in(
    preset: &Preset,
    keys: &[KeyFacts<'_>],
    resolved: &Resolved,
) -> Result<Layer, String> {
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
    let derived = |named: &dyn Fn(&KeyFacts<'_>) -> bool| {
        keys.iter()
            .filter(|key| named(key))
            .filter_map(|key| {
                let mut probe = Report::default();
                applying(
                    &key.facts,
                    |when| resolved.conditions_hold(when),
                    &mut probe,
                    &String::new,
                )
            })
            .any(|prepared| prepared.fact.derivation())
    };
    roles.retain(|role| !derived(&|key| key.key.role().as_deref() == Some(role.as_str())));
    inputs.retain(|input| {
        !derived(&|key| key.key.input().as_deref() == Some(input.as_str()))
            && !(preset.id() == "kde-breeze" && input == "accent")
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
pub fn conditions(vector: &[String], flavor: Option<&str>) -> Result<Conditions, Unread> {
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

/// Whether a fact's `when` cannot hold under the run's `flavor` (S10,
/// S17): its `not_conditions` name that flavour, or its `conditions` name
/// another; its own combination is then no combination of that run.
fn excludes_flavor(when: &Value, flavor: Option<&str>) -> bool {
    let Some(flavor) = flavor else {
        return false;
    };
    let own = format!("flavor:{flavor}");
    strings(when.get("not_conditions")).contains(&own)
        || strings(when.get("conditions"))
            .iter()
            .any(|condition| condition.starts_with("flavor:") && *condition != own)
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

/// A hash of a combination's fields but its context, which equality
/// compares.
fn identity(combination: &Combination) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    (
        &combination.widget,
        &combination.variants,
        &combination.states,
        &combination.order,
        &combination.size,
        &combination.density,
        &combination.direction,
        combination.before,
        combination.after,
    )
        .hash(&mut hasher);
    hasher.finish()
}

/// A data combination as the model's.
fn combination_of(point: Point, widget: &str, context: Option<&Value>) -> Combination {
    let mut combination = Combination::new(widget);
    combination.variants = point.variants;
    combination.states = point.states;
    combination.order = point.order;
    combination.size = point.size;
    combination.density = point.density;
    combination.context = context.cloned();
    combination
}

/// A data combination's fields, as a state model's enumeration gives them
/// ([`combination_of`]).
fn point_of(combination: &Combination) -> Point {
    Point {
        conditions: Vec::new(),
        context: None,
        variants: combination.variants.clone(),
        states: combination.states.clone(),
        order: combination.order.clone(),
        size: combination.size.clone(),
        density: combination.density.clone(),
    }
}

/// Whether the gate skips each combination of `block`, of the widget
/// `widget`'s state model whose combinations are `space`, on its context
/// alone ([`Resolved::session`]): the context names what the model does
/// not hold yet, and every name the block's combinations hold reads, so
/// none is refused before its context is read (a combination's names read
/// where each of them does).
fn context_skips(widget: &str, space: &Space<'_>, block: &Block<'_>) -> bool {
    let Some(context) = block.context else {
        return false;
    };
    let reads = |set: &dyn Fn(&mut Combination)| {
        let mut combination = Combination::new(widget);
        set(&mut combination);
        names_read(&combination)
    };
    !context_held(context)
        && reads(&|combination| combination.variants.clone_from(&block.variants))
        && space.flag_sets.iter().all(|(states, order)| {
            reads(&|combination| {
                combination.states.clone_from(states);
                combination.order.clone_from(order);
            })
        })
        && block
            .sizes
            .iter()
            .all(|size| reads(&|combination| combination.size.clone_from(size)))
        && space
            .densities
            .iter()
            .all(|density| reads(&|combination| combination.density.clone_from(density)))
}

/// A widget's state model in force for one preset and flavour: its
/// combinations, projected onto what its rules and facts tell apart, those
/// the gate skips on their context alone left out ([`context_skips`]), and
/// its condition vectors.
struct Planned<'m> {
    entry: &'m Model,
    space: Space<'m>,
    vectors: Vec<Vec<String>>,
}

impl Planned<'_> {
    /// Whether its enumeration gives `combination`, left out or not.
    fn gives(&self, combination: &Combination) -> bool {
        let point = point_of(combination);
        let context = combination.context.as_ref();
        self.space.holds(&point, context)
            && combination_of(point, &self.entry.widget, context) == *combination
    }
}

/// Combinations each held once, in the order first given, told apart by a
/// hash of all but their context ([`identity`]) and equality.
#[derive(Default)]
struct Distinct {
    held: Vec<Combination>,
    by_identity: HashMap<u64, Vec<usize>>,
}

impl Distinct {
    /// The place of `combination` among those held.
    fn position(&self, combination: &Combination) -> Option<usize> {
        self.by_identity
            .get(&identity(combination))?
            .iter()
            .copied()
            .find(|at| self.held.get(*at) == Some(combination))
    }

    /// `combination` held, unless it is already.
    fn insert(&mut self, combination: Combination) {
        if self.position(&combination).is_none() {
            self.by_identity
                .entry(identity(&combination))
                .or_default()
                .push(self.held.len());
            self.held.push(combination);
        }
    }
}

/// A condition vector's combinations, given in turn to the function it is
/// called with, and the number of those left out, which the gate skips
/// ([`plan_vector`]).
type Combinations<'c> = dyn Fn(&mut dyn FnMut(Combination)) -> u64 + 'c;

/// The state models of `planned` whose vectors hold `vector`, one per
/// widget, and the combinations of `own`, the facts' own `when`s', that
/// none of their enumerations gives.
fn vector_plan<'p, 'm>(
    planned: &'p [Planned<'m>],
    own: Option<&'p Distinct>,
    vector: &[String],
) -> (Vec<&'p Planned<'m>>, Vec<&'p Combination>) {
    let models: Vec<&Planned<'_>> = planned
        .iter()
        .filter(|planned| planned.vectors.iter().any(|held| held == vector))
        .collect();
    let rest = own
        .iter()
        .flat_map(|own| &own.held)
        .filter(|combination| !models.iter().any(|planned| planned.gives(combination)))
        .collect();
    (models, rest)
}

/// The widget combinations of the condition vector `vector`, each once,
/// given to `each` as they are enumerated: each state model's of `planned`
/// whose vectors hold `vector`, in turn (one model per widget, each giving
/// each of its combinations once, [`Space::each`]), then each of `own`,
/// the facts' own `when`s', that none of them gives. Those the gate skips
/// on their context alone are left out, never enumerated
/// ([`context_skips`]); the number of them.
fn plan_vector(
    planned: &[Planned<'_>],
    own: Option<&Distinct>,
    vector: &[String],
    each: &mut dyn FnMut(Combination),
) -> u64 {
    let (models, rest) = vector_plan(planned, own, vector);
    let mut left_out = 0_u64;
    for planned in models {
        let widget = &planned.entry.widget;
        left_out = left_out.saturating_add(planned.space.each(vector, |point, context| {
            each(combination_of(point, widget, context));
        }));
    }
    for combination in rest {
        each(combination.clone());
    }
    left_out
}

/// The number of the widget combinations of the condition vector `vector`
/// ([`plan_vector`]), those left out among them.
fn vector_count(planned: &[Planned<'_>], own: Option<&Distinct>, vector: &[String]) -> u64 {
    let (models, rest) = vector_plan(planned, own, vector);
    models
        .iter()
        .map(|planned| planned.space.count())
        .chain(std::iter::once(
            u64::try_from(rest.len()).unwrap_or(u64::MAX),
        ))
        .fold(0, u64::saturating_add)
}

/// Runs the gate over the facts root `root` for `presets`, each with the
/// platform directory it is checked against (S17 *Generations*), the
/// citations' evidence read under the evidence root `evidence` (S17 *The
/// evidence root*: [`facts::evidence_root`]'s, or a fixture's own root), the
/// accent run's scheme files from `sources` (S17 *The scheme files read*).
pub fn run(
    root: &Path,
    evidence: &Path,
    sources: &facts::Sources,
    presets: &[(String, Preset)],
) -> Report {
    run_shard(root, evidence, sources, presets, Shard::WHOLE)
}

/// [`run`]'s shard `shard` alone ([`Shard`]): its condition vectors
/// checked, and where it is the first, the rest of the run recorded and the
/// accent run run whole, which no condition vector splits (S17).
pub fn run_shard(
    root: &Path,
    evidence: &Path,
    sources: &facts::Sources,
    presets: &[(String, Preset)],
    shard: Shard,
) -> Report {
    let started = Instant::now();
    let mut report = Report::default();
    let mut elsewhere = Report::default();
    let mut split = Split::new(shard);
    let evidence = facts::Evidence::new(evidence);
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
    let units = Units::repository().unwrap_or_else(|error| {
        shard
            .shared(&mut report, &mut elsewhere)
            .fail(format!("the units do not read: {error}"));
        Units::none()
    });
    for directory in directories {
        let name = directory
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if name == "imported" || name == "staged" || name == "palettes" {
            continue;
        }
        let shared = shard.shared(&mut report, &mut elsewhere);
        let platform = Platform::read(&directory, &units, shared);
        read_all(&platform, shared);
        // Every pending fact is listed, with the combinations it applied
        // in, none where it applied in none (S17).
        for fact in &platform.facts {
            if let Stated::Pending(id) = &fact.stated {
                shared
                    .pending
                    .entry(fact.place())
                    .or_insert_with(|| (id.clone(), fact.key.clone(), 0));
            }
        }
        shared.other_presets += platform
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
            let unspelled = unspelled_variants(entry, shared);
            let entry = &entry.without(&unspelled);
            let vectors = model::condition_vectors(
                &entry.axis_conditions,
                &["light".to_owned(), "dark".to_owned()],
            );
            let count = model::count(entry, &vectors);
            shared.combinations.insert(
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
            // While the flavours unit is not enabled the default flavour's
            // state models alone are in force (S10, ADD-A147): every
            // flavour's section is still checked whole against its own state
            // model at depth, the combinations only the default flavour's.
            let in_force = if platform.units.flavours_enabled() {
                flavors.len()
            } else {
                1
            };
            let shared = shard.shared(&mut report, &mut elsewhere);
            let keys = prepare(&platform, preset.id(), &evidence, shared);
            let not_drawn = not_drawn_map(&platform, preset.id(), shared);
            for flavor in &flavors {
                declarations_run(
                    &platform,
                    preset,
                    *flavor,
                    shard.shared(&mut report, &mut elsewhere),
                );
            }
            for flavor in flavors.into_iter().take(in_force) {
                check(
                    &platform,
                    preset,
                    flavor,
                    &keys,
                    &not_drawn,
                    &mut split,
                    &mut report,
                );
                let shared = shard.shared(&mut report, &mut elsewhere);
                let entries = projected(&platform, preset, flavor, shared);
                derivations(
                    &platform.name,
                    preset,
                    flavor,
                    &keys,
                    &entries,
                    &not_drawn,
                    shared,
                );
            }
            if shard.index == 0 {
                accent_run(
                    &AccentRun {
                        root,
                        platform: &platform,
                        preset,
                        keys: &keys,
                        evidence: &evidence,
                        sources,
                    },
                    &mut report,
                );
            }
            // The listed paths are found over every vector (S17, ADD-A128):
            // a split run leaves them to its shards' findings merged
            // ([`Shard`]).
            if shard == Shard::WHOLE {
                listed_paths(preset, &mut report);
            }
        }
    }
    report.elapsed = started.elapsed();
    report
}

/// The preset's `[pending]` table against the listed paths the run found
/// (S17, ADD-A128): an entry the preset's bases state is no listed path and
/// fails; so does an entry no combination leaves to a pending fact with
/// nothing beneath, and one whose register ids are not that path's pending
/// facts' set. A listed path the table lacks is S12's missing field, which
/// the preset's own loading refuses. An entry's list of ids is read on any
/// path, broader than S11's letter ("a list of ids where a field S6.1 sets
/// whole has leaves pending under different ids"): a plain field pending
/// under different ids per combination has no other encoding; a spec row
/// is pending to state it.
pub fn listed_paths(preset: &Preset, report: &mut Report) {
    for (path, ids) in preset.pending() {
        let id = ids.join(", ");
        // A pending default size is a declaration, which the declarations
        // run compares with its ids: listed once, at its declaration, and
        // stated nowhere (S17, ADD-A131).
        if is_listed_default(path) {
            if preset.states_default(path) {
                report.fail(format!(
                    "{}: `{path}` ({id}) is in its [pending] table, and the preset states it",
                    preset.id()
                ));
            } else {
                report.listed.insert(
                    format!("{} {path}", preset.id()),
                    (ids.iter().cloned().collect(), 1),
                );
            }
            continue;
        }
        if preset.states(path) {
            report.fail(format!(
                "{}: `{path}` ({id}) is in its [pending] table, and the preset states it",
                preset.id()
            ));
            continue;
        }
        let found = listed_key(path)
            .and_then(|key| report.listed.get(&format!("{} {key}", preset.id())))
            .cloned();
        match found {
            None => report.fail(format!(
                "{}: `{path}` ({id}) is in its [pending] table, and no combination leaves it to a \
                 pending fact with nothing beneath",
                preset.id()
            )),
            Some((facts, _)) if facts != ids.iter().cloned().collect::<BTreeSet<String>>() => {
                let found: Vec<String> = facts.into_iter().collect();
                let (fact, are) = if found.len() == 1 {
                    ("fact", "is")
                } else {
                    ("facts", "are")
                };
                report.fail(format!(
                    "{}: `{path}` is listed as {id} in its [pending] table, and its pending {fact} \
                     {are} {}",
                    preset.id(),
                    found.join(", ")
                ));
            }
            Some(_) => {}
        }
    }
}

/// What the accent run reads (S17 *The accent run*).
pub struct AccentRun<'a, 'f> {
    /// The facts root, whose `sources.toml` pins the scheme files' entries.
    pub root: &'a Path,
    pub platform: &'f Platform,
    pub preset: &'a Preset,
    /// The preset's facts prepared ([`prepare`]): its no-accent facts.
    pub keys: &'a [KeyFacts<'f>],
    pub evidence: &'a facts::Evidence,
    pub sources: &'a facts::Sources,
}

/// One fact of an accent-run pair, read for its key.
struct Measured<'f> {
    fact: &'f Fact,
    key: FactKey,
    when: When,
    /// Its value; `None` where it is pending.
    value: Option<FactValue>,
    tolerance: Tolerance,
}

/// The facts measured with one scheme file and one accent (S17 *The accent
/// run*), in file order, and the scheme their `when` names.
struct Pair<'f> {
    scheme_file: String,
    accent: String,
    scheme: String,
    facts: Vec<Measured<'f>>,
}

/// The accent-run facts of `run`'s platform that can apply to its preset,
/// in pairs ([`Pair`]), each read for its key; a fact the gate refuses
/// fails and is left out (S17 *Fields*, ADD-A142): one outside the `kde`
/// directory, a `scheme_file` or an `accent` without the other, a
/// `scheme_file` that is no `<entry> <path>`, a fact with no `scheme:`
/// condition or several, and one whose pair's first fact names another
/// scheme. What the model does not hold yet is counted, as [`read_all`]
/// counts it.
fn pairs<'f>(run: &AccentRun<'_, 'f>, report: &mut Report) -> Vec<Pair<'f>> {
    let preset = run.preset.id();
    let mut out: Vec<Pair<'f>> = Vec::new();
    for fact in run.platform.facts.iter().filter(|fact| fact.accent_run) {
        let place = fact.place();
        let Some((written, _)) = facts::for_preset(fact.when.as_ref(), preset) else {
            continue;
        };
        if run.platform.name != "kde" {
            report.fail(format!(
                "{place}: an accent-run fact outside the `kde` directory, which no accent run of \
                 the gate compares (S17)"
            ));
            continue;
        }
        let (Some(scheme_file), Some(accent)) = (&fact.scheme_file, &fact.accent) else {
            report.fail(format!(
                "{place}: `scheme_file` and `accent` are stated both or neither (S17)"
            ));
            continue;
        };
        if facts::scheme_file_parts(scheme_file).is_none() {
            report.fail(format!(
                "{place}: the scheme file `{scheme_file}` is no `<entry> <path>` (S17)"
            ));
            continue;
        }
        let schemes: Vec<String> =
            strings(written.as_ref().and_then(|when| when.get("conditions")))
                .into_iter()
                .filter_map(|condition| condition.strip_prefix("scheme:").map(str::to_owned))
                .collect();
        let [scheme] = schemes.as_slice() else {
            report.fail(format!(
                "{place}: {} `scheme:` conditions in its `when`, not one (S17, ADD-A142)",
                schemes.len()
            ));
            continue;
        };
        if !fact.enabled {
            *report
                .units_not_enabled
                .entry(fact.key.clone())
                .or_default() += 1;
            continue;
        }
        let key = match run.platform.key(&fact.key) {
            Some(Ok(key)) => key.clone(),
            Some(Err(Unread::NotHeld(_))) | None => {
                *report.keys_not_held.entry(fact.key.clone()).or_default() += 1;
                continue;
            }
            Some(Err(Unread::Refused(why))) => {
                report.fail(format!("{place}: the key `{}` is refused: {why}", fact.key));
                continue;
            }
        };
        let when = match read_when(written.as_ref()) {
            Ok(when) => when,
            Err(Unread::NotHeld(why)) => {
                *report.whens_not_held.entry(why).or_default() += 1;
                continue;
            }
            Err(Unread::Refused(why)) => {
                report.fail(format!("{place}: its `when` is refused: {why}"));
                continue;
            }
        };
        let value = match &fact.stated {
            Stated::Pending(_) => None,
            Stated::Value(value) => match parse_value(&key, value) {
                Ok(value) => Some(value),
                Err(Unread::NotHeld(why) | Unread::Refused(why)) => {
                    report.fail(format!(
                        "{place}: the value of `{}` is refused: {why}",
                        fact.key
                    ));
                    continue;
                }
            },
        };
        let measured = Measured {
            fact,
            key,
            when,
            value,
            tolerance: facts::tolerance(fact, run.evidence, preset)
                .drawn_by(drawing(&run.platform.name, None)),
        };
        match out
            .iter_mut()
            .find(|pair| pair.scheme_file == *scheme_file && pair.accent == *accent)
        {
            Some(pair) if pair.scheme != *scheme => report.fail(format!(
                "{place}: under scheme:{scheme}, its pair's first fact under scheme:{} \
                 (S17, ADD-A142)",
                pair.scheme
            )),
            Some(pair) => pair.facts.push(measured),
            None => out.push(Pair {
                scheme_file: scheme_file.clone(),
                accent: accent.clone(),
                scheme: scheme.clone(),
                facts: vec![measured],
            }),
        }
    }
    out
}

/// A citation's source less its line part (`breeze@6.8.2
/// colors/BreezeLight.colors:115` is `breeze@6.8.2
/// colors/BreezeLight.colors`).
fn source_file(source: &str) -> &str {
    match source.rsplit_once(':') {
        Some((file, lines))
            if !lines.is_empty()
                && lines
                    .chars()
                    .all(|c| c.is_ascii_digit() || c == '-' || c == ',') =>
        {
            file
        }
        _ => source,
    }
}

/// The preset's own scheme files, by scheme (S17 *The preset's own scheme
/// files*, ADD-A142): at each scheme the preset declares, the one `.colors`
/// file that the scheme-input facts (`input.kde_scheme.*`) whose `when`
/// names `scheme:<s>` cite, as `<entry> <path>`; a scheme with no such file
/// or several fails and has none.
fn own_files(run: &AccentRun<'_, '_>, report: &mut Report) -> BTreeMap<String, String> {
    let mut cited: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for fact in &run.platform.facts {
        if fact.accent_run || !fact.key.starts_with("input.kde_scheme.") {
            continue;
        }
        let Some((written, _)) = facts::for_preset(fact.when.as_ref(), run.preset.id()) else {
            continue;
        };
        let schemes: Vec<String> =
            strings(written.as_ref().and_then(|when| when.get("conditions")))
                .into_iter()
                .filter_map(|condition| condition.strip_prefix("scheme:").map(str::to_owned))
                .collect();
        let files: Vec<String> = fact
            .cites
            .iter()
            .filter_map(|cite| cite.get("source").and_then(Value::as_str))
            .map(source_file)
            .filter(|file| file.ends_with(".colors"))
            .map(str::to_owned)
            .collect();
        for scheme in schemes {
            cited
                .entry(scheme)
                .or_default()
                .extend(files.iter().cloned());
        }
    }
    let mut out = BTreeMap::new();
    for scheme in run.preset.schemes() {
        let files: Vec<&String> = cited.get(&scheme).into_iter().flatten().collect();
        match files.as_slice() {
            [file] => {
                out.insert(scheme, (*file).clone());
            }
            _ => report.fail(format!(
                "{}: {} own scheme files at scheme:{scheme}, not one: the `.colors` files its \
                 scheme-input facts under scheme:{scheme} cite ({}; S17, ADD-A142)",
                run.preset.id(),
                files.len(),
                files
                    .iter()
                    .map(|file| file.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
        }
    }
    out
}

/// The accent the preset states for no accent under `scheme`: the literal
/// of its `input.accent` fact that applies there (on `kde-breeze` Breeze's
/// own `#00000000`, `colorssettings.kcfg`'s default, S17 *The accent
/// run*), which the run sets on each own file and which marks a file's
/// no-accent pair; an error where none applies or it is no literal.
fn no_accent(run: &AccentRun<'_, '_>, scheme: &str) -> Result<String, String> {
    let vector = vec![format!("scheme:{scheme}")];
    let conditions = conditions(&vector, None).map_err(|unread| format!("{unread:?}"))?;
    let resolved = run.preset.resolve(&conditions, &Layer::default())?;
    let key = run
        .keys
        .iter()
        .find(|key| key.name == "input.accent")
        .ok_or_else(|| format!("{}: no `input.accent` fact", run.preset.id()))?;
    let mut probe = Report::default();
    let fact = applying(
        &key.facts,
        |when| resolved.conditions_hold(when),
        &mut probe,
        &String::new,
    )
    .ok_or_else(|| format!("no `input.accent` fact applies under {vector:?}"))?;
    match &fact.fact.stated {
        Stated::Value(Value::String(accent)) if !fact.fact.derivation() => Ok(accent.clone()),
        _ => Err(format!(
            "{}: the `input.accent` fact under {vector:?} states no colour literal",
            fact.fact.place()
        )),
    }
}

/// Whether the accents `a` and `b` are one literal, as written but for
/// the case of their letters.
fn same_accent(a: &str, b: &str) -> bool {
    a.trim().eq_ignore_ascii_case(b.trim())
}

/// One accent-run comparison: `value` of the fact at `place` for `key`
/// evaluated in `resolved`'s root scope beside the key's own value, at
/// `tolerance`; a value whose evaluation reaches a synthetic layer's step
/// is skipped and listed as the normal run lists it (S17, ADD-A128).
fn accent_compare(
    resolved: &Resolved,
    root: &RootInputs,
    (name, key): (&str, &FactKey),
    (value, tolerance): (&FactValue, &Tolerance),
    place: &str,
    combination: &dyn Fn() -> String,
    report: &mut Report,
) {
    match resolved.global_synthetic(key, Some(value), root, &[]) {
        Ok(true) => {
            let entry = report
                .synthetic
                .entry(place.to_owned())
                .or_insert_with(|| (name.to_owned(), 0));
            entry.1 += 1;
        }
        Ok(false) => {
            let outcome = resolved.compare_global(key, value, root, tolerance);
            record(outcome, name, place, report, combination, Pass::Accent);
        }
        Err(unread) => record(Err(unread), name, place, report, combination, Pass::Accent),
    }
}

/// Each fact of `pair` compared in `resolved` (S17 *The accent run*); a
/// pending one listed with its id and skipped, a fact whose `when` does not
/// hold there failed. With `counted`, each fact compared adds to
/// `Report::accent_run`.
fn compare_pair(
    pair: &Pair<'_>,
    resolved: &Resolved,
    root: &RootInputs,
    combination: &dyn Fn() -> String,
    counted: bool,
    report: &mut Report,
) {
    for measured in &pair.facts {
        let place = measured.fact.place();
        if !resolved.conditions_hold(&measured.when) {
            report.fail(format!(
                "{place}: its `when` does not hold where its pair is resolved, {}",
                combination()
            ));
            continue;
        }
        let Some(value) = &measured.value else {
            if let Stated::Pending(id) = &measured.fact.stated {
                let entry = report
                    .pending
                    .entry(place)
                    .or_insert_with(|| (id.clone(), measured.fact.key.clone(), 0));
                entry.2 += 1;
            }
            continue;
        };
        accent_compare(
            resolved,
            root,
            (&measured.fact.key, &measured.key),
            (value, &measured.tolerance),
            &place,
            combination,
            report,
        );
        if counted {
            report.accent_run += 1;
        }
    }
}

/// The accent run (S17 *The accent run*, ADD-A142) of `run`'s preset over
/// its platform's accent-run facts, run whole by the first shard alone:
///
/// - each pair (a scheme file and an accent) is resolved under the scheme
///   its facts name, with that accent set through `set_accent`: over the
///   preset's own scheme inputs for one of its own files ([`own_files`]),
///   else with the palette `from_port` converts from the file applied,
///   the file read from `run.sources`; its facts compared;
/// - each own file is resolved with the preset's no-accent accent
///   ([`no_accent`]) set, and every key the pairs state compared with the
///   preset's no-accent facts that apply there;
/// - each other file is also resolved with no accent set, its port's own
///   roles showing (*A port's own roles*), and compared with its no-accent
///   pair; a key of its pairs with no fact there, stated or pending, fails
///   as a key with no fact (R4).
///
/// A pair naming an own file under the other scheme fails, as does a file
/// the sources lack or hold at another pin, never comparing nothing.
pub fn accent_run(run: &AccentRun<'_, '_>, report: &mut Report) {
    let started = Instant::now();
    let pairs = pairs(run, report);
    if pairs.is_empty() {
        return;
    }
    let own = own_files(run, report);
    // The no-accent accent at each scheme a pair or an own file names.
    let mut markers: BTreeMap<&str, String> = BTreeMap::new();
    for scheme in pairs
        .iter()
        .map(|pair| pair.scheme.as_str())
        .chain(own.keys().map(String::as_str))
    {
        if markers.contains_key(scheme) {
            continue;
        }
        match no_accent(run, scheme) {
            Ok(accent) => {
                markers.insert(scheme, accent);
            }
            Err(why) => report.fail(format!("the no-accent accent at scheme:{scheme}: {why}")),
        }
    }
    // Each file that is not the preset's own, read once; a pair naming an
    // own file under the other scheme refused.
    let mut ports: BTreeMap<&str, Vec<u8>> = BTreeMap::new();
    let mut refused: BTreeSet<&str> = BTreeSet::new();
    let mut others: Vec<(&str, &str)> = Vec::new();
    let mut kept: Vec<&Pair<'_>> = Vec::new();
    for pair in &pairs {
        let file = pair.scheme_file.as_str();
        if own.get(&pair.scheme).map(String::as_str) == Some(file) {
            kept.push(pair);
            continue;
        }
        if let Some((scheme, _)) = own.iter().find(|(_, own)| own.as_str() == file) {
            report.fail(format!(
                "the pair {file} {} names the preset's own file at scheme:{scheme} under \
                 scheme:{} (S17, ADD-A142)",
                pair.accent, pair.scheme
            ));
            continue;
        }
        if !others.contains(&(file, pair.scheme.as_str())) {
            others.push((file, &pair.scheme));
        }
        if !ports.contains_key(file) && !refused.contains(file) {
            match run.sources.read(run.root, file) {
                Ok(bytes) => {
                    ports.insert(file, bytes);
                }
                Err(error) => {
                    report.fail(error);
                    refused.insert(file);
                }
            }
        }
        if ports.contains_key(file) {
            kept.push(pair);
        }
    }
    // Every pair through set_accent, each own file's no-accent row, and
    // each other file's no-accent pair with no accent set, its keys with
    // no fact there, stated or pending, failed.
    let mut jobs: Vec<AccentJob<'_, '_>> = kept.into_iter().map(AccentJob::Pair).collect();
    for (scheme, file) in &own {
        if let Some(accent) = markers.get(scheme.as_str()) {
            jobs.push(AccentJob::Own {
                scheme,
                file,
                accent,
            });
        }
    }
    for (file, scheme) in others {
        let Some(marker) = markers.get(scheme) else {
            continue;
        };
        let none = pairs.iter().find(|pair| {
            pair.scheme_file == file && pair.scheme == scheme && same_accent(&pair.accent, marker)
        });
        let mut keys: Vec<&str> = Vec::new();
        for measured in pairs
            .iter()
            .filter(|pair| pair.scheme_file == file)
            .flat_map(|pair| &pair.facts)
        {
            if !keys.contains(&measured.fact.key.as_str()) {
                keys.push(&measured.fact.key);
            }
        }
        for name in keys {
            let held = none
                .is_some_and(|pair| pair.facts.iter().any(|measured| measured.fact.key == name));
            if !held {
                report.fail_in(
                    ("Accent".to_owned(), name.to_owned(), String::new()),
                    format!("`{name}` has no fact under {file} with no accent set under [\"scheme:{scheme}\"]"),
                );
            }
        }
        if let Some(none) = none
            && ports.contains_key(file)
        {
            jobs.push(AccentJob::Port(none));
        }
    }
    // The keys the pairs state: the roles each own file's no-accent row
    // compares.
    let mut stated: Vec<&str> = Vec::new();
    for measured in pairs.iter().flat_map(|pair| &pair.facts) {
        if !stated.contains(&measured.fact.key.as_str()) {
            stated.push(&measured.fact.key);
        }
    }
    let root = root_inputs();
    let found = parallel(&jobs, |job| {
        let mut found = Report::default();
        job.check(run, &ports, &stated, &root, &mut found);
        found
    });
    for found in found {
        report.merge(found);
    }
    super::timing::print(
        &format!("{} accent run", run.preset.id()),
        started.elapsed(),
    );
}

/// One resolution of the accent run ([`accent_run`]) and what it compares.
enum AccentJob<'p, 'f> {
    /// A pair, its accent set through `set_accent`, over its file's port
    /// where the file is not the preset's own.
    Pair(&'p Pair<'f>),
    /// The preset's own file `file` at `scheme` with the no-accent accent
    /// `accent` set, against the preset's no-accent facts.
    Own {
        scheme: &'p str,
        file: &'p str,
        accent: &'p str,
    },
    /// A file's no-accent pair with no accent set, its port's own roles
    /// showing (*A port's own roles*).
    Port(&'p Pair<'f>),
}

impl AccentJob<'_, '_> {
    /// The job's resolution, its comparisons recorded in `report`: `ports`
    /// holds the files that are not the preset's own, `stated` the keys an
    /// own file's row compares.
    fn check(
        &self,
        run: &AccentRun<'_, '_>,
        ports: &BTreeMap<&str, Vec<u8>>,
        stated: &[&str],
        root: &RootInputs,
        report: &mut Report,
    ) {
        let (scheme, port, accent) = match self {
            AccentJob::Pair(pair) => (
                pair.scheme.as_str(),
                ports.get(pair.scheme_file.as_str()),
                Some(pair.accent.as_str()),
            ),
            AccentJob::Own { scheme, accent, .. } => (*scheme, None, Some(*accent)),
            AccentJob::Port(pair) => (
                pair.scheme.as_str(),
                ports.get(pair.scheme_file.as_str()),
                None,
            ),
        };
        let vector = vec![format!("scheme:{scheme}")];
        let conditions = match conditions(&vector, None) {
            Ok(conditions) => conditions,
            Err(Unread::NotHeld(why) | Unread::Refused(why)) => {
                report.fail(format!("the conditions {vector:?} are refused: {why}"));
                return;
            }
        };
        let resolved = match run
            .preset
            .resolve_accent(&conditions, port.map(Vec::as_slice), accent)
        {
            Ok(resolved) => resolved,
            Err(why) => {
                report.fail(format!(
                    "Accent: {} does not resolve under {vector:?}: {why}",
                    run.preset.id()
                ));
                return;
            }
        };
        match self {
            AccentJob::Pair(pair) => {
                let combination = || {
                    format!(
                        "{} with the accent {} under {vector:?}",
                        pair.scheme_file, pair.accent
                    )
                };
                compare_pair(pair, &resolved, root, &combination, true, report);
            }
            AccentJob::Port(pair) => {
                let combination =
                    || format!("{} with no accent set under {vector:?}", pair.scheme_file);
                compare_pair(pair, &resolved, root, &combination, false, report);
            }
            AccentJob::Own { file, accent, .. } => {
                let combination = || format!("{file} with the accent {accent} under {vector:?}");
                compare_own(run, &resolved, stated, root, &combination, report);
            }
        }
    }
}

/// Each key of `stated` compared in `resolved`, an own file with the
/// no-accent accent set, with the preset's no-accent fact that applies
/// there (S17 *The accent run*); a key with none fails (R4), a pending one
/// is listed with its id.
fn compare_own(
    run: &AccentRun<'_, '_>,
    resolved: &Resolved,
    stated: &[&str],
    root: &RootInputs,
    combination: &dyn Fn() -> String,
    report: &mut Report,
) {
    for name in stated {
        let fact = run
            .keys
            .iter()
            .find(|key| key.name == *name)
            .and_then(|key| {
                let fact = applying(
                    &key.facts,
                    |when| resolved.conditions_hold(when),
                    report,
                    combination,
                );
                fact.map(|fact| (key, fact))
            });
        let Some((key, prepared)) = fact else {
            report.fail_in(
                ("Accent".to_owned(), (*name).to_owned(), String::new()),
                format!("`{name}` has no fact under {}", combination()),
            );
            continue;
        };
        match (&prepared.fact.stated, &prepared.value) {
            (Stated::Pending(id), _) => {
                let entry = report
                    .pending
                    .entry(prepared.fact.place())
                    .or_insert_with(|| (id.clone(), (*name).to_owned(), 0));
                entry.2 += 1;
            }
            (Stated::Value(_), Some(value)) => accent_compare(
                resolved,
                root,
                (name, &key.key),
                (
                    value,
                    &prepared
                        .tolerance
                        .drawn_by(drawing(&run.platform.name, None)),
                ),
                &prepared.fact.place(),
                combination,
                report,
            ),
            (Stated::Value(_), None) => report.fail(format!(
                "{}: `{name}`'s fact is a set-whole block's leaf, which the accent run does not \
                 compose, {}",
                prepared.fact.place(),
                combination()
            )),
        }
    }
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
    let entries = declared_entries(platform, preset, flavor, report);
    let depth = Depth::new(platform, preset.id());
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
    for entry in &entries {
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
        let sized = |entries: &[(String, Vec<String>, Option<String>)]| {
            entries
                .iter()
                .map(|(name, list, default)| (name.clone(), set(list), default.clone()))
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
            "later.variants",
            format!("{:?}", set(&declared.later)),
            format!(
                "{:?}",
                set(&later_variants(
                    entry,
                    &platform.units,
                    preset.id(),
                    &depth.drawn_variants(&entry.widget)
                ))
            ),
        );
        differ(
            "sizes_by_variant",
            format!("{:?}", sized(&declared.sizes_by_variant)),
            format!("{:?}", sized(&entry.sizes_by_variant)),
        );
        // An entry whose context names what the model does not hold yet is
        // one the preset cannot declare (S12), skipped as its combinations
        // are ([`context_skips`]).
        let contexts: Vec<(String, Vec<String>, Option<String>)> = entry
            .sizes_by_context
            .iter()
            .filter(|(selector, _, _)| context_held(selector))
            .map(|(selector, sizes, default)| {
                (
                    model::selector_spelled(selector),
                    sizes.clone(),
                    default.clone(),
                )
            })
            .collect();
        differ(
            "sizes_by_context",
            format!("{:?}", sized(&declared.sizes_by_context)),
            format!("{:?}", sized(&contexts)),
        );
    }
}

/// A fact's own combinations, its own combination completed to those the
/// state model `entry` draws (S17: the gate also resolves each fact's own
/// `when`): with what the entry requires ([`required`]); where the size it
/// names is one its variants do not take (`ios-13`'s facts naming `mini`
/// and no style, which only the styles take), once with each
/// `sizes_by_variant` entry's variant whose sizes hold it, the `when` not
/// excluding it and no variant held sharing an `exclusive_variants` group
/// with it, each completion that then takes the size (ADD-A141); where none
/// does, the completion with the size alone, which the run finds not
/// available. Empty where nothing required is left, as the fact then
/// applies to no element the platform draws.
pub fn complete(combination: Combination, when: &Value, entry: &Model) -> Vec<Combination> {
    let Some(completed) = required(combination.clone(), when, entry) else {
        return Vec::new();
    };
    let Some(size) = completed.size.clone() else {
        return vec![completed];
    };
    let takes = |held: &Combination| {
        model::size_options(entry, &held.variants, held.context.as_ref())
            .contains(&Some(size.clone()))
    };
    if takes(&completed) {
        return vec![completed];
    }
    let excluded = strings(when.get("not_variants"));
    let mut styled: Vec<Combination> = Vec::new();
    for (variant, sizes, _) in &entry.sizes_by_variant {
        if !sizes.contains(&size)
            || excluded.contains(variant)
            || combination.variants.contains(variant)
            || exclusive_with(entry, variant, &combination.variants)
        {
            continue;
        }
        let mut with = combination.clone();
        with.variants.push(variant.clone());
        if let Some(with) = required(with, when, entry)
            && takes(&with)
            && !styled.contains(&with)
        {
            styled.push(with);
        }
    }
    if styled.is_empty() {
        vec![completed]
    } else {
        styled
    }
}

/// The combination completed with what the state model `entry` requires
/// (S17): where the entry requires a variant the `when` names none of, the
/// first required variant its `when` does not exclude and no variant it
/// holds shares an `exclusive_variants` group with; `None` where none is
/// left.
fn required(mut combination: Combination, when: &Value, entry: &Model) -> Option<Combination> {
    let required = &entry.requires_variants;
    if required.is_empty() || required.iter().any(|v| combination.variants.contains(v)) {
        return Some(combination);
    }
    let excluded = strings(when.get("not_variants"));
    let variant = required.iter().find(|variant| {
        !excluded.contains(variant) && !exclusive_with(entry, variant, &combination.variants)
    })?;
    combination.variants.push(variant.clone());
    Some(combination)
}

/// Whether `variant` shares one of the entry's `exclusive_variants` groups
/// with a variant `held`.
fn exclusive_with(entry: &Model, variant: &String, held: &[String]) -> bool {
    entry
        .exclusive_variants
        .iter()
        .any(|group| group.contains(variant) && held.iter().any(|other| group.contains(other)))
}

/// Each state-model entry's `not_drawn` entries read for the preset, once,
/// by [`entry_id`].
type NotDrawn = BTreeMap<String, Vec<When>>;

/// A state-model entry's identity: its file, widget and conditions.
fn entry_id(entry: &Model) -> String {
    format!("{} {} {:?}", entry.file, entry.widget, entry.conditions)
}

/// Every entry's `not_drawn` entries for the preset, each refusal reported
/// once, whatever the passes and flavours that read them.
fn not_drawn_map(platform: &Platform, preset: &str, report: &mut Report) -> NotDrawn {
    platform
        .models
        .iter()
        .map(|entry| (entry_id(entry), not_drawn_whens(entry, preset, report)))
        .collect()
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
/// decided by the facts that apply under the vector. The vectors are
/// checked as many at once as [`threads`] gives, each resolved, checked
/// into its own report and dropped with its combinations, which are
/// enumerated as they are checked ([`plan_vector`]), never all held: one
/// vector's run to millions. The reports are merged in the vectors' order,
/// as one run checking each in turn records them. Only the vectors `split`
/// gives this run's shard are checked; the rest is recorded by the first
/// shard alone ([`Shard`]).
fn check(
    platform: &Platform,
    preset: &Preset,
    flavor: Option<&str>,
    keys: &[KeyFacts<'_>],
    entry_not_drawn: &NotDrawn,
    split: &mut Split,
    report: &mut Report,
) {
    let started = Instant::now();
    let mut elsewhere = Report::default();
    let shared = split.shard.shared(report, &mut elsewhere);
    let schemes = preset.schemes();
    let drawing = drawing(&platform.name, flavor);
    let entries = projected(platform, preset, flavor, shared);
    let mut planned: Vec<Planned<'_>> = Vec::new();
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
            entry_not_drawn
                .get(&entry_id(entry))
                .cloned()
                .unwrap_or_default(),
        );
        let mut mentioned = Mentioned::of_model(entry);
        for key in keys
            .iter()
            .filter(|key| key.widget.as_deref() == Some(entry.widget.as_str()))
        {
            for prepared in &key.facts {
                if let Some(when) = &prepared.written {
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
                    .map(|(variant, _, _)| variant),
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
        let space = Space::of(entry, &mentioned, |space, block| {
            context_skips(&entry.widget, space, block)
        });
        planned.push(Planned {
            entry,
            space,
            vectors,
        });
    }
    // Each fact's own `when`, by condition vector.
    let mut own: BTreeMap<Vec<String>, Distinct> = BTreeMap::new();
    for key in keys {
        for prepared in &key.facts {
            let Some(when) = &prepared.written else {
                continue;
            };
            if excludes_flavor(when, flavor) {
                continue;
            }
            let widget = key.key.widget();
            let Some((vectors, combination)) =
                own_combination(when, widget.as_deref().unwrap_or_default(), &schemes)
            else {
                shared.combinations_skipped += 1;
                continue;
            };
            let completed = match entries
                .iter()
                .find(|entry| Some(&entry.widget) == widget.as_ref())
            {
                Some(entry) => complete(combination, when, entry),
                None => vec![combination],
            };
            if completed.is_empty() {
                shared.whens_not_drawn += 1;
                continue;
            }
            for vector in vectors {
                match &widget {
                    Some(_) => {
                        let held = own.entry(vector).or_default();
                        for combination in &completed {
                            held.insert(combination.clone());
                        }
                    }
                    None => {
                        global_vectors.insert(vector);
                    }
                }
            }
        }
    }
    let mut vectors = global_vectors.clone();
    vectors.extend(own.keys().cloned());
    let vectors: Vec<Vec<String>> = vectors.into_iter().collect();
    let plan = |vector: &[String], each: &mut dyn FnMut(Combination)| {
        plan_vector(&planned, own.get(vector), vector, each)
    };
    let count = |vector: &[String]| vector_count(&planned, own.get(vector), vector);
    let root = root_inputs();
    // As many vectors checked at once as there are threads, the threads
    // shared among those being checked ([`Threads`]); those the state
    // models plan, the most combinations, first.
    let planned_in = |vector: &[String]| {
        planned
            .iter()
            .filter(|planned| planned.vectors.iter().any(|held| held == vector))
            .count()
    };
    let mut order: Vec<usize> = (0..vectors.len()).collect();
    order.sort_by_key(|at| {
        std::cmp::Reverse(vectors.get(*at).map_or(0, |vector| planned_in(vector)))
    });
    // This shard's vectors, the split decided over every vector, the most
    // combinations first.
    let mut weights: Vec<(usize, u64)> = vectors
        .iter()
        .enumerate()
        .map(|(at, vector)| (at, count(vector)))
        .collect();
    weights.sort_by_key(|(at, weight)| (std::cmp::Reverse(*weight), *at));
    let mine: HashSet<usize> = split.take(&weights).into_iter().collect();
    order.retain(|at| mine.contains(at));
    let widgets = widget_keys(keys);
    let threads = Threads {
        count: threads(),
        checking: AtomicUsize::new(0),
    };
    let at_once = threads.count.min(order.len()).max(1);
    // Each vector resolved and checked on its own, into its own report;
    // the reports merged in the vectors' order, as one run checking each
    // in turn records them.
    let mut found = parallel_on(at_once, &order, |at| {
        let mut report = Report::default();
        threads.checking.fetch_add(1, Ordering::Relaxed);
        if let Some(vector) = vectors.get(*at) {
            let resolved = timed(Step::Resolve, || {
                resolve_vector(preset, keys, vector, flavor, &count, &mut report)
            });
            if let Some((normal, perturbed)) = resolved {
                let passes = std::iter::once((Pass::Normal, Some(&normal)))
                    .chain(std::iter::once((Pass::Perturbed, perturbed.as_ref())));
                // The normal pass's probes and the facts that hold in its
                // resolution, which the perturbed pass replays where it
                // reads alike ([`replays`]).
                let mut probed: Option<(Probed, HashMap<&str, Holding<'_>>)> = None;
                for (pass, resolved) in passes {
                    let Some(resolved) = resolved else {
                        continue;
                    };
                    if global_vectors.contains(vector) {
                        check_globals(
                            preset.id(),
                            resolved,
                            keys,
                            vector,
                            (pass, drawing),
                            &root,
                            &mut report,
                        );
                        if pass == Pass::Normal {
                            definitions(
                                preset.id(),
                                &platform.units,
                                resolved,
                                &root,
                                &mut report,
                                &|| format!("{vector:?}"),
                            );
                        }
                    }
                    let holding = holding(&widgets, resolved);
                    let run = Run {
                        preset: preset.id(),
                        resolved,
                        vector,
                        not_drawn: &not_drawn,
                        pass,
                        drawing,
                        root: &root,
                        threads: &threads,
                        widgets: &widgets,
                        holding: &holding,
                    };
                    let combinations = |each: &mut dyn FnMut(Combination)| plan(vector, each);
                    match &probed {
                        Some((normal_probed, normal_holding))
                            if replays(&normal, resolved, &root, normal_holding, &holding) =>
                        {
                            replay(&run, normal_probed, &combinations, &mut report);
                        }
                        _ => {
                            if pass == Pass::Perturbed {
                                super::timing::note(Step::Unlike);
                            }
                            let found = widgets_checked(&run, &combinations, &mut report);
                            if super::timing::measuring() {
                                super::timing::sizes(
                                    &format!("{pass:?} {vector:?} probed"),
                                    &found.sizes(),
                                );
                            }
                            if pass == Pass::Normal {
                                probed = Some((found, holding));
                            }
                        }
                    }
                }
            }
        }
        threads.checking.fetch_sub(1, Ordering::Relaxed);
        if super::timing::measuring() {
            super::timing::sizes(
                &format!("{:?} report", vectors.get(*at)),
                &[
                    ("failures", report.failures.len()),
                    (
                        "failures bytes",
                        report.failures.iter().map(String::capacity).sum(),
                    ),
                    ("pending", report.pending.len()),
                    ("listed", report.listed.len()),
                    ("synthetic", report.synthetic.len()),
                    ("unchecked", report.unchecked.len()),
                    ("groups", report.groups.len()),
                ],
            );
        }
        (*at, report)
    });
    found.sort_by_key(|(at, _)| *at);
    for (_, found) in found {
        report.merge(found);
    }
    super::timing::print(
        &format!("{} {flavor:?}, {} vectors", preset.id(), order.len()),
        started.elapsed(),
    );
}

/// A condition vector resolved for the normal run and the perturbed run,
/// whose stand-in the facts that apply under it decide; `None` where its
/// conditions cannot be set, its combinations (`count` of them) skipped,
/// or the normal run does not resolve.
fn resolve_vector(
    preset: &Preset,
    keys: &[KeyFacts<'_>],
    vector: &[String],
    flavor: Option<&str>,
    count: &dyn Fn(&[String]) -> u64,
    report: &mut Report,
) -> Option<(Resolved, Option<Resolved>)> {
    let conditions = match conditions(vector, flavor) {
        Ok(conditions) => conditions,
        Err(Unread::NotHeld(_)) => {
            report.combinations_skipped += count(vector);
            return None;
        }
        Err(Unread::Refused(why)) => {
            report.fail(format!("the conditions {vector:?} are refused: {why}"));
            return None;
        }
    };
    let resolve =
        |pass: Pass, layer: &Layer, report: &mut Report| match preset.resolve(&conditions, layer) {
            Ok(resolved) => Some(resolved),
            Err(why) => {
                report.fail(format!(
                    "{pass:?}: {} does not resolve under {vector:?}: {why}",
                    preset.id()
                ));
                None
            }
        };
    let normal = resolve(Pass::Normal, &Layer::default(), report)?;
    let layer = match stand_in(preset, keys, &normal) {
        Ok(layer) => layer,
        Err(why) => {
            report.fail(format!("Perturbed: the stand-in layer is refused: {why}"));
            return None;
        }
    };
    let perturbed = resolve(Pass::Perturbed, &layer, report);
    Some((normal, perturbed))
}

/// How the renderer of a facts directory's platform draws a colour a
/// measured citation read from drawn pixels (S17, ADD-A131): GTK's
/// (`gnome`) and, for what QPainter draws, Qt's raster engine (`kde`) premultiply and clamp
/// each premultiplied channel; Compose (`material`) clamps each straight
/// channel; Qt Quick's scene graph (`kde` under `flavor:kirigami`),
/// `macos`, `ios`, `windows` and any other directory no source states.
pub fn drawing(platform: &str, flavor: Option<&str>) -> Drawing {
    match (platform, flavor) {
        ("kde", Some("kirigami")) => Drawing::Unread,
        ("gnome" | "kde", _) => Drawing::Premultiplied,
        ("material", _) => Drawing::Clamped,
        _ => Drawing::Unread,
    }
}

/// The values the specification itself defines, which no fact states
/// (S17, ADD-A125, ADD-A131): `token.text.font_dpi` 72 on the Apple presets
/// (S9.4), `token.icons.animated_visuals` `{}` on every preset but
/// `windows-11` (S4.5); each where an enabled unit holds its path on the
/// preset (`units`: S4.5's is staged with the motion unit, S6.1, ADD-A147,
/// as scripts/generate_preset.py's `definitions()` leaves it out),
/// compared with the resolved token as a fact would be.
pub fn definitions(
    preset: &str,
    units: &Units,
    resolved: &Resolved,
    root: &RootInputs,
    report: &mut Report,
    combination: &dyn Fn() -> String,
) {
    let apple = preset.starts_with("macos-") || preset.starts_with("ios-");
    let defined = [
        (apple, "token.text.font_dpi", Value::Integer(72), "S9.4"),
        (
            preset != "windows-11",
            "token.icons.animated_visuals",
            Value::Table(toml::Table::new()),
            "S4.5",
        ),
    ];
    for (applies, name, value, section) in defined {
        if !applies || !units.hold_on(name, Some(preset)) {
            continue;
        }
        let place = format!("the definition of {section}");
        let outcome = parse_key(name).and_then(|key| {
            let value = parse_value(&key, &value)?;
            resolved.compare_global(&key, &value, root, &Tolerance::quote())
        });
        record(outcome, name, &place, report, combination, Pass::Normal);
    }
}

/// The threads a check's condition vectors share: [`threads`] of them,
/// split among the vectors being checked, so a vector still checked as
/// others finish takes the threads they leave.
struct Threads {
    count: usize,
    checking: AtomicUsize,
}

impl Threads {
    /// The threads each vector being checked probes its combinations on.
    fn each(&self) -> usize {
        (self.count / self.checking.load(Ordering::Relaxed).max(1)).max(1)
    }
}

/// One resolution of a condition vector, for one pass.
struct Run<'a, 'f> {
    preset: &'a str,
    resolved: &'a Resolved,
    vector: &'a [String],
    /// Each widget's `not_drawn` entries.
    not_drawn: &'a BTreeMap<String, Vec<When>>,
    pass: Pass,
    /// How the measured platform's renderer draws a colour (S17).
    drawing: Drawing,
    root: &'a RootInputs,
    /// The threads its combinations are probed and compared on.
    threads: &'a Threads,
    /// Each widget's keys of the preset ([`widget_keys`]).
    widgets: &'a HashMap<&'a str, WidgetKeys<'a, 'f>>,
    /// Each widget's facts whose conditions hold in `resolved`
    /// ([`holding`]).
    holding: &'a HashMap<&'a str, Holding<'a>>,
}

impl<'a, 'f> Run<'a, 'f> {
    /// The facts that apply to the keys of `widget`, `session`'s
    /// combination's widget ([`applying_in`]); none where it has none.
    fn applying(
        &self,
        widget: &str,
        session: &native_theme::gate::Session<'_>,
        report: &mut Report,
        describe: &dyn Fn() -> String,
    ) -> Applying<'_, 'f> {
        static NO_KEYS: WidgetKeys<'static, 'static> = WidgetKeys {
            keys: Vec::new(),
            whens: Vec::new(),
            starts: Vec::new(),
            classes: Vec::new(),
        };
        static NO_WHENS: Holding<'static> = Holding {
            whens: Vec::new(),
            at: Vec::new(),
        };
        let keys = self.widgets.get(widget).unwrap_or(&NO_KEYS);
        let holding = self.holding.get(widget).unwrap_or(&NO_WHENS);
        applying_in(keys, holding, session, report, describe)
    }

    /// [`Run::applying`] and the session's step signature
    /// ([`Session::signature`]) at once, each part's element matched once
    /// ([`Session::probed`]): what a probe of shapes alone asks.
    ///
    /// [`Session::signature`]: native_theme::gate::Session::signature
    /// [`Session::probed`]: native_theme::gate::Session::probed
    fn probed(
        &self,
        widget: &str,
        session: &native_theme::gate::Session<'_>,
        report: &mut Report,
        describe: &dyn Fn() -> String,
    ) -> (Applying<'_, 'f>, Vec<u32>) {
        static NO_KEYS: WidgetKeys<'static, 'static> = WidgetKeys {
            keys: Vec::new(),
            whens: Vec::new(),
            starts: Vec::new(),
            classes: Vec::new(),
        };
        static NO_WHENS: Holding<'static> = Holding {
            whens: Vec::new(),
            at: Vec::new(),
        };
        let keys = self.widgets.get(widget).unwrap_or(&NO_KEYS);
        let holding = self.holding.get(widget).unwrap_or(&NO_WHENS);
        let (holds, steps) = session.probed(&holding.whens);
        (
            applying_from(keys, holding, &holds, report, describe),
            steps,
        )
    }
}

/// Every token, colour-role and input key under one condition vector.
fn check_globals(
    preset: &str,
    resolved: &Resolved,
    keys: &[KeyFacts<'_>],
    vector: &[String],
    (pass, drawing): (Pass, Drawing),
    root: &RootInputs,
    report: &mut Report,
) {
    let combination = || format!("{vector:?}");
    let globals: Vec<&KeyFacts<'_>> = keys
        .iter()
        .filter(|key| key.key.widget().is_none())
        .collect();
    let facts: Vec<Option<&Prepared<'_>>> = globals
        .iter()
        .map(|key| {
            applying(
                &key.facts,
                |when| resolved.conditions_hold(when),
                report,
                &combination,
            )
        })
        .collect();
    for (index, (key, fact)) in globals.iter().zip(facts.iter().copied()).enumerate() {
        // A later fact of a block holding the key states it (S17 *Which
        // fact applies*, as for a widget's keys): the block's comparison is
        // the key's.
        if blocks_of(&globals, index).into_iter().any(|block| {
            facts
                .get(block)
                .copied()
                .flatten()
                .is_some_and(|stated| later(stated, fact))
        }) {
            continue;
        }
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
        if pass == Pass::Normal
            && let Some(Prepared {
                fact:
                    Fact {
                        stated: Stated::Pending(id),
                        ..
                    },
                ..
            }) = fact
            && !(key.key.input().is_some() && under_contrast(fact.map(|fact| fact.fact)))
            && listed_or_failed(
                resolved.global_listed(&key.key, root),
                &key.name,
                report,
                &combination,
            )
        {
            found_listed(report, preset, key, resolved.listed_scheme(&key.key), id);
        }
        if let Verdict::Compare(value, tolerance) = verdict(
            &key.name,
            false,
            fact,
            whole.as_ref(),
            pass,
            report,
            &combination,
        ) {
            let except = stated_below(&globals, &facts, index, fact);
            match resolved.global_synthetic(&key.key, Some(value), root, &except) {
                Ok(true) => synthetic_skip(report, &place, &key.name, pass),
                Ok(false) => {
                    let tolerance = tolerance.drawn_by(drawing);
                    let outcome =
                        resolved.compare_global_except(&key.key, value, root, &tolerance, &except);
                    record(outcome, &key.name, &place, report, &combination, pass);
                }
                Err(unread) => record(Err(unread), &key.name, &place, report, &combination, pass),
            }
        }
    }
}

/// Whether a fact holds under a contrast condition only: its `when`'s
/// `conditions` name one. An input's such pending fact is a contrast base,
/// which is never listed (S17, ADD-A132): the gate lists it with its id as
/// any pending fact, and its id is no listed input's.
fn under_contrast(fact: Option<&Fact>) -> bool {
    fact.is_some_and(|fact| {
        strings(fact.when.as_ref().and_then(|when| when.get("conditions")))
            .iter()
            .any(|condition| condition == "contrast_increased" || condition == "contrast_high")
    })
}

/// Whether a key is a listed path where its pending fact applies (S17,
/// ADD-A128); a refusal to tell is a failure, recorded, and no listed path.
fn listed_or_failed(
    listed: Result<bool, Unread>,
    key: &str,
    report: &mut Report,
    combination: &dyn Fn() -> String,
) -> bool {
    listed.unwrap_or_else(|(Unread::NotHeld(why) | Unread::Refused(why))| {
        report.fail(format!(
            "`{key}` under {}: whether it is a listed path: {why}",
            combination()
        ));
        false
    })
}

/// A fact skipped as its evaluation reaches a synthetic value (S17,
/// ADD-A128), counted and reported by its place.
fn synthetic_skip(report: &mut Report, place: &str, key: &str, pass: Pass) {
    if pass == Pass::Normal {
        let entry = report
            .synthetic
            .entry(place.to_owned())
            .or_insert_with(|| (key.to_owned(), 0));
        entry.1 += 1;
    }
}

/// A listed path found (S17, ADD-A128): the pending fact `id` of `key`
/// applies where the preset leaves the key, or the set-whole block it is a
/// leaf of, to its synthetic value; an input's at the scheme `scheme`
/// (ADD-A132).
fn found_listed(
    report: &mut Report,
    preset: &str,
    key: &KeyFacts<'_>,
    scheme: Option<&str>,
    id: &str,
) {
    let name = key
        .set_whole
        .as_ref()
        .map_or(key.name.as_str(), |block| block.block.as_str());
    let name = match scheme {
        Some(scheme) => format!("{name} {scheme}"),
        None => name.to_owned(),
    };
    let entry = report
        .listed
        .entry(format!("{preset} {name}"))
        .or_insert_with(|| (BTreeSet::new(), 0));
    entry.0.insert(id.to_owned());
    entry.1 += 1;
}

/// What the probe of one combination found: skipped (a context or a
/// condition the gate cannot set), a failure that ends it (without its
/// pass's label), named by `not_drawn`, or its signature with the failures
/// its facts' order gives.
enum Probe {
    Skipped,
    Failed(String),
    NotDrawn,
    Queried {
        failures: Vec<String>,
        signature: Signature,
    },
}

/// A combination's element, which of its widget's steps apply with each
/// part's answer, and each key's applying fact ([`signature`]): two
/// combinations with the same signature resolve every key alike, so the
/// gate compares one of them (S17: "reusing what the set's rules share").
type Signature = (String, Vec<u32>, Vec<u32>);

/// One pass's probes of a condition vector's combinations
/// ([`check_widgets`]), what the perturbed pass replays ([`replay`]).
#[derive(Default)]
struct Probed {
    /// The failures that ended a probe, without the pass's label, each
    /// once, in the order first found.
    failures: Vec<String>,
    /// Each of `failures` by a hash of its text ([`text_hash`]).
    seen: HashMap<u64, Vec<usize>>,
    /// The combinations skipped, those the enumeration left out among
    /// them.
    skipped: u64,
    /// The distinct combinations, the first of each signature, in order,
    /// each held by the places of its names in `names`
    /// ([`Names::held`]): a vector holds hundreds of thousands of them.
    queried: Vec<Box<[usize]>>,
    /// The names and contexts `queried` holds.
    names: Names,
    /// Each element and the steps that apply to it with each part's
    /// answer among the signatures, held once: a vector's signatures share
    /// far fewer of them than they are.
    alike: Interned<(String, Vec<u32>)>,
    /// Each key's applying fact among the signatures ([`signature`]), held
    /// once.
    facts: Interned<Vec<u32>>,
    /// Each distinct signature, its element and steps by their place in
    /// `alike` and its facts by theirs in `facts`, by its combination's
    /// place in `queried`.
    signatures: HashMap<(usize, usize), usize>,
}

impl Probed {
    /// The place in `alike` of `element` and `steps`, where it holds them.
    fn alike_at(&self, element: &str, steps: &[u32]) -> Option<usize> {
        self.alike
            .find(alike_hash(element, steps), |(held, held_steps)| {
                held == element && held_steps == steps
            })
    }

    /// The place in `queried` of the distinct combination of `signature`,
    /// where it holds one.
    fn place_of(&self, signature: &Signature) -> Option<usize> {
        let (element, steps, facts) = signature;
        let alike = self.alike_at(element, steps)?;
        let facts = self.facts.find(facts_hash(facts), |held| held == facts)?;
        self.signatures.get(&(alike, facts)).copied()
    }

    /// `combination` held as the distinct one of `signature` where none is
    /// held yet; given back where it would not be given back as it is
    /// ([`Names::combination`]), so it is not held.
    fn insert(
        &mut self,
        signature: Signature,
        combination: Combination,
    ) -> Result<(), Box<Combination>> {
        let (element, steps, facts) = signature;
        let alike = self
            .alike
            .place(alike_hash(&element, &steps), (element, steps));
        let facts = self.facts.place(facts_hash(&facts), facts);
        let next = self.queried.len();
        let std::collections::hash_map::Entry::Vacant(vacant) =
            self.signatures.entry((alike, facts))
        else {
            return Ok(());
        };
        let held = self.names.held(combination.clone());
        if self.names.combination(&held).as_ref() != Some(&combination) {
            return Err(Box::new(combination));
        }
        vacant.insert(next);
        self.queried.push(held);
        Ok(())
    }

    /// The distinct combination `held` holds ([`Names::combination`]).
    fn combination(&self, held: &[usize]) -> Option<Combination> {
        self.names.combination(held)
    }

    /// `failure` found, held where it is not already.
    fn failed(&mut self, failure: String) {
        let hash = text_hash(&failure);
        let places = self.seen.entry(hash).or_default();
        if places
            .iter()
            .any(|at| self.failures.get(*at) == Some(&failure))
        {
            return;
        }
        places.push(self.failures.len());
        self.failures.push(failure);
    }

    /// What it holds, in counts and bytes ([`super::timing::sizes`]).
    fn sizes(&self) -> Vec<(&'static str, usize)> {
        let word = std::mem::size_of::<usize>();
        let queried: usize = self
            .queried
            .iter()
            .map(|held| (held.len() + 2) * word)
            .sum();
        let (mut element, mut steps) = (0, 0);
        for (held, held_steps) in &self.alike.values {
            element += held.capacity();
            steps += held_steps.capacity() * 4;
        }
        let facts: usize = self
            .facts
            .values
            .iter()
            .map(|held| held.capacity() * 4)
            .sum();
        vec![
            ("queried", self.queried.len()),
            ("queried bytes", queried),
            ("names", self.names.names.len()),
            ("alike", self.alike.values.len()),
            ("alike elements bytes", element),
            ("alike steps bytes", steps),
            ("facts", self.facts.values.len()),
            ("facts bytes", facts),
            (
                "signatures table bytes",
                self.signatures.capacity() * (std::mem::size_of::<((usize, usize), usize)>() + 1),
            ),
            ("failures", self.failures.len()),
        ]
    }
}

/// Values held once each, by their place, found by a hash of each.
struct Interned<T> {
    values: Vec<T>,
    by_hash: HashMap<u64, Vec<usize>>,
}

impl<T> Default for Interned<T> {
    fn default() -> Self {
        Interned {
            values: Vec::new(),
            by_hash: HashMap::new(),
        }
    }
}

impl<T> Interned<T> {
    /// The place of the value of the hash `hash` that `is` names.
    fn find(&self, hash: u64, is: impl Fn(&T) -> bool) -> Option<usize> {
        self.by_hash
            .get(&hash)?
            .iter()
            .copied()
            .find(|at| self.values.get(*at).is_some_and(&is))
    }

    /// The place of `value`, of the hash `hash`, held where it is not yet.
    fn place(&mut self, hash: u64, value: T) -> usize
    where
        T: PartialEq,
    {
        if let Some(at) = self.find(hash, |held| *held == value) {
            return at;
        }
        let at = self.values.len();
        self.by_hash.entry(hash).or_default().push(at);
        self.values.push(value);
        at
    }
}

/// A hash of the facts that apply to a combination's keys ([`Probed`]).
fn facts_hash(facts: &[u32]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    facts.hash(&mut hasher);
    hasher.finish()
}

/// The names and contexts of the distinct combinations [`Probed`] holds,
/// each once, so a combination is held by their places ([`Names::held`]).
#[derive(Default)]
struct Names {
    names: Vec<String>,
    by_name: HashMap<String, usize>,
    contexts: Vec<Value>,
}

impl Names {
    /// The place of `name`, held where it is not yet.
    fn name(&mut self, name: String) -> usize {
        if let Some(at) = self.by_name.get(&name) {
            return *at;
        }
        let at = self.names.len();
        self.names.push(name.clone());
        self.by_name.insert(name, at);
        at
    }

    /// `combination` by the places of its names: its widget; its
    /// variants, states and order, each list after its length; its size,
    /// density and direction, each `0` where it has none and its place
    /// after one otherwise; its context so among `contexts`; its siblings
    /// before and after it.
    fn held(&mut self, combination: Combination) -> Box<[usize]> {
        let Combination {
            widget,
            variants,
            states,
            order,
            size,
            density,
            direction,
            context,
            before,
            after,
            ..
        } = combination;
        let mut out = vec![self.name(widget)];
        for list in [variants, states, order] {
            out.push(list.len());
            for name in list {
                out.push(self.name(name));
            }
        }
        for name in [size, density, direction] {
            out.push(name.map_or(0, |name| self.name(name).saturating_add(1)));
        }
        out.push(context.map_or(0, |context| {
            let at = match self.contexts.iter().position(|held| *held == context) {
                Some(at) => at,
                None => {
                    self.contexts.push(context);
                    self.contexts.len().saturating_sub(1)
                }
            };
            at.saturating_add(1)
        }));
        out.push(before);
        out.push(after);
        out.into_boxed_slice()
    }

    /// The combination [`Names::held`] gave `held`; `None` where it names
    /// a place no name holds.
    fn combination(&self, held: &[usize]) -> Option<Combination> {
        let mut places = held.iter().copied();
        let name = |at: usize| self.names.get(at).cloned();
        let mut combination = Combination::new(&name(places.next()?)?);
        let list = |places: &mut dyn Iterator<Item = usize>| -> Option<Vec<String>> {
            let count = places.next()?;
            (0..count).map(|_| name(places.next()?)).collect()
        };
        combination.variants = list(&mut places)?;
        combination.states = list(&mut places)?;
        combination.order = list(&mut places)?;
        let optional = |places: &mut dyn Iterator<Item = usize>| -> Option<Option<String>> {
            match places.next()? {
                0 => Some(None),
                at => name(at.checked_sub(1)?).map(Some),
            }
        };
        combination.size = optional(&mut places)?;
        combination.density = optional(&mut places)?;
        combination.direction = optional(&mut places)?;
        combination.context = match places.next()? {
            0 => None,
            at => Some(self.contexts.get(at.checked_sub(1)?)?.clone()),
        };
        combination.before = places.next()?;
        combination.after = places.next()?;
        places.next().is_none().then_some(combination)
    }
}

/// How many combinations [`check_widgets`] probes, or compares, at once
/// for each thread it runs on.
const PROBED_BY_THREAD: usize = 1 << 11;

/// A widget's keys, in order, and each of their facts' `when` with its
/// key, in order, each key's from its place in `starts`: what every
/// combination of the widget asks, gathered once. Facts whose `when` is
/// written alike and whose keys name the same part (the widget's values
/// one) share a class in `classes`: whether the `when` holds is the same
/// for all of them, as [`Session::holds_each`] matches each `when` against
/// its key's part's element.
///
/// [`Session::holds_each`]: native_theme::gate::Session::holds_each
#[derive(Default)]
struct WidgetKeys<'k, 'f> {
    keys: Vec<&'k KeyFacts<'f>>,
    whens: Vec<(&'k When, &'k FactKey)>,
    starts: Vec<usize>,
    classes: Vec<usize>,
}

/// Each widget's keys among `keys`, by the widget.
fn widget_keys<'k, 'f>(keys: &'k [KeyFacts<'f>]) -> HashMap<&'k str, WidgetKeys<'k, 'f>> {
    let mut out: HashMap<&str, WidgetKeys<'_, '_>> = HashMap::new();
    let mut classes: HashMap<(&str, String, &str), usize> = HashMap::new();
    for key in keys {
        let Some(widget) = key.widget.as_deref() else {
            continue;
        };
        // The key's part, or `values`: what follows its widget (S17's key,
        // `parse_key`); a key spelled with a quoted segment its own.
        let part = if key.name.contains(['"', '\'']) {
            key.name.as_str()
        } else {
            key.name.split('.').nth(1).unwrap_or_default()
        };
        let entry = out.entry(widget).or_default();
        entry.keys.push(key);
        entry.starts.push(entry.whens.len());
        for fact in &key.facts {
            let written = fact
                .written
                .as_ref()
                .map(Value::to_string)
                .unwrap_or_default();
            let next = classes.len();
            let class = *classes.entry((widget, written, part)).or_insert(next);
            entry.whens.push((&fact.when, &key.key));
            entry.classes.push(class);
        }
    }
    out
}

/// The `when`s of a widget's facts ([`WidgetKeys`]) whose conditions hold
/// in a resolution, one of each class, with its key; and each of those
/// facts by its place among the widget's, with its class's place among
/// them. Those whose conditions do not hold hold for none of its
/// combinations.
struct Holding<'k> {
    whens: Vec<(&'k When, &'k FactKey)>,
    at: Vec<(usize, usize)>,
}

/// Each widget's [`Holding`] of `widgets` in `resolved`.
fn holding<'k>(
    widgets: &HashMap<&'k str, WidgetKeys<'k, '_>>,
    resolved: &Resolved,
) -> HashMap<&'k str, Holding<'k>> {
    widgets
        .iter()
        .map(|(name, widget)| {
            let mut holding = Holding {
                whens: Vec::new(),
                at: Vec::new(),
            };
            let mut of_class: HashMap<usize, usize> = HashMap::new();
            for (at, (held, class)) in widget.whens.iter().zip(&widget.classes).enumerate() {
                if resolved.conditions_hold(held.0) {
                    let next = holding.whens.len();
                    let place = *of_class.entry(*class).or_insert(next);
                    if place == next {
                        holding.whens.push(*held);
                    }
                    holding.at.push((at, place));
                }
            }
            (*name, holding)
        })
        .collect()
}

/// A widget's keys for a combination, with the fact that applies to each,
/// by its place among the key's facts, and whether each of its facts'
/// `when` holds.
struct Applying<'w, 'f> {
    widget: &'w WidgetKeys<'w, 'f>,
    held: Vec<bool>,
    at: Vec<Option<usize>>,
}

impl<'w, 'f> Applying<'w, 'f> {
    /// Whether each of the facts of the key at `index` holds.
    fn held_of(&self, index: usize) -> &[bool] {
        let (Some(key), Some(start)) = (
            self.widget.keys.get(index),
            self.widget.starts.get(index).copied(),
        ) else {
            return &[];
        };
        self.held
            .get(start..start.saturating_add(key.facts.len()))
            .unwrap_or_default()
    }

    /// The fact that applies to each key.
    fn facts(&self) -> Vec<Option<&'w Prepared<'f>>> {
        self.widget
            .keys
            .iter()
            .zip(&self.at)
            .map(|(key, at)| at.and_then(|at| key.facts.get(at)))
            .collect()
    }
}

/// The facts that apply to the keys `widget` of `session`'s combination's
/// widget, those whose conditions hold in its resolution `holding`, each
/// matched against the combination's element; a pending fact a later one
/// shadows is a failure in `report`.
fn applying_in<'w, 'f>(
    widget: &'w WidgetKeys<'w, 'f>,
    holding: &Holding<'_>,
    session: &native_theme::gate::Session<'_>,
    report: &mut Report,
    describe: &dyn Fn() -> String,
) -> Applying<'w, 'f> {
    applying_from(
        widget,
        holding,
        &session.holds_each(&holding.whens),
        report,
        describe,
    )
}

/// [`applying_in`], whether each of `holding`'s `when`s holds given in
/// `holds`.
fn applying_from<'w, 'f>(
    widget: &'w WidgetKeys<'w, 'f>,
    holding: &Holding<'_>,
    holds: &[bool],
    report: &mut Report,
    describe: &dyn Fn() -> String,
) -> Applying<'w, 'f> {
    let mut held = vec![false; widget.whens.len()];
    for (at, place) in &holding.at {
        if let (Some(slot), Some(holds)) = (held.get_mut(*at), holds.get(*place)) {
            *slot = *holds;
        }
    }
    let mut applying = Applying {
        widget,
        held,
        at: Vec::with_capacity(widget.keys.len()),
    };
    for (index, key) in widget.keys.iter().enumerate() {
        let at = applying_at(&key.facts, applying.held_of(index), report, describe);
        applying.at.push(at);
    }
    applying
}

/// The environment variable that sets how many threads the gate's parallel
/// steps use (the module's docs of `facts_gate`).
pub const THREADS_VARIABLE: &str = "NT_FACTS_GATE_THREADS";

/// The threads the gate's parallel steps use ([`thread_count`] of
/// [`THREADS_VARIABLE`]); a value it refuses, or one that is not Unicode,
/// fails.
fn threads() -> usize {
    let value = match std::env::var(THREADS_VARIABLE) {
        Ok(value) => Some(value),
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(value)) => {
            panic!("{THREADS_VARIABLE} is not Unicode: {value:?}")
        }
    };
    thread_count(value.as_deref()).unwrap_or_else(|error| panic!("{THREADS_VARIABLE}: {error}"))
}

/// The threads `value`, [`THREADS_VARIABLE`]'s value, sets: its count, or
/// the machine's parallelism where it is `None`; a value that is no number
/// above zero is an error, never a silently different run. The count sets
/// the speed and the memory: as many condition vectors are checked at once
/// as there are threads ([`check`]), each holding its two resolutions, a
/// chunk of its combinations, the distinct ones it compares with their
/// signatures ([`Probed`]) and the answers its memo shares ([`Memo`]); and
/// a resolution takes the most while it is built. On 16 threads
/// (2026-11-09) adwaita, kde-breeze, windows-11, material-3 and
/// macos-monterey peak at 1.1 to 2.3 GB; macos-tahoe's first 16 vectors,
/// resolving at once, pass 12 GB within seconds (about 0.9 GB a
/// resolution being built): the default, every thread the machine has,
/// holds macos-tahoe under 12 GB only where it is 8 or fewer (7 to 8.5 GB
/// on 8).
pub fn thread_count(value: Option<&str>) -> Result<usize, String> {
    match value {
        None => Ok(std::thread::available_parallelism().map_or(1, std::num::NonZero::get)),
        Some(value) => value
            .trim()
            .parse::<usize>()
            .ok()
            .filter(|count| *count > 0)
            .ok_or_else(|| format!("`{value}` is no number above zero")),
    }
}

/// The environment variable that limits a run to one shard of the gate's
/// work, `k/N` (the module's docs of `facts_gate`).
pub const SHARD_VARIABLE: &str = "NT_FACTS_GATE_SHARD";

/// One of the parts the gate's work is split into ([`run_shard`]): the
/// condition vectors of every preset's normal and perturbed runs, each
/// vector checked by one shard alone, and the rest of the run, which every
/// shard does as far as the vectors need it and the first alone records:
/// the facts read, each preset prepared, its declarations run and its
/// derivations. A preset's listed paths, which the combinations of all its
/// vectors find, are checked by no shard: [`listed_paths`] over the shards'
/// reports merged checks them, and those reports then hold the whole run's
/// findings and counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shard {
    /// Its place among the shards, from 0.
    pub index: usize,
    /// How many shards the work is split into.
    pub count: usize,
}

impl Shard {
    /// The whole run, one shard.
    pub const WHOLE: Shard = Shard { index: 0, count: 1 };

    /// `report` where this shard records the work every shard does, the
    /// first; else `elsewhere`, which nothing reads.
    fn shared<'r>(self, report: &'r mut Report, elsewhere: &'r mut Report) -> &'r mut Report {
        if self.index == 0 { report } else { elsewhere }
    }
}

/// The shard [`SHARD_VARIABLE`]'s value names ([`shard_of`]); a value it
/// refuses, or one that is not Unicode, fails.
pub fn shard() -> Shard {
    let value = match std::env::var(SHARD_VARIABLE) {
        Ok(value) => Some(value),
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(value)) => {
            panic!("{SHARD_VARIABLE} is not Unicode: {value:?}")
        }
    };
    shard_of(value.as_deref()).unwrap_or_else(|error| panic!("{SHARD_VARIABLE}: {error}"))
}

/// The shard `value`, [`SHARD_VARIABLE`]'s value, names: `k/N`, the `k`th
/// of `N` shards, `1 ≤ k ≤ N`; the whole run where it is `None`. Any other
/// value is an error, never a silently different run.
pub fn shard_of(value: Option<&str>) -> Result<Shard, String> {
    let Some(value) = value else {
        return Ok(Shard::WHOLE);
    };
    let number = |part: &str| part.trim().parse::<usize>().ok();
    value
        .split_once('/')
        .and_then(|(k, n)| Some((number(k)?, number(n)?)))
        .filter(|(k, n)| (1..=*n).contains(k))
        .map(|(k, n)| Shard {
            index: k - 1,
            count: n,
        })
        .ok_or_else(|| format!("`{value}` is no shard: `k/N`, with 1 ≤ k ≤ N"))
}

/// The condition vectors' split among the shards ([`Shard`]), one
/// preset's and flavour's at a time: each vector, the most combinations
/// first, given to the shard with the fewest combinations of them so far,
/// of those that tie the one next in turn, the turn going on from one
/// preset to the next. A vector's combinations cost about the same as its
/// preset's others', far from another preset's, so each preset's vectors
/// are spread evenly; every shard decides the same split.
struct Split {
    shard: Shard,
    turn: usize,
}

impl Split {
    fn new(shard: Shard) -> Split {
        Split { shard, turn: 0 }
    }

    /// Of the vectors `weights` gives, in turn (each one's place and its
    /// combinations), those this shard checks, in that order.
    fn take(&mut self, weights: &[(usize, u64)]) -> Vec<usize> {
        let count = self.shard.count.max(1);
        let mut loads = vec![0_u64; count];
        let mut mine = Vec::new();
        for (at, weight) in weights {
            let after_turn = |shard: usize| (shard + count - self.turn % count) % count;
            let least = (0..count)
                .min_by_key(|shard| (loads.get(*shard).copied().unwrap_or(0), after_turn(*shard)))
                .unwrap_or(0);
            if let Some(load) = loads.get_mut(least) {
                *load = load.saturating_add(*weight);
            }
            self.turn = (least + 1) % count;
            if least == self.shard.index {
                mine.push(*at);
            }
        }
        mine
    }
}

/// `f` over each of `items` on [`threads`] threads, the results in the
/// items' order.
pub fn parallel<T: Sync, R: Send>(items: &[T], f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    parallel_on(threads(), items, f)
}

/// `f` over each of `items` on `threads` threads, the results in the
/// items' order; on the calling thread alone where `threads` is one.
fn parallel_on<T: Sync, R: Send>(
    threads: usize,
    items: &[T],
    f: impl Fn(&T) -> R + Sync,
) -> Vec<R> {
    if threads <= 1 {
        return items.iter().map(f).collect();
    }
    let next = std::sync::atomic::AtomicUsize::new(0);
    let mut done: Vec<(usize, R)> = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..threads.min(items.len()))
            .map(|_| {
                scope.spawn(|| {
                    let mut out = Vec::new();
                    loop {
                        let index = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some(item) = items.get(index) else {
                            break;
                        };
                        out.push((index, f(item)));
                    }
                    out
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().unwrap())
            .collect()
    });
    done.sort_by_key(|(index, _)| *index);
    done.into_iter().map(|(_, result)| result).collect()
}

/// A condition vector's combinations, which `combinations` gives in turn,
/// probed, each one's parts queried as `query` says: the failures that end
/// a probe and those its facts' order gives recorded in the combinations'
/// order, and the distinct ones kept, the first of each signature, for
/// [`compare_widgets`] to compare and the perturbed pass to replay
/// ([`replay`]); a combination a `not_drawn` entry names, as the query
/// matches it, is not queried (S17). The combinations are probed on the
/// vector's share of the threads ([`Threads`]).
fn check_widgets(
    run: &Run<'_, '_>,
    combinations: &Combinations<'_>,
    query: Query,
    report: &mut Report,
) -> Probed {
    let (resolved, vector, pass) = (run.resolved, run.vector, run.pass);
    let metrics = StubMetrics;
    let probe = |combination: &Combination| {
        let describe = || format!("{vector:?} {combination:?}");
        // A combination a `not_drawn` entry names is not the state
        // model's (S17), its parts never queried.
        let not_drawn = run
            .not_drawn
            .get(&combination.widget)
            .map_or(&[][..], Vec::as_slice);
        let session = match timed(Step::Session, || {
            resolved.session_drawn(combination, run.root, &metrics, not_drawn, query)
        }) {
            Ok(Some(session)) => session,
            Ok(None) => return Probe::NotDrawn,
            Err(Unread::NotHeld(_)) => return Probe::Skipped,
            Err(Unread::Refused(why)) => {
                return Probe::Failed(format!("{why} under {}", describe()));
            }
        };
        if let Some(why) = session.unavailable() {
            return Probe::Failed(format!("not available ({why}) under {}", describe()));
        }
        let mut probe = Report::default();
        let signature = match query {
            Query::Styles => {
                let applying = timed(Step::Applying, || {
                    run.applying(&combination.widget, &session, &mut probe, &describe)
                });
                timed(Step::Signature, || {
                    signature(combination, &session, &applying)
                })
            }
            Query::Shapes => {
                let (applying, steps) = timed(Step::Applying, || {
                    run.probed(&combination.widget, &session, &mut probe, &describe)
                });
                signature_of(combination, steps, &applying)
            }
        };
        Probe::Queried {
            failures: probe.failures,
            signature,
        }
    };
    // Probed in chunks as they are given, each chunk's findings recorded in
    // order before the next is probed, so only a chunk's combinations and
    // probes are held at once, and the distinct ones kept (the signatures
    // span every chunk).
    let mut probed = Probed::default();
    let mut probe_chunk = |chunk: &mut Vec<Combination>, report: &mut Report| {
        let probes = parallel_on(run.threads.each(), chunk, |combination| {
            timed(Step::Probe, || probe(combination))
        });
        for (combination, probe) in chunk.drain(..).zip(probes) {
            match probe {
                Probe::Skipped => {
                    report.combinations_skipped += 1;
                    probed.skipped += 1;
                }
                Probe::Failed(failure) => {
                    report.fail(format!("{pass:?}: {failure}"));
                    probed.failed(failure);
                }
                Probe::NotDrawn => {}
                Probe::Queried {
                    failures,
                    signature,
                } => {
                    for failure in failures {
                        report.fail(failure);
                    }
                    if let Err(combination) = probed.insert(signature, combination) {
                        report.fail(format!(
                            "{pass:?}: a distinct combination is not held as it was probed \
                             under {vector:?} {combination:?}"
                        ));
                    }
                }
            }
        }
    };
    let mut chunk: Vec<Combination> = Vec::new();
    let left_out = combinations(&mut |combination| {
        chunk.push(combination);
        if chunk.len() >= PROBED_BY_THREAD.saturating_mul(run.threads.each()) {
            probe_chunk(&mut chunk, report);
        }
    });
    probe_chunk(&mut chunk, report);
    report.combinations_skipped += left_out;
    probed.skipped += left_out;
    probed
}

/// The distinct combinations `probed` holds compared in `run`, in chunks,
/// each chunk's findings merged into `report` in order before the next is
/// compared; where `alike`, each first asked whether it probes in `run`
/// as `probed` holds it ([`compare_widget`]). `false` where one does not,
/// its chunk's findings and those after it left out.
fn compare_widgets(run: &Run<'_, '_>, probed: &Probed, alike: bool, report: &mut Report) -> bool {
    let threads = run.threads.each();
    let size = PROBED_BY_THREAD.saturating_mul(threads);
    let memo = Memo::of_probed(probed);
    for (chunk_at, chunk) in probed.queried.chunks(size).enumerate() {
        let items: Vec<(usize, &[usize])> = chunk
            .iter()
            .enumerate()
            .map(|(at, held)| (chunk_at.saturating_mul(size).saturating_add(at), &**held))
            .collect();
        let found = parallel_on(threads, &items, |(at, held)| {
            let mut report = Report::default();
            let Some(combination) = probed.combination(held) else {
                report.fail(format!(
                    "{:?}: the distinct combination at {at} under {:?} names a name it does \
                     not hold",
                    run.pass, run.vector
                ));
                return (true, report);
            };
            let expected = alike.then_some((probed, *at));
            let compared = timed(Step::Compare, || {
                compare_widget(run, &combination, expected, &memo, &mut report)
            });
            (compared, report)
        });
        if found.iter().any(|(compared, _)| !compared) {
            return false;
        }
        memo.compared(chunk_at.saturating_add(1).saturating_mul(size));
        for (_, found) in found {
            report.merge(found);
        }
    }
    if super::timing::measuring() {
        super::timing::sizes(&format!("{:?} {:?}", run.pass, run.vector), &memo.sizes());
    }
    true
}

/// The perturbed pass's widget checks in `run` from the normal pass's
/// probes `probed` (S17: "reusing what the set's rules share"): its
/// failures recorded again under the pass's label, its skipped
/// combinations counted again, its distinct combinations compared in
/// `run`'s resolution. The probes read the same in both resolutions
/// ([`replays`]), so the perturbed pass's own probes find the same
/// failures, skipped combinations and distinct ones in the same order; the
/// failures a pass's label does not name are the normal pass's own, held
/// in `report` already. Where a distinct combination does not probe in
/// `run` as `probed` holds it, `run`'s combinations are probed and
/// compared as the normal pass's are ([`check_widgets`]).
fn replay(
    run: &Run<'_, '_>,
    probed: &Probed,
    combinations: &Combinations<'_>,
    report: &mut Report,
) {
    let mut replayed = Report::default();
    for failure in &probed.failures {
        replayed.fail(format!("{:?}: {failure}", run.pass));
    }
    replayed.combinations_skipped += probed.skipped;
    if compare_widgets(run, probed, true, &mut replayed) {
        super::timing::note(Step::Replayed);
        report.merge(replayed);
    } else {
        super::timing::note(Step::Differed);
        widgets_checked(run, combinations, report);
    }
}

/// [`check_widgets`] in `run`, each combination's parts queried for their
/// shapes alone ([`Query::Shapes`]), which is all a probe reads, and each
/// distinct combination first asked whether it probes with its parts'
/// styles queried as it did so ([`compare_widgets`]): two combinations
/// with the same signature resolve every key alike, so a style resolving
/// for one resolves for the other. Where one does not probe so, the pass's
/// findings are left out and its combinations probed with their parts'
/// styles queried ([`Query::Styles`]).
fn widgets_checked(
    run: &Run<'_, '_>,
    combinations: &Combinations<'_>,
    report: &mut Report,
) -> Probed {
    let mut shaped = Report::default();
    let probed = check_widgets(run, combinations, Query::Shapes, &mut shaped);
    if compare_widgets(run, &probed, true, &mut shaped) {
        report.merge(shaped);
        probed
    } else {
        super::timing::note(Step::Unshaped);
        let probed = check_widgets(run, combinations, Query::Styles, report);
        compare_widgets(run, &probed, false, report);
        probed
    }
}

/// Whether the perturbed pass's widget checks in `perturbed` replay the
/// normal pass's in `normal` ([`replay`]): every combination's probe reads
/// the two alike ([`Resolved::probes_alike`]), and the facts that hold in
/// each, `normal_holding` and `holding`, are the same.
fn replays(
    normal: &Resolved,
    perturbed: &Resolved,
    root: &RootInputs,
    normal_holding: &HashMap<&str, Holding<'_>>,
    holding: &HashMap<&str, Holding<'_>>,
) -> bool {
    normal_holding.len() == holding.len()
        && normal_holding.iter().all(|(widget, normal)| {
            holding
                .get(widget)
                .is_some_and(|holding| holding.at == normal.at)
        })
        && normal.probes_alike(perturbed, root)
}

/// A combination's signature ([`Probe::Queried`]): its element, which of
/// its widget's steps apply to it with each part's answer
/// ([`Session::signature`]), and each key's applying fact by its place
/// among the key's facts, the widget's keys always in one order.
///
/// [`Session::signature`]: native_theme::gate::Session::signature
fn signature(
    combination: &Combination,
    session: &native_theme::gate::Session<'_>,
    applying: &Applying<'_, '_>,
) -> Signature {
    signature_of(combination, session.signature(), applying)
}

/// [`signature`], the steps that apply with each part's answer given in
/// `steps`.
fn signature_of(
    combination: &Combination,
    steps: Vec<u32>,
    applying: &Applying<'_, '_>,
) -> Signature {
    let facts: Vec<u32> = applying
        .at
        .iter()
        .map(|at| at.and_then(|at| u32::try_from(at).ok()).unwrap_or(u32::MAX))
        .collect();
    let element = format!(
        "{} {:?} {:?}",
        combination.widget, combination.context, combination.direction
    );
    (element, steps, facts)
}

/// Every widget key of `combination`, queried, compared with the fact that
/// applies, what its session's queries answer read from `memo` where an
/// earlier distinct combination's session asked them; where `expected`
/// names it, `probed`'s distinct combination at its place, only once it
/// probes in `run` with the signature `probed` holds for it. `false` where
/// it does not, nothing compared.
fn compare_widget(
    run: &Run<'_, '_>,
    combination: &Combination,
    expected: Option<(&Probed, usize)>,
    memo: &Memo<'_>,
    report: &mut Report,
) -> bool {
    let (resolved, vector, pass) = (run.resolved, run.vector, run.pass);
    let metrics = StubMetrics;
    let describe = || format!("{vector:?} {combination:?}");
    let Ok(session) = resolved.session(combination, run.root, &metrics) else {
        return expected.is_none();
    };
    let mut probe = Report::default();
    let applying = run.applying(&combination.widget, &session, &mut probe, &describe);
    if let Some((probed, at)) = expected
        && probed.place_of(&signature(combination, &session, &applying)) != Some(at)
    {
        return false;
    }
    report.queried += 1;
    let asked = memo.of(combination, &session);
    let widget_keys = &applying.widget.keys;
    let facts = &applying.facts();
    for (index, (key, fact)) in widget_keys.iter().zip(facts.iter().copied()).enumerate() {
        // A later fact of a block holding the key states it (S17 *Which
        // fact applies*): the block's comparison is the key's.
        if blocks_of(widget_keys, index).into_iter().any(|block| {
            facts
                .get(block)
                .copied()
                .flatten()
                .is_some_and(|stated| later(stated, fact))
        }) {
            continue;
        }
        let place = fact.map(|fact| fact.fact.place()).unwrap_or_default();
        // A pending fact applying where the preset leaves its path to the
        // synthetic value is a listed path found; a field of a part not
        // drawn is none, as S12 judges a missing field only where its part
        // is drawn (S17 *Where a listed path is found*, ADD-A128,
        // ADD-A144).
        if pass == Pass::Normal
            && let Some(Prepared {
                fact:
                    Fact {
                        stated: Stated::Pending(id),
                        ..
                    },
                ..
            }) = fact
            && session.drawn(&key.key)
            && listed_or_failed(
                asked.answer(|| Asked::Listed(index), || session.listed(&key.key)),
                &key.name,
                report,
                &describe,
            )
        {
            found_listed(report, run.preset, key, None, id);
        }
        if !session.drawn(&key.key) {
            // A field of a part not drawn: no fact is asked for it and
            // none is compared, one that applies there no failure; the
            // part's own key states whether it is drawn (S17, ADD-A130).
            if pass == Pass::Normal && fact.is_some() {
                report.fields_not_drawn += 1;
            }
            continue;
        }
        let leaf = |name: &str| {
            widget_keys
                .iter()
                .enumerate()
                .find(|(_, key)| key.name == name)
                .map_or(LeafFact::NoKey, |(at, key)| {
                    leaf_of(key, applying.held_of(at))
                })
        };
        // A block no fact of its own states here is stated by its leaves'
        // facts that apply (S17 *Which fact applies*): each leaf's
        // comparison is the block's. A whole part with no fact is drawn,
        // which its own verdict compares.
        if fact.is_none()
            && !key.key.whole_part()
            && stated_by_leaves(widget_keys, facts, &key.name)
        {
            continue;
        }
        let whole = composed(key, fact, &leaf, pass, report, &describe);
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
                let except = stated_below(widget_keys, facts, index, fact);
                // A fact's own value is the same wherever it applies; a
                // value composed from leaves is read anew.
                let own = fact
                    .and_then(|fact| fact.value.as_ref())
                    .is_some_and(|own| std::ptr::eq(own, value));
                // A value composed from a set-whole block's leaves is the
                // block's, read at the block's place (a step's place is the
                // key the fact names, S17), and compared at the key's leaf
                // alone: the other leaves are the theme's, grafted in.
                let (read_at, except) = match key.set_whole.as_ref().filter(|_| !own) {
                    Some(block) => {
                        let leaf = key.key.below(block.key()).unwrap_or_default();
                        let mut grafted: Vec<Vec<String>> = except
                            .iter()
                            .map(|below| leaf.iter().chain(below).cloned().collect())
                            .collect();
                        grafted.extend(
                            block
                                .leaves
                                .iter()
                                .map(|(_, name, _)| vec![name.clone()])
                                .filter(|other| *other != leaf),
                        );
                        (block.key(), grafted)
                    }
                    None => (&key.key, except),
                };
                let at = std::ptr::from_ref(value).addr();
                let synthetic = || session.synthetic(read_at, Some(value), &except);
                let synthetic = if own {
                    asked.answer(|| Asked::Synthetic(index, at, except.clone()), synthetic)
                } else {
                    synthetic()
                };
                match synthetic {
                    Ok(true) => synthetic_skip(report, &place, &key.name, pass),
                    Ok(false) => {
                        let tolerance = tolerance.drawn_by(run.drawing);
                        let compare =
                            || session.compare_except(read_at, value, &tolerance, &except);
                        let outcome = if own {
                            asked.answer(|| Asked::Compared(index, at, except.clone()), compare)
                        } else {
                            compare()
                        };
                        record(outcome, &key.name, &place, report, &describe, pass);
                    }
                    Err(unread) => {
                        record(Err(unread), &key.name, &place, report, &describe, pass);
                    }
                }
            }
            Verdict::Drawn if pass == Pass::Normal => {
                let outcome = asked.answer(
                    || Asked::Drawn(index),
                    || session.compare(&key.key, None, &Tolerance::quote()),
                );
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
    true
}

/// What the sessions of a condition vector's distinct combinations
/// answered in one pass ([`compare_widgets`]), by their element and the
/// steps that apply to it with each part's answer
/// ([`Session::signature`]): two such sessions resolve every key alike
/// (S17: "reusing what the set's rules share"), so each question is asked
/// once for each, whichever facts apply to the other keys. Only the
/// sessions of an element and steps the pass's probes hold for more than
/// one distinct combination are held, each until the last of them is
/// compared ([`Memo::compared`]): a session no later one is alike with
/// asks each question once anyway.
///
/// [`Session::signature`]: native_theme::gate::Session::signature
struct Memo<'p> {
    /// The probes whose distinct combinations are compared.
    probed: &'p Probed,
    /// The place in `probed.queried` of the last distinct combination of
    /// each element and steps, by its place in `probed.alike`, that
    /// `probed` holds for more than one.
    last: HashMap<usize, usize>,
    sessions: Mutex<Sessions>,
}

/// The sessions a [`Memo`] holds, by their element and steps' place in
/// its probes' `alike`, and the most it held at once.
#[derive(Default)]
struct Sessions {
    held: HashMap<usize, Arc<Answers>>,
    most: usize,
}

/// A hash of an element and its steps, by which [`Probed`] finds them.
fn alike_hash(element: &str, steps: &[u32]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    element.hash(&mut hasher);
    steps.hash(&mut hasher);
    hasher.finish()
}

impl<'p> Memo<'p> {
    /// The memo of the distinct combinations `probed` holds.
    fn of_probed(probed: &'p Probed) -> Memo<'p> {
        let mut uses: HashMap<usize, (usize, usize)> = HashMap::new();
        for ((alike, _), at) in &probed.signatures {
            let (count, last) = uses.entry(*alike).or_insert((0, 0));
            *count += 1;
            *last = (*last).max(*at);
        }
        Memo {
            probed,
            last: uses
                .into_iter()
                .filter(|(_, (count, _))| *count > 1)
                .map(|(alike, (_, last))| (alike, last))
                .collect(),
            sessions: Mutex::default(),
        }
    }

    /// The answers of the sessions alike with `session`, `combination`'s;
    /// none held where no later distinct combination is alike with it.
    fn of(&self, combination: &Combination, session: &native_theme::gate::Session<'_>) -> Asking {
        if self.last.is_empty() {
            return Asking(None);
        }
        let element = format!(
            "{} {:?} {:?}",
            combination.widget, combination.context, combination.direction
        );
        let Some(alike) = self
            .probed
            .alike_at(&element, &session.signature())
            .filter(|alike| self.last.contains_key(alike))
        else {
            return Asking(None);
        };
        let mut sessions = self.sessions.lock().unwrap_or_else(PoisonError::into_inner);
        let answers = Arc::clone(sessions.held.entry(alike).or_default());
        sessions.most = sessions.most.max(sessions.held.len());
        Asking(Some(answers))
    }

    /// Every distinct combination before the place `before` compared: the
    /// sessions none after it is alike with let go.
    fn compared(&self, before: usize) {
        let mut sessions = self.sessions.lock().unwrap_or_else(PoisonError::into_inner);
        let last = &self.last;
        sessions
            .held
            .retain(|alike, _| last.get(alike).is_some_and(|last| *last >= before));
    }

    /// How many elements and steps it holds sessions for, and the most
    /// sessions it held at once ([`super::timing::sizes`]).
    fn sizes(&self) -> Vec<(&'static str, usize)> {
        let sessions = self.sessions.lock().unwrap_or_else(PoisonError::into_inner);
        vec![
            ("memo shared", self.last.len()),
            ("memo held", sessions.held.len()),
            ("memo most", sessions.most),
        ]
    }
}

/// What [`Memo::of`] gives a session: the answers of the sessions alike
/// with it, or none held, each question asked.
struct Asking(Option<Arc<Answers>>);

impl Asking {
    /// The answer to the question `asked` gives, `ask`ed where none is
    /// held.
    fn answer<A: Answer>(&self, asked: impl FnOnce() -> Asked, ask: impl FnOnce() -> A) -> A {
        match &self.0 {
            Some(answers) => answers.answer(asked(), ask),
            None => ask(),
        }
    }
}

/// A question [`compare_widget`] asks a session about the key at its
/// index among its widget's keys: whether it is a listed path; whether a
/// fact's own value (by its address) reaches a synthetic value there, and
/// its comparison, each but at the places below the key a later fact
/// states; the whole part's comparison with none.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Asked {
    Listed(usize),
    Synthetic(usize, usize, Vec<Vec<String>>),
    Compared(usize, usize, Vec<Vec<String>>),
    Drawn(usize),
}

/// What one session answered ([`Memo`]).
#[derive(Default)]
struct Answers {
    listed: Mutex<HashMap<Asked, Result<bool, Unread>>>,
    compared: Mutex<HashMap<Asked, Result<Outcome, Unread>>>,
}

/// An answer [`Answers`] holds, of its kind.
trait Answer: Clone {
    fn held(answers: &Answers) -> &Mutex<HashMap<Asked, Self>>;
}

impl Answer for Result<bool, Unread> {
    fn held(answers: &Answers) -> &Mutex<HashMap<Asked, Self>> {
        &answers.listed
    }
}

impl Answer for Result<Outcome, Unread> {
    fn held(answers: &Answers) -> &Mutex<HashMap<Asked, Self>> {
        &answers.compared
    }
}

impl Answers {
    /// The answer to `asked`, `ask`ed where none is held yet; asked
    /// outside the lock, so two threads may ask it at once, alike.
    fn answer<A: Answer>(&self, asked: Asked, ask: impl FnOnce() -> A) -> A {
        let lock = || A::held(self).lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(held) = lock().get(&asked) {
            return held.clone();
        }
        let answer = ask();
        lock().insert(asked, answer.clone());
        answer
    }
}

/// Whether a key below `block` among `keys` has a fact that applies, as
/// `facts` holds each key's.
fn stated_by_leaves(keys: &[&KeyFacts<'_>], facts: &[Option<&Prepared<'_>>], block: &str) -> bool {
    let below = format!("{block}.");
    keys.iter()
        .zip(facts)
        .any(|(key, fact)| fact.is_some() && key.name.starts_with(&below))
}

/// The keys of `keys` that are blocks holding the key at `index`: a part
/// field's or a widget value's key below which it lies (a whole part's key
/// is none: its fields' facts state its fields, its own whether it is
/// drawn, S17).
fn blocks_of(keys: &[&KeyFacts<'_>], index: usize) -> Vec<usize> {
    let Some(key) = keys.get(index) else {
        return Vec::new();
    };
    keys.iter()
        .enumerate()
        .filter(|(_, block)| key.key.below(&block.key).is_some())
        .map(|(at, _)| at)
        .collect()
}

/// Whether `fact` stands after `than` in their file (S17 *Which fact
/// applies*: the last in file order applies); a fact of another file is
/// in no order with it.
fn later(fact: &Prepared<'_>, than: Option<&Prepared<'_>>) -> bool {
    than.is_none_or(|than| fact.fact.file == than.fact.file && fact.fact.index > than.fact.index)
}

/// The places below the key at `index`, a block whose `fact` applies, that
/// a later fact of their own keys states, each by its path below the
/// block's.
fn stated_below(
    keys: &[&KeyFacts<'_>],
    facts: &[Option<&Prepared<'_>>],
    index: usize,
    fact: Option<&Prepared<'_>>,
) -> Vec<Vec<String>> {
    let Some(block) = keys.get(index) else {
        return Vec::new();
    };
    keys.iter()
        .zip(facts)
        .filter_map(|(key, stated)| {
            let below = key.key.below(&block.key)?;
            later((*stated)?, fact).then_some(below)
        })
        .collect()
}

/// One measured derivation's pass under one condition vector (S17 *A
/// measured derivation*): what its comparison reads, and the resolution
/// it is compared in, by its index among the distinct ones.
struct PassCheck<'k> {
    key: &'k KeyFacts<'k>,
    prepared: &'k Prepared<'k>,
    value: &'k FactValue,
    pass: &'k facts::DerivationPass,
    result: &'k FactValue,
    combination: Combination,
    not_drawn: &'k [When],
    vector: Vec<String>,
    resolution: usize,
    /// The pass's completions under the vector ([`complete`]), by index,
    /// and how many: an undrawn one of several fails only where every one
    /// of them is undrawn.
    group: (usize, usize),
}

/// A measured derivation's undrawn completions under a vector: how many,
/// of how many, and the first one's key, place, vector and refusal.
type Undrawn = (usize, usize, String, String, String, Unread);

/// Every measured derivation's passes (S17 *A measured derivation*): for
/// each pass, the preset resolved with the stand-in stating the varied
/// inputs under the fact's `when` and the pass's conditions, the fact's
/// expression evaluated in the key's place compared with the pass's result
/// within the citation's tolerance. Each distinct pair of a condition
/// vector and a stand-in is resolved once, the pairs in parallel, as many
/// passes vary an input to the same values under the same conditions.
fn derivations(
    platform: &str,
    preset: &Preset,
    flavor: Option<&str>,
    keys: &[KeyFacts<'_>],
    entries: &[Model],
    entry_not_drawn: &NotDrawn,
    report: &mut Report,
) {
    let root = root_inputs();
    let metrics = StubMetrics;
    let schemes = preset.schemes();
    let drawing = drawing(platform, flavor);
    let mut checks: Vec<PassCheck<'_>> = Vec::new();
    let mut groups = 0_usize;
    let mut resolutions: Vec<(Vec<String>, String)> = Vec::new();
    for key in keys {
        for prepared in &key.facts {
            let Some(value) = &prepared.value else {
                continue;
            };
            for (pass, result) in &prepared.passes {
                let describe =
                    || format!("the pass {:?} of {}", pass.inputs, prepared.fact.place());
                let empty = Value::Table(toml::Table::new());
                let when = prepared.written.clone().unwrap_or(empty);
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
                let completed = match entry {
                    Some(entry) => complete(combination, &when, entry),
                    None => vec![combination],
                };
                if completed.is_empty() {
                    report.fail(format!(
                        "{}: its `when` names no combination the state model draws",
                        describe()
                    ));
                    continue;
                }
                let not_drawn: &[When] = entry
                    .and_then(|entry| entry_not_drawn.get(&entry_id(entry)))
                    .map_or(&[], Vec::as_slice);
                let mut text = String::new();
                for (input, color) in &pass.inputs {
                    text.push_str(&format!(
                        "[inputs.{input}]\nlight = {color}\ndark = {color}\n"
                    ));
                }
                if let Err(error) = Layer::from_toml(&text) {
                    report.fail(format!("{}: its inputs do not read: {error}", describe()));
                    continue;
                }
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
                    let pair = (vector.clone(), text.clone());
                    let resolution = match resolutions.iter().position(|held| *held == pair) {
                        Some(at) => at,
                        None => {
                            resolutions.push(pair);
                            resolutions.len() - 1
                        }
                    };
                    for combination in &completed {
                        checks.push(PassCheck {
                            key,
                            prepared,
                            value,
                            pass,
                            result,
                            combination: combination.clone(),
                            not_drawn,
                            vector: vector.clone(),
                            resolution,
                            group: (groups, completed.len()),
                        });
                    }
                    groups += 1;
                }
            }
        }
    }
    let resolved = parallel(&resolutions, |(vector, text)| {
        let layer = Layer::from_toml(text).unwrap();
        conditions(vector, flavor)
            .map_err(|unread| format!("{unread:?}"))
            .and_then(|conditions| preset.resolve_pass(&conditions, &layer))
    });
    let mut undrawn: BTreeMap<usize, Undrawn> = BTreeMap::new();
    for check in checks {
        let PassCheck {
            key,
            prepared,
            value,
            pass,
            result,
            combination,
            not_drawn,
            vector,
            resolution,
            group,
        } = check;
        let describe = || format!("the pass {:?} of {}", pass.inputs, prepared.fact.place());
        let widget = key.key.widget();
        let resolved = match &resolved[resolution] {
            Ok(resolved) => resolved,
            Err(why) => {
                report.fail(format!(
                    "{} does not resolve under {vector:?}: {why}",
                    describe()
                ));
                continue;
            }
        };
        let synthetic = match &widget {
            None => resolved.global_synthetic(&key.key, Some(value), &root, &[]),
            Some(_) => resolved
                .session(&combination, &root, &metrics)
                .and_then(|session| session.synthetic(&key.key, Some(value), &[])),
        };
        if synthetic.unwrap_or(false) {
            synthetic_skip(report, &prepared.fact.place(), &key.name, Pass::Normal);
            continue;
        }
        let outcome = match &widget {
            None => resolved.compare_global_pass(
                &key.key,
                value,
                result,
                &root,
                &pass.tolerance.drawn_by(drawing),
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
                            &pass.tolerance.drawn_by(drawing),
                        ),
                        Err(why) => {
                            let refused = Unread::Refused(format!(
                                "the pass's combination {combination:?} is {why}"
                            ));
                            let (index, total) = group;
                            if total > 1 {
                                let held = undrawn.entry(index).or_insert((
                                    0,
                                    total,
                                    key.name.clone(),
                                    describe(),
                                    format!("{vector:?}"),
                                    refused,
                                ));
                                held.0 += 1;
                                continue;
                            }
                            Err(refused)
                        }
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
    // A pass none of whose completions is drawn holds in no combination the
    // platform draws: a finding of its fact (S17, ADD-A141).
    for (count, total, name, place, under, refused) in undrawn.into_values() {
        if count == total {
            record(
                Err(refused),
                &name,
                &place,
                report,
                &|| under.clone(),
                Pass::Measured,
            );
        }
    }
}
