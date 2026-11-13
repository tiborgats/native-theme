//! S12's structural checks: each value, patch, rule and declaration on its
//! own, every problem collected, one problem per fault.

use std::borrow::Cow;
use std::rc::Rc;

use super::walk::{ExprForm, Form, KEY_OF_PLACE, Node, Number, Path, Visit, Walk, Walker};
use super::{Context, LayerKind};
use crate::blocks::motion::{
    BreezeTest, Keyframe, PresentationKind, PropertyGroup, StepJump, Timing, Transition,
    TransitionEngine, TransitionKind,
};
use crate::blocks::outline::Radius;
use crate::blocks::outline::StrokePatch;
use crate::blocks::paint::{BlendMode, PaintKind, PaintPatch};
use crate::blocks::part_style::PartStylePatch;
use crate::blocks::shadow::{Shadow, ShadowFalloff};
use crate::blocks::text::{AxisTag, AxisValue, FeatureTag};
use crate::blocks::{BlockPatch, Table};
use crate::declare::{
    DataName, DataOwner, Declarations, DefaultSize, FontMetric, ScalarUnit, WidgetDecl,
};
use crate::error::{Problem, ProblemKind};
use crate::expr::{
    Bound, ElementDatum, Expr, FieldType, Func, Literal, MAX_DEPTH, Name, Ref, Ty, TypeEnv,
    is_kde_list,
};
use crate::inputs::{InputBases, InputName, InputValue};
use crate::key::Key;
use crate::layer::{Bases, Layer, Spelled};
use crate::presets::PresetId;
use crate::registry::{self, FieldInfo, PartInfo};
use crate::roles::{ColorRolesPatch, SurfaceRoles, SurfaceSet};
use crate::selector::{
    Condition, ContextMatch, ControlSize, Density, PartRef, PositionMatch, Rule, RuleBody,
    Selector, SetMatch, SiblingMatch, StateFlag, ValuesPatch, VariantRef, WidgetOrPart, WidgetRef,
};
use crate::tokens::ABSENT_PARAM_GROUPS;
use crate::toml::read::{Named, listed_path, toml_key};
use crate::units::{Ms, Px, Scalar};
use crate::value::{SchemeMap, SurfaceMap, Value};
use crate::widgets::{
    ButtonPatch, PartPatch, PartPresence, TypedPart, WidgetKind, WidgetValuesPatch, WidgetsPatch,
};

/// KDE's seven colour sets, `kde_accent`'s `set` (S2.2).
const KDE_SETS: [&str; 7] = [
    "window",
    "view",
    "button",
    "selection",
    "tooltip",
    "complementary",
    "header",
];

/// Every structural problem of `layer` against `cx`.
pub(super) fn layer(layer: &Layer, cx: &Context<'_>) -> Vec<Problem> {
    layer_exempt(layer, cx, &[])
}

/// Every structural problem of `layer` against `cx`, a preset's listed
/// paths `exempt` (each as `listed_path` gives it) from its completeness
/// check, which the layers above it state on every route (S17, ADD-A131).
pub(super) fn layer_exempt(layer: &Layer, cx: &Context<'_>, exempt: &[String]) -> Vec<Problem> {
    let mut problems = in_flavor(layer, cx, None, exempt);
    // Under each flavour, the declarations in force are the default
    // section's with that flavour's keys replacing it (S10): the bases are
    // checked against each, a rule against those of the flavours it can
    // apply under (S12's combinations under each `flavor:` condition).
    let flavors = &cx.declarations.flavors;
    if flavors.len() > 1 {
        let union = problems.len();
        for flavor in flavors {
            for problem in in_flavor(layer, cx, Some(flavor), exempt) {
                let flavored = matches!(
                    problem.kind,
                    ProblemKind::Undeclared { .. }
                        | ProblemKind::UndeclaredData { .. }
                        | ProblemKind::WidgetUnavailable { .. }
                        | ProblemKind::Missing
                );
                let known = problems
                    .iter()
                    .take(union)
                    .any(|held| held.path == problem.path);
                if flavored && !known && !problems.contains(&problem) {
                    problems.push(problem);
                }
            }
        }
    }
    problems
}

/// Every structural problem of `layer` against `cx`, a widget's names
/// against `flavor`'s declarations where one is given, its rules then only
/// those that can apply under it.
fn in_flavor(
    layer: &Layer,
    cx: &Context<'_>,
    flavor: Option<&Key>,
    exempt: &[String],
) -> Vec<Problem> {
    let mut checker = Checker::new(cx, matches!(layer.spelled, Spelled::Toml));
    checker.flavor = flavor;
    checker.exempt = exempt.to_vec();
    checker.bases(&layer.bases, "");
    for (preset, bases) in &layer.for_preset {
        if cx.preset == Some(*preset) {
            checker.bases(bases, &format!("for_preset.{}", preset.key()));
        }
    }
    for (index, rule) in layer.rules.iter().enumerate() {
        let under = flavor.is_none_or(|flavor| can_apply_under(rule, flavor));
        if !excluded(rule, cx.preset) && under {
            checker.rule(rule, &format!("rules[{index}]"));
        }
    }
    let mut problems: Vec<Problem> = Vec::new();
    for mut problem in checker.problems {
        problem.path = layer.shorthands.written(&problem.path, &layer.rules);
        if !problems.contains(&problem) {
            problems.push(problem);
        }
    }
    problems
}

/// Whether a rule can apply while `flavor` is in force: its conditions name
/// no other flavour as holding, and not this one as not holding (one
/// flavour holds at a time, S12).
fn can_apply_under(rule: &Rule, flavor: &Key) -> bool {
    let conditions = &rule.selector.conditions;
    let other = |c: &Condition| matches!(c, Condition::Flavor(held) if held != flavor);
    let this = |c: &Condition| matches!(c, Condition::Flavor(held) if held == flavor);
    !conditions.all_of.iter().any(other) && !conditions.none_of.iter().any(this)
}

/// Whether a rule's `Preset` condition excludes the current preset: such a
/// rule is neither validated nor applied, so one layer can carry rules for
/// several presets' keyed names (S12).
fn excluded(rule: &Rule, preset: Option<PresetId>) -> bool {
    let conditions = &rule.selector.conditions;
    let other =
        |condition: &Condition| matches!(condition, Condition::Preset(id) if Some(*id) != preset);
    let current =
        |condition: &Condition| matches!(condition, Condition::Preset(id) if Some(*id) == preset);
    conditions.all_of.iter().any(other) || conditions.none_of.iter().any(current)
}

/// What a walk is in: which checks its values take.
#[derive(Clone, Copy, Debug)]
enum Region {
    /// The colour roles.
    Roles,
    /// The tokens.
    Tokens,
    /// A typed widget's part: the widget, the part, its catalogue entry
    /// and the widget's parts.
    Part(
        WidgetKind,
        TypedPart,
        &'static PartInfo,
        &'static [PartInfo],
    ),
    /// A typed widget's values.
    Values(WidgetKind),
}

/// A declaration of S10 a keyed name is checked against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Decl {
    /// `color_roles`: the keyed colour roles.
    ColorRoles,
    /// `accent_ramp`: the accent ramp's steps.
    AccentRamp,
    /// `text_styles`: the keyed text styles.
    TextStyles,
    /// `elevation`: the elevation levels.
    Elevation,
    /// `drag_cursors`: the drag cursors.
    DragCursors,
    /// `symbolic_colors`: an icon's symbolic colours, by mark name.
    SymbolicColors,
    /// `icon_contexts`: the keyed icon contexts.
    IconContexts,
    /// `params`: the parameter groups.
    Params,
    /// `symbol_scales`: the symbol scales.
    SymbolScales,
    /// `icon_state_effects`: the icon state effects.
    IconStateEffects,
    /// `symbol_variants`: the symbol variants.
    SymbolVariants,
    /// `spec_versions`: the material scheme's spec versions.
    SpecVersions,
    /// `materials`: the materials.
    Materials,
    /// `blend_modes`: the keyed blend modes.
    BlendModes,
    /// `focus_kinds`: the keyed focus kinds.
    FocusKinds,
    /// `presentation_events`: the keyed presentation events.
    PresentationEvents,
    /// `transition_kinds`: the keyed transition kinds.
    TransitionKinds,
    /// `presentation_kinds`: the keyed presentation kinds.
    PresentationKinds,
}

impl Decl {
    /// Every declaration.
    const ALL: [Decl; 18] = [
        Decl::ColorRoles,
        Decl::AccentRamp,
        Decl::TextStyles,
        Decl::Elevation,
        Decl::DragCursors,
        Decl::SymbolicColors,
        Decl::IconContexts,
        Decl::Params,
        Decl::SymbolScales,
        Decl::IconStateEffects,
        Decl::SymbolVariants,
        Decl::SpecVersions,
        Decl::Materials,
        Decl::BlendModes,
        Decl::FocusKinds,
        Decl::PresentationEvents,
        Decl::TransitionKinds,
        Decl::PresentationKinds,
    ];

    /// Its name in `[declare]`.
    fn name(self) -> &'static str {
        match self {
            Decl::ColorRoles => "color_roles",
            Decl::AccentRamp => "accent_ramp",
            Decl::TextStyles => "text_styles",
            Decl::Elevation => "elevation",
            Decl::DragCursors => "drag_cursors",
            Decl::SymbolicColors => "symbolic_colors",
            Decl::IconContexts => "icon_contexts",
            Decl::Params => "params",
            Decl::SymbolScales => "symbol_scales",
            Decl::IconStateEffects => "icon_state_effects",
            Decl::SymbolVariants => "symbol_variants",
            Decl::SpecVersions => "spec_versions",
            Decl::Materials => "materials",
            Decl::BlendModes => "blend_modes",
            Decl::FocusKinds => "focus_kinds",
            Decl::PresentationEvents => "presentation_events",
            Decl::TransitionKinds => "transition_kinds",
            Decl::PresentationKinds => "presentation_kinds",
        }
    }

    /// The declaration named `name`.
    fn from_name(name: &str) -> Option<Decl> {
        Decl::ALL.into_iter().find(|decl| decl.name() == name)
    }

    /// The keys it lists; `params`' are its groups' names, a keyed kind's
    /// its kinds'.
    fn keys(self, decl: &Declarations) -> Vec<&Key> {
        let keys: &[Key] = match self {
            Decl::ColorRoles => &decl.color_roles,
            Decl::AccentRamp => &decl.accent_ramp,
            Decl::TextStyles => &decl.text_styles,
            Decl::Elevation => &decl.elevation,
            Decl::DragCursors => &decl.drag_cursors,
            Decl::SymbolicColors => &decl.symbolic_colors,
            Decl::IconContexts => &decl.icon_contexts,
            Decl::SymbolScales => &decl.symbol_scales,
            Decl::IconStateEffects => &decl.icon_state_effects,
            Decl::SymbolVariants => &decl.symbol_variants,
            Decl::SpecVersions => &decl.spec_versions,
            Decl::Materials => &decl.materials,
            Decl::BlendModes => &decl.blend_modes,
            Decl::FocusKinds => &decl.focus_kinds,
            Decl::PresentationEvents => &decl.presentation_events,
            Decl::TransitionKinds => return decl.transition_kinds.iter().map(|(k, _)| k).collect(),
            Decl::PresentationKinds => {
                return decl.presentation_kinds.iter().map(|(k, _)| k).collect();
            }
            Decl::Params => return decl.params.iter().map(|(k, _)| k).collect(),
        };
        keys.iter().collect()
    }
}

/// The declaration of the keyed names of one slot (S10): the slot, a field
/// or an enum's variant, by its owner and name; whether a problem writes the
/// name `@<key>` (an enum's keyed variant); whether the slot is a map whose
/// keys a base states.
#[derive(Clone, Copy, Debug)]
struct Declaration {
    owner: &'static str,
    field: &'static str,
    decl: Decl,
    keyed: bool,
    map: bool,
}

impl Declaration {
    /// The declaration's name in `[declare]`.
    fn name(&self) -> &'static str {
        self.decl.name()
    }
}

/// A slot's declaration.
const fn row(
    owner: &'static str,
    field: &'static str,
    decl: Decl,
    keyed: bool,
    map: bool,
) -> Declaration {
    Declaration {
        owner,
        field,
        decl,
        keyed,
        map,
    }
}

/// The keyed names' declarations by the slot holding them (S10). One table
/// for the checks, for what a preset uses and for a key field's type.
const DECLARATIONS: [Declaration; 20] = [
    row("ColorRoles", "keyed", Decl::ColorRoles, false, true),
    row("GlobalRoles", "accent_ramp", Decl::AccentRamp, false, true),
    row("TextTokens", "keyed", Decl::TextStyles, false, true),
    row("Tokens", "elevation", Decl::Elevation, false, true),
    row(
        "IconStyle",
        "symbolic_colors",
        Decl::SymbolicColors,
        false,
        true,
    ),
    row("IconSizeTokens", "keyed", Decl::IconContexts, false, true),
    row("Tokens", "params", Decl::Params, false, true),
    row("IconStyle", "scale", Decl::SymbolScales, false, false),
    row(
        "IconStyle",
        "state_effect",
        Decl::IconStateEffects,
        false,
        false,
    ),
    row(
        "IconStyle",
        "symbol_variants",
        Decl::SymbolVariants,
        false,
        false,
    ),
    row("Material", "identity", Decl::Materials, false, false),
    row(
        "MaterialScheme",
        "spec_version",
        Decl::SpecVersions,
        false,
        false,
    ),
    row("Shadow", "elevation", Decl::Elevation, false, false),
    row("GlobalRole", "accent_ramp", Decl::AccentRamp, false, false),
    row("BlendMode", "keyed", Decl::BlendModes, true, false),
    row("FocusKind", "keyed", Decl::FocusKinds, true, false),
    row(
        "PresentationEvent",
        "keyed",
        Decl::PresentationEvents,
        true,
        false,
    ),
    row("RoleRef", "keyed", Decl::ColorRoles, true, false),
    row("TransitionKind", "key", Decl::TransitionKinds, true, false),
    row(
        "PresentationKind",
        "key",
        Decl::PresentationKinds,
        true,
        false,
    ),
];

/// The declaration a keyed name in the slot `slot` (a field or an enum's
/// variant, owner and name) is checked against: a map's key by the map's
/// field, a `Key` leaf by its field or variant (S10).
fn declaration_of(slot: Option<(&'static str, &'static str)>) -> Option<Declaration> {
    let (owner, field) = slot?;
    DECLARATIONS
        .iter()
        .find(|row| row.owner == owner && row.field == field)
        .copied()
}

/// The declaration a `Key` field `field` of the block `owner` holds a key
/// of: a reference to the field has that key's type (S2.2).
fn declaration_of_field(owner: &str, field: &str) -> Option<&'static str> {
    DECLARATIONS
        .iter()
        .find(|row| row.owner == owner && row.field == field && !row.keyed && !row.map)
        .map(Declaration::name)
}

/// The keys the declaration `name` lists; none for a name that is no
/// declaration of S10's (a widget's `keys` entry is read elsewhere).
fn declared<'a>(decl: &'a Declarations, name: &str) -> Vec<&'a Key> {
    Decl::from_name(name).map_or_else(Vec::new, |d| d.keys(decl))
}

struct Checker<'a> {
    cx: &'a Context<'a>,
    problems: Vec<Problem>,
    region: Region,
    /// Walking a layer's bases, where `base` is an error.
    in_bases: bool,
    /// The layer was read from TOML: its paths are spelled by its keys.
    toml: bool,
    /// The flavour whose declarations a widget's names are checked
    /// against; `None`: each flavour's, a name declared in any.
    flavor: Option<&'a Key>,
    /// The declared parameters of the keyed transition or presentation kind
    /// walked: what its `params` are typed by (S2.7, S10).
    keyed_kind: Option<&'a [(Key, ScalarUnit)]>,
    /// Each widget's declarations under the flavours checked against,
    /// computed once.
    widget_decls: Vec<(WidgetKind, Rc<[WidgetDecl]>)>,
    /// The listed paths a preset's completeness check leaves to the layers
    /// over it (S17, ADD-A128).
    exempt: Vec<String>,
    /// Walking the fields kept beneath a part's `Absent` or `DrawnAs`: the
    /// part needs none, so neither its list nor any block below it is a
    /// completeness check (S6.3).
    latent: bool,
    /// The typed widget of the fade's nested element walked, which its
    /// `variant`, walked next, is checked against (S2.8).
    fade_widget: Option<WidgetKind>,
}

impl<'a> Checker<'a> {
    fn new(cx: &'a Context<'a>, toml: bool) -> Checker<'a> {
        Checker {
            cx,
            problems: Vec::new(),
            region: Region::Tokens,
            in_bases: false,
            toml,
            flavor: None,
            keyed_kind: None,
            widget_decls: Vec::new(),
            exempt: Vec::new(),
            latent: false,
            fade_widget: None,
        }
    }

    fn decl(&self) -> &'a Declarations {
        self.cx.declarations
    }

    fn problem(&mut self, at: &str, kind: ProblemKind) {
        self.problems.push(Problem::new(at, kind));
    }

    fn undeclared(&mut self, at: &str, name: &str, declaration: &str) {
        self.problem(
            at,
            ProblemKind::Undeclared {
                name: name.to_owned(),
                declaration: declaration.to_owned(),
            },
        );
    }

    /// Walks `value` from `root` in `region`.
    fn walk<T: Walk + ?Sized>(&mut self, root: &str, region: Region, value: &T) {
        self.region = region;
        let toml = self.toml;
        let mut walker = Walker::new(root, toml, self);
        walker.walk(value);
    }

    fn bases(&mut self, bases: &Bases, at: &str) {
        self.in_bases = true;
        let colors = join(at, "colors");
        let whole = self.cx.kind == LayerKind::Preset;
        for (scheme, roles) in [("light", &bases.colors.light), ("dark", &bases.colors.dark)] {
            match roles {
                Some(roles) => {
                    let at = join(&colors, scheme);
                    let roles = self.palette_roles(roles);
                    self.walk(&at, Region::Roles, &*roles);
                    if whole {
                        self.surfaces_stated(roles.surfaces.as_ref(), &join(&at, "surfaces"));
                    }
                }
                None if whole => self.problem(&join(&colors, scheme), ProblemKind::Missing),
                None => {}
            }
        }
        self.inputs(&bases.inputs, &join(at, "inputs"));
        self.walk(&join(at, "tokens"), Region::Tokens, &bases.tokens);
        self.widgets(&bases.widgets, &join(at, "widgets"));
        self.in_bases = false;
    }

    /// In a preset's own bases, every declared surface set's roles are
    /// stated (S2.2, S3), a set the bases leave out being missing as
    /// [`unlisted`] gives it beside the listed paths (S17, ADD-A131): none
    /// where each of its roles is listed.
    fn surfaces_stated<T>(&mut self, sets: Option<&SurfaceMap<T>>, at: &str) {
        for set in &self.decl().surface_sets {
            if sets.is_none_or(|sets| sets.get(*set).is_none()) {
                for at in unlisted(&join(at, set.name()), &self.exempt) {
                    self.problem(&at, ProblemKind::Missing);
                }
            }
        }
    }

    fn widgets(&mut self, widgets: &WidgetsPatch, at: &str) {
        let WidgetsPatch { button } = widgets;
        let at = join(at, "button");
        match button {
            Some(button) => self.button(button, &at),
            None => self.widget_missing(WidgetKind::Button, &at),
        }
    }

    /// In a preset's own bases, an available widget is stated (S2.2, S10).
    fn widget_missing(&mut self, widget: WidgetKind, at: &str) {
        if self.cx.kind == LayerKind::Preset && self.available(widget) {
            self.problem(at, ProblemKind::Missing);
        }
    }

    fn button(&mut self, button: &ButtonPatch, at: &str) {
        let (parts, values) = button.clone().into_parts();
        let whole = self.cx.kind == LayerKind::Preset;
        if whole {
            let info = registry::widgets()
                .iter()
                .find(|info| info.kind == WidgetKind::Button);
            for part in info.map_or(&[][..], |info| info.parts) {
                if !parts.iter().any(|(stated, _)| stated.name() == part.name) {
                    for at in unlisted_part(&join(at, part.name), part, &self.exempt) {
                        self.problem(&at, ProblemKind::Missing);
                    }
                }
            }
        }
        for (part, patch) in parts {
            self.part(TypedPart::Button(part), &patch, &join(at, part.name()));
        }
        let values_at = join(at, "values");
        match values {
            Some(values) => self.walk(&values_at, Region::Values(WidgetKind::Button), &values),
            None if whole => self.problem(&values_at, ProblemKind::Missing),
            None => {}
        }
    }

    /// A part's patch (S6.1, S6.3): absent only where the part may be, a
    /// `DrawnAs` of an available widget and declared variants, a style only
    /// of the part's fields; the fields beside an `Absent` or a `DrawnAs`
    /// are kept beneath it, so none is missing there (ADD-A131).
    fn part(&mut self, part: TypedPart, patch: &PartPatch, at: &str) {
        let widget = widget_of(part);
        let Some(info) = part_info(part) else {
            return;
        };
        match &patch.presence {
            Some(PartPresence::Absent) => {
                if info.root || !info.may_be_absent {
                    self.problem(at, ProblemKind::AbsentNotAllowed);
                }
            }
            Some(PartPresence::DrawnAs { target, variants }) => {
                self.drawn_as(target, variants, at);
            }
            Some(PartPresence::Present) | None => {}
        }
        let style = &*patch.style;
        self.part_fields(style, widget, info, at);
        let parts = registry::widgets()
            .iter()
            .find(|w| w.kind == widget)
            .map_or(&[] as &[PartInfo], |w| w.parts);
        self.region = Region::Part(widget, part, info, parts);
        let toml = self.toml;
        let latent = std::mem::replace(
            &mut self.latent,
            !matches!(patch.presence, Some(PartPresence::Present) | None),
        );
        let mut walker = Walker::new(at, toml, self);
        walker.part(info.fields, |w| w.walk(style));
        self.latent = latent;
    }

    /// A style's fields, each one its part's catalogue entry lists (S2.11).
    fn part_fields(
        &mut self,
        style: &PartStylePatch,
        widget: WidgetKind,
        info: &PartInfo,
        at: &str,
    ) {
        for field in style.set_fields() {
            if !info.fields.iter().any(|listed| listed.path == field) {
                let part = format!("{}.{}", widget_name(widget), info.name);
                self.problem(&join(at, field), ProblemKind::NotAPartField { part });
            }
        }
    }

    /// A `DrawnAs` (S6.1): its target at `drawn_as.target`, or, read from
    /// TOML, at the key `drawn_as.widget` that names its widget (S11).
    fn drawn_as(&mut self, target: &WidgetOrPart, variants: &[VariantRef], at: &str) {
        let drawn_as = join(at, "drawn_as");
        let target_at = join(&drawn_as, if self.toml { "widget" } else { "target" });
        let widget = match target {
            WidgetOrPart::Widget(WidgetRef::Typed(widget)) => Some(*widget),
            WidgetOrPart::Part(PartRef::Typed(part)) => Some(widget_of(*part)),
            WidgetOrPart::Widget(WidgetRef::Component(key))
            | WidgetOrPart::Part(PartRef::Component { key, .. }) => {
                self.undeclared(&target_at, &format!("component.{key}"), "component");
                None
            }
        };
        let Some(widget) = widget else { return };
        if !self.available(widget) {
            self.problem(
                &target_at,
                ProblemKind::WidgetUnavailable {
                    widget: widget_name(widget),
                },
            );
        }
        for (index, variant) in variants.iter().enumerate() {
            self.variant(widget, variant, &format!("{drawn_as}.variants[{index}]"));
        }
    }

    /// The widget's declarations under each flavour (S10): a name is
    /// declared where some flavour declares it; which flavour a rule
    /// applies under is S12's combination check's.
    fn widget_decls(&mut self, widget: WidgetKind) -> Rc<[WidgetDecl]> {
        if let Some((_, decls)) = self.widget_decls.iter().find(|(held, _)| *held == widget) {
            return Rc::clone(decls);
        }
        let decls: Rc<[WidgetDecl]> = match self.flavor {
            Some(flavor) => Rc::from([self.decl().in_flavor(widget, flavor)]),
            None => Rc::from(self.decl().in_each_flavor(widget)),
        };
        self.widget_decls.push((widget, Rc::clone(&decls)));
        decls
    }

    /// Whether the widget is available under the default flavour (S10), or
    /// under the flavour the walk checks against.
    fn available(&self, widget: WidgetKind) -> bool {
        let decl = match self.flavor {
            Some(flavor) => self.decl().in_flavor(widget, flavor),
            None => self.decl().in_default_flavor(widget),
        };
        decl.available != Some(false)
    }

    /// The name of the widget's declaration `key` in the declarations
    /// checked against: a flavour's under that flavour.
    fn declaration(&self, widget: WidgetKind, key: &str) -> String {
        match self.flavor {
            Some(flavor) => format!("flavor.{flavor}.widget.{}.{key}", widget_name(widget)),
            None => format!("widget.{}.{key}", widget_name(widget)),
        }
    }

    fn variant(&mut self, widget: WidgetKind, variant: &VariantRef, at: &str) {
        let decls = self.widget_decls(widget);
        let declared = decls
            .iter()
            .any(|decl| decl.variants.as_ref().is_some_and(|v| v.contains(variant)));
        if !declared {
            let declaration = self.declaration(widget, "variants");
            self.undeclared(at, &variant_name(variant), &declaration);
        } else if decls.iter().any(|decl| decl.is_later(variant)) {
            let declaration = self.declaration(widget, "later");
            self.later(at, &variant_name(variant), &declaration);
        }
    }

    /// A name the preset declares that a unit not yet enabled states (S10's
    /// *Later names*, S12; ADD-A147).
    fn later(&mut self, at: &str, name: &str, declaration: &str) {
        self.problem(
            at,
            ProblemKind::Later {
                name: name.to_owned(),
                declaration: declaration.to_owned(),
            },
        );
    }

    fn flag(&mut self, widget: WidgetKind, flag: &StateFlag, at: &str) {
        let StateFlag::Widget(key) = flag else {
            return;
        };
        let declared = self
            .widget_decls(widget)
            .iter()
            .any(|decl| decl.flags.as_ref().is_some_and(|f| f.contains(flag)));
        if !declared {
            let declaration = self.declaration(widget, "flags");
            self.undeclared(at, &format!("@{key}"), &declaration);
        }
    }

    /// A size a rule selects, declared where the widget's sizes or a
    /// variant entry's take it: a context entry only leaves sizes out, so a
    /// size it alone names is taken by no element (S10, ADD-A131).
    fn size(&mut self, widget: WidgetKind, size: ControlSize, at: &str) {
        let decls = self.widget_decls(widget);
        let declared = decls.iter().any(|decl| {
            decl.sizes.as_ref().is_some_and(|s| s.contains(&size))
                || decl
                    .sizes_by_variant
                    .iter()
                    .flatten()
                    .any(|entry| entry.sizes.contains(&size))
        });
        // A size of a unit not enabled (S10's *Later names*; ADD-A147): no
        // default in force of the widget's, its variant entries' or its
        // context entries'; while the flavours unit is not enabled, the
        // default flavour's alone, and no entry of a `later` variant, as no
        // element holds one.
        let default =
            |held: Option<&DefaultSize>| matches!(held, Some(DefaultSize::Size(d)) if *d == size);
        let in_force_decls = if crate::widgets::FLAVOURS_UNIT {
            &decls[..]
        } else {
            decls.get(..1).unwrap_or_default()
        };
        let in_force = in_force_decls.iter().any(|decl| {
            default(decl.default_size.as_ref())
                || decl
                    .sizes_by_variant
                    .iter()
                    .flatten()
                    .filter(|entry| !decl.is_later(&entry.variant))
                    .any(|entry| default(entry.default_size.as_ref()))
                || decl
                    .sizes_by_context
                    .iter()
                    .flatten()
                    .any(|entry| default(entry.default_size.as_ref()))
        });
        if !declared {
            let declaration = self.declaration(widget, "sizes");
            self.undeclared(at, size.name(), &declaration);
        } else if !crate::widgets::SIZES_UNIT && !in_force {
            let declaration = self.declaration(widget, "sizes");
            self.later(at, size.name(), &declaration);
        }
    }

    fn density(&mut self, widget: WidgetKind, density: &Density, at: &str) {
        let declared = self
            .widget_decls(widget)
            .iter()
            .any(|decl| decl.densities.as_ref().is_some_and(|d| d.contains(density)));
        let name = match density {
            Density::Default => "default".to_owned(),
            Density::Compact => "compact".to_owned(),
            Density::Keyed(key) => format!("@{key}"),
        };
        let declaration = self.declaration(widget, "densities");
        if !declared {
            self.undeclared(at, &name, &declaration);
        } else if !crate::widgets::SIZES_UNIT {
            // A density of a unit not enabled: the densities are the sizes
            // unit's (S10's *Later names*; ADD-A147).
            self.later(at, &name, &declaration);
        }
    }

    fn rule(&mut self, rule: &Rule, at: &str) {
        let (when, set, target) = if self.toml {
            ("when", "set", "target")
        } else {
            ("selector", "body", "body")
        };
        let (when, set, target) = (join(at, when), join(at, set), join(at, target));
        let widget = match &rule.body {
            RuleBody::Colors(roles) => {
                self.conditions_only(&rule.selector, &when);
                let roles = self.palette_roles(roles);
                self.walk(&set, Region::Roles, &*roles);
                None
            }
            RuleBody::Tokens(tokens) => {
                self.conditions_only(&rule.selector, &when);
                self.walk(&set, Region::Tokens, tokens);
                None
            }
            RuleBody::Part(PartRef::Typed(part), patch) => {
                self.part(*part, patch, &set);
                Some(widget_of(*part))
            }
            RuleBody::Part(PartRef::Component { key, part }, _) => {
                let name = format!("component.{key}.{part}");
                self.undeclared(&target, &name, "component");
                None
            }
            RuleBody::Values(ValuesPatch::Typed(values)) => match values {
                WidgetValuesPatch::Button(values) => {
                    let widget = WidgetKind::Button;
                    self.walk(&set, Region::Values(widget), values);
                    Some(widget)
                }
            },
        };
        self.selector(&rule.selector, widget, &when);
    }

    /// A selector's size field `name`: a `when` key of its own in TOML, a
    /// field of `size` in the model.
    fn size_field(&self, name: &'static str) -> &'static str {
        match (self.toml, name) {
            (false, "sizes") => "size.sizes",
            (false, "densities") => "size.densities",
            _ => name,
        }
    }

    /// A `Colors` or `Tokens` rule selects on conditions only (S7, S12).
    fn conditions_only(&mut self, selector: &Selector, at: &str) {
        let only = Selector {
            conditions: selector.conditions.clone(),
            ..Selector::default()
        };
        if *selector != only {
            self.problem(at, ProblemKind::NotConditions);
        }
    }

    /// A selector's names, each declared (S10), each at its `when` key and
    /// index (`rules[2].when.not_states[0]`, S11), or at its field and index
    /// in a layer built in Rust (`rules[2].selector.states.none_of[0]`).
    fn selector(&mut self, selector: &Selector, widget: Option<WidgetKind>, at: &str) {
        let positive = selector.conditions.all_of.len();
        for (index, (condition, item)) in items(&selector.conditions, at, "conditions", self.toml)
            .into_iter()
            .enumerate()
        {
            self.condition(condition, &item, index >= positive);
        }
        if let Some(widget) = widget {
            self.element(widget, &selector.variants, &selector.states, at);
            self.latest(widget, &selector.latest, at);
            for (size, item) in items(
                &selector.size.sizes,
                at,
                self.size_field("sizes"),
                self.toml,
            ) {
                self.size(widget, *size, &item);
            }
            for (density, item) in items(
                &selector.size.densities,
                at,
                self.size_field("densities"),
                self.toml,
            ) {
                self.density(widget, density, &item);
            }
        }
        for (list, contexts) in [
            ("context", &selector.context),
            ("not_context", &selector.not_context),
        ] {
            for (index, context) in contexts.iter().enumerate() {
                self.context(context, &format!("{at}.{list}[{index}]"));
            }
        }
        if let Some(position) = &selector.position {
            self.position(position, &join(at, "position"));
        }
    }

    /// A selector's `latest` terms (S7.2, S12): each set one the widget
    /// declares, its flag a member of it, at `latest.<set>` (S11) or at the
    /// term's index in a layer built in Rust.
    fn latest(&mut self, widget: WidgetKind, latest: &[(Key, StateFlag)], at: &str) {
        for (index, (set, flag)) in latest.iter().enumerate() {
            let item = if self.toml {
                format!("{at}.latest.{set}")
            } else {
                format!("{at}.latest[{index}]")
            };
            let decls = self.widget_decls(widget);
            let members: Vec<&Vec<StateFlag>> = decls
                .iter()
                .filter_map(|decl| {
                    decl.recency
                        .iter()
                        .flatten()
                        .find(|(held, _)| held == set)
                        .map(|(_, flags)| flags)
                })
                .collect();
            if members.len() < decls.len() {
                let declaration = self.declaration(widget, "recency");
                self.undeclared(&item, &set.to_string(), &declaration);
            } else if members.iter().any(|flags| !flags.contains(flag)) {
                let kind = ProblemKind::NotInRecencySet {
                    flag: flag.name(),
                    set: set.to_string(),
                };
                self.problem(&item, kind);
            }
        }
    }

    /// An element's variants and flags, of `widget`.
    fn element(
        &mut self,
        widget: WidgetKind,
        variants: &SetMatch<VariantRef>,
        states: &SetMatch<StateFlag>,
        at: &str,
    ) {
        for (variant, item) in items(variants, at, "variants", self.toml) {
            self.variant(widget, variant, &item);
        }
        for (flag, item) in items(states, at, "states", self.toml) {
            self.flag(widget, flag, &item);
        }
    }

    fn context(&mut self, context: &ContextMatch, at: &str) {
        if let Some(surface) = context.surface {
            self.surface_set(surface, &join(at, "surface"));
        }
        let Some(widget) = target_widget(context.target.as_ref()) else {
            return;
        };
        self.element(widget, &context.variants, &context.states, at);
        for (size, item) in items(&context.sizes, at, "sizes", self.toml) {
            self.size(widget, *size, &item);
        }
    }

    fn position(&mut self, position: &PositionMatch, at: &str) {
        for (name, sibling) in [("previous", &position.previous), ("next", &position.next)] {
            if let Some(sibling) = sibling {
                self.sibling(sibling, &join(at, name));
            }
        }
    }

    fn sibling(&mut self, sibling: &SiblingMatch, at: &str) {
        let Some(widget) = target_widget(sibling.target.as_ref()) else {
            return;
        };
        self.element(widget, &sibling.variants, &sibling.states, at);
    }

    /// A condition, declared (S10); `negative` where a selector's
    /// `not_conditions` names it.
    fn condition(&mut self, condition: &Condition, at: &str, negative: bool) {
        let decl = self.decl();
        let (name, declaration, declared) = match condition {
            Condition::SystemSetting(key) => (
                key,
                "conditions",
                decl.conditions.iter().any(|setting| setting.key == *key),
            ),
            Condition::DeviceModel(key) => (key, "device_models", decl.device_models.contains(key)),
            Condition::Flavor(key) => {
                // A flavour of a unit not enabled (S10's *Later names*,
                // S12; ADD-A147): every declared flavour but the default.
                // Named positively, the rule applies under it alone; the
                // default named negatively, under the other flavours alone,
                // each of that unit.
                let default = decl.flavors.first() == Some(key);
                let others = decl.flavors.len() > 1;
                if !crate::widgets::FLAVOURS_UNIT
                    && decl.flavors.contains(key)
                    && if negative {
                        default && others
                    } else {
                        !default
                    }
                {
                    self.later(at, key.as_str(), "flavors");
                    return;
                }
                (key, "flavors", decl.flavors.contains(key))
            }
            Condition::ContainerSize { container, size } => (
                size,
                "container_sizes",
                decl.container_sizes
                    .iter()
                    .any(|(held, sizes)| held == container && sizes.contains(size)),
            ),
            _ => return,
        };
        if !declared {
            self.undeclared(at, name.as_str(), declaration);
        }
    }

    /// A surface set, declared (S10). A palette's statement of a set the
    /// current preset does not declare is skipped before the walk
    /// ([`Checker::palette_roles`]); any other name of a set, a palette's
    /// too, is checked (S12, S14).
    fn surface_set(&mut self, set: SurfaceSet, at: &str) {
        if !self.decl().surface_sets.contains(&set) {
            self.undeclared(at, set.name(), "surface_sets");
        }
    }

    /// A palette's colour roles without the surface sets the current preset
    /// does not declare, which are skipped, not validated (S12, S14); any
    /// other layer's as they are.
    fn palette_roles<'r>(&self, roles: &'r ColorRolesPatch) -> Cow<'r, ColorRolesPatch> {
        let declared = &self.decl().surface_sets;
        let skips = roles
            .surfaces
            .as_ref()
            .is_some_and(|sets| sets.iter().any(|(set, _)| !declared.contains(&set)));
        if self.cx.kind != LayerKind::Palette || !skips {
            return Cow::Borrowed(roles);
        }
        let mut kept = roles.clone();
        if let Some(sets) = kept.surfaces.take() {
            let mut held = SurfaceMap::default();
            for (set, value) in sets
                .into_entries()
                .filter(|(set, _)| declared.contains(set))
            {
                held.insert(set, value);
            }
            kept.surfaces = Some(held);
        }
        Cow::Owned(kept)
    }

    /// The inputs' bases (S9.1): each declared, an accent input only in a
    /// palette's or a reader's layer, KDE's lists naming each set or key
    /// once and only declared ones.
    fn inputs(&mut self, inputs: &InputBases, at: &str) {
        let decl = self.decl();
        let toml = self.toml;
        let entries = if toml {
            at.to_owned()
        } else {
            join(at, "entries")
        };
        for (name, value) in &inputs.entries {
            let path = join(&entries, &name.to_string());
            if !decl.inputs.contains(name) {
                self.undeclared(&path, &name.to_string(), "inputs");
            }
            if self.cx.kind == LayerKind::Application && decl.accent_inputs.contains(name) {
                self.problem(&path, ProblemKind::AccentInput);
            }
            self.input_values(value, &path);
        }
        let kde = |list: &str, field: &str| {
            if toml {
                format!("{at}.kde_scheme.{list}")
            } else {
                join(at, field)
            }
        };
        for (list, map) in [
            (kde("groups", "kde_groups"), &inputs.kde_groups),
            (
                kde("inactive_groups", "kde_inactive_groups"),
                &inputs.kde_inactive_groups,
            ),
        ] {
            for (scheme, sets) in schemes(map) {
                let path = format!("{list}.{scheme}");
                for (index, set) in sets.iter().enumerate() {
                    let item = format!("{path}[{index}]");
                    if sets.iter().take(index).any(|earlier| earlier == set) {
                        let name = set.name().to_owned();
                        self.problem(&item, ProblemKind::ListedTwice { name });
                    }
                    self.surface_set(*set, &item);
                }
            }
        }
        let unset_keys = kde("unset_keys", "kde_unset_keys");
        for (scheme, names) in schemes(&inputs.kde_unset_keys) {
            let path = format!("{unset_keys}.{scheme}");
            for (index, name) in names.iter().enumerate() {
                let item = format!("{path}[{index}]");
                if names.iter().take(index).any(|earlier| earlier == name) {
                    let listed = name.to_string();
                    self.problem(&item, ProblemKind::ListedTwice { name: listed });
                }
                if !(is_kde_scheme_key(name) && decl.inputs.contains(name)) {
                    self.undeclared(&item, &name.to_string(), "inputs");
                }
            }
        }
    }

    /// An input's values, read by the colour roles (S9): per scheme and per
    /// contrast condition, `[inputs.<name>.contrast.<condition>]` in TOML.
    fn input_values(&mut self, value: &InputValue, at: &str) {
        let (per_scheme, per_contrast) = if self.toml {
            (at.to_owned(), join(at, "contrast"))
        } else {
            (join(at, "per_scheme"), join(at, "per_contrast"))
        };
        self.walk(&per_scheme, Region::Roles, &value.per_scheme);
        for (contrast, schemes) in &value.per_contrast {
            self.walk(
                &join(&per_contrast, contrast.name()),
                Region::Roles,
                schemes,
            );
        }
    }

    /// An expression's structural checks (S2.2, S3.3, S10, S12); one nested
    /// deeper than [`MAX_DEPTH`] is that one problem, not followed down.
    ///
    /// An expression with no structural problem is then typed against its
    /// field (S2.2, S12): one that mistypes, names nothing, or calls a
    /// platform function not implemented yet is refused. An expression's
    /// structural problems are its one problem each, so a fault the
    /// structure shows is not reported again by its typing.
    fn expression(&mut self, path: &Path, expr: &Expr, form: ExprForm, ty: Option<Ty>) {
        let at = path.to_string();
        if depth(expr) > MAX_DEPTH {
            self.problem(&at, ProblemKind::TooDeep);
            return;
        }
        let found = self.problems.len();
        // A bool, a text, a key or an enum takes a literal or a reference
        // (S12's call item); a block (an enum block among them), an option
        // or a map a reference only, which typing refuses a call on as the
        // mistype it is (S2.2), and so the walk here where it gives no type.
        if matches!(expr, Expr::Call(..)) {
            match form {
                ExprForm::NoCalls => self.problem(&at, ProblemKind::CallNotAllowed),
                ExprForm::ReferenceOnly if ty.is_none() => {
                    let reason = "the field's expression is a reference only".to_owned();
                    self.problem(&at, ProblemKind::IllTyped { reason });
                }
                ExprForm::Calls | ExprForm::ReferenceOnly => {}
            }
        }
        let in_text_style = path.fields().any(|(owner, _)| owner == "TextStyle");
        let on_font_size = path.last_field() == Some(("Font", "size"));
        // `base` in a keyframe's patch is the part's value beneath it,
        // wherever the keyframe stands (S2.7).
        let in_bases = self.in_bases;
        self.in_bases = in_bases && !path.within("Keyframe", "patch");
        self.expr_node(expr, &at, in_text_style, on_font_size);
        self.in_bases = in_bases;
        let ty = match ty {
            Some(Ty::Key(set)) if set == KEY_OF_PLACE => self.key_type(path),
            Some(ty) => Some(ty),
            None => self.declared_unit(path).map(unit_type),
        };
        if self.problems.len() == found
            && let Some(ty) = ty
        {
            let types = Types {
                decl: self.decl(),
                region: self.region,
            };
            if let Err(error) = expr.check(&FieldType::new(ty, Bound::Unbounded), &types) {
                let reason = error.to_string();
                self.problem(&at, ProblemKind::IllTyped { reason });
            }
        }
    }

    /// The type of a key at `path` (S2.2, S10): a key of its field's
    /// declaration, or a widget value's, a key its `keys` entry lists.
    fn key_type(&self, path: &Path) -> Option<Ty> {
        if let Some(declaration) = declaration_of(path.slot()).filter(|d| !d.map) {
            return Some(Ty::Key(Cow::Borrowed(declaration.name())));
        }
        match (self.region, path.first_field()) {
            (Region::Values(widget), Some((_, value))) => Some(widget_keys(widget, value)),
            _ => None,
        }
    }

    /// One node's checks, then its arguments'; `expression` has bounded the
    /// nesting by [`MAX_DEPTH`].
    fn expr_node(&mut self, expr: &Expr, at: &str, in_text_style: bool, on_font_size: bool) {
        match expr {
            Expr::Lit(_) => {}
            Expr::Ref(reference) => self.reference(reference, at, in_text_style),
            Expr::Call(func, args) => {
                self.call(*func, args, at, on_font_size);
                for arg in args {
                    self.expr_node(arg, at, in_text_style, on_font_size);
                }
            }
        }
    }

    /// A call's own checks; `on_font_size`: its expression is stated on a
    /// `Font.size`.
    fn call(&mut self, func: Func, args: &[Expr], at: &str, on_font_size: bool) {
        match func {
            Func::Pick if args.len() < 2 => self.problem(at, ProblemKind::PickWithoutColors),
            Func::RoundPt => {
                let operand_refused = match args {
                    [Expr::Ref(reference)] => !self.is_font_size(reference),
                    [_] => true,
                    _ => false,
                };
                if !on_font_size || operand_refused {
                    self.problem(at, ProblemKind::RoundPtMisused);
                }
            }
            Func::KirigamiLerp => {
                if let Some(Expr::Lit(Literal::Number(balance))) = args.get(2)
                    && !(0.0..=1.0).contains(balance)
                {
                    self.problem(at, ProblemKind::OutOfRange { expected: "0..=1" });
                }
            }
            Func::KdeAccent => {
                let name = |index: usize| match args.get(index) {
                    Some(Expr::Lit(Literal::Name(Name::Variant(name)))) => Some(name.as_str()),
                    _ => None,
                };
                let wrong = [
                    name(0).filter(|set| !KDE_SETS.contains(set)),
                    name(2).filter(|group| *group == "disabled"),
                ];
                for name in wrong.into_iter().flatten() {
                    let name = name.to_owned();
                    self.problem(at, ProblemKind::KdeAccentArgument { name });
                }
                // One of KDE's sets, declared by the preset (S2.2).
                if let Some(set) = name(0)
                    .filter(|set| KDE_SETS.contains(set))
                    .and_then(SurfaceSet::from_name)
                {
                    self.surface_set(set, at);
                }
                self.own_operands(func, args, at);
            }
            Func::KdeAccentWm => self.own_operands(func, args, at),
            _ => {}
        }
    }

    /// The inputs and tokens a KDE accent call reads itself, checked as its
    /// operands (S2.2): each input declared, each once; KDE's three lists
    /// are S9.1's own, no declared input.
    fn own_operands(&mut self, func: Func, args: &[Expr], at: &str) {
        let operands = func.token_operands(args);
        for (index, reference) in operands.iter().enumerate() {
            let earlier = operands.get(..index).unwrap_or_default();
            if !is_kde_list(reference) && !earlier.contains(reference) {
                self.reference_names(reference, at);
            }
        }
    }

    /// Whether `reference` names a font size (S2.2): a field of that type,
    /// or another part's `font_size` metric.
    fn is_font_size(&self, reference: &Ref) -> bool {
        let types = Types {
            decl: self.decl(),
            region: self.region,
        };
        let metric = matches!(reference, Ref::Widget { path, .. }
            if matches!(path.as_slice(), [metric] if metric.as_str() == "font_size"));
        metric || types.reference(reference) == Some(Ty::FontSize)
    }

    fn reference(&mut self, reference: &Ref, at: &str, in_text_style: bool) {
        if *reference == Ref::Base && self.in_bases {
            self.problem(at, ProblemKind::BaseInBases);
        }
        if let Region::Roles | Region::Tokens = self.region {
            let element_only = match reference {
                Ref::Element(_)
                | Ref::Widget { .. }
                | Ref::Values(_)
                | Ref::Part(_)
                | Ref::BodyFontHeight => true,
                Ref::FontSize | Ref::LineHeight => {
                    !(matches!(self.region, Region::Tokens) && in_text_style)
                }
                _ => false,
            };
            if element_only {
                let operand = reference.to_string();
                self.problem(at, ProblemKind::ElementOperand { operand });
            }
            if matches!(self.region, Region::Roles) && matches!(reference, Ref::Surface(_)) {
                self.problem(at, ProblemKind::SurfaceInRoles);
            }
        }
        match self.region {
            Region::Part(widget, part, info, parts) => {
                self.element_data(reference, widget, DataOwner::Part(part), at);
                if let Ref::Widget { part: other, .. } = reference
                    && let Some(part) = other_element(parts, info, other.as_str())
                {
                    self.problem(at, ProblemKind::OtherElementsPart { part });
                }
            }
            Region::Values(widget) => self.element_data(reference, widget, DataOwner::Values, at),
            Region::Roles | Region::Tokens => {}
        }
        self.reference_names(reference, at);
    }

    /// An `element.` operand, `body_font_height` or another part's font
    /// metric, declared by the part's `data` (S10); the root's inputs need
    /// none.
    fn element_data(&mut self, reference: &Ref, widget: WidgetKind, owner: DataOwner, at: &str) {
        let name = match reference {
            Ref::Element(datum) if !is_root_input(*datum) => DataName::Element(*datum),
            Ref::BodyFontHeight => DataName::BodyFontHeight,
            Ref::Widget { part, path } => {
                let metric = match path.as_slice() {
                    [metric] => <FontMetric as Named>::from_name(metric.as_str()),
                    _ => None,
                };
                match (TypedPart::from_names(widget, part.as_str()), metric) {
                    (Some(part), Some(metric)) => DataName::PartMetric { part, metric },
                    _ => return,
                }
            }
            _ => return,
        };
        let declared = self.widget_decls(widget).iter().any(|decl| {
            decl.data
                .iter()
                .flatten()
                .any(|(held, names)| *held == owner && names.contains(&name))
        });
        if !declared {
            let datum = reference.to_string();
            self.problem(at, ProblemKind::UndeclaredData { datum });
        }
    }

    /// A reference's keyed names, each declared (S10).
    fn reference_names(&mut self, reference: &Ref, at: &str) {
        let decl = self.decl();
        fn segment(path: &[Key], index: usize) -> Option<&str> {
            path.get(index).map(Key::as_str)
        }
        match reference {
            Ref::Role { set, .. } => self.surface_set(*set, at),
            Ref::KeyedRole(key) => {
                if !decl.color_roles.contains(key) {
                    self.undeclared(at, &format!("@{key}"), "color_roles");
                }
            }
            Ref::GlobalRole(path) => {
                if let (Some("accent_ramp"), Some(key)) = (segment(path, 0), path.get(1))
                    && !decl.accent_ramp.contains(key)
                {
                    self.undeclared(at, key.as_str(), "accent_ramp");
                }
            }
            Ref::Active(inner) => self.reference_names(inner, at),
            Ref::Token(path) => {
                let keyed: Option<(&[Key], &str, Option<&Key>)> =
                    match (segment(path, 0), segment(path, 1)) {
                        (Some("text"), Some("keyed")) => {
                            Some((&decl.text_styles, "text_styles", path.get(2)))
                        }
                        (Some("elevation"), _) => Some((&decl.elevation, "elevation", path.get(1))),
                        (Some("params"), _) => {
                            let group = path
                                .get(1)
                                .and_then(|g| decl.params.iter().find(|(held, _)| held == g));
                            match (path.get(1), group) {
                                (Some(_), Some((_, group))) => {
                                    Some((&group.keys, "params", path.get(2)))
                                }
                                (Some(_), None) => Some((&[], "params", path.get(1))),
                                (None, _) => None,
                            }
                        }
                        _ => None,
                    };
                if let Some((names, declaration, Some(key))) = keyed
                    && !names.contains(key)
                {
                    self.undeclared(at, key.as_str(), declaration);
                }
            }
            Ref::Input(path) => {
                let name = path.iter().map(Key::as_str).collect::<Vec<_>>().join(".");
                let declared =
                    InputName::new(&name).is_ok_and(|input| decl.inputs.contains(&input));
                if !declared {
                    self.undeclared(at, &name, "inputs");
                }
            }
            _ => {}
        }
    }

    /// A map's key, declared where the map's field names a declaration
    /// (S10); a keyed scalar's key in its declared group, the group's own
    /// undeclared name the one problem where the group is not declared.
    fn declared_key(&mut self, path: &Path, key: &str, suffix: &'static str) {
        let decl = self.decl();
        let Some(declaration) = declaration_of(path.last_field()) else {
            return;
        };
        let Ok(key) = Key::new(key) else {
            return;
        };
        let at = join(
            &path.to_string(),
            &spelled_key(key.as_str(), suffix, self.toml),
        );
        let listed = if declaration.decl == Decl::Params {
            match path.keys().as_slice() {
                [] => decl.params.iter().any(|(group, _)| *group == key),
                [group, ..] => decl
                    .params
                    .iter()
                    .find(|(held, _)| held.as_str() == *group)
                    .is_none_or(|(_, params)| params.keys.contains(&key)),
            }
        } else {
            declaration.decl.keys(decl).contains(&&key)
        };
        if !listed {
            self.undeclared(&at, key.as_str(), declaration.name());
        }
    }

    /// A stated keyed scalar's unit, its declaration's (S4.6, S10).
    fn scalar_unit(&mut self, path: &Path, scalar: &Scalar) {
        if let Some(unit) = self.declared_unit(path) {
            self.unit(&path.to_string(), scalar, unit);
        }
    }

    /// The declared unit of the keyed scalar at `path`: a `params` group's,
    /// a material parameter's, or a keyed transition or presentation kind's
    /// parameter's (S2.7, S4.6, S10).
    fn declared_unit(&self, path: &Path) -> Option<ScalarUnit> {
        let decl = self.decl();
        match (path.last_field(), path.keys().as_slice()) {
            (Some(("Tokens", "params")), [group, _]) => group_unit(decl, group),
            (Some(("Material", "params")), [key]) => decl
                .material_params
                .iter()
                .find(|(held, _)| held.as_str() == *key)
                .map(|(_, unit)| *unit),
            (Some(("TransitionKind" | "PresentationKind", "params")), [key]) => self
                .keyed_kind?
                .iter()
                .find(|(held, _)| held.as_str() == *key)
                .map(|(_, unit)| *unit),
            _ => None,
        }
    }

    /// A stated keyed scalar against its declared unit: a plain number, in
    /// that unit (S10, S11: TOML's only spelling), or one of that unit; a
    /// fraction's within `0..=1`; a number that is not finite is that one
    /// problem only.
    fn unit(&mut self, at: &str, scalar: &Scalar, unit: ScalarUnit) {
        let fits = matches!(
            (scalar, unit),
            (Scalar::Number(_), _)
                | (Scalar::Px(_), ScalarUnit::Px)
                | (Scalar::Ms(_), ScalarUnit::Ms)
                | (Scalar::Deg(_), ScalarUnit::Deg)
                | (Scalar::Fraction(_), ScalarUnit::Fraction)
        );
        if !fits {
            let unit = unit_name(unit);
            self.problem(at, ProblemKind::UnitMismatch { unit });
        } else if let (Scalar::Number(n), ScalarUnit::Fraction) = (scalar, unit)
            && n.is_finite()
            && !(0.0..=1.0).contains(n)
        {
            self.problem(at, ProblemKind::OutOfRange { expected: "0..=1" });
        }
    }

    /// A keyed kind's parameters (S10), where the kind is declared (its key
    /// is checked as the walk reaches it): each declared, a stated one of
    /// its declared unit.
    fn keyed_params(
        &mut self,
        path: &Path,
        kinds: &'a [(Key, Vec<(Key, ScalarUnit)>)],
        declaration: &str,
        key: &Key,
        params: &[(Key, Value<Scalar>)],
    ) {
        let units = kinds
            .iter()
            .find(|(held, _)| held == key)
            .map(|(_, units)| units);
        self.keyed_kind = units.map(Vec::as_slice);
        let Some(units) = units else {
            return;
        };
        let at = path.join("keyed.params");
        for (param, value) in params {
            let param_at = join(&at, &spelled_key(param.as_str(), value.suffix(), self.toml));
            match units.iter().find(|(held, _)| held == param) {
                None => self.undeclared(&param_at, param.as_str(), declaration),
                Some((_, unit)) => {
                    if let Value::Stated(scalar) = value {
                        self.unit(&param_at, scalar, *unit);
                    }
                }
            }
        }
    }

    /// A `Key` leaf, declared where its field or variant names a declaration
    /// (S10); a widget value's key, listed by the widget's `keys` entry for
    /// the value, every key refused where the widget lists none. A
    /// `WinUiAnimatedIcon` engine's `visual` names an entry of WinUI's
    /// animated visuals, no key a value holds, a token staged with the
    /// motion unit, which brings the check against the map (S2.7, S6.1,
    /// S12).
    fn declared_leaf(&mut self, path: &Path, key: &Key) {
        if path.slot() == Some(("TransitionEngine", "visual")) {
            return;
        }
        let decl = self.decl();
        let at = path.to_string();
        if let Some(declaration) = declaration_of(path.slot()) {
            if !declaration.decl.keys(decl).contains(&key) {
                let name = if declaration.keyed {
                    format!("@{key}")
                } else {
                    key.to_string()
                };
                self.undeclared(&at, &name, declaration.name());
            }
            return;
        }
        if let (Region::Values(widget), Some((_, value))) = (self.region, path.first_field()) {
            let listed: Vec<Vec<Key>> = self
                .widget_decls(widget)
                .iter()
                .flat_map(|decl| decl.keys.iter().flatten())
                .filter(|(held, _)| *held == value)
                .map(|(_, keys)| keys.clone())
                .collect();
            if !listed.iter().any(|keys| keys.contains(key)) {
                let declaration = self.declaration(widget, &format!("keys.{value}"));
                self.undeclared(&at, key.as_str(), &declaration);
            }
        }
    }

    /// A transition's `properties` (S2.7, S12): a list naming no group
    /// animates nothing.
    fn properties(&mut self, properties: Option<&[PropertyGroup]>, path: &Path) {
        if properties.is_some_and(<[PropertyGroup]>::is_empty) {
            self.problem(&path.join("properties"), ProblemKind::NoProperties);
        }
    }

    /// A timing's own constraints (S2.7); a number that is not finite is
    /// reported where the walk reaches it, as that one problem.
    fn timing(&mut self, timing: &Timing, path: &Path) {
        let mut checks: Vec<(String, bool, &'static str)> = Vec::new();
        let range = |x: f32, ok: bool| !x.is_finite() || ok;
        match timing {
            Timing::CubicBezier(x1, _, x2, _) => {
                for (index, x) in [(0, x1), (2, x2)] {
                    let at = format!("{}[{index}]", path.join("cubic_bezier"));
                    checks.push((at, range(*x, (0.0..=1.0).contains(x)), "0..=1"));
                }
            }
            Timing::Steps { count, jump } => {
                let at = path.join("steps.count");
                if *jump == StepJump::JumpNone {
                    checks.push((at, *count >= 2, "at least two steps under `jump-none`"));
                } else {
                    checks.push((at, *count >= 1, "at least one step"));
                }
            }
            Timing::Spring {
                mass,
                stiffness,
                damping_ratio,
                rest_epsilon,
                ..
            } => {
                let at = |field: &str| path.join(&format!("spring.{field}"));
                checks.push((at("mass"), range(*mass, *mass > 0.0), "above zero"));
                let stiff = range(*stiffness, *stiffness > 0.0);
                checks.push((at("stiffness"), stiff, "above zero"));
                let ratio = range(*damping_ratio, *damping_ratio >= 0.0);
                checks.push((at("damping_ratio"), ratio, "at least zero"));
                let epsilon = range(*rest_epsilon, *rest_epsilon > 0.0);
                checks.push((at("rest_epsilon"), epsilon, "above zero"));
            }
            Timing::Linear | Timing::Platform(_) => {}
        }
        for (at, ok, expected) in checks {
            if !ok {
                self.problem(&at, ProblemKind::OutOfRange { expected });
            }
        }
    }

    /// A loop's keyframes' `at` increase (S2.7), in a widget's values each
    /// keyframe names its part, and each keyframe's patch sets only fields
    /// the catalogue entry of the part it names lists, or of the part the
    /// loop is on where it names none (S2.7, S2.11).
    fn keyframes(&mut self, keyframes: &[Keyframe], path: &Path) {
        let at = path.join("keyframes");
        if !increasing(keyframes) {
            self.problem(&at, ProblemKind::KeyframesNotIncreasing);
        }
        for (index, keyframe) in keyframes.iter().enumerate() {
            if keyframe.part.is_none() && matches!(self.region, Region::Values(_)) {
                let at = format!("{at}[{index}].part");
                self.problem(&at, ProblemKind::KeyframeWithoutPart);
            }
            let part = match (&keyframe.part, &self.region) {
                (Some(PartRef::Typed(part)), _) => {
                    part_info(*part).map(|info| (widget_of(*part), info))
                }
                (None, Region::Part(widget, _, info, _)) => Some((*widget, *info)),
                (Some(PartRef::Component { .. }), _)
                | (None, Region::Roles | Region::Tokens | Region::Values(_)) => None,
            };
            let patch_at = format!("{at}[{index}].patch");
            if let Some((widget, info)) = part {
                self.part_fields(&keyframe.patch, widget, info, &patch_at);
            }
        }
    }

    /// A `Keyframes` transition's keyframes (S2.7, S12; ADD-A134): at least
    /// one, their `at` increasing, none naming a part, as each styles the
    /// part the transition times, and each patch setting only fields its
    /// part has. What a part's transition's keyframes may state is
    /// [`Checker::keyframe_groups`]'.
    fn transition_keyframes(&mut self, keyframes: &[Keyframe], path: &Path) {
        let at = path.join("keyframes.keyframes");
        if keyframes.is_empty() {
            self.problem(&at, ProblemKind::NoKeyframe);
        }
        if !increasing(keyframes) {
            self.problem(&at, ProblemKind::KeyframesNotIncreasing);
        }
        for (index, keyframe) in keyframes.iter().enumerate() {
            let keyframe_at = format!("{at}[{index}]");
            if keyframe.part.is_some() {
                let at = format!("{keyframe_at}.part");
                self.problem(&at, ProblemKind::KeyframeNamesPart);
            }
            if let Region::Part(widget, _, info, _) = self.region {
                let patch_at = format!("{keyframe_at}.patch");
                self.part_fields(&keyframe.patch, widget, info, &patch_at);
            }
        }
    }

    /// A part's `Keyframes` transition's keyframes state only fields its
    /// groups time (S2.7, S12; ADD-A134; [`times`]).
    fn keyframe_groups(
        &mut self,
        properties: &[PropertyGroup],
        kind: &TransitionKind,
        path: &Path,
    ) {
        let TransitionKind::Keyframes { keyframes } = kind else {
            return;
        };
        let at = path.join("kind.keyframes.keyframes");
        for (index, keyframe) in keyframes.iter().enumerate() {
            for place in stated_places(&keyframe.patch) {
                if !properties.iter().any(|group| times(*group, &place)) {
                    let at = format!("{at}[{index}].patch.{place}");
                    self.problem(&at, ProblemKind::KeyframeOutsideGroups);
                }
            }
        }
    }

    /// Axes and features that duplicate a field's route (S2.8).
    fn routes(
        &mut self,
        axes: &[(AxisTag, AxisValue)],
        features: &[(FeatureTag, u32)],
        routed: &[&str],
        path: &Path,
    ) {
        for (tag, _) in axes {
            let tag = tag.to_string();
            if routed.contains(&tag.as_str()) {
                self.problem(&path.join("axes"), ProblemKind::RouteDuplicated { tag });
            }
        }
        for (tag, _) in features {
            let tag = tag.to_string();
            if tag == "smcp" {
                self.problem(&path.join("features"), ProblemKind::RouteDuplicated { tag });
            }
        }
    }

    /// A transition's `from` (S2.7, S12): no empty list, no flag twice; a
    /// keyed flag's declaration is its own node's check.
    fn from(&mut self, from: Option<&[StateFlag]>, path: &Path) {
        let Some(from) = from else {
            return;
        };
        let at = path.join("from");
        if from.is_empty() {
            self.problem(&at, ProblemKind::FromNoState);
        }
        for (index, flag) in from.iter().enumerate() {
            if from.iter().take(index).any(|earlier| earlier == flag) {
                let name = flag.name();
                self.problem(&format!("{at}[{index}]"), ProblemKind::ListedTwice { name });
            }
        }
    }

    /// A `StateLayer` transition times the part's `interaction`: refused on
    /// a part whose fields lack it (S2.12, S12).
    fn state_layer(&mut self, properties: &[PropertyGroup], path: &Path) {
        if let Region::Part(_, _, info, _) = self.region
            && properties.contains(&PropertyGroup::StateLayer)
            && !info.fields.iter().any(|field| field.path == "interaction")
        {
            self.problem(
                &path.join("properties"),
                ProblemKind::StateLayerWithoutInteraction,
            );
        }
    }

    /// An indication box names a part of the ripple's own widget (S2.12,
    /// S12): a `<widget>.<part>` naming another is refused.
    fn indication_box(&mut self, part: Option<&PartRef>, path: &Path) {
        let (Region::Part(widget, ..), Some(part)) = (self.region, part) else {
            return;
        };
        let own = match part {
            PartRef::Typed(typed) => widget_of(*typed) == widget,
            PartRef::Component { .. } => false,
        };
        if !own {
            let part = part_ref_name(part);
            self.problem(
                &path.join("part"),
                ProblemKind::IndicationBoxOtherWidget { part },
            );
        }
    }

    /// S12's checks of a transition's engine against the transition (S2.7;
    /// ADD-A126; [`takes`], [`test_holds`]), on a whole transition of a
    /// keyed list; and its Breeze test's own ([`Checker::breeze_test`]),
    /// wherever an engine is stated. A delay written as an expression the
    /// checks over combinations read where it resolves, a delay being
    /// refused by its value. A `FocusMove` kind is refused on its own.
    fn engine(&mut self, engine: &TransitionEngine, whole: Option<&Transition>, path: &Path) {
        let toml = self.toml;
        let field = |key: &str, suffix: &str| path.join(&spelled_key(key, suffix, toml));
        let name = engine_name(engine);
        let at = format!("{}.{name}", field("engine", ""));
        let test = match engine {
            TransitionEngine::Breeze(test) | TransitionEngine::BreezeTabBar(test) => {
                self.breeze_test(test, &at);
                Some(test)
            }
            TransitionEngine::GtkCss
            | TransitionEngine::AdwAnimation { .. }
            | TransitionEngine::QtQuick
            | TransitionEngine::ComposeAnimatable
            | TransitionEngine::ComposeTransition
            | TransitionEngine::ComposeHeld
            | TransitionEngine::WinUi
            | TransitionEngine::WinUiAnimatedIcon { .. }
            | TransitionEngine::GtkSwitch
            | TransitionEngine::CssAnimation => None,
        };
        let Some(transition) = whole else {
            return;
        };
        let part = match self.region {
            Region::Part(_, part, ..) => Some(part),
            Region::Roles | Region::Tokens | Region::Values(_) => None,
        };
        let geometry = transition.properties == [PropertyGroup::Geometry];
        let mut reads = vec![
            (field("engine", ""), Read::Place { geometry, part }),
            (
                field("duration", "_ms"),
                Read::Duration {
                    absent: transition.duration == Value::Absent,
                },
            ),
            (
                field("timing", ""),
                Read::Timing {
                    absent: transition.timing == Value::Absent,
                },
            ),
        ];
        // A delay written as an expression is read where it resolves.
        if matches!(transition.delay, Value::Stated(Ms(delay)) if delay != 0.0) {
            reads.push((field("delay", "_ms"), Read::Delay));
        }
        reads.push((field("kind", ""), Read::Kind(kind_name(&transition.kind))));
        for (at, read) in reads {
            if let Some(takes) = takes(name, read) {
                let kind = ProblemKind::EngineMismatch {
                    engine: name,
                    takes,
                };
                self.problem(&at, kind);
            }
        }
        if let Some(test) = test {
            let zero = transition.duration == Value::Stated(Ms(0.0));
            for (list, expected) in test_holds(test, zero) {
                let at = list.map_or_else(|| at.clone(), |list| format!("{at}.{list}"));
                self.problem(&at, ProblemKind::BreezeTestInvalid { expected });
            }
        }
    }

    /// A Breeze engine's test at `at`, as it reads alone (S2.7, S12;
    /// ADD-A126): no flag twice in a list, none in both `any` and `none`;
    /// only the window's conditions (S7.1's group by the connector, whose
    /// toggled style the connector can resolve). What it must be against its
    /// transition is [`test_holds`]'. A keyed flag or condition is checked
    /// against the declarations as the walk reaches it.
    fn breeze_test(&mut self, test: &BreezeTest, at: &str) {
        let invalid = |expected| ProblemKind::BreezeTestInvalid { expected };
        for (list, flags) in [("any", &test.any), ("none", &test.none)] {
            for (index, flag) in flags.iter().enumerate() {
                if flags.iter().take(index).any(|earlier| earlier == flag) {
                    let name = flag.name();
                    let at = format!("{at}.{list}[{index}]");
                    self.problem(&at, ProblemKind::ListedTwice { name });
                }
            }
        }
        for (index, flag) in test.none.iter().enumerate() {
            if test.any.contains(flag) {
                let expected = "names no flag in both `any` and `none`";
                self.problem(&format!("{at}.none[{index}]"), invalid(expected));
            }
        }
        for (index, condition) in test.conditions.iter().enumerate() {
            if !window_condition(condition) {
                let expected = "names the window's conditions only";
                self.problem(&format!("{at}.conditions[{index}]"), invalid(expected));
            }
        }
    }

    /// A `KirigamiSmoothstep` shadow layer's `spread` a stated zero and its
    /// `inset` false (S2.5, S12; ADD-A134): Kirigami's shadow has no spread
    /// and is never inset. A layer is stated whole, so its own fields are
    /// all the check reads; a spread written as an expression can be other
    /// than zero, and is refused.
    fn smoothstep(
        &mut self,
        falloff: Option<&ShadowFalloff>,
        spread: Option<&Value<Px>>,
        inset: Option<bool>,
        path: &Path,
    ) {
        if falloff != Some(&ShadowFalloff::KirigamiSmoothstep) {
            return;
        }
        let toml = self.toml;
        let field = |key: &str, suffix: &str| path.join(&spelled_key(key, suffix, toml));
        if spread.is_some_and(|spread| *spread != Value::Stated(Px(0.0))) {
            self.problem(
                &field("spread", "_px"),
                ProblemKind::SmoothstepSpreadOrInset,
            );
        }
        if inset == Some(true) {
            self.problem(&field("inset", ""), ProblemKind::SmoothstepSpreadOrInset);
        }
    }

    /// A font's stated weight, the CSS and OpenType weight, 1 to 1000
    /// (S2.8).
    fn weight(&mut self, weight: Option<&Value<u16>>, path: &Path) {
        if let Some(Value::Stated(weight)) = weight
            && !(1..=1000).contains(weight)
        {
            let expected = "1..=1000";
            self.problem(&path.join("weight"), ProblemKind::OutOfRange { expected });
        }
    }
}

impl Visit for Checker<'_> {
    /// A block's or a leaf's own checks.
    fn check(&mut self, path: &Path, node: Node<'_>) {
        let at = path.to_string();
        let decl = self.decl();
        match node {
            Node::Paint(paint) => match paint.layers.first() {
                None => self.problem(&at, ProblemKind::NoLayer),
                Some(bottom) if bottom.blend == BlendMode::Mask => {
                    self.problem(&path.join("layers[0].blend"), ProblemKind::MaskAtBottom);
                }
                Some(_) => {}
            },
            Node::PaintPatch(patch) => {
                if let PaintPatch::Append(_) = patch
                    && self.in_bases
                    && !path.within("Keyframe", "patch")
                {
                    self.problem(&path.join("append"), ProblemKind::AppendInBases);
                }
            }
            Node::PaintKind(kind) => {
                let stops = match kind {
                    PaintKind::Linear { stops, .. }
                    | PaintKind::Radial { stops, .. }
                    | PaintKind::Conic { stops, .. } => Some(stops.len()),
                    PaintKind::Color(_) | PaintKind::Pattern { .. } | PaintKind::Material(_) => {
                        None
                    }
                };
                if stops.is_some_and(|stops| stops < 2) {
                    self.problem(&at, ProblemKind::TooFewStops);
                }
            }
            Node::Shadow(shadow) => {
                if let Shadow::Layers(layers) = shadow
                    && layers.is_empty()
                {
                    self.problem(&at, ProblemKind::NoLayer);
                }
            }
            Node::ShadowLayer(layer) => {
                let falloff = Some(&layer.falloff);
                self.smoothstep(falloff, Some(&layer.spread), Some(layer.inset), path);
            }
            Node::ShadowLayerPatch(patch) => {
                let (falloff, spread) = (patch.falloff.as_ref(), patch.spread.as_ref());
                self.smoothstep(falloff, spread, patch.inset, path);
            }
            Node::Transition(transition) => {
                self.properties(Some(&transition.properties), path);
                self.from(transition.from.as_deref(), path);
                self.state_layer(&transition.properties, path);
                self.engine(&transition.engine, Some(transition), path);
                self.keyframe_groups(&transition.properties, &transition.kind, path);
            }
            Node::StateFlag(flag) => match self.region {
                Region::Part(widget, ..) | Region::Values(widget) => self.flag(widget, flag, &at),
                // A token's flag (a transition's `from`) is matched against
                // the element of any widget that reads it: every widget
                // declares a keyed one (S10, S12).
                Region::Tokens => {
                    for widget in registry::widgets().iter().map(|info| info.kind) {
                        self.flag(widget, flag, &at);
                    }
                }
                Region::Roles => {}
            },
            Node::TransitionPatch(transition) => {
                self.properties(transition.properties.as_deref(), path);
                self.from(transition.from.as_ref().and_then(Option::as_deref), path);
                if let Some(properties) = &transition.properties {
                    self.state_layer(properties, path);
                }
                if let Some(engine) = &transition.engine {
                    self.engine(engine, None, path);
                }
                if let (Some(properties), Some(kind)) = (&transition.properties, &transition.kind) {
                    self.keyframe_groups(properties, kind, path);
                }
            }
            Node::TransitionKind(kind) => match kind {
                TransitionKind::Keyed { key, params } => {
                    let kinds = &decl.transition_kinds;
                    self.keyed_params(path, kinds, "transition_kinds", key, params);
                }
                TransitionKind::FocusMove => self.problem(&at, ProblemKind::FocusMoveNotRun),
                TransitionKind::Keyframes { keyframes } => {
                    self.transition_keyframes(keyframes, path);
                }
                TransitionKind::Interpolate => {}
            },
            Node::PresentationKind(kind) => {
                if let PresentationKind::Keyed { key, params } = kind {
                    let kinds = &decl.presentation_kinds;
                    self.keyed_params(path, kinds, "presentation_kinds", key, params);
                }
            }
            Node::Timing(timing) => self.timing(timing, path),
            Node::Loop(looped) => self.keyframes(&looped.keyframes, path),
            Node::LoopPatch(looped) => {
                if let Some(keyframes) = &looped.keyframes {
                    self.keyframes(keyframes, path);
                }
            }
            Node::Radius(radius) => {
                if let Radius::FollowPart = radius
                    && !path.within("FocusIndicator", "corners")
                {
                    self.problem(&at, ProblemKind::FollowPartOutsideFocus);
                }
            }
            Node::Font(font) => {
                let (axes, features) = (stated_map(&font.axes), stated_map(&font.features));
                self.routes(axes, features, &["wght", "wdth"], path);
                self.weight(Some(&font.weight), path);
            }
            Node::FontPatch(font) => {
                let axes = font.axes.as_ref().map_or(&[][..], stated_map);
                let features = font.features.as_ref().map_or(&[][..], stated_map);
                self.routes(axes, features, &["wght", "wdth"], path);
                self.weight(font.weight.as_ref(), path);
            }
            Node::IconStyle(style) => self.routes(&style.axes, &[], &["wght"], path),
            Node::IconStylePatch(style) => {
                let axes = style.axes.as_deref().unwrap_or_default();
                self.routes(axes, &[], &["wght"], path);
            }
            Node::Scalar(scalar) => self.scalar_unit(path, scalar),
            Node::Key(key) => self.declared_leaf(path, key),
            Node::Condition(condition) => self.condition(condition, &at, false),
            // A fade's nested element (S2.8): its variant, walked after its
            // widget, one the widget declares, of an enabled unit (S10, S12).
            Node::Widget(widget) => {
                self.fade_widget = match widget {
                    WidgetRef::Typed(kind) if path.slot() == Some(("FadeUnder", "widget")) => {
                        Some(*kind)
                    }
                    WidgetRef::Typed(_) | WidgetRef::Component(_) => None,
                };
            }
            Node::Variant(variant) => {
                if path.slot() == Some(("FadeUnder", "variant"))
                    && let Some(widget) = self.fade_widget.take()
                {
                    self.variant(widget, variant, &at);
                }
            }
            Node::SurfaceSet(set) => self.surface_set(set, &at),
            Node::IndicationBox(indication) => self.indication_box(Some(&indication.part), path),
            Node::IndicationBoxPatch(patch) => self.indication_box(patch.part.as_ref(), path),
        }
    }

    fn value(&mut self, path: &Path, form: Form<'_>, expr: ExprForm, ty: Option<Ty>) {
        let marks = path.marks();
        let absent_param = path.last_field() == Some(("Tokens", "params"))
            && path
                .keys()
                .first()
                .is_some_and(|group| ABSENT_PARAM_GROUPS.contains(group));
        let bare_entry = matches!(
            path.map_entry(),
            Some(
                ("TextTokens", "keyed") | ("IconSizeTokens", "keyed") | ("ColorRoles", "surfaces")
            )
        );
        if bare_entry && !matches!(form, Form::Stated) {
            self.problem(&path.to_string(), ProblemKind::NotABlock);
            return;
        }
        match form {
            Form::Absent if !(marks.may_be_absent || absent_param) => {
                self.problem(&path.to_string(), ProblemKind::AbsentNotAllowed);
            }
            Form::Auto if !marks.auto => {
                self.problem(&path.to_string(), ProblemKind::AutoNotAllowed);
            }
            Form::Expr(e) => self.expression(path, e, expr, ty),
            Form::Absent | Form::Auto | Form::Stated => {}
        }
    }

    fn number(&mut self, path: &Path, n: f32, number: Number) {
        let at = path.to_string();
        if !n.is_finite() {
            self.problem(&at, ProblemKind::NotFinite);
            return;
        }
        let signed = path.marks().signed;
        let field = path.last_field();
        let expected = match (number, field) {
            (Number::Fraction, _) if !(0.0..=1.0).contains(&n) => Some("0..=1"),
            (Number::Length | Number::FontPt | Number::FontPx, _) if n < 0.0 && !signed => {
                Some("at least zero")
            }
            (Number::Plain, Some(("Material", "saturation") | ("Glyph", "draw_extent")))
                if n < 0.0 =>
            {
                Some("at least zero")
            }
            (Number::Plain, Some(("TextTokens", "font_dpi"))) if n <= 0.0 => Some("above zero"),
            _ => None,
        };
        if let Some(expected) = expected {
            self.problem(&at, ProblemKind::OutOfRange { expected });
        }
    }

    fn duplicate(&mut self, path: &Path, key: &str, suffix: &'static str) {
        let spelled = spelled_key(key, suffix, self.toml);
        let at = join(&path.to_string(), &spelled);
        self.problem(&at, ProblemKind::DuplicateKey { other: spelled });
    }

    fn map_key(&mut self, path: &Path, key: &str, suffix: &'static str) {
        self.declared_key(path, key, suffix);
    }

    /// In a preset's own bases, the bottom of the stack, every field a
    /// patch has is stated, a part's style every field its catalogue entry
    /// lists (S2.2: a field no layer states is missing); a keyframe's
    /// patch sets only what it animates.
    fn patch(&mut self, path: &Path, all: &'static [&'static str], set: &[&'static str]) {
        let keyframe = path.within("Keyframe", "patch");
        if !(self.in_bases && self.cx.kind == LayerKind::Preset) || keyframe || self.latent {
            return;
        }
        let missing: Vec<&str> = match path.part_fields() {
            Some(fields) => fields
                .iter()
                .map(|field| field.path)
                .filter(|field| !set.contains(field))
                .collect(),
            None => all
                .iter()
                .copied()
                .filter(|field| !set.contains(field))
                .collect(),
        };
        for field in missing {
            for at in unlisted(&path.join(field), &self.exempt) {
                self.problem(&at, ProblemKind::Missing);
            }
        }
    }
}

/// The paths a preset's completeness check reports missing where its bases
/// leave the field `at` unstated, beside its listed paths `exempt` (each as
/// [`listed_path`] gives it), which the layers over it state (S2.2, S12,
/// S17, ADD-A131): none where `at` is listed; `at` itself where no listed
/// path lies below it; else, a block being stated where each of its fields
/// is, each field of its struct block (a map's entry's included) that is
/// so missing, and none for any other value a listed path lies below (a
/// map named whole, a list, an enum block's variant), the layers stating
/// what they list there.
pub(super) fn unlisted(at: &str, exempt: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let field = crate::toml::read::listed_field(at);
    unlisted_below(at, field, exempt, &mut out);
    out
}

/// The paths a preset's completeness check reports missing where its bases
/// state nothing of the part `part` at `at`, beside its listed paths
/// `exempt` (S6.3, S12, S17, ADD-A131): the part itself where no listed
/// path lies below it, its presence listed or not; else, a part no patch
/// makes absent being drawn with its fields, each field [`unlisted`] gives.
fn unlisted_part(at: &str, part: &PartInfo, exempt: &[String]) -> Vec<String> {
    let below = exempt.iter().any(|path| {
        path.strip_prefix(at)
            .is_some_and(|rest| rest.starts_with('.'))
    });
    if !below {
        return vec![at.to_owned()];
    }
    part.fields
        .iter()
        .flat_map(|field| unlisted(&join(at, field.path), exempt))
        .collect()
}

/// [`unlisted`] at `at`, the field `field` names `levels` levels of its
/// map's keys below it (`None`: a field the registry does not find).
fn unlisted_below(
    at: &str,
    field: Option<(&'static FieldInfo, usize)>,
    exempt: &[String],
    out: &mut Vec<String>,
) {
    let listed = listed_path(at);
    if exempt.contains(&listed) {
        return;
    }
    let below = exempt.iter().any(|path| {
        path.strip_prefix(listed.as_str())
            .is_some_and(|rest| rest.starts_with('.'))
    });
    if !below {
        out.push(at.to_owned());
        return;
    }
    if let Some((
        FieldInfo {
            ty:
                registry::FieldType::Block {
                    ty,
                    form: registry::BlockForm::Struct,
                    fields,
                    ..
                },
            ..
        },
        levels,
    )) = field
        && registry::key_levels(ty) == levels
        && !entry_type(ty).starts_with("Vec<")
    {
        for field in *fields {
            unlisted_below(&join(at, field.path), Some((field, 0)), exempt, out);
        }
    }
}

/// The type a map's entry holds, as the model writes it (`TextStyle)>` of
/// `Vec<(Key,TextStyle)>`, the closing brackets kept), or the type itself
/// where it is no map.
fn entry_type(ty: &str) -> &str {
    let mut rest = ty;
    loop {
        if let Some(inner) = rest.strip_prefix("SurfaceMap<") {
            rest = inner;
        } else if let Some(inner) = rest.strip_prefix("Vec<(") {
            rest = inner.split_once(',').map_or("", |(_, value)| value);
        } else {
            return rest;
        }
    }
}

/// The entries a `Value` of a map states; none for a reference, `Absent` or
/// `Auto`, whose entries the stack resolves (S6.1).
fn stated_map<K, V>(value: &Value<Vec<(K, V)>>) -> &[(K, V)] {
    match value {
        Value::Stated(map) => map,
        Value::Expr(_) | Value::Absent | Value::Auto => &[],
    }
}

/// What typing knows of the model where an expression is stated (S2.2):
/// the registry's types of the fields, tokens and roles a reference names,
/// and the preset's declarations.
struct Types<'a> {
    decl: &'a Declarations,
    region: Region,
}

/// What typing knows of the model where an expression of `target`'s is
/// evaluated: the same types the check typed it with (S2.2, S12).
pub(crate) fn type_env(decl: &Declarations, target: TypeTarget) -> impl TypeEnv + '_ {
    let region = match target {
        TypeTarget::Roles => Region::Roles,
        TypeTarget::Tokens => Region::Tokens,
        TypeTarget::Values(widget) => Region::Values(widget),
        TypeTarget::Part(part) => {
            let widget = widget_of(part);
            let parts = registry::widgets()
                .iter()
                .find(|w| w.kind == widget)
                .map_or(&[] as &[PartInfo], |w| w.parts);
            match super::stack::part_info(part) {
                Some(info) => Region::Part(widget, part, info, parts),
                None => Region::Tokens,
            }
        }
    };
    Types { decl, region }
}

/// Where an expression is stated, for [`type_env`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TypeTarget {
    /// The colour roles.
    Roles,
    /// The tokens.
    Tokens,
    /// A typed widget's part.
    Part(TypedPart),
    /// A typed widget's values.
    Values(WidgetKind),
}

impl TypeEnv for Types<'_> {
    fn reference(&self, reference: &Ref) -> Option<Ty> {
        let joined = |path: &[Key]| path.iter().map(Key::as_str).collect::<Vec<_>>().join(".");
        match reference {
            Ref::Role { set, path } => field_type(
                registry::colors(),
                &format!("surfaces.{}.{}", set.name(), joined(path)),
            ),
            Ref::GlobalRole(path) => {
                field_type(registry::colors(), &format!("global.{}", joined(path)))
            }
            Ref::KeyedRole(_) | Ref::Input(_) => Some(Ty::Color),
            Ref::Surface(path) => field_type(<SurfaceRoles as Table>::FIELDS, &joined(path)),
            Ref::Token(path) => match path.as_slice() {
                [params, group, _] if params.as_str() == "params" => {
                    group_unit(self.decl, group.as_str()).map(unit_type)
                }
                _ => field_type(registry::tokens(), &joined(path)),
            },
            Ref::Part(path) if is_corner(path) => match self.region {
                Region::Part(_, _, info, _) => {
                    registry::find(info.fields, &joined(path)).map(|_| Ty::Length)
                }
                Region::Roles | Region::Tokens | Region::Values(_) => None,
            },
            Ref::Part(path) => match self.region {
                Region::Part(_, _, info, _) => field_type(info.fields, &joined(path)),
                Region::Roles | Region::Tokens | Region::Values(_) => None,
            },
            Ref::Widget { part, path } => {
                let widget = match self.region {
                    Region::Part(widget, ..) | Region::Values(widget) => widget,
                    Region::Roles | Region::Tokens => return None,
                };
                let info = registry::widgets()
                    .iter()
                    .find(|info| info.kind == widget)?
                    .part(part.as_str())?;
                match path.as_slice() {
                    [metric] if <FontMetric as Named>::from_name(metric.as_str()).is_some() => {
                        Some(Ty::Length)
                    }
                    _ if is_corner(path)
                        && registry::find(info.fields, &joined(path)).is_some() =>
                    {
                        Some(Ty::Length)
                    }
                    _ => field_type(info.fields, &joined(path)),
                }
            }
            Ref::Values(name) => {
                let widget = match self.region {
                    Region::Part(widget, ..) | Region::Values(widget) => widget,
                    Region::Roles | Region::Tokens => return None,
                };
                let info = registry::widgets()
                    .iter()
                    .find(|info| info.kind == widget)?;
                match (
                    info.values.iter().find(|field| field.path == name.as_str()),
                    field_type(info.values, name.as_str()),
                ) {
                    (Some(_), Some(Ty::Key(set))) if set == KEY_OF_PLACE => {
                        Some(widget_keys(widget, name.as_str()))
                    }
                    (_, ty) => ty,
                }
            }
            Ref::Active(_)
            | Ref::Base
            | Ref::Scheme
            | Ref::FontSize
            | Ref::LineHeight
            | Ref::Element(_)
            | Ref::BodyFontHeight
            | Ref::TextScale => None,
        }
    }

    fn declares(&self, set: &str, key: &Key) -> bool {
        match set.strip_prefix("widget.") {
            Some(rest) => {
                let Some((widget, value)) = rest.split_once(".keys.") else {
                    return false;
                };
                let Some(info) = registry::widget(widget) else {
                    return false;
                };
                self.decl.in_each_flavor(info.kind).iter().any(|decl| {
                    decl.keys
                        .iter()
                        .flatten()
                        .any(|(held, keys)| *held == value && keys.contains(key))
                })
            }
            None => declared(self.decl, set).contains(&key),
        }
    }
}

/// The type a reference to the field at `path` below `fields` has: its
/// registry entry's, a key field's a key of its field's declaration (S2.2,
/// S10); a widget value's key is left to its widget's `keys` entry.
fn field_type(fields: &'static [FieldInfo], path: &str) -> Option<Ty> {
    let info = registry::find_value(fields, path)?;
    match (info.value)() {
        Some(Ty::Key(set)) if set == KEY_OF_PLACE => {
            let (parent, name) = path.rsplit_once('.').unwrap_or(("", path));
            let owner = registry::find(fields, parent).and_then(|parent| match parent.ty {
                registry::FieldType::Block { block, .. }
                | registry::FieldType::Patch { block, .. } => Some(block),
                registry::FieldType::Leaf(_) => None,
            });
            match owner.and_then(|owner| declaration_of_field(owner, name)) {
                Some(declaration) => Some(Ty::Key(Cow::Borrowed(declaration))),
                None => Some(Ty::Key(set)),
            }
        }
        ty => ty,
    }
}

/// Whether a part's path names one of its corners, `corners.<corner>` or
/// `focus.corners.<corner>`: a corner operand, a length, which the query
/// resolves (S8: a `Full` corner to half the box's smaller side).
fn is_corner(path: &[Key]) -> bool {
    let names: Vec<&str> = path.iter().map(Key::as_str).collect();
    let corner = |name: &str| {
        matches!(
            name,
            "top_left" | "top_right" | "bottom_right" | "bottom_left"
        )
    };
    match names.as_slice() {
        ["corners", name] | ["focus", "corners", name] => corner(name),
        _ => false,
    }
}

/// The type of a key the widget's `keys` entry for `value` lists (S10).
fn widget_keys(widget: WidgetKind, value: &str) -> Ty {
    Ty::Key(Cow::Owned(format!(
        "widget.{}.keys.{value}",
        widget_name(widget)
    )))
}

/// A `params` group's declared unit.
fn group_unit(decl: &Declarations, group: &str) -> Option<ScalarUnit> {
    decl.params
        .iter()
        .find(|(held, _)| held.as_str() == group)
        .map(|(_, group)| group.unit)
}

/// The type of a keyed scalar of a declared unit (S2.2, S4.6).
fn unit_type(unit: ScalarUnit) -> Ty {
    match unit {
        ScalarUnit::Px => Ty::Length,
        ScalarUnit::Ms => Ty::Duration,
        ScalarUnit::Deg => Ty::Angle,
        ScalarUnit::Fraction | ScalarUnit::Number => Ty::Number,
    }
}

/// Every structural problem of a preset's declarations (S10, S12): an
/// `exclusive_variants` group naming a variant twice; a section's recency
/// sets; its sizes and default sizes ([`sizes`]). `synthetic`: the
/// synthetic route's preset, whose pending default sizes are waived (S17).
pub(super) fn declarations(decl: &Declarations, synthetic: bool) -> Vec<Problem> {
    // Each section's entries against the declarations in force where they
    // apply: the widget's own under the default flavour, a flavour's
    // section under that flavour, each key it leaves out the widget's own
    // (S10).
    let own = decl.widgets.iter().map(|(widget, section)| {
        let at = format!("declare.widget.{}", widget_name(*widget));
        (
            at,
            section,
            decl.in_default_flavor(*widget),
            section.clone(),
            *widget,
        )
    });
    let flavors = decl.flavor.iter().flat_map(|(flavor, section)| {
        section.widgets.iter().map(move |(widget, section)| {
            let at = format!("declare.flavor.{flavor}.widget.{}", widget_name(*widget));
            let in_force = decl.in_flavor(*widget, flavor);
            (at, section, in_force.clone(), in_force, *widget)
        })
    });
    let mut problems = Vec::new();
    for (at, section, in_force, view, widget) in own.chain(flavors) {
        let own_at = format!("declare.widget.{}", widget_name(widget));
        problems.extend(sizes(&at, &own_at, section, &view, synthetic));
        for (group_index, group) in section.exclusive_variants.iter().flatten().enumerate() {
            for (index, variant) in group.iter().enumerate() {
                if group.iter().take(index).any(|earlier| earlier == variant) {
                    problems.push(Problem::new(
                        &format!("{at}.exclusive_variants[{group_index}][{index}]"),
                        ProblemKind::ListedTwice {
                            name: variant_name(variant),
                        },
                    ));
                }
            }
        }
        // `later.variants` lists declared variants (S10's *Later names*,
        // S12; ADD-A147): each one the variants in force declare, once.
        for (index, variant) in section.later.iter().flatten().enumerate() {
            let place = format!("{at}.later.variants[{index}]");
            let name = variant_name(variant);
            let twice = section
                .later
                .iter()
                .flatten()
                .take(index)
                .any(|held| held == variant);
            if twice {
                problems.push(Problem::new(&place, ProblemKind::ListedTwice { name }));
            } else if !view.variants.iter().flatten().any(|held| held == variant) {
                let declaration = if section.variants.is_some() {
                    format!("{at}.variants")
                } else {
                    format!("{own_at}.variants")
                };
                problems.push(Problem::new(
                    &place,
                    ProblemKind::Undeclared { name, declaration },
                ));
            }
        }
        problems.extend(recency(&at, section, &in_force));
        // A section replacing the flags, the variants or the sizes and not
        // the recency sets or the default sizes keeps the widget's own,
        // checked against what it replaces (S10).
        if section.recency.is_none() && section.flags.is_some() {
            for (_, flags) in in_force.recency.iter().flatten() {
                for flag in flags {
                    let declared = !matches!(flag, StateFlag::Widget(_))
                        || in_force.flags.iter().flatten().any(|held| held == flag);
                    if !declared {
                        let declaration = format!("{at}.flags");
                        let kind = ProblemKind::Undeclared {
                            name: flag.name(),
                            declaration: declaration.clone(),
                        };
                        problems.push(Problem::new(&declaration, kind));
                    }
                }
            }
        }
    }
    problems
}

/// A section's sizes and default sizes (S10, S12) as `view` holds them in
/// force: the widget's own section, or a flavour's section over it, which
/// is checked where it states a key of them, each key it leaves out the
/// widget's own (`own_at`), a problem of one it leaves out reported at the
/// first it states. Refused: a `default_size` missing where sizes are
/// taken, or not one of them; a `sizes_by_variant` entry naming an
/// undeclared variant, a variant an earlier entry names, a default that is
/// not one of its sizes, or none where it leaves the widget's out; a
/// `sizes_by_context` entry's selector stating what S12 refuses there,
/// and, for each variant entry and for none, an element taking sizes
/// outside the context and none inside it, or a default in force there
/// that is not among them. A pending default is the missing default size,
/// but where `synthetic`, which waives every check reading it (S10's *A
/// pending default*, S17).
fn sizes(
    at: &str,
    own_at: &str,
    section: &WidgetDecl,
    view: &WidgetDecl,
    synthetic: bool,
) -> Vec<Problem> {
    let states = |key: &str| match key {
        "sizes" => section.sizes.is_some(),
        "default_size" => section.default_size.is_some(),
        "variants" => section.variants.is_some(),
        "sizes_by_variant" => section.sizes_by_variant.is_some(),
        "sizes_by_context" => section.sizes_by_context.is_some(),
        _ => false,
    };
    let keys = [
        "sizes",
        "default_size",
        "variants",
        "sizes_by_variant",
        "sizes_by_context",
    ];
    let Some(changed) = keys.into_iter().find(|key| states(key)) else {
        return Vec::new();
    };
    let place = |key: &str, rest: &str| {
        if states(key) {
            format!("{at}.{key}{rest}")
        } else {
            format!("{at}.{changed}")
        }
    };
    let mut out: Vec<Problem> = Vec::new();
    let mut push = |path: String, kind: ProblemKind| {
        let problem = Problem::new(&path, kind);
        if !out.contains(&problem) {
            out.push(problem);
        }
    };
    let missing = |default: Option<&DefaultSize>| match default {
        None => true,
        Some(DefaultSize::Pending(_)) => !synthetic,
        Some(DefaultSize::Size(_)) => false,
    };
    let not_taken = |size: ControlSize| ProblemKind::SizeNotTaken {
        size: size.name().to_owned(),
    };
    // The widget's default, stated with its sizes.
    let sizes = view.sizes.as_deref().unwrap_or_default();
    let default_at = if states("sizes") || states("default_size") {
        format!("{at}.default_size")
    } else {
        format!("{at}.{changed}")
    };
    match &view.default_size {
        Some(DefaultSize::Size(size)) if !sizes.contains(size) => {
            push(default_at, not_taken(*size));
        }
        // A pending default is missing whatever the sizes, but on the
        // synthetic route (S10's *A pending default*): a variant entry's
        // sizes may fall back on it.
        Some(DefaultSize::Pending(_)) if !synthetic => {
            push(default_at, ProblemKind::DefaultSizeMissing);
        }
        default if !sizes.is_empty() && missing(default.as_ref()) => {
            push(default_at, ProblemKind::DefaultSizeMissing);
        }
        _ => {}
    }
    // The variants' entries.
    let declared = view.variants.as_deref().unwrap_or_default();
    let declaration = if states("variants") {
        format!("{at}.variants")
    } else {
        format!("{own_at}.variants")
    };
    let entries = view.sizes_by_variant.as_deref().unwrap_or_default();
    for (index, entry) in entries.iter().enumerate() {
        let item = |rest: &str| place("sizes_by_variant", &format!("[{index}]{rest}"));
        let name = variant_name(&entry.variant);
        if entries
            .iter()
            .take(index)
            .any(|earlier| earlier.variant == entry.variant)
        {
            push(item(""), ProblemKind::ListedTwice { name: name.clone() });
        }
        if !declared.contains(&entry.variant) {
            let declaration = declaration.clone();
            let place = if !states("sizes_by_variant") && states("variants") {
                format!("{at}.variants")
            } else {
                item(".variant")
            };
            push(place, ProblemKind::Undeclared { name, declaration });
        }
        match &entry.default_size {
            Some(DefaultSize::Size(size)) if !entry.sizes.contains(size) => {
                push(item(".default_size"), not_taken(*size));
            }
            Some(DefaultSize::Pending(_)) if !synthetic => {
                push(item(".default_size"), ProblemKind::DefaultSizeMissing);
            }
            None if !entry.sizes.is_empty() => {
                let widgets = match &view.default_size {
                    Some(DefaultSize::Size(size)) => entry.sizes.contains(size),
                    // The widget's pending default is the missing default
                    // size at its own place, or waived.
                    Some(DefaultSize::Pending(_)) => true,
                    None => false,
                };
                if !widgets {
                    push(item(".default_size"), ProblemKind::DefaultSizeMissing);
                }
            }
            _ => {}
        }
    }
    // The contexts' entries, for each variant entry and for none.
    let contexts = view.sizes_by_context.as_deref().unwrap_or_default();
    for (index, entry) in contexts.iter().enumerate() {
        let item = |rest: &str| place("sizes_by_context", &format!("[{index}]{rest}"));
        for key in entry.refused_keys.iter().copied() {
            push(
                item(&format!(".context.{key}")),
                ProblemKind::SizeContextKey { key },
            );
        }
        if matches!(entry.default_size, Some(DefaultSize::Pending(_))) && !synthetic {
            push(item(".default_size"), ProblemKind::DefaultSizeMissing);
        }
        let holders = std::iter::once(None).chain(entries.iter().map(Some));
        for by_variant in holders {
            let outside = view.sizes_taken(by_variant, None);
            let inside = view.sizes_taken(by_variant, Some(entry));
            if inside.is_empty() {
                if !outside.is_empty() {
                    let variant = by_variant.map(|by| variant_name(&by.variant));
                    push(item(""), ProblemKind::NoSizeInContext { variant });
                }
                continue;
            }
            // A missing or pending default is reported, or waived, at its
            // own place.
            if let Some(DefaultSize::Size(size)) = view.default_in_force(by_variant, Some(entry))
                && !inside.contains(size)
            {
                let rest = if entry.default_size.is_some() {
                    ".default_size"
                } else {
                    ""
                };
                push(item(rest), not_taken(*size));
            }
        }
    }
    out
}

/// A section's recency sets (S10, S12): each of two flags or more, no flag
/// twice, a keyed flag declared in the flags in force, and no two of the
/// sets in force ordering two flags oppositely.
fn recency(at: &str, section: &WidgetDecl, in_force: &WidgetDecl) -> Vec<Problem> {
    let mut problems = Vec::new();
    let sets: &[(Key, Vec<StateFlag>)] = section.recency.as_deref().unwrap_or_default();
    for (set, flags) in sets {
        let set_at = format!("{at}.recency.{set}");
        if flags.len() < 2 {
            problems.push(Problem::new(&set_at, ProblemKind::RecencyTooSmall));
        }
        for (index, flag) in flags.iter().enumerate() {
            let item = format!("{set_at}[{index}]");
            if flags.iter().take(index).any(|earlier| earlier == flag) {
                let name = flag.name();
                problems.push(Problem::new(&item, ProblemKind::ListedTwice { name }));
            }
            let declared = !matches!(flag, StateFlag::Widget(_))
                || in_force.flags.iter().flatten().any(|held| held == flag);
            if !declared {
                let name = flag.name();
                let declaration = format!("{at}.flags");
                problems.push(Problem::new(
                    &item,
                    ProblemKind::Undeclared { name, declaration },
                ));
            }
        }
        let place = |flags: &[StateFlag], flag: &StateFlag| flags.iter().position(|f| f == flag);
        let opposite = in_force
            .recency
            .iter()
            .flatten()
            .filter(|(other, _)| other != set)
            .find(|(_, other)| {
                flags.iter().enumerate().any(|(i, a)| {
                    flags.iter().skip(i.saturating_add(1)).any(|b| {
                        matches!(
                            (place(other, a), place(other, b)),
                            (Some(x), Some(y)) if x > y
                        )
                    })
                })
            });
        if let Some((other, _)) = opposite {
            let set = other.to_string();
            problems.push(Problem::new(&set_at, ProblemKind::RecencyOrder { set }));
        }
    }
    problems
}

/// What a preset's layer uses of its declared keyed names (S12): the
/// selector names its rules select on, the keyed names its bases state,
/// the events its `presentations` lists name.
#[derive(Default)]
struct Uses {
    flags: Vec<StateFlag>,
    /// The recency sets `latest` terms name, by the widget they are of.
    recency: Vec<(Option<WidgetKind>, Key)>,
    conditions: Vec<Condition>,
    events: Vec<Key>,
    /// The bases' keyed names, by their declaration (`params` as
    /// `<group>.<key>`).
    stated: Vec<(&'static str, String)>,
    /// Walking the bases, not a rule.
    in_bases: bool,
}

impl Visit for Uses {
    fn check(&mut self, path: &Path, node: Node<'_>) {
        if let Node::Key(key) = node
            && path.slot() == Some(("PresentationEvent", "keyed"))
        {
            self.events.push(key.clone());
        }
    }

    fn value(&mut self, _: &Path, _: Form<'_>, _: ExprForm, _: Option<Ty>) {}

    fn number(&mut self, _: &Path, _: f32, _: Number) {}

    fn duplicate(&mut self, _: &Path, _: &str, _: &'static str) {}

    fn patch(&mut self, _: &Path, _: &'static [&'static str], _: &[&'static str]) {}

    fn map_key(&mut self, path: &Path, key: &str, _: &'static str) {
        if !self.in_bases {
            return;
        }
        let Some(declaration) = declaration_of(path.last_field()).filter(|d| d.map) else {
            return;
        };
        if declaration.decl == Decl::Params {
            if let Some(group) = path.keys().first() {
                self.stated.push(("params", format!("{group}.{key}")));
            }
            return;
        }
        self.stated.push((declaration.name(), key.to_owned()));
    }
}

impl Uses {
    /// A rule's selector, the rule targeting a part or values of `widget`.
    fn selector(&mut self, selector: &Selector, widget: Option<WidgetKind>) {
        let conditions = &selector.conditions;
        self.conditions
            .extend(conditions.all_of.iter().chain(&conditions.none_of).cloned());
        self.recency
            .extend(selector.latest.iter().map(|(set, _)| (widget, set.clone())));
        let mut states = vec![&selector.states];
        states.extend(
            selector
                .context
                .iter()
                .chain(&selector.not_context)
                .map(|c| &c.states),
        );
        if let Some(position) = &selector.position {
            states.extend(
                [&position.previous, &position.next]
                    .into_iter()
                    .flatten()
                    .map(|s| &s.states),
            );
        }
        for set in states {
            self.flags
                .extend(set.all_of.iter().chain(&set.none_of).cloned());
        }
        // A `latest` term selects on its flag too (S7.2).
        self.flags
            .extend(selector.latest.iter().map(|(_, flag)| flag.clone()));
    }

    /// A part's patch: its style's keyed names.
    fn part(&mut self, patch: &PartPatch) {
        self.walk(&*patch.style);
    }

    fn walk<T: Walk + ?Sized>(&mut self, value: &T) {
        let mut walker = Walker::new("", false, self);
        walker.walk(value);
    }

    fn bases(&mut self, bases: &Bases) {
        self.in_bases = true;
        for roles in [&bases.colors.light, &bases.colors.dark]
            .into_iter()
            .flatten()
        {
            self.walk(roles);
        }
        self.walk(&bases.tokens);
        let WidgetsPatch { button } = &bases.widgets;
        if let Some(button) = button {
            self.button(button);
        }
        self.in_bases = false;
    }

    fn button(&mut self, button: &ButtonPatch) {
        let (parts, values) = button.clone().into_parts();
        for (_, patch) in parts {
            self.part(&patch);
        }
        if let Some(values) = values {
            self.walk(&values);
        }
    }
}

/// The places a layer's bases state, each as its walk names it: a patch's
/// set fields, a value, a number, a map's entry; and the values holding
/// what lies below them whole (a reference, `Absent`, `Auto`).
#[derive(Default)]
struct Stated {
    paths: Vec<String>,
    wholes: Vec<String>,
}

impl Visit for Stated {
    fn check(&mut self, path: &Path, _: Node<'_>) {
        self.paths.push(path.to_string());
    }

    fn value(&mut self, path: &Path, form: Form<'_>, _: ExprForm, _: Option<Ty>) {
        match form {
            Form::Stated => self.paths.push(path.to_string()),
            Form::Absent | Form::Auto | Form::Expr(_) => self.wholes.push(path.to_string()),
        }
    }

    fn number(&mut self, path: &Path, _: f32, _: Number) {
        self.paths.push(path.to_string());
    }

    fn duplicate(&mut self, _: &Path, _: &str, _: &'static str) {}

    fn map_key(&mut self, path: &Path, key: &str, _: &'static str) {
        self.paths.push(path.join(&toml_key(key)));
    }

    fn patch(&mut self, path: &Path, _: &'static [&'static str], set: &[&'static str]) {
        self.paths.extend(set.iter().map(|field| path.join(field)));
    }
}

impl Stated {
    fn walk<T: Walk + ?Sized>(&mut self, root: &str, toml: bool, value: &T) {
        let mut walker = Walker::new(root, toml, self);
        walker.walk(value);
    }
}

/// Whether `bases`, of `layer`, state the listed path `listed` (as
/// `listed_path` gives it), an element of it where it is a list, or a
/// value holding it whole: a reference, `Absent`, a part stated `Absent`
/// or drawn as another (S17, ADD-A128); a listed input
/// (`inputs.<name>.<scheme>`) where its inputs state the input's
/// per-scheme value at that scheme (ADD-A132); a listed presence
/// (`widgets.<widget>.<part>`) where its bases state the part's presence,
/// `Absent`, drawn as another or `Present` (S17: "an application that
/// draws or hides the part", ADD-A131), a `Present` stating none of the
/// part's fields.
pub(super) fn states(bases: &Bases, layer: &Layer, listed: &str) -> bool {
    let toml = matches!(layer.spelled, Spelled::Toml);
    let mut stated = Stated::default();
    let Bases {
        colors,
        inputs,
        tokens,
        widgets,
    } = bases;
    if let Some((input, scheme)) = crate::toml::read::listed_input(listed) {
        return inputs
            .entries
            .iter()
            .any(|(name, value)| *name == input && value.per_scheme.get(scheme).is_some());
    }
    for (scheme, roles) in [("light", &colors.light), ("dark", &colors.dark)] {
        if let Some(roles) = roles {
            stated.walk(&format!("colors.{scheme}"), toml, roles);
        }
    }
    stated.walk("tokens", toml, tokens);
    let WidgetsPatch { button } = widgets;
    if let Some(button) = button {
        let (parts, values) = button.clone().into_parts();
        for (part, patch) in parts {
            let at = format!("widgets.button.{}", part.name());
            stated.walk(&at, toml, &*patch.style);
            match patch.presence {
                Some(PartPresence::Absent | PartPresence::DrawnAs { .. }) => {
                    stated.wholes.push(at);
                }
                Some(PartPresence::Present) => stated.paths.push(at),
                None => {}
            }
        }
        if let Some(values) = values {
            stated.walk("widgets.button.values", toml, &values);
        }
    }
    let exact = stated
        .paths
        .iter()
        .chain(&stated.wholes)
        .any(|path| listed_path(path) == listed);
    // A list's element stated states the list (a paint's `layers`), which a
    // layer's bases set whole (S6.1).
    let element = stated.paths.iter().any(|path| {
        path.strip_prefix(listed)
            .is_some_and(|rest| rest.starts_with('['))
    });
    exact
        || element
        || stated.wholes.iter().any(|path| {
            listed
                .strip_prefix(listed_path(path).as_str())
                .is_some_and(|rest| rest.starts_with('.'))
        })
}

/// Each declared keyed name nothing uses (S12): a keyed flag, a system
/// setting, a device model, a flavour or a container size no rule selects
/// on (but the default flavour, each container's default size and a
/// setting for applications), a keyed name no base states, an event no
/// `presentations` list names. A variant a platform preset declares and no
/// rule selects on is no problem (S12, ADD-A128): it draws as the widget's
/// base, and the facts gate checks the declared variants both ways against
/// the state model (S17). Only a preset's declarations reach this check:
/// an application layer and a palette declare no variants in the model.
/// The synthetic route's layers `synthetic` state keyed names in their
/// bases as the preset's do (S12, S17, ADD-A133).
pub(super) fn unused(
    decl: &Declarations,
    layer: &Layer,
    preset: Option<PresetId>,
    synthetic: &[Layer],
) -> Vec<Problem> {
    let mut uses = Uses::default();
    uses.bases(&layer.bases);
    for layer in synthetic {
        uses.bases(&layer.bases);
    }
    for rule in layer.rules.iter().filter(|rule| !excluded(rule, preset)) {
        let widget = match &rule.body {
            RuleBody::Colors(roles) => {
                uses.walk(roles);
                None
            }
            RuleBody::Tokens(tokens) => {
                uses.walk(tokens);
                None
            }
            RuleBody::Part(part, patch) => {
                uses.part(patch);
                match part {
                    PartRef::Typed(part) => Some(widget_of(*part)),
                    PartRef::Component { .. } => None,
                }
            }
            RuleBody::Values(ValuesPatch::Typed(WidgetValuesPatch::Button(values))) => {
                uses.walk(values);
                Some(WidgetKind::Button)
            }
        };
        uses.selector(&rule.selector, widget);
    }
    let mut unused: Vec<(String, String)> = Vec::new();
    let own = decl.widgets.iter().map(|(widget, section)| {
        (
            format!("declare.widget.{}", widget_name(*widget)),
            *widget,
            section,
        )
    });
    // A flavour of a unit not enabled is exempt, with its sections: it cannot
    // be asked for and no rule may name it (S12, S16; ADD-A147). The
    // default flavour's sections are checked, and so is the section of a
    // flavour `flavors` does not declare, no flavour of any unit.
    let later = |flavor: &Key| {
        !crate::widgets::FLAVOURS_UNIT
            && decl.flavors.contains(flavor)
            && decl.flavors.first() != Some(flavor)
    };
    let flavors = decl
        .flavor
        .iter()
        .filter(|(flavor, _)| !later(flavor))
        .flat_map(|(flavor, section)| {
            section.widgets.iter().map(move |(widget, section)| {
                let at = format!("declare.flavor.{flavor}.widget.{}", widget_name(*widget));
                (at, *widget, section)
            })
        });
    let sections: Vec<_> = own.chain(flavors).collect();
    for (at, widget, section) in &sections {
        for flag in section.flags.iter().flatten() {
            if let StateFlag::Widget(key) = flag
                && !uses.flags.contains(flag)
            {
                unused.push((format!("{at}.flags"), format!("@{key}")));
            }
        }
        for (set, _) in section.recency.iter().flatten() {
            if !uses.recency.contains(&(Some(*widget), set.clone())) {
                unused.push((format!("{at}.recency"), set.to_string()));
            }
        }
    }
    for model in &decl.device_models {
        if !uses
            .conditions
            .contains(&Condition::DeviceModel(model.clone()))
        {
            unused.push(("declare.device_models".to_owned(), model.to_string()));
        }
    }
    for setting in decl.conditions.iter().filter(|s| !s.for_applications) {
        if !uses
            .conditions
            .contains(&Condition::SystemSetting(setting.key.clone()))
        {
            unused.push(("declare.conditions".to_owned(), format!("@{}", setting.key)));
        }
    }
    for flavor in decl
        .flavors
        .iter()
        .skip(1)
        .filter(|_| crate::widgets::FLAVOURS_UNIT)
    {
        if !uses.conditions.contains(&Condition::Flavor(flavor.clone())) {
            unused.push(("declare.flavors".to_owned(), flavor.to_string()));
        }
    }
    for (container, sizes) in &decl.container_sizes {
        for size in sizes.iter().skip(1) {
            let condition = Condition::ContainerSize {
                container: container.clone(),
                size: size.clone(),
            };
            if !uses.conditions.contains(&condition) {
                unused.push(("declare.container_sizes".to_owned(), size.to_string()));
            }
        }
    }
    for event in &decl.presentation_events {
        if !uses.events.contains(event) {
            unused.push(("declare.presentation_events".to_owned(), event.to_string()));
        }
    }
    // The maps a base states, by the table; `params`' keys are their
    // groups', below.
    let mut stated: Vec<Decl> = Vec::new();
    for row in DECLARATIONS
        .iter()
        .filter(|row| row.map && row.decl != Decl::Params)
    {
        if !stated.contains(&row.decl) {
            stated.push(row.decl);
        }
    }
    for declaration in stated {
        let name = declaration.name();
        for key in declaration.keys(decl) {
            let used = uses
                .stated
                .iter()
                .any(|(held, used)| *held == name && used == key.as_str());
            if !used {
                unused.push((format!("declare.{name}"), key.to_string()));
            }
        }
    }
    for (group, params) in &decl.params {
        for key in &params.keys {
            let name = format!("{group}.{key}");
            if !uses
                .stated
                .iter()
                .any(|(held, used)| *held == "params" && *used == name)
            {
                unused.push(("declare.params".to_owned(), name));
            }
        }
    }
    unused
        .into_iter()
        .map(|(at, name)| Problem::new(&at, ProblemKind::Unused { name }))
        .collect()
}

/// `at.key`, or `key` at the root.
fn join(at: &str, key: &str) -> String {
    if at.is_empty() {
        key.to_owned()
    } else {
        format!("{at}.{key}")
    }
}

/// A map's key as a problem names it: the model's key, or, read from TOML,
/// the key as written, with its unit's suffix (S11); quoted where TOML
/// does not allow it bare.
fn spelled_key(key: &str, suffix: &str, toml: bool) -> String {
    if toml {
        format!("{}{suffix}", toml_key(key))
    } else {
        toml_key(key)
    }
}

/// One of a transition's fields, as S12's engine checks read it (S2.7;
/// ADD-A126).
#[derive(Clone, Copy, Debug)]
pub(super) enum Read {
    /// Its place: whether it is the part's `Geometry` transition alone, and
    /// the part, where it is one's.
    Place {
        geometry: bool,
        part: Option<TypedPart>,
    },
    /// Its duration, `Absent` or not.
    Duration { absent: bool },
    /// Its timing, `Absent` or not.
    Timing { absent: bool },
    /// A delay other than zero.
    Delay,
    /// Its kind, as S11 spells it.
    Kind(&'static str),
}

/// What the engine S11 spells `engine` takes where the transition's field
/// `read` refuses it (S2.7, S12; ADD-A126), `None` where the engine takes
/// the field: `WinUiAnimatedIcon` a `Keyed` transition with no duration,
/// timing or delay, its visual timing it; `Breeze` and `BreezeTabBar` an
/// `Interpolate` one with a duration and no delay; `GtkSwitch` the switch
/// thumb's `Geometry` transition alone, `Interpolate`, with no delay;
/// `ComposeHeld` a segmented control's segment's `Geometry` transition
/// alone; `CssAnimation` a `Keyframes` transition, and no other engine one;
/// and no value engine a `Keyed` one. A `FocusMove` kind is refused on its
/// own.
pub(super) fn takes(engine: &str, read: Read) -> Option<&'static str> {
    let icon = engine == "win_ui_animated_icon";
    let breeze = matches!(engine, "breeze" | "breeze_tab_bar");
    let switch = engine == "gtk_switch";
    let css = engine == "css_animation";
    match read {
        Read::Place { geometry, part } => match engine {
            "gtk_switch" if !(geometry && part.is_some_and(switch_thumb)) => {
                Some("the switch thumb's `geometry` transition only")
            }
            "compose_held" if !(geometry && part.is_some_and(segment)) => {
                Some("a segmented control's segment's `geometry` transition only")
            }
            _ => None,
        },
        Read::Duration { absent: false } if icon => Some("no duration: its visual times it"),
        Read::Duration { absent: true } if breeze => Some("a duration, on which its time runs"),
        Read::Timing { absent: false } if icon => Some("no timing: its visual times it"),
        Read::Delay if icon || breeze || switch => Some("no delay"),
        Read::Kind("focus_move") => None,
        Read::Kind(kind) if icon && kind != "keyed" => Some("a `keyed` transition only"),
        Read::Kind(kind) if (breeze || switch) && kind != "interpolate" => {
            Some("an `interpolate` transition only")
        }
        Read::Kind(kind) if css && kind != "keyframes" => Some("a `keyframes` transition only"),
        Read::Kind("keyframes") if !css => Some("no `keyframes` transition"),
        Read::Kind("keyed") if !icon && !breeze && !switch => Some("no `keyed` transition"),
        Read::Duration { .. } | Read::Timing { .. } | Read::Delay | Read::Kind(_) => None,
    }
}

/// What a Breeze test must be against its transition (S2.7, S12;
/// ADD-A126), each refusal by the list it names (`None`: the test): one
/// of `any` or `conditions` named on a transition that is not a stated
/// zero (`zero`), and all three lists empty on a stated zero, which reads
/// none. A focus indicator's own transition, staged with the motion unit
/// (S6.1), brings its rule of `focus_visible` in `any` back with it.
fn test_holds(test: &BreezeTest, zero: bool) -> Vec<(Option<&'static str>, &'static str)> {
    let mut out = Vec::new();
    if !zero && test.any.is_empty() && test.conditions.is_empty() {
        out.push((
            None,
            "names a flag in `any` or a condition: it holds nowhere",
        ));
    }
    if zero {
        for (list, empty) in [
            ("any", test.any.is_empty()),
            ("none", test.none.is_empty()),
            ("conditions", test.conditions.is_empty()),
        ] {
            if !empty {
                out.push((Some(list), "of a stated zero is empty: a zero reads none"));
            }
        }
    }
    out
}

/// A transition kind as S11 spells it.
fn kind_name(kind: &TransitionKind) -> &'static str {
    match kind {
        TransitionKind::Interpolate => "interpolate",
        TransitionKind::Keyed { .. } => "keyed",
        TransitionKind::FocusMove => "focus_move",
        TransitionKind::Keyframes { .. } => "keyframes",
    }
}

/// Whether keyframes' `at` increase: each above the one before it (S2.7,
/// S12).
fn increasing(keyframes: &[Keyframe]) -> bool {
    keyframes
        .iter()
        .zip(keyframes.iter().skip(1))
        .all(|(earlier, later)| earlier.at.0 < later.at.0)
}

/// The places a keyframe's patch states, as [`times`] reads them: each
/// field it sets, and below a stroke or a glyph stated field by field each
/// of their fields it sets (`stroke.top.width`, `glyph.draw_extent`), which
/// `StrokeWidth` and `GlyphDraw` time apart from their blocks.
fn stated_places(patch: &PartStylePatch) -> Vec<String> {
    let mut out = Vec::new();
    for field in patch.set_fields() {
        match (field, &patch.stroke, &patch.glyph) {
            ("stroke", Some(BlockPatch::Fields(stroke)), _) => out.extend(stroke_places(stroke)),
            ("glyph", _, Some(BlockPatch::Fields(glyph))) => out.extend(
                glyph
                    .set_fields()
                    .into_iter()
                    .map(|inner| format!("glyph.{inner}")),
            ),
            _ => out.push(field.to_owned()),
        }
    }
    out
}

/// The places a stroke's patch states: each edge's fields it sets
/// (`stroke.top.width`), and its own other fields.
fn stroke_places(stroke: &StrokePatch) -> Vec<String> {
    let edges = [
        ("top", &stroke.top),
        ("right", &stroke.right),
        ("bottom", &stroke.bottom),
        ("left", &stroke.left),
    ];
    let mut out: Vec<String> = edges
        .iter()
        .filter_map(|(name, edge)| edge.as_ref().map(|edge| (name, edge.set_fields())))
        .flat_map(|(name, fields)| {
            fields
                .into_iter()
                .map(move |field| format!("stroke.{name}.{field}"))
        })
        .collect();
    out.extend(
        stroke
            .set_fields()
            .into_iter()
            .filter(|field| !edges.iter().any(|(name, _)| name == field))
            .map(|field| format!("stroke.{field}")),
    );
    out
}

/// Whether a transition of `group` times a keyframe's `place`
/// ([`stated_places`]; S2.7): each group its part style's field of that
/// name, `Stroke` and `Glyph` every field of their blocks, `StrokeWidth` the
/// edges' widths, `GlyphDraw` the glyph's source (its path), `draw_extent`
/// and `draw_start`, `Geometry` the fields that size and place the laid-out
/// box (padding, margin and the size limits), and `StateLayer` the part's
/// `interaction`.
fn times(group: PropertyGroup, place: &str) -> bool {
    let block = place.split('.').next().unwrap_or(place);
    match group {
        PropertyGroup::Fill => place == "fill",
        PropertyGroup::Stroke => block == "stroke",
        PropertyGroup::StrokeWidth => matches!(
            place,
            "stroke.top.width" | "stroke.right.width" | "stroke.bottom.width" | "stroke.left.width"
        ),
        PropertyGroup::Corners => place == "corners",
        PropertyGroup::Shape => place == "shape",
        PropertyGroup::Shadow => place == "shadow",
        PropertyGroup::Opacity => place == "opacity",
        PropertyGroup::Transform => place == "transform",
        PropertyGroup::Text => place == "text",
        PropertyGroup::Icon => place == "icon",
        PropertyGroup::Glyph => block == "glyph",
        PropertyGroup::GlyphDraw => matches!(
            place,
            "glyph.source" | "glyph.draw_extent" | "glyph.draw_start"
        ),
        PropertyGroup::Geometry => matches!(
            place,
            "padding" | "margin" | "min_width" | "min_height" | "max_width" | "max_height"
        ),
        PropertyGroup::StateLayer => place == "interaction",
    }
}

/// A transition engine as S11 spells it.
fn engine_name(engine: &TransitionEngine) -> &'static str {
    match engine {
        TransitionEngine::GtkCss => "gtk_css",
        TransitionEngine::AdwAnimation { .. } => "adw_animation",
        TransitionEngine::QtQuick => "qt_quick",
        TransitionEngine::Breeze(_) => "breeze",
        TransitionEngine::BreezeTabBar(_) => "breeze_tab_bar",
        TransitionEngine::ComposeAnimatable => "compose_animatable",
        TransitionEngine::ComposeTransition => "compose_transition",
        TransitionEngine::ComposeHeld => "compose_held",
        TransitionEngine::WinUi => "win_ui",
        TransitionEngine::WinUiAnimatedIcon { .. } => "win_ui_animated_icon",
        TransitionEngine::GtkSwitch => "gtk_switch",
        TransitionEngine::CssAnimation => "css_animation",
    }
}

/// Whether a part is the switch's thumb, the one part `GtkSwitch` slides
/// (S2.7): none of the catalogue's parts yet.
fn switch_thumb(part: TypedPart) -> bool {
    match part {
        TypedPart::Button(_) => false,
    }
}

/// Whether a part is a segmented control's segment, whose layout the
/// connector draws from `ComposeHeld`'s fraction (S2.7): none of the
/// catalogue's parts yet.
fn segment(part: TypedPart) -> bool {
    match part {
        TypedPart::Button(_) => false,
    }
}

/// Whether a condition is the window's, S7.1's group set by the connector.
fn window_condition(condition: &Condition) -> bool {
    match condition {
        Condition::WindowInactive
        | Condition::NotMain
        | Condition::AppInactive
        | Condition::Maximized
        | Condition::FullScreen
        | Condition::Tiled(_)
        | Condition::Elevated
        | Condition::Input(_)
        | Condition::NotComposited
        | Condition::Shaded
        | Condition::AttentionFlash
        | Condition::TintDimmed
        | Condition::MnemonicsVisible
        | Condition::Freeform
        | Condition::Posture(_)
        | Condition::UnitScale => true,
        Condition::Scheme(_)
        | Condition::ContrastIncreased
        | Condition::ContrastHigh
        | Condition::ReducedTransparency
        | Condition::ReducedMotion
        | Condition::AnimationsOff
        | Condition::BoldText
        | Condition::ButtonShapes
        | Condition::DifferentiateWithoutColor
        | Condition::TabletMode
        | Condition::MobileFormFactor
        | Condition::BatterySaver
        | Condition::ContainerSize { .. }
        | Condition::OverlayScrollbars
        | Condition::OnOffLabels
        | Condition::FullKeyboardAccess
        | Condition::PreferCrossFade
        | Condition::AccessibilityTextSize
        | Condition::Idiom(_)
        | Condition::SystemSetting(_)
        | Condition::DeviceModel(_)
        | Condition::Flavor(_)
        | Condition::Preset(_) => false,
    }
}

/// A set test's items, each at its `when` key and index, read from TOML:
/// `all_of` under `name`, `none_of` under `not_<name>` (S11); built in
/// Rust, under `name.all_of` and `name.none_of`.
fn items<'s, T>(set: &'s SetMatch<T>, at: &str, name: &str, toml: bool) -> Vec<(&'s T, String)> {
    let (all, none) = if toml {
        (format!("{at}.{name}"), format!("{at}.not_{name}"))
    } else {
        (
            format!("{at}.{name}.all_of"),
            format!("{at}.{name}.none_of"),
        )
    };
    let all = set
        .all_of
        .iter()
        .enumerate()
        .map(|(index, item)| (item, format!("{all}[{index}]")));
    let none = set
        .none_of
        .iter()
        .enumerate()
        .map(|(index, item)| (item, format!("{none}[{index}]")));
    all.chain(none).collect()
}

/// A scheme map's entries, by the scheme's name.
fn schemes<T>(map: &SchemeMap<T>) -> impl Iterator<Item = (&'static str, &T)> {
    [("light", map.light.as_ref()), ("dark", map.dark.as_ref())]
        .into_iter()
        .filter_map(|(name, value)| value.map(|value| (name, value)))
}

/// The widget a part belongs to.
fn widget_of(part: TypedPart) -> WidgetKind {
    match part {
        TypedPart::Button(_) => WidgetKind::Button,
    }
}

/// The part's catalogue entry.
fn part_info(part: TypedPart) -> Option<&'static PartInfo> {
    let name = match part {
        TypedPart::Button(part) => part.name(),
    };
    registry::widgets()
        .iter()
        .find(|info| info.kind == widget_of(part))
        .and_then(|info| info.part(name))
}

pub(super) fn widget_name(widget: WidgetKind) -> &'static str {
    registry::widgets()
        .iter()
        .find(|info| info.kind == widget)
        .map_or("", |info| info.name)
}

/// A part as S11 writes it with its widget: `<widget>.<part>`, a keyed
/// component's `component.<key>.<part>`.
pub(super) fn part_ref_name(part: &PartRef) -> String {
    match part {
        PartRef::Typed(part) => format!("{}.{}", widget_name(part.widget()), part.name()),
        PartRef::Component { key, part } => format!("component.{key}.{part}"),
    }
}

/// The typed widget a context's or a sibling's target names.
fn target_widget(target: Option<&WidgetOrPart>) -> Option<WidgetKind> {
    match target? {
        WidgetOrPart::Widget(WidgetRef::Typed(widget)) => Some(*widget),
        WidgetOrPart::Part(PartRef::Typed(part)) => Some(widget_of(*part)),
        WidgetOrPart::Widget(WidgetRef::Component(_))
        | WidgetOrPart::Part(PartRef::Component { .. }) => None,
    }
}

fn variant_name(variant: &VariantRef) -> String {
    match variant {
        VariantRef::Typed(variant) => variant.name().to_owned(),
        VariantRef::Keyed(key) => format!("@{key}"),
    }
}

fn unit_name(unit: ScalarUnit) -> &'static str {
    match unit {
        ScalarUnit::Px => "px",
        ScalarUnit::Ms => "ms",
        ScalarUnit::Deg => "deg",
        ScalarUnit::Fraction => "fraction",
        ScalarUnit::Number => "number",
    }
}

/// The root's inputs, which need no declaration (S8, S10).
fn is_root_input(datum: ElementDatum) -> bool {
    matches!(
        datum,
        ElementDatum::WindowWidth
            | ElementDatum::WindowHeight
            | ElementDatum::ScreenWidth
            | ElementDatum::ScreenHeight
    )
}

/// An input of KDE's scheme: `kde_scheme.<set>.<key>` or
/// `kde_scheme.<set>.inactive.<key>` (S9.1).
fn is_kde_scheme_key(name: &InputName) -> bool {
    match name.segments() {
        [scheme, _, _] => scheme.as_str() == "kde_scheme",
        [scheme, _, inactive, _] => {
            scheme.as_str() == "kde_scheme" && inactive.as_str() == "inactive"
        }
        _ => false,
    }
}

/// The element a part is queried with: the nearest of itself and the parts
/// it nests in that is an *(element)* part, else the widget's root (S8).
fn element_of(parts: &'static [PartInfo], part: &'static PartInfo) -> &'static str {
    let mut current = part;
    for _ in parts {
        if current.element || current.root {
            return current.name;
        }
        match current
            .parent
            .and_then(|parent| parts.iter().find(|p| p.name == parent))
        {
            Some(parent) => current = parent,
            None => break,
        }
    }
    parts
        .iter()
        .find(|p| p.root)
        .map_or(current.name, |root| root.name)
}

/// The part `other` names among `parts`, where it is of another element
/// than `own`, the part a `widget.<other>` reference is stated on (S2.2).
fn other_element(
    parts: &'static [PartInfo],
    own: &'static PartInfo,
    other: &str,
) -> Option<String> {
    let other_info = parts.iter().find(|part| part.name == other)?;
    (element_of(parts, own) != element_of(parts, other_info)).then(|| other.to_owned())
}

/// How deep calls and `active(…)` nest in `expr` (S2.2), counted no further
/// than one past [`MAX_DEPTH`], so an expression built in Rust deeper than
/// that is refused without being followed down, and without recursion.
fn depth(expr: &Expr) -> usize {
    let limit = MAX_DEPTH.saturating_add(1);
    let mut deepest = 0_usize;
    let mut pending: Vec<(&Expr, usize)> = vec![(expr, 0)];
    while let Some((expr, at)) = pending.pop() {
        let reached = match expr {
            Expr::Lit(_) => at,
            Expr::Ref(reference) => at.saturating_add(ref_depth(reference)),
            Expr::Call(_, args) => {
                let inner = at.saturating_add(1);
                if inner < limit {
                    pending.extend(args.iter().map(|arg| (arg, inner)));
                }
                inner
            }
        };
        deepest = deepest.max(reached);
        if deepest >= limit {
            return deepest;
        }
    }
    deepest
}

/// How many `active(…)` enclose the reference at the end of `reference`'s
/// chain, counted without recursion.
fn ref_depth(reference: &Ref) -> usize {
    let mut depth = 0_usize;
    let mut reference = reference;
    while let Ref::Active(inner) = reference {
        depth = depth.saturating_add(1);
        reference = inner;
    }
    depth
}

/// The checks no layer can reach through the enabled widgets yet, run
/// directly: a widget's loop value's keyframe (no enabled widget's values
/// hold a loop), an *(element)* part's reference to its host (`button`
/// has no *(element)* part), and a block's own checks where no enabled
/// part lists the field that holds it (a part's `transitions`, an icon's
/// loop: the motion unit's, D6 outcome 3, S6.1).
#[cfg(test)]
pub(super) mod triggers {
    use super::tests::{PARTS, looped};
    use super::*;
    use crate::units::Fraction;
    use crate::widgets::ButtonPart;

    fn checker_over<'a>(cx: &'a Context<'a>) -> Checker<'a> {
        Checker::new(cx, false)
    }

    /// The problems S12's structural checks find in `block` walked from
    /// `at` (its places spelled by TOML's keys where `toml`), as a field of
    /// the part `part` of the catalogue's entry, or of the tokens where
    /// `None`, over `decl`: the block's own checks, no widget in between.
    pub(in crate::validate) fn walked_block<T: Walk + ?Sized>(
        block: &T,
        at: &str,
        part: Option<TypedPart>,
        toml: bool,
        decl: &Declarations,
    ) -> Vec<Problem> {
        let cx = Context {
            declarations: decl,
            preset: None,
            kind: LayerKind::Application,
            reader_accent_left_out: false,
            below: &[],
        };
        let mut checker = Checker::new(&cx, toml);
        let region = part
            .and_then(|part| {
                let widget = widget_of(part);
                let parts = registry::widgets()
                    .iter()
                    .find(|w| w.kind == widget)
                    .map_or(&[] as &[PartInfo], |w| w.parts);
                part_info(part).map(|info| Region::Part(widget, part, info, parts))
            })
            .unwrap_or(Region::Tokens);
        checker.walk(at, region, block);
        checker.problems
    }

    /// A keyframe of a loop stated as a widget's value with no part.
    pub(in crate::validate) fn keyframe_without_part() -> Vec<Problem> {
        let decl = Declarations::default();
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Application,
            reader_accent_left_out: false,
            below: &[],
        };
        let mut checker = checker_over(&cx);
        checker.walk("probe", Region::Values(WidgetKind::Button), &looped(None));
        checker.problems
    }

    /// A synthetic *(element)* part's mark reading its host's label.
    pub(in crate::validate) fn other_elements_part() -> Vec<Problem> {
        let decl = Declarations::default();
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Application,
            reader_accent_left_out: false,
            below: &[],
        };
        let mut checker = checker_over(&cx);
        if let Some(mark) = PARTS.iter().find(|part| part.name == "mark")
            && let Ok(expr) = Expr::parse("widget.label.opacity")
        {
            let region = Region::Part(
                WidgetKind::Button,
                TypedPart::Button(ButtonPart::Body),
                mark,
                &PARTS,
            );
            checker.walk("probe", region, &Value::<Fraction>::Expr(expr));
        }
        checker.problems
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::blocks::motion::Keyframe;
    use crate::blocks::motion::Loop;
    use crate::blocks::part_style::PartStylePatch;
    use crate::key;
    use crate::registry::FieldInfo;
    use crate::units::{Fraction, Ms};
    use crate::validate::walk::Marks;
    use crate::widgets::ButtonPart;

    const NO_PROBLEM: [&str; 0] = [];

    /// The problems of `looped` walked in `region`.
    pub(super) fn walked(looped: &Loop, region: Region) -> Vec<String> {
        let decl = Declarations::default();
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Application,
            reader_accent_left_out: false,
            below: &[],
        };
        let mut checker = Checker::new(&cx, false);
        checker.walk("probe", region, looped);
        checker.problems.iter().map(Problem::to_string).collect()
    }

    pub(super) fn looped(part: Option<PartRef>) -> Loop {
        Loop {
            period: Value::Stated(Ms(1000.0)),
            timing: Value::Stated(Timing::Linear),
            keyframes: vec![Keyframe {
                at: Fraction(0.0),
                part,
                timing: Value::Absent,
                patch: PartStylePatch::default(),
            }],
        }
    }

    /// No enabled widget's values hold a loop yet, so the check is walked
    /// over a loop as a widget's value directly (S2.7).
    #[test]
    fn refusal_keyframe_without_part() {
        let values = Region::Values(WidgetKind::Button);
        assert_eq!(
            walked(&looped(None), values),
            ["probe.keyframes[0].part: a widget's or component's loop's keyframe names no part"]
        );
        let body = PartRef::Typed(TypedPart::Button(ButtonPart::Body));
        assert!(walked(&looped(Some(body)), values).is_empty());
        let info = part_info(TypedPart::Button(ButtonPart::Icon)).unwrap();
        let on_part = Region::Part(
            WidgetKind::Button,
            TypedPart::Button(ButtonPart::Icon),
            info,
            &[],
        );
        assert!(walked(&looped(None), on_part).is_empty());
    }

    const fn part(
        name: &'static str,
        root: bool,
        element: bool,
        parent: Option<&'static str>,
    ) -> PartInfo {
        PartInfo {
            name,
            root,
            element,
            parent,
            may_be_absent: false,
            fields: &[] as &[FieldInfo],
        }
    }

    /// A synthetic widget: a root, an *(element)* row and the row's mark.
    pub(super) static PARTS: [PartInfo; 4] = [
        part("body", true, false, None),
        part("label", false, false, None),
        part("row", false, true, None),
        part("mark", false, false, Some("row")),
    ];

    /// `button` has no *(element)* part, so the check behind
    /// `OtherElementsPart` runs over a synthetic widget (S2.2, S8): a
    /// `widget.<part>` reference across two elements is refused, within one
    /// it is not.
    #[test]
    fn refusal_other_elements_part() {
        let part = |name: &str| PARTS.iter().find(|p| p.name == name).unwrap();
        let read = |on: &str, text: &str| {
            let decl = Declarations::default();
            let cx = Context {
                declarations: &decl,
                preset: None,
                kind: LayerKind::Application,
                reader_accent_left_out: false,
                below: &[],
            };
            let mut checker = Checker::new(&cx, false);
            let region = Region::Part(
                WidgetKind::Button,
                TypedPart::Button(ButtonPart::Body),
                part(on),
                &PARTS,
            );
            let value = Value::<Fraction>::Expr(Expr::parse(text).unwrap());
            checker.walk("probe", region, &value);
            checker
                .problems
                .iter()
                .map(Problem::to_string)
                .collect::<Vec<_>>()
        };
        // The row's mark reads the host's label: another element.
        assert_eq!(
            read("mark", "widget.label.opacity"),
            ["probe: `widget.label` is another element's part"]
        );
        // The host's label reads the row's mark: another element.
        assert_eq!(
            read("label", "widget.mark.opacity"),
            ["probe: `widget.mark` is another element's part"]
        );
        // Within one element: the mark reads its row, the label the body;
        // the synthetic parts state no field, so the typing finds none.
        for (on, text) in [
            ("mark", "widget.row.opacity"),
            ("label", "widget.body.opacity"),
        ] {
            assert!(
                read(on, text)
                    .iter()
                    .all(|problem| !problem.contains("another element's part")),
                "{on}: {text}"
            );
        }
        // A name the widget does not have is the typing's unknown name.
        assert_eq!(other_element(&PARTS, part("label"), "nope"), None);
    }

    /// The depth counts no further than one past the limit: an expression
    /// built in Rust far deeper than 32 is not followed down (S12).
    #[test]
    fn depth_stops_past_the_limit() {
        let nested = |calls: usize| {
            let inner = Expr::Lit(Literal::Number(1.0));
            (0..calls).fold(inner, |expr, _| Expr::Call(Func::Floor, vec![expr]))
        };
        assert_eq!(depth(&nested(MAX_DEPTH)), MAX_DEPTH);
        assert_eq!(depth(&nested(MAX_DEPTH + 1)), MAX_DEPTH + 1);
        assert_eq!(depth(&nested(200)), MAX_DEPTH + 1);
    }

    /// A rule naming no preset is validated under any; one whose `Preset`
    /// condition names another preset than the current one is not, nor one
    /// whose `not_conditions` names the current preset, so one layer can
    /// carry rules for several presets' keyed names (S12): a timing
    /// `adwaita` does not declare, named under `preset:adwaita`, is refused
    /// on `adwaita` alone.
    #[test]
    fn a_rule_naming_another_preset_is_not_validated() -> Result<(), crate::error::LayerError> {
        let rule = Rule::new(Selector::default(), RuleBody::Tokens(Box::default()));
        assert!(!excluded(&rule, None));
        assert!(!excluded(&rule, Some(PresetId::Adwaita)));
        let mut named = rule.clone();
        named
            .selector
            .conditions
            .all_of
            .push(Condition::Preset(PresetId::Adwaita));
        assert!(excluded(&named, None));
        assert!(!excluded(&named, Some(PresetId::Adwaita)));
        let mut unless = rule;
        unless
            .selector
            .conditions
            .none_of
            .push(Condition::Preset(PresetId::Adwaita));
        assert!(!excluded(&unless, None));
        assert!(excluded(&unless, Some(PresetId::Adwaita)));
        let layer = Layer::from_toml(
            "[[rules]]\nwhen = { conditions = [\"preset:adwaita\"] }\ntarget = \"tokens\"\n\
             set = { elevation.probe = [] }",
        )?;
        let problems = |preset: Option<PresetId>| {
            let decl = Declarations::default();
            let cx = Context {
                declarations: &decl,
                preset,
                kind: LayerKind::Application,
                reader_accent_left_out: false,
                below: &[],
            };
            super::layer(&layer, &cx)
                .iter()
                .map(Problem::to_string)
                .collect::<Vec<_>>()
        };
        assert_eq!(problems(None), Vec::<String>::new());
        assert_eq!(
            problems(Some(PresetId::Adwaita)),
            ["rules[0].set.elevation.probe: `probe` is not declared in `elevation`"]
        );
        Ok(())
    }

    /// A widget value's key is listed by the widget's `keys` entry for the
    /// value (S10), every key refused where the widget lists none.
    /// `button` holds no `Value<Key>` value, so the check is walked over a
    /// key as a widget's value directly.
    #[test]
    fn a_widget_values_key_is_listed() {
        let problems = |keys: Option<Vec<(&'static str, Vec<Key>)>>| {
            let decl = Declarations {
                widgets: vec![(
                    WidgetKind::Button,
                    WidgetDecl {
                        keys,
                        ..WidgetDecl::default()
                    },
                )],
                ..Declarations::default()
            };
            let cx = Context {
                declarations: &decl,
                preset: None,
                kind: LayerKind::Application,
                reader_accent_left_out: false,
                below: &[],
            };
            let mut checker = Checker::new(&cx, false);
            checker.region = Region::Values(WidgetKind::Button);
            let toml = checker.toml;
            let mut walker = Walker::new("probe", toml, &mut checker);
            walker.field("ButtonValues", "mode", Marks::default(), "", |w| {
                Walk::walk(&key!("compact"), w);
            });
            checker
                .problems
                .iter()
                .map(Problem::to_string)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            problems(None),
            ["probe.mode: `compact` is not declared in `widget.button.keys.mode`"]
        );
        assert_eq!(
            problems(Some(vec![("mode", vec![key!("compact")])])),
            NO_PROBLEM
        );
    }

    /// A call on a `Value` whose type gives no expression type and takes a
    /// reference only (a `ViewBox`, as the `Walk` default leaves it) is the
    /// mistype typing would report, which it cannot reach without a type
    /// (S2.2, S12).
    #[test]
    fn a_call_where_the_walk_gives_no_type_is_ill_typed() {
        let decl = Declarations::default();
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Application,
            reader_accent_left_out: false,
            below: &[],
        };
        let mut checker = Checker::new(&cx, true);
        let call = crate::expr::Expr::parse("mul(1, 2)").unwrap();
        let value: Value<crate::units::ViewBox> = Value::Expr(call);
        let mut walker = Walker::new("probe", true, &mut checker);
        walker.field("Glyph", "view_box", Marks::default(), "", |w| {
            Walk::walk(&value, w);
        });
        let problems: Vec<String> = checker.problems.iter().map(Problem::to_string).collect();
        assert_eq!(
            problems,
            [
                "probe.view_box: the expression does not type: the field's expression is a reference only"
            ]
        );
    }

    /// A `WinUiAnimatedIcon` engine's `visual` under a widget's values
    /// names an entry of WinUI's animated visuals (a token staged with the
    /// motion unit), no key the value holds:
    /// it is not checked against the widget's `keys` (S2.7, S10, S12).
    #[test]
    fn a_visual_is_no_widget_values_key() {
        use crate::blocks::motion::{Transition, TransitionEngine, TransitionKind};
        use crate::selector::StateFlag;
        let decl = Declarations {
            widgets: vec![(WidgetKind::Button, WidgetDecl::default())],
            transition_kinds: vec![(key!("normal_off"), Vec::new())],
            ..Declarations::default()
        };
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Application,
            reader_accent_left_out: false,
            below: &[],
        };
        let mut checker = Checker::new(&cx, true);
        checker.region = Region::Values(WidgetKind::Button);
        let mut walker = Walker::new("probe", true, &mut checker);
        walker.field("ButtonValues", "transitions", Marks::default(), "", |w| {
            Walk::walk(
                &vec![Transition {
                    properties: vec![crate::blocks::motion::PropertyGroup::Icon],
                    kind: TransitionKind::Keyed {
                        key: key!("normal_off"),
                        params: Vec::new(),
                    },
                    duration: Value::Absent,
                    delay: Value::Stated(crate::units::Ms(0.0)),
                    timing: Value::Absent,
                    engine: TransitionEngine::WinUiAnimatedIcon {
                        visual: key!("accept"),
                    },
                    from: Some(vec![StateFlag::Hover]),
                }],
                w,
            );
        });
        let problems: Vec<String> = checker.problems.iter().map(Problem::to_string).collect();
        assert_eq!(problems, NO_PROBLEM);
    }
}

#[cfg(test)]
mod table_tests {
    use super::*;
    use crate::registry::FieldType as Registry;

    /// Every `(owner, field)` slot the registry's tables hold, from the
    /// roots down: a struct block's fields by its name, an enum block's
    /// variants' fields by the enum's, each path's last segment.
    fn slots() -> Vec<(&'static str, &'static str)> {
        fn visit(
            owner: &'static str,
            fields: &'static [FieldInfo],
            out: &mut Vec<(&'static str, &'static str)>,
            seen: &mut Vec<&'static str>,
        ) {
            for field in fields {
                let name = field.path.rsplit('.').next().unwrap_or(field.path);
                out.push((owner, name));
                let (block, below) = match field.ty {
                    Registry::Leaf(_) => continue,
                    Registry::Block { block, fields, .. } => (block, fields),
                    Registry::Patch { block, fields, .. } => (block, fields()),
                };
                if !seen.contains(&block) {
                    seen.push(block);
                    visit(block, below, out, seen);
                }
            }
        }
        let mut out = Vec::new();
        let mut seen = Vec::new();
        visit("ColorRoles", registry::colors(), &mut out, &mut seen);
        visit("Tokens", registry::tokens(), &mut out, &mut seen);
        for widget in registry::widgets() {
            for part in widget.parts {
                visit("PartStyle", part.fields, &mut out, &mut seen);
            }
            visit("WidgetValues", widget.values, &mut out, &mut seen);
        }
        // The motion blocks, whose types stay while no part lists
        // `transitions` and no widget `presentations` (the motion unit's,
        // D6 outcome 3, S6.1).
        use crate::blocks::motion::{PresentationTransition, Transition};
        visit("Transition", Transition::FIELDS, &mut out, &mut seen);
        visit(
            "PresentationTransition",
            PresentationTransition::FIELDS,
            &mut out,
            &mut seen,
        );
        out
    }

    /// The `Key` slots a walk meets in values holding a keyed variant of a
    /// leaf enum, which the registry's tables do not reach.
    fn walked_key_slots() -> Vec<(&'static str, &'static str)> {
        use crate::blocks::focus::{FocusIndicatorPatch, FocusKind};
        use crate::blocks::motion::{
            PresentationEvent, PresentationKind, PresentationStep, PresentationTransition, Timing,
            Transition, TransitionKind,
        };
        use crate::blocks::paint::{BlendMode, Paint, PaintKind, PaintLayer};
        use crate::blocks::part_style::{Binding, RoleRef};
        use crate::roles::GlobalRole;
        use crate::validate::walk::{ExprForm, Form, Node, Number, Path, Visit, Walker};

        struct Slots(Vec<(&'static str, &'static str)>);
        impl Visit for Slots {
            fn check(&mut self, path: &Path, node: Node<'_>) {
                if let (Node::Key(_), Some(slot)) = (node, path.slot()) {
                    self.0.push(slot);
                }
            }
            fn value(&mut self, _: &Path, _: Form<'_>, _: ExprForm, _: Option<Ty>) {}
            fn number(&mut self, _: &Path, _: f32, _: Number) {}
            fn duplicate(&mut self, _: &Path, _: &str, _: &'static str) {}
            fn map_key(&mut self, _: &Path, _: &str, _: &'static str) {}
            fn patch(&mut self, _: &Path, _: &'static [&'static str], _: &[&'static str]) {}
        }
        let k = || crate::key!("probe");
        let color = Value::Stated(crate::color::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        });
        let mut slots = Slots(Vec::new());
        let mut w = Walker::new("", false, &mut slots);
        w.walk(&Paint {
            layers: vec![PaintLayer {
                kind: PaintKind::Color(color.clone()),
                blend: BlendMode::Keyed(k()),
            }],
        });
        w.walk(&FocusIndicatorPatch {
            kind: Some(FocusKind::Keyed(k())),
            ..FocusIndicatorPatch::default()
        });
        w.walk(&vec![
            Binding::Role {
                role: RoleRef::Keyed(k()),
                value: color.clone(),
            },
            Binding::Role {
                role: RoleRef::Global(GlobalRole::AccentRamp(k())),
                value: color,
            },
        ]);
        w.walk(&Transition {
            properties: Vec::new(),
            kind: TransitionKind::Keyed {
                key: k(),
                params: Vec::new(),
            },
            duration: Value::Absent,
            delay: Value::Absent,
            timing: Value::Stated(Timing::Linear),
            engine: crate::blocks::motion::TransitionEngine::GtkCss,
            from: None,
        });
        w.walk(&PresentationTransition {
            event: PresentationEvent::Keyed(k()),
            steps: vec![PresentationStep {
                part: None,
                kind: PresentationKind::Keyed {
                    key: k(),
                    params: Vec::new(),
                },
                duration: Value::Absent,
                delay: Value::Absent,
                timing: Value::Stated(Timing::Linear),
                stagger: Value::Absent,
            }],
        });
        slots.0
    }

    /// The checks name slots by their owner's and field's names: each named
    /// slot is one the model has, so a rename fails here instead of
    /// silently dropping a check.
    #[test]
    fn named_slots_exist() {
        let mut slots = slots();
        slots.extend(walked_key_slots());
        for row in &DECLARATIONS {
            assert!(
                slots.contains(&(row.owner, row.field)),
                "{}.{}",
                row.owner,
                row.field
            );
        }
        // The slots `value`'s and `number`'s checks name.
        for slot in [
            ("TextTokens", "keyed"),
            ("IconSizeTokens", "keyed"),
            ("ColorRoles", "surfaces"),
            ("Tokens", "params"),
            ("Material", "saturation"),
            ("Glyph", "draw_extent"),
            ("TextTokens", "font_dpi"),
            ("TransitionKind", "params"),
            ("PresentationKind", "params"),
            ("Material", "params"),
        ] {
            assert!(slots.contains(&slot), "{slot:?}");
        }
    }

    /// A key field's declaration is its block's: the same field name in
    /// another block names none.
    #[test]
    fn a_key_fields_declaration_is_its_blocks() {
        assert_eq!(
            declaration_of_field("IconStyle", "scale"),
            Some("symbol_scales")
        );
        assert_eq!(declaration_of_field("Transform", "scale"), None);
        let fields = <crate::blocks::part_style::PartStyle as crate::blocks::Table>::FIELDS;
        let block = |path| match registry::find(fields, path).map(|info| info.ty) {
            Some(
                registry::FieldType::Block { block, .. } | registry::FieldType::Patch { block, .. },
            ) => Some(block),
            _ => None,
        };
        assert_eq!(block("icon"), Some("IconStyle"));
        assert_eq!(block("shadow"), Some("Shadow"));
        assert_eq!(
            block("transitions.kind.keyframes.keyframes.patch"),
            Some("PartStyle")
        );
    }
}
