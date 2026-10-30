//! S12's structural checks: each value, patch, rule and declaration on its
//! own, every problem collected, one problem per fault.

use std::borrow::Cow;

use super::walk::{ExprForm, Form, KEY_OF_PLACE, Node, Number, Path, Visit, Walk, Walker};
use super::{Context, LayerKind};
use crate::next::blocks::Table;
use crate::next::blocks::motion::{Keyframe, PresentationKind, StepJump, Timing, TransitionKind};
use crate::next::blocks::outline::Radius;
use crate::next::blocks::paint::{BlendMode, PaintKind, PaintPatch};
use crate::next::blocks::part_style::PartStylePatch;
use crate::next::blocks::shadow::Shadow;
use crate::next::blocks::text::{AxisTag, AxisValue, FeatureTag};
use crate::next::declare::{DataName, DataOwner, Declarations, FontMetric, ScalarUnit, WidgetDecl};
use crate::next::error::{Problem, ProblemKind};
use crate::next::expr::{
    Bound, ElementDatum, Expr, FieldType, Func, Literal, MAX_DEPTH, Name, Ref, Ty, TypeEnv,
};
use crate::next::inputs::{InputBases, InputName, InputValue};
use crate::next::key::Key;
use crate::next::layer::{Bases, Layer, Spelled};
use crate::next::presets::PresetId;
use crate::next::registry::{self, FieldInfo, PartInfo};
use crate::next::roles::{ColorRolesPatch, SurfaceRoles, SurfaceSet};
use crate::next::selector::{
    Condition, ContextMatch, ControlSize, Density, PartRef, PositionMatch, Rule, RuleBody,
    Selector, SetMatch, SiblingMatch, StateFlag, ValuesPatch, VariantRef, WidgetOrPart, WidgetRef,
};
use crate::next::tokens::ABSENT_PARAM_GROUPS;
use crate::next::toml::read::{Named, toml_key};
use crate::next::units::Scalar;
use crate::next::value::{SchemeMap, SurfaceMap, Value};
use crate::next::widgets::{
    ButtonPatch, PartPatch, TypedPart, WidgetKind, WidgetValuesPatch, WidgetsPatch,
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
    let mut problems = in_flavor(layer, cx, None);
    // Under each flavour, the declarations in force are the default
    // section's with that flavour's keys replacing it (S10): the bases are
    // checked against each, a rule against those of the flavours it can
    // apply under (S12's combinations under each `flavor:` condition).
    let flavors = &cx.declarations.flavors;
    if flavors.len() > 1 {
        let union = problems.len();
        for flavor in flavors {
            for problem in in_flavor(layer, cx, Some(flavor)) {
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
fn in_flavor(layer: &Layer, cx: &Context<'_>, flavor: Option<&Key>) -> Vec<Problem> {
    let mut checker = Checker::new(cx, matches!(layer.spelled, Spelled::Toml));
    checker.flavor = flavor;
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
    checker.problems
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
    /// A typed widget's part.
    Part(WidgetKind, TypedPart, &'static PartInfo),
    /// A typed widget's values.
    Values(WidgetKind),
}

/// A declaration a keyed name is checked against (S10): its name in
/// `[declare]`, whether a problem writes the name `@<key>` (an enum's keyed
/// variant), and whether the name is a map's key, which a base states.
#[derive(Clone, Copy, Debug)]
struct Declaration {
    name: &'static str,
    keyed: bool,
    map: bool,
}

/// The keyed names' declarations by the slot holding them (S10): the owner
/// and the field or variant, the declaration, whether a problem writes the
/// name `@<key>` (an enum's keyed variant), and whether the slot is a map
/// whose keys a base states. One table for the checks, for what a preset
/// uses and for a key field's type.
const DECLARATIONS: [(&str, &str, &str, bool, bool); 25] = [
    ("ColorRoles", "keyed", "color_roles", false, true),
    ("GlobalRoles", "accent_ramp", "accent_ramp", false, true),
    ("TextTokens", "keyed", "text_styles", false, true),
    ("Tokens", "elevation", "elevation", false, true),
    ("MotionTokens", "durations", "durations", false, true),
    ("MotionTokens", "timings", "timings", false, true),
    ("PointerTokens", "drag_cursors", "drag_cursors", false, true),
    ("IconStyle", "status_colors", "status_colors", false, true),
    ("IconSizeTokens", "keyed", "icon_contexts", false, true),
    ("Tokens", "params", "params", false, true),
    ("IconStyle", "scale", "symbol_scales", false, false),
    (
        "IconStyle",
        "state_effect",
        "icon_state_effects",
        false,
        false,
    ),
    (
        "IconStyle",
        "symbol_variants",
        "symbol_variants",
        false,
        false,
    ),
    ("TextRendering", "engine", "rendering_engines", false, false),
    ("TextRendering", "lcd_filter", "lcd_filters", false, false),
    (
        "MaterialScheme",
        "spec_version",
        "spec_versions",
        false,
        false,
    ),
    ("Material", "identity", "materials", false, false),
    ("Shadow", "elevation", "elevation", false, false),
    ("GlobalRole", "accent_ramp", "accent_ramp", false, false),
    ("BlendMode", "keyed", "blend_modes", true, false),
    ("FocusKind", "keyed", "focus_kinds", true, false),
    (
        "PresentationEvent",
        "keyed",
        "presentation_events",
        true,
        false,
    ),
    ("RoleRef", "keyed", "color_roles", true, false),
    ("TransitionKind", "key", "transition_kinds", true, false),
    ("PresentationKind", "key", "presentation_kinds", true, false),
];

/// The declaration a keyed name in the slot `slot` (a field or an enum's
/// variant, owner and name) is checked against: a map's key by the map's
/// field, a `Key` leaf by its field or variant (S10).
fn declaration_of(slot: Option<(&'static str, &'static str)>) -> Option<Declaration> {
    let (owner, field) = slot?;
    DECLARATIONS
        .iter()
        .find(|(o, f, ..)| *o == owner && *f == field)
        .map(|(_, _, name, keyed, map)| Declaration {
            name,
            keyed: *keyed,
            map: *map,
        })
}

/// The declaration a `Key` field named `field` holds a key of, whichever
/// block holds it: a reference to the field has that key's type (S2.2).
fn declaration_of_field(field: &str) -> Option<&'static str> {
    DECLARATIONS
        .iter()
        .find(|(_, f, _, keyed, map)| *f == field && !keyed && !map)
        .map(|(_, _, name, ..)| *name)
}

/// The keys the declaration `name` lists; `params`' are its groups' names,
/// a keyed kind's its kinds'.
fn declared<'a>(decl: &'a Declarations, name: &str) -> Vec<&'a Key> {
    let keys: &[Key] = match name {
        "color_roles" => &decl.color_roles,
        "accent_ramp" => &decl.accent_ramp,
        "text_styles" => &decl.text_styles,
        "elevation" => &decl.elevation,
        "durations" => &decl.durations,
        "timings" => &decl.timings,
        "drag_cursors" => &decl.drag_cursors,
        "status_colors" => &decl.status_colors,
        "icon_contexts" => &decl.icon_contexts,
        "symbol_scales" => &decl.symbol_scales,
        "icon_state_effects" => &decl.icon_state_effects,
        "symbol_variants" => &decl.symbol_variants,
        "rendering_engines" => &decl.rendering_engines,
        "lcd_filters" => &decl.lcd_filters,
        "spec_versions" => &decl.spec_versions,
        "materials" => &decl.materials,
        "blend_modes" => &decl.blend_modes,
        "focus_kinds" => &decl.focus_kinds,
        "presentation_events" => &decl.presentation_events,
        "transition_kinds" => return decl.transition_kinds.iter().map(|(k, _)| k).collect(),
        "presentation_kinds" => return decl.presentation_kinds.iter().map(|(k, _)| k).collect(),
        "params" => return decl.params.iter().map(|(k, _)| k).collect(),
        _ => &[],
    };
    keys.iter().collect()
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
    /// stated (S2.2, S3).
    fn surfaces_stated<T>(&mut self, sets: Option<&SurfaceMap<T>>, at: &str) {
        for set in &self.decl().surface_sets {
            if sets.is_none_or(|sets| sets.get(*set).is_none()) {
                self.problem(&join(at, set.name()), ProblemKind::Missing);
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
                    self.problem(&join(at, part.name), ProblemKind::Missing);
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
    /// style only of the part's fields, a `DrawnAs` of an available widget
    /// and declared variants.
    fn part(&mut self, part: TypedPart, patch: &PartPatch, at: &str) {
        let widget = widget_of(part);
        let Some(info) = part_info(part) else {
            return;
        };
        match patch {
            PartPatch::Absent => {
                if info.root || !info.may_be_absent {
                    self.problem(at, ProblemKind::AbsentNotAllowed);
                }
            }
            PartPatch::Style(style) => {
                self.part_fields(style, widget, info, at);
                self.region = Region::Part(widget, part, info);
                let toml = self.toml;
                let mut walker = Walker::new(at, toml, self);
                walker.part(info.fields, |w| w.walk(style));
            }
            PartPatch::DrawnAs { target, variants } => self.drawn_as(target, variants, at),
        }
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
    fn widget_decls(&self, widget: WidgetKind) -> std::vec::IntoIter<WidgetDecl> {
        match self.flavor {
            Some(flavor) => vec![self.decl().in_flavor(widget, flavor)].into_iter(),
            None => self.decl().in_each_flavor(widget).into_iter(),
        }
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
        let declared = self
            .widget_decls(widget)
            .any(|decl| decl.variants.as_ref().is_some_and(|v| v.contains(variant)));
        if !declared {
            let declaration = self.declaration(widget, "variants");
            self.undeclared(at, &variant_name(variant), &declaration);
        }
    }

    fn flag(&mut self, widget: WidgetKind, flag: &StateFlag, at: &str) {
        let StateFlag::Widget(key) = flag else {
            return;
        };
        let declared = self
            .widget_decls(widget)
            .any(|decl| decl.flags.as_ref().is_some_and(|f| f.contains(flag)));
        if !declared {
            let declaration = self.declaration(widget, "flags");
            self.undeclared(at, &format!("@{key}"), &declaration);
        }
    }

    fn size(&mut self, widget: WidgetKind, size: ControlSize, at: &str) {
        let declared = self.widget_decls(widget).any(|decl| {
            decl.sizes.as_ref().is_some_and(|s| s.contains(&size))
                || decl
                    .sizes_by_context
                    .iter()
                    .flatten()
                    .any(|entry| entry.sizes.contains(&size))
                || decl
                    .sizes_by_variant
                    .iter()
                    .flatten()
                    .any(|entry| entry.sizes.contains(&size))
        });
        if !declared {
            let declaration = self.declaration(widget, "sizes");
            self.undeclared(at, size.name(), &declaration);
        }
    }

    fn density(&mut self, widget: WidgetKind, density: &Density, at: &str) {
        let declared = self
            .widget_decls(widget)
            .any(|decl| decl.densities.as_ref().is_some_and(|d| d.contains(density)));
        if !declared {
            let name = match density {
                Density::Default => "default".to_owned(),
                Density::Compact => "compact".to_owned(),
                Density::Keyed(key) => format!("@{key}"),
            };
            let declaration = self.declaration(widget, "densities");
            self.undeclared(at, &name, &declaration);
        }
    }

    fn rule(&mut self, rule: &Rule, at: &str) {
        let when = join(at, "when");
        let set = join(at, "set");
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
                self.undeclared(&join(at, "target"), &name, "component");
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
    /// index (`rules[2].when.not_states[0]`, S11).
    fn selector(&mut self, selector: &Selector, widget: Option<WidgetKind>, at: &str) {
        for (condition, item) in items(&selector.conditions, at, "conditions") {
            self.condition(condition, &item);
        }
        if let Some(widget) = widget {
            self.element(widget, &selector.variants, &selector.states, at);
            for (size, item) in items(&selector.size.sizes, at, "sizes") {
                self.size(widget, *size, &item);
            }
            for (density, item) in items(&selector.size.densities, at, "densities") {
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

    /// An element's variants and flags, of `widget`.
    fn element(
        &mut self,
        widget: WidgetKind,
        variants: &SetMatch<VariantRef>,
        states: &SetMatch<StateFlag>,
        at: &str,
    ) {
        for (variant, item) in items(variants, at, "variants") {
            self.variant(widget, variant, &item);
        }
        for (flag, item) in items(states, at, "states") {
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
        for (size, item) in items(&context.sizes, at, "sizes") {
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

    fn condition(&mut self, condition: &Condition, at: &str) {
        let decl = self.decl();
        let (name, declaration, declared) = match condition {
            Condition::SystemSetting(key) => (
                key,
                "conditions",
                decl.conditions.iter().any(|setting| setting.key == *key),
            ),
            Condition::DeviceModel(key) => (key, "device_models", decl.device_models.contains(key)),
            Condition::Flavor(key) => (key, "flavors", decl.flavors.contains(key)),
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
            self.undeclared(at, &format!("@{name}"), declaration);
        }
    }

    /// A surface set, declared (S10); a palette's is skipped where the
    /// current preset does not declare it, not refused (S12, S14).
    fn surface_set(&mut self, set: SurfaceSet, at: &str) {
        if !self.decl().surface_sets.contains(&set) && self.cx.kind != LayerKind::Palette {
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
        for (name, value) in &inputs.entries {
            let path = join(at, &name.to_string());
            if !decl.inputs.contains(name) {
                self.undeclared(&path, &name.to_string(), "inputs");
            }
            if self.cx.kind == LayerKind::Application && decl.accent_inputs.contains(name) {
                self.problem(&path, ProblemKind::AccentInput);
            }
            self.input_values(value, &path);
        }
        for (list, map) in [
            ("groups", &inputs.kde_groups),
            ("inactive_groups", &inputs.kde_inactive_groups),
        ] {
            for (scheme, sets) in schemes(map) {
                let path = format!("{at}.kde_scheme.{list}.{scheme}");
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
        for (scheme, names) in schemes(&inputs.kde_unset_keys) {
            let path = format!("{at}.kde_scheme.unset_keys.{scheme}");
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
        // A bool, a text, a key or an enum takes a literal or a reference, a
        // block (an enum block among them) a reference only: no call.
        if form != ExprForm::Calls && matches!(expr, Expr::Call(..)) {
            self.problem(&at, ProblemKind::CallNotAllowed);
        }
        let in_text_style = path.fields().any(|(owner, _)| owner == "TextStyle");
        self.expr_node(expr, &at, in_text_style);
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
            return Some(Ty::Key(Cow::Borrowed(declaration.name)));
        }
        match (self.region, path.first_field()) {
            (Region::Values(widget), Some((_, value))) => Some(widget_keys(widget, value)),
            _ => None,
        }
    }

    /// One node's checks, then its arguments'; `expression` has bounded the
    /// nesting by [`MAX_DEPTH`].
    fn expr_node(&mut self, expr: &Expr, at: &str, in_text_style: bool) {
        match expr {
            Expr::Lit(_) => {}
            Expr::Ref(reference) => self.reference(reference, at, in_text_style),
            Expr::Call(func, args) => {
                self.call(*func, args, at);
                for arg in args {
                    self.expr_node(arg, at, in_text_style);
                }
            }
        }
    }

    fn call(&mut self, func: Func, args: &[Expr], at: &str) {
        match func {
            Func::Pick if args.len() < 2 => self.problem(at, ProblemKind::PickWithoutColors),
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
            }
            _ => {}
        }
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
            Region::Part(widget, part, info) => {
                self.element_data(reference, widget, DataOwner::Part(part), at);
                if let Ref::Widget { part: other, .. } = reference {
                    self.same_element(widget, info, other, at);
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
        let declared = self.widget_decls(widget).any(|decl| {
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

    /// A `widget.<part>` reference reads a part of the element its own part
    /// is queried with (S2.2).
    fn same_element(&mut self, widget: WidgetKind, info: &'static PartInfo, other: &Key, at: &str) {
        let Some(widget_info) = registry::widgets().iter().find(|w| w.kind == widget) else {
            return;
        };
        self.same_element_in(widget_info.parts, info, other, at);
    }

    /// A `widget.<other>` reference stated on `info`, one of `parts`, reads
    /// a part of the same element (S2.2).
    fn same_element_in(
        &mut self,
        parts: &'static [PartInfo],
        info: &'static PartInfo,
        other: &Key,
        at: &str,
    ) {
        if let Some(part) = other_element(parts, info, other.as_str()) {
            self.problem(at, ProblemKind::OtherElementsPart { part });
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
                        (Some("motion"), Some("timings")) => {
                            Some((&decl.timings, "timings", path.get(2)))
                        }
                        (Some("motion"), Some("durations")) => {
                            Some((&decl.durations, "durations", path.get(2)))
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
        let listed = if declaration.name == "params" {
            match path.keys().as_slice() {
                [] => decl.params.iter().any(|(group, _)| *group == key),
                [group, ..] => decl
                    .params
                    .iter()
                    .find(|(held, _)| held.as_str() == *group)
                    .is_none_or(|(_, params)| params.keys.contains(&key)),
            }
        } else {
            declared(decl, declaration.name).contains(&&key)
        };
        if !listed {
            self.undeclared(&at, key.as_str(), declaration.name);
        }
    }

    /// A stated keyed scalar's unit, its declaration's (S4.6, S10).
    fn scalar_unit(&mut self, path: &Path, scalar: &Scalar) {
        if let Some(unit) = self.declared_unit(path) {
            self.unit(&path.to_string(), scalar, unit);
        }
    }

    /// The declared unit of the keyed scalar at `path`: a `params` group's,
    /// or a material parameter's (S4.6, S10).
    fn declared_unit(&self, path: &Path) -> Option<ScalarUnit> {
        let decl = self.decl();
        match (path.last_field(), path.keys().as_slice()) {
            (Some(("Tokens", "params")), [group, _]) => group_unit(decl, group),
            (Some(("Material", "params")), [key]) => decl
                .material_params
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
        let Some((_, units)) = kinds.iter().find(|(held, _)| held == key) else {
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
    /// the value, every key refused where the widget lists none.
    fn declared_leaf(&mut self, path: &Path, key: &Key) {
        let decl = self.decl();
        let at = path.to_string();
        if let Some(declaration) = declaration_of(path.slot()) {
            if !declared(decl, declaration.name).contains(&key) {
                let name = if declaration.keyed {
                    format!("@{key}")
                } else {
                    key.to_string()
                };
                self.undeclared(&at, &name, declaration.name);
            }
            return;
        }
        if let (Region::Values(widget), Some((_, value))) = (self.region, path.first_field()) {
            let listed: Vec<Vec<Key>> = self
                .widget_decls(widget)
                .flat_map(|decl| decl.keys.unwrap_or_default())
                .filter(|(held, _)| *held == value)
                .map(|(_, keys)| keys)
                .collect();
            if !listed.iter().any(|keys| keys.contains(key)) {
                let declaration = self.declaration(widget, &format!("keys.{value}"));
                self.undeclared(&at, key.as_str(), &declaration);
            }
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

    /// A loop's keyframes' `at` increase (S2.7), and, in a widget's values,
    /// each keyframe names its part.
    fn keyframes(&mut self, keyframes: &[Keyframe], path: &Path) {
        let increasing = keyframes
            .iter()
            .zip(keyframes.iter().skip(1))
            .all(|(earlier, later)| earlier.at.0 < later.at.0);
        let at = path.join("keyframes");
        if !increasing {
            self.problem(&at, ProblemKind::KeyframesNotIncreasing);
        }
        if matches!(self.region, Region::Values(_)) {
            for (index, keyframe) in keyframes.iter().enumerate() {
                if keyframe.part.is_none() {
                    let at = format!("{at}[{index}].part");
                    self.problem(&at, ProblemKind::KeyframeWithoutPart);
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
            Node::Transition(transition) => {
                if transition.properties.is_empty() {
                    self.problem(&path.join("properties"), ProblemKind::NoProperties);
                }
            }
            Node::TransitionPatch(transition) => {
                if transition.properties.as_ref().is_some_and(Vec::is_empty) {
                    self.problem(&path.join("properties"), ProblemKind::NoProperties);
                }
            }
            Node::TransitionKind(kind) => {
                if let TransitionKind::Keyed { key, params } = kind {
                    let kinds = &decl.transition_kinds;
                    self.keyed_params(path, kinds, "transition_kinds", key, params);
                }
            }
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
                self.routes(&font.axes, &font.features, &["wght", "wdth"], path);
                self.weight(Some(&font.weight), path);
            }
            Node::FontPatch(font) => {
                let axes = font.axes.as_deref().unwrap_or_default();
                let features = font.features.as_deref().unwrap_or_default();
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
            Node::SurfaceSet(set) => self.surface_set(set, &at),
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
        if number == Number::Duration && field == Some(("ThemeChange", "duration")) && n == 0.0 {
            self.problem(&at, ProblemKind::ZeroDuration);
            return;
        }
        let expected = match (number, field) {
            (Number::Fraction, _) if !(0.0..=1.0).contains(&n) => Some("0..=1"),
            (Number::Length | Number::FontSize, _) if n < 0.0 && !signed => Some("at least zero"),
            (Number::Plain, Some(("Material", "saturation"))) if n < 0.0 => Some("at least zero"),
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
        if !(self.in_bases && self.cx.kind == LayerKind::Preset) || keyframe {
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
            self.problem(&path.join(field), ProblemKind::Missing);
        }
    }
}

/// What typing knows of the model where an expression is stated (S2.2):
/// the registry's types of the fields, tokens and roles a reference names,
/// and the preset's declarations.
struct Types<'a> {
    decl: &'a Declarations,
    region: Region,
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
                Region::Part(..) => Some(Ty::Length),
                Region::Roles | Region::Tokens | Region::Values(_) => None,
            },
            Ref::Part(path) => match self.region {
                Region::Part(_, _, info) => field_type(info.fields, &joined(path)),
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
    let info = registry::find(fields, path)?;
    match (info.value)() {
        Some(Ty::Key(set)) if set == KEY_OF_PLACE => {
            let name = path.rsplit('.').next()?;
            match declaration_of_field(name) {
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
/// `exclusive_variants` group naming a variant twice; a
/// `default_size_by_variant` entry naming an undeclared variant, a size the
/// variant does not take, or a variant an earlier entry names.
pub(super) fn declarations(decl: &Declarations) -> Vec<Problem> {
    // Each section's entries against the declarations in force where they
    // apply: the widget's own under the default flavour, a flavour's
    // section under that flavour, each key it leaves out the widget's own
    // (S10).
    let own = decl.widgets.iter().map(|(widget, section)| {
        let at = format!("declare.widget.{}", widget_name(*widget));
        (at, section, decl.in_default_flavor(*widget))
    });
    let flavors = decl.flavor.iter().flat_map(|(flavor, section)| {
        section.widgets.iter().map(move |(widget, section)| {
            let at = format!("declare.flavor.{flavor}.widget.{}", widget_name(*widget));
            (at, section, decl.in_flavor(*widget, flavor))
        })
    });
    let mut problems = Vec::new();
    for (at, section, in_force) in own.chain(flavors) {
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
        let entries: Vec<_> = section.default_size_by_variant.iter().flatten().collect();
        for (index, entry) in entries.iter().enumerate() {
            let item = format!("{at}.default_size_by_variant[{index}]");
            let name = variant_name(&entry.variant);
            if entries
                .iter()
                .take(index)
                .any(|earlier| earlier.variant == entry.variant)
            {
                let kind = ProblemKind::ListedTwice { name: name.clone() };
                problems.push(Problem::new(&item, kind));
            }
            let declared = in_force
                .variants
                .as_ref()
                .is_some_and(|variants| variants.contains(&entry.variant));
            if !declared {
                let declaration = format!("{at}.variants");
                let kind = ProblemKind::Undeclared { name, declaration };
                problems.push(Problem::new(&join(&item, "variant"), kind));
            }
            let sizes = in_force
                .sizes_by_variant
                .iter()
                .flatten()
                .find(|by| by.variant == entry.variant)
                .map(|by| &by.sizes)
                .or(in_force.sizes.as_ref());
            if !sizes.is_some_and(|sizes| sizes.contains(&entry.size)) {
                let size = entry.size.name().to_owned();
                problems.push(Problem::new(
                    &join(&item, "size"),
                    ProblemKind::SizeNotTaken { size },
                ));
            }
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
    /// The variants rules select on or draw a part as, by the widget they
    /// are of; `None`: a context or sibling matched by variant alone, of
    /// any widget.
    variants: Vec<(Option<WidgetKind>, VariantRef)>,
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
        if declaration.name == "params" {
            if let Some(group) = path.keys().first() {
                self.stated.push(("params", format!("{group}.{key}")));
            }
            return;
        }
        self.stated.push((declaration.name, key.to_owned()));
    }
}

impl Uses {
    /// A rule's selector, the rule targeting a part or values of `widget`.
    fn selector(&mut self, selector: &Selector, widget: Option<WidgetKind>) {
        let conditions = &selector.conditions;
        self.conditions
            .extend(conditions.all_of.iter().chain(&conditions.none_of).cloned());
        if widget.is_some() {
            self.variants_of(widget, &selector.variants);
        }
        for context in selector.context.iter().chain(&selector.not_context) {
            let widget = target_widget(context.target.as_ref());
            self.variants_of(widget, &context.variants);
        }
        if let Some(position) = &selector.position {
            for sibling in [&position.previous, &position.next].into_iter().flatten() {
                let widget = target_widget(sibling.target.as_ref());
                self.variants_of(widget, &sibling.variants);
            }
        }
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
    }

    fn variants_of(&mut self, widget: Option<WidgetKind>, set: &SetMatch<VariantRef>) {
        let named = set.all_of.iter().chain(&set.none_of);
        self.variants
            .extend(named.map(|variant| (widget, variant.clone())));
    }

    /// A part's patch: its style's keyed names, the variants a `DrawnAs`
    /// draws it as.
    fn part(&mut self, patch: &PartPatch) {
        match patch {
            PartPatch::Style(style) => self.walk(style),
            PartPatch::Absent => {}
            PartPatch::DrawnAs { target, variants } => {
                let widget = target_widget(Some(target));
                self.variants
                    .extend(variants.iter().map(|variant| (widget, variant.clone())));
            }
        }
    }

    /// Whether a rule selects on, or a part is drawn as, `variant` of
    /// `widget`.
    fn uses_variant(&self, widget: WidgetKind, variant: &VariantRef) -> bool {
        self.variants
            .iter()
            .any(|(held, used)| held.is_none_or(|held| held == widget) && used == variant)
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

/// Each declared keyed name nothing uses (S12): a keyed flag, a keyed
/// variant, a system setting, a device model, a flavour or a container size
/// no rule selects on (but the default flavour, each container's default
/// size, a setting for applications, and the variants `exempt_variant` and
/// `content_variants` name), a keyed name no base states, an event no
/// `presentations` list names.
pub(super) fn unused(decl: &Declarations, layer: &Layer, preset: Option<PresetId>) -> Vec<Problem> {
    let mut uses = Uses::default();
    uses.bases(&layer.bases);
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
    let flavors = decl.flavor.iter().flat_map(|(flavor, section)| {
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
        for variant in section.variants.iter().flatten() {
            if let VariantRef::Keyed(key) = variant
                && !uses.uses_variant(*widget, variant)
                && !exempt_variant(decl, *widget, variant)
                && !content_variants(*widget).contains(&key.as_str())
            {
                unused.push((format!("{at}.variants"), format!("@{key}")));
            }
        }
    }
    for model in &decl.device_models {
        if !uses
            .conditions
            .contains(&Condition::DeviceModel(model.clone()))
        {
            unused.push(("declare.device_models".to_owned(), format!("@{model}")));
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
    for flavor in decl.flavors.iter().skip(1) {
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
    let stated = [
        "color_roles",
        "accent_ramp",
        "text_styles",
        "elevation",
        "durations",
        "timings",
        "drag_cursors",
        "icon_contexts",
        "status_colors",
    ];
    for declaration in stated {
        for key in declared(decl, declaration) {
            let used = uses
                .stated
                .iter()
                .any(|(held, name)| *held == declaration && name == key.as_str());
            if !used {
                unused.push((format!("declare.{declaration}"), key.to_string()));
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

/// A variant no rule needs to select on (S12): one `requires_variants`,
/// `sizes_by_variant` or a breakpoint names, in any section of the widget.
fn exempt_variant(decl: &Declarations, widget: WidgetKind, variant: &VariantRef) -> bool {
    let sections = decl.widgets.iter().chain(
        decl.flavor
            .iter()
            .flat_map(|(_, flavor)| flavor.widgets.iter()),
    );
    sections
        .filter(|(held, _)| *held == widget)
        .any(|(_, section)| {
            section
                .requires_variants
                .iter()
                .flatten()
                .any(|held| held == variant)
                || section
                    .sizes_by_variant
                    .iter()
                    .flatten()
                    .any(|by| by.variant == *variant)
                || section
                    .breakpoints
                    .iter()
                    .flatten()
                    .any(|breakpoint| breakpoint.variant == *variant)
        })
}

/// A widget's keyed content variants, which the connector sets by one
/// platform's rule under every preset (S8; the catalogue's content-variant
/// pairs, *Entry format*, ADD-A123): no rule need select on them.
fn content_variants(widget: WidgetKind) -> &'static [&'static str] {
    match widget {
        WidgetKind::Button => &["custom_child", "arrow_only", "with_arrow"],
    }
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

/// A set test's items, each at its `when` key and index: `all_of` under
/// `name`, `none_of` under `not_<name>` (S11).
fn items<'s, T>(set: &'s SetMatch<T>, at: &str, name: &str) -> Vec<(&'s T, String)> {
    let all = set
        .all_of
        .iter()
        .enumerate()
        .map(|(index, item)| (item, format!("{at}.{name}[{index}]")));
    let none = set
        .none_of
        .iter()
        .enumerate()
        .map(|(index, item)| (item, format!("{at}.not_{name}[{index}]")));
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

fn widget_name(widget: WidgetKind) -> &'static str {
    registry::widgets()
        .iter()
        .find(|info| info.kind == widget)
        .map_or("", |info| info.name)
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
/// hold a loop) and an *(element)* part's reference to its host (`button`
/// has no *(element)* part).
#[cfg(test)]
pub(super) mod triggers {
    use super::tests::{PARTS, looped};
    use super::*;

    fn checker_over<'a>(cx: &'a Context<'a>) -> Checker<'a> {
        Checker::new(cx, false)
    }

    /// A keyframe of a loop stated as a widget's value with no part.
    pub(in crate::next::validate) fn keyframe_without_part() -> Vec<Problem> {
        let decl = Declarations::default();
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Application,
            below: &[],
        };
        let mut checker = checker_over(&cx);
        checker.walk("probe", Region::Values(WidgetKind::Button), &looped(None));
        checker.problems
    }

    /// A synthetic *(element)* part's mark reading its host's label.
    pub(in crate::next::validate) fn other_elements_part() -> Vec<Problem> {
        let decl = Declarations::default();
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Application,
            below: &[],
        };
        let mut checker = checker_over(&cx);
        if let Some(mark) = PARTS.iter().find(|part| part.name == "mark") {
            checker.same_element_in(&PARTS, mark, &crate::key!("label"), "probe");
        }
        checker.problems
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::key;
    use crate::next::blocks::motion::Keyframe;
    use crate::next::blocks::motion::Loop;
    use crate::next::blocks::part_style::PartStylePatch;
    use crate::next::registry::FieldInfo;
    use crate::next::units::{Fraction, Ms};
    use crate::next::validate::walk::Marks;
    use crate::next::widgets::ButtonPart;

    const NO_PROBLEM: [&str; 0] = [];

    /// The problems of `looped` walked in `region`.
    pub(super) fn walked(looped: &Loop, region: Region) -> Vec<String> {
        let decl = Declarations::default();
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Application,
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
        // The row's mark reads the host's label: another element.
        assert_eq!(
            other_element(&PARTS, part("mark"), "label"),
            Some("label".to_owned())
        );
        // The host's label reads the row's mark: another element.
        assert_eq!(
            other_element(&PARTS, part("label"), "mark"),
            Some("mark".to_owned())
        );
        // Within one element: the mark reads its row, the label the body.
        assert_eq!(other_element(&PARTS, part("mark"), "row"), None);
        assert_eq!(other_element(&PARTS, part("label"), "body"), None);
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

    /// A rule names no preset here: no preset is bundled, so `PresetId` has
    /// no value to name (S16). The test that a rule naming another preset
    /// is skipped enters with the first bundled preset (M1a task 15).
    #[test]
    fn a_rule_naming_no_preset_is_validated() {
        let rule = Rule::new(
            Selector::default(),
            RuleBody::Tokens(crate::next::tokens::TokensPatch::default()),
        );
        assert!(!excluded(&rule, None));
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
}
