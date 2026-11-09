//! The facts gate's way into the model (S17). The gate
//! (`native-theme/tests/facts_gate.rs`) reads the facts and the state
//! models, enumerates each widget's combinations and reports; this module
//! answers what only the model can, as the query does: a preset resolved
//! with a stand-in under a combination's conditions, whether a fact's
//! `when` holds for an element, and a key's value beside a fact's value
//! evaluated in the key's place, over the same operands in the same scope.

mod tree;

use std::cell::{OnceCell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex, PoisonError};

use ::toml::{Table, Value as Item};

pub use tree::{Drawing, Tolerance, Tree};

use super::blocks::part_style::PartStylePatch;
use super::declare::{DefaultSize, WidgetDecl};
use super::error::{LayerError, Problem, ProblemKind};
use super::expr::{Expr, Ref, Ty};
use super::inputs::{Contrast, InputBases, InputName};
use super::key::Key;
use super::layer::Layer;
use super::presets::PresetId;
use super::query::{
    Element, FontMetrics, Matched, RootInputs, Scope, Sibling, Styled, Unavailable,
};
use super::registry;
use super::resolve::compose::{Fold, Found, Target};
use super::resolve::evaluate::{Extra, Reached, Resolve};
use super::resolve::{
    Pos, ReaderState, ResolvedTheme, Stack, WindowConditions, WindowState, input_value,
};
use super::roles::ColorRoles;
use super::selector::{
    Condition, ContextMatch, ContextScope, ControlSize, Density, Direction, InputKind, PartRef,
    RuleBody, Scheme, Selector, StateFlag, StateFlags, ValuesPatch, VariantRef, WidgetOrPart,
    WidgetRef,
};
use super::tokens::Tokens;
use super::toml::layer::{read_inputs_table, read_preset, read_rule_body};
use super::toml::read::{Named, Reader};
use super::toml::when;
use super::widgets::{
    ButtonValues, PartPatch, PartPresence, TypedPart, WidgetKind, WidgetValuesPatch,
};

/// Why the gate cannot read a fact's key, `when` or value: a name or a key
/// the model does not hold yet, which the gate skips while 0.8.0 is built
/// (S17), or a fact the model's reader refuses.
#[derive(Clone, Debug, PartialEq)]
pub enum Unread {
    /// The model does not hold it yet.
    NotHeld(String),
    /// The model's reader refuses it.
    Refused(String),
}

impl Unread {
    /// The problems of a reading: not held where every one names a key or
    /// name the model does not hold, else refused.
    fn of(problems: &[Problem]) -> Unread {
        let text = problems
            .iter()
            .map(Problem::to_string)
            .collect::<Vec<_>>()
            .join("; ");
        let unknown = !problems.is_empty()
            && problems.iter().all(|problem| {
                matches!(
                    problem.kind,
                    ProblemKind::UnknownKey | ProblemKind::UnknownName { .. }
                )
            });
        if unknown {
            Unread::NotHeld(text)
        } else {
            Unread::Refused(text)
        }
    }
}

/// A refused change's text, each problem named.
fn refused(error: &LayerError) -> String {
    match error {
        LayerError::Invalid(problems) => problems
            .iter()
            .map(Problem::to_string)
            .collect::<Vec<_>>()
            .join("; "),
        other => other.to_string(),
    }
}

/// Why a preset does not load ([`load`]).
#[derive(Debug)]
pub enum Refusal {
    /// The reader's or validation's refusal, each problem in its own words.
    Layer(LayerError),
    /// The synthetic route refuses a listed path of the `[pending]` table,
    /// naming it ([`synthetic_layers`]).
    Synthetic(String),
}

/// Loads the preset `text` states under `scheme` as [`Preset`] loads one
/// and `Theme::from_preset_synthetic` a bundled one: through the synthetic
/// route where its `[pending]` table lists paths (S17, ADD-A128), as the
/// bundled preset its `id` names, if one does, so its `preset:` conditions
/// hold; else as a preset no `PresetId` names. The `preset-check` tool's
/// way in (`tools/preset-check`): the preset id it loads as, or every
/// problem.
pub fn load(text: &str, scheme: Scheme) -> Result<Option<PresetId>, Refusal> {
    let file = read_preset(text).map_err(Refusal::Layer)?;
    let id = PresetId::from_key(&file.id);
    let stack = if file.pending.is_empty() {
        Stack::new(id, file.declarations, file.layer, scheme, file.pending)
    } else {
        let layers = synthetic_layers(&file.pending, &file.layer).map_err(Refusal::Synthetic)?;
        Stack::synthetic(
            id,
            file.declarations,
            file.layer,
            scheme,
            file.pending,
            layers,
        )
    };
    stack.map(|_| id).map_err(Refusal::Layer)
}

/// A preset the gate checks: its stack, validated as `Theme` validates a
/// bundled preset (S12); where its `[pending]` table lists paths, through
/// the synthetic route, its synthetic layers' steps recorded wherever an
/// evaluation reads one (S17, ADD-A128).
#[derive(Clone, Debug)]
pub struct Preset {
    stack: Stack,
    id: String,
    pending: Vec<(String, Vec<String>)>,
    /// The listed paths its own bases state, read before the synthetic
    /// route folds a listed input into them (S17, ADD-A132).
    stated: Vec<String>,
}

impl Preset {
    /// The bundled preset `id`.
    pub fn bundled(id: PresetId) -> Result<Preset, String> {
        Preset::read(id.source(), Some(id))
    }

    /// A preset in S11's format that no `PresetId` names (a fixture's), so
    /// no `preset:` condition holds for it.
    pub fn from_toml(text: &str) -> Result<Preset, String> {
        Preset::read(text, None)
    }

    fn read(text: &str, id: Option<PresetId>) -> Result<Preset, String> {
        let file = read_preset(text).map_err(|error| refused(&error))?;
        let pending = file.pending.clone();
        let stated = pending
            .iter()
            .map(|(path, _)| super::toml::read::listed_path(path))
            .filter(|listed| super::validate::states(&file.layer, listed))
            .collect();
        let stack = if file.pending.is_empty() {
            Stack::new(
                id,
                file.declarations,
                file.layer,
                Scheme::Light,
                file.pending,
            )
        } else {
            let layers = synthetic_layers(&file.pending, &file.layer)?;
            Stack::synthetic(
                id,
                file.declarations,
                file.layer,
                Scheme::Light,
                file.pending,
                layers,
            )
        };
        Ok(Preset {
            stack: stack.map_err(|error| refused(&error))?,
            id: file.id,
            pending,
            stated,
        })
    }

    /// Whether the preset's own bases state the listed path `path` (as its
    /// `[pending]` table spells it), or a value holding it whole, as its
    /// file states them: a path it states is no listed path (S17,
    /// ADD-A128).
    #[must_use]
    pub fn states(&self, path: &str) -> bool {
        self.stated.contains(&super::toml::read::listed_path(path))
    }

    /// Whether the preset's declarations state the default size the
    /// listed declaration `path` names (as its `[pending]` table spells
    /// it): a pending default is stated nowhere (S10's *A pending default*,
    /// S17, ADD-A131).
    #[must_use]
    pub fn states_default(&self, path: &str) -> bool {
        let Some(listed) = super::toml::read::listed_default(path) else {
            return false;
        };
        let mut declarations = self.stack.declarations().clone();
        matches!(
            super::toml::layer::default_slot(&mut declarations, &listed),
            Some(Some(DefaultSize::Size(_)))
        )
    }

    /// Its `[pending]` table: each listed path as S11 spells it, with its
    /// register ids (S17, ADD-A128).
    #[must_use]
    pub fn pending(&self) -> &[(String, Vec<String>)] {
        &self.pending
    }

    /// The preset's `id`.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The flavours it declares, its default first (S10).
    #[must_use]
    pub fn flavors(&self) -> Vec<String> {
        self.stack
            .declarations()
            .flavors
            .iter()
            .map(|flavor| flavor.as_str().to_owned())
            .collect()
    }

    /// The schemes it declares; both where it declares none (S10).
    #[must_use]
    pub fn schemes(&self) -> Vec<String> {
        let declared = &self.stack.declarations().schemes;
        let schemes: &[Scheme] = if declared.is_empty() {
            &Scheme::ALL
        } else {
            declared
        };
        schemes
            .iter()
            .map(|scheme| scheme.name().to_owned())
            .collect()
    }

    /// The inputs it declares, each as a reference names it below `input.`.
    #[must_use]
    pub fn inputs(&self) -> Vec<String> {
        self.stack
            .declarations()
            .inputs
            .iter()
            .map(InputName::to_string)
            .collect()
    }

    /// A measured derivation's pass layer `layer` (S17 *A measured
    /// derivation*) with the preset's accent inputs as one group (S9.2):
    /// where it states any of them, each other one with the preset's own
    /// value, every operand the pass does not vary resolving from the
    /// preset; else as it is. With the inputs it adds so.
    fn with_accent_group(&self, mut layer: Layer) -> (Layer, Vec<InputName>) {
        let group = &self.stack.declarations().accent_inputs;
        let entries = &mut layer.bases.inputs.entries;
        let mut carried = Vec::new();
        if !entries.iter().any(|(name, _)| group.contains(name)) {
            return (layer, carried);
        }
        for name in group {
            if entries.iter().any(|(held, _)| held == name) {
                continue;
            }
            let own = self
                .stack
                .preset_layer()
                .bases
                .inputs
                .entries
                .iter()
                .find(|(held, _)| held == name);
            if let Some((_, value)) = own {
                entries.push((name.clone(), value.clone()));
                carried.push(name.clone());
            }
        }
        (layer, carried)
    }

    /// The declarations of the widget `widget` in force under `flavor`, else
    /// under the default flavour (S10).
    pub fn declared(&self, widget: &str, flavor: Option<&str>) -> Result<Declared, Unread> {
        let kind = widget_kind(widget)?;
        let declarations = self.stack.declarations();
        let decl = match flavor {
            Some(flavor) => {
                let flavor =
                    Key::new(flavor).map_err(|error| Unread::Refused(error.to_string()))?;
                declarations.in_flavor(kind, &flavor)
            }
            None => declarations.in_default_flavor(kind),
        };
        Ok(Declared::of(&decl))
    }

    /// The flags and the variants the preset's rules on the widget `widget`
    /// name, its parts' and its values', each as S11 writes it: what its
    /// rules can tell an element apart by (S17).
    #[must_use]
    pub fn mentions(&self, widget: &str) -> (Vec<String>, Vec<String>) {
        let Ok(kind) = widget_kind(widget) else {
            return (Vec::new(), Vec::new());
        };
        let (mut states, mut variants) = (Vec::new(), Vec::new());
        for rule in &self.stack.preset_layer().rules {
            let on_widget = match &rule.body {
                RuleBody::Part(PartRef::Typed(part), _) => part.widget() == kind,
                RuleBody::Values(ValuesPatch::Typed(values)) => match values {
                    WidgetValuesPatch::Button(_) => kind == WidgetKind::Button,
                },
                RuleBody::Part(PartRef::Component { .. }, _)
                | RuleBody::Colors(_)
                | RuleBody::Tokens(_) => false,
            };
            if !on_widget {
                continue;
            }
            let selector = &rule.selector;
            let flags = selector
                .states
                .all_of
                .iter()
                .chain(&selector.states.none_of);
            states.extend(flags.map(StateFlag::name));
            states.extend(selector.latest.iter().map(|(_, flag)| flag.name()));
            let named = selector
                .variants
                .all_of
                .iter()
                .chain(&selector.variants.none_of);
            variants.extend(named.map(variant_name));
        }
        (states, variants)
    }

    /// The preset resolved with a stand-in stating `stand_in` in
    /// `conditions` and no reader (S17), validated as `Theme` validates the
    /// stand-in: one stating some of the preset's accent inputs at a scheme
    /// is refused, naming those it lacks (S9.2).
    pub fn resolve(&self, conditions: &Conditions, stand_in: &Layer) -> Result<Resolved, String> {
        self.resolve_carrying(conditions, stand_in.clone(), Vec::new())
    }

    /// [`Preset::resolve`] with a measured derivation's pass layer `pass`
    /// as the stand-in (S17 *A measured derivation*): where it states any
    /// of the preset's accent inputs, each other one with the preset's own
    /// value, the group stated as one (S9.2) and every operand the pass does
    /// not vary resolving from the preset; the preset's own values it
    /// carries are the preset's, a folded synthetic one still marked
    /// (ADD-A132). Only a pass's layer is completed so.
    pub fn resolve_pass(&self, conditions: &Conditions, pass: &Layer) -> Result<Resolved, String> {
        let (pass, carried) = self.with_accent_group(pass.clone());
        self.resolve_carrying(conditions, pass, carried)
    }

    /// The preset resolved with the stand-in `stand_in` in `conditions`,
    /// carrying the preset's own values of the inputs `carried`.
    fn resolve_carrying(
        &self,
        conditions: &Conditions,
        stand_in: Layer,
        carried: Vec<InputName>,
    ) -> Result<Resolved, String> {
        let mut stack = self.stack.clone();
        if let Some(flavor) = &conditions.flavor {
            stack
                .set_flavor(flavor.clone())
                .map_err(|error| refused(&error))?;
        }
        stack
            .set_reader_stand_in_carrying(conditions.state.clone(), stand_in, carried)
            .map_err(|error| refused(&error))?;
        let theme = stack
            .resolve(&mut WindowState::default(), &conditions.window)
            .theme;
        Ok(Resolved {
            theme,
            stack,
            window: conditions.window.clone(),
            contexts: Arc::default(),
        })
    }
}

/// The fact key a listed path names (S11's `[pending]` key as S17's key
/// spells it: `tokens.behavior.touch.delay_ms` is
/// `token.behavior.touch.delay`, `widgets.button.icon.icon.size_px` is
/// `button.icon.icon.size`, `colors.light.global.status` and
/// `colors.dark.global.status` are `role.global.status`,
/// `colors.light.surfaces.window.background` is `role.window.background`,
/// `colors.light.keyed.badge` is `role.@badge`, `inputs.accent.dark` is
/// `input.accent dark`, the key and the scheme it is listed at,
/// `widgets.button.label` is `button.label`); `None` for a path of no field
/// the gate reads.
#[must_use]
pub fn listed_key(path: &str) -> Option<String> {
    if let Some((input, scheme)) = super::toml::read::listed_input(path) {
        return Some(format!("input.{input} {}", scheme.name()));
    }
    let path = super::toml::read::listed_path(path);
    if let Some(rest) = path.strip_prefix("tokens.") {
        return Some(format!("token.{rest}"));
    }
    if let Some(rest) = path.strip_prefix("widgets.") {
        return Some(rest.to_owned());
    }
    let (_, roles) = path.strip_prefix("colors.")?.split_once('.')?;
    let role = if let Some(rest) = roles.strip_prefix("surfaces.") {
        rest.to_owned()
    } else if let Some(key) = roles.strip_prefix("keyed.") {
        format!("@{key}")
    } else {
        roles.to_owned()
    };
    Some(format!("role.{role}"))
}

/// A synthetic value of the type `ty`, written as the model writes it, of
/// the field `info` or of an entry of its map (S17, ADD-A128), with the
/// unit suffix S11 spells its key with. Synthetic, never bundled, no
/// platform's: an empty list or map, but a list validation refuses empty
/// holding one element (a paint's layers the colour `#010203`, a
/// transition's properties `fill`, S12) and an `Option` of a list `none`
/// (a transition's `from`: any state), as is an indication box (the
/// part's own box); a list of two halves, 1 in the
/// type's unit, `#010203`, the text `Synthetic`, an enum's first variant
/// but an icon's rendering mode `original`, a glyph's named system image
/// `synthetic`; a struct block's every field
/// so, `absent` where a field may be absent; a corner's radius 1 px; any
/// other enum block's first unit variant. `None` for a type with none yet, and for a key: a key names
/// what the preset declares or what its slot lists (S10), which no
/// synthetic name is, so a listed key is refused by name until the layers
/// take the preset's declarations.
fn synthetic_of(ty: &str, info: &registry::FieldInfo) -> Option<(&'static str, Item)> {
    // An indication box's part names a part of the ripple's widget, which
    // no synthetic name is: `none`, the part's own box (S2.12).
    if ty.starts_with("Option<Vec<") || ty == "Option<IndicationBox>" {
        return Some(("", Item::String("none".to_owned())));
    }
    let wrapped = ["Option<", "Value<"]
        .iter()
        .find_map(|wrap| ty.strip_prefix(wrap)?.strip_suffix('>'));
    if let Some(inner) = wrapped {
        return synthetic_of(inner, info);
    }
    if registry::key_levels(ty) > 0 {
        return Some(("", Item::Table(Table::new())));
    }
    if let Some(element) = ty.strip_prefix("Vec<") {
        let one = match element.strip_suffix('>') {
            Some("PaintLayer") => vec![Item::String("#010203".to_owned())],
            Some("PropertyGroup") => vec![Item::String("fill".to_owned())],
            _ => Vec::new(),
        };
        return Some(("", Item::Array(one)));
    }
    match ty {
        "(f32,f32)" => return Some(("", Item::Array(vec![Item::Float(0.5), Item::Float(0.5)]))),
        "Scalar" => return Some(("", Item::Integer(1))),
        // S12 refuses an icon style's absent `fill` beside any other mode,
        // and accepts `original` beside a stated fill and an absent one.
        "RenderingMode" => return Some(("", Item::String("original".to_owned()))),
        // S12 refuses a glyph's absent size beside any other source, and
        // accepts a system control beside a stated size and an absent one,
        // and beside any fill and stroke. S15's placement rule (on a part
        // drawn in every state, never on one hidden by state) is no S12 check, so the value is
        // accepted on any glyph; the real source is checked when measured.
        "GlyphSource" => {
            let mut control = Table::new();
            control.insert(
                "control".to_owned(),
                Item::String("app_kit_checkbox".to_owned()),
            );
            control.insert("replaces".to_owned(), Item::Array(Vec::new()));
            let mut source = Table::new();
            source.insert("system_control".to_owned(), Item::Table(control));
            return Some(("", Item::Table(source)));
        }
        _ => {}
    }
    let one = Item::Integer(1);
    let leaf = match (info.value)().map(Ty::held) {
        Some(Ty::Duration) => Some(("_ms", one)),
        Some(Ty::Length) => Some(("_px", one)),
        Some(Ty::Angle) => Some(("_deg", one)),
        Some(Ty::FontSize) => Some(("_pt", one)),
        Some(Ty::Number) => Some(("", one)),
        Some(Ty::Bool) => Some(("", Item::Boolean(true))),
        Some(Ty::Text) => Some(("", Item::String("Synthetic".to_owned()))),
        Some(Ty::Color | Ty::Paint) => Some(("", Item::String("#010203".to_owned()))),
        Some(Ty::Key(_)) => return None,
        Some(Ty::Enum(names)) => {
            return names
                .variants
                .first()
                .map(|first| ("", Item::String((*first).to_owned())));
        }
        // `held` opens every `Option`, so `Ty::Option` is not met here; it
        // is listed for the match to be exhaustive.
        Some(Ty::Shadow | Ty::Block(_) | Ty::Option(_) | Ty::Map { .. }) | None => None,
    };
    if leaf.is_some() {
        return leaf;
    }
    match info.ty {
        registry::FieldType::Block {
            form: registry::BlockForm::Struct,
            fields,
            block,
            ..
        } => {
            let mut table = Table::new();
            for field in fields {
                let (suffix, item) =
                    if field.may_be_absent && absent_synthetic(Some(block), field.path) {
                        (unit_suffix(field), Item::String("absent".to_owned()))
                    } else {
                        synthetic_of(written(field), field)?
                    };
                table.insert(format!("{}{suffix}", field.path), item);
            }
            Some(("", Item::Table(table)))
        }
        // A corner's plain radius: `ellipse` is refused but on all four
        // corners with a `circular` curve, and `follow_part` outside a
        // focus indicator or beside offsets that can differ (S12), which
        // the corners beside a listed one need not give.
        registry::FieldType::Block {
            block: "Radius", ..
        } => Some(("_px", Item::Integer(1))),
        registry::FieldType::Block {
            form: registry::BlockForm::Enum { unit_variants },
            ..
        } => unit_variants
            .first()
            .map(|first| ("", Item::String((*first).to_owned()))),
        registry::FieldType::Leaf(_) | registry::FieldType::Patch { .. } => None,
    }
}

/// The fields whose `Absent` S12 refuses under a condition of their place
/// (S12), or beside which an absent field the conditional marks read is
/// refused, so a synthetic value states them (S17, ADD-A131: never
/// `Absent` where a conditional mark refuses it there): a glyph's `size`,
/// absent only on a `SystemControl` source, its `fill`, absent beside an
/// absent `stroke` only there (S6.3), and its `stroke`, beside which, stated,
/// an absent `fill` is accepted on every source; an edge's `dash`, absent
/// only where its `width` or `paint` is; an icon style's `fill`, absent only
/// beside an `original` rendering mode (S2.9, S12); a ripple's `color`,
/// `alpha` and `padding`, absent only where its `effect` is (S2.12); and a
/// text style's drawn-text leaves, absent in a keyed style and a slot only,
/// which no part referencing them takes absent (S2.8); and the behaviour
/// token's `focus_visible_when`, which a widget's own `focus_visible_when`,
/// never absent, references.
const STATED_UNDER_CONDITION: [(&str, &str); 13] = [
    ("Glyph", "size"),
    ("Glyph", "fill"),
    ("Glyph", "stroke"),
    ("Edge", "dash"),
    ("IconStyle", "fill"),
    ("RippleStyle", "color"),
    ("RippleStyle", "alpha"),
    ("RippleStyle", "padding"),
    ("TextStyle", "fill"),
    ("TextStyle", "elision"),
    ("TextStyle", "wrap"),
    ("TextStyle", "max_lines"),
    ("Behavior", "focus_visible_when"),
];

/// Whether a synthetic value states `absent` at the field `field` of the
/// block `owner`, marked may be absent: where no condition of its place
/// can refuse it ([`STATED_UNDER_CONDITION`]).
fn absent_synthetic(owner: Option<&str>, field: &str) -> bool {
    !owner.is_some_and(|owner| STATED_UNDER_CONDITION.contains(&(owner, field)))
}

/// A field's type as the model writes it.
fn written(info: &registry::FieldInfo) -> &'static str {
    match info.ty {
        registry::FieldType::Leaf(ty)
        | registry::FieldType::Block { ty, .. }
        | registry::FieldType::Patch { ty, .. } => ty,
    }
}

/// The unit suffix S11 spells every value of a one-unit field with (S11:
/// `delay_ms = "absent"`), none for any other.
fn unit_suffix(info: &registry::FieldInfo) -> &'static str {
    match (info.value)() {
        Some(Ty::Duration) => "_ms",
        Some(Ty::Length) => "_px",
        Some(Ty::Angle) => "_deg",
        _ => "",
    }
}

/// The synthetic value of a listed path (S17, ADD-A128), its last key
/// spelled with the unit suffix of the value: `absent` where the field may
/// be absent, an empty list at a focus indicator's transition's
/// `properties`, else [`synthetic_of`] its type, or its map's value type
/// where the path names an entry; an input's, `#010203` at its listed
/// scheme alone (`inputs.<name>.<scheme>`, ADD-A132), which the synthetic
/// route folds into the preset's own input base.
fn synthetic_value(path: &str) -> Result<(Vec<String>, Item), String> {
    if super::toml::read::listed_input(path).is_some() {
        let keys = super::toml::read::key_segments(path)
            .ok_or_else(|| "a key that does not read".to_owned())?;
        return Ok((keys, Item::String("#010203".to_owned())));
    }
    let (info, levels) = super::toml::read::listed_field(path)
        .ok_or_else(|| "the model holds no field there".to_owned())?;
    let mut ty = written(info);
    for _ in 0..levels {
        ty = map_value(ty).ok_or_else(|| format!("`{ty}` is no map"))?;
    }
    let owner = super::toml::read::listed_owner(path);
    let (suffix, item) = if levels == 0 && info.may_be_absent && absent_synthetic(owner, info.path)
    {
        (unit_suffix(info), Item::String("absent".to_owned()))
    } else if focus_properties(path) {
        ("", Item::Array(Vec::new()))
    } else {
        synthetic_of(ty, info)
            .ok_or_else(|| format!("the gate has no synthetic value of `{ty}`"))?
    };
    let mut keys = super::toml::read::key_segments(path)
        .ok_or_else(|| "a key that does not read".to_owned())?;
    if let Some(last) = keys.last_mut() {
        *last = format!("{}{suffix}", super::toml::read::listed_path(last));
    }
    Ok((keys, item))
}

/// Whether a listed path is a `FocusIndicator.transition`'s `properties`,
/// empty as the transition times the whole indicator (S2.10, S12): the
/// indicator is every `focus` field's.
fn focus_properties(path: &str) -> bool {
    super::toml::read::key_segments(path).is_some_and(|keys| {
        keys.ends_with(&["focus", "transition", "properties"].map(String::from))
    })
}

/// A map's value type, as the model writes it: `Vec<(K,V)>`'s and
/// `SurfaceMap<V>`'s `V`.
fn map_value(ty: &str) -> Option<&str> {
    if let Some(inner) = ty.strip_prefix("SurfaceMap<") {
        return inner.strip_suffix('>');
    }
    let pair = ty.strip_prefix("Vec<(")?.strip_suffix(")>")?;
    let mut depth = 0_usize;
    for (at, c) in pair.char_indices() {
        match c {
            '<' | '(' => depth = depth.saturating_add(1),
            '>' | ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => return pair.get(at.saturating_add(1)..),
            _ => {}
        }
    }
    None
}

/// A listed path as its `[pending]` table spells it, with its register ids.
type Listed = (String, Vec<String>);

/// The synthetic layers of a preset's `[pending]` table over its own layer
/// `preset` (S17's synthetic route, ADD-A128): one layer per listed path,
/// stating it in its bases with a synthetic value, but the listed paths
/// below one entry of a map of bare blocks (a keyed text style, a keyed
/// icon context) that no base of the preset states, which share one layer
/// creating the entry with every listed field stated and stating its keyed
/// name (ADD-A133); the layers [`Preset`] and S19.1's build test load such
/// a preset with. A listed path the model holds no field at, or of a type
/// with no synthetic value yet, is refused, naming it. A listed default
/// size and a listed presence have none: no layer states a declaration,
/// and the synthetic route waives the checks that read it, and it draws
/// the part (S17, ADD-A131).
pub fn synthetic_layers(pending: &[Listed], preset: &Layer) -> Result<Vec<Layer>, String> {
    let mut groups: Vec<(Option<String>, Vec<&Listed>)> = Vec::new();
    for listed in pending.iter().filter(|(path, _)| {
        !is_listed_default(path) && super::toml::read::listed_part(path).is_none()
    }) {
        let entry = unstated_entry(&listed.0, preset);
        let shared = entry.as_ref().and_then(|entry| {
            groups
                .iter_mut()
                .find(|(held, _)| held.as_ref() == Some(entry))
        });
        match shared {
            Some((_, members)) => members.push(listed),
            None => groups.push((entry, vec![listed])),
        }
    }
    groups
        .into_iter()
        .map(|(_, members)| {
            let named = || {
                members
                    .iter()
                    .map(|(path, ids)| format!("`{path}` ({})", ids.join(", ")))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let mut table = Table::new();
            for (path, ids) in &members {
                let (keys, value) = synthetic_value(path)
                    .map_err(|why| format!("`{path}` ({}): {why}", ids.join(", ")))?;
                if !insert_at(&mut table, &keys, value) {
                    return Err(format!("{}: two values at one place", named()));
                }
            }
            let text = ::toml::to_string(&table).map_err(|error| error.to_string())?;
            Layer::from_toml(&text).map_err(|error| format!("{}: {error}", named()))
        })
        .collect()
}

/// The entry of a map of bare blocks (S6.1: a keyed text style, a keyed
/// icon context, a surface set's roles) the listed path `path` lies below,
/// as S11 spells its key, where no base of the preset's layer `preset`
/// states it (ADD-A133); `None` for any other path.
fn unstated_entry(path: &str, preset: &Layer) -> Option<String> {
    let segments = super::toml::read::key_segments(path)?;
    let mut entry = String::new();
    for segment in segments.iter().take(segments.len().saturating_sub(1)) {
        if !entry.is_empty() {
            entry.push('.');
        }
        entry.push_str(&super::toml::read::toml_key(segment));
        let Some((info, levels)) = super::toml::read::listed_field(&entry) else {
            continue;
        };
        let registry::FieldType::Block {
            ty,
            form: registry::BlockForm::Struct,
            ..
        } = info.ty
        else {
            continue;
        };
        if levels == 0 || registry::key_levels(ty) != levels {
            continue;
        }
        let mut held = ty;
        for _ in 0..levels {
            held = map_value(held)?;
        }
        let bare = !held.starts_with("Value<") && !held.starts_with("Vec<");
        return (bare && !super::validate::states(preset, &entry)).then_some(entry);
    }
    None
}

/// Inserts `value` into `table` at the dotted place `keys`, opening the
/// tables on the way; `false` where a value already lies there or above.
fn insert_at(table: &mut Table, keys: &[String], value: Item) -> bool {
    let Some((last, above)) = keys.split_last() else {
        return false;
    };
    let mut node = table;
    for key in above {
        let next = node
            .entry(key.clone())
            .or_insert_with(|| Item::Table(Table::new()));
        let Item::Table(next) = next else {
            return false;
        };
        node = next;
    }
    if node.contains_key(last) {
        return false;
    }
    node.insert(last.clone(), value);
    true
}

/// Whether the listed path `path` names a pending default size's
/// declaration (S17, ADD-A131), which the declarations run compares.
#[must_use]
pub fn is_listed_default(path: &str) -> bool {
    super::toml::read::listed_default(path).is_some()
}

/// The typed widgets the model holds, each as S11 writes it.
#[must_use]
pub fn widgets() -> Vec<String> {
    registry::widgets()
        .iter()
        .map(|info| info.name.to_owned())
        .collect()
}

/// The parts of the widget `widget`, each as S11 writes it; none for a
/// widget the model does not hold.
#[must_use]
pub fn parts(widget: &str) -> Vec<String> {
    registry::widget(widget)
        .map(|info| info.parts.iter().map(|part| part.name.to_owned()).collect())
        .unwrap_or_default()
}

/// The content variants of the widget `widget`, as S11 writes them: the
/// variants its connector sets from the element's content (the catalogue's
/// pairs, S8), which S12 exempts from being selected on.
#[must_use]
pub fn content_variants(widget: &str) -> Vec<String> {
    registry::widget(widget)
        .map(|info| {
            info.content_variants
                .iter()
                .map(|(_, variant)| (*variant).to_owned())
                .collect()
        })
        .unwrap_or_default()
}

/// The typed widget `name` names.
fn widget_kind(name: &str) -> Result<WidgetKind, Unread> {
    registry::widget(name)
        .map(|info| info.kind)
        .ok_or_else(|| Unread::NotHeld(format!("the model holds no widget `{name}` yet")))
}

/// A widget's declarations in force (S10), each spelled as S11 spells it,
/// each left out stated as none.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Declared {
    /// `available`; `true` where the preset does not state it.
    pub available: bool,
    /// `variants`.
    pub variants: Vec<String>,
    /// `sizes`.
    pub sizes: Vec<String>,
    /// `default_size`: a size's name, or a pending one spelled `pending
    /// <ids>` ([`default_spelled`]).
    pub default_size: Option<String>,
    /// `densities`.
    pub densities: Vec<String>,
    /// `flags`.
    pub flags: Vec<String>,
    /// `exclusive`: each group highest precedence first.
    pub exclusive: Vec<Vec<String>>,
    /// `exclusive_variants`.
    pub exclusive_variants: Vec<Vec<String>>,
    /// `recency`: each set by its name, its flags in the stated order.
    pub recency: Vec<(String, Vec<String>)>,
    /// `requires_variants`.
    pub requires_variants: Vec<String>,
    /// `sizes_by_variant`: each variant, its sizes and its default size,
    /// in written order.
    pub sizes_by_variant: Vec<(String, Vec<String>, Option<String>)>,
    /// `sizes_by_context`: each context selector ([`context_spelled`]), its
    /// sizes and its default size, in written order.
    pub sizes_by_context: Vec<(String, Vec<String>, Option<String>)>,
}

/// A default size as the gate compares it: a size's name, or a pending one
/// as `pending <ids>`, its register ids sorted, each once, joined by `, `:
/// a set, as S17 compares ids (S10's *A pending default*).
#[must_use]
pub fn default_spelled(size: Option<&str>, pending: &[String]) -> Option<String> {
    match size {
        Some(size) => Some(size.to_owned()),
        None if pending.is_empty() => None,
        None => {
            let mut ids: Vec<&str> = pending.iter().map(String::as_str).collect();
            ids.sort_unstable();
            ids.dedup();
            Some(format!("pending {}", ids.join(", ")))
        }
    }
}

/// An S11 context selector as the gate compares it: its keys sorted, each
/// `<key> = <value>`, a list's names sorted, `scope` stated (`"any"` where
/// it is left out); the form [`Declared::sizes_by_context`] spells and a
/// state model's selector is brought to.
#[must_use]
pub fn context_spelled(keys: &[(String, String)]) -> String {
    let mut keys: Vec<(String, String)> = keys.to_vec();
    if !keys.iter().any(|(key, _)| key == "scope") {
        keys.push(("scope".to_owned(), "\"any\"".to_owned()));
    }
    keys.sort();
    let fields: Vec<String> = keys
        .iter()
        .map(|(key, value)| format!("{key} = {value}"))
        .collect();
    format!("{{ {} }}", fields.join(", "))
}

/// A list of names as [`context_spelled`] writes it: sorted, each quoted.
#[must_use]
pub fn names_spelled(names: &[String]) -> String {
    let mut names: Vec<String> = names.iter().map(|name| format!("{name:?}")).collect();
    names.sort();
    format!("[{}]", names.join(", "))
}

/// A declared default size as [`default_spelled`] writes it.
fn declared_default(default: Option<&DefaultSize>) -> Option<String> {
    match default {
        Some(DefaultSize::Size(size)) => default_spelled(Some(size.name()), &[]),
        Some(DefaultSize::Pending(ids)) => default_spelled(None, ids),
        None => None,
    }
}

/// A `sizes_by_context` entry's selector as [`context_spelled`] writes it.
fn selector_spelled(context: &ContextMatch) -> String {
    let mut keys = Vec::new();
    let scope = match &context.scope {
        ContextScope::AnyAncestor => "\"any\"".to_owned(),
        ContextScope::NearestAmong(widgets) => {
            let names: Vec<String> = widgets.iter().map(widget_name).collect();
            format!("{{ nearest = {} }}", names_spelled(&names))
        }
    };
    keys.push(("scope".to_owned(), scope));
    match &context.target {
        Some(WidgetOrPart::Widget(widget)) => {
            keys.push(("widget".to_owned(), format!("{:?}", widget_name(widget))));
        }
        Some(WidgetOrPart::Part(PartRef::Typed(part))) => {
            let widget = widget_name(&WidgetRef::Typed(part.widget()));
            keys.push(("widget".to_owned(), format!("{widget:?}")));
            keys.push(("part".to_owned(), format!("{:?}", part.name())));
        }
        Some(WidgetOrPart::Part(PartRef::Component { key, part })) => {
            keys.push((
                "widget".to_owned(),
                format!("{:?}", format!("component.{key}")),
            ));
            keys.push(("part".to_owned(), format!("{:?}", part.as_str())));
        }
        None => {}
    }
    let lists = [
        (
            "variants",
            context
                .variants
                .all_of
                .iter()
                .map(variant_name)
                .collect::<Vec<_>>(),
        ),
        (
            "not_variants",
            context.variants.none_of.iter().map(variant_name).collect(),
        ),
        (
            "states",
            context.states.all_of.iter().map(StateFlag::name).collect(),
        ),
        (
            "not_states",
            context.states.none_of.iter().map(StateFlag::name).collect(),
        ),
        (
            "sizes",
            context
                .sizes
                .all_of
                .iter()
                .map(|size| size.name().to_owned())
                .collect(),
        ),
        (
            "not_sizes",
            context
                .sizes
                .none_of
                .iter()
                .map(|size| size.name().to_owned())
                .collect(),
        ),
    ];
    for (key, names) in lists {
        if !names.is_empty() {
            keys.push((key.to_owned(), names_spelled(&names)));
        }
    }
    if let Some(surface) = context.surface {
        keys.push(("surface".to_owned(), format!("{:?}", surface.name())));
    }
    context_spelled(&keys)
}

/// A variant as S11 writes it: a typed one's name, a keyed one's `@key`.
fn variant_name(variant: &VariantRef) -> String {
    match variant {
        VariantRef::Typed(variant) => variant.name().to_owned(),
        VariantRef::Keyed(key) => format!("@{key}"),
    }
}

/// A density as S11 writes it.
fn density_name(density: &Density) -> String {
    match density {
        Density::Default => "default".to_owned(),
        Density::Compact => "compact".to_owned(),
        Density::Keyed(key) => format!("@{key}"),
    }
}

/// A widget as S11 writes it.
fn widget_name(widget: &WidgetRef) -> String {
    match widget {
        WidgetRef::Typed(kind) => registry::widgets()
            .iter()
            .find(|info| info.kind == *kind)
            .map_or_else(String::new, |info| info.name.to_owned()),
        WidgetRef::Component(key) => format!("component.{key}"),
    }
}

impl Declared {
    fn of(decl: &WidgetDecl) -> Declared {
        let variants = |list: &[VariantRef]| list.iter().map(variant_name).collect::<Vec<_>>();
        let sizes = |list: &[ControlSize]| {
            list.iter()
                .map(|size| size.name().to_owned())
                .collect::<Vec<_>>()
        };
        let flags = |list: &[StateFlag]| list.iter().map(StateFlag::name).collect::<Vec<_>>();
        Declared {
            available: decl.available.unwrap_or(true),
            variants: variants(decl.variants.as_deref().unwrap_or_default()),
            sizes: sizes(decl.sizes.as_deref().unwrap_or_default()),
            default_size: declared_default(decl.default_size.as_ref()),
            densities: decl.densities.iter().flatten().map(density_name).collect(),
            flags: flags(decl.flags.as_deref().unwrap_or_default()),
            exclusive: decl
                .exclusive
                .iter()
                .flatten()
                .map(|group| flags(group))
                .collect(),
            exclusive_variants: decl
                .exclusive_variants
                .iter()
                .flatten()
                .map(|group| variants(group))
                .collect(),
            recency: decl
                .recency
                .iter()
                .flatten()
                .map(|(set, members)| (set.as_str().to_owned(), flags(members)))
                .collect(),
            requires_variants: variants(decl.requires_variants.as_deref().unwrap_or_default()),
            sizes_by_variant: decl
                .sizes_by_variant
                .iter()
                .flatten()
                .map(|by| {
                    let default = declared_default(by.default_size.as_ref());
                    (variant_name(&by.variant), sizes(&by.sizes), default)
                })
                .collect(),
            sizes_by_context: decl
                .sizes_by_context
                .iter()
                .flatten()
                .map(|by| {
                    let default = declared_default(by.default_size.as_ref());
                    (selector_spelled(&by.context), sizes(&by.sizes), default)
                })
                .collect(),
        }
    }
}

/// The conditions a combination resolves under: the stand-in's state, the
/// window's conditions and the flavour (S17).
#[derive(Clone, Debug)]
pub struct Conditions {
    state: ReaderState,
    window: WindowConditions,
    flavor: Option<Key>,
}

impl Conditions {
    /// The no-reader state's conditions (S16) under the scheme `scheme`:
    /// no other condition holding, factor 1, no font DPI of its own, a mouse
    /// the last input, the default flavour.
    pub fn new(scheme: &str) -> Result<Conditions, String> {
        let scheme = Scheme::from_name(scheme).ok_or_else(|| format!("no scheme `{scheme}`"))?;
        Ok(Conditions {
            state: ReaderState::new(scheme, 1.0),
            window: WindowConditions::new(InputKind::Mouse),
            flavor: None,
        })
    }

    /// The conditions with `condition`, as S11 writes it, holding: set on
    /// the stand-in's state, the window or the flavour. Refused for a
    /// preset's condition, which the preset resolved decides, and for the
    /// accessibility text size, whose category the gate does not choose.
    pub fn set(&mut self, condition: &str) -> Result<(), Unread> {
        let mut cx = Reader::default();
        let parsed = when::condition(condition, "condition", &mut cx);
        let Some(condition) = parsed.filter(|_| !cx.failed()) else {
            return Err(Unread::of(&cx.into_problems()));
        };
        let state = &mut self.state;
        let window = &mut self.window;
        match condition {
            Condition::Scheme(scheme) => state.scheme = scheme,
            Condition::ContrastIncreased => state.contrast = Some(Contrast::Increased),
            Condition::ContrastHigh => state.contrast = Some(Contrast::High),
            Condition::ReducedTransparency => state.reduced_transparency = true,
            Condition::ReducedMotion => state.reduced_motion = true,
            Condition::AnimationsOff => state.animations_off = true,
            Condition::BoldText => state.bold_text = true,
            Condition::ButtonShapes => state.button_shapes = true,
            Condition::DifferentiateWithoutColor => state.differentiate_without_color = true,
            Condition::TabletMode => state.tablet_mode = true,
            Condition::MobileFormFactor => state.mobile_form_factor = true,
            Condition::BatterySaver => state.battery_saver = true,
            Condition::ContainerSize { container, size } => {
                state.container_sizes.retain(|(held, _)| *held != container);
                state.container_sizes.push((container, size));
            }
            Condition::OverlayScrollbars => state.overlay_scrollbars = true,
            Condition::OnOffLabels => state.on_off_labels = true,
            Condition::FullKeyboardAccess => state.full_keyboard_access = true,
            Condition::PreferCrossFade => state.prefer_cross_fade = true,
            Condition::Idiom(idiom) => state.idiom = Some(idiom),
            Condition::SystemSetting(key) => {
                if !state.system_settings.contains(&key) {
                    state.system_settings.push(key);
                }
            }
            Condition::DeviceModel(key) => state.device_model = Some(key),
            Condition::WindowInactive => window.inactive = true,
            Condition::NotMain => window.not_main = true,
            Condition::AppInactive => window.app_inactive = true,
            Condition::Maximized => window.maximized = true,
            Condition::FullScreen => window.full_screen = true,
            Condition::Tiled(edges) => window.tiled = edges,
            Condition::Elevated => window.elevated = true,
            Condition::Input(input) => window.input = input,
            Condition::NotComposited => window.not_composited = true,
            Condition::Shaded => window.shaded = true,
            Condition::AttentionFlash => window.attention_flash = true,
            Condition::TintDimmed => window.tint_dimmed = true,
            Condition::MnemonicsVisible => window.mnemonics_visible = true,
            Condition::Freeform => window.freeform = true,
            Condition::Posture(posture) => window.posture = Some(posture),
            Condition::UnitScale => window.unit_scale = true,
            Condition::Flavor(flavor) => self.flavor = Some(flavor),
            Condition::AccessibilityTextSize => {
                return Err(Unread::NotHeld(
                    "the accessibility text size names no one category to set".to_owned(),
                ));
            }
            Condition::Preset(_) => {
                return Err(Unread::Refused(
                    "a preset's condition is the preset resolved's".to_owned(),
                ));
            }
        }
        Ok(())
    }
}

/// A fact's `when` (S11, S17), read as a rule's.
#[derive(Clone, Debug)]
pub struct When(Selector);

/// A fact's `when`, or why it cannot be read.
pub fn parse_when(item: &Item) -> Result<When, Unread> {
    let mut cx = Reader::default();
    let selector = when::read_selector(item, "when", &mut cx);
    match selector {
        Some(selector) if !cx.failed() => Ok(When(selector)),
        _ => Err(Unread::of(&cx.into_problems())),
    }
}

/// A fact's key (S17): what the model holds at it.
#[derive(Clone, Debug)]
pub struct FactKey {
    place: Place,
}

/// What a fact's key names.
#[derive(Clone, Debug)]
enum Place {
    /// A whole part: its absence or its substitution.
    WholePart(TypedPart),
    /// A field of a part, by its path below the part.
    Part(TypedPart, Vec<String>),
    /// A widget value, by its path below the widget's values.
    Values(WidgetKind, Vec<String>),
    /// A token, by its path below the tokens.
    Token(Vec<String>),
    /// A colour role: its reference, and its path below the colour roles.
    Role(Ref, Vec<String>),
    /// An input.
    Input(InputName),
    /// One of KDE's scheme lists (S9.1).
    KdeList(KdeList),
}

/// KDE's scheme lists: `input.kde_scheme.groups`, `.inactive_groups`,
/// `.unset_keys` (S9.1).
#[derive(Clone, Copy, Debug, PartialEq)]
enum KdeList {
    Groups,
    InactiveGroups,
    UnsetKeys,
}

impl KdeList {
    const ALL: [KdeList; 3] = [KdeList::Groups, KdeList::InactiveGroups, KdeList::UnsetKeys];

    fn name(self) -> &'static str {
        match self {
            KdeList::Groups => "groups",
            KdeList::InactiveGroups => "inactive_groups",
            KdeList::UnsetKeys => "unset_keys",
        }
    }
}

/// The segments of a dotted key, a quoted segment taken as written inside
/// its quotes (S17: a map's text key as a TOML dotted key writes it).
fn segments(key: &str) -> Result<Vec<String>, Unread> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut chars = key.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => quoted = !quoted,
            '\\' if quoted => {
                if let Some(escaped) = chars.next() {
                    current.push(escaped);
                }
            }
            '.' if !quoted => out.push(std::mem::take(&mut current)),
            other => current.push(other),
        }
    }
    if quoted {
        return Err(Unread::Refused(format!("`{key}`: an unclosed quote")));
    }
    out.push(current);
    if out.iter().any(String::is_empty) {
        return Err(Unread::Refused(format!("`{key}`: an empty segment")));
    }
    Ok(out)
}

/// Whether `fields` holds the field `path` names.
fn holds_field(fields: &'static [registry::FieldInfo], path: &[String]) -> bool {
    registry::find(fields, &path.join(".")).is_some()
}

/// Whether a colour role's path below the colour roles names a map whole
/// (`global.accent_ramp`), not a colour.
fn names_map(path: &[String]) -> bool {
    let path = path.join(".");
    registry::find(registry::colors(), &path).is_some()
        && registry::find_value(registry::colors(), &path).is_none()
}

/// A colour role reference's path below the colour roles.
fn role_path(reference: &Ref) -> Option<Vec<String>> {
    fn names(head: &[&str], path: &[Key]) -> Vec<String> {
        head.iter()
            .map(|segment| (*segment).to_owned())
            .chain(path.iter().map(|key| key.as_str().to_owned()))
            .collect()
    }
    match reference {
        Ref::Role { set, path } => Some(names(&["surfaces", set.name()], path)),
        Ref::GlobalRole(path) => Some(names(&["global"], path)),
        Ref::KeyedRole(key) => Some(vec!["keyed".to_owned(), key.as_str().to_owned()]),
        _ => None,
    }
}

/// A fact's key (S17), where the model holds it.
pub fn parse_key(key: &str) -> Result<FactKey, Unread> {
    let not_held = || Unread::NotHeld(format!("the model holds no `{key}` yet"));
    let segments = segments(key)?;
    let Some((first, rest)) = segments.split_first() else {
        return Err(not_held());
    };
    let place = match first.as_str() {
        "token" => {
            if !holds_field(registry::tokens(), rest) {
                return Err(not_held());
            }
            Place::Token(rest.to_vec())
        }
        "role" => {
            let parsed = Expr::parse(key).map_err(|error| Unread::Refused(error.to_string()))?;
            let Expr::Ref(reference) = parsed else {
                return Err(not_held());
            };
            let Some(path) = role_path(&reference) else {
                return Err(not_held());
            };
            if !holds_field(registry::colors(), &path) {
                return Err(not_held());
            }
            Place::Role(reference, path)
        }
        "input" => {
            let list = KdeList::ALL.into_iter().find(|list| {
                matches!(rest, [scheme, name] if scheme == "kde_scheme" && name == list.name())
            });
            match list {
                Some(list) => Place::KdeList(list),
                None => {
                    let name = InputName::new(&rest.join("."))
                        .map_err(|error| Unread::Refused(error.to_string()))?;
                    Place::Input(name)
                }
            }
        }
        "component" => return Err(not_held()),
        widget => {
            let kind = widget_kind(widget)?;
            let Some(info) = registry::widget(widget) else {
                return Err(not_held());
            };
            let Some((part, path)) = rest.split_first() else {
                return Err(not_held());
            };
            if part == "values" {
                if !holds_field(info.values, path) {
                    return Err(not_held());
                }
                Place::Values(kind, path.to_vec())
            } else {
                let Some(typed) = TypedPart::from_names(kind, part) else {
                    return Err(not_held());
                };
                if path.is_empty() {
                    Place::WholePart(typed)
                } else {
                    let fields = typed.info().map(|info| info.fields).unwrap_or_default();
                    if !holds_field(fields, path) {
                        return Err(not_held());
                    }
                    Place::Part(typed, path.to_vec())
                }
            }
        }
    };
    Ok(FactKey { place })
}

impl FactKey {
    /// The input the key names, as a reference names it below `input.`
    /// (`accent`, `accent_ramp.light1`); `None` for any other key.
    #[must_use]
    pub fn input(&self) -> Option<String> {
        match &self.place {
            Place::Input(name) => Some(name.to_string()),
            Place::WholePart(_)
            | Place::Part(..)
            | Place::Values(..)
            | Place::Token(_)
            | Place::Role(..)
            | Place::KdeList(_) => None,
        }
    }

    /// The widget whose element the key is queried with; `None` for a
    /// token, a role or an input, read in the root scope.
    #[must_use]
    pub fn widget(&self) -> Option<String> {
        let kind = match &self.place {
            Place::WholePart(part) | Place::Part(part, _) => part.widget(),
            Place::Values(widget, _) => *widget,
            Place::Token(_) | Place::Role(..) | Place::Input(_) | Place::KdeList(_) => return None,
        };
        Some(widget_name(&WidgetRef::Typed(kind)))
    }

    /// Whether the key names a whole part.
    #[must_use]
    pub fn whole_part(&self) -> bool {
        matches!(self.place, Place::WholePart(_))
    }

    /// The key's path below the key `block`, where `block` names a part's
    /// field, a widget value, a token or a colour role holding it
    /// (`offset.top` below `button.body.focus` for
    /// `button.body.focus.offset.top`, below `token.focus` for
    /// `token.focus.offset.top`); `None` where it names no block holding the
    /// key.
    #[must_use]
    pub fn below(&self, block: &FactKey) -> Option<Vec<String>> {
        let (path, above) = match (&self.place, &block.place) {
            (Place::Part(part, path), Place::Part(other, above)) if part == other => (path, above),
            (Place::Values(widget, path), Place::Values(other, above)) if widget == other => {
                (path, above)
            }
            (Place::Token(path), Place::Token(above))
            | (Place::Role(_, path), Place::Role(_, above)) => (path, above),
            _ => return None,
        };
        let rest = path.strip_prefix(above.as_slice())?;
        (!rest.is_empty()).then(|| rest.to_vec())
    }

    /// The colour role's path below `colors.<scheme>` the key names
    /// (`global.accent`, `surfaces.window.foreground`, `keyed.<key>`);
    /// `None` for any other key.
    #[must_use]
    pub fn role(&self) -> Option<String> {
        match &self.place {
            Place::Role(_, path) => Some(path.join(".")),
            Place::WholePart(_)
            | Place::Part(..)
            | Place::Values(..)
            | Place::Token(_)
            | Place::Input(_)
            | Place::KdeList(_) => None,
        }
    }
}

/// The expressions a fact's value states: each string leaf of it, nested
/// in tables and lists too, that starts with `=` (S2.2), without the `=`.
fn expressions(value: &Item, out: &mut Vec<String>) {
    match value {
        Item::String(text) => {
            if let Some(expression) = text.strip_prefix('=') {
                out.push(expression.trim().to_owned());
            }
        }
        Item::Array(items) => {
            for item in items {
                expressions(item, out);
            }
        }
        Item::Table(table) => {
            for item in table.values() {
                expressions(item, out);
            }
        }
        Item::Integer(_) | Item::Float(_) | Item::Boolean(_) | Item::Datetime(_) => {}
    }
}

/// Whether a fact's value is a derivation (S17): it states an expression,
/// as itself or in a leaf of a table or list it is.
#[must_use]
pub fn is_derivation(value: &Item) -> bool {
    let mut out = Vec::new();
    expressions(value, &mut out);
    !out.is_empty()
}

impl Preset {
    /// The colour roles' paths, below `colors.<scheme>`, that `value`'s
    /// expressions read as operands, a `surface.<role>` one in every surface
    /// set the preset declares, as an element's scope may bind any of them
    /// (S3.3); and the inputs they read (the perturbed run's, S17). None for
    /// a literal.
    #[must_use]
    pub fn operands(&self, value: &Item) -> (Vec<String>, Vec<String>) {
        let mut texts = Vec::new();
        expressions(value, &mut texts);
        let mut refs = Vec::new();
        for text in texts {
            if let Ok(expr) = Expr::parse(&text) {
                super::resolve::evaluate::references(&expr, &mut refs);
            }
        }
        let (mut roles, mut inputs) = (Vec::new(), Vec::new());
        let add = |list: &mut Vec<String>, name: String| {
            if !list.contains(&name) {
                list.push(name);
            }
        };
        for reference in refs {
            let reference = match reference {
                Ref::Active(inner) => *inner,
                other => other,
            };
            match &reference {
                Ref::Input(path) => {
                    add(
                        &mut inputs,
                        path.iter().map(Key::as_str).collect::<Vec<_>>().join("."),
                    );
                }
                Ref::Surface(path) => {
                    for set in &self.stack.declarations().surface_sets {
                        let named = Ref::Role {
                            set: *set,
                            path: path.clone(),
                        };
                        if let Some(path) = role_path(&named) {
                            add(&mut roles, path.join("."));
                        }
                    }
                }
                other => {
                    if let Some(path) = role_path(other) {
                        add(&mut roles, path.join("."));
                    }
                }
            }
        }
        (roles, inputs)
    }
}

/// A fact's value read for its key, as a preset states it (S11, S17).
#[derive(Clone, Debug)]
pub struct FactValue {
    body: Body,
}

/// What a fact's value states.
#[derive(Clone, Debug)]
enum Body {
    /// A rule's patch of the key's target, stating the key alone.
    Rule(Box<RuleBody>),
    /// A whole part's presence: its absence, its substitution or its being
    /// drawn again.
    Part(PartPresence),
    /// An input's or one of KDE's lists' value under both schemes, as a
    /// layer's bases state it.
    Inputs(InputBases),
}

/// The units a fact's value may carry in its text (S17: `"4px"`), and the
/// suffix S11 spells a key with them by.
const UNITS: [(&str, &str); 4] = [("px", "_px"), ("ms", "_ms"), ("deg", "_deg"), ("pt", "_pt")];

/// A number written with a unit (`"4px"`, `"-1.5pt"`): the number and the
/// unit's key suffix.
fn with_unit(text: &str) -> Option<(f64, &'static str)> {
    UNITS.iter().find_map(|(unit, suffix)| {
        let number = text.strip_suffix(unit)?;
        number.trim().parse::<f64>().ok().map(|n| (n, *suffix))
    })
}

/// `value` nested under `path`, outermost first.
fn nested(path: &[String], value: Item) -> Item {
    path.iter().rev().fold(value, |inner, segment| {
        let mut table = Table::new();
        table.insert(segment.clone(), inner);
        Item::Table(table)
    })
}

/// The ways S11 spells the place `last` with `value` (S17: the key carries
/// no suffix): a unit's number under its suffixed key; anything else under
/// the bare key, else a suffixed one, a one-unit field's every value taking
/// its suffix.
fn spellings(last: &str, value: &Item) -> Vec<(String, Item)> {
    if let Some((number, suffix)) = value.as_str().and_then(with_unit) {
        let number = if number.fract() == 0.0 && number.abs() < 1e15 {
            format!("{number:.0}")
                .parse::<i64>()
                .map_or(Item::Float(number), Item::Integer)
        } else {
            Item::Float(number)
        };
        return vec![(format!("{last}{suffix}"), number)];
    }
    std::iter::once(last.to_owned())
        .chain(UNITS.iter().map(|(_, suffix)| format!("{last}{suffix}")))
        .map(|key| (key, value.clone()))
        .collect()
}

/// A rule's body stating `value` at `path` below `target`, as the first of
/// S11's spellings the reader takes. Where none does, the problems of the
/// first spelling whose key the reader knows (a one-unit field's suffixed
/// key for an expression, S11), else the first's.
fn rule_at(target: &str, path: &[String], value: &Item) -> Result<RuleBody, Unread> {
    let Some((last, outer)) = path.split_last() else {
        return Err(Unread::Refused("an empty key".to_owned()));
    };
    let mut tried: Vec<Vec<Problem>> = Vec::new();
    for (key, spelled) in spellings(last, value) {
        let mut inner = Table::new();
        inner.insert(key, spelled);
        let set = nested(outer, Item::Table(inner));
        match read_rule_body(target, &set) {
            Ok(body) => return Ok(body),
            Err(problems) => tried.push(problems),
        }
    }
    let known = tried.iter().position(|problems| {
        !problems
            .iter()
            .any(|problem| problem.kind == ProblemKind::UnknownKey)
    });
    let chosen = known.and_then(|index| tried.get(index)).or(tried.first());
    Err(Unread::of(chosen.map(Vec::as_slice).unwrap_or_default()))
}

/// A leaf of a block S6.1 sets whole, an `Option<B>` struct block
/// (`token.behavior.key_repeat.delay`): its facts are stated per leaf
/// (S17), and the gate composes the leaves' applying facts into one value
/// of the block per combination.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SetWhole {
    /// The block's key (`token.behavior.key_repeat`).
    pub block: String,
    /// The block's leaves: each one's key and its field's name and unit
    /// suffix (S11).
    pub leaves: Vec<(String, String, &'static str)>,
    key: FactKey,
}

/// The table of the place `key` names below, and the place's path.
fn place_fields(key: &FactKey) -> Option<(&'static [registry::FieldInfo], &[String])> {
    match &key.place {
        Place::Token(path) => Some((registry::tokens(), path)),
        Place::Values(widget, path) => {
            let name = widget_name(&WidgetRef::Typed(*widget));
            registry::widget(&name).map(|info| (info.values, path.as_slice()))
        }
        Place::Part(part, path) => part.info().map(|info| (info.fields, path.as_slice())),
        Place::WholePart(_) | Place::Role(..) | Place::Input(_) | Place::KdeList(_) => None,
    }
}

/// The unit suffix S11 spells a field of type `ty` with, for a value with
/// no unit of its own (an expression): `Value<Ms>`'s `_ms`, …
fn suffix_of(ty: &str) -> &'static str {
    match ty {
        "Value<Ms>" => "_ms",
        "Value<Px>" => "_px",
        "Value<Deg>" => "_deg",
        _ => "",
    }
}

/// The set-whole block `key` is a leaf of, if any (S6.1, S17).
#[must_use]
pub fn set_whole(key: &str) -> Option<SetWhole> {
    let parsed = parse_key(key).ok()?;
    let (fields, path) = place_fields(&parsed)?;
    let (_, above) = path.split_last()?;
    let depth = (1..=above.len()).find(|n| {
        above.get(..*n).is_some_and(|head| {
            matches!(
                registry::find(fields, &head.join(".")).map(|info| info.ty),
                Some(registry::FieldType::Block {
                    ty,
                    form: registry::BlockForm::Struct,
                    ..
                }) if ty.starts_with("Option<")
            )
        })
    })?;
    let head = above.get(..depth)?;
    let info = registry::find(fields, &head.join("."))?;
    let registry::FieldType::Block { fields: inner, .. } = info.ty else {
        return None;
    };
    let below = path.len().checked_sub(depth)?;
    let block = segments(key).ok().and_then(|all| {
        let keep = all.len().checked_sub(below)?;
        all.get(..keep).map(|head| head.join("."))
    })?;
    let leaves = inner
        .iter()
        .map(|leaf| {
            let ty = match leaf.ty {
                registry::FieldType::Leaf(ty)
                | registry::FieldType::Block { ty, .. }
                | registry::FieldType::Patch { ty, .. } => ty,
            };
            (
                format!("{block}.{}", leaf.path),
                leaf.path.to_owned(),
                suffix_of(ty),
            )
        })
        .collect();
    let key = parse_key(&block).ok()?;
    Some(SetWhole { block, leaves, key })
}

impl SetWhole {
    /// The block's value composed from each leaf's value as its fact
    /// states it (S17: `"500ms"`, an expression), in the leaves' order.
    pub fn compose(&self, values: &[&Item]) -> Result<FactValue, Unread> {
        let mut table = Table::new();
        for ((_, name, suffix), value) in self.leaves.iter().zip(values) {
            let (spelled, item) = match value.as_str().and_then(with_unit) {
                Some(_) => spellings(name, value)
                    .into_iter()
                    .next()
                    .unwrap_or_else(|| (name.clone(), (*value).clone())),
                None => (format!("{name}{suffix}"), (*value).clone()),
            };
            table.insert(spelled, item);
        }
        parse_value(&self.key, &Item::Table(table))
    }
}

/// A fact's value read for `key` (S11, S17).
pub fn parse_value(key: &FactKey, value: &Item) -> Result<FactValue, Unread> {
    let body = match &key.place {
        Place::WholePart(part) => {
            let set = match value {
                Item::String(text) if text == "absent" || text == "present" => {
                    let mut table = Table::new();
                    table.insert("absent".to_owned(), Item::Boolean(text == "absent"));
                    Item::Table(table)
                }
                other => other.clone(),
            };
            match read_rule_body(&part_target(*part), &set)
                .map_err(|problems| Unread::of(&problems))?
            {
                RuleBody::Part(
                    _,
                    PartPatch {
                        presence: Some(presence),
                        style,
                    },
                ) if *style == PartStylePatch::default() => Body::Part(presence),
                _ => {
                    return Err(Unread::Refused(
                        "a whole part's value is `absent`, `present` or its `drawn_as`".to_owned(),
                    ));
                }
            }
        }
        Place::Part(part, path) => Body::Rule(Box::new(rule_at(
            &part_target(*part),
            path,
            &whole_block(key, value),
        )?)),
        Place::Values(widget, path) => {
            let target = format!("{}.values", widget_name(&WidgetRef::Typed(*widget)));
            Body::Rule(Box::new(rule_at(&target, path, &whole_block(key, value))?))
        }
        Place::Token(path) => {
            Body::Rule(Box::new(rule_at("tokens", path, &whole_block(key, value))?))
        }
        Place::Role(_, path) => Body::Rule(Box::new(rule_at("colors", path, value)?)),
        Place::Input(name) => Body::Inputs(inputs_at(&segments_of(name), value)?),
        Place::KdeList(list) => {
            let path = ["kde_scheme".to_owned(), list.name().to_owned()];
            Body::Inputs(inputs_at(&path, value)?)
        }
    };
    Ok(FactValue { body })
}

/// A fact's value as a rule's `set` states it: a table at a `Value` struct
/// block is the whole block (S17: "by a table (the whole block, as the
/// preset layer's bases spell it, S2.2)"), which a rule states as
/// `{ replace = { … } }`, a bare table there being a `Fields` patch (S2.2,
/// S11); a paint, an enum block and every other value as written.
fn whole_block(key: &FactKey, value: &Item) -> Item {
    let struct_block = place_fields(key).is_some_and(|(fields, path)| {
        matches!(
            registry::find_entry(fields, &path.join(".")),
            Some((
                registry::FieldInfo {
                    ty: registry::FieldType::Block {
                        ty,
                        block,
                        form: registry::BlockForm::Struct,
                        ..
                    },
                    ..
                },
                0,
            )) if ty.starts_with("Value<") && matches!(Ty::block(block), Ty::Block(_))
        )
    });
    match value {
        Item::Table(_) if struct_block => nested(&["replace".to_owned()], value.clone()),
        other => other.clone(),
    }
}

/// An input's name's segments.
fn segments_of(name: &InputName) -> Vec<String> {
    name.segments()
        .iter()
        .map(|key| key.as_str().to_owned())
        .collect()
}

/// The inputs an `[inputs]` table states with `value` under both schemes
/// at `path` (S11).
fn inputs_at(path: &[String], value: &Item) -> Result<InputBases, Unread> {
    let mut schemes = Table::new();
    for scheme in Scheme::ALL {
        schemes.insert(scheme.name().to_owned(), value.clone());
    }
    read_inputs_table(&nested(path, Item::Table(schemes))).map_err(|problems| Unread::of(&problems))
}

/// One of KDE's lists `bases` states under `scheme`.
fn list_tree(bases: &InputBases, list: KdeList, scheme: Scheme) -> Option<Tree> {
    match list {
        KdeList::Groups => Tree::of(&bases.kde_groups.get(scheme)),
        KdeList::InactiveGroups => Tree::of(&bases.kde_inactive_groups.get(scheme)),
        KdeList::UnsetKeys => Tree::of(&bases.kde_unset_keys.get(scheme)),
    }
}

/// A part's rule target, `<widget>.<part>`.
fn part_target(part: TypedPart) -> String {
    format!(
        "{}.{}",
        widget_name(&WidgetRef::Typed(part.widget())),
        part.name()
    )
}

/// What the gate found comparing a key with a fact (S17).
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    /// The theme's value agrees with the fact's.
    Agrees,
    /// They differ: the first leaf that differs, and each side's value.
    Differs {
        /// The leaf's path in the value.
        at: String,
        /// The theme's value.
        theme: String,
        /// The fact's value, evaluated where the theme's is.
        fact: String,
    },
    /// The key's part is not drawn, or drawn by another widget, where the
    /// comparison needs it drawn: a measured derivation's pass, whose
    /// evidence measured the part drawn, fails so (S17, ADD-A130).
    NotDrawn,
}

/// A preset resolved for one combination's conditions (S17).
#[derive(Clone, Debug)]
pub struct Resolved {
    theme: ResolvedTheme,
    stack: Stack,
    window: WindowConditions,
    /// Each combination context read so far, as [`context_chain`] reads
    /// it: a gate's run queries millions of combinations in a few contexts.
    contexts: Arc<Mutex<Vec<ContextRead>>>,
}

/// A combination context and its ancestors as [`context_chain`] reads it.
type ContextRead = (Item, Result<Vec<ContextElement>, Unread>);

/// A value the gate read, a part's or a widget value's tree (whole, or at
/// a key's place), and the places its evaluation read a synthetic layer's
/// step at (S17's synthetic route, ADD-A128).
struct Read {
    tree: Option<Tree>,
    reached: Vec<Reached>,
}

/// Whether a value's evaluation read a synthetic layer's step at the place
/// `path` of `target`, above it or below it: any step (`reaches`), or one
/// deciding the place itself (`decides`); a step at or below a place
/// `except` names below `path` left out, a later fact of its own key
/// stating it (S17 *Which fact applies*).
#[derive(Clone, Copy, Debug, Default)]
struct Reach {
    reaches: bool,
    decides: bool,
}

impl Reach {
    fn of(reached: &[Reached], target: Target, path: &[String], except: &[Vec<String>]) -> Reach {
        let meets = reached.iter().filter(|held| {
            held.meets(target, path)
                && !except.iter().any(|below| {
                    held.path
                        .strip_prefix(path)
                        .is_some_and(|rest| rest.starts_with(below))
                })
        });
        let (mut reaches, mut decides) = (false, false);
        for held in meets {
            reaches = true;
            decides |= held.direct;
        }
        Reach { reaches, decides }
    }

    /// What one value's evaluation read, a role's or an input's: every
    /// record counts, wherever its place (a token a role's or an input's
    /// value reads is its own place), and the value's own place, the
    /// roles', decides.
    fn whole(reached: &[Reached]) -> Reach {
        Reach {
            reaches: !reached.is_empty(),
            ..Reach::of(reached, Target::Roles, &[], &[])
        }
    }
}

/// The place a fact's value is evaluated in beside the theme's own steps,
/// stated after every other step and kept for every element.
const LAST: Pos = Pos {
    rule: true,
    layer: usize::MAX,
    index: usize::MAX,
};

/// The step `body` states, at its target's root.
fn extra_step(body: &RuleBody) -> Option<Fold<'_>> {
    let (target, found) = match body {
        RuleBody::Colors(roles) => (Target::Roles, Found::Node(&**roles)),
        RuleBody::Tokens(tokens) => (Target::Tokens, Found::Node(&**tokens)),
        RuleBody::Part(PartRef::Typed(part), patch) => {
            (Target::Part(*part), Found::Node(&*patch.style))
        }
        RuleBody::Values(ValuesPatch::Typed(WidgetValuesPatch::Button(values))) => {
            (Target::Values(WidgetKind::Button), Found::Node(&**values))
        }
        RuleBody::Part(..) => return None,
    };
    Some(Fold {
        pos: LAST,
        target,
        found,
    })
}

impl Resolved {
    /// Whether `when`'s conditions hold, as resolution decided them.
    #[must_use]
    pub fn conditions_hold(&self, when: &When) -> bool {
        self.stack.conditions_hold(&self.window, &when.0)
    }

    /// The scope a token, a role or an input is read in: the root's.
    fn root(&self, root: &RootInputs) -> Scope {
        self.theme.root_scope(root)
    }

    /// A token's, a role's or an input's value read in the root scope:
    /// the theme's own, or `value`'s evaluated in its place, with the path
    /// of the key's place in it, and whether its evaluation read a
    /// synthetic layer's step there, but at the places below it `except`
    /// names (S17, ADD-A128).
    fn global_tree(
        &self,
        key: &FactKey,
        value: Option<&FactValue>,
        root: &RootInputs,
        except: &[Vec<String>],
    ) -> Result<(Option<Tree>, Vec<String>, Reach), Unread> {
        let scope = self.root(root);
        let applies: &dyn Fn(&Selector) -> bool = &|_| true;
        let theme = &self.theme;
        let read = |extra: Option<Extra<'_>>| -> (Option<Tree>, Reach) {
            match &key.place {
                Place::Token(path) => {
                    theme.evaluate_in(&scope, None, applies, Target::Tokens, extra, |cx| {
                        let folds = cx.roots(Target::Tokens);
                        let tree = Tree::of(&Tokens::resolve(&folds, cx));
                        (tree, Reach::of(cx.reached(), Target::Tokens, path, except))
                    })
                }
                // A role naming a map (`role.global.accent_ramp`) is read in
                // the colour roles resolved whole, so a fact stating it is
                // compared whole (S17); a colour role through its reference,
                // as an expression reads it (S3.3).
                Place::Role(_, path) if names_map(path) => {
                    theme.evaluate_in(&scope, None, applies, Target::Roles, extra, |cx| {
                        let folds = cx.roots(Target::Roles);
                        let tree = Tree::of(&ColorRoles::resolve(&folds, cx));
                        (tree, Reach::of(cx.reached(), Target::Roles, path, except))
                    })
                }
                Place::Role(reference, _) => {
                    theme.evaluate_in(&scope, None, applies, Target::Roles, extra, |cx| {
                        let tree = Tree::of(&cx.color_of(reference));
                        (tree, Reach::whole(cx.reached()))
                    })
                }
                Place::Input(name) => {
                    let reference = Ref::Input(name.segments().to_vec());
                    // The input's own value decides where its folded
                    // synthetic value is in force, the mark saying so, a
                    // listed input (S17, ADD-A132): no synthetic layer
                    // states an input. What it reads decides nothing of it.
                    let decides = extra.is_none() && theme.synthetic_inputs.contains(name);
                    theme.evaluate_in(&scope, None, applies, Target::Roles, extra, |cx| {
                        let tree = Tree::of(&cx.color_of(&reference));
                        let reach = Reach {
                            decides,
                            ..Reach::whole(cx.reached())
                        };
                        (tree, reach)
                    })
                }
                Place::KdeList(list) => {
                    let lists = &theme.kde_lists;
                    let tree = match list {
                        KdeList::Groups => Tree::of(&lists.groups),
                        KdeList::InactiveGroups => Tree::of(&lists.inactive_groups),
                        KdeList::UnsetKeys => Tree::of(&lists.unset_keys),
                    };
                    (tree, Reach::default())
                }
                Place::WholePart(_) | Place::Part(..) | Place::Values(..) => {
                    (None, Reach::default())
                }
            }
        };
        let path = match &key.place {
            Place::Token(path) => path.clone(),
            Place::Role(_, path) if names_map(path) => path.clone(),
            Place::Role(..) | Place::Input(_) | Place::KdeList(_) => Vec::new(),
            Place::WholePart(_) | Place::Part(..) | Place::Values(..) => {
                return Err(Unread::Refused(
                    "a widget's key is read with its element".to_owned(),
                ));
            }
        };
        let Some(value) = value else {
            let (tree, reach) = read(None);
            return Ok((tree, path, reach));
        };
        let scheme = theme.scheme;
        let contrast = self.stack.contrast();
        // The fact's step is read at the place its key names below the
        // target's root (a role's below the colour roles), the tree read
        // there being the key's own value (`path` below it).
        let place = match &key.place {
            Place::Role(_, below) => below.as_slice(),
            _ => path.as_slice(),
        };
        let (tree, reach) = match (&value.body, &key.place) {
            (Body::Rule(body), _) => read(extra_step(body).map(|fold| Extra::Step(fold, place))),
            (Body::Inputs(bases), Place::Input(name)) => bases
                .entries
                .iter()
                .find(|(held, _)| held == name)
                .and_then(|(_, stated)| input_value(stated, scheme, contrast))
                .map_or_else(
                    || (None, Reach::default()),
                    |color| read(Some(Extra::Input(name, &color))),
                ),
            (Body::Inputs(bases), Place::KdeList(list)) => {
                (list_tree(bases, *list, scheme), Reach::default())
            }
            (Body::Inputs(_) | Body::Part(_), _) => (None, Reach::default()),
        };
        Ok((tree, path, reach))
    }

    /// Whether a token's, a role's or an input's value, or `value`
    /// evaluated in its place, reaches a synthetic value (S17, ADD-A128):
    /// its evaluation reads a step of a synthetic layer, through any
    /// reference, call, binding, rule or base, but at the places below the
    /// key `except` names, each by its path below the key's, which a later
    /// fact of their own keys states (S17 *Which fact applies*). Never
    /// without a `[pending]` table.
    pub fn global_synthetic(
        &self,
        key: &FactKey,
        value: Option<&FactValue>,
        root: &RootInputs,
        except: &[Vec<String>],
    ) -> Result<bool, Unread> {
        for value in [None, value] {
            if self.global_tree(key, value, root, except)?.2.reaches {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// The scheme an input's key is listed at (`inputs.<name>.<scheme>`,
    /// S17, ADD-A132): the scheme in force, as S11 writes it; `None` for
    /// any other key.
    #[must_use]
    pub fn listed_scheme(&self, key: &FactKey) -> Option<&'static str> {
        matches!(key.place, Place::Input(_)).then(|| self.theme.scheme.name())
    }

    /// Whether a synthetic layer's step decides a token's or a role's
    /// value itself (S17, ADD-A128): the preset leaves its place, or a
    /// block it is a leaf of, unstated here, a listed path.
    pub fn global_listed(&self, key: &FactKey, root: &RootInputs) -> Result<bool, Unread> {
        Ok(self.global_tree(key, None, root, &[])?.2.decides)
    }

    /// A token's, a role's or an input's value, read in the root scope,
    /// compared with `value`'s evaluated there (S17).
    pub fn compare_global(
        &self,
        key: &FactKey,
        value: &FactValue,
        root: &RootInputs,
        tolerance: &Tolerance,
    ) -> Result<Outcome, Unread> {
        let (actual, path, _) = self.global_tree(key, None, root, &[])?;
        let (expected, _, _) = self.global_tree(key, Some(value), root, &[])?;
        let tolerance = in_scope(&self.theme, &self.root(root), tolerance);
        differs(actual.as_ref(), expected.as_ref(), &path, &tolerance)
    }

    /// [`Resolved::compare_global`] but at the places below the key
    /// `except` names, each by its path below the key's, which a later
    /// fact of their own keys states (S17 *Which fact applies*: the last
    /// fact in file order whose `when` holds, a block's leaf stated after
    /// the block): their own keys compare them.
    pub fn compare_global_except(
        &self,
        key: &FactKey,
        value: &FactValue,
        root: &RootInputs,
        tolerance: &Tolerance,
        except: &[Vec<String>],
    ) -> Result<Outcome, Unread> {
        let (actual, path, _) = self.global_tree(key, None, root, &[])?;
        let (mut expected, _, _) = self.global_tree(key, Some(value), root, &[])?;
        if let (Some(expected), Some(actual)) = (expected.as_mut(), actual.as_ref()) {
            for below in except {
                let at: Vec<String> = path.iter().chain(below).cloned().collect();
                expected.graft(&at, actual);
            }
        }
        let tolerance = in_scope(&self.theme, &self.root(root), tolerance);
        differs(actual.as_ref(), expected.as_ref(), &path, &tolerance)
    }

    /// A measured derivation's pass (S17 *A measured derivation*): the
    /// fact's `value` evaluated in the key's place, its operands this
    /// resolution's, compared with the pass's measured `result`.
    pub fn compare_global_pass(
        &self,
        key: &FactKey,
        value: &FactValue,
        result: &FactValue,
        root: &RootInputs,
        tolerance: &Tolerance,
    ) -> Result<Outcome, Unread> {
        let (measured, path, _) = self.global_tree(key, Some(result), root, &[])?;
        let (derived, _, _) = self.global_tree(key, Some(value), root, &[])?;
        let tolerance = in_scope(&self.theme, &self.root(root), tolerance);
        differs(measured.as_ref(), derived.as_ref(), &path, &tolerance)
    }

    /// The combination's element queried top-down, each part in the scope
    /// the query of the part enclosing it returns (S8), measured by
    /// `metrics`.
    pub fn session<'r>(
        &'r self,
        combination: &Combination,
        root: &RootInputs,
        metrics: &'r dyn FontMetrics,
    ) -> Result<Session<'r>, Unread> {
        let widget = widget_kind(&combination.widget)?;
        let parsed = Parsed::of(combination)?;
        let ancestors = match &combination.context {
            Some(context) => self.context_chain(context)?,
            None => Vec::new(),
        };
        let mut scope = self.root(root);
        for ancestor in &ancestors {
            scope = self.context_scope(ancestor, scope, metrics)?;
        }
        let mut session = Session {
            resolved: self,
            metrics,
            parsed,
            widget,
            parts: Vec::new(),
            values: OnceCell::new(),
            evaluated: RefCell::new(Vec::new()),
        };
        let info = registry::widgets()
            .iter()
            .find(|info| info.kind == widget)
            .ok_or_else(|| {
                Unread::NotHeld(format!("the model holds no `{}` yet", combination.widget))
            })?;
        for part_info in info.parts {
            let Some(part) = TypedPart::from_names(widget, part_info.name) else {
                continue;
            };
            let queried_in = match part.parent() {
                None => scope.clone(),
                Some(parent) => session
                    .parts
                    .iter()
                    .find(|held| held.part == parent)
                    .map_or_else(|| scope.clone(), Queried::children_scope),
            };
            let answer =
                session.with_element(part, &queried_in, |element| self.theme.style(element));
            let (answer, returned) = match answer {
                Styled::Style((_, returned)) => (Answer::Drawn, Some(returned)),
                Styled::NotDrawn => (Answer::NotDrawn, None),
                Styled::DrawnAs { target, variants } => {
                    (Answer::DrawnAs { target, variants }, None)
                }
                Styled::NotAvailable(unavailable) => (Answer::NotAvailable(unavailable), None),
            };
            session.parts.push(Queried {
                part,
                scope: queried_in,
                returned,
                answer,
                actual: OnceCell::new(),
            });
        }
        Ok(session)
    }

    /// [`context_chain`] of `context`, read once for every combination in
    /// it.
    fn context_chain(&self, context: &Item) -> Result<Vec<ContextElement>, Unread> {
        let mut contexts = self.contexts.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some((_, read)) = contexts.iter().find(|(held, _)| held == context) {
            return read.clone();
        }
        let read = context_chain(context);
        contexts.push((context.clone(), read.clone()));
        read
    }

    /// The scope inside a context's ancestor `context`: its element
    /// queried in `scope`.
    fn context_scope(
        &self,
        context: &ContextElement,
        scope: Scope,
        metrics: &dyn FontMetrics,
    ) -> Result<Scope, Unread> {
        let mut element = Element::new(PartRef::Typed(context.part), &scope, metrics)
            .with_variants(&context.variants)
            .with_states(context.states.clone(), &[]);
        if let Some(size) = context.size {
            element = element.with_size(size);
        }
        match self.theme.style(&element) {
            Styled::Style((_, inside)) => Ok(inside),
            other => Err(Unread::Refused(format!(
                "the context's ancestor is {other:?}"
            ))),
        }
    }
}

/// A combination context's ancestor (S11's context selector, as a state
/// model's `axes.contexts` writes it): its widget's or part's element,
/// holding the context's variants, states and size.
#[derive(Clone, Debug)]
struct ContextElement {
    part: TypedPart,
    variants: Vec<VariantRef>,
    states: StateFlags,
    size: Option<ControlSize>,
}

/// Whether the model holds what the combination context `context` names:
/// [`Resolved::session`] skips each combination in a context naming a
/// widget or part it does not hold yet, an ancestor of a chain among them,
/// once the combination's names read ([`names_read`]).
pub fn context_held(context: &Item) -> bool {
    !matches!(context_chain(context), Err(Unread::NotHeld(_)))
}

/// Whether the names `combination` holds read, as [`Resolved::session`]
/// reads them before its context.
pub fn names_read(combination: &Combination) -> bool {
    Parsed::of(combination).is_ok()
}

/// The ancestors a combination's `context` names, outermost first, read:
/// a state model's `axes.contexts` entry is one ancestor's selector, or a
/// list of them, a chain, each the element inside the one before (S17).
fn context_chain(context: &Item) -> Result<Vec<ContextElement>, Unread> {
    match context {
        Item::Array(chain) if chain.is_empty() => {
            Err(Unread::Refused("an empty context chain".to_owned()))
        }
        Item::Array(chain) => chain.iter().map(context_element).collect(),
        ancestor => Ok(vec![context_element(ancestor)?]),
    }
}

/// The ancestor one selector of a combination's `context` names, read.
fn context_element(context: &Item) -> Result<ContextElement, Unread> {
    let mut table = Table::new();
    table.insert("context".to_owned(), Item::Array(vec![context.clone()]));
    let When(selector) = parse_when(&Item::Table(table))?;
    let Some(context) = selector.context.first() else {
        return Err(Unread::Refused("an empty context".to_owned()));
    };
    let part = match &context.target {
        Some(WidgetOrPart::Part(PartRef::Typed(part))) => *part,
        Some(WidgetOrPart::Widget(WidgetRef::Typed(kind))) => registry::widgets()
            .iter()
            .find(|info| info.kind == *kind)
            .and_then(|info| info.parts.iter().find(|part| part.root))
            .and_then(|root| TypedPart::from_names(*kind, root.name))
            .ok_or_else(|| Unread::NotHeld("a context widget with no root".to_owned()))?,
        _ => {
            return Err(Unread::NotHeld(
                "a context naming no typed widget the gate can draw".to_owned(),
            ));
        }
    };
    let variants = context.variants.all_of.clone();
    let mut states = StateFlags::new();
    for flag in &context.states.all_of {
        states = states.with(flag.clone());
    }
    Ok(ContextElement {
        part,
        variants,
        states,
        size: context.sizes.all_of.first().copied(),
    })
}

/// What the query answered for a part.
#[derive(Clone, Debug, PartialEq)]
enum Answer {
    Drawn,
    NotDrawn,
    DrawnAs {
        target: WidgetOrPart,
        variants: Vec<VariantRef>,
    },
    NotAvailable(Unavailable),
}

/// One part queried for a combination.
struct Queried {
    part: TypedPart,
    /// The scope it is queried in.
    scope: Scope,
    /// The scope its query returned, where it is drawn.
    returned: Option<Scope>,
    answer: Answer,
    /// The theme's own resolved style, read once.
    actual: OnceCell<Rc<Read>>,
}

impl Queried {
    /// The scope its children are queried in: the one its query returned,
    /// else its own, as a part not drawn passes its parent's on (S8).
    fn children_scope(&self) -> Scope {
        self.returned.clone().unwrap_or_else(|| self.scope.clone())
    }
}

/// A combination of a widget's state model (S17), as the gate enumerates
/// it, each name spelled as S11 spells it.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Combination {
    /// The widget.
    pub widget: String,
    /// The variants.
    pub variants: Vec<String>,
    /// The state flags.
    pub states: Vec<String>,
    /// The order the flags were set in, the latest last (S7.2).
    pub order: Vec<String>,
    /// The control size; `None`: the declared default.
    pub size: Option<String>,
    /// The density; `None`: the default.
    pub density: Option<String>,
    /// The direction: `ltr` or `rtl`; `None`: the root's.
    pub direction: Option<String>,
    /// The ancestor context, as S11's context selector writes it, or a
    /// chain of them, a list, outermost first (S17's `axes.contexts`).
    pub context: Option<Item>,
    /// How many siblings of the same part precede the element.
    pub before: usize,
    /// How many follow it.
    pub after: usize,
}

impl Combination {
    /// The combination of `widget` holding nothing: no variant, no flag, no
    /// size, density, direction or context, no siblings.
    #[must_use]
    pub fn new(widget: &str) -> Combination {
        Combination {
            widget: widget.to_owned(),
            ..Combination::default()
        }
    }
}

/// A combination's names read into the model's.
#[derive(Clone, Debug)]
struct Parsed {
    variants: Vec<VariantRef>,
    states: StateFlags,
    order: Vec<StateFlag>,
    size: Option<ControlSize>,
    density: Option<Density>,
    direction: Option<Direction>,
    before: usize,
    after: usize,
}

impl Parsed {
    fn of(combination: &Combination) -> Result<Parsed, Unread> {
        let mut cx = Reader::default();
        let variants: Vec<VariantRef> = combination
            .variants
            .iter()
            .filter_map(|name| when::variant(name, "variants", &mut cx))
            .collect();
        let mut states = StateFlags::new();
        for name in &combination.states {
            if let Some(flag) = when::state(name, "states", &mut cx) {
                states = states.with(flag);
            }
        }
        let order: Vec<StateFlag> = combination
            .order
            .iter()
            .filter_map(|name| when::state(name, "order", &mut cx))
            .collect();
        let size = combination
            .size
            .as_deref()
            .and_then(|name| when::named::<ControlSize>(name, "size", &mut cx));
        let density = combination
            .density
            .as_deref()
            .and_then(|name| when::density(name, "density", &mut cx));
        let direction = combination
            .direction
            .as_deref()
            .and_then(|name| when::named::<Direction>(name, "direction", &mut cx));
        if cx.failed() {
            return Err(Unread::of(&cx.into_problems()));
        }
        Ok(Parsed {
            variants,
            states,
            order,
            size,
            density,
            direction,
            before: combination.before,
            after: combination.after,
        })
    }
}

/// The theme's and the fact's values at `path` compared, a point's
/// tolerance in the px a point resolves to; refused where the gate cannot
/// read either side there, so two sides it cannot read never agree.
fn differs(
    actual: Option<&Tree>,
    expected: Option<&Tree>,
    path: &[String],
    tolerance: &Tolerance,
) -> Result<Outcome, Unread> {
    differs_there(
        actual.and_then(|tree| tree.at(path)),
        expected.and_then(|tree| tree.at(path)),
        path,
        tolerance,
    )
}

/// [`differs`] of the two sides' subtrees at `path`, each where read.
fn differs_there(
    actual: Option<&Tree>,
    expected: Option<&Tree>,
    path: &[String],
    tolerance: &Tolerance,
) -> Result<Outcome, Unread> {
    let unread = |side: &str| {
        Unread::Refused(format!(
            "the gate cannot read {side} value at `{}`",
            path.join(".")
        ))
    };
    let actual = actual.ok_or_else(|| unread("the theme's"))?;
    let expected = expected.ok_or_else(|| unread("the fact's"))?;
    Ok(match actual.differs(expected, tolerance) {
        None => Outcome::Agrees,
        Some(at) => Outcome::Differs {
            at,
            theme: actual.to_string(),
            fact: expected.to_string(),
        },
    })
}

/// `tolerance` where a point resolves as it does in `scope` (S9.4).
fn in_scope(theme: &ResolvedTheme, scope: &Scope, tolerance: &Tolerance) -> Tolerance {
    match theme.px_per_point(scope) {
        Some(px) => tolerance.at_points_of(f64::from(px)),
        None => tolerance.clone(),
    }
}

/// One combination's element, its parts queried (S17).
pub struct Session<'r> {
    resolved: &'r Resolved,
    metrics: &'r dyn FontMetrics,
    parsed: Parsed,
    widget: WidgetKind,
    parts: Vec<Queried>,
    /// The theme's own resolved values, read once.
    values: OnceCell<Rc<Read>>,
    /// Each fact's body evaluated in its key's place, read once.
    evaluated: RefCell<Vec<(RuleBody, Rc<Read>)>>,
}

impl Session<'_> {
    /// `f` over the element of `part` in `scope`, holding the combination.
    fn with_element<R>(
        &self,
        part: TypedPart,
        scope: &Scope,
        f: impl FnOnce(&Element<'_>) -> R,
    ) -> R {
        let parsed = &self.parsed;
        let count = parsed.before.saturating_add(parsed.after).saturating_add(1);
        let siblings: Vec<Sibling<'_>> = (0..count)
            .map(|_| Sibling::new(PartRef::Typed(part), &[], StateFlags::new()))
            .collect();
        let mut element = Element::new(PartRef::Typed(part), scope, self.metrics)
            .with_variants(&parsed.variants)
            .with_states(parsed.states.clone(), &parsed.order)
            .with_siblings(&siblings, parsed.before);
        if let Some(size) = parsed.size {
            element = element.with_size(size);
        }
        if let Some(density) = &parsed.density {
            element = element.with_density(density.clone());
        }
        if let Some(direction) = parsed.direction {
            element = element.with_direction(direction);
        }
        f(&element)
    }

    /// The root part's query.
    fn root(&self) -> Option<&Queried> {
        self.parts.first()
    }

    /// The query of `part`.
    fn queried(&self, part: TypedPart) -> Option<&Queried> {
        self.parts.iter().find(|held| held.part == part)
    }

    /// Why the element is not available, where it is not: its root's
    /// answer (S8).
    #[must_use]
    pub fn unavailable(&self) -> Option<String> {
        match self.root().map(|root| &root.answer) {
            Some(Answer::NotAvailable(unavailable)) => Some(format!("{unavailable:?}")),
            _ => None,
        }
    }

    /// Whether each `when` holds for the element of its key's part (its
    /// root's for a value; a token's, a role's or an input's by its
    /// conditions alone): its conditions as resolution decided them, the
    /// rest matched against the element as the query matches a rule (S8),
    /// each part's element matched once.
    #[must_use]
    pub fn holds_each(&self, whens: &[(&When, &FactKey)]) -> Vec<bool> {
        let mut out: Vec<bool> = whens
            .iter()
            .map(|(when, _)| self.resolved.conditions_hold(when))
            .collect();
        let theme = &self.resolved.theme;
        let root = self.root().map(|root| root.part);
        for queried in &self.parts {
            let of_part = |key: &FactKey| match &key.place {
                Place::WholePart(part) | Place::Part(part, _) => *part == queried.part,
                Place::Values(..) => root == Some(queried.part),
                Place::Token(_) | Place::Role(..) | Place::Input(_) | Place::KdeList(_) => false,
            };
            let indices: Vec<usize> = whens
                .iter()
                .enumerate()
                .filter(|(index, (_, key))| out.get(*index) == Some(&true) && of_part(key))
                .map(|(index, _)| index)
                .collect();
            if indices.is_empty() {
                continue;
            }
            self.with_element(queried.part, &queried.scope, |element| {
                let matched = Matched::new(theme, element).ok();
                for index in indices {
                    let holds = matched.as_ref().is_some_and(|matched| {
                        whens
                            .get(index)
                            .is_some_and(|(when, _)| matched.matches(&when.0))
                    });
                    if let Some(slot) = out.get_mut(index) {
                        *slot = holds;
                    }
                }
            });
        }
        for (slot, (_, key)) in out.iter_mut().zip(whens) {
            let queried = match &key.place {
                Place::WholePart(part) | Place::Part(part, _) => self.queried(*part).is_some(),
                Place::Values(..) => root.is_some(),
                Place::Token(_) | Place::Role(..) | Place::Input(_) | Place::KdeList(_) => true,
            };
            *slot &= queried;
        }
        out
    }

    /// Which of the widget's steps apply to the element, and what each
    /// part's query answered: two combinations with the same signature
    /// resolve every key alike, so the gate compares one of them (S17:
    /// "reusing what the set's rules share").
    #[must_use]
    pub fn signature(&self) -> Vec<u32> {
        let theme = &self.resolved.theme;
        let mut out = Vec::new();
        let index = |at: usize| u32::try_from(at).unwrap_or(u32::MAX);
        for queried in &self.parts {
            out.push(u32::MAX);
            out.push(match &queried.answer {
                Answer::Drawn => 0,
                Answer::NotDrawn => 1,
                Answer::DrawnAs { .. } => 2,
                Answer::NotAvailable(_) => 3,
            });
            let steps = theme.part_steps(queried.part);
            self.with_element(queried.part, &queried.scope, |element| {
                if let Ok(matched) = Matched::new(theme, element) {
                    for (at, step) in steps.iter().enumerate() {
                        if step
                            .selector
                            .as_ref()
                            .is_none_or(|selector| matched.matches(selector))
                        {
                            out.push(index(at));
                        }
                    }
                }
            });
        }
        out.push(u32::MAX);
        if let Some(root) = self.root() {
            let steps = theme
                .values
                .iter()
                .filter(|(held, _)| *held == self.widget)
                .flat_map(|(_, steps)| steps);
            self.with_element(root.part, &root.scope, |element| {
                if let Ok(matched) = Matched::new(theme, element) {
                    for (at, step) in steps.enumerate() {
                        if step
                            .selector
                            .as_ref()
                            .is_none_or(|selector| matched.matches(selector))
                        {
                            out.push(index(at));
                        }
                    }
                }
            });
        }
        out
    }

    /// The key's value for the element compared with the fact's `value`
    /// evaluated in its place; for a whole part, the query's answer with
    /// the fact's absence or substitution, and with none (no fact of the
    /// whole part applies) a drawn part (S17).
    pub fn compare(
        &self,
        key: &FactKey,
        value: Option<&FactValue>,
        tolerance: &Tolerance,
    ) -> Result<Outcome, Unread> {
        match &key.place {
            Place::WholePart(part) => {
                let queried = self
                    .queried(*part)
                    .ok_or_else(|| Unread::Refused("a part the widget does not have".to_owned()))?;
                let presence = value.and_then(|value| match &value.body {
                    Body::Part(presence) => Some(presence),
                    Body::Rule(_) | Body::Inputs(_) => None,
                });
                let expected = match (value, presence) {
                    (None, _) | (Some(_), Some(PartPresence::Present)) => Answer::Drawn,
                    (Some(_), Some(PartPresence::Absent)) => Answer::NotDrawn,
                    (Some(_), Some(PartPresence::DrawnAs { target, variants })) => {
                        Answer::DrawnAs {
                            target: target.clone(),
                            variants: variants.clone(),
                        }
                    }
                    (Some(_), None) => {
                        return Err(Unread::Refused(
                            "a whole part's value is `absent`, `present` or its `drawn_as`"
                                .to_owned(),
                        ));
                    }
                };
                Ok(if queried.answer == expected {
                    Outcome::Agrees
                } else {
                    Outcome::Differs {
                        at: ".".to_owned(),
                        theme: format!("{:?}", queried.answer),
                        fact: format!("{expected:?}"),
                    }
                })
            }
            Place::Part(..) | Place::Values(..) => {
                let Some((actual, path)) = self.field_tree(key, None)? else {
                    return Ok(Outcome::NotDrawn);
                };
                let expected = self.field_tree(key, Some(rule_body(value)?))?;
                differs_there(
                    actual.tree.as_ref(),
                    expected.as_ref().and_then(|(read, _)| read.tree.as_ref()),
                    &path,
                    &self.in_scope(key, tolerance),
                )
            }
            Place::Token(_) | Place::Role(..) | Place::Input(_) | Place::KdeList(_) => Err(
                Unread::Refused("a token, a role or an input is read in the root scope".to_owned()),
            ),
        }
    }

    /// [`Session::compare`] of a part's field or a widget value, but at the
    /// places below the key `except` names, each by its path below the
    /// key's, which a later fact of their own keys states (S17 *Which fact
    /// applies*: the last fact in file order whose `when` holds, a block's
    /// leaf stated after the block): their own keys compare them.
    pub fn compare_except(
        &self,
        key: &FactKey,
        value: &FactValue,
        tolerance: &Tolerance,
        except: &[Vec<String>],
    ) -> Result<Outcome, Unread> {
        if except.is_empty() || !matches!(key.place, Place::Part(..) | Place::Values(..)) {
            return self.compare(key, Some(value), tolerance);
        }
        let Some((actual, path)) = self.field_tree(key, None)? else {
            return Ok(Outcome::NotDrawn);
        };
        let mut expected = self
            .field_tree(key, Some(rule_body(Some(value))?))?
            .and_then(|(read, _)| read.tree.clone());
        if let (Some(expected), Some(actual)) = (expected.as_mut(), actual.tree.as_ref()) {
            for below in except {
                expected.graft(below, actual);
            }
        }
        differs_there(
            actual.tree.as_ref(),
            expected.as_ref(),
            &path,
            &self.in_scope(key, tolerance),
        )
    }

    /// A measured derivation's pass (S17 *A measured derivation*): the
    /// fact's `value` evaluated in the key's place for the element, its
    /// operands this resolution's, compared with the pass's measured
    /// `result`.
    pub fn compare_pass(
        &self,
        key: &FactKey,
        value: &FactValue,
        result: &FactValue,
        tolerance: &Tolerance,
    ) -> Result<Outcome, Unread> {
        let Some((measured, path)) = self.field_tree(key, Some(rule_body(Some(result))?))? else {
            return Ok(Outcome::NotDrawn);
        };
        let derived = self.field_tree(key, Some(rule_body(Some(value))?))?;
        differs_there(
            measured.tree.as_ref(),
            derived.as_ref().and_then(|(read, _)| read.tree.as_ref()),
            &path,
            &self.in_scope(key, tolerance),
        )
    }

    /// `tolerance` where a point resolves as it does in the scope the key's
    /// part (its root's, for a value) is queried in (S9.4).
    fn in_scope(&self, key: &FactKey, tolerance: &Tolerance) -> Tolerance {
        let queried = match &key.place {
            Place::WholePart(part) | Place::Part(part, _) => self.queried(*part),
            Place::Values(..)
            | Place::Token(_)
            | Place::Role(..)
            | Place::Input(_)
            | Place::KdeList(_) => self.root(),
        };
        match queried {
            Some(queried) => in_scope(&self.resolved.theme, &queried.scope, tolerance),
            None => tolerance.clone(),
        }
    }

    /// Whether the key's part is drawn for the element, where the key is a
    /// part's field: its query answered a style of its own (S8). A whole
    /// part's key, a value's, a token's, a role's and an input's are read
    /// whatever the parts' answers.
    #[must_use]
    pub fn drawn(&self, key: &FactKey) -> bool {
        match &key.place {
            Place::Part(part, _) => self
                .queried(*part)
                .is_some_and(|queried| queried.answer == Answer::Drawn),
            Place::WholePart(_)
            | Place::Values(..)
            | Place::Token(_)
            | Place::Role(..)
            | Place::Input(_)
            | Place::KdeList(_) => true,
        }
    }

    /// Whether the key's value for the element, or `value` evaluated in
    /// its place, reaches a synthetic value (S17, ADD-A128): its
    /// evaluation reads a step of a synthetic layer, through any reference,
    /// call, binding, rule or base, or such a step decides whether its part
    /// is drawn; but at the places below the key `except` names, each by
    /// its path below the key's, which a later fact of their own keys
    /// states (S17 *Which fact applies*). Never without a `[pending]` table.
    pub fn synthetic(
        &self,
        key: &FactKey,
        value: Option<&FactValue>,
        except: &[Vec<String>],
    ) -> Result<bool, Unread> {
        let (target, path) = match &key.place {
            Place::WholePart(part) => return Ok(self.shown_synthetic(*part)),
            Place::Part(part, path) => {
                if self.shown_synthetic(*part) {
                    return Ok(true);
                }
                (Target::Part(*part), path)
            }
            Place::Values(widget, path) => (Target::Values(*widget), path),
            Place::Token(_) | Place::Role(..) | Place::Input(_) | Place::KdeList(_) => {
                return Ok(false);
            }
        };
        let body = value.and_then(|value| rule_body(Some(value)).ok());
        for body in [None, body] {
            if let Some((read, _)) = self.field_tree(key, body)?
                && Reach::of(&read.reached, target, path, except).reaches
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Whether a synthetic layer's step decides the key's value for the
    /// element itself (S17, ADD-A128): the preset leaves the place, or a
    /// block it is a leaf of, unstated here, a listed path. A whole part's
    /// key is one where no step for the element states the part's presence
    /// (`absent = true`, `absent = false`, `drawn_as`), a listed presence
    /// (ADD-A131).
    pub fn listed(&self, key: &FactKey) -> Result<bool, Unread> {
        let (target, path) = match &key.place {
            Place::Part(part, path) => (Target::Part(*part), path),
            Place::Values(widget, path) => (Target::Values(*widget), path),
            Place::WholePart(part) => return Ok(!self.presence_stated(*part)),
            Place::Token(_) | Place::Role(..) | Place::Input(_) | Place::KdeList(_) => {
                return Ok(false);
            }
        };
        Ok(self
            .field_tree(key, None)?
            .is_some_and(|(read, _)| Reach::of(&read.reached, target, path, &[]).decides))
    }

    /// Whether a step kept for the element states `part`'s presence:
    /// `Absent`, `Present` or drawn as another (S6.3).
    fn presence_stated(&self, part: TypedPart) -> bool {
        let Some(queried) = self.queried(part) else {
            return false;
        };
        let theme = &self.resolved.theme;
        self.with_element(queried.part, &queried.scope, |element| {
            let Ok(matched) = Matched::new(theme, element) else {
                return false;
            };
            theme.presence_stated(part, |step| {
                step.selector
                    .as_ref()
                    .is_none_or(|selector| matched.matches(selector))
            })
        })
    }

    /// Whether a synthetic layer's step decides whether `part` is drawn
    /// for the element (S6.3, S17, ADD-A128; `shown_by_synthetic`).
    fn shown_synthetic(&self, part: TypedPart) -> bool {
        let Some(queried) = self.queried(part) else {
            return false;
        };
        let theme = &self.resolved.theme;
        self.with_element(queried.part, &queried.scope, |element| {
            let Ok(matched) = Matched::new(theme, element) else {
                return false;
            };
            theme.shown_by_synthetic(part, |step| {
                step.selector
                    .as_ref()
                    .is_none_or(|selector| matched.matches(selector))
            })
        })
    }

    /// Whether `when`, a state model's `not_drawn` entry (S17), holds for
    /// the element of the widget's root: its conditions as resolution
    /// decided them, the rest matched as the query matches a rule (S8),
    /// its contexts among them.
    #[must_use]
    pub fn root_holds(&self, when: &When) -> bool {
        if !self.resolved.conditions_hold(when) {
            return false;
        }
        let Some(root) = self.root() else {
            return false;
        };
        let theme = &self.resolved.theme;
        self.with_element(root.part, &root.scope, |element| {
            Matched::new(theme, element).is_ok_and(|matched| matched.matches(&when.0))
        })
    }

    /// `body` evaluated in its key's place by `evaluate`, once.
    fn evaluated_once(&self, body: &RuleBody, evaluate: impl FnOnce() -> Read) -> Rc<Read> {
        if let Some((_, read)) = self
            .evaluated
            .borrow()
            .iter()
            .find(|(held, _)| held == body)
        {
            return Rc::clone(read);
        }
        let read = Rc::new(evaluate());
        self.evaluated
            .borrow_mut()
            .push((body.clone(), Rc::clone(&read)));
        read
    }

    /// A part field's or a widget value's tree for the element, with the
    /// key's path in it: the theme's own (read once), or with `body`'s step
    /// after every other; `None` where the key's part is not drawn.
    fn field_tree(&self, key: &FactKey, body: Option<&RuleBody>) -> Result<Option<Field>, Unread> {
        let theme = &self.resolved.theme;
        let place: &[String] = match &key.place {
            Place::Part(_, path) | Place::Values(_, path) => path,
            Place::WholePart(_)
            | Place::Token(_)
            | Place::Role(..)
            | Place::Input(_)
            | Place::KdeList(_) => &[],
        };
        let extra = body
            .and_then(extra_step)
            .map(|fold| Extra::Step(fold, place));
        match &key.place {
            Place::Part(part, path) => {
                let queried = self
                    .queried(*part)
                    .ok_or_else(|| Unread::Refused("a part the widget does not have".to_owned()))?;
                if queried.answer != Answer::Drawn {
                    return Ok(None);
                }
                // The whole part, or its tree at the key's place alone.
                let evaluate = |extra: Option<Extra<'_>>, at: Option<&[String]>| -> Read {
                    self.with_element(queried.part, &queried.scope, |element| {
                        let Ok(matched) = Matched::new(theme, element) else {
                            return Read {
                                tree: None,
                                reached: Vec::new(),
                            };
                        };
                        let applies = |selector: &Selector| matched.matches(selector);
                        theme.evaluate_in(
                            element.scope,
                            Some(element),
                            &applies,
                            Target::Part(queried.part),
                            extra,
                            |cx| {
                                let resolved = cx.part(queried.part);
                                let tree = match at {
                                    None => Tree::of(&resolved),
                                    Some(path) => Tree::of_at(&resolved, path),
                                };
                                Read {
                                    tree,
                                    reached: cx.reached().to_vec(),
                                }
                            },
                        )
                    })
                };
                let read = match body {
                    None => {
                        let whole = queried.actual.get_or_init(|| Rc::new(evaluate(None, None)));
                        Rc::new(Read {
                            tree: whole.tree.as_ref().and_then(|tree| tree.at(path)).cloned(),
                            reached: whole.reached.clone(),
                        })
                    }
                    Some(body) => self.evaluated_once(body, || evaluate(extra, Some(path))),
                };
                Ok(Some((read, path.clone())))
            }
            Place::Values(widget, path) => {
                let root = self
                    .root()
                    .ok_or_else(|| Unread::Refused("a widget with no root".to_owned()))?;
                let evaluate = |extra: Option<Extra<'_>>, at: Option<&[String]>| -> Read {
                    self.with_element(root.part, &root.scope, |element| {
                        let Ok(matched) = Matched::new(theme, element) else {
                            return Read {
                                tree: None,
                                reached: Vec::new(),
                            };
                        };
                        let applies = |selector: &Selector| matched.matches(selector);
                        theme.evaluate_in(
                            element.scope,
                            Some(element),
                            &applies,
                            Target::Values(*widget),
                            extra,
                            |cx| {
                                let folds = cx.roots(Target::Values(*widget));
                                let tree = match widget {
                                    WidgetKind::Button => {
                                        let resolved = ButtonValues::resolve(&folds, cx);
                                        match at {
                                            None => Tree::of(&resolved),
                                            Some(path) => Tree::of_at(&resolved, path),
                                        }
                                    }
                                };
                                Read {
                                    tree,
                                    reached: cx.reached().to_vec(),
                                }
                            },
                        )
                    })
                };
                let read = match body {
                    None => {
                        let whole = self.values.get_or_init(|| Rc::new(evaluate(None, None)));
                        Rc::new(Read {
                            tree: whole.tree.as_ref().and_then(|tree| tree.at(path)).cloned(),
                            reached: whole.reached.clone(),
                        })
                    }
                    Some(body) => self.evaluated_once(body, || evaluate(extra, Some(path))),
                };
                Ok(Some((read, path.clone())))
            }
            Place::WholePart(_)
            | Place::Token(_)
            | Place::Role(..)
            | Place::Input(_)
            | Place::KdeList(_) => Err(Unread::Refused(
                "not a part's field or a widget value".to_owned(),
            )),
        }
    }
}

/// A part field's or a widget value's tree at the key's place, where it
/// resolved, with the places its evaluation read a synthetic layer's step
/// at (S17, ADD-A128), and the key's path.
type Field = (Rc<Read>, Vec<String>);

/// A part's or a widget value's fact as a rule's body.
fn rule_body(value: Option<&FactValue>) -> Result<&RuleBody, Unread> {
    match value.map(|value| &value.body) {
        Some(Body::Rule(body)) => Ok(body),
        _ => Err(Unread::Refused("a field's fact states no value".to_owned())),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    /// A pending default's ids are a set (S17): spelled sorted, each once,
    /// whatever order the preset or the state model writes them in.
    #[test]
    fn default_spelled_takes_ids_as_a_set() {
        let ids = |list: &[&str]| list.iter().map(|id| (*id).to_owned()).collect::<Vec<_>>();
        assert_eq!(
            default_spelled(None, &ids(&["U2", "U1", "U2"])).as_deref(),
            Some("pending U1, U2")
        );
        assert_eq!(
            default_spelled(None, &ids(&["U2", "U1"])),
            default_spelled(None, &ids(&["U1", "U2"]))
        );
        assert_eq!(default_spelled(None, &[]), None);
    }

    /// Every path a fact's key can name below `fields`, the listed paths
    /// the generator can write (S17, ADD-A128): each field, a struct
    /// block's fields inside it but a set-whole block's, a map's entry,
    /// each map key one its key type reads.
    fn listable(prefix: &str, fields: &'static [registry::FieldInfo], out: &mut Vec<String>) {
        for field in fields {
            let path = format!("{prefix}.{}", field.path);
            out.push(path.clone());
            let mut entry = path;
            let mut inner = written(field);
            let levels = registry::key_levels(inner);
            for level in 1..=levels {
                let key = match inner.split_once(',').map(|(head, _)| head) {
                    Some("Vec<(String") => "Synthetic",
                    Some("Vec<(LanguageTag") => "x-synth",
                    Some("Vec<(SizeClass") => "compact",
                    Some("Vec<(IconRole") => "action_save",
                    Some("Vec<(AxisTag") => "GRAD",
                    Some("Vec<(FeatureTag") => "liga",
                    _ if inner.starts_with("SurfaceMap<") => "window",
                    _ => "synthetic",
                };
                entry = format!("{entry}.{key}");
                inner = map_value(inner).unwrap();
                if level == levels {
                    out.push(entry.clone());
                }
            }
            let list = ["Option<", "Value<"]
                .iter()
                .fold(inner, |ty, wrap| {
                    ty.strip_prefix(wrap)
                        .and_then(|rest| rest.strip_suffix('>'))
                        .unwrap_or(ty)
                })
                .starts_with("Vec<");
            // A set-whole block's leaves are listed as the block (S6.1, S17).
            let set_whole = inner.starts_with("Option<");
            if let registry::FieldType::Block {
                form: registry::BlockForm::Struct,
                fields: below,
                ..
            } = field.ty
                && !list
                && !set_whole
            {
                listable(&entry, below, out);
            }
        }
    }

    /// The fixture preset `base` with the parts the conditional marks read
    /// drawn (S12): the button body's stroke drawn on every edge, and the
    /// icon part drawn with an icon whose fill is absent beside an
    /// `original` rendering mode and a glyph from a named image whose fill
    /// is absent beside a stated stroke, or, `sizeless`, a glyph from a
    /// system control whose size is absent; every other value synthetic,
    /// the synthetic values' own.
    fn drawn_variant(base: &str, sizeless: bool) -> String {
        let block = |path: &str| {
            let (info, _) = super::super::toml::read::listed_field(path).unwrap();
            let Some((_, Item::Table(block))) = synthetic_of(written(info), info) else {
                panic!("{path}: a block's synthetic value is a table");
            };
            block
        };
        let absent = Item::String("absent".to_owned());
        let mut icon = block("widgets.button.icon.icon");
        icon.insert("fill".to_owned(), absent.clone());
        let original = Item::String("original".to_owned());
        icon.insert("rendering_mode".to_owned(), original);
        let mut glyph = block("widgets.button.icon.glyph");
        let mut source = Table::new();
        if sizeless {
            glyph.insert("size_px".to_owned(), absent.clone());
            let mut control = Table::new();
            control.insert(
                "control".to_owned(),
                Item::String("app_kit_radio".to_owned()),
            );
            control.insert("replaces".to_owned(), Item::Array(Vec::new()));
            source.insert("system_control".to_owned(), Item::Table(control));
        } else {
            let mut name = Table::new();
            name.insert("name".to_owned(), Item::String("synthetic".to_owned()));
            source.insert("named_system_image".to_owned(), Item::Table(name));
        }
        glyph.insert("source".to_owned(), Item::Table(source));
        glyph.insert("fill".to_owned(), absent);
        let glyph_stroke = Item::Table(block("widgets.button.icon.glyph.stroke"));
        glyph.insert("stroke".to_owned(), glyph_stroke);
        let mut part = Table::new();
        part.insert("icon".to_owned(), Item::Table(icon));
        part.insert("glyph".to_owned(), Item::Table(glyph));
        part.insert("transitions".to_owned(), Item::Array(Vec::new()));
        let mut stroke = block("widgets.button.body.stroke");
        for edge in ["top", "right", "bottom", "left"] {
            if let Some(Item::Table(edge)) = stroke.get_mut(edge) {
                edge.insert("width_px".to_owned(), Item::Integer(1));
                edge.insert("paint".to_owned(), Item::String("#010203".to_owned()));
                edge.insert("dash".to_owned(), Item::String("solid".to_owned()));
            }
        }
        let text = |keys: &[&str], item: Item| {
            let keys: Vec<String> = keys.iter().map(|key| (*key).to_owned()).collect();
            ::toml::to_string(&nested(&keys, item)).unwrap()
        };
        let edited = base
            .replacen(
                "[widgets.button.icon]\nabsent = true\n",
                &text(&["widgets", "button", "icon"], Item::Table(part)),
                1,
            )
            .replacen("stroke = \"absent\"\ncorners_px", "corners_px", 1);
        let stroke = text(
            &["widgets", "button", "body", "stroke"],
            Item::Table(stroke),
        );
        let drawn = format!("{edited}\n{stroke}");
        assert_eq!(drawn.matches("absent = true").count(), 1, "{drawn}");
        assert!(!drawn.contains("stroke = \"absent\"\ncorners_px"));
        drawn
    }

    /// The drawn variant loads as a preset, so the build test's refusals
    /// over it are the synthetic values'.
    #[test]
    fn drawn_variant_loads() {
        let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
        for sizeless in [false, true] {
            let file = read_preset(&drawn_variant(base, sizeless)).unwrap();
            crate::resolve::Stack::new(
                None,
                file.declarations,
                file.layer,
                crate::selector::Scheme::Light,
                file.pending,
            )
            .unwrap();
        }
    }

    /// Whether a problem is the fixture preset's surroundings of a listed
    /// path, not the synthetic value at its slot: a map key the fixture
    /// does not declare or hold, so the layer creates the entry with its
    /// other fields unset.
    fn surroundings(kind: &ProblemKind) -> bool {
        matches!(
            kind,
            ProblemKind::Undeclared { .. } | ProblemKind::EntryCreatedUnset { .. }
        )
    }

    /// A synthetic layer reads for every path the generator can write
    /// into `[pending]`, of every type the model holds there, each spelled
    /// as its type takes it, and loads through the synthetic route over the
    /// facts gate's fixture preset with the path in its `[pending]` table,
    /// so validation takes its value at its slot (S11, S17, ADD-A128) as
    /// stated (ADD-A131): a type with no synthetic value, or one validation
    /// refuses at any combination it reaches, fails here until it has one;
    /// a refusal of the fixture's surroundings ([`surroundings`]) is not the
    /// value's. A map's key is one validation takes (an axis no field
    /// routes). A key the field cannot leave `absent` has no synthetic value
    /// ([`synthetic_of`]) and is refused by name, the one such path today
    /// `MaterialScheme.spec_version` (S4.8). A path the model holds no field
    /// at is refused, naming it.
    #[test]
    fn synthetic_layers_state_every_listable_type() {
        let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
        let drawn = drawn_variant(base, false);
        let sizeless = drawn_variant(base, true);
        let mut paths = Vec::new();
        listable("tokens", registry::tokens(), &mut paths);
        listable("colors.light", registry::colors(), &mut paths);
        for widget in registry::widgets() {
            listable(
                &format!("widgets.{}.values", widget.name),
                widget.values,
                &mut paths,
            );
            for part in widget.parts {
                listable(
                    &format!("widgets.{}.{}", widget.name, part.name),
                    part.fields,
                    &mut paths,
                );
            }
        }
        assert!(paths.len() > 500, "{}", paths.len());
        let mut failures = Vec::new();
        let mut keys = Vec::new();
        for path in &paths {
            let spelled = ["", "_ms", "_px", "_deg", "_pt"]
                .iter()
                .map(|suffix| format!("{path}{suffix}"))
                .find(|spelled| super::super::toml::read::listed_field(spelled).is_some());
            let Some(spelled) = spelled else {
                failures.push(format!("{path}: no spelling names its field"));
                continue;
            };
            let pending = [(spelled.clone(), vec!["U1".to_owned()])];
            let layers = match synthetic_layers(&pending, &Layer::default()) {
                Ok(layers) => layers,
                Err(why) if why.ends_with("the gate has no synthetic value of `Key`") => {
                    keys.push(spelled);
                    continue;
                }
                Err(why) => {
                    failures.push(why);
                    continue;
                }
            };
            for (fixture, text) in [
                ("base", base),
                ("drawn", drawn.as_str()),
                ("sizeless", sizeless.as_str()),
            ] {
                let file =
                    read_preset(&format!("[pending]\n\"{spelled}\" = \"U1\"\n{text}")).unwrap();
                let Err(LayerError::Invalid(problems)) = crate::resolve::Stack::synthetic(
                    None,
                    file.declarations,
                    file.layer,
                    crate::selector::Scheme::Light,
                    file.pending,
                    layers.clone(),
                ) else {
                    continue;
                };
                failures.extend(
                    problems
                        .iter()
                        .filter(|problem| !surroundings(&problem.kind))
                        .map(|problem| format!("{spelled} over {fixture}: {problem}")),
                );
            }
        }
        assert_eq!(failures, Vec::<String>::new());
        assert_eq!(keys, ["tokens.material.spec_version"]);
        let refused = synthetic_layers(
            &[("tokens.behavior.bogus_ms".to_owned(), vec!["U1".to_owned()])],
            &Layer::default(),
        )
        .unwrap_err();
        assert!(refused.contains("holds no field"), "{refused}");
    }

    /// A key has no synthetic value (S10: it names what the preset
    /// declares or its slot lists), so a listed key the field cannot leave
    /// `absent`, or a block holding one, is refused by name, never loaded
    /// with a name validation refuses; one that may be absent is `absent`.
    #[test]
    fn a_listed_key_is_refused_by_name() {
        let key = "widgets.button.icon.icon.scale";
        let (info, _) = super::super::toml::read::listed_field(key).unwrap();
        assert!(synthetic_of(written(info), info).is_none());
        let (_, absent) = synthetic_value(key).unwrap();
        assert_eq!(absent, Item::String("absent".to_owned()));
    }

    /// A field S12 refuses `Absent` at under a condition of its place takes
    /// a stated synthetic value, whole or in a block (S17, ADD-A131): a
    /// glyph's size, fill and stroke, an edge's dash, an icon style's fill,
    /// and a slot's or keyed style's drawn-text leaves, which their places
    /// alone mark; an icon's rendering mode is `original`, accepted beside
    /// any fill, and a glyph's source a system control, accepted beside an
    /// absent size; a field marked wherever it is stays absent.
    #[test]
    fn a_conditionally_marked_field_is_stated() {
        let value = |path: &str| synthetic_value(path).unwrap().1;
        assert_eq!(value("widgets.button.icon.glyph.size_px"), Item::Integer(1));
        assert_eq!(
            value("widgets.button.icon.glyph.fill"),
            Item::String("#010203".to_owned())
        );
        assert_eq!(
            value("widgets.button.icon.icon.fill"),
            Item::String("#010203".to_owned())
        );
        assert_eq!(
            value("tokens.text.roles.body.elision"),
            Item::String("none".to_owned())
        );
        assert_eq!(
            value("tokens.text.keyed.title.fill"),
            Item::String("#010203".to_owned())
        );
        assert_eq!(
            value("widgets.button.icon.glyph.draw_start"),
            Item::String("absent".to_owned())
        );
        assert!(matches!(
            value("widgets.button.icon.glyph.stroke"),
            Item::Table(_)
        ));
        assert_eq!(
            value("widgets.button.body.stroke.top.dash"),
            Item::String("solid".to_owned())
        );
        assert_eq!(
            value("widgets.button.icon.icon.rendering_mode"),
            Item::String("original".to_owned())
        );
        assert!(matches!(
            value("widgets.button.icon.glyph.source"),
            Item::Table(source) if source.contains_key("system_control")
        ));
        for whole in ["widgets.button.icon.icon", "widgets.button.icon.glyph"] {
            let (info, _) = super::super::toml::read::listed_field(whole).unwrap();
            let Some((_, Item::Table(block))) = synthetic_of(written(info), info) else {
                panic!("{whole}: a block's synthetic value is a table");
            };
            assert_eq!(
                block.get("fill"),
                Some(&Item::String("#010203".to_owned())),
                "{whole}"
            );
        }
        let Item::Table(body) = value("tokens.text.roles.body") else {
            panic!("a slot's synthetic value is a table");
        };
        assert_eq!(body.get("max_lines"), Some(&Item::Integer(1)));
        assert_eq!(
            body.get("decoration"),
            Some(&Item::String("absent".to_owned()))
        );
    }

    /// The synthetic layers of a set-whole block (`Option<KeyRepeat>`), a
    /// keyed icon size's leaf and a colour role state the listed path, its
    /// key spelled with the unit its value takes (S11, S17, ADD-A128).
    #[test]
    fn synthetic_layers_spell_their_values() {
        let layer = |path: &str| {
            let mut layers = synthetic_layers(
                &[(path.to_owned(), vec!["U1".to_owned()])],
                &Layer::default(),
            )
            .unwrap();
            layers.pop().unwrap()
        };
        let repeat = layer("tokens.behavior.key_repeat");
        assert_eq!(
            repeat.bases.tokens,
            Layer::from_toml("[tokens.behavior.key_repeat]\ndelay_ms = 1\ninterval_ms = 1\n")
                .unwrap()
                .bases
                .tokens
        );
        let status = layer("colors.light.global.status");
        assert!(status.bases.colors.light.is_some());
        assert!(status.bases.colors.dark.is_none());
        let font = layer("widgets.button.label.text.font.size");
        assert_eq!(
            font.bases.widgets,
            Layer::from_toml("[widgets.button.label.text.font]\nsize_pt = 1\n")
                .unwrap()
                .bases
                .widgets
        );
    }

    /// A value no spelling reads is reported by the spelling whose key the
    /// reader knows: a one-unit field's suffixed key for an expression
    /// (S11), not the bare key's "unknown key".
    #[test]
    fn a_refused_leaf_reports_its_known_spelling() {
        let key = parse_key("token.behavior.double_click_time").unwrap();
        let Err(Unread::Refused(why)) = parse_value(&key, &Item::String("= div(1ms,".to_owned()))
        else {
            panic!("an expression that does not parse is refused");
        };
        assert!(why.contains("double_click_time_ms"), "{why}");
        assert!(!why.contains("unknown key"), "{why}");
    }

    /// A side the gate cannot read is refused, so two such sides never
    /// agree (S17).
    #[test]
    fn unresolved_sides_never_agree() {
        let path = vec!["width".to_owned()];
        let tree = Tree::of(&vec![("width".to_owned(), 4.0_f32)]).unwrap();
        let quote = Tolerance::quote();
        assert!(matches!(
            differs(None, None, &path, &quote),
            Err(Unread::Refused(_))
        ));
        assert!(matches!(
            differs(Some(&tree), None, &path, &quote),
            Err(Unread::Refused(_))
        ));
        let missing = vec!["height".to_owned()];
        assert!(matches!(
            differs(Some(&tree), Some(&tree), &missing, &quote),
            Err(Unread::Refused(_))
        ));
        assert_eq!(
            differs(Some(&tree), Some(&tree), &path, &quote),
            Ok(Outcome::Agrees)
        );
    }

    /// A leaf of `Option<KeyRepeat>`, which S6.1 sets whole, names its
    /// block and every leaf with its unit suffix (S17).
    #[test]
    fn a_set_whole_leaf_names_its_block() {
        let block = set_whole("token.behavior.key_repeat.delay").unwrap();
        assert_eq!(block.block, "token.behavior.key_repeat");
        assert_eq!(
            block.leaves,
            [
                (
                    "token.behavior.key_repeat.delay".to_owned(),
                    "delay".to_owned(),
                    "_ms"
                ),
                (
                    "token.behavior.key_repeat.interval".to_owned(),
                    "interval".to_owned(),
                    "_ms"
                ),
            ]
        );
        assert!(set_whole("token.behavior.double_click_time").is_none());
    }
}
