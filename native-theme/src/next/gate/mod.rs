//! The facts gate's way into the model (S17). The gate
//! (`native-theme/tests/facts_gate.rs`) reads the facts and the state
//! models, enumerates each widget's combinations and reports; this module
//! answers what only the model can, as the query does: a preset resolved
//! with a stand-in under a combination's conditions, whether a fact's
//! `when` holds for an element, and a key's value beside a fact's value
//! evaluated in the key's place, over the same operands in the same scope.

mod tree;

use std::cell::OnceCell;

use ::toml::{Table, Value as Item};

pub use tree::{Tolerance, Tree};

use super::declare::WidgetDecl;
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
use super::resolve::evaluate::{Extra, Resolve};
use super::resolve::{
    Pos, ReaderState, ResolvedTheme, Stack, WindowConditions, WindowState, input_value,
};
use super::selector::{
    Condition, ControlSize, Density, Direction, InputKind, PartRef, RuleBody, Scheme, Selector,
    StateFlag, StateFlags, ValuesPatch, VariantRef, WidgetOrPart, WidgetRef,
};
use super::tokens::Tokens;
use super::toml::layer::{read_inputs_table, read_preset, read_rule_body};
use super::toml::read::{Named, Reader};
use super::toml::when;
use super::widgets::{ButtonValues, PartPatch, TypedPart, WidgetKind, WidgetValuesPatch};

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

/// A preset the gate checks: its stack, validated as `Theme` validates a
/// bundled preset (S12); where its `[pending]` table lists paths, through
/// the synthetic route, once with each type's first synthetic value and
/// once with its second, so a value reaching one is told from the preset's
/// own (S17, ADD-A128).
#[derive(Clone, Debug)]
pub struct Preset {
    stack: Stack,
    id: String,
    pending: Vec<(String, String)>,
    /// The synthetic route's stack with the second synthetic values.
    alternate: Option<Stack>,
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
        if file.pending.is_empty() {
            let stack = Stack::new(id, file.declarations, file.layer, Scheme::Light)
                .map_err(|error| refused(&error))?;
            return Ok(Preset {
                stack,
                id: file.id,
                pending: Vec::new(),
                alternate: None,
            });
        }
        let synthetic = |second: bool| {
            let layers = synthetic_layers(&file.pending, second)?;
            Stack::synthetic(
                id,
                file.declarations.clone(),
                file.layer.clone(),
                Scheme::Light,
                &file.pending,
                layers,
            )
            .map_err(|error| refused(&error))
        };
        Ok(Preset {
            stack: synthetic(false)?,
            alternate: Some(synthetic(true)?),
            id: file.id,
            pending: file.pending,
        })
    }

    /// Its `[pending]` table: each listed path as S11 spells it, with its
    /// register id (S17, ADD-A128).
    #[must_use]
    pub fn pending(&self) -> &[(String, String)] {
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
    /// stand-in.
    pub fn resolve(&self, conditions: &Conditions, stand_in: &Layer) -> Result<Resolved, String> {
        let alternate = match &self.alternate {
            Some(stack) => Some(Box::new(resolve_stack(stack, conditions, stand_in)?)),
            None => None,
        };
        let mut resolved = resolve_stack(&self.stack, conditions, stand_in)?;
        resolved.alternate = alternate;
        Ok(resolved)
    }
}

/// `stack` resolved with a stand-in stating `stand_in` in `conditions`.
fn resolve_stack(
    stack: &Stack,
    conditions: &Conditions,
    stand_in: &Layer,
) -> Result<Resolved, String> {
    let mut stack = stack.clone();
    if let Some(flavor) = &conditions.flavor {
        stack
            .set_flavor(flavor.clone())
            .map_err(|error| refused(&error))?;
    }
    stack
        .set_reader_stand_in(conditions.state.clone(), stand_in.clone())
        .map_err(|error| refused(&error))?;
    let theme = stack
        .resolve(&mut WindowState::default(), &conditions.window)
        .theme;
    Ok(Resolved {
        theme,
        stack,
        window: conditions.window.clone(),
        alternate: None,
    })
}

/// The field a listed path names (S11's `[pending]` key: `tokens.…` or
/// `widgets.<widget>.<part>.…`, `widgets.<widget>.values.…`), its last
/// key's unit suffix left out.
fn listed_field(path: &str) -> Option<&'static registry::FieldInfo> {
    let path = super::toml::read::listed_path(path);
    let parts = segments(&path).ok()?;
    let (head, rest) = parts.split_first()?;
    let (fields, below): (&'static [registry::FieldInfo], &[String]) = match head.as_str() {
        "tokens" => (registry::tokens(), rest),
        "widgets" => {
            let (widget, rest) = rest.split_first()?;
            let info = registry::widget(widget)?;
            let (part, rest) = rest.split_first()?;
            if part == "values" {
                (info.values, rest)
            } else {
                let part = info.parts.iter().find(|held| held.name == part)?;
                (part.fields, rest)
            }
        }
        _ => return None,
    };
    registry::find(fields, &below.join("."))
}

/// The fact key a listed path names (S11's `[pending]` key as S17's key
/// spells it: `tokens.behavior.touch.delay_ms` is
/// `token.behavior.touch.delay`, `widgets.button.icon.icon.size_px` is
/// `button.icon.icon.size`); `None` for a path of no field the gate reads.
#[must_use]
pub fn listed_key(path: &str) -> Option<String> {
    let path = super::toml::read::listed_path(path);
    if let Some(rest) = path.strip_prefix("tokens.") {
        Some(format!("token.{rest}"))
    } else {
        path.strip_prefix("widgets.").map(str::to_owned)
    }
}

/// A synthetic value of the field `info` (S17, ADD-A128): one of two per
/// type, `second` picking the other, so a value reaching it tells the two
/// apart. Synthetic, never bundled, no platform's; `None` for a type the
/// gate has none for yet.
fn synthetic_value(info: &registry::FieldInfo, second: bool) -> Option<Item> {
    let pick = |first: &str, other: &str| (if second { other } else { first }).to_owned();
    let names = |first: &str, other: &str| Item::Array(vec![Item::String(pick(first, other))]);
    if let Some(ty) = (info.value)() {
        return match ty {
            Ty::Duration | Ty::Length | Ty::Angle | Ty::FontSize => {
                Some(Item::Integer(if second { 2 } else { 1 }))
            }
            Ty::Number => Some(Item::Float(if second { 0.75 } else { 0.25 })),
            Ty::Bool => Some(Item::Boolean(!second)),
            Ty::Text => Some(Item::String(pick("Synthetic A", "Synthetic B"))),
            Ty::Color | Ty::Paint => Some(Item::String(pick("#010203", "#030201"))),
            Ty::Enum(names) => {
                let (first, other) = (names.variants.first()?, names.variants.get(1)?);
                Some(Item::String(pick(first, other)))
            }
            Ty::Key(_) | Ty::Shadow | Ty::Block(_) => None,
        };
    }
    let registry::FieldType::Leaf(ty) = info.ty else {
        return None;
    };
    let ty: String = ty.chars().filter(|c| !c.is_whitespace()).collect();
    let map = |key: &str| {
        let mut table = Table::new();
        table.insert(key.to_owned(), names("Synthetic A", "Synthetic B"));
        Item::Table(table)
    };
    match ty.as_str() {
        "Vec<String>" => Some(names("Synthetic A", "Synthetic B")),
        "Vec<(String,Vec<String>)>" => Some(map("Synthetic")),
        "Vec<(LanguageTag,Vec<String>)>" => Some(map("x-synth")),
        _ => None,
    }
}

/// One layer per listed path of a preset's `[pending]` table, stating it
/// in its bases with a synthetic value (S17's synthetic route, ADD-A128):
/// the layers [`Preset`] and S19.1's build test load such a preset with.
/// `second` picks each type's other synthetic value. A listed path the
/// model holds no field at, or of a type with no synthetic value yet, is
/// refused, naming it.
pub fn synthetic_layers(pending: &[(String, String)], second: bool) -> Result<Vec<Layer>, String> {
    pending
        .iter()
        .map(|(path, id)| {
            let info = listed_field(path)
                .ok_or_else(|| format!("`{path}` ({id}): the model holds no field there"))?;
            let value = synthetic_value(info, second).ok_or_else(|| {
                format!("`{path}` ({id}): the gate has no synthetic value of its type")
            })?;
            let mut item = value;
            for key in segments(path)
                .map_err(|unread| format!("`{path}` ({id}): {unread:?}"))?
                .iter()
                .rev()
            {
                let mut table = Table::new();
                table.insert(key.clone(), item);
                item = Item::Table(table);
            }
            let text = ::toml::to_string(&item).map_err(|error| error.to_string())?;
            Layer::from_toml(&text).map_err(|error| format!("`{path}` ({id}): {error}"))
        })
        .collect()
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
    /// `default_size`.
    pub default_size: Option<String>,
    /// `default_size_by_variant`: each variant and its size.
    pub default_size_by_variant: Vec<(String, String)>,
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
    /// `sizes_by_variant`: each variant and its sizes.
    pub sizes_by_variant: Vec<(String, Vec<String>)>,
    /// `sizes_by_context`: each context widget and its sizes.
    pub sizes_by_context: Vec<(String, Vec<String>)>,
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
            default_size: decl.default_size.map(|size| size.name().to_owned()),
            default_size_by_variant: decl
                .default_size_by_variant
                .iter()
                .flatten()
                .map(|by| (variant_name(&by.variant), by.size.name().to_owned()))
                .collect(),
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
                .map(|by| (variant_name(&by.variant), sizes(&by.sizes)))
                .collect(),
            sizes_by_context: decl
                .sizes_by_context
                .iter()
                .flatten()
                .map(|by| (widget_name(&by.context), sizes(&by.sizes)))
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
    /// A whole part's absence or substitution.
    Part(Box<PartPatch>),
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
                Item::String(text) if text == "absent" => {
                    let mut table = Table::new();
                    table.insert("absent".to_owned(), Item::Boolean(true));
                    Item::Table(table)
                }
                other => other.clone(),
            };
            match read_rule_body(&part_target(*part), &set)
                .map_err(|problems| Unread::of(&problems))?
            {
                RuleBody::Part(_, patch @ (PartPatch::Absent | PartPatch::DrawnAs { .. })) => {
                    Body::Part(Box::new(patch))
                }
                _ => {
                    return Err(Unread::Refused(
                        "a whole part's value is `absent` or its `drawn_as`".to_owned(),
                    ));
                }
            }
        }
        Place::Part(part, path) => Body::Rule(Box::new(rule_at(&part_target(*part), path, value)?)),
        Place::Values(widget, path) => {
            let target = format!("{}.values", widget_name(&WidgetRef::Typed(*widget)));
            Body::Rule(Box::new(rule_at(&target, path, value)?))
        }
        Place::Token(path) => Body::Rule(Box::new(rule_at("tokens", path, value)?)),
        Place::Role(_, path) => Body::Rule(Box::new(rule_at("colors", path, value)?)),
        Place::Input(name) => Body::Inputs(inputs_at(&segments_of(name), value)?),
        Place::KdeList(list) => {
            let path = ["kde_scheme".to_owned(), list.name().to_owned()];
            Body::Inputs(inputs_at(&path, value)?)
        }
    };
    Ok(FactValue { body })
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
    /// The synthetic route's second resolution, with each type's other
    /// synthetic value (S17, ADD-A128).
    alternate: Option<Box<Resolved>>,
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
        RuleBody::Colors(roles) => (Target::Roles, Found::Node(roles)),
        RuleBody::Tokens(tokens) => (Target::Tokens, Found::Node(tokens)),
        RuleBody::Part(PartRef::Typed(part), PartPatch::Style(style)) => {
            (Target::Part(*part), Found::Node(style))
        }
        RuleBody::Values(ValuesPatch::Typed(WidgetValuesPatch::Button(values))) => {
            (Target::Values(WidgetKind::Button), Found::Node(values))
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
    /// of the key's place in it.
    fn global_tree(
        &self,
        key: &FactKey,
        value: Option<&FactValue>,
        root: &RootInputs,
    ) -> Result<(Option<Tree>, Vec<String>), Unread> {
        let scope = self.root(root);
        let applies: &dyn Fn(&Selector) -> bool = &|_| true;
        let theme = &self.theme;
        let read = |extra: Option<Extra<'_>>| -> Option<Tree> {
            match &key.place {
                Place::Token(_) => {
                    theme.evaluate_in(&scope, None, applies, Target::Tokens, extra, |cx| {
                        let folds = cx.roots(Target::Tokens);
                        Tree::of(&Tokens::resolve(&folds, cx))
                    })
                }
                Place::Role(reference, _) => {
                    theme.evaluate_in(&scope, None, applies, Target::Roles, extra, |cx| {
                        Tree::of(&cx.color_of(reference))
                    })
                }
                Place::Input(name) => {
                    let reference = Ref::Input(name.segments().to_vec());
                    theme.evaluate_in(&scope, None, applies, Target::Roles, extra, |cx| {
                        Tree::of(&cx.color_of(&reference))
                    })
                }
                Place::KdeList(list) => {
                    let lists = &theme.kde_lists;
                    match list {
                        KdeList::Groups => Tree::of(&lists.groups),
                        KdeList::InactiveGroups => Tree::of(&lists.inactive_groups),
                        KdeList::UnsetKeys => Tree::of(&lists.unset_keys),
                    }
                }
                Place::WholePart(_) | Place::Part(..) | Place::Values(..) => None,
            }
        };
        let path = match &key.place {
            Place::Token(path) => path.clone(),
            Place::Role(..) | Place::Input(_) | Place::KdeList(_) => Vec::new(),
            Place::WholePart(_) | Place::Part(..) | Place::Values(..) => {
                return Err(Unread::Refused(
                    "a widget's key is read with its element".to_owned(),
                ));
            }
        };
        let Some(value) = value else {
            return Ok((read(None), path));
        };
        let scheme = theme.scheme;
        let contrast = self.stack.contrast();
        let tree = match (&value.body, &key.place) {
            (Body::Rule(body), _) => read(extra_step(body).map(Extra::Step)),
            (Body::Inputs(bases), Place::Input(name)) => bases
                .entries
                .iter()
                .find(|(held, _)| held == name)
                .and_then(|(_, stated)| input_value(stated, scheme, contrast))
                .and_then(|color| read(Some(Extra::Input(name, &color)))),
            (Body::Inputs(bases), Place::KdeList(list)) => list_tree(bases, *list, scheme),
            (Body::Inputs(_) | Body::Part(_), _) => None,
        };
        Ok((tree, path))
    }

    /// Whether a token's, a role's or an input's value, or `value`
    /// evaluated in its place, reaches a synthetic value (S17, ADD-A128):
    /// it differs between the synthetic route's two resolutions. Never
    /// without a `[pending]` table.
    pub fn global_synthetic(
        &self,
        key: &FactKey,
        value: Option<&FactValue>,
        root: &RootInputs,
    ) -> Result<bool, Unread> {
        let Some(alternate) = &self.alternate else {
            return Ok(false);
        };
        for value in [None, value] {
            let (first, path) = self.global_tree(key, value, root)?;
            let (second, _) = alternate.global_tree(key, value, root)?;
            if first.as_ref().and_then(|tree| tree.at(&path))
                != second.as_ref().and_then(|tree| tree.at(&path))
            {
                return Ok(true);
            }
        }
        Ok(false)
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
        let (actual, path) = self.global_tree(key, None, root)?;
        let (expected, _) = self.global_tree(key, Some(value), root)?;
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
        let (measured, path) = self.global_tree(key, Some(result), root)?;
        let (derived, _) = self.global_tree(key, Some(value), root)?;
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
        let mut scope = self.root(root);
        if let Some(context) = &combination.context {
            scope = self.context_scope(context, scope, metrics)?;
        }
        let alternate = match &self.alternate {
            Some(alternate) => Some(Box::new(alternate.session(combination, root, metrics)?)),
            None => None,
        };
        let mut session = Session {
            resolved: self,
            metrics,
            parsed,
            widget,
            parts: Vec::new(),
            values: OnceCell::new(),
            alternate,
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

    /// The scope inside a context's ancestor (S11's context selector, as a
    /// state model's `axes.contexts` writes it): its widget's or part's
    /// element, holding the context's variants and states, queried in
    /// `scope`.
    fn context_scope(
        &self,
        context: &Item,
        scope: Scope,
        metrics: &dyn FontMetrics,
    ) -> Result<Scope, Unread> {
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
        let mut element = Element::new(PartRef::Typed(part), &scope, metrics)
            .with_variants(&variants)
            .with_states(states, &[]);
        if let Some(size) = context.sizes.all_of.first() {
            element = element.with_size(*size);
        }
        match self.theme.style(&element) {
            Styled::Style((_, inside)) => Ok(inside),
            other => Err(Unread::Refused(format!(
                "the context's ancestor is {other:?}"
            ))),
        }
    }
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
    actual: OnceCell<Option<Tree>>,
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
    /// The ancestor context, as S11's context selector writes it.
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
    let unread = |side: &str| {
        Unread::Refused(format!(
            "the gate cannot read {side} value at `{}`",
            path.join(".")
        ))
    };
    let actual = actual
        .and_then(|tree| tree.at(path))
        .ok_or_else(|| unread("the theme's"))?;
    let expected = expected
        .and_then(|tree| tree.at(path))
        .ok_or_else(|| unread("the fact's"))?;
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
    values: OnceCell<Option<Tree>>,
    /// The same element in the synthetic route's second resolution (S17,
    /// ADD-A128).
    alternate: Option<Box<Session<'r>>>,
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

    /// Whether `when` holds for the element of the key's part (its root's
    /// for a value): its conditions as resolution decided them, the rest
    /// matched against the element as the query matches a rule (S8).
    #[must_use]
    pub fn holds(&self, when: &When, key: &FactKey) -> bool {
        if !self.resolved.conditions_hold(when) {
            return false;
        }
        let part = match &key.place {
            Place::WholePart(part) | Place::Part(part, _) => self.queried(*part),
            Place::Values(..) => self.root(),
            Place::Token(_) | Place::Role(..) | Place::Input(_) | Place::KdeList(_) => return true,
        };
        let Some(queried) = part else {
            return false;
        };
        let theme = &self.resolved.theme;
        self.with_element(queried.part, &queried.scope, |element| {
            Matched::new(theme, element).is_ok_and(|matched| matched.matches(&when.0))
        })
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
                let patch = value.and_then(|value| match &value.body {
                    Body::Part(patch) => Some(patch.as_ref()),
                    Body::Rule(_) | Body::Inputs(_) => None,
                });
                let expected = match (value, patch) {
                    (None, _) => Answer::Drawn,
                    (Some(_), Some(PartPatch::Absent)) => Answer::NotDrawn,
                    (Some(_), Some(PartPatch::DrawnAs { target, variants })) => Answer::DrawnAs {
                        target: target.clone(),
                        variants: variants.clone(),
                    },
                    (Some(_), Some(PartPatch::Style(_)) | None) => {
                        return Err(Unread::Refused(
                            "a whole part's value is `absent` or its `drawn_as`".to_owned(),
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
                let (expected, _) = self
                    .field_tree(key, Some(rule_body(value)?))?
                    .unwrap_or_default();
                differs(
                    actual.as_ref(),
                    expected.as_ref(),
                    &path,
                    &self.in_scope(key, tolerance),
                )
            }
            Place::Token(_) | Place::Role(..) | Place::Input(_) | Place::KdeList(_) => Err(
                Unread::Refused("a token, a role or an input is read in the root scope".to_owned()),
            ),
        }
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
        let (derived, _) = self
            .field_tree(key, Some(rule_body(Some(value))?))?
            .unwrap_or_default();
        differs(
            measured.as_ref(),
            derived.as_ref(),
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
    /// its place, reaches a synthetic value (S17, ADD-A128): it differs
    /// between the synthetic route's two resolutions, a whole part by its
    /// answer. Never without a `[pending]` table.
    pub fn synthetic(&self, key: &FactKey, value: Option<&FactValue>) -> Result<bool, Unread> {
        let Some(alternate) = &self.alternate else {
            return Ok(false);
        };
        match &key.place {
            Place::WholePart(part) => Ok(self.queried(*part).map(|held| &held.answer)
                != alternate.queried(*part).map(|held| &held.answer)),
            Place::Part(..) | Place::Values(..) => {
                let body = value.and_then(|value| rule_body(Some(value)).ok());
                for body in [None, body] {
                    let read = |session: &Session<'_>| -> Result<Option<Tree>, Unread> {
                        Ok(session
                            .field_tree(key, body)?
                            .and_then(|(tree, path)| tree.and_then(|tree| tree.at(&path).cloned())))
                    };
                    if read(self)? != read(alternate)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            Place::Token(_) | Place::Role(..) | Place::Input(_) | Place::KdeList(_) => Ok(false),
        }
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

    /// A part field's or a widget value's tree for the element, with the
    /// key's path in it: the theme's own (read once), or with `body`'s step
    /// after every other; `None` where the key's part is not drawn.
    fn field_tree(&self, key: &FactKey, body: Option<&RuleBody>) -> Result<Option<Field>, Unread> {
        let theme = &self.resolved.theme;
        let extra = body.and_then(extra_step).map(Extra::Step);
        match &key.place {
            Place::Part(part, path) => {
                let queried = self
                    .queried(*part)
                    .ok_or_else(|| Unread::Refused("a part the widget does not have".to_owned()))?;
                if queried.answer != Answer::Drawn {
                    return Ok(None);
                }
                let evaluate = |extra: Option<Extra<'_>>| {
                    self.with_element(queried.part, &queried.scope, |element| {
                        let matched = Matched::new(theme, element).ok()?;
                        let applies = |selector: &Selector| matched.matches(selector);
                        theme.evaluate_in(
                            element.scope,
                            Some(element),
                            &applies,
                            Target::Part(queried.part),
                            extra,
                            |cx| Tree::of(&cx.part(queried.part)),
                        )
                    })
                };
                let tree = match body {
                    None => queried.actual.get_or_init(|| evaluate(None)).clone(),
                    Some(_) => evaluate(extra),
                };
                Ok(Some((tree, path.clone())))
            }
            Place::Values(widget, path) => {
                let root = self
                    .root()
                    .ok_or_else(|| Unread::Refused("a widget with no root".to_owned()))?;
                let evaluate = |extra: Option<Extra<'_>>| {
                    self.with_element(root.part, &root.scope, |element| {
                        let matched = Matched::new(theme, element).ok()?;
                        let applies = |selector: &Selector| matched.matches(selector);
                        theme.evaluate_in(
                            element.scope,
                            Some(element),
                            &applies,
                            Target::Values(*widget),
                            extra,
                            |cx| {
                                let folds = cx.roots(Target::Values(*widget));
                                match widget {
                                    WidgetKind::Button => {
                                        Tree::of(&ButtonValues::resolve(&folds, cx))
                                    }
                                }
                            },
                        )
                    })
                };
                let tree = match body {
                    None => self.values.get_or_init(|| evaluate(None)).clone(),
                    Some(_) => evaluate(extra),
                };
                Ok(Some((tree, path.clone())))
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

/// A part field's or a widget value's tree, where it resolved, and the
/// key's path in it.
type Field = (Option<Tree>, Vec<String>);

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

    /// A value no spelling reads is reported by the spelling whose key the
    /// reader knows: a one-unit field's suffixed key for an expression
    /// (S11), not the bare key's "unknown key".
    /// A synthetic layer per listed path, its value of the field's type,
    /// two distinct ones per type (S17, ADD-A128): a duration, a list of
    /// families, a map of them, an enum; a path the model holds no field
    /// at, or a type with no synthetic value, is refused naming it.
    #[test]
    fn synthetic_layers_state_each_listed_path() {
        let pending: Vec<(String, String)> = [
            "tokens.behavior.touch.delay_ms",
            "tokens.text.fallbacks.families",
            "tokens.text.fallbacks.per_family",
            "tokens.text.fallbacks.per_language",
            "tokens.icons.sizes.keyed.check.rendering_mode",
            "widgets.button.icon.icon.rendering_mode",
        ]
        .iter()
        .map(|path| ((*path).to_owned(), "U1".to_owned()))
        .collect();
        let first = synthetic_layers(&pending, false).unwrap();
        let second = synthetic_layers(&pending, true).unwrap();
        assert_eq!(first.len(), pending.len());
        for (a, b) in first.iter().zip(&second) {
            assert_ne!(a.bases, b.bases);
        }
        let refused = |path: &str| {
            synthetic_layers(&[(path.to_owned(), "U1".to_owned())], false).unwrap_err()
        };
        assert!(refused("tokens.behavior.bogus_ms").contains("holds no field"));
        assert!(refused("tokens.focus.shadow").contains("no synthetic value"));
    }

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
