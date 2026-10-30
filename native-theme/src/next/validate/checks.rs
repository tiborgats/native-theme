//! S12's structural checks: each value, patch, rule and declaration on its
//! own, every problem collected.

use std::any::Any;

use super::walk::{ExprForm, Form, Number, Path, Visit, Walk, Walker};
use super::{Context, LayerKind};
use crate::next::blocks::BlockPatch;
use crate::next::blocks::focus::FocusKind;
use crate::next::blocks::icon::{IconStyle, IconStylePatch};
use crate::next::blocks::motion::{
    Keyframe, Loop, LoopPatch, PresentationEvent, PresentationKind, StepJump, Timing, Transition,
    TransitionKind, TransitionPatch,
};
use crate::next::blocks::outline::Radius;
use crate::next::blocks::paint::{
    BlendMode, Material, MaterialPatch, Paint, PaintKind, PaintPatch,
};
use crate::next::blocks::part_style::{Binding, PartStylePatch, RoleRef};
use crate::next::blocks::shadow::Shadow;
use crate::next::blocks::text::{AxisTag, AxisValue, FeatureTag, Font, FontPatch};
use crate::next::declare::{DataName, DataOwner, Declarations, FontMetric, ScalarUnit, WidgetDecl};
use crate::next::error::{Problem, ProblemKind};
use crate::next::expr::{ElementDatum, Expr, Func, Literal, MAX_DEPTH, Name, Ref};
use crate::next::inputs::{InputBases, InputName};
use crate::next::key::Key;
use crate::next::layer::{Bases, Layer};
use crate::next::registry::{self, PartInfo};
use crate::next::roles::{GlobalRole, SurfaceRoles, SurfaceSet};
use crate::next::selector::{
    Condition, ContextMatch, ControlSize, Density, PartRef, PositionMatch, Rule, RuleBody,
    Selector, SiblingMatch, StateFlag, ValuesPatch, VariantRef, WidgetOrPart, WidgetRef,
};
use crate::next::tokens::ThemeChange;
use crate::next::toml::read::Named;
use crate::next::units::{Ms, Scalar};
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
    let mut checker = Checker::new(cx);
    checker.bases(&layer.bases, "");
    for (preset, bases) in &layer.for_preset {
        if cx.preset == Some(*preset) {
            checker.bases(bases, &format!("for_preset.{}", preset.key()));
        }
    }
    for (index, rule) in layer.rules.iter().enumerate() {
        checker.rule(rule, &format!("rules[{index}]"));
    }
    checker.problems
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

struct Checker<'a> {
    cx: &'a Context<'a>,
    problems: Vec<Problem>,
    region: Region,
    /// Walking a layer's bases, where `base` is an error.
    in_bases: bool,
}

impl<'a> Checker<'a> {
    fn new(cx: &'a Context<'a>) -> Checker<'a> {
        Checker {
            cx,
            problems: Vec::new(),
            region: Region::Tokens,
            in_bases: false,
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
        let mut walker = Walker::new(root, self);
        walker.walk(value);
    }

    fn bases(&mut self, bases: &Bases, at: &str) {
        self.in_bases = true;
        let colors = join(at, "colors");
        let whole = self.cx.kind == LayerKind::Preset;
        for (scheme, roles) in [("light", &bases.colors.light), ("dark", &bases.colors.dark)] {
            match roles {
                Some(roles) => self.walk(&join(&colors, scheme), Region::Roles, roles),
                None if whole => self.problem(&join(&colors, scheme), ProblemKind::Missing),
                None => {}
            }
        }
        self.inputs(&bases.inputs, &join(at, "inputs"));
        self.walk(&join(at, "tokens"), Region::Tokens, &bases.tokens);
        self.widgets(&bases.widgets, &join(at, "widgets"));
        self.in_bases = false;
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
        let available = !self
            .widget_decls(widget)
            .any(|decl| decl.available == Some(false));
        if self.cx.kind == LayerKind::Preset && available {
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
                let mut walker = Walker::new(at, self);
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

    fn drawn_as(&mut self, target: &WidgetOrPart, variants: &[VariantRef], at: &str) {
        let widget = match target {
            WidgetOrPart::Widget(WidgetRef::Typed(widget)) => Some(*widget),
            WidgetOrPart::Part(PartRef::Typed(part)) => Some(widget_of(*part)),
            WidgetOrPart::Widget(WidgetRef::Component(key))
            | WidgetOrPart::Part(PartRef::Component { key, .. }) => {
                self.undeclared(
                    &join(at, "target"),
                    &format!("component.{key}"),
                    "component",
                );
                None
            }
        };
        let Some(widget) = widget else { return };
        if self
            .widget_decls(widget)
            .any(|decl| decl.available == Some(false))
        {
            self.problem(
                &join(at, "target"),
                ProblemKind::WidgetUnavailable {
                    widget: widget_name(widget),
                },
            );
        }
        for (index, variant) in variants.iter().enumerate() {
            self.variant(widget, variant, &format!("{at}.variants[{index}]"));
        }
    }

    /// The widget's declarations: its own section and each flavour's.
    fn widget_decls(&self, widget: WidgetKind) -> impl Iterator<Item = &'a WidgetDecl> + use<'a> {
        let decl = self.decl();
        decl.widgets
            .iter()
            .chain(
                decl.flavor
                    .iter()
                    .flat_map(|(_, flavor)| flavor.widgets.iter()),
            )
            .filter(move |(kind, _)| *kind == widget)
            .map(|(_, decl)| decl)
    }

    fn variant(&mut self, widget: WidgetKind, variant: &VariantRef, at: &str) {
        let declared = self
            .widget_decls(widget)
            .any(|decl| decl.variants.as_ref().is_some_and(|v| v.contains(variant)));
        if !declared {
            let declaration = format!("widget.{}.variants", widget_name(widget));
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
            let declaration = format!("widget.{}.flags", widget_name(widget));
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
            let declaration = format!("widget.{}.sizes", widget_name(widget));
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
            let declaration = format!("widget.{}.densities", widget_name(widget));
            self.undeclared(at, &name, &declaration);
        }
    }

    fn rule(&mut self, rule: &Rule, at: &str) {
        let when = join(at, "when");
        let set = join(at, "set");
        let widget = match &rule.body {
            RuleBody::Colors(roles) => {
                self.conditions_only(&rule.selector, &when);
                self.walk(&set, Region::Roles, roles);
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

    /// A selector's names, each declared (S10).
    fn selector(&mut self, selector: &Selector, widget: Option<WidgetKind>, at: &str) {
        let conditions = &selector.conditions;
        for condition in conditions.all_of.iter().chain(&conditions.none_of) {
            self.condition(condition, &join(at, "conditions"));
        }
        if let Some(widget) = widget {
            self.element(
                widget,
                [&selector.variants.all_of, &selector.variants.none_of],
                [&selector.states.all_of, &selector.states.none_of],
                at,
            );
            let sizes = &selector.size.sizes;
            for size in sizes.all_of.iter().chain(&sizes.none_of) {
                self.size(widget, *size, &join(at, "sizes"));
            }
            let densities = &selector.size.densities;
            for density in densities.all_of.iter().chain(&densities.none_of) {
                self.density(widget, density, &join(at, "densities"));
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
        variants: [&Vec<VariantRef>; 2],
        states: [&Vec<StateFlag>; 2],
        at: &str,
    ) {
        for variant in variants.into_iter().flatten() {
            self.variant(widget, variant, &join(at, "variants"));
        }
        for flag in states.into_iter().flatten() {
            self.flag(widget, flag, &join(at, "states"));
        }
    }

    fn context(&mut self, context: &ContextMatch, at: &str) {
        if let Some(surface) = context.surface {
            self.surface_set(surface, &join(at, "surface"));
        }
        let Some(widget) = target_widget(context.target.as_ref()) else {
            return;
        };
        self.element(
            widget,
            [&context.variants.all_of, &context.variants.none_of],
            [&context.states.all_of, &context.states.none_of],
            at,
        );
        for size in context.sizes.all_of.iter().chain(&context.sizes.none_of) {
            self.size(widget, *size, &join(at, "sizes"));
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
        self.element(
            widget,
            [&sibling.variants.all_of, &sibling.variants.none_of],
            [&sibling.states.all_of, &sibling.states.none_of],
            at,
        );
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

    fn surface_set(&mut self, set: SurfaceSet, at: &str) {
        if !self.decl().surface_sets.contains(&set) {
            self.undeclared(at, set.name(), "surface_sets");
        }
    }

    /// The inputs' bases (S9.1): each declared, an accent input only in a
    /// palette's or a reader's layer, KDE's lists naming each set or key
    /// once and only declared ones.
    fn inputs(&mut self, inputs: &InputBases, at: &str) {
        let decl = self.decl();
        for (name, _) in &inputs.entries {
            let path = join(at, &name.to_string());
            if !decl.inputs.contains(name) {
                self.undeclared(&path, &name.to_string(), "inputs");
            }
            if self.cx.kind == LayerKind::Application && decl.accent_inputs.contains(name) {
                self.problem(&path, ProblemKind::AccentInput);
            }
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

    /// An expression's structural checks (S2.2, S3.3, S10, S12).
    fn expression(&mut self, path: &Path, expr: &Expr, form: ExprForm) {
        let at = path.to_string();
        if form == ExprForm::NoCalls && matches!(expr, Expr::Call(..)) {
            self.problem(&at, ProblemKind::CallNotAllowed);
        }
        if depth(expr) > MAX_DEPTH {
            self.problem(&at, ProblemKind::TooDeep);
        }
        let in_text_style = path.fields().any(|(owner, _)| owner == "TextStyle");
        self.expr_node(expr, &at, in_text_style);
    }

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
        let Some(other_info) = widget_info.part(other.as_str()) else {
            return;
        };
        if element_of(widget_info.parts, info) != element_of(widget_info.parts, other_info) {
            let part = other.to_string();
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
    /// (S10).
    fn declared_key(&mut self, path: &Path, key: &str) {
        let decl = self.decl();
        let Ok(key) = Key::new(key) else {
            return;
        };
        let at = path.to_string();
        let declaration: Option<(&[Key], &str)> = match path.last_field() {
            Some(("ColorRoles", "keyed")) => Some((&decl.color_roles, "color_roles")),
            Some(("GlobalRoles", "accent_ramp")) => Some((&decl.accent_ramp, "accent_ramp")),
            Some(("TextTokens", "keyed")) => Some((&decl.text_styles, "text_styles")),
            Some(("Tokens", "elevation")) => Some((&decl.elevation, "elevation")),
            Some(("MotionTokens", "durations")) => Some((&decl.durations, "durations")),
            Some(("MotionTokens", "timings")) => Some((&decl.timings, "timings")),
            Some(("PointerTokens", "drag_cursors")) => Some((&decl.drag_cursors, "drag_cursors")),
            Some(("IconStyle", "status_colors")) => Some((&decl.status_colors, "status_colors")),
            Some(("IconSizeTokens", "keyed")) => Some((&decl.icon_contexts, "icon_contexts")),
            Some(("Tokens", "params")) => {
                let declared = match path.keys().as_slice() {
                    [] => decl.params.iter().any(|(group, _)| *group == key),
                    [group, ..] => decl.params.iter().any(|(held, params)| {
                        held.as_str() == *group && params.keys.contains(&key)
                    }),
                };
                if !declared {
                    self.undeclared(&join(&at, key.as_str()), key.as_str(), "params");
                }
                None
            }
            _ => None,
        };
        if let Some((names, declaration)) = declaration
            && !names.contains(&key)
        {
            self.undeclared(&join(&at, key.as_str()), key.as_str(), declaration);
        }
    }

    /// A keyed scalar's unit, its declaration's (S4.6, S10).
    fn scalar_unit(&mut self, path: &Path, scalar: &Scalar) {
        let decl = self.decl();
        let unit = match (path.last_field(), path.keys().as_slice()) {
            (Some(("Tokens", "params")), [group, ..]) => decl
                .params
                .iter()
                .find(|(held, _)| held.as_str() == *group)
                .map(|(_, group)| group.unit),
            (Some(("Material", "params")), [key, ..]) => decl
                .material_params
                .iter()
                .find(|(held, _)| held.as_str() == *key)
                .map(|(_, unit)| *unit),
            _ => None,
        };
        if let Some(unit) = unit {
            self.unit(&path.to_string(), scalar, unit);
        }
    }

    /// A stated keyed scalar against its declared unit: a suffixed one of
    /// that unit, a bare number of a `number` or `fraction` one, a
    /// fraction's within `0..=1`.
    fn unit(&mut self, at: &str, scalar: &Scalar, unit: ScalarUnit) {
        let fits = matches!(
            (scalar, unit),
            (Scalar::Px(_), ScalarUnit::Px)
                | (Scalar::Ms(_), ScalarUnit::Ms)
                | (Scalar::Deg(_), ScalarUnit::Deg)
                | (Scalar::Number(_), ScalarUnit::Number | ScalarUnit::Fraction)
                | (Scalar::Fraction(_), ScalarUnit::Fraction)
        );
        if !fits {
            let unit = unit_name(unit);
            self.problem(at, ProblemKind::UnitMismatch { unit });
        } else if let (Scalar::Number(n), ScalarUnit::Fraction) = (scalar, unit)
            && !(0.0..=1.0).contains(n)
        {
            self.problem(at, ProblemKind::OutOfRange { expected: "0..=1" });
        }
    }

    /// A keyed kind and its parameters: each declared, of its declared unit.
    fn keyed_params(
        &mut self,
        at: &str,
        kinds: &'a [(Key, Vec<(Key, ScalarUnit)>)],
        declaration: &str,
        key: &Key,
        params: &[(Key, Value<Scalar>)],
    ) {
        let Some((_, units)) = kinds.iter().find(|(held, _)| held == key) else {
            self.undeclared(at, &format!("@{key}"), declaration);
            return;
        };
        for (param, value) in params {
            let param_at = format!("{at}.params.{param}");
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

    /// A block's or a leaf's own checks.
    fn block(&mut self, path: &Path, node: &dyn Any) {
        let at = path.to_string();
        let decl = self.decl();
        if let Some(paint) = node.downcast_ref::<Paint>() {
            match paint.layers.first() {
                None => self.problem(&at, ProblemKind::NoLayer),
                Some(bottom) if bottom.blend == BlendMode::Mask => {
                    self.problem(&join(&at, "layers[0].blend"), ProblemKind::MaskAtBottom);
                }
                Some(_) => {}
            }
        } else if let Some(shadow) = node.downcast_ref::<Shadow>() {
            match shadow {
                Shadow::Layers(layers) if layers.is_empty() => {
                    self.problem(&at, ProblemKind::NoLayer);
                }
                Shadow::Elevation(key) if !decl.elevation.contains(key) => {
                    self.undeclared(&at, key.as_str(), "elevation");
                }
                Shadow::Layers(_) | Shadow::Elevation(_) => {}
            }
        } else if let Some(kind) = node.downcast_ref::<PaintKind>() {
            let stops = match kind {
                PaintKind::Linear { stops, .. }
                | PaintKind::Radial { stops, .. }
                | PaintKind::Conic { stops, .. } => Some(stops.len()),
                PaintKind::Color(_) | PaintKind::Pattern { .. } | PaintKind::Material(_) => None,
            };
            if stops.is_some_and(|stops| stops < 2) {
                self.problem(&at, ProblemKind::TooFewStops);
            }
        } else if let Some(transition) = node.downcast_ref::<Transition>() {
            if transition.properties.is_empty() {
                self.problem(&join(&at, "properties"), ProblemKind::NoProperties);
            }
        } else if let Some(transition) = node.downcast_ref::<TransitionPatch>() {
            if transition.properties.as_ref().is_some_and(Vec::is_empty) {
                self.problem(&join(&at, "properties"), ProblemKind::NoProperties);
            }
        } else if let Some(timing) = node.downcast_ref::<Timing>() {
            self.timing(timing, &at);
        } else if let Some(change) = node.downcast_ref::<ThemeChange>() {
            if let ThemeChange::Crossfade { duration, .. } | ThemeChange::Suspend { duration } =
                change
                && *duration == Value::Stated(Ms(0.0))
            {
                self.problem(&join(&at, "duration"), ProblemKind::ZeroDuration);
            }
        } else if let Some(looped) = node.downcast_ref::<Loop>() {
            self.keyframes(&looped.keyframes, &at);
        } else if let Some(looped) = node.downcast_ref::<LoopPatch>() {
            if let Some(keyframes) = &looped.keyframes {
                self.keyframes(keyframes, &at);
            }
        } else if let Some(keyframe) = node.downcast_ref::<Keyframe>() {
            if keyframe.part.is_none() && matches!(self.region, Region::Values(_)) {
                self.problem(&join(&at, "part"), ProblemKind::KeyframeWithoutPart);
            }
        } else if let Some(Radius::FollowPart) = node.downcast_ref::<Radius>() {
            if !path.within("FocusIndicator", "corners") {
                self.problem(&at, ProblemKind::FollowPartOutsideFocus);
            }
        } else if let Some(font) = node.downcast_ref::<Font>() {
            self.routes(&font.axes, &font.features, &["wght", "wdth"], &at);
        } else if let Some(font) = node.downcast_ref::<FontPatch>() {
            let axes = font.axes.as_deref().unwrap_or_default();
            let features = font.features.as_deref().unwrap_or_default();
            self.routes(axes, features, &["wght", "wdth"], &at);
        } else if let Some(style) = node.downcast_ref::<IconStyle>() {
            self.routes(&style.axes, &[], &["wght"], &at);
        } else if let Some(style) = node.downcast_ref::<IconStylePatch>() {
            self.routes(
                style.axes.as_deref().unwrap_or_default(),
                &[],
                &["wght"],
                &at,
            );
        } else if let Some(material) = node.downcast_ref::<Material>() {
            if !decl.materials.contains(&material.identity) {
                let name = material.identity.as_str();
                self.undeclared(&join(&at, "identity"), name, "materials");
            }
        } else if let Some(material) = node.downcast_ref::<MaterialPatch>() {
            if let Some(identity) = &material.identity
                && !decl.materials.contains(identity)
            {
                self.undeclared(&join(&at, "identity"), identity.as_str(), "materials");
            }
        } else if let Some(BlendMode::Keyed(key)) = node.downcast_ref::<BlendMode>() {
            if !decl.blend_modes.contains(key) {
                self.undeclared(&at, &format!("@{key}"), "blend_modes");
            }
        } else if let Some(FocusKind::Keyed(key)) = node.downcast_ref::<FocusKind>() {
            if !decl.focus_kinds.contains(key) {
                self.undeclared(&at, &format!("@{key}"), "focus_kinds");
            }
        } else if let Some(TransitionKind::Keyed { key, params }) =
            node.downcast_ref::<TransitionKind>()
        {
            let kinds = &decl.transition_kinds;
            self.keyed_params(&at, kinds, "transition_kinds", key, params);
        } else if let Some(PresentationKind::Keyed { key, params }) =
            node.downcast_ref::<PresentationKind>()
        {
            let kinds = &decl.presentation_kinds;
            self.keyed_params(&at, kinds, "presentation_kinds", key, params);
        } else if let Some(PresentationEvent::Keyed(key)) = node.downcast_ref::<PresentationEvent>()
        {
            if !decl.presentation_events.contains(key) {
                self.undeclared(&at, &format!("@{key}"), "presentation_events");
            }
        } else if let Some(role) = node.downcast_ref::<RoleRef>() {
            match role {
                RoleRef::Keyed(key) if !decl.color_roles.contains(key) => {
                    self.undeclared(&at, &format!("@{key}"), "color_roles");
                }
                RoleRef::Set(set, _) => self.surface_set(*set, &at),
                RoleRef::Global(GlobalRole::AccentRamp(key)) if !decl.accent_ramp.contains(key) => {
                    self.undeclared(&at, key.as_str(), "accent_ramp");
                }
                _ => {}
            }
        } else if let Some(Binding::Surface(set)) = node.downcast_ref::<Binding>() {
            self.surface_set(*set, &at);
        } else if let Some(sets) = node.downcast_ref::<SurfaceMap<BlockPatch<SurfaceRoles>>>() {
            for (set, _) in sets.iter() {
                self.surface_set(set, &join(&at, set.name()));
            }
            if self.in_bases && self.cx.kind == LayerKind::Preset {
                for set in &decl.surface_sets {
                    if sets.get(*set).is_none() {
                        self.problem(&join(&at, set.name()), ProblemKind::Missing);
                    }
                }
            }
        } else if let Some(PaintPatch::Append(_)) = node.downcast_ref::<PaintPatch>() {
            if self.in_bases {
                self.problem(&join(&at, "append"), ProblemKind::AppendInBases);
            }
        } else if let Some(sets) = node.downcast_ref::<SurfaceMap<SurfaceRoles>>() {
            for (set, _) in sets.iter() {
                self.surface_set(set, &join(&at, set.name()));
            }
        } else if let Some(scalar) = node.downcast_ref::<Scalar>() {
            self.scalar_unit(path, scalar);
        } else if let Some(key) = node.downcast_ref::<Key>() {
            self.declared_leaf(path, key, &at);
        }
    }

    /// A `Key` leaf, declared where its field names a declaration (S10).
    fn declared_leaf(&mut self, path: &Path, key: &Key, at: &str) {
        let decl = self.decl();
        let declaration: Option<(&[Key], &str)> = match path.last_field() {
            Some(("IconStyle", "scale")) => Some((&decl.symbol_scales, "symbol_scales")),
            Some(("IconStyle", "state_effect")) => {
                Some((&decl.icon_state_effects, "icon_state_effects"))
            }
            Some(("IconStyle", "symbol_variants")) => {
                Some((&decl.symbol_variants, "symbol_variants"))
            }
            Some(("TextRendering", "engine")) => {
                Some((&decl.rendering_engines, "rendering_engines"))
            }
            Some(("TextRendering", "lcd_filter")) => Some((&decl.lcd_filters, "lcd_filters")),
            Some(("MaterialScheme", "spec_version")) => {
                Some((&decl.spec_versions, "spec_versions"))
            }
            _ => None,
        };
        if let Some((names, declaration)) = declaration {
            if !names.contains(key) {
                self.undeclared(at, key.as_str(), declaration);
            }
            return;
        }
        if let (Region::Values(widget), Some((_, value))) = (self.region, path.first_field()) {
            let listed: Vec<&Vec<Key>> = self
                .widget_decls(widget)
                .flat_map(|decl| decl.keys.iter().flatten())
                .filter(|(held, _)| *held == value)
                .map(|(_, keys)| keys)
                .collect();
            if !listed.is_empty() && !listed.iter().any(|keys| keys.contains(key)) {
                let declaration = format!("widget.{}.keys.{value}", widget_name(widget));
                self.undeclared(at, key.as_str(), &declaration);
            }
        }
    }

    /// A timing's own constraints (S2.7).
    fn timing(&mut self, timing: &Timing, at: &str) {
        let mut checks: Vec<(String, bool, &'static str)> = Vec::new();
        match timing {
            Timing::CubicBezier(x1, _, x2, _) => {
                for (index, x) in [(0, x1), (2, x2)] {
                    let at = format!("{at}.cubic_bezier[{index}]");
                    checks.push((at, (0.0..=1.0).contains(x), "0..=1"));
                }
            }
            Timing::Steps { count, jump } => {
                let at = format!("{at}.steps.count");
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
                checks.push((format!("{at}.spring.mass"), *mass > 0.0, "above zero"));
                checks.push((
                    format!("{at}.spring.stiffness"),
                    *stiffness > 0.0,
                    "above zero",
                ));
                let ratio = *damping_ratio >= 0.0;
                checks.push((format!("{at}.spring.damping_ratio"), ratio, "at least zero"));
                let epsilon = *rest_epsilon > 0.0;
                checks.push((format!("{at}.spring.rest_epsilon"), epsilon, "above zero"));
            }
            Timing::Linear | Timing::Platform(_) => {}
        }
        for (at, ok, expected) in checks {
            if !ok {
                self.problem(&at, ProblemKind::OutOfRange { expected });
            }
        }
    }

    /// A loop's keyframes' `at` increase (S2.7).
    fn keyframes(&mut self, keyframes: &[Keyframe], at: &str) {
        let increasing = keyframes
            .iter()
            .zip(keyframes.iter().skip(1))
            .all(|(earlier, later)| earlier.at.0 < later.at.0);
        if !increasing {
            self.problem(&join(at, "keyframes"), ProblemKind::KeyframesNotIncreasing);
        }
    }

    /// Axes and features that duplicate a field's route (S2.8).
    fn routes(
        &mut self,
        axes: &[(AxisTag, AxisValue)],
        features: &[(FeatureTag, u32)],
        routed: &[&str],
        at: &str,
    ) {
        for (tag, _) in axes {
            let tag = tag.to_string();
            if routed.contains(&tag.as_str()) {
                self.problem(&join(at, "axes"), ProblemKind::RouteDuplicated { tag });
            }
        }
        for (tag, _) in features {
            let tag = tag.to_string();
            if tag == "smcp" {
                self.problem(&join(at, "features"), ProblemKind::RouteDuplicated { tag });
            }
        }
    }
}

impl Visit for Checker<'_> {
    fn node(&mut self, path: &Path, node: &dyn Any) {
        self.block(path, node);
    }

    fn value(&mut self, path: &Path, form: Form<'_>, expr: ExprForm) {
        let marks = path.marks();
        match form {
            Form::Absent if !marks.may_be_absent => {
                self.problem(&path.to_string(), ProblemKind::AbsentNotAllowed);
            }
            Form::Auto if !marks.auto => {
                self.problem(&path.to_string(), ProblemKind::AutoNotAllowed);
            }
            Form::Expr(e) => self.expression(path, e, expr),
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
        let expected = match (number, path.last_field()) {
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

    fn duplicate(&mut self, path: &Path, key: &str) {
        let at = join(&path.to_string(), key);
        let other = key.to_owned();
        self.problem(&at, ProblemKind::DuplicateKey { other });
    }

    fn map_key(&mut self, path: &Path, key: &str) {
        self.declared_key(path, key);
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
        let at = path.to_string();
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
            self.problem(&join(&at, field), ProblemKind::Missing);
        }
    }
}

/// Every structural problem of a preset's declarations (S10, S12): an
/// `exclusive_variants` group naming a variant twice; a
/// `default_size_by_variant` entry naming an undeclared variant, a size the
/// variant does not take, or a variant an earlier entry names.
pub(super) fn declarations(decl: &Declarations) -> Vec<Problem> {
    let own = decl
        .widgets
        .iter()
        .map(|(widget, section)| (format!("declare.widget.{}", widget_name(*widget)), section));
    let flavors = decl.flavor.iter().flat_map(|(flavor, section)| {
        section.widgets.iter().map(move |(widget, section)| {
            let at = format!("declare.flavor.{flavor}.widget.{}", widget_name(*widget));
            (at, section)
        })
    });
    let mut problems = Vec::new();
    for (at, section) in own.chain(flavors) {
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
            let declared = section
                .variants
                .as_ref()
                .is_some_and(|variants| variants.contains(&entry.variant));
            if !declared {
                let declaration = format!("{at}.variants");
                let kind = ProblemKind::Undeclared { name, declaration };
                problems.push(Problem::new(&join(&item, "variant"), kind));
            }
            let sizes = section
                .sizes_by_variant
                .iter()
                .flatten()
                .find(|by| by.variant == entry.variant)
                .map(|by| &by.sizes)
                .or(section.sizes.as_ref());
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
    conditions: Vec<Condition>,
    events: Vec<Key>,
    /// The bases' keyed names, by their declaration (`params` as
    /// `<group>.<key>`).
    stated: Vec<(&'static str, String)>,
    /// Walking the bases, not a rule.
    in_bases: bool,
}

impl Visit for Uses {
    fn node(&mut self, _: &Path, node: &dyn Any) {
        if let Some(PresentationEvent::Keyed(key)) = node.downcast_ref::<PresentationEvent>() {
            self.events.push(key.clone());
        }
    }

    fn value(&mut self, _: &Path, _: Form<'_>, _: ExprForm) {}

    fn number(&mut self, _: &Path, _: f32, _: Number) {}

    fn duplicate(&mut self, _: &Path, _: &str) {}

    fn patch(&mut self, _: &Path, _: &'static [&'static str], _: &[&'static str]) {}

    fn map_key(&mut self, path: &Path, key: &str) {
        if !self.in_bases {
            return;
        }
        let declaration = match path.last_field() {
            Some(("ColorRoles", "keyed")) => "color_roles",
            Some(("GlobalRoles", "accent_ramp")) => "accent_ramp",
            Some(("TextTokens", "keyed")) => "text_styles",
            Some(("Tokens", "elevation")) => "elevation",
            Some(("MotionTokens", "durations")) => "durations",
            Some(("MotionTokens", "timings")) => "timings",
            Some(("PointerTokens", "drag_cursors")) => "drag_cursors",
            Some(("Tokens", "params")) => {
                if let Some(group) = path.keys().first() {
                    self.stated.push(("params", format!("{group}.{key}")));
                }
                return;
            }
            _ => return,
        };
        self.stated.push((declaration, key.to_owned()));
    }
}

impl Uses {
    fn selector(&mut self, selector: &Selector) {
        let conditions = &selector.conditions;
        self.conditions
            .extend(conditions.all_of.iter().chain(&conditions.none_of).cloned());
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

    fn walk<T: Walk + ?Sized>(&mut self, value: &T) {
        let mut walker = Walker::new("", self);
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
            if let PartPatch::Style(style) = patch {
                self.walk(&style);
            }
        }
        if let Some(values) = values {
            self.walk(&values);
        }
    }
}

/// Each declared keyed name nothing uses (S12): a keyed flag, a system
/// setting, a flavour or a container size no rule selects on (but the
/// default flavour, each container's default size and a setting for
/// applications), a keyed name no base states, an event no `presentations`
/// list names.
pub(super) fn unused(decl: &Declarations, layer: &Layer) -> Vec<Problem> {
    let mut uses = Uses::default();
    uses.bases(&layer.bases);
    for rule in &layer.rules {
        uses.selector(&rule.selector);
        match &rule.body {
            RuleBody::Colors(roles) => uses.walk(roles),
            RuleBody::Tokens(tokens) => uses.walk(tokens),
            RuleBody::Part(_, PartPatch::Style(style)) => uses.walk(style),
            RuleBody::Part(..) => {}
            RuleBody::Values(ValuesPatch::Typed(WidgetValuesPatch::Button(values))) => {
                uses.walk(values);
            }
        }
    }
    let mut unused: Vec<(String, String)> = Vec::new();
    let sections = decl.widgets.iter().chain(
        decl.flavor
            .iter()
            .flat_map(|(_, flavor)| flavor.widgets.iter()),
    );
    for (widget, section) in sections {
        for flag in section.flags.iter().flatten() {
            if let StateFlag::Widget(key) = flag
                && !uses.flags.contains(flag)
            {
                let at = format!("declare.widget.{}.flags", widget_name(*widget));
                unused.push((at, format!("@{key}")));
            }
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
    let stated: [(&str, &[Key]); 7] = [
        ("color_roles", &decl.color_roles),
        ("accent_ramp", &decl.accent_ramp),
        ("text_styles", &decl.text_styles),
        ("elevation", &decl.elevation),
        ("durations", &decl.durations),
        ("timings", &decl.timings),
        ("drag_cursors", &decl.drag_cursors),
    ];
    for (declaration, keys) in stated {
        for key in keys {
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

/// `at.key`, or `key` at the root.
fn join(at: &str, key: &str) -> String {
    if at.is_empty() {
        key.to_owned()
    } else {
        format!("{at}.{key}")
    }
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

/// How deep calls and `active(…)` nest in `expr` (S2.2).
fn depth(expr: &Expr) -> usize {
    match expr {
        Expr::Lit(_) => 0,
        Expr::Ref(reference) => ref_depth(reference),
        Expr::Call(_, args) => args.iter().map(depth).fold(0, usize::max).saturating_add(1),
    }
}

fn ref_depth(reference: &Ref) -> usize {
    match reference {
        Ref::Active(inner) => ref_depth(inner).saturating_add(1),
        _ => 0,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::next::blocks::motion::Loop;
    use crate::next::blocks::part_style::PartStylePatch;
    use crate::next::registry::FieldInfo;
    use crate::next::units::Fraction;
    use crate::next::widgets::ButtonPart;

    /// The problems of `looped` walked in `region`.
    fn walked(looped: &Loop, region: Region) -> Vec<String> {
        let decl = Declarations::default();
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Application,
        };
        let mut checker = Checker::new(&cx);
        checker.walk("probe", region, looped);
        checker.problems.iter().map(Problem::to_string).collect()
    }

    fn looped(part: Option<PartRef>) -> Loop {
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
    static PARTS: [PartInfo; 4] = [
        part("body", true, false, None),
        part("label", false, false, None),
        part("row", false, true, None),
        part("mark", false, false, Some("row")),
    ];

    /// `button` has no *(element)* part, so the element each part is
    /// queried with is checked on a synthetic widget (S2.2, S8): a
    /// `widget.<part>` reference across two elements is refused, within one
    /// it is not.
    #[test]
    fn refusal_other_elements_part() {
        let element = |name: &str| {
            let part = PARTS.iter().find(|p| p.name == name).unwrap();
            element_of(&PARTS, part)
        };
        assert_eq!(element("mark"), "row");
        assert_eq!(element("row"), "row");
        assert_eq!(element("label"), "body");
        assert_ne!(element("mark"), element("label"));
        assert_eq!(element("body"), element("label"));
    }
}
