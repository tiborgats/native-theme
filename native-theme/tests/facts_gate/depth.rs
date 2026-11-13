//! S17's predicate of a fact's units and the unit a fact not of the enabled
//! units waits for (ADD-A147), decided as `scripts/check_facts_trace.py`'s
//! class `Depth` decides them, so the gate and the scripts attribute every
//! fact alike (`NT_FACTS_GATE_UNITS` compares the two reports).
//!
//! A combination at depth of a widget's state model in force (the default
//! flavour's entry, each flavour's beside it once the flavours unit is
//! enabled) is a variant set it draws of variants at depth, a state
//! combination of flags at depth, a condition vector of conditions at depth,
//! a size and a density in force, a context (none, or an `axes.contexts`
//! entry whose widgets are enabled) and a direction, less those a
//! `not_drawn` entry names; they are enumerated whole, once per entry and
//! preset, with no cap. A fact is of the enabled units where its key and the
//! widgets its positive lists name are held, its names are at depth, and its
//! `when` holds (S11) in one of them where its part is drawn (the last
//! presence fact holding there is not `absent` or a `drawn_as`, a pending one
//! counting as drawn); each distinct `when` is decided once per entry or
//! part. Any other fact is attributed by names alone ([`Depth::unit`]).

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

use toml::{Table, Value};

use super::facts::{self, Stated};
use super::model::{self, Model, strings};
use super::run::Platform;

/// The unit of a fact drawn only in a context naming no widget: the
/// identities, with their hosts.
const HOSTLESS: &str = "identities";

/// Where a fact of a preset stands (S17 *While the units are built*).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Place {
    /// Of the enabled units: the run checks it.
    Enabled,
    /// Waiting for this unit, named by its key, a widget its `when` names,
    /// the host it is drawn only inside, or the last later unit holding a
    /// name its `when` lists positively.
    Later(String),
    /// Not of the enabled units, and no unit's names place it: reported
    /// under no unit.
    Unplaced,
}

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
    /// Each variant set drawn, with the sizes it takes in each of
    /// `contexts`, by its place.
    sets: Vec<(Names, Vec<Vec<Option<String>>>)>,
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

/// The variant sets an entry draws, whatever unit holds their names: its
/// variant options (none, each single variant, each listed set, each set its
/// products give, less those `requires_variants` excludes) less those a
/// `not_drawn` entry stating only variants removes (check_facts_trace.py's
/// `drawn_sets`).
fn drawn_sets(entry: &Model) -> Vec<Names> {
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
    model::variant_options(entry)
        .into_iter()
        .map(|set| set.into_iter().collect::<Names>())
        .filter(|set| {
            !undrawn
                .iter()
                .any(|table| set_match(table, "variants", set))
        })
        .collect()
}

/// The default size in force of an element of `entry` holding `variants`
/// in the `axes.contexts` entry `context` (`None`: none), as the query
/// reads it (S10): the first `sizes_by_context` entry's whose selector its
/// ancestors match, where it states one, else its first `sizes_by_variant`
/// entry's whose variant it holds, where it states one, else the entry's;
/// `None` where that default is no size the element takes (those of its
/// variant entry, else the entry's, that its context entry keeps): a pending
/// default, which leaves no size in force, or none stated.
fn default_size(entry: &Model, variants: &Names, context: Option<&Value>) -> Option<String> {
    let by_variant = entry
        .sizes_by_variant
        .iter()
        .find(|(variant, _, _)| variants.contains(variant));
    let by_context = entry
        .sizes_by_context
        .iter()
        .find(|(selector, _, _)| model::selector_matches(selector, context));
    let taken = by_variant.map_or(&entry.sizes, |(_, sizes, _)| sizes);
    let default = by_context
        .and_then(|(_, _, default)| default.as_ref())
        .or_else(|| by_variant.and_then(|(_, _, default)| default.as_ref()))
        .or(entry.default_size.as_ref())?;
    let kept = by_context.is_none_or(|(_, kept, _)| kept.contains(default));
    (taken.contains(default) && kept).then(|| default.clone())
}

/// Whether a presence fact's value leaves its part drawn: not `absent` or a
/// `drawn_as` (S6.3); a pending one (`None`) does (S17).
fn draws(value: Option<&Value>) -> bool {
    match value {
        Some(Value::String(text)) => text != "absent",
        Some(Value::Table(table)) => !table.contains_key("drawn_as"),
        _ => true,
    }
}

/// Whether a context entry of a `when` names a widget (its own, or its
/// `nearest` scope's).
fn names_widget(entry: &Table) -> bool {
    entry.get("widget").and_then(Value::as_str).is_some()
        || entry
            .get("scope")
            .and_then(Value::as_table)
            .is_some_and(|scope| !strings(scope.get("nearest")).is_empty())
}

/// One presence fact for the preset: its `when` read, as written for the
/// preset, and its value (`None`: pending).
type Presence = (Match, Option<Value>, Option<Value>);

/// The combinations at depth of a widget, each with the place of its
/// entry's [`Space`].
type Combos = Rc<Vec<(usize, Combo)>>;

/// The predicate over one platform's facts for one preset.
pub struct Depth<'p> {
    platform: &'p Platform,
    preset: String,
    /// The preset's default flavour's condition (`flavor:<f>`), where it
    /// declares flavours: it holds in every combination of the default
    /// flavour's entries.
    default_flavour: Option<String>,
    spaces: RefCell<HashMap<Flavoured<()>, Rc<Vec<Space>>>>,
    combos: RefCell<HashMap<Flavoured<()>, Combos>>,
    drawn: RefCell<HashMap<String, Rc<Vec<usize>>>>,
    holds: RefCell<HashMap<(String, String), bool>>,
    hosts: RefCell<HashMap<(String, String), Option<String>>>,
    defaults: RefCell<HashMap<Flavoured<Names>, Rc<BTreeSet<String>>>>,
    parts: RefCell<HashMap<String, Option<String>>>,
    sets: RefCell<HashMap<Flavoured<()>, Option<EntrySets>>>,
}

/// An entry in force and the variant sets it draws ([`drawn_sets`]).
type EntrySets = Rc<(Model, Vec<Names>)>;

/// A widget, a flavour (`None`: the default) and what else a memo is of.
type Flavoured<T> = (String, Option<String>, T);

impl<'p> Depth<'p> {
    /// The predicate for `preset`, whose default flavour is `default_flavour`
    /// where it declares flavours.
    pub fn new(platform: &'p Platform, preset: &str, default_flavour: Option<&str>) -> Depth<'p> {
        Depth {
            platform,
            preset: preset.to_owned(),
            default_flavour: default_flavour.map(|flavour| format!("flavor:{flavour}")),
            spaces: RefCell::new(HashMap::new()),
            combos: RefCell::new(HashMap::new()),
            drawn: RefCell::new(HashMap::new()),
            holds: RefCell::new(HashMap::new()),
            hosts: RefCell::new(HashMap::new()),
            defaults: RefCell::new(HashMap::new()),
            parts: RefCell::new(HashMap::new()),
            sets: RefCell::new(HashMap::new()),
        }
    }

    /// The entries in force: the default flavour's, then each flavour's (an
    /// entry naming `flavor:<f>`), whatever unit holds the flavour.
    fn entries(&self, widget: &str) -> Vec<(Option<String>, Model)> {
        let mut out: Vec<(Option<String>, Model)> =
            model::in_force(&self.platform.models, &self.preset, None)
                .into_iter()
                .filter(|entry| entry.widget == widget && entry.available)
                .map(|entry| (None, entry.clone()))
                .collect();
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
            if entry.widget == widget && entry.available && for_preset {
                out.push((Some(flavor.clone()), entry.clone()));
            }
        }
        out
    }

    /// The default flavour's entry in force (`flavour` `None`), or the
    /// flavour's, with the variant sets it draws; computed once.
    fn entry_for(&self, widget: &str, flavour: Option<&str>) -> Option<EntrySets> {
        let memo = (widget.to_owned(), flavour.map(str::to_owned), ());
        if let Some(found) = self.sets.borrow().get(&memo) {
            return found.clone();
        }
        let found = self
            .entries(widget)
            .into_iter()
            .find(|(held, _)| held.as_deref() == flavour)
            .map(|(_, entry)| {
                let sets = drawn_sets(&entry);
                Rc::new((entry, sets))
            });
        self.sets.borrow_mut().insert(memo, found.clone());
        found
    }

    /// Per entry in force, its dimensions at depth (check_facts_trace.py's
    /// `space`), computed once per widget.
    fn space(&self, widget: &str) -> Rc<Vec<Space>> {
        self.space_of(widget, None)
    }

    /// [`Depth::space`], or with `only` (a `flavor:<f>` condition) that
    /// flavour's own entry alone, enumerated as though its flavour were in
    /// force (its condition in every vector, its other names at depth),
    /// whatever the flavours unit's state: what a flavour's `[declare]`
    /// section is true to (S10; check_facts_trace.py's `space` with
    /// `only`). Computed once per widget and flavour.
    fn space_of(&self, widget: &str, only: Option<&str>) -> Rc<Vec<Space>> {
        let memo = (widget.to_owned(), only.map(str::to_owned), ());
        if let Some(found) = self.spaces.borrow().get(&memo) {
            return Rc::clone(found);
        }
        let units = &self.platform.units;
        let preset = self.preset.as_str();
        let sized = units.sizes_enabled();
        let flavours = units.flavours_enabled();
        let mut out = Vec::new();
        for (flavor, entry) in self.entries(widget) {
            let skipped = match only {
                Some(only) => flavor.as_deref() != Some(only),
                None => flavor.as_ref().is_some_and(|flavor| {
                    !flavours || units.condition_holder(flavor, widget).is_some()
                }),
            };
            if skipped {
                continue;
            }
            let off: Vec<String> = entry
                .variants
                .iter()
                .filter(|variant| !units.at_depth(widget, preset, variant, true))
                .cloned()
                .collect();
            // Each context: none, or an `axes.contexts` entry whose
            // ancestors name widgets an enabled unit holds.
            let mut contexts = vec![(Vec::new(), false)];
            let mut context_values: Vec<Option<&Value>> = vec![None];
            for context in &entry.contexts {
                let chain = ancestors_of(context);
                if !chain.is_empty() && self.held(&chain) {
                    contexts.push((chain, true));
                    context_values.push(Some(context));
                }
            }
            let mut sets = Vec::new();
            for held in drawn_sets(&entry.without(&off)) {
                let taken = entry
                    .sizes_by_variant
                    .iter()
                    .find(|(variant, _, _)| held.contains(variant))
                    .map_or(&entry.sizes, |(_, sizes, _)| sizes);
                // Per context, the sizes an element takes: while the sizes
                // unit is not enabled, its default in force there alone.
                let sizes: Vec<Vec<Option<String>>> = context_values
                    .iter()
                    .map(|context| {
                        if sized && !taken.is_empty() {
                            taken.iter().cloned().map(Some).collect()
                        } else if sized {
                            vec![None]
                        } else {
                            vec![default_size(&entry, &held, *context)]
                        }
                    })
                    .collect();
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
                            .any(|flag| !units.at_depth(widget, preset, flag, false))
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
                if condition.starts_with("scheme:") || !units.condition_at_depth(condition, widget)
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
                    vector.extend(flavor.clone().or_else(|| self.default_flavour.clone()));
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
            let mut rtl = vec![false];
            if units.direction_holder("rtl").is_none() {
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
        let out = Rc::new(out);
        self.spaces.borrow_mut().insert(memo, Rc::clone(&out));
        out
    }

    /// Whether every ancestor of a host chain names a widget an enabled
    /// unit holds.
    fn held(&self, chain: &[Ancestor]) -> bool {
        chain.iter().all(|ancestor| {
            ancestor
                .widget
                .as_ref()
                .is_some_and(|widget| self.platform.units.hold(widget))
        })
    }

    /// Every combination at depth of the widget's entries in force, with
    /// the place of its entry's [`Space`]: each choice of a variant set, a
    /// size, a density, a state combination, a condition vector, a context
    /// and a direction, less those a `not_drawn` entry names; enumerated
    /// whole once per widget, with no cap.
    fn combos(&self, widget: &str) -> Combos {
        self.combos_of(widget, None)
    }

    /// [`Depth::combos`] over [`Depth::space_of`]'s entries.
    fn combos_of(&self, widget: &str, only: Option<&str>) -> Combos {
        let memo = (widget.to_owned(), only.map(str::to_owned), ());
        if let Some(found) = self.combos.borrow().get(&memo) {
            return Rc::clone(found);
        }
        let mut out = Vec::new();
        for (at, space) in self.space_of(widget, only).iter().enumerate() {
            for (variants, sizes) in &space.sets {
                for ((ancestors, siblings), sizes) in space.contexts.iter().zip(sizes) {
                    for size in sizes {
                        for density in &space.densities {
                            for states in &space.states {
                                for conditions in &space.vectors {
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
                                        if !space
                                            .not_drawn
                                            .iter()
                                            .any(|nd| combo_holds(nd, &c, &space.recency))
                                        {
                                            out.push((at, c));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let out = Rc::new(out);
        self.combos.borrow_mut().insert(memo, Rc::clone(&out));
        out
    }

    /// The variants some combination at depth of the widget holds
    /// ([`Depth::combos`]; check_facts_trace.py's `combos`, which
    /// scripts/generate_preset.py's `later_variants()` reads); with
    /// `flavour` (a flavour's name, not the default's), those of that
    /// flavour's own entry ([`Depth::space_of`]).
    pub fn drawn_variants(&self, widget: &str, flavour: Option<&str>) -> Names {
        let only = flavour.map(|flavour| format!("flavor:{flavour}"));
        self.combos_of(widget, only.as_deref())
            .iter()
            .flat_map(|(_, combo)| combo.variants.iter().cloned())
            .collect()
    }

    /// The part's presence facts for the preset, in order, each as a Match,
    /// its `when` as written for the preset, and the value it states
    /// (`None`: pending).
    fn presences(&self, part_key: &str) -> Vec<Presence> {
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
                Some((compiled(written.as_ref()), written, value))
            })
            .collect()
    }

    /// The places in [`Depth::combos`] of the combinations at depth where
    /// the part is drawn: the last of its presence facts holding there
    /// draws it, or none holds there; computed once per part.
    fn drawn(&self, widget: &str, part: &str) -> Rc<Vec<usize>> {
        if let Some(found) = self.drawn.borrow().get(part) {
            return Rc::clone(found);
        }
        let spaces = self.space(widget);
        let presences = self.presences(part);
        let out: Vec<usize> = self
            .combos(widget)
            .iter()
            .enumerate()
            .filter(|(_, (at, c))| {
                let recency = spaces.get(*at).map_or(&[][..], |space| &space.recency);
                presences
                    .iter()
                    .rev()
                    .find(|(when, _, _)| combo_holds(when, c, recency))
                    .is_none_or(|(_, _, value)| draws(value.as_ref()))
            })
            .map(|(place, _)| place)
            .collect();
        let out = Rc::new(out);
        self.drawn
            .borrow_mut()
            .insert(part.to_owned(), Rc::clone(&out));
        out
    }

    /// The widget and the part (`<widget>.<part>`, `None` for a widget
    /// value, a part's own key or a key of no widget) of a key.
    fn widget_part(key: &str) -> (&str, Option<String>) {
        let segments: Vec<&str> = key.split('.').collect();
        let widget = segments.first().copied().unwrap_or_default();
        let part = match segments.get(1) {
            Some(part) if segments.len() > 2 && *part != "values" => {
                Some(format!("{widget}.{part}"))
            }
            _ => None,
        };
        (widget, part)
    }

    /// Whether a fact of a widget's key (its `when` as written for the
    /// preset) holds in a combination at depth where its part is drawn:
    /// every such combination read, no cap; each distinct `when` decided
    /// once per entry or part.
    fn holds_at(&self, key: &str, written: Option<&Value>) -> bool {
        let (widget, part) = Self::widget_part(key);
        let memo = (
            part.clone().unwrap_or_else(|| widget.to_owned()),
            written.map(Value::to_string).unwrap_or_default(),
        );
        if let Some(found) = self.holds.borrow().get(&memo) {
            return *found;
        }
        let m = compiled(written);
        let spaces = self.space(widget);
        let combos = self.combos(widget);
        let holds = |(at, c): &(usize, Combo)| {
            spaces
                .get(*at)
                .is_some_and(|space| combo_holds(&m, c, &space.recency))
        };
        let found = !m.never
            && match &part {
                Some(part) => self
                    .drawn(widget, part)
                    .iter()
                    .any(|place| combos.get(*place).is_some_and(holds)),
                None => combos.iter().any(holds),
            };
        self.holds.borrow_mut().insert(memo, found);
        found
    }

    /// The sizes the sizes unit does not hold for a `when`: the default in
    /// force of each variant set holding its variants that its flavour's
    /// entry draws (the default flavour's where it names none or the
    /// default), with no context or in each of the entry's contexts whose
    /// widgets an enabled unit holds.
    fn size_defaults(&self, widget: &str, when: &Table) -> Rc<BTreeSet<String>> {
        let flavour = names(when.get("conditions"))
            .into_iter()
            .find(|condition| condition.starts_with("flavor:"))
            .filter(|condition| Some(condition) != self.default_flavour.as_ref());
        let variants = names(when.get("variants"));
        let memo = (widget.to_owned(), flavour, variants);
        if let Some(found) = self.defaults.borrow().get(&memo) {
            return Rc::clone(found);
        }
        let found: BTreeSet<String> = self
            .entry_for(widget, memo.1.as_deref())
            .map(|found| {
                let (entry, sets) = &*found;
                let contexts: Vec<Option<&Value>> = std::iter::once(None)
                    .chain(
                        entry
                            .contexts
                            .iter()
                            .filter(|context| {
                                let chain = ancestors_of(context);
                                !chain.is_empty() && self.held(&chain)
                            })
                            .map(Some),
                    )
                    .collect();
                sets.iter()
                    .filter(|set| memo.2.is_subset(set))
                    .flat_map(|set| {
                        contexts
                            .iter()
                            .filter_map(|context| default_size(entry, set, *context))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let found = Rc::new(found);
        self.defaults.borrow_mut().insert(memo, Rc::clone(&found));
        found
    }

    /// The unit of the host inside which alone the key's entry draws one of
    /// `variants` (the first in sorted order so drawn), else its part;
    /// `None` where it draws them outside a host.
    fn host_unit(&self, key: &str, variants: &Names) -> Option<String> {
        let (widget, part) = Self::widget_part(key);
        variants
            .iter()
            .find_map(|variant| self.variant_host(widget, variant))
            .or_else(|| part.and_then(|part| self.part_host(&part)))
    }

    /// Where the widget's default entry in force draws no variant set
    /// holding `variant` outside a host context, the unit of the first
    /// `axes.contexts` entry, of those whose widgets are not all enabled,
    /// inside which it draws one: the entry's innermost widget, the
    /// identities where it names none; else `None`. Read from the entry's
    /// structure alone (its sets and its `not_drawn` entries stating only
    /// variants and contexts), once per widget and variant.
    fn variant_host(&self, widget: &str, variant: &str) -> Option<String> {
        let memo = (widget.to_owned(), variant.to_owned());
        if let Some(found) = self.hosts.borrow().get(&memo) {
            return found.clone();
        }
        let found = self.entry_for(widget, None).and_then(|found| {
            let (entry, sets) = &*found;
            let sets: Vec<&Names> = sets.iter().filter(|set| set.contains(variant)).collect();
            let gone: Vec<Match> = entry
                .not_drawn
                .iter()
                .filter(|table| {
                    !table.is_empty()
                        && table.keys().all(|key| {
                            ["variants", "not_variants", "context", "not_context"]
                                .contains(&key.as_str())
                        })
                })
                .map(|table| compiled(Some(&Value::Table(table.clone()))))
                .collect();
            let drawn = |ancestors: &[Ancestor]| {
                sets.iter().any(|set| {
                    let c = Combo {
                        conditions: Names::new(),
                        states: Names::new(),
                        variants: (*set).clone(),
                        size: None,
                        density: None,
                        rtl: false,
                        ancestors: ancestors.to_vec(),
                        siblings: !ancestors.is_empty(),
                    };
                    !gone.iter().any(|m| combo_holds(m, &c, &[]))
                })
            };
            if sets.is_empty() || drawn(&[]) {
                return None;
            }
            entry.contexts.iter().find_map(|context| {
                let chain = ancestors_of(context);
                if self.held(&chain) || !drawn(&chain) {
                    return None;
                }
                Some(
                    chain
                        .iter()
                        .filter_map(|ancestor| ancestor.widget.clone())
                        .next_back()
                        .unwrap_or_else(|| HOSTLESS.to_owned()),
                )
            })
        });
        self.hosts.borrow_mut().insert(memo, found.clone());
        found
    }

    /// Where the part's presence facts draw it only inside a host context
    /// (after the last one with no `when` that does not draw it, every one
    /// that draws it names a context, and one does): the widget the first
    /// such fact's first context entry names, the identities where it
    /// names none; else `None`.
    fn part_host(&self, part: &str) -> Option<String> {
        if let Some(found) = self.parts.borrow().get(part) {
            return found.clone();
        }
        let found = self.part_host_of(part);
        self.parts
            .borrow_mut()
            .insert(part.to_owned(), found.clone());
        found
    }

    /// [`Depth::part_host`], computed.
    fn part_host_of(&self, part: &str) -> Option<String> {
        let presences = self.presences(part);
        let unconditional = |written: &Option<Value>| {
            written.as_ref().is_none_or(|when| {
                when.as_table().is_some_and(|table| {
                    table
                        .values()
                        .all(|value| value.as_array().is_some_and(Vec::is_empty))
                })
            })
        };
        let base = presences
            .iter()
            .rposition(|(_, written, value)| unconditional(written) && !draws(value.as_ref()))?;
        let contexts: Vec<Vec<Value>> = presences
            .iter()
            .skip(base.saturating_add(1))
            .filter(|(_, _, value)| draws(value.as_ref()))
            .map(|(_, written, _)| {
                written
                    .as_ref()
                    .and_then(|when| when.get("context"))
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
            })
            .collect();
        if contexts.is_empty() || contexts.iter().any(Vec::is_empty) {
            return None;
        }
        let first = contexts.first().and_then(|context| context.first());
        Some(
            first
                .and_then(|entry| entry.get("widget"))
                .and_then(Value::as_str)
                .map_or_else(|| HOSTLESS.to_owned(), str::to_owned),
        )
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

    /// The state flags a `when` names positively: its `states` and its
    /// `latest` flags.
    fn flags(when: &Table) -> Vec<String> {
        let mut flags = strings(when.get("states"));
        if let Some(latest) = when.get("latest").and_then(Value::as_table) {
            flags.extend(latest.values().filter_map(Value::as_str).map(str::to_owned));
        }
        flags
    }

    /// Whether a fact whose key and named widgets are held is of the
    /// enabled units: its conditions at depth for its key (of distinct
    /// kinds for a key of no widget), its variants and flags at depth, and
    /// its `when` holding in a combination at depth where its part is drawn
    /// (check_facts_trace.py's `Depth.of_enabled`).
    fn of_enabled(&self, key: &str, when: &Table, written: Option<&Value>) -> bool {
        let units = &self.platform.units;
        let preset = self.preset.as_str();
        let conditions = strings(when.get("conditions"));
        if conditions
            .iter()
            .any(|condition| !units.condition_at_depth(condition, key))
        {
            return false;
        }
        let (widget, _) = Self::widget_part(key);
        if ["role", "token", "input"].contains(&widget) {
            let kinds: BTreeSet<String> = conditions.iter().map(|c| condition_kind(c)).collect();
            return kinds.len() == names(when.get("conditions")).len();
        }
        if strings(when.get("variants"))
            .iter()
            .any(|variant| !units.at_depth(widget, preset, variant, true))
            || Self::flags(when)
                .iter()
                .any(|flag| !units.at_depth(widget, preset, flag, false))
        {
            return false;
        }
        self.holds_at(key, written)
    }

    /// Where a fact of the preset (its `when` with the preset decided,
    /// `written`) stands (check_facts_trace.py's `Depth.unit`): of the
    /// enabled units ([`Depth::of_enabled`]); else, by names alone, its
    /// key's unit where the model does not hold the key, a widget its
    /// positive lists name that the model does not hold, the host inside
    /// which alone its entry draws a variant it names or its part (where it
    /// names no context), or the last later unit, in the units file's
    /// order, that holds a name its `when` lists positively (a variant, a
    /// flag, a condition or flavour, a size no default in force, a density,
    /// a direction, a context naming no widget); else unplaced.
    pub fn unit(&self, key: &str, written: Option<&Value>) -> Place {
        let units = &self.platform.units;
        let preset = self.preset.as_str();
        if !(key.starts_with("input.") || units.hold_on(key, Some(preset))) {
            return Place::Later(units.key_unit(key));
        }
        let empty = Table::new();
        let when = written.and_then(Value::as_table).unwrap_or(&empty);
        if let Some(widget) = Self::context_names(when)
            .into_iter()
            .find(|widget| !units.hold(widget))
        {
            return Place::Later(widget);
        }
        if self.of_enabled(key, when, written) {
            return Place::Enabled;
        }
        let (widget, _) = Self::widget_part(key);
        let entry = !["role", "token", "input"].contains(&widget);
        let contexts = tables(when.get("context"));
        let names_context = when
            .get("context")
            .and_then(Value::as_array)
            .is_some_and(|context| !context.is_empty());
        if entry
            && !names_context
            && let Some(host) = self.host_unit(key, &names(when.get("variants")))
        {
            return Place::Later(host);
        }
        let mut found: Vec<(usize, &str)> = strings(when.get("conditions"))
            .iter()
            .filter_map(|condition| units.condition_holder(condition, key))
            .collect();
        if entry {
            found.extend(
                strings(when.get("variants"))
                    .iter()
                    .filter_map(|variant| units.holder(widget, preset, variant, true)),
            );
            found.extend(
                Self::flags(when)
                    .iter()
                    .filter_map(|flag| units.holder(widget, preset, flag, false)),
            );
            if let Some(direction) = when.get("direction").and_then(Value::as_str) {
                found.extend(units.direction_holder(direction));
            }
            if let Some(sizes) = units.sizes_holder() {
                let defaults = self.size_defaults(widget, when);
                if !strings(when.get("densities")).is_empty()
                    || strings(when.get("sizes"))
                        .iter()
                        .any(|size| !defaults.contains(size))
                {
                    found.push(sizes);
                }
            }
            if contexts.iter().any(|context| !names_widget(context)) {
                found.extend(units.waiting(HOSTLESS));
            }
        }
        found
            .into_iter()
            .max_by_key(|(at, _)| *at)
            .map_or(Place::Unplaced, |(_, unit)| Place::Later(unit.to_owned()))
    }
}
