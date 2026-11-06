//! The state models (S17 *State models*): each widget's entry, the one in
//! force for a preset and a flavour, and its combinations, enumerated as
//! S17 states them and projected onto what the rules and facts can tell
//! apart.

use std::collections::{BTreeSet, HashMap};

use native_theme::gate::{context_spelled, default_spelled, names_spelled};
use toml::{Table, Value};

/// The strings of a TOML list; none where it is not one.
pub fn strings(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

/// A list of lists of strings.
fn groups(value: Option<&Value>) -> Vec<Vec<String>> {
    value
        .and_then(Value::as_array)
        .map(|items| items.iter().map(|item| strings(Some(item))).collect())
        .unwrap_or_default()
}

/// A `default_size` as the gate compares it ([`default_spelled`]): a
/// size's name, or `{ pending = "U<n>" }` as `pending U<n>`.
fn default_of(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(size) => default_spelled(Some(size), &[]),
        Value::Table(table) => {
            let ids = match table.get("pending")? {
                Value::String(id) => vec![id.clone()],
                ids => strings(Some(ids)),
            };
            default_spelled(None, &ids)
        }
        _ => None,
    }
}

/// `[{ variant, sizes[, default_size] }]` (S10).
fn sizes_by_variant(value: Option<&Value>) -> Vec<(String, Vec<String>, Option<String>)> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    let name = item.get("variant")?.as_str()?.to_owned();
                    let default = default_of(item.get("default_size"));
                    Some((name, strings(item.get("sizes")), default))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// `[{ context, sizes[, default_size] }]` (S10), each selector kept as
/// stated.
fn sizes_by_context(value: Option<&Value>) -> Vec<(Value, Vec<String>, Option<String>)> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    let context = item.get("context")?.clone();
                    let default = default_of(item.get("default_size"));
                    Some((context, strings(item.get("sizes")), default))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// An S11 context selector as [`context_spelled`] writes it, a key stating
/// an empty list left out, as the crate's selector holds no such key.
pub fn selector_spelled(selector: &Value) -> String {
    let keys: Vec<(String, String)> = selector
        .as_table()
        .map(|table| {
            table
                .iter()
                .filter(|(_, value)| !matches!(value, Value::Array(items) if items.is_empty()))
                .map(|(key, value)| (key.clone(), value_spelled(value)))
                .collect()
        })
        .unwrap_or_default();
    context_spelled(&keys)
}

/// A selector's value as [`context_spelled`] writes it: a list's names
/// sorted, `scope`'s `nearest` table by its list.
fn value_spelled(value: &Value) -> String {
    match value {
        Value::Array(_) => names_spelled(&strings(Some(value))),
        Value::Table(table) => {
            let fields: Vec<String> = table
                .iter()
                .map(|(key, value)| format!("{key} = {}", value_spelled(value)))
                .collect();
            format!("{{ {} }}", fields.join(", "))
        }
        Value::String(text) => format!("{text:?}"),
        other => other.to_string(),
    }
}

/// Whether the S11 context selector `selector` matches the ancestors an
/// `axes.contexts` entry `context` builds (S17's chain, outermost first:
/// each ancestor its `widget`, `part`, `variants` and `states`), as
/// `check_facts.py`'s `selector_matches` reads it; `None` for no context.
pub fn selector_matches(selector: &Value, context: Option<&Value>) -> bool {
    let chain: Vec<&Value> = match context {
        None => Vec::new(),
        Some(Value::Array(items)) => items.iter().collect(),
        Some(table) => vec![table],
    };
    let candidates: Vec<&Value> = match selector.get("scope") {
        Some(Value::Table(scope)) => {
            let nearest = strings(scope.get("nearest"));
            chain
                .iter()
                .rev()
                .find(|ancestor| {
                    ancestor
                        .get("widget")
                        .and_then(Value::as_str)
                        .is_some_and(|widget| nearest.iter().any(|held| held == widget))
                })
                .into_iter()
                .copied()
                .collect()
        }
        _ => chain,
    };
    let holds = |ancestor: &&Value| {
        let same = |key: &str| {
            selector
                .get(key)
                .is_none_or(|wanted| ancestor.get(key) == Some(wanted))
        };
        let held = strings(ancestor.get("variants"));
        same("widget")
            && same("part")
            && strings(selector.get("variants"))
                .iter()
                .all(|variant| held.contains(variant))
            && !strings(selector.get("not_variants"))
                .iter()
                .any(|variant| held.contains(variant))
    };
    candidates.iter().any(holds)
}

/// One group of a variant product (S17 *Variant products*).
#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    /// `one`: exactly one; else `any`: at most one.
    pub exactly_one: bool,
    /// The variants.
    pub variants: Vec<String>,
}

/// One state-model entry (S17).
#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub file: String,
    pub widget: String,
    /// Its `when.conditions`: `preset:` and `flavor:` entries replace the
    /// shared entry.
    pub conditions: Vec<String>,
    pub available: bool,
    pub variants: Vec<String>,
    pub sizes: Vec<String>,
    pub default_size: Option<String>,
    pub densities: Vec<String>,
    pub flags: Vec<String>,
    pub states: Vec<String>,
    pub axis_conditions: Vec<String>,
    pub contexts: Vec<Value>,
    pub exclusive: Vec<Vec<String>>,
    pub exclusive_variants: Vec<Vec<String>>,
    pub recency: Vec<(String, Vec<String>)>,
    pub requires_variants: Vec<String>,
    pub sizes_by_variant: Vec<(String, Vec<String>, Option<String>)>,
    pub sizes_by_context: Vec<(Value, Vec<String>, Option<String>)>,
    pub variant_sets: Vec<Vec<String>>,
    pub variant_products: Vec<Vec<Group>>,
    pub not_drawn: Vec<Table>,
    /// The variants the gate leaves out of this entry for a preset, as only
    /// facts it skips name them (ADD-A128): no combination holds one.
    pub skipped_variants: Vec<String>,
}

impl Model {
    /// The entries of a `state-models.toml`; refused where an entry's
    /// `axes.states` holds more flags than the gate enumerates the subsets
    /// of ([`MAX_FLAGS`]), or a `not_drawn` entry a key the count does not
    /// evaluate ([`NOT_DRAWN_KEYS`]).
    pub fn read(file: &str, text: &str) -> Result<Vec<Model>, String> {
        let table: Table = toml::from_str(text).map_err(|error| format!("{file}: {error}"))?;
        let entries = table
            .get("model")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let models: Vec<Model> = entries
            .iter()
            .filter_map(Value::as_table)
            .map(|entry| Model::of(file, entry))
            .collect();
        for model in &models {
            if model.states.len() > MAX_FLAGS {
                return Err(format!(
                    "{file}: `{}` states {} flags, more than the {MAX_FLAGS} the gate enumerates",
                    model.widget,
                    model.states.len()
                ));
            }
            for when in &model.not_drawn {
                if let Some(key) = when
                    .keys()
                    .find(|key| !NOT_DRAWN_KEYS.contains(&key.as_str()))
                {
                    return Err(format!(
                        "{file}: a `not_drawn` entry of `{}` states `{key}`, which the gate's count \
                         does not evaluate",
                        model.widget
                    ));
                }
            }
        }
        Ok(models)
    }

    fn of(file: &str, entry: &Table) -> Model {
        let axes = entry.get("axes");
        let recency = entry
            .get("recency")
            .and_then(Value::as_table)
            .map(|sets| {
                sets.iter()
                    .map(|(name, flags)| (name.clone(), strings(Some(flags))))
                    .collect()
            })
            .unwrap_or_default();
        let products = entry
            .get("variant_products")
            .and_then(Value::as_array)
            .map(|products| {
                products
                    .iter()
                    .map(|product| {
                        product
                            .as_array()
                            .map(|groups| {
                                groups
                                    .iter()
                                    .filter_map(|group| {
                                        let (kind, variants) = group.as_table()?.iter().next()?;
                                        Some(Group {
                                            exactly_one: kind == "one",
                                            variants: strings(Some(variants)),
                                        })
                                    })
                                    .collect()
                            })
                            .unwrap_or_default()
                    })
                    .collect()
            })
            .unwrap_or_default();
        Model {
            file: file.to_owned(),
            widget: entry
                .get("widget")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned(),
            conditions: strings(entry.get("when").and_then(|when| when.get("conditions"))),
            available: entry
                .get("available")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            variants: strings(entry.get("variants")),
            sizes: strings(entry.get("sizes")),
            default_size: default_of(entry.get("default_size")),
            densities: strings(entry.get("densities")),
            flags: strings(entry.get("flags")),
            states: strings(axes.and_then(|axes| axes.get("states"))),
            axis_conditions: strings(axes.and_then(|axes| axes.get("conditions"))),
            contexts: axes
                .and_then(|axes| axes.get("contexts"))
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            exclusive: groups(entry.get("exclusive")),
            exclusive_variants: groups(entry.get("exclusive_variants")),
            recency,
            requires_variants: strings(entry.get("requires_variants")),
            sizes_by_variant: sizes_by_variant(entry.get("sizes_by_variant")),
            sizes_by_context: sizes_by_context(entry.get("sizes_by_context")),
            variant_sets: groups(entry.get("variant_sets")),
            variant_products: products,
            not_drawn: entry
                .get("not_drawn")
                .and_then(Value::as_array)
                .map(|items| items.iter().filter_map(Value::as_table).cloned().collect())
                .unwrap_or_default(),
            skipped_variants: Vec::new(),
        }
    }

    /// The entry without the variants `skipped` (ADD-A128): out of
    /// `variants` and each `exclusive_variants` group (a group left with
    /// fewer than two goes, as it excludes nothing), and out of every
    /// combination, a set holding one dropped whole.
    pub fn without(&self, skipped: &[String]) -> Model {
        let mut out = self.clone();
        out.variants.retain(|variant| !skipped.contains(variant));
        out.exclusive_variants = self
            .exclusive_variants
            .iter()
            .map(|group| {
                group
                    .iter()
                    .filter(|variant| !skipped.contains(variant))
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .filter(|group| group.len() >= 2)
            .collect();
        out.skipped_variants = skipped.to_vec();
        out
    }
}

/// The entries in force for `preset` and `flavor`, one per widget: an
/// entry naming the preset (or the flavour) replaces the shared one of its
/// widget, a preset's over a flavour's (S17). Entries naming another
/// preset or flavour are left out.
pub fn in_force<'m>(models: &'m [Model], preset: &str, flavor: Option<&str>) -> Vec<&'m Model> {
    let rank = |model: &Model| -> Option<u8> {
        let mut rank = 0_u8;
        for condition in &model.conditions {
            if let Some(id) = condition.strip_prefix("preset:") {
                if id != preset {
                    return None;
                }
                rank = rank.max(2);
            } else if let Some(name) = condition.strip_prefix("flavor:") {
                if Some(name) != flavor {
                    return None;
                }
                rank = rank.max(1);
            }
        }
        Some(rank)
    };
    let mut out: Vec<(&Model, u8)> = Vec::new();
    for model in models {
        let Some(rank) = rank(model) else {
            continue;
        };
        match out.iter_mut().find(|(held, _)| held.widget == model.widget) {
            Some(slot) if slot.1 < rank => *slot = (model, rank),
            Some(_) => {}
            None => out.push((model, rank)),
        }
    }
    out.into_iter().map(|(model, _)| model).collect()
}

/// A condition kind: one value or none of each per combination (S17), a
/// scheme always one (the no-reader state's under each scheme in turn).
fn condition_kind(condition: &str) -> String {
    match condition.split_once(':') {
        Some(("container_size", rest)) => format!(
            "container_size:{}",
            rest.split_once(':')
                .map_or(rest, |(container, _)| container)
        ),
        Some((kind, _)) => kind.to_owned(),
        None if condition.starts_with("contrast_") => "contrast".to_owned(),
        None => condition.to_owned(),
    }
}

/// The condition vectors of `conditions`: for each kind one value or none,
/// a scheme one of `schemes` (those listed where the axis lists schemes).
pub fn condition_vectors(conditions: &[String], schemes: &[String]) -> Vec<Vec<String>> {
    let listed: Vec<String> = conditions
        .iter()
        .filter_map(|condition| condition.strip_prefix("scheme:"))
        .filter(|scheme| schemes.iter().any(|held| held == scheme))
        .map(|scheme| format!("scheme:{scheme}"))
        .collect();
    let schemes: Vec<String> = if listed.is_empty() {
        schemes
            .iter()
            .map(|scheme| format!("scheme:{scheme}"))
            .collect()
    } else {
        listed
    };
    let mut kinds: Vec<(String, Vec<String>)> = Vec::new();
    for condition in conditions
        .iter()
        .filter(|condition| !condition.starts_with("scheme:"))
    {
        let kind = condition_kind(condition);
        match kinds.iter_mut().find(|(held, _)| *held == kind) {
            Some((_, values)) => {
                if !values.contains(condition) {
                    values.push(condition.clone());
                }
            }
            None => kinds.push((kind, vec![condition.clone()])),
        }
    }
    let mut out: Vec<Vec<String>> = schemes.into_iter().map(|scheme| vec![scheme]).collect();
    for (_, values) in kinds {
        let mut next = Vec::new();
        for vector in &out {
            next.push(vector.clone());
            for value in &values {
                let mut with = vector.clone();
                with.push(value.clone());
                next.push(with);
            }
        }
        out = next;
    }
    out
}

/// The most flags an entry's `axes.states` may hold: the gate enumerates
/// every subset, 2^20 of them at most.
pub const MAX_FLAGS: usize = 20;

/// The keys of a `not_drawn` entry the count of combinations evaluates
/// ([`names_element`], [`names_rest`]); the run matches every key as the
/// query does.
pub const NOT_DRAWN_KEYS: [&str; 12] = [
    "states",
    "not_states",
    "variants",
    "not_variants",
    "sizes",
    "not_sizes",
    "densities",
    "not_densities",
    "conditions",
    "not_conditions",
    "context",
    "not_context",
];

/// Every subset of `states` (at most [`MAX_FLAGS`], which reading the
/// entry checks), reduced by `exclusive` (each group keeping its first
/// present flag), each once, in a stable order.
pub fn state_sets(states: &[String], exclusive: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    for mask in 0_u32..(1_u32 << states.len()) {
        let mut held: Vec<String> = states
            .iter()
            .enumerate()
            .filter(|(at, _)| mask & (1 << at) != 0)
            .map(|(_, flag)| flag.clone())
            .collect();
        for group in exclusive {
            let mut present = group.iter().filter(|flag| held.contains(flag));
            if present.next().is_some() {
                let dropped: Vec<String> = present.cloned().collect();
                held.retain(|flag| !dropped.contains(flag));
            }
        }
        if !out.contains(&held) {
            out.push(held);
        }
    }
    out
}

/// The orders of a state set (S17): one per order of its flags that belong
/// to a recency set, the orders giving every set the same latest flag
/// counted once; the single empty order where it holds fewer than two.
pub fn orders(states: &[String], recency: &[(String, Vec<String>)]) -> Vec<Vec<String>> {
    let members: Vec<String> = states
        .iter()
        .filter(|flag| recency.iter().any(|(_, set)| set.contains(flag)))
        .cloned()
        .collect();
    if members.len() < 2 {
        return vec![Vec::new()];
    }
    let mut seen: Vec<Vec<Option<String>>> = Vec::new();
    let mut out = Vec::new();
    for order in permutations(&members) {
        let latest: Vec<Option<String>> = recency
            .iter()
            .map(|(_, set)| order.iter().rev().find(|flag| set.contains(flag)).cloned())
            .collect();
        if !seen.contains(&latest) {
            seen.push(latest);
            out.push(order);
        }
    }
    out
}

fn permutations(items: &[String]) -> Vec<Vec<String>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for (at, item) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(at);
        for mut tail in permutations(&rest) {
            tail.insert(0, item.clone());
            out.push(tail);
        }
    }
    out
}

/// The sets `variant_products` gives (S17): every choice of one variant or
/// none from each `any` group and one from each `one` group, those of two
/// or more variants, less those holding two of an `exclusive_variants`
/// group or that `requires_variants` excludes.
fn product_sets(model: &Model) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    for product in &model.variant_products {
        let mut partial: Vec<Vec<String>> = vec![Vec::new()];
        for group in product {
            let mut next = Vec::new();
            for held in &partial {
                if !group.exactly_one {
                    next.push(held.clone());
                }
                for variant in &group.variants {
                    let mut with = held.clone();
                    with.push(variant.clone());
                    next.push(with);
                }
            }
            partial = next;
        }
        for set in partial {
            let distinct: BTreeSet<&String> = set.iter().collect();
            if set.len() < 2 || distinct.len() != set.len() {
                continue;
            }
            let two_of_a_group = model
                .exclusive_variants
                .iter()
                .any(|group| group.iter().filter(|variant| set.contains(variant)).count() > 1);
            if two_of_a_group || !required(model, &set) {
                continue;
            }
            let mut sorted = set;
            sorted.sort();
            if !out.contains(&sorted) {
                out.push(sorted);
            }
        }
    }
    out
}

/// Whether `set` holds one of the entry's `requires_variants`, where it
/// lists any.
fn required(model: &Model, set: &[String]) -> bool {
    model.requires_variants.is_empty()
        || model
            .requires_variants
            .iter()
            .any(|variant| set.contains(variant))
}

/// The entry's variant options (S17): none, each single variant, each
/// listed set and each set its products give, less those `requires_variants`
/// excludes and those holding a skipped variant, each once.
pub fn variant_options(model: &Model) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = vec![Vec::new()];
    out.extend(model.variants.iter().map(|variant| vec![variant.clone()]));
    for set in model
        .variant_sets
        .iter()
        .cloned()
        .chain(product_sets(model))
    {
        let mut sorted = set;
        sorted.sort();
        if !out.contains(&sorted) {
            out.push(sorted);
        }
    }
    out.retain(|set| {
        required(model, set)
            && !set
                .iter()
                .any(|variant| model.skipped_variants.contains(variant))
    });
    out
}

/// The sizes an option takes (S10's rule as the query applies it): the
/// first `sizes_by_variant` entry's whose variant it holds, else the
/// entry's `sizes`, less those the first `sizes_by_context` entry whose
/// selector the context's ancestors match leaves out; one `None` where it
/// takes none.
pub fn size_options(
    model: &Model,
    variants: &[String],
    context: Option<&Value>,
) -> Vec<Option<String>> {
    let sizes = model
        .sizes_by_variant
        .iter()
        .find(|(variant, _, _)| variants.contains(variant))
        .map_or(&model.sizes, |(_, sizes, _)| sizes);
    let kept = model
        .sizes_by_context
        .iter()
        .find(|(selector, _, _)| selector_matches(selector, context))
        .map(|(_, sizes, _)| sizes);
    let taken: Vec<Option<String>> = sizes
        .iter()
        .filter(|size| kept.is_none_or(|kept| kept.contains(size)))
        .cloned()
        .map(Some)
        .collect();
    if taken.is_empty() { vec![None] } else { taken }
}

/// One combination as data (S17), before the model reads it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub conditions: Vec<String>,
    pub context: Option<String>,
    pub variants: Vec<String>,
    pub states: Vec<String>,
    pub order: Vec<String>,
    pub size: Option<String>,
    pub density: Option<String>,
}

/// Whether a `not_drawn` entry's `when` names an element by its variants
/// and its context, for the count of an entry's combinations
/// ([`NOT_DRAWN_KEYS`], the only keys reading an entry lets it state): a
/// context matched as S7 matches one against the axis' context (each key it
/// states equal, each list it states held). The run matches every entry as
/// the query does.
fn names_element(when: &Table, variants: &[String], context: Option<&Value>) -> bool {
    let contexts = |key: &str| {
        when.get(key)
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
    };
    let matches = |wanted: &Value| {
        let (Some(wanted), Some(held)) = (wanted.as_table(), context.and_then(Value::as_table))
        else {
            return false;
        };
        wanted.iter().all(|(key, value)| match value {
            Value::Array(_) => {
                let listed = strings(held.get(key));
                strings(Some(value)).iter().all(|x| listed.contains(x))
            }
            other => held.get(key) == Some(other),
        })
    };
    strings(when.get("variants"))
        .iter()
        .all(|x| variants.contains(x))
        && !strings(when.get("not_variants"))
            .iter()
            .any(|x| variants.contains(x))
        && contexts("context").iter().all(matches)
        && !contexts("not_context").iter().any(matches)
}

/// Whether a `not_drawn` entry's `when` holds on the rest of a combination
/// its variants and context hold on ([`names_element`]): its states, its
/// conditions, and its size and density (`held`), each equal to every one a
/// list names, as S7's `all_of` holds on an element's one size.
fn names_rest(
    when: &Table,
    conditions: &[String],
    states: &[String],
    held: (Option<&str>, Option<&str>),
) -> bool {
    let (size, density) = held;
    let all = |key: &str, held: &[String]| strings(when.get(key)).iter().all(|x| held.contains(x));
    let none =
        |key: &str, held: &[String]| !strings(when.get(key)).iter().any(|x| held.contains(x));
    let one = |key: &str, held: Option<&str>| {
        strings(when.get(key))
            .iter()
            .all(|x| held == Some(x.as_str()))
    };
    let not_one = |key: &str, held: Option<&str>| {
        !strings(when.get(key))
            .iter()
            .any(|x| held == Some(x.as_str()))
    };
    all("states", states)
        && none("not_states", states)
        && one("sizes", size)
        && not_one("not_sizes", size)
        && one("densities", density)
        && not_one("not_densities", density)
        && all("conditions", conditions)
        && none("not_conditions", conditions)
}

/// What the rules and facts of a widget can tell apart (S17: "reusing what
/// the set's rules share"): the flags and variants any selector names, any
/// `not_drawn` entry names, any declaration that changes an element's
/// availability or size names, every member of an exclusive group or a
/// recency set, and the variants the preset does not declare, which make
/// an element unavailable. A combination's projection onto these resolves
/// every key as the combination does.
#[derive(Clone, Debug, Default)]
pub struct Mentioned {
    pub states: BTreeSet<String>,
    pub variants: BTreeSet<String>,
}

impl Mentioned {
    /// What the entry itself tells apart: its `not_drawn` entries' flags
    /// and variants, its exclusive groups' and recency sets' members, and
    /// the variants its size rules name.
    pub fn of_model(model: &Model) -> Mentioned {
        let mut out = Mentioned::default();
        for when in &model.not_drawn {
            out.add_when(&Value::Table(when.clone()));
        }
        out.states.extend(model.exclusive.iter().flatten().cloned());
        out.states.extend(
            model
                .recency
                .iter()
                .flat_map(|(_, set)| set.iter().cloned()),
        );
        out.variants.extend(
            model
                .sizes_by_variant
                .iter()
                .map(|(variant, _, _)| variant.clone()),
        );
        out
    }

    /// The flags and variants a fact's or rule's `when` names, its
    /// `latest` flags among them.
    pub fn add_when(&mut self, when: &Value) {
        for key in ["states", "not_states"] {
            self.states.extend(strings(when.get(key)));
        }
        for key in ["variants", "not_variants"] {
            self.variants.extend(strings(when.get(key)));
        }
        if let Some(latest) = when.get("latest").and_then(Value::as_table) {
            self.states
                .extend(latest.values().filter_map(Value::as_str).map(str::to_owned));
        }
    }

    fn states_of(&self, states: &[String]) -> Vec<String> {
        states
            .iter()
            .filter(|flag| self.states.contains(*flag))
            .cloned()
            .collect()
    }

    fn variants_of(&self, variants: &[String]) -> Vec<String> {
        variants
            .iter()
            .filter(|variant| self.variants.contains(*variant))
            .cloned()
            .collect()
    }
}

/// A variant option, its context and the sizes it takes there.
type VariantSizes<'m> = (Vec<String>, Option<&'m Value>, Vec<Option<String>>);

/// Each combination of `model` under the condition vectors `vectors` (S17),
/// projected by `mentioned` where it is given, given to `each` with its
/// context once per distinct combination; `requires_variants` and the size
/// rules applied, and, for the count alone (no `mentioned`, [`count`]),
/// `not_drawn`, which the run matches as the query does. The count of
/// combinations.
pub fn combinations(
    model: &Model,
    vectors: &[Vec<String>],
    mentioned: Option<&Mentioned>,
    mut each: impl FnMut(Point, Option<&Value>),
) -> u64 {
    let Some(mentioned) = mentioned else {
        return count(model, vectors);
    };
    let mut flag_sets: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    for states in state_sets(&model.states, &model.exclusive) {
        for order in orders(&states, &model.recency) {
            let pair = (mentioned.states_of(&states), mentioned.states_of(&order));
            if !flag_sets.contains(&pair) {
                flag_sets.push(pair);
            }
        }
    }
    let contexts: Vec<Option<&Value>> = std::iter::once(None)
        .chain(model.contexts.iter().map(Some))
        .collect();
    let spelled = |context: Option<&Value>| context.map(Value::to_string);
    let densities = densities(model);
    let mut variant_sizes: Vec<VariantSizes<'_>> = Vec::new();
    for context in &contexts {
        for variants in variant_options(model) {
            let sizes = size_options(model, &variants, *context);
            let entry = (mentioned.variants_of(&variants), *context, sizes);
            if !variant_sizes.contains(&entry) {
                variant_sizes.push(entry);
            }
        }
    }
    let mut count = 0_u64;
    for conditions in vectors {
        for (variants, context, sizes) in &variant_sizes {
            let written = spelled(*context);
            for (states, order) in &flag_sets {
                for size in sizes {
                    for density in &densities {
                        count += 1;
                        let point = Point {
                            conditions: conditions.clone(),
                            context: written.clone(),
                            variants: variants.clone(),
                            states: states.clone(),
                            order: order.clone(),
                            size: size.clone(),
                            density: density.clone(),
                        };
                        each(point, *context);
                    }
                }
            }
        }
    }
    count
}

/// None and each of the entry's densities.
fn densities(model: &Model) -> Vec<Option<String>> {
    std::iter::once(None)
        .chain(model.densities.iter().cloned().map(Some))
        .collect()
}

/// The number of `model`'s combinations under `vectors` (S17), less those a
/// `not_drawn` entry names: per variant option and context, the entries
/// naming that element ([`names_element`]) and the combinations of
/// conditions, flags (each set once per order), size and density none of
/// them names ([`names_rest`]), the latter counted once per distinct set of
/// such entries and sizes, as the options of a widget with many variant
/// products share a few.
fn count(model: &Model, vectors: &[Vec<String>]) -> u64 {
    let flag_sets: Vec<(Vec<String>, u64)> = state_sets(&model.states, &model.exclusive)
        .into_iter()
        .map(|states| {
            let orders = orders(&states, &model.recency).len() as u64;
            (states, orders)
        })
        .collect();
    let densities = densities(model);
    let mut contexts: Vec<Option<&Value>> = vec![None];
    for context in &model.contexts {
        if !contexts.contains(&Some(context)) {
            contexts.push(Some(context));
        }
    }
    let options = variant_options(model);
    let mut memo: HashMap<(Vec<usize>, Vec<Option<String>>), u64> = HashMap::new();
    let mut count = 0_u64;
    for context in &contexts {
        for variants in &options {
            let named: Vec<usize> = model
                .not_drawn
                .iter()
                .enumerate()
                .filter(|(_, when)| names_element(when, variants, *context))
                .map(|(at, _)| at)
                .collect();
            let sizes = size_options(model, variants, *context);
            count += *memo
                .entry((named, sizes))
                .or_insert_with_key(|(named, sizes)| {
                    let named: Vec<&Table> = named
                        .iter()
                        .filter_map(|at| model.not_drawn.get(*at))
                        .collect();
                    let mut drawn = 0_u64;
                    for conditions in vectors {
                        for (states, orders) in &flag_sets {
                            for size in sizes {
                                for density in &densities {
                                    let held = (size.as_deref(), density.as_deref());
                                    if !named
                                        .iter()
                                        .any(|when| names_rest(when, conditions, states, held))
                                    {
                                        drawn += orders;
                                    }
                                }
                            }
                        }
                    }
                    drawn
                });
        }
    }
    count
}
