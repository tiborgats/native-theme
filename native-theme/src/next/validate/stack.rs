//! The stack as S12's checks over combinations read it: each layer's bases
//! and rules flattened into the values they state, place by place, in the
//! stack's order (S8), and what a place resolves to under one combination of
//! the rules' applicability, references and rules followed as the query
//! follows them (S2.2's lazy composition).

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::combos::World;
use super::walk::{Form, Marks, Node, Path, Segment, Visit, Walk, Walker};
use super::{Context, LayerKind};
use crate::next::blocks::paint::PaintPatch;
use crate::next::blocks::part_style::{Binding, RoleRef};
use crate::next::expr::Expr;
use crate::next::layer::{Bases, Layer, Spelled};
use crate::next::presets::PresetId;
use crate::next::registry::{self, FieldInfo, PartInfo};
use crate::next::roles::ColorRolesPatch;
use crate::next::selector::{Condition, PartRef, Rule, RuleBody, Scheme, Selector, ValuesPatch};
use crate::next::value::Value;
use crate::next::widgets::{PartPatch, Shown, TypedPart, WidgetKind, WidgetValuesPatch};

/// A layer of the stack, and what it is to the stack.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Stacked<'a> {
    /// The layer.
    pub(crate) layer: &'a Layer,
    /// What it is.
    pub(crate) kind: LayerKind,
}

/// What a source states values of: the colour roles of a scheme, the
/// tokens, a typed widget's part or its values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Target {
    /// The colour roles under one scheme.
    Roles(Scheme),
    /// The tokens.
    Tokens,
    /// A typed widget's part.
    Part(TypedPart),
    /// A typed widget's values.
    Values(WidgetKind),
}

/// Where a source lies in the stack's order (S8): every layer's bases, in
/// layer order (a layer's `for_preset` part after its common bases), then
/// every layer's rules, in layer and stated order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Pos {
    /// A rule's, not a base's.
    pub(super) rule: bool,
    /// The layer, for a base; the rule's place in the stack's order, for a
    /// rule.
    pub(super) major: usize,
    /// A base's part: 0 the common bases, 1 the current preset's.
    pub(super) minor: usize,
}

/// A stated number, or an enum's variant, as far as a check reads it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Fact {
    /// Stated, its content not read.
    Unknown,
    /// Stated as holding nothing: an `Option` stated `None`, an empty list.
    Nothing,
    /// A stated number.
    Number(f32),
    /// An enum's variant, in snake_case.
    Variant(&'static str),
}

/// A place's value as a source states it.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Val {
    /// `Absent`.
    Absent,
    /// `Auto`.
    Auto,
    /// An expression.
    Expr(Expr),
    /// A stated value.
    Stated(Fact),
}

/// What one step of a source does to its places.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum OpKind {
    /// Sets the place, and replaces what lies below it.
    Set(Val),
    /// A `Fields` patch: applies the fields after it over the value below;
    /// `unset`, the fields it leaves unset, for a map's entry it can create.
    Fields {
        /// The fields the patch leaves unset.
        unset: Vec<&'static str>,
        /// The patch is a map entry's (S6.1).
        entry: bool,
    },
    /// An `Append`: layers added over the paint below.
    Append,
    /// A list replaced whole: its earlier items are gone.
    Clear,
    /// A field a patch sets to a value holding nothing (`None`, an empty
    /// list): stated, what lies below it kept.
    Touch,
    /// What a part is drawn as (S6.1, S6.3).
    Shown(Shown),
}

/// One step of a source: what it does, where.
#[derive(Clone, Debug)]
pub(super) struct Op {
    /// The place, in the model's names below the target, a list's item an
    /// index (`stroke.top.dash`, `transitions[3:0].duration`).
    pub(super) path: String,
    /// The place as a problem names it, spelled as the layer was written.
    pub(super) at: String,
    /// What it does.
    pub(super) kind: OpKind,
    /// The field's marks there (S6.3).
    pub(super) marks: Marks,
    /// The place of the `TextStyle` holding the field, where one does.
    pub(super) text_style: Option<String>,
    /// The innermost field, owner and name.
    pub(super) field: Option<(&'static str, &'static str)>,
}

/// A colour binding a part's patch states (S3.3).
#[derive(Clone, Debug)]
pub(super) struct BindingOp {
    /// The role it binds.
    pub(super) role: RoleRef,
    /// Its value.
    pub(super) value: Val,
    /// Where it is stated.
    pub(super) at: String,
}

/// One base's or rule's patch of one target, flattened.
#[derive(Clone, Debug)]
pub(super) struct Source {
    /// What it patches.
    pub(super) target: Target,
    /// Where it lies in the stack's order.
    pub(super) pos: Pos,
    /// The layer it is of.
    pub(super) layer: usize,
    /// The rule it is, by its index in [`Stack::rules`]; `None`: a base.
    pub(super) rule: Option<usize>,
    /// The scheme a `Colors` rule's source applies under.
    pub(super) scheme: Option<Scheme>,
    /// Its steps, in order.
    pub(super) ops: Vec<Op>,
    /// A part's colour bindings.
    pub(super) bindings: Vec<BindingOp>,
}

/// A rule of the stack.
#[derive(Clone, Copy, Debug)]
pub(super) struct RuleInfo<'a> {
    /// Its selector.
    pub(super) selector: &'a Selector,
    /// The widget whose part or values it targets.
    pub(super) widget: Option<WidgetKind>,
}

/// The stack, flattened.
pub(super) struct Stack<'a> {
    /// Every source, in the stack's order.
    pub(super) sources: Vec<Source>,
    /// Every rule validated (a rule whose `Preset` condition excludes the
    /// current preset is neither validated nor applied, S12).
    pub(super) rules: Vec<RuleInfo<'a>>,
    /// The layers.
    pub(super) layers: Vec<Stacked<'a>>,
    /// The current preset's declarations, preset and the added layer's kind.
    pub(super) cx: Context<'a>,
    /// The worlds of each set of rules a check reaches, enumerated once.
    pub(super) cache: RefCell<HashMap<Vec<usize>, Rc<Vec<World>>>>,
}

/// A list's items whose list merges by key (S6.1): kept beside the earlier
/// items, never replacing them by their place.
const KEYED_ITEMS: [&str; 3] = ["Transition", "Binding", "PresentationTransition"];

/// Whether `ancestor` lies above `path`.
pub(super) fn is_above(ancestor: &str, path: &str) -> bool {
    if ancestor.is_empty() {
        return !path.is_empty();
    }
    path.strip_prefix(ancestor)
        .is_some_and(|rest| rest.starts_with('.') || rest.starts_with('['))
}

/// `path` with `tail` below it.
pub(super) fn below(path: &str, tail: &str) -> String {
    match (path.is_empty(), tail.is_empty()) {
        (true, _) => tail.to_owned(),
        (_, true) => path.to_owned(),
        _ if tail.starts_with('[') => format!("{path}{tail}"),
        _ => format!("{path}.{tail}"),
    }
}

/// The part of `path` below `ancestor`.
pub(super) fn tail<'p>(ancestor: &str, path: &'p str) -> &'p str {
    let rest = path.strip_prefix(ancestor).unwrap_or(path);
    rest.strip_prefix('.').unwrap_or(rest)
}

/// A walk's visit that flattens what it meets into a source's steps.
struct Flatten {
    /// The source's number, which makes a merged list's items its own.
    source: usize,
    ops: Vec<Op>,
    /// Lists this source replaces, already cleared.
    cleared: Vec<String>,
    /// Places this source appends to.
    appended: Vec<String>,
}

impl Flatten {
    /// The place `path` names, a list's first item replacing the list
    /// (a list merged by key, or appended to, keeping its earlier items).
    fn place(&mut self, path: &Path) -> String {
        let segments: Vec<Segment<'_>> = path.segments().collect();
        let mut out = String::new();
        for (index, segment) in segments.iter().enumerate() {
            match segment {
                Segment::Field { name, .. } | Segment::Variant { name, .. } => {
                    out = below(&out, name);
                }
                Segment::Key(key) => out = below(&out, key),
                Segment::Index(item) => {
                    let next =
                        segments
                            .get(index.saturating_add(1))
                            .and_then(|segment| match segment {
                                Segment::Field { owner, .. } | Segment::Variant { owner, .. } => {
                                    Some(*owner)
                                }
                                Segment::Key(_) | Segment::Index(_) => None,
                            });
                    let keyed = next.is_some_and(|owner| KEYED_ITEMS.contains(&owner));
                    if keyed || self.appended.contains(&out) {
                        out = format!("{out}[{}:{item}]", self.source);
                    } else {
                        if !self.cleared.contains(&out) {
                            self.cleared.push(out.clone());
                            self.ops.push(Op {
                                path: out.clone(),
                                at: String::new(),
                                kind: OpKind::Clear,
                                marks: Marks::default(),
                                text_style: None,
                                field: None,
                            });
                        }
                        out = format!("{out}[{item}]");
                    }
                }
            }
        }
        out
    }

    fn push(&mut self, path: &Path, place: String, kind: OpKind) {
        let text_style = text_style(path);
        self.ops.push(Op {
            path: place,
            at: path.to_string(),
            kind,
            marks: path.marks(),
            text_style,
            field: path.last_field(),
        });
    }

    /// Refines a stated value just set at `place` to `fact`, or states it.
    fn fact(&mut self, path: &Path, fact: Fact) {
        let place = self.place(path);
        if let Some(last) = self.ops.last_mut()
            && last.path == place
            && let OpKind::Set(Val::Stated(held)) = &mut last.kind
        {
            if *held == Fact::Unknown {
                *held = fact;
            }
            return;
        }
        self.push(path, place, OpKind::Set(Val::Stated(fact)));
    }

    /// The steps, each `Touch` another step of the source sets below or at
    /// its place dropped: what remains is a field set to nothing.
    fn finish(self) -> Vec<Op> {
        let ops = self.ops;
        ops.iter()
            .enumerate()
            .filter(|(index, op)| {
                op.kind != OpKind::Touch
                    || !ops.iter().skip(index.saturating_add(1)).any(|later| {
                        later.kind != OpKind::Clear
                            && (later.path == op.path || is_above(&op.path, &later.path))
                    })
            })
            .map(|(_, op)| op.clone())
            .collect()
    }
}

/// The place of the `TextStyle` holding the field `path` names: the path up
/// to the first field the text style owns.
fn text_style(path: &Path) -> Option<String> {
    let mut out = String::new();
    for segment in path.segments() {
        match segment {
            Segment::Field {
                owner: "TextStyle", ..
            } => return Some(out),
            Segment::Field { name, .. } | Segment::Variant { name, .. } => {
                out = below(&out, name);
            }
            Segment::Key(key) => out = below(&out, key),
            Segment::Index(item) => out = format!("{out}[{item}]"),
        }
    }
    None
}

impl Visit for Flatten {
    fn check(&mut self, path: &Path, node: Node<'_>) {
        if let Node::PaintPatch(PaintPatch::Append(_)) = node {
            let place = self.place(path);
            self.appended.push(place.clone());
            self.push(path, place, OpKind::Append);
        }
    }

    fn value(
        &mut self,
        path: &Path,
        form: Form<'_>,
        _: super::walk::ExprForm,
        _: Option<crate::next::expr::Ty>,
    ) {
        let val = match form {
            Form::Absent => Val::Absent,
            Form::Auto => Val::Auto,
            Form::Expr(expr) => Val::Expr(expr.clone()),
            Form::Stated => Val::Stated(Fact::Unknown),
        };
        let place = self.place(path);
        self.push(path, place, OpKind::Set(val));
    }

    fn number(&mut self, path: &Path, n: f32, _: super::walk::Number) {
        self.fact(path, Fact::Number(n));
    }

    fn duplicate(&mut self, _: &Path, _: &str, _: &'static str) {}

    fn map_key(&mut self, _: &Path, _: &str, _: &'static str) {}

    fn patch(&mut self, path: &Path, all: &'static [&'static str], set: &[&'static str]) {
        let place = self.place(path);
        if !place.is_empty() {
            let unset = all.iter().copied().filter(|f| !set.contains(f)).collect();
            let entry = path.map_entry().is_some();
            self.push(path, place.clone(), OpKind::Fields { unset, entry });
        }
        for field in set {
            let at = path.join(field);
            self.ops.push(Op {
                path: below(&place, field),
                at,
                kind: OpKind::Touch,
                marks: Marks::default(),
                text_style: None,
                field: None,
            });
        }
    }

    fn enum_variant(&mut self, path: &Path, _: &'static str, name: &'static str) {
        self.fact(path, Fact::Variant(name));
    }
}

/// `value`'s steps, walked from `root` (spelled by TOML's keys where
/// `toml`), as the style of a part whose catalogue entry is `part`.
fn flatten<T: Walk + ?Sized>(
    value: &T,
    root: &str,
    toml: bool,
    source: usize,
    part: Option<&'static [FieldInfo]>,
) -> Vec<Op> {
    let mut flat = Flatten {
        source,
        ops: Vec::new(),
        cleared: Vec::new(),
        appended: Vec::new(),
    };
    {
        let mut walker = Walker::new(root, toml, &mut flat);
        match part {
            Some(fields) => walker.part(fields, |w| w.walk(value)),
            None => walker.walk(value),
        }
    }
    flat.finish()
}

/// `at.key`, or `key` at the root.
fn join(at: &str, key: &str) -> String {
    if at.is_empty() {
        key.to_owned()
    } else {
        format!("{at}.{key}")
    }
}

/// Whether a rule's `Preset` condition excludes the current preset (S12).
pub(super) fn excluded(rule: &Rule, preset: Option<PresetId>) -> bool {
    let conditions = &rule.selector.conditions;
    let other =
        |condition: &Condition| matches!(condition, Condition::Preset(id) if Some(*id) != preset);
    let current =
        |condition: &Condition| matches!(condition, Condition::Preset(id) if Some(*id) == preset);
    conditions.all_of.iter().any(other) || conditions.none_of.iter().any(current)
}

/// The catalogue entry of a typed part.
pub(super) fn part_info(part: TypedPart) -> Option<&'static PartInfo> {
    let (widget, name) = match part {
        TypedPart::Button(part) => (WidgetKind::Button, part.name()),
    };
    registry::widgets()
        .iter()
        .find(|info| info.kind == widget)
        .and_then(|info| info.part(name))
}

/// The widget a part belongs to.
pub(super) fn widget_of(part: TypedPart) -> WidgetKind {
    match part {
        TypedPart::Button(_) => WidgetKind::Button,
    }
}

/// A role binding's value as a source states it.
fn binding_val(value: &Value<crate::next::color::Color>) -> Val {
    match value {
        Value::Stated(_) => Val::Stated(Fact::Unknown),
        Value::Expr(expr) => Val::Expr(expr.clone()),
        Value::Absent => Val::Absent,
        Value::Auto => Val::Auto,
    }
}

impl<'a> Stack<'a> {
    /// The stack of `layers`, the last the one being added, as `cx` sees it.
    pub(super) fn new(layers: &[Stacked<'a>], cx: Context<'a>) -> Stack<'a> {
        let mut stack = Stack {
            sources: Vec::new(),
            rules: Vec::new(),
            layers: layers.to_vec(),
            cx,
            cache: RefCell::new(HashMap::new()),
        };
        for (index, stacked) in layers.iter().enumerate() {
            let toml = matches!(stacked.layer.spelled, Spelled::Toml);
            let common = Pos {
                rule: false,
                major: index,
                minor: 0,
            };
            stack.bases(&stacked.layer.bases, "", common, index, toml);
            for (preset, bases) in &stacked.layer.for_preset {
                if cx.preset == Some(*preset) {
                    let pos = Pos { minor: 1, ..common };
                    stack.bases(
                        bases,
                        &format!("for_preset.{}", preset.key()),
                        pos,
                        index,
                        toml,
                    );
                }
            }
        }
        for (index, stacked) in layers.iter().enumerate() {
            let toml = matches!(stacked.layer.spelled, Spelled::Toml);
            for (number, rule) in stacked.layer.rules.iter().enumerate() {
                if !excluded(rule, cx.preset) {
                    let set = if toml { "set" } else { "body" };
                    stack.rule(rule, &format!("rules[{number}].{set}"), index, toml);
                }
            }
        }
        for source in &mut stack.sources {
            if let Some(stacked) = layers.get(source.layer) {
                let shorthands = &stacked.layer.shorthands;
                for op in &mut source.ops {
                    op.at = shorthands.written(&op.at);
                }
                for binding in &mut source.bindings {
                    binding.at = shorthands.written(&binding.at);
                }
            }
        }
        stack
    }

    fn push(&mut self, target: Target, pos: Pos, layer: usize, rule: Option<usize>, ops: Vec<Op>) {
        self.sources.push(Source {
            target,
            pos,
            layer,
            rule,
            scheme: None,
            ops,
            bindings: Vec::new(),
        });
    }

    fn bases(&mut self, bases: &Bases, at: &str, pos: Pos, layer: usize, toml: bool) {
        for (scheme, roles) in [
            (Scheme::Light, &bases.colors.light),
            (Scheme::Dark, &bases.colors.dark),
        ] {
            if let Some(roles) = roles {
                let root = join(at, &format!("colors.{}", scheme.name()));
                self.roles(roles, &root, Target::Roles(scheme), pos, layer, None, toml);
            }
        }
        let number = self.sources.len();
        let ops = flatten(&bases.tokens, &join(at, "tokens"), toml, number, None);
        self.push(Target::Tokens, pos, layer, None, ops);
        if let Some(button) = &bases.widgets.button {
            let (parts, values) = button.clone().into_parts();
            let root = join(at, "widgets.button");
            for (part, patch) in parts {
                let at = join(&root, part.name());
                self.part(TypedPart::Button(part), &patch, &at, pos, layer, None, toml);
            }
            if let Some(values) = values {
                let number = self.sources.len();
                let ops = flatten(&values, &join(&root, "values"), toml, number, None);
                self.push(Target::Values(WidgetKind::Button), pos, layer, None, ops);
            }
        }
    }

    #[expect(clippy::too_many_arguments, reason = "one source's place in the stack")]
    fn roles(
        &mut self,
        roles: &ColorRolesPatch,
        root: &str,
        target: Target,
        pos: Pos,
        layer: usize,
        rule: Option<usize>,
        toml: bool,
    ) {
        let number = self.sources.len();
        let declared = &self.cx.declarations.surface_sets;
        // A surface set the preset does not declare is refused (S10) or, a
        // palette's, skipped (S14): either way no value of it is read.
        let ops = flatten(roles, root, toml, number, None)
            .into_iter()
            .filter(|op| {
                op.path
                    .strip_prefix("surfaces.")
                    .and_then(|rest| rest.split(['.', '[']).next())
                    .and_then(crate::next::roles::SurfaceSet::from_name)
                    .is_none_or(|set| declared.contains(&set))
            })
            .collect();
        self.push(target, pos, layer, rule, ops);
    }

    #[expect(clippy::too_many_arguments, reason = "one source's place in the stack")]
    fn part(
        &mut self,
        part: TypedPart,
        patch: &PartPatch,
        at: &str,
        pos: Pos,
        layer: usize,
        rule: Option<usize>,
        toml: bool,
    ) {
        let Some(info) = part_info(part) else {
            return;
        };
        let number = self.sources.len();
        let shown = |shown: Shown| Op {
            path: String::new(),
            at: at.to_owned(),
            kind: OpKind::Shown(shown),
            marks: Marks::default(),
            text_style: None,
            field: None,
        };
        let mut bindings = Vec::new();
        let ops = match patch {
            PartPatch::Absent => vec![shown(Shown::Absent)],
            PartPatch::DrawnAs { target, variants } => vec![shown(Shown::DrawnAs {
                target: target.clone(),
                variants: variants.clone(),
            })],
            PartPatch::Style(style) => {
                for (index, binding) in style.bindings.iter().flatten().enumerate() {
                    if let Binding::Role { role, value } = binding {
                        bindings.push(BindingOp {
                            role: role.clone(),
                            value: binding_val(value),
                            at: format!("{at}.bindings[{index}].value"),
                        });
                    }
                }
                let mut ops = vec![shown(Shown::Drawn)];
                ops.extend(flatten(style, at, toml, number, Some(info.fields)));
                ops
            }
        };
        self.sources.push(Source {
            target: Target::Part(part),
            pos,
            layer,
            rule,
            scheme: None,
            ops,
            bindings,
        });
    }

    fn rule(&mut self, rule: &'a Rule, at: &str, layer: usize, toml: bool) {
        let index = self.rules.len();
        let pos = Pos {
            rule: true,
            major: index,
            minor: 0,
        };
        let widget = match &rule.body {
            RuleBody::Part(PartRef::Typed(part), _) => Some(widget_of(*part)),
            RuleBody::Values(ValuesPatch::Typed(WidgetValuesPatch::Button(_))) => {
                Some(WidgetKind::Button)
            }
            RuleBody::Colors(_)
            | RuleBody::Tokens(_)
            | RuleBody::Part(PartRef::Component { .. }, _) => None,
        };
        self.rules.push(RuleInfo {
            selector: &rule.selector,
            widget,
        });
        match &rule.body {
            RuleBody::Colors(roles) => {
                for scheme in Scheme::ALL {
                    let target = Target::Roles(scheme);
                    self.roles(roles, at, target, pos, layer, Some(index), toml);
                    if let Some(source) = self.sources.last_mut() {
                        source.scheme = Some(scheme);
                    }
                }
            }
            RuleBody::Tokens(tokens) => {
                let number = self.sources.len();
                let ops = flatten(tokens, at, toml, number, None);
                self.push(Target::Tokens, pos, layer, Some(index), ops);
            }
            RuleBody::Part(PartRef::Typed(part), patch) => {
                self.part(*part, patch, at, pos, layer, Some(index), toml);
            }
            RuleBody::Part(PartRef::Component { .. }, _) => {}
            RuleBody::Values(ValuesPatch::Typed(WidgetValuesPatch::Button(values))) => {
                let number = self.sources.len();
                let ops = flatten(values, at, toml, number, None);
                self.push(
                    Target::Values(WidgetKind::Button),
                    pos,
                    layer,
                    Some(index),
                    ops,
                );
            }
        }
    }

    /// The sources of `target`, in the stack's order.
    pub(super) fn of(&self, target: Target) -> impl Iterator<Item = (usize, &Source)> {
        self.sources
            .iter()
            .enumerate()
            .filter(move |(_, source)| source.target == target)
    }
}
