//! S17's predicate of a fact's units and its later unit (ADD-A147), decided
//! exactly by names as `scripts/check_facts_trace.py`'s class `Depth`
//! decides them, so the gate and the scripts attribute every fact alike
//! (`NT_FACTS_GATE_UNITS` compares the two reports).
//!
//! A combination of a widget's state model in force (the default flavour's
//! entry, each flavour's beside it once the flavours unit is enabled) is a
//! variant set it draws, a state combination, a condition vector, a size and
//! a density in force, a context (none, or an `axes.contexts` entry whose
//! widgets are enabled) and a direction, less those a `not_drawn` entry
//! names. A fact holds where its `when` holds (S11) and its part is drawn
//! (the last presence fact holding there is not `absent` or a `drawn_as`, a
//! pending one counting as drawn). Holding is decided exactly: each
//! dimension is read projected on the names the `when`s read name, each
//! projection once, and every projected combination is tested.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use toml::{Table, Value};

use super::facts::{self, Stated};
use super::model::{self, Model, strings};
use super::run::Platform;

/// Where a fact holding in no combination even with every later unit and
/// every host context is reported: under no unit (S17).
pub const STRAY: &str = "stray";

/// The unit of a fact drawn only in a context naming no widget: the
/// identities, with their hosts.
const HOSTLESS: &str = "identities";

/// An element alone among its siblings: first, last, only and odd.
fn lone(key: &str, value: &Value) -> bool {
    match key {
        "first" | "last" | "only" => value.as_bool() == Some(true),
        "parity" => value.as_str() == Some("odd"),
        _ => false,
    }
}

type Names = BTreeSet<String>;

/// An ancestor as a context entry reads it: its widget, part, surface,
/// variants, states and sizes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Ancestor {
    widget: Option<String>,
    part: Option<String>,
    surface: Option<String>,
    variants: Names,
    states: Names,
    sizes: Names,
}

/// One combination (the module's documentation).
#[derive(Clone, Debug)]
struct Combo {
    conditions: Names,
    states: Names,
    variants: Names,
    size: Option<String>,
    density: Option<String>,
    rtl: bool,
    ancestors: Vec<Ancestor>,
    siblings: bool,
}

/// A `when` read once: per kind (conditions, states, variants, sizes,
/// densities) its positive and negative names, whether it holds nowhere
/// (a size class), its `latest` flags, its context entries and negative
/// ones, whether its position needs a sibling, its direction.
#[derive(Clone, Debug, Default)]
struct Match {
    lists: [(Names, Names); 5],
    never: bool,
    latest: Vec<(String, String)>,
    context: Vec<Table>,
    not_context: Vec<Table>,
    sibling: bool,
    rtl: bool,
}

/// The kinds a [`Match`] lists, in its order.
const KINDS: [&str; 5] = ["conditions", "states", "variants", "sizes", "densities"];

fn names(value: Option<&Value>) -> Names {
    strings(value).into_iter().collect()
}

fn tables(value: Option<&Value>) -> Vec<Table> {
    value
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_table).cloned().collect())
        .unwrap_or_default()
}

fn compiled(when: Option<&Value>) -> Match {
    let Some(when) = when.and_then(Value::as_table) else {
        return Match::default();
    };
    let lists = KINDS.map(|kind| {
        (
            names(when.get(kind)),
            names(when.get(&format!("not_{kind}"))),
        )
    });
    let size_class = when.get("size_class").and_then(Value::as_table);
    let never = size_class.is_some_and(|table| {
        ["horizontal", "vertical"]
            .iter()
            .any(|axis| !strings(table.get(*axis)).is_empty())
    });
    let sibling = when
        .get("position")
        .and_then(Value::as_table)
        .is_some_and(|position| {
            position.iter().any(|(key, value)| {
                key == "previous" || key == "next" || (key != "among" && !lone(key, value))
            })
        });
    let latest = when
        .get("latest")
        .and_then(Value::as_table)
        .map(|table| {
            table
                .iter()
                .filter_map(|(set, flag)| Some((set.clone(), flag.as_str()?.to_owned())))
                .collect()
        })
        .unwrap_or_default();
    Match {
        lists,
        never,
        latest,
        context: tables(when.get("context")),
        not_context: tables(when.get("not_context")),
        sibling,
        rtl: when.get("direction").and_then(Value::as_str) == Some("rtl"),
    }
}

/// check_facts.orders_possible: one order of the flags `held` makes each
/// `latest` flag its set's latest: each set after every other held member
/// of its set, with no cycle.
fn orders_possible(
    latest: &[(String, String)],
    held: &Names,
    recency: &[(String, Vec<String>)],
) -> bool {
    let mut after: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (set, flag) in latest {
        let members = recency
            .iter()
            .filter(|(name, _)| name == set)
            .flat_map(|(_, flags)| flags);
        let entry = after.entry(flag.as_str()).or_default();
        entry.extend(
            members
                .filter(|member| held.contains(*member) && *member != flag)
                .map(String::as_str),
        );
    }
    let mut done: BTreeSet<&str> = BTreeSet::new();
    fn acyclic<'a>(
        flag: &'a str,
        after: &BTreeMap<&'a str, BTreeSet<&'a str>>,
        done: &mut BTreeSet<&'a str>,
        active: &mut BTreeSet<&'a str>,
    ) -> bool {
        if done.contains(flag) {
            return true;
        }
        if !active.insert(flag) {
            return false;
        }
        let ok = after
            .get(flag)
            .is_none_or(|next| next.iter().all(|next| acyclic(next, after, done, active)));
        active.remove(flag);
        done.insert(flag);
        ok
    }
    let flags: Vec<&str> = after.keys().copied().collect();
    flags
        .into_iter()
        .all(|flag| acyclic(flag, &after, &mut done, &mut BTreeSet::new()))
}

fn set_match(entry: &Table, kind: &str, have: &Names) -> bool {
    names(entry.get(kind)).is_subset(have)
        && names(entry.get(&format!("not_{kind}"))).is_disjoint(have)
}

fn entry_holds(entry: &Table, ancestors: &[Ancestor]) -> bool {
    let matches = |ancestor: &Ancestor| {
        let fixed = [
            ("widget", &ancestor.widget),
            ("part", &ancestor.part),
            ("surface", &ancestor.surface),
        ];
        fixed.iter().all(|(key, held)| {
            entry
                .get(*key)
                .is_none_or(|value| value.as_str() == held.as_deref())
        }) && set_match(entry, "variants", &ancestor.variants)
            && set_match(entry, "states", &ancestor.states)
            && set_match(entry, "sizes", &ancestor.sizes)
    };
    match entry.get("scope").and_then(Value::as_table) {
        Some(scope) => {
            let nearest = names(scope.get("nearest"));
            ancestors
                .iter()
                .rev()
                .find(|ancestor| {
                    ancestor
                        .widget
                        .as_ref()
                        .is_some_and(|widget| nearest.contains(widget))
                })
                .is_some_and(matches)
        }
        None => ancestors.iter().any(matches),
    }
}

fn combo_holds(m: &Match, c: &Combo, recency: &[(String, Vec<String>)]) -> bool {
    if m.never {
        return false;
    }
    let one = |value: &Option<String>| value.iter().cloned().collect::<Names>();
    let haves = [
        c.conditions.clone(),
        c.states.clone(),
        c.variants.clone(),
        one(&c.size),
        one(&c.density),
    ];
    let lists_hold = m
        .lists
        .iter()
        .zip(&haves)
        .all(|((pos, neg), have)| pos.is_subset(have) && neg.is_disjoint(have));
    if !lists_hold {
        return false;
    }
    if !m.latest.is_empty()
        && !(m.latest.iter().all(|(_, flag)| c.states.contains(flag))
            && orders_possible(&m.latest, &c.states, recency))
    {
        return false;
    }
    if m.rtl != c.rtl || (m.sibling && !c.siblings) {
        return false;
    }
    m.context
        .iter()
        .all(|entry| entry_holds(entry, &c.ancestors))
        && !m
            .not_context
            .iter()
            .any(|entry| entry_holds(entry, &c.ancestors))
}

/// One entry in force's dimensions (check_facts_trace.py's `space`).
struct Space {
    recency: Vec<(String, Vec<String>)>,
    sets: Vec<(Names, Vec<Option<String>>)>,
    states: Vec<Names>,
    vectors: Vec<Names>,
    densities: Vec<Option<String>>,
    not_drawn: Vec<Match>,
    contexts: Vec<(Vec<Ancestor>, bool)>,
    rtl: Vec<bool>,
}

fn ancestors_of(context: &Value) -> Vec<Ancestor> {
    let chain: Vec<&Value> = match context {
        Value::Array(items) => items.iter().collect(),
        other => vec![other],
    };
    chain
        .into_iter()
        .filter_map(Value::as_table)
        .map(|entry| Ancestor {
            widget: entry
                .get("widget")
                .and_then(Value::as_str)
                .map(str::to_owned),
            part: entry.get("part").and_then(Value::as_str).map(str::to_owned),
            surface: entry
                .get("surface")
                .and_then(Value::as_str)
                .map(str::to_owned),
            variants: names(entry.get("variants")),
            states: names(entry.get("states")),
            sizes: names(entry.get("sizes")),
        })
        .collect()
}

/// A condition's kind, one value of which a combination holds at most.
fn condition_kind(condition: &str) -> String {
    if condition == "contrast_increased" || condition == "contrast_high" {
        return "contrast".to_owned();
    }
    match condition.split_once(':') {
        Some(("container_size", rest)) => format!(
            "container_size:{}",
            rest.split_once(':')
                .map_or(rest, |(container, _)| container)
        ),
        Some((kind, _)) => kind.to_owned(),
        None => condition.to_owned(),
    }
}

/// Which host contexts a space holds: those whose widgets are enabled, and
/// with them the one at that place.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Hosts {
    Held,
    At(usize),
}

/// The predicate over one platform's facts for one preset.
pub struct Depth<'p> {
    platform: &'p Platform,
    preset: String,
    spaces: RefCell<HashMap<(String, Vec<String>, Hosts), std::rc::Rc<Vec<Space>>>>,
}

impl<'p> Depth<'p> {
    pub fn new(platform: &'p Platform, preset: &str) -> Depth<'p> {
        Depth {
            platform,
            preset: preset.to_owned(),
            spaces: RefCell::new(HashMap::new()),
        }
    }

    /// The entries in force: the default flavour's, then each flavour's
    /// (an entry naming `flavor:<f>`) where the flavours unit is enabled or
    /// added and its condition is not held off.
    fn entries(&self, widget: &str, added: &[String]) -> Vec<(Option<String>, Model)> {
        let units = &self.platform.units;
        let mut out: Vec<(Option<String>, Model)> =
            model::in_force(&self.platform.models, &self.preset, None)
                .into_iter()
                .filter(|entry| entry.widget == widget && entry.available)
                .map(|entry| (None, entry.clone()))
                .collect();
        if units.flavours_off(added) {
            return out;
        }
        for entry in &self.platform.models {
            let Some(flavor) = entry
                .conditions
                .iter()
                .find(|condition| condition.starts_with("flavor:"))
            else {
                continue;
            };
            let for_preset = entry.conditions.iter().all(|condition| {
                condition
                    .strip_prefix("preset:")
                    .is_none_or(|id| id == self.preset)
            });
            if entry.widget == widget
                && entry.available
                && for_preset
                && !units.condition_off(flavor, widget, added)
            {
                out.push((Some(flavor.clone()), entry.clone()));
            }
        }
        out
    }

    fn space(&self, widget: &str, added: &[String], hosts: Hosts) -> std::rc::Rc<Vec<Space>> {
        let cache_key = (widget.to_owned(), added.to_vec(), hosts);
        if let Some(found) = self.spaces.borrow().get(&cache_key) {
            return std::rc::Rc::clone(found);
        }
        let units = &self.platform.units;
        let preset = self.preset.as_str();
        let sized = !units.sizes_off(added);
        let mut out = Vec::new();
        for (flavor, entry) in self.entries(widget, added) {
            let off: Vec<String> = entry
                .variants
                .iter()
                .filter(|variant| units.name_off(widget, preset, variant, true, added))
                .cloned()
                .collect();
            let restricted = entry.without(&off);
            let undrawn: Vec<&Table> = entry
                .not_drawn
                .iter()
                .filter(|table| {
                    !table.is_empty()
                        && table
                            .keys()
                            .all(|key| key == "variants" || key == "not_variants")
                })
                .collect();
            let mut sets = Vec::new();
            for set in model::variant_options(&restricted) {
                let held: Names = set.iter().cloned().collect();
                if undrawn
                    .iter()
                    .any(|table| set_match(table, "variants", &held))
                {
                    continue;
                }
                let by_variant = entry
                    .sizes_by_variant
                    .iter()
                    .find(|(variant, _, _)| held.contains(variant));
                let taken = by_variant.map_or(&entry.sizes, |(_, sizes, _)| sizes);
                let sizes: Vec<Option<String>> = if sized {
                    if taken.is_empty() {
                        vec![None]
                    } else {
                        taken.iter().cloned().map(Some).collect()
                    }
                } else if taken.is_empty() {
                    vec![None]
                } else {
                    let default = match by_variant {
                        Some((_, _, Some(default))) => Some(default.clone()),
                        _ => entry.default_size.clone(),
                    };
                    vec![default]
                };
                if !sets.iter().any(|(held_set, _)| *held_set == held) {
                    sets.push((held, sizes));
                }
            }
            let states_only: Vec<&Table> = entry
                .not_drawn
                .iter()
                .filter(|table| {
                    !table.is_empty()
                        && table
                            .keys()
                            .all(|key| key == "states" || key == "not_states")
                })
                .collect();
            let states: Vec<Names> = model::state_sets(&entry.states, &entry.exclusive)
                .into_iter()
                .map(|set| set.into_iter().collect::<Names>())
                .filter(|set| {
                    !states_only
                        .iter()
                        .any(|table| set_match(table, "states", set))
                        && !set
                            .iter()
                            .any(|flag| units.name_off(widget, preset, flag, false, added))
                })
                .collect();
            let schemes: Vec<String> = {
                let listed: Vec<String> = entry
                    .axis_conditions
                    .iter()
                    .filter(|condition| condition.starts_with("scheme:"))
                    .cloned()
                    .collect();
                if listed.is_empty() {
                    vec!["scheme:light".to_owned(), "scheme:dark".to_owned()]
                } else {
                    listed
                }
            };
            let mut kinds: Vec<(String, Vec<String>)> = Vec::new();
            for condition in &entry.axis_conditions {
                if condition.starts_with("scheme:") || units.condition_off(condition, widget, added)
                {
                    continue;
                }
                let kind = condition_kind(condition);
                match kinds.iter_mut().find(|(held, _)| *held == kind) {
                    Some((_, values)) => values.push(condition.clone()),
                    None => kinds.push((kind, vec![condition.clone()])),
                }
            }
            let mut vectors: Vec<Names> = schemes
                .into_iter()
                .map(|scheme| {
                    let mut vector: Names = std::iter::once(scheme).collect();
                    vector.extend(flavor.clone());
                    vector
                })
                .collect();
            for (_, values) in &kinds {
                let mut next = Vec::new();
                for vector in &vectors {
                    next.push(vector.clone());
                    for value in values {
                        let mut with = vector.clone();
                        with.insert(value.clone());
                        next.push(with);
                    }
                }
                vectors = next;
            }
            let densities: Vec<Option<String>> = std::iter::once(None)
                .chain(entry.densities.iter().filter(|_| sized).cloned().map(Some))
                .collect();
            let not_drawn: Vec<Match> = entry
                .not_drawn
                .iter()
                .filter(|table| !table.is_empty())
                .map(|table| compiled(Some(&Value::Table(table.clone()))))
                .filter(|matched| !matched.never)
                .collect();
            let mut contexts = vec![(Vec::new(), false)];
            for (at, context) in entry.contexts.iter().enumerate() {
                let chain = ancestors_of(context);
                let held = chain.iter().all(|ancestor| {
                    ancestor
                        .widget
                        .as_ref()
                        .is_some_and(|widget| units.hold(widget))
                });
                let wanted = match hosts {
                    Hosts::Held => held,
                    Hosts::At(place) => place == at || held,
                };
                if wanted && !chain.is_empty() {
                    contexts.push((chain, true));
                }
            }
            let mut rtl = vec![false];
            if !units.direction_off("rtl", added) {
                rtl.push(true);
            }
            out.push(Space {
                recency: entry.recency.clone(),
                sets,
                states,
                vectors,
                densities,
                not_drawn,
                contexts,
                rtl,
            });
        }
        let out = std::rc::Rc::new(out);
        self.spaces
            .borrow_mut()
            .insert(cache_key, std::rc::Rc::clone(&out));
        out
    }

    /// The part's presence facts for the preset, in order, each as a Match
    /// with the value it states (`None`: pending).
    fn presences(&self, part_key: &str) -> Vec<(Match, Option<Value>)> {
        self.platform
            .facts
            .iter()
            .filter(|fact| fact.key == part_key && !fact.accent_run)
            .filter_map(|fact| {
                let (written, _) = facts::for_preset(fact.when.as_ref(), &self.preset)?;
                let value = match &fact.stated {
                    Stated::Value(value) => Some(value.clone()),
                    Stated::Pending(_) => None,
                };
                Some((compiled(written.as_ref()), value))
            })
            .collect()
    }

    fn drawn_in(
        presences: &[(Match, Option<Value>)],
        c: &Combo,
        recency: &[(String, Vec<String>)],
    ) -> bool {
        let last = presences
            .iter()
            .rev()
            .find(|(when, _)| combo_holds(when, c, recency));
        match last.and_then(|(_, value)| value.as_ref()) {
            Some(Value::String(text)) => text != "absent",
            Some(Value::Table(table)) => !table.contains_key("drawn_as"),
            _ => true,
        }
    }

    /// Whether a fact of a widget's key holds, with the later units `added`
    /// and the host contexts `hosts`, in a combination its state model in
    /// force draws where its part is drawn, decided exactly.
    fn holds_exact(&self, key: &str, when: Option<&Value>, added: &[String], hosts: Hosts) -> bool {
        let segments: Vec<&str> = key.split('.').collect();
        let widget = segments.first().copied().unwrap_or_default();
        let part = match segments.get(1) {
            Some(part) if segments.len() > 2 && *part != "values" => {
                Some(format!("{widget}.{part}"))
            }
            _ => None,
        };
        let presences = part
            .as_deref()
            .map(|part| self.presences(part))
            .unwrap_or_default();
        let m = compiled(when);
        if m.never {
            return false;
        }
        for space in self.space(widget, added, hosts).iter() {
            let mut reads: Vec<&Match> = vec![&m];
            reads.extend(&space.not_drawn);
            reads.extend(presences.iter().map(|(when, _)| when));
            let relevant = |at: usize| -> Names {
                reads
                    .iter()
                    .filter_map(|read| read.lists.get(at))
                    .flat_map(|(pos, neg)| pos.iter().chain(neg))
                    .cloned()
                    .collect()
            };
            let (rc, mut rs, rv, rz, rd) = (
                relevant(0),
                relevant(1),
                relevant(2),
                relevant(3),
                relevant(4),
            );
            if reads.iter().any(|read| !read.latest.is_empty()) {
                rs.extend(
                    space
                        .recency
                        .iter()
                        .flat_map(|(_, flags)| flags.iter().cloned()),
                );
            }
            let project =
                |set: &Names, keep: &Names| -> Names { set.intersection(keep).cloned().collect() };
            let keep = |value: &Option<String>, within: &Names| {
                value.clone().filter(|held| within.contains(held))
            };
            let fits = |set: &Names, at: usize| {
                m.lists
                    .get(at)
                    .is_some_and(|(pos, neg)| pos.is_subset(set) && neg.is_disjoint(set))
            };
            let mut classes: BTreeSet<(Names, Option<String>, Option<String>)> = BTreeSet::new();
            for (set, sizes) in &space.sets {
                if !fits(set, 2) {
                    continue;
                }
                for size in sizes {
                    for density in &space.densities {
                        classes.insert((project(set, &rv), keep(size, &rz), keep(density, &rd)));
                    }
                }
            }
            let kinds: BTreeSet<Names> = space
                .states
                .iter()
                .filter(|set| fits(set, 1))
                .map(|set| project(set, &rs))
                .collect();
            let conds: BTreeSet<Names> = space
                .vectors
                .iter()
                .filter(|set| fits(set, 0))
                .map(|set| project(set, &rc))
                .collect();
            for (variants, size, density) in &classes {
                for states in &kinds {
                    for conditions in &conds {
                        for (ancestors, siblings) in &space.contexts {
                            for rtl in &space.rtl {
                                let c = Combo {
                                    conditions: conditions.clone(),
                                    states: states.clone(),
                                    variants: variants.clone(),
                                    size: size.clone(),
                                    density: density.clone(),
                                    rtl: *rtl,
                                    ancestors: ancestors.clone(),
                                    siblings: *siblings,
                                };
                                if combo_holds(&m, &c, &space.recency)
                                    && !space
                                        .not_drawn
                                        .iter()
                                        .any(|nd| combo_holds(nd, &c, &space.recency))
                                    && Self::drawn_in(&presences, &c, &space.recency)
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// The widgets a `when`'s positive lists name as an ancestor or a
    /// sibling.
    fn context_names(when: &Table) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        for entry in tables(when.get("context")) {
            out.extend(
                entry
                    .get("widget")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
            );
            if let Some(scope) = entry.get("scope").and_then(Value::as_table) {
                out.extend(strings(scope.get("nearest")));
            }
        }
        if let Some(position) = when.get("position").and_then(Value::as_table) {
            out.extend(strings(position.get("among")));
            for side in ["previous", "next"] {
                out.extend(
                    position
                        .get(side)
                        .and_then(|sibling| sibling.get("widget"))
                        .and_then(Value::as_str)
                        .map(str::to_owned),
                );
            }
        }
        out
    }

    /// `None` where a fact of the preset (its `when` with the preset
    /// decided, `written`) is of the enabled units; else the later unit it
    /// waits for, its key's or a context widget's unit where the model does
    /// not hold them, or [`STRAY`] (check_facts_trace.py's `Depth.unit`).
    pub fn unit(&self, key: &str, written: Option<&Value>) -> Option<String> {
        let units = &self.platform.units;
        let preset = self.preset.as_str();
        if !(key.starts_with("input.") || units.hold_on(key, Some(preset))) {
            return Some(units.key_unit(key));
        }
        let empty = Table::new();
        let when = written.and_then(Value::as_table).unwrap_or(&empty);
        if let Some(widget) = Self::context_names(when)
            .into_iter()
            .find(|widget| !units.hold(widget))
        {
            return Some(widget);
        }
        let waiting = units.waiting_names();
        let place = |unit: &str| waiting.iter().position(|held| held == unit);
        let mut needed: Vec<usize> = strings(when.get("conditions"))
            .iter()
            .filter_map(|condition| units.condition_holder(condition, key))
            .filter_map(|(_, unit)| place(unit))
            .collect();
        let widget = key.split('.').next().unwrap_or_default();
        if ["role", "token", "input"].contains(&widget) {
            let conditions = strings(when.get("conditions"));
            let kinds: BTreeSet<String> = conditions.iter().map(|c| condition_kind(c)).collect();
            if kinds.len() != conditions.len() {
                return Some(STRAY.to_owned());
            }
            return needed
                .into_iter()
                .max()
                .and_then(|at| waiting.get(at).cloned());
        }
        for variant in strings(when.get("variants")) {
            needed.extend(
                units
                    .holder(widget, preset, &variant, true)
                    .and_then(|(_, unit)| place(unit)),
            );
        }
        let mut flags = strings(when.get("states"));
        if let Some(latest) = when.get("latest").and_then(Value::as_table) {
            flags.extend(latest.values().filter_map(Value::as_str).map(str::to_owned));
        }
        for flag in flags {
            needed.extend(
                units
                    .holder(widget, preset, &flag, false)
                    .and_then(|(_, unit)| place(unit)),
            );
        }
        if let Some(direction) = when.get("direction").and_then(Value::as_str) {
            needed.extend(
                units
                    .direction_holder(direction)
                    .and_then(|(_, unit)| place(unit)),
            );
        }
        if needed.is_empty() && self.holds_exact(key, written, &[], Hosts::Held) {
            return None;
        }
        let added = |upto: usize| -> Vec<String> {
            waiting
                .iter()
                .take(upto.saturating_add(1))
                .cloned()
                .collect()
        };
        let last = waiting.len().saturating_sub(1);
        let every = added(last);
        if !self.holds_exact(key, written, &every, Hosts::Held) {
            let model = model::in_force(&self.platform.models, preset, None)
                .into_iter()
                .find(|entry| entry.widget == widget);
            for (at, context) in model
                .iter()
                .flat_map(|entry| entry.contexts.iter().enumerate())
            {
                if self.holds_exact(key, written, &every, Hosts::At(at)) {
                    let named = ancestors_of(context)
                        .into_iter()
                        .filter_map(|ancestor| ancestor.widget)
                        .next_back();
                    return Some(named.unwrap_or_else(|| HOSTLESS.to_owned()));
                }
            }
            return Some(STRAY.to_owned());
        }
        let (mut lo, mut hi) = (needed.into_iter().max().unwrap_or(0), last);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.holds_exact(key, written, &added(mid), Hosts::Held) {
                hi = mid;
            } else {
                lo = mid.saturating_add(1);
            }
        }
        waiting.get(lo).cloned()
    }
}
