//! S12's cycle check through the inputs (S9): an input whose value reads
//! itself, directly or through other inputs and colour roles. A cycle is one
//! within one scheme and contrast, over the values in force there: each
//! input's from the last layer stating it for that scheme, its per-contrast
//! base where that contrast holds and it states one, as resolution merges
//! them (S8, S9.3); each role's from the last base stating it and every
//! colour rule that can hold under that scheme and contrast
//! (conservatively, the other conditions not decided).

use super::super::stack::{Stacked, excluded};
use crate::next::expr::{Expr, Ref};
use crate::next::inputs::{Contrast, InputName};
use crate::next::key::Key;
use crate::next::layer::Layer;
use crate::next::presets::PresetId;
use crate::next::resolve::compose::{Found, Locate};
use crate::next::resolve::input_value;
use crate::next::roles::ColorRolesPatch;
use crate::next::selector::{Condition, RuleBody, Scheme, SetMatch};
use crate::next::value::Value;

/// The contrasts an input's value can be read under: none, and each.
const CONTRASTS: [Option<Contrast>; 3] = [None, Some(Contrast::Increased), Some(Contrast::High)];

/// A layer's bases' parts in force: its common bases, and the current
/// preset's own.
fn bases_of(layer: &Layer, preset: Option<PresetId>) -> Vec<&crate::next::layer::Bases> {
    let mut out = vec![&layer.bases];
    out.extend(
        layer
            .for_preset
            .iter()
            .filter(|(id, _)| Some(*id) == preset)
            .map(|(_, bases)| bases),
    );
    out
}

/// Whether a rule's conditions can hold under `scheme` and `contrast`.
fn can_hold(conditions: &SetMatch<Condition>, scheme: Scheme, contrast: Option<Contrast>) -> bool {
    let holds = |condition: &Condition| match condition {
        Condition::Scheme(held) => Some(*held == scheme),
        Condition::ContrastIncreased => Some(contrast == Some(Contrast::Increased)),
        Condition::ContrastHigh => Some(contrast == Some(Contrast::High)),
        _ => None,
    };
    !conditions.all_of.iter().any(|c| holds(c) == Some(false))
        && !conditions.none_of.iter().any(|c| holds(c) == Some(true))
}

/// The expression a step states at a role's place, or above it.
fn role_expr<'a>(patch: &'a ColorRolesPatch, path: &[&str]) -> Option<&'a Expr> {
    match patch.locate(path) {
        Found::Through { expr, .. } => Some(expr),
        found => match found.get::<Value<crate::next::color::Color>>() {
            Some(Value::Expr(expr)) => Some(expr),
            _ => found
                .get::<Option<Value<crate::next::color::Color>>>()
                .and_then(Option::as_ref)
                .and_then(|value| match value {
                    Value::Expr(expr) => Some(expr),
                    _ => None,
                }),
        },
    }
}

/// A role reference's place below the colour roles.
fn role_path(reference: &Ref) -> Option<Vec<String>> {
    fn names(path: &[Key]) -> impl Iterator<Item = String> + '_ {
        path.iter().map(|key| key.as_str().to_owned())
    }
    Some(match reference {
        Ref::Role { set, path } => {
            let mut out = vec!["surfaces".to_owned(), set.name().to_owned()];
            out.extend(names(path));
            out
        }
        Ref::GlobalRole(path) => {
            let mut out = vec!["global".to_owned()];
            out.extend(names(path));
            out
        }
        Ref::KeyedRole(key) => vec!["keyed".to_owned(), key.as_str().to_owned()],
        Ref::Active(inner) => return role_path(inner),
        _ => return None,
    })
}

/// A node of the search: an input, or a colour role's place.
#[derive(Clone, PartialEq)]
enum Node {
    Input(InputName),
    Role(Vec<String>),
}

/// The values in force under one scheme and contrast.
struct InForce<'a> {
    layers: &'a [Stacked<'a>],
    preset: Option<PresetId>,
    scheme: Scheme,
    contrast: Option<Contrast>,
}

impl<'a> InForce<'a> {
    /// The input `name`'s value in force, and the layer stating it.
    fn input(&self, name: &InputName) -> Option<(usize, Value<crate::next::color::Color>)> {
        let mut out = None;
        for (layer, stacked) in self.layers.iter().enumerate() {
            for bases in bases_of(stacked.layer, self.preset) {
                for (held, value) in &bases.inputs.entries {
                    if held == name
                        && let Some(value) = input_value(value, self.scheme, self.contrast)
                    {
                        out = Some((layer, value));
                    }
                }
            }
        }
        out
    }

    /// The expressions a role's place can take: the last base's, and each
    /// colour rule's that can hold.
    fn role(&self, path: &[String]) -> Vec<&'a Expr> {
        let segments: Vec<&str> = path.iter().map(String::as_str).collect();
        let mut base: Option<&'a Expr> = None;
        let mut out = Vec::new();
        for stacked in self.layers {
            for bases in bases_of(stacked.layer, self.preset) {
                if let Some(patch) = bases.colors.get(self.scheme) {
                    if let Some(expr) = role_expr(patch, &segments) {
                        base = Some(expr);
                    } else if !matches!(patch.locate(&segments), Found::Missing) {
                        base = None;
                    }
                }
            }
            for rule in &stacked.layer.rules {
                if let RuleBody::Colors(patch) = &rule.body
                    && !excluded(rule, self.preset)
                    && can_hold(&rule.selector.conditions, self.scheme, self.contrast)
                    && let Some(expr) = role_expr(patch, &segments)
                {
                    out.push(expr);
                }
            }
        }
        out.extend(base);
        out
    }

    /// The expressions a node's value can take.
    fn exprs(&self, node: &Node) -> Vec<Expr> {
        match node {
            Node::Input(name) => match self.input(name) {
                Some((_, Value::Expr(expr))) => vec![expr],
                _ => Vec::new(),
            },
            Node::Role(path) => self.role(path).into_iter().cloned().collect(),
        }
    }
}

/// The nodes an expression's references read.
fn reads(expr: &Expr, inputs: &[InputName]) -> Vec<Node> {
    let mut refs = Vec::new();
    crate::next::resolve::evaluate::references(expr, &mut refs);
    refs.iter()
        .filter_map(|reference| match reference {
            Ref::Input(path) => inputs
                .iter()
                .find(|name| name.segments() == path.as_slice())
                .cloned()
                .map(Node::Input),
            other => role_path(other).map(Node::Role),
        })
        .collect()
}

/// Every input whose value in force reads itself under some scheme and
/// contrast, with the layer stating that value.
pub(super) fn input_cycles(
    layers: &[Stacked<'_>],
    preset: Option<PresetId>,
) -> Vec<(usize, InputName)> {
    let mut names: Vec<InputName> = Vec::new();
    for stacked in layers {
        for bases in bases_of(stacked.layer, preset) {
            for (name, _) in &bases.inputs.entries {
                if !names.contains(name) {
                    names.push(name.clone());
                }
            }
        }
    }
    let mut out: Vec<(usize, InputName)> = Vec::new();
    for scheme in Scheme::ALL {
        for contrast in CONTRASTS {
            let in_force = InForce {
                layers,
                preset,
                scheme,
                contrast,
            };
            for name in &names {
                let Some((layer, Value::Expr(expr))) = in_force.input(name) else {
                    continue;
                };
                let start = Node::Input(name.clone());
                let mut seen: Vec<Node> = Vec::new();
                let mut pending = reads(&expr, &names);
                let mut cycle = false;
                while let Some(node) = pending.pop() {
                    if node == start {
                        cycle = true;
                        break;
                    }
                    if seen.contains(&node) {
                        continue;
                    }
                    for expr in in_force.exprs(&node) {
                        pending.extend(reads(&expr, &names));
                    }
                    seen.push(node);
                }
                if cycle && !out.contains(&(layer, name.clone())) {
                    out.push((layer, name.clone()));
                }
            }
        }
    }
    out
}
