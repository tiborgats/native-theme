//! A layer's file (S11): its bases, its `for_preset` parts and its rules,
//! and a preset's header and declarations.

use ::toml::{Table, Value as Item};

use super::declare::read_declarations;
use super::model::{part_named, widget_named, widget_or_part};
use super::read::{self, FromToml, Reader, Spelling, join, table, text};
use super::when::read_selector;
use crate::next::blocks::part_style::PartStylePatch;
use crate::next::color::Color;
use crate::next::declare::Declarations;
use crate::next::error::{LayerError, Problem, ProblemKind};
use crate::next::inputs::{Contrast, InputBases, InputName, InputValue};
use crate::next::layer::{Bases, Layer, Spelled};
use crate::next::presets::PresetId;
use crate::next::registry;
use crate::next::roles::{ColorRolesPatch, SurfaceSet};
use crate::next::selector::{
    PartRef, Rule, RuleBody, Scheme, Selector, ValuesPatch, VariantRef, WidgetOrPart,
};
use crate::next::tokens::TokensPatch;
use crate::next::value::{SchemeMap, Value};
use crate::next::widgets::{PartPatch, WidgetKind, WidgetValuesPatch};

/// A typed widget's patch, as the `ThemeParts` derive writes it: a slot per
/// part and one for its values.
pub(crate) trait WidgetPatch: Default {
    /// The values' patch.
    type Values: FromToml;

    /// The widget.
    const KIND: WidgetKind;

    /// The slot of the part `name` names.
    fn part_slot(&mut self, name: &str) -> Option<&mut Option<PartPatch>>;

    /// The values' slot.
    fn values_slot(&mut self) -> &mut Option<Self::Values>;
}

/// The application layer `text` states (S11), or every problem it has.
pub(crate) fn read_layer(text: &str) -> Result<Layer, LayerError> {
    let file = parse(text)?;
    let mut cx = Reader::default();
    let mut layer = Layer::default();
    for (key, item) in &file {
        match key.as_str() {
            "rules" => layer.rules = read_rules(item, key, &mut cx),
            "for_preset" => layer.for_preset = read_for_preset(item, key, &mut cx),
            _ => {
                if !read_base(&mut layer.bases, key, item, key, false, &mut cx) {
                    cx.problem(key, ProblemKind::UnknownKey);
                }
            }
        }
    }
    finished(layer, cx)
}

/// A preset's file (S11): its header, its declarations (S10) and its layer,
/// whose bases state whole blocks (S2.2).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PresetFile {
    /// `[preset] id`: S16's `PresetId` key.
    pub(crate) id: String,
    /// `[preset] generation`: the platform generation the facts were read
    /// from.
    pub(crate) generation: String,
    /// `[declare]`.
    pub(crate) declarations: Declarations,
    /// The bases and rules.
    pub(crate) layer: Layer,
    /// `[pending]`: each listed path as S11 spells it, with its register id
    /// (S17, ADD-A128).
    pub(crate) pending: Vec<(String, String)>,
}

/// The preset `text` states (S11), or every problem it has.
pub(crate) fn read_preset(text: &str) -> Result<PresetFile, LayerError> {
    let file = parse(text)?;
    let mut cx = Reader::default();
    let mut layer = Layer::default();
    let mut header = None;
    let mut declarations = None;
    // The listed paths are read first: a block built whole may leave them
    // out (S17, ADD-A128).
    let pending = file
        .get("pending")
        .map(|item| read_pending(item, "pending", &mut cx))
        .unwrap_or_default();
    cx.listed = pending
        .iter()
        .map(|(path, _)| read::listed_path(path))
        .collect();
    for (key, item) in &file {
        match key.as_str() {
            "preset" => header = read_header(item, key, &mut cx),
            "declare" => declarations = read_declarations(item, key, &mut cx),
            "pending" => {}
            "rules" => layer.rules = read_rules(item, key, &mut cx),
            _ => {
                if !read_base(&mut layer.bases, key, item, key, true, &mut cx) {
                    cx.problem(key, ProblemKind::UnknownKey);
                }
            }
        }
    }
    for (path, _) in &pending {
        if crate::next::validate::states(&layer, &read::listed_path(path)) {
            cx.problem(
                &join("pending", path),
                ProblemKind::DuplicateKey {
                    other: path.clone(),
                },
            );
        }
    }
    let header = read::required(header, "preset", &mut cx);
    let declarations = declarations.unwrap_or_default();
    let layer = finished(layer, cx)?;
    match header {
        Some((id, generation)) => Ok(PresetFile {
            id,
            generation,
            declarations,
            layer,
            pending,
        }),
        None => Err(LayerError::Invalid(vec![Problem::new(
            "preset",
            ProblemKind::Missing,
        )])),
    }
}

/// `[pending]` (S11, S17): each listed path, its TOML path quoted, with its
/// register id, `U` and a number (ADD-A122's form); an entry of another
/// form is refused.
fn read_pending(item: &Item, at: &str, cx: &mut Reader) -> Vec<(String, String)> {
    let Some(entries) = table(item, at, cx) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (path, id) in entries {
        let at = join(at, path);
        let Some(id) = id.as_str() else {
            cx.problem(
                &at,
                ProblemKind::Mistyped {
                    expected: "a register id, `U<n>`",
                },
            );
            continue;
        };
        let register = id
            .strip_prefix('U')
            .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()));
        if !register {
            cx.problem(
                &at,
                ProblemKind::Mistyped {
                    expected: "a register id, `U<n>`",
                },
            );
            continue;
        }
        out.push((path.clone(), id.to_owned()));
    }
    out
}

/// `text` as a TOML table, or its parse error.
fn parse(text: &str) -> Result<Table, LayerError> {
    text.parse::<Table>().map_err(|error| {
        LayerError::Invalid(vec![Problem::new(
            "",
            ProblemKind::Toml(error.message().to_owned()),
        )])
    })
}

/// The layer read, or every problem its reading recorded.
fn finished(mut layer: Layer, cx: Reader) -> Result<Layer, LayerError> {
    if cx.failed() {
        Err(LayerError::Invalid(cx.into_problems()))
    } else {
        layer.spelled = Spelled::Toml;
        layer.shorthands = cx.shorthands;
        layer.shorthands.read_rules(&layer.rules);
        Ok(layer)
    }
}

/// `[preset]`: its `id` and `generation`.
fn read_header(item: &Item, at: &str, cx: &mut Reader) -> Option<(String, String)> {
    let fields = table(item, at, cx)?;
    let mut id = None;
    let mut generation = None;
    for (key, item) in fields {
        let at = join(at, key);
        match key.as_str() {
            "id" => id = String::read(item, Spelling::Bare, &at, cx),
            "generation" => generation = String::read(item, Spelling::Bare, &at, cx),
            _ => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    let id = read::required(id, &join(at, "id"), cx);
    let generation = read::required(generation, &join(at, "generation"), cx);
    Some((id?, generation?))
}

/// A top-level key of a layer's bases, at `at`, into `bases`; `false` where
/// `key` is none of theirs. `whole`: a preset's own bases, whose tables
/// state whole blocks (S2.2).
fn read_base(
    bases: &mut Bases,
    key: &str,
    item: &Item,
    at: &str,
    whole: bool,
    cx: &mut Reader,
) -> bool {
    cx.bases = true;
    cx.whole = whole;
    match key {
        "colors" => bases.colors = read_colors(item, at, cx),
        "inputs" => bases.inputs = read_inputs(item, at, cx),
        "tokens" => {
            if let Some(tokens) = TokensPatch::read(item, Spelling::Bare, at, cx) {
                bases.tokens = tokens;
            }
        }
        "widgets" => {
            if let Some(widgets) = table(item, at, cx) {
                for (name, item) in widgets {
                    let at = join(at, name);
                    if !bases.widgets.read_widget(name, item, &at, cx) {
                        cx.problem(&at, ProblemKind::UnknownKey);
                    }
                }
            }
        }
        _ => {
            cx.bases = false;
            cx.whole = false;
            return false;
        }
    }
    cx.bases = false;
    cx.whole = false;
    true
}

/// `[colors.<scheme>]`: the colour roles per scheme.
fn read_colors(item: &Item, at: &str, cx: &mut Reader) -> SchemeMap<ColorRolesPatch> {
    let mut colors = SchemeMap::default();
    let Some(schemes) = table(item, at, cx) else {
        return colors;
    };
    for (name, item) in schemes {
        let at = join(at, name);
        let roles = ColorRolesPatch::read(item, Spelling::Bare, &at, cx);
        match (<Scheme as read::Named>::from_name(name), roles) {
            (Some(Scheme::Light), roles) => colors.light = roles,
            (Some(Scheme::Dark), roles) => colors.dark = roles,
            (None, _) => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    colors
}

/// `[inputs.<name>]`: each input's value per scheme and per contrast, and
/// KDE's scheme lists (S9, S11).
fn read_inputs(item: &Item, at: &str, cx: &mut Reader) -> InputBases {
    let mut inputs = InputBases::default();
    if let Some(names) = table(item, at, cx) {
        read_input_names(names, &mut Vec::new(), at, &mut inputs, cx);
    }
    inputs
}

/// The inputs an `[inputs]` table states (S11), as a layer's bases read
/// them, or every problem reading them.
pub(crate) fn read_inputs_table(item: &Item) -> Result<InputBases, Vec<Problem>> {
    let mut cx = Reader::default();
    let inputs = read_inputs(item, "inputs", &mut cx);
    if cx.failed() {
        Err(cx.into_problems())
    } else {
        Ok(inputs)
    }
}

/// The keys an input's value is stated by.
const INPUT_VALUE_KEYS: [&str; 3] = ["light", "dark", "contrast"];

/// The inputs below the path `path`, at `at`.
fn read_input_names(
    names: &Table,
    path: &mut Vec<String>,
    at: &str,
    inputs: &mut InputBases,
    cx: &mut Reader,
) {
    for (name, item) in names {
        let at = join(at, name);
        if path.len() == 1
            && path.first().is_some_and(|first| first == "kde_scheme")
            && let Some(list) = KdeList::named(name)
        {
            read_kde_list(list, item, &at, inputs, cx);
            continue;
        }
        let Some(fields) = table(item, &at, cx) else {
            continue;
        };
        path.push(name.clone());
        if fields
            .keys()
            .any(|key| INPUT_VALUE_KEYS.contains(&key.as_str()))
        {
            let input = InputName::new(&path.join("."));
            match input {
                Ok(input) => {
                    if let Some(value) = read_input_value(fields, &at, cx) {
                        inputs.entries.push((input, value));
                    }
                }
                Err(error) => cx.problem(&at, ProblemKind::NotAKey(error)),
            }
        } else {
            read_input_names(fields, path, &at, inputs, cx);
        }
        path.pop();
    }
}

/// One input's value: `light`, `dark`, and `contrast.<increased|high>`'s
/// `light` and `dark` (S11).
fn read_input_value(fields: &Table, at: &str, cx: &mut Reader) -> Option<InputValue> {
    let mut per_scheme = None;
    let mut per_contrast = Vec::new();
    let mut schemes = Table::new();
    for (key, item) in fields {
        match key.as_str() {
            "light" | "dark" => {
                schemes.insert(key.clone(), item.clone());
            }
            "contrast" => {
                let at = join(at, key);
                let Some(contrasts) = table(item, &at, cx) else {
                    continue;
                };
                for (name, item) in contrasts {
                    let at = join(&at, name);
                    let contrast = read::named::<Contrast>(&Item::String(name.clone()), &at, cx);
                    let values =
                        table(item, &at, cx).map(|values| per_scheme_colors(values, &at, cx));
                    if let (Some(contrast), Some(values)) = (contrast, values) {
                        per_contrast.push((contrast, values));
                    }
                }
            }
            _ => cx.problem(&join(at, key), ProblemKind::UnknownKey),
        }
    }
    if !schemes.is_empty() {
        per_scheme = Some(per_scheme_colors(&schemes, at, cx));
    }
    let mut value = InputValue::new(per_scheme.unwrap_or_default());
    value.per_contrast = per_contrast;
    Some(value)
}

/// A table of `light` and `dark` colours.
fn per_scheme_colors(fields: &Table, at: &str, cx: &mut Reader) -> SchemeMap<Value<Color>> {
    let mut map = SchemeMap::default();
    for (key, item) in fields {
        let at = join(at, key);
        let value = Value::<Color>::read(item, Spelling::Bare, &at, cx);
        match key.as_str() {
            "light" => map.light = value,
            "dark" => map.dark = value,
            _ => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    map
}

/// KDE's scheme lists, `input.kde_scheme.<list>` (S9.1).
#[derive(Clone, Copy)]
enum KdeList {
    Groups,
    InactiveGroups,
    UnsetKeys,
}

impl KdeList {
    fn named(name: &str) -> Option<KdeList> {
        match name {
            "groups" => Some(KdeList::Groups),
            "inactive_groups" => Some(KdeList::InactiveGroups),
            "unset_keys" => Some(KdeList::UnsetKeys),
            _ => None,
        }
    }
}

/// `[inputs.kde_scheme.<list>]`, `light` and `dark` each an array.
fn read_kde_list(list: KdeList, item: &Item, at: &str, inputs: &mut InputBases, cx: &mut Reader) {
    let Some(schemes) = table(item, at, cx) else {
        return;
    };
    for (name, item) in schemes {
        let at = join(at, name);
        let Some(scheme) = <Scheme as read::Named>::from_name(name) else {
            cx.problem(&at, ProblemKind::UnknownKey);
            continue;
        };
        match list {
            KdeList::Groups => {
                if let Some(sets) = Vec::<SurfaceSet>::read(item, Spelling::Bare, &at, cx) {
                    set_scheme(&mut inputs.kde_groups, scheme, sets);
                }
            }
            KdeList::InactiveGroups => {
                if let Some(sets) = Vec::<SurfaceSet>::read(item, Spelling::Bare, &at, cx) {
                    set_scheme(&mut inputs.kde_inactive_groups, scheme, sets);
                }
            }
            KdeList::UnsetKeys => {
                if let Some(names) = read::array(item, &at, cx) {
                    let names = names
                        .iter()
                        .enumerate()
                        .filter_map(|(index, item)| {
                            let at = read::item(&at, index);
                            let name = text(item, &at, cx)?;
                            InputName::new(name)
                                .map_err(|error| cx.problem(&at, ProblemKind::NotAKey(error)))
                                .ok()
                        })
                        .collect();
                    set_scheme(&mut inputs.kde_unset_keys, scheme, names);
                }
            }
        }
    }
}

fn set_scheme<T>(map: &mut SchemeMap<T>, scheme: Scheme, value: T) {
    match scheme {
        Scheme::Light => map.light = Some(value),
        Scheme::Dark => map.dark = Some(value),
    }
}

/// A typed widget's table into its patch: each part's table, and `values`.
pub(crate) fn read_widget<P: WidgetPatch>(
    slot: &mut Option<P>,
    item: &Item,
    at: &str,
    cx: &mut Reader,
) {
    let Some(fields) = table(item, at, cx) else {
        return;
    };
    let patch = slot.get_or_insert_with(P::default);
    let outer = cx.widget.replace(P::KIND);
    for (key, item) in fields {
        let at = join(at, key);
        if key == "values" {
            if let Some(values) = P::Values::read(item, Spelling::Bare, &at, cx) {
                *patch.values_slot() = Some(values.finish(&at, cx));
            }
        } else if let Some(part) = patch.part_slot(key) {
            if let Some(read) = read_part(P::KIND, key, item, &at, cx) {
                *part = Some(read);
            }
        } else {
            cx.problem(&at, ProblemKind::UnknownKey);
        }
    }
    cx.widget = outer;
}

/// A part's table (S6.1, S11): `absent = true`, a `drawn_as` table, or its
/// style's fields, only those its catalogue entry lists (S2.11).
fn read_part(
    widget: WidgetKind,
    part: &str,
    item: &Item,
    at: &str,
    cx: &mut Reader,
) -> Option<PartPatch> {
    let fields = table(item, at, cx)?;
    let outer = cx.widget.replace(widget);
    let read = if let Some(absent) = fields.get("absent") {
        others_unknown(fields, "absent", at, cx);
        match absent {
            Item::Boolean(true) => Some(PartPatch::Absent),
            _ => cx.mistyped(&join(at, "absent"), "`true`: the whole part is not drawn"),
        }
    } else if let Some(drawn_as) = fields.get("drawn_as") {
        others_unknown(fields, "drawn_as", at, cx);
        read_drawn_as(drawn_as, &join(at, "drawn_as"), cx)
    } else {
        Some(PartPatch::Style(read_style(widget, part, fields, at, cx)))
    };
    cx.widget = outer;
    read
}

/// Every key of `fields` but `kept` refused: an absent or drawn-as part
/// states nothing else.
fn others_unknown(fields: &Table, kept: &str, at: &str, cx: &mut Reader) {
    for key in fields.keys().filter(|key| *key != kept) {
        cx.problem(&join(at, key), ProblemKind::UnknownKey);
    }
}

/// A part's style fields: each key a field of the part's catalogue list.
fn read_style(
    widget: WidgetKind,
    part: &str,
    fields: &Table,
    at: &str,
    cx: &mut Reader,
) -> PartStylePatch {
    let listed = registry::widgets()
        .iter()
        .find(|info| info.kind == widget)
        .and_then(|info| info.part(part).map(|part| (info.name, part)));
    let mut patch = PartStylePatch::default();
    for (key, item) in fields {
        let key_at = join(at, key);
        let field = <PartStylePatch as read::Fields>::field_of(key);
        match (field, listed) {
            (None, _) | (_, None) => cx.problem(&key_at, ProblemKind::UnknownKey),
            (Some(field), Some((widget, info))) => {
                if info.fields.iter().any(|listed| listed.path == field) {
                    read::Fields::read_entry(&mut patch, key, item, at, cx);
                } else {
                    cx.problem(
                        &key_at,
                        ProblemKind::NotAPartField {
                            part: format!("{widget}.{part}"),
                        },
                    );
                }
            }
        }
    }
    read::Fields::finish_fields(&mut patch, at, cx);
    patch
}

/// `drawn_as = { widget, part, variants }` (S6.1, S11).
fn read_drawn_as(item: &Item, at: &str, cx: &mut Reader) -> Option<PartPatch> {
    let fields = table(item, at, cx)?;
    let mut variants = None;
    for (key, item) in fields {
        let key_at = join(at, key);
        match key.as_str() {
            "widget" | "part" => {}
            "variants" => variants = Vec::<VariantRef>::read(item, Spelling::Bare, &key_at, cx),
            _ => cx.problem(&key_at, ProblemKind::UnknownKey),
        }
    }
    let target = widget_or_part(fields.get("widget"), fields.get("part"), at, cx);
    let target: Option<WidgetOrPart> = match target {
        Some(None) => read::required(None, &join(at, "widget"), cx),
        Some(Some(target)) => Some(target),
        None => None,
    };
    let variants = read::required(variants, &join(at, "variants"), cx);
    Some(PartPatch::DrawnAs {
        target: target?,
        variants: variants?,
    })
}

/// `[[rules]]`: each rule's `when`, `target` and `set` (S11).
fn read_rules(item: &Item, at: &str, cx: &mut Reader) -> Vec<Rule> {
    let Some(items) = read::array(item, at, cx) else {
        return Vec::new();
    };
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| read_rule(item, &read::item(at, index), cx))
        .collect()
}

/// One rule.
fn read_rule(item: &Item, at: &str, cx: &mut Reader) -> Option<Rule> {
    let fields = table(item, at, cx)?;
    for key in fields.keys() {
        if !["when", "target", "set"].contains(&key.as_str()) {
            cx.problem(&join(at, key), ProblemKind::UnknownKey);
        }
    }
    let selector = match fields.get("when") {
        Some(when) => read_selector(when, &join(at, "when"), cx),
        None => Some(Selector::default()),
    };
    let target = read::required(fields.get("target"), &join(at, "target"), cx);
    let set = read::required(fields.get("set"), &join(at, "set"), cx);
    let body = match (target, set) {
        (Some(target), Some(set)) => read_body(target, set, at, cx),
        _ => None,
    };
    Some(Rule::new(selector?, body?))
}

/// A rule's body from its target and its `set` (S11), as a rule states
/// them, or every problem reading them.
pub(crate) fn read_rule_body(target: &str, set: &Item) -> Result<RuleBody, Vec<Problem>> {
    let mut cx = Reader::default();
    let target = Item::String(target.to_owned());
    let body = read_body(&target, set, "", &mut cx);
    match body {
        Some(body) if !cx.failed() => Ok(body),
        _ => Err(cx.into_problems()),
    }
}

/// A rule's target and its patch (S11): `colors`, `tokens`,
/// `<widget>.<part>`, `<widget>.values`.
fn read_body(target: &Item, set: &Item, at: &str, cx: &mut Reader) -> Option<RuleBody> {
    let target_at = join(at, "target");
    let set_at = join(at, "set");
    let name = text(target, &target_at, cx)?;
    match name.split_once('.') {
        None if name == "colors" => {
            ColorRolesPatch::read(set, Spelling::Bare, &set_at, cx).map(RuleBody::Colors)
        }
        None if name == "tokens" => {
            TokensPatch::read(set, Spelling::Bare, &set_at, cx).map(RuleBody::Tokens)
        }
        Some(("component", _)) => cx.unknown(&target_at, name),
        Some((widget, rest)) if !rest.contains('.') => {
            let widget = widget_named(widget, &target_at, cx)?;
            if rest == "values" {
                let outer = cx.widget.replace(widget);
                let values = WidgetValuesPatch::read(widget, set, &set_at, cx);
                cx.widget = outer;
                return values.map(|values| RuleBody::Values(ValuesPatch::Typed(values)));
            }
            let part = part_named(widget, rest, &target_at, cx)?;
            read_part(widget, rest, set, &set_at, cx)
                .map(|patch| RuleBody::Part(PartRef::Typed(part), patch))
        }
        _ => cx.mistyped(
            &target_at,
            "\"colors\", \"tokens\", \"<widget>.<part>\" or \"<widget>.values\"",
        ),
    }
}

/// `[for_preset.<id>]`: a preset's own bases (S11).
fn read_for_preset(item: &Item, at: &str, cx: &mut Reader) -> Vec<(PresetId, Bases)> {
    let mut out = Vec::new();
    let Some(presets) = table(item, at, cx) else {
        return out;
    };
    for (id, item) in presets {
        let at = join(at, id);
        let preset = PresetId::from_key(id);
        if preset.is_none() {
            cx.problem(&at, ProblemKind::UnknownKey);
        }
        let mut bases = Bases::default();
        if let Some(fields) = table(item, &at, cx) {
            for (key, item) in fields {
                if !read_base(&mut bases, key, item, &join(&at, key), false, cx) {
                    cx.problem(&join(&at, key), ProblemKind::UnknownKey);
                }
            }
        }
        if let Some(preset) = preset {
            out.push((preset, bases));
        }
    }
    out
}
