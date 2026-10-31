//! What a part reads of its element (S8, S10): the data its preset
//! declares, and whether its glyph or icon is drawn to its box.

use crate::next::blocks::outline::Radius;
use crate::next::declare::{DataName, DataOwner};
use crate::next::expr::Ty;
use crate::next::expr::{ElementDatum, Expr, Ref};
use crate::next::resolve::ResolvedTheme;
use crate::next::resolve::compose::{Found, Locate, Target};
use crate::next::selector::PartRef;
use crate::next::validate::walk::Node;
use crate::next::validate::walk::{ExprForm, Form, Number, Path, Visit, Walker};
use crate::next::widgets::{PartPatch, TypedPart, WidgetValuesPatch};

use super::matching::declarations;
use crate::next::resolve::evaluate::references;

/// What the connector supplies for a part (S8): the element data and font
/// metrics its values read, and whether its glyph or icon is drawn to its
/// box, so measured without it.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct DataNeeds {
    /// The element data and metrics read, each once.
    pub data: Vec<DataName>,
    /// Its `glyph.size` or `icon.size` reads the element's laid-out box:
    /// the connector measures the element without the glyph or icon, lays
    /// it out, and queries once with the laid-out box (S8).
    pub drawn_to_box: bool,
}

/// Every expression a step states, by its place below the step's target.
#[derive(Default)]
struct Exprs {
    found: Vec<(String, Expr)>,
}

impl Visit for Exprs {
    fn check(&mut self, _: &Path, _: Node<'_>) {}

    fn value(&mut self, path: &Path, form: Form<'_>, _: ExprForm, _: Option<Ty>) {
        if let Form::Expr(expr) = form {
            self.found.push((path.to_string(), expr.clone()));
        }
    }

    fn number(&mut self, _: &Path, _: f32, _: Number) {}

    fn duplicate(&mut self, _: &Path, _: &str, _: &'static str) {}

    fn map_key(&mut self, _: &Path, _: &str, _: &'static str) {}

    fn patch(&mut self, _: &Path, _: &'static [&'static str], _: &[&'static str]) {}
}

/// Whether the place `path` lies at or below `prefix`, or above it.
fn related(path: &str, prefix: &str) -> bool {
    let within = |inner: &str, outer: &str| {
        outer.is_empty()
            || inner == outer
            || inner
                .strip_prefix(outer)
                .is_some_and(|rest| rest.starts_with('.') || rest.starts_with('['))
    };
    within(path, prefix) || within(prefix, path)
}

/// Whether a path names a part's corner: `corners.<corner>` or
/// `focus.corners.<corner>`, a corner operand (S8).
fn corner(path: &[&str]) -> bool {
    let named = |name: &str| {
        matches!(
            name,
            "top_left" | "top_right" | "bottom_right" | "bottom_left"
        )
    };
    match path {
        ["corners", name] | ["focus", "corners", name] => named(name),
        _ => false,
    }
}

impl ResolvedTheme {
    /// What the connector supplies for `part` (S8): the data its preset
    /// declares for it (with the widget values' where it starts an element,
    /// a root or an *(element)* part, as `values()` is queried with its
    /// element), and whether its glyph or icon is drawn to its box. A `Full`
    /// corner read as an operand needs no pass of its own: S12 refuses one
    /// unless the reading part declares `width` and `height`, so the
    /// declaration is the answer for every stack over the preset (S10).
    /// Nothing for a component's part, as no preset declares a component
    /// yet.
    #[must_use]
    pub fn data_needs(&self, part: PartRef) -> DataNeeds {
        let PartRef::Typed(part) = part else {
            return DataNeeds::default();
        };
        let element = part.starts_element();
        let decl = declarations(self, part.widget());
        let mut data: Vec<DataName> = Vec::new();
        for (owner, names) in decl.data.iter().flatten() {
            if *owner == DataOwner::Part(part) || (element && *owner == DataOwner::Values) {
                for name in names {
                    if !data.contains(name) {
                        data.push(*name);
                    }
                }
            }
        }
        let sized = ["glyph.size", "icon.size"]
            .map(|path| (Target::Part(part), path.to_owned()))
            .to_vec();
        let (direct, full) = self.reads_box(sized);
        DataNeeds {
            data,
            drawn_to_box: direct || full,
        }
    }

    /// Whether the expressions at the places `pending` names, references
    /// followed through `part.`, `widget.` and `values.`, read the element's
    /// width or height, and whether they read a corner operand that can be
    /// `Full` (S8), conservatively over every step the part keeps.
    fn reads_box(&self, mut pending: Vec<(Target, String)>) -> (bool, bool) {
        let mut seen: Vec<(Target, String)> = Vec::new();
        let (mut direct, mut full) = (false, false);
        while let Some((target, prefix)) = pending.pop() {
            if seen.contains(&(target, prefix.clone())) {
                continue;
            }
            seen.push((target, prefix.clone()));
            for (path, expr) in self.exprs(target) {
                if !related(&path, &prefix) {
                    continue;
                }
                let mut refs = Vec::new();
                references(&expr, &mut refs);
                for reference in refs {
                    let (other, path) = match &reference {
                        Ref::Element(ElementDatum::Width | ElementDatum::Height) => {
                            direct = true;
                            continue;
                        }
                        Ref::Part(path) => (target, path),
                        Ref::Widget { part, path } => {
                            let widget = match target {
                                Target::Part(own) => own.widget(),
                                Target::Values(widget) => widget,
                                Target::Roles | Target::Tokens => continue,
                            };
                            match TypedPart::from_names(widget, part.as_str()) {
                                Some(other) => (Target::Part(other), path),
                                None => continue,
                            }
                        }
                        Ref::Values(name) => {
                            let widget = match target {
                                Target::Part(own) => own.widget(),
                                Target::Values(widget) => widget,
                                Target::Roles | Target::Tokens => continue,
                            };
                            pending.push((Target::Values(widget), name.as_str().to_owned()));
                            continue;
                        }
                        _ => continue,
                    };
                    let Target::Part(other) = other else {
                        continue;
                    };
                    let segments: Vec<&str> = path.iter().map(|key| key.as_str()).collect();
                    if corner(&segments) && self.can_be_full(other, &segments) {
                        full = true;
                    }
                    pending.push((Target::Part(other), segments.join(".")));
                }
            }
        }
        (direct, full)
    }

    /// Whether some step of `part` can make the corner at `path` `Full`:
    /// one stating it so, or a reference above it (conservatively).
    fn can_be_full(&self, part: TypedPart, path: &[&str]) -> bool {
        self.part_steps(part).iter().any(|step| match &step.patch {
            PartPatch::Style(style) => match style.locate(path) {
                Found::Through { .. } => true,
                found => matches!(found.get::<Radius>(), Some(Radius::Full { .. })),
            },
            PartPatch::Absent | PartPatch::DrawnAs { .. } => false,
        })
    }

    /// Every expression the steps of `target` state, by place.
    fn exprs(&self, target: Target) -> Vec<(String, Expr)> {
        let mut exprs = Exprs::default();
        match target {
            Target::Part(part) => {
                for step in self.part_steps(part) {
                    if let PartPatch::Style(style) = &step.patch {
                        Walker::new("", false, &mut exprs).walk(style);
                    }
                }
            }
            Target::Values(widget) => {
                for (_, steps) in self.values.iter().filter(|(held, _)| *held == widget) {
                    for step in steps {
                        match &step.patch {
                            WidgetValuesPatch::Button(values) => {
                                Walker::new("", false, &mut exprs).walk(values);
                            }
                        }
                    }
                }
            }
            Target::Roles | Target::Tokens => {}
        }
        exprs.found
    }
}
