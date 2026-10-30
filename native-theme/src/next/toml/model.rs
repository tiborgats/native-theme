//! The readers written by hand (S11): a struct block's patch over the block
//! below, a paint's, the shorthands (`corners_px`, the stroke's edge keys,
//! S2.4), and the model's names that are spelled their own way (a part, a
//! widget, a variant, a role, an icon, a tag).

use ::toml::Value as Item;

use super::read::{
    self, Build, FromToml, MapKey, Reader, Spelling, join, read_fields, table, text,
};
use crate::IconRole;
use crate::IconSet;
use crate::next::blocks::outline::{
    Corners, CornersPatch, EdgePatch, PixelSnap, Radius, Stroke, StrokePatch,
};
use crate::next::blocks::paint::{BlendMode, Paint, PaintKind, PaintLayer, PaintPatch};
use crate::next::blocks::text::{AxisTag, FeatureTag, LanguageTag};
use crate::next::blocks::{BlockPatch, FieldsBlock};
use crate::next::error::ProblemKind;
use crate::next::registry;
use crate::next::roles::{GlobalRole, SurfaceRole, SurfaceSet};
use crate::next::selector::{PartRef, VariantRef, WidgetOrPart, WidgetRef};
use crate::next::units::{Px, ViewBox};
use crate::next::value::Value;
use crate::next::widgets::{TypedPart, Variant, WidgetKind};

/// A struct block field's patch (S2.2): in a rule's `set` or an
/// application layer's bases a table is a `Fields` patch, a whole block
/// stated by `{ replace = { … } }` or a reference; in a preset's own bases a
/// table is the whole block.
impl<T> FromToml for BlockPatch<T>
where
    T: FieldsBlock + Build<Patch = <T as FieldsBlock>::Patch> + FromToml,
    <T as FieldsBlock>::Patch: FromToml,
{
    const SPELLINGS: &'static [Spelling] = <<T as FieldsBlock>::Patch as FromToml>::SPELLINGS;

    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        if spelling != Spelling::Bare {
            return <<T as FieldsBlock>::Patch as FromToml>::read(item, spelling, at, cx)
                .map(BlockPatch::Fields);
        }
        match item {
            Item::Table(fields) if fields.len() == 1 && fields.contains_key("replace") => {
                let at = join(at, "replace");
                let whole = fields.get("replace")?;
                let patch =
                    <<T as FieldsBlock>::Patch as FromToml>::read(whole, Spelling::Bare, &at, cx)?;
                T::build(patch, &at, false, cx).map(|block| BlockPatch::Set(Value::Stated(block)))
            }
            Item::Table(_) => <<T as FieldsBlock>::Patch as FromToml>::read(item, spelling, at, cx)
                .map(BlockPatch::Fields),
            _ => Value::<T>::read(item, spelling, at, cx).map(BlockPatch::Set),
        }
    }

    fn combine(&mut self, later: Self) -> bool {
        match (self, later) {
            (BlockPatch::Fields(held), BlockPatch::Fields(later)) => held.combine(later),
            _ => false,
        }
    }

    fn finish(self, at: &str, cx: &mut Reader) -> Self {
        match self {
            BlockPatch::Fields(fields) if cx.whole => {
                match T::build(fields.clone(), at, false, cx) {
                    Some(block) => BlockPatch::Set(Value::Stated(block)),
                    None => BlockPatch::Fields(fields),
                }
            }
            other => other,
        }
    }
}

/// A paint's patch (S2.3, R6): the paint, a colour standing for one `Color`
/// layer, a reference or `"absent"`; or `{ append = [ … ] }`, layers over
/// `base`, refused in a layer's bases (S6.1).
impl FromToml for PaintPatch {
    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        if let Item::Table(fields) = item
            && fields.len() == 1
            && let Some(layers) = fields.get("append")
        {
            let at = join(at, "append");
            if cx.bases {
                cx.problem(&at, ProblemKind::AppendInBases);
            }
            return Vec::<PaintLayer>::read(layers, Spelling::Bare, &at, cx)
                .map(PaintPatch::Append);
        }
        Value::<Paint>::read(item, spelling, at, cx).map(PaintPatch::Set)
    }
}

/// A paint stated whole: `{ layers = [ … ] }`, or a colour standing for one
/// `Color` layer (S2.2).
impl FromToml for Paint {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        match item {
            Item::Table(fields) => {
                let mut layers = None;
                for (key, value) in fields {
                    let at = join(at, key);
                    if key == "layers" {
                        layers = Vec::<PaintLayer>::read(value, Spelling::Bare, &at, cx);
                    } else {
                        cx.problem(&at, ProblemKind::UnknownKey);
                    }
                }
                read::required(layers, &join(at, "layers"), cx).map(|layers| Paint { layers })
            }
            _ => {
                cx.shorthand(format!("{at}.layers[0]"), at);
                PaintLayer::read(item, Spelling::Item, at, cx).map(|layer| Paint {
                    layers: vec![layer],
                })
            }
        }
    }
}

/// A paint layer: `{ kind = …, blend = … }`, or a colour (a literal or an
/// expression) standing for one `Color` layer with `BlendMode::Normal`
/// (S2.2).
impl FromToml for PaintLayer {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        match item {
            Item::Table(fields) => {
                let patch = read_fields(fields, at, cx);
                PaintLayer::build(patch, at, true, cx)
            }
            _ => {
                cx.shorthand(format!("{at}.kind.color"), at);
                Value::read(item, Spelling::Item, at, cx).map(|color| PaintLayer {
                    kind: PaintKind::Color(color),
                    blend: BlendMode::Normal,
                })
            }
        }
    }
}

/// A corner (S2.4, S11): `<corner>_px` for `Radius::Px`, the bare name for
/// the other variants.
impl FromToml for Radius {
    const SPELLINGS: &'static [Spelling] = &[Spelling::Bare, Spelling::Px];

    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        if spelling == Spelling::Px {
            return Value::<Px>::read(item, Spelling::Px, at, cx).map(Radius::Px);
        }
        let (name, payload) = read::variant(item, at, cx)?;
        let maximum_or_minimum = |name: &str, payload: &Item, cx: &mut Reader| {
            let at = join(at, name);
            let fields = table(payload, &at, cx)?;
            let field = if name == "full" { "maximum" } else { "minimum" };
            let mut value = None;
            for (key, item) in fields {
                let at = join(&at, key);
                if read::spelled(field, key, &[Spelling::Px]).is_some() {
                    value = Value::<Px>::read(item, Spelling::Px, &at, cx);
                } else {
                    cx.problem(&at, ProblemKind::UnknownKey);
                }
            }
            read::required(value, &join(&at, field), cx)
        };
        match (name, payload) {
            ("follow_part", None) => Some(Radius::FollowPart),
            ("full", Some(payload)) => {
                maximum_or_minimum(name, payload, cx).map(|maximum| Radius::Full { maximum })
            }
            ("concentric", Some(payload)) => {
                maximum_or_minimum(name, payload, cx).map(|minimum| Radius::Concentric { minimum })
            }
            (_, None) => cx.unknown(at, name),
            (_, Some(_)) => {
                cx.problem(&join(at, name), ProblemKind::UnknownKey);
                None
            }
        }
    }
}

/// A `Corners` field's patch: its table, or the shorthand `<field>_px` for
/// the four corners' `Radius::Px` at once, the curve beside it in the table
/// (S2.4).
impl FromToml for CornersPatch {
    const SPELLINGS: &'static [Spelling] = &[Spelling::Bare, Spelling::Px];

    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        if spelling == Spelling::Px {
            if let Some(field) = at.strip_suffix("_px") {
                for corner in ["top_left", "top_right", "bottom_right", "bottom_left"] {
                    cx.shorthand(format!("{field}.{corner}_px"), at);
                }
            }
            let radius = Value::<Px>::read(item, Spelling::Px, at, cx)?;
            return Some(CornersPatch {
                top_left: Some(Radius::Px(radius.clone())),
                top_right: Some(Radius::Px(radius.clone())),
                bottom_right: Some(Radius::Px(radius.clone())),
                bottom_left: Some(Radius::Px(radius)),
                curve: None,
            });
        }
        let fields = table(item, at, cx)?;
        Some(read_fields(fields, at, cx))
    }

    fn combine(&mut self, later: Self) -> bool {
        read::Fields::combine_fields(self, later)
    }
}

impl FromToml for Corners {
    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let patch = CornersPatch::read(item, spelling, at, cx)?;
        Corners::build(patch, at, true, cx)
    }
}

/// A stroke's patch (S2.4): its table, whose keys `width_px`, `paint`, `dash`
/// and `snap` each set that field on all four edges; an edge's table then
/// states none of them (a duplicate key).
impl FromToml for StrokePatch {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let fields = table(item, at, cx)?;
        let mut patch = StrokePatch::default();
        let mut every = EdgePatch::default();
        for (key, item) in fields {
            let key_at = join(at, key);
            match key.as_str() {
                "width_px" => every.width = Value::read(item, Spelling::Px, &key_at, cx),
                "paint" => every.paint = PaintPatch::read(item, Spelling::Bare, &key_at, cx),
                "dash" => every.dash = Value::read(item, Spelling::Bare, &key_at, cx),
                "snap" => every.snap = PixelSnap::read(item, Spelling::Bare, &key_at, cx),
                _ => {
                    if !read::Fields::read_entry(&mut patch, key, item, at, cx) {
                        cx.problem(&key_at, ProblemKind::UnknownKey);
                    }
                }
            }
        }
        for (name, edge) in [
            ("top", &mut patch.top),
            ("right", &mut patch.right),
            ("bottom", &mut patch.bottom),
            ("left", &mut patch.left),
        ] {
            let edge_at = join(at, name);
            for (key, set) in [
                ("width_px", every.width.is_some()),
                ("paint", every.paint.is_some()),
                ("dash", every.dash.is_some()),
                ("snap", every.snap.is_some()),
            ] {
                if set {
                    cx.shorthand(join(&edge_at, key), &join(at, key));
                }
            }
            let at = edge_at;
            let edge = edge.get_or_insert_with(EdgePatch::default);
            shorthand(&mut edge.width, &every.width, "width_px", &at, cx);
            shorthand(&mut edge.paint, &every.paint, "paint", &at, cx);
            shorthand(&mut edge.dash, &every.dash, "dash", &at, cx);
            shorthand(&mut edge.snap, &every.snap, "snap", &at, cx);
        }
        for edge in [
            &mut patch.top,
            &mut patch.right,
            &mut patch.bottom,
            &mut patch.left,
        ] {
            if edge.as_ref().is_some_and(|e| *e == EdgePatch::default()) {
                *edge = None;
            }
        }
        read::Fields::finish_fields(&mut patch, at, cx);
        Some(patch)
    }
}

/// A stroke shorthand's value set on one edge, unless the edge's own table
/// states it too: a duplicate key.
fn shorthand<T: Clone>(
    slot: &mut Option<T>,
    every: &Option<T>,
    key: &str,
    at: &str,
    cx: &mut Reader,
) {
    if let Some(value) = every {
        if slot.is_some() {
            cx.problem(
                &join(at, key),
                ProblemKind::DuplicateKey {
                    other: key.to_owned(),
                },
            );
        } else {
            *slot = Some(value.clone());
        }
    }
}

impl FromToml for Stroke {
    fn read(item: &Item, spelling: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let patch = StrokePatch::read(item, spelling, at, cx)?;
        Stroke::build(patch, at, true, cx)
    }
}

/// A typed widget by its name.
pub(crate) fn widget_named(name: &str, at: &str, cx: &mut Reader) -> Option<WidgetKind> {
    registry::widget(name)
        .map(|widget| widget.kind)
        .or_else(|| cx.unknown(at, name))
}

/// The typed widget a table's key names; an unknown key where it names
/// none.
pub(crate) fn widget_key(name: &str, at: &str, cx: &mut Reader) -> Option<WidgetKind> {
    let widget = registry::widget(name).map(|widget| widget.kind);
    if widget.is_none() {
        cx.problem(at, ProblemKind::UnknownKey);
    }
    widget
}

/// A widget (S1): a typed widget's name; a keyed component, as
/// `component.<key>`, is no name while no unit enables a component.
impl FromToml for WidgetRef {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let name = text(item, at, cx)?;
        widget_named(name, at, cx).map(WidgetRef::Typed)
    }
}

/// The part `part` of the typed widget `widget`, by its bare name.
pub(crate) fn part_named(
    widget: WidgetKind,
    part: &str,
    at: &str,
    cx: &mut Reader,
) -> Option<TypedPart> {
    TypedPart::from_names(widget, part).or_else(|| cx.unknown(at, part))
}

/// A part (S1): `<widget>.<part>`, or a bare part name of the widget whose
/// table is read.
impl FromToml for PartRef {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let name = text(item, at, cx)?;
        let (widget, part) = match name.split_once('.') {
            Some((widget, part)) => (widget_named(widget, at, cx)?, part),
            None => match cx.widget {
                Some(widget) => (widget, name),
                None => {
                    return cx.mistyped(at, "`<widget>.<part>`: no widget's table is read here");
                }
            },
        };
        part_named(widget, part, at, cx).map(PartRef::Typed)
    }
}

/// The widget or part the keys `widget` and `part` of a table spell (S1,
/// S11): `part`, a bare name of `widget`'s parts, is stated only with
/// `widget` (S12).
pub(crate) fn widget_or_part(
    widget: Option<&Item>,
    part: Option<&Item>,
    at: &str,
    cx: &mut Reader,
) -> Option<Option<WidgetOrPart>> {
    match (widget, part) {
        (None, None) => Some(None),
        (None, Some(_)) => {
            cx.problem(&join(at, "part"), ProblemKind::PartWithoutWidget);
            None
        }
        (Some(widget), part) => {
            let widget_at = join(at, "widget");
            let widget = WidgetRef::read(widget, Spelling::Bare, &widget_at, cx)?;
            match (widget, part) {
                (widget, None) => Some(Some(WidgetOrPart::Widget(widget))),
                (WidgetRef::Typed(kind), Some(part)) => {
                    let at = join(at, "part");
                    let name = text(part, &at, cx)?;
                    part_named(kind, name, &at, cx)
                        .map(|part| Some(WidgetOrPart::Part(PartRef::Typed(part))))
                }
                (WidgetRef::Component(_), Some(_)) => None,
            }
        }
    }
}

/// A variant (S10): a typed variant's name, or `@<key>` for a keyed one.
impl FromToml for VariantRef {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let name = text(item, at, cx)?;
        if name.starts_with('@') {
            return read::keyed(name, at, cx).map(VariantRef::Keyed);
        }
        read::named::<Variant>(item, at, cx).map(VariantRef::Typed)
    }
}

impl FromToml for SurfaceSet {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let name = text(item, at, cx)?;
        SurfaceSet::map_key(name, at, cx)
    }
}

/// A surface set's role, by its path (`disabled.foreground`).
impl FromToml for SurfaceRole {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let path = text(item, at, cx)?;
        SurfaceRole::from_path(path).or_else(|| cx.unknown(at, path))
    }
}

/// A global role, by its path (`status.error.fill`).
impl FromToml for GlobalRole {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let path = text(item, at, cx)?;
        GlobalRole::from_path(path).or_else(|| cx.unknown(at, path))
    }
}

/// An icon set by its name in snake_case (`sf_symbols`, S11).
impl FromToml for IconSet {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let name = text(item, at, cx)?;
        (!name.contains('-'))
            .then(|| IconSet::from_name(&name.replace('_', "-")))
            .flatten()
            .or_else(|| cx.unknown(at, name))
    }
}

/// An icon role by its name in snake_case (`action_save`, S11).
impl MapKey for IconRole {
    fn map_key(name: &str, at: &str, cx: &mut Reader) -> Option<Self> {
        let kebab = name.replace('_', "-");
        IconRole::ALL
            .iter()
            .copied()
            .find(|role| !name.contains('-') && role.name() == kebab)
            .or_else(|| cx.unknown(at, name))
    }

    fn key_name(&self) -> String {
        self.name().replace('-', "_")
    }
}

impl FromToml for IconRole {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let name = text(item, at, cx)?;
        IconRole::map_key(name, at, cx)
    }
}

/// A BCP 47 language tag.
impl MapKey for LanguageTag {
    fn map_key(name: &str, at: &str, cx: &mut Reader) -> Option<Self> {
        LanguageTag::new(name).or_else(|| cx.mistyped(at, "a BCP 47 language tag"))
    }

    fn key_name(&self) -> String {
        self.as_str().to_owned()
    }
}

impl FromToml for LanguageTag {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let name = text(item, at, cx)?;
        LanguageTag::map_key(name, at, cx)
    }
}

/// An OpenType axis tag.
impl MapKey for AxisTag {
    fn map_key(name: &str, at: &str, cx: &mut Reader) -> Option<Self> {
        AxisTag::new(name).or_else(|| cx.mistyped(at, "a four-letter OpenType tag"))
    }

    fn key_name(&self) -> String {
        self.to_string()
    }
}

/// An OpenType feature tag.
impl MapKey for FeatureTag {
    fn map_key(name: &str, at: &str, cx: &mut Reader) -> Option<Self> {
        FeatureTag::new(name).or_else(|| cx.mistyped(at, "a four-letter OpenType tag"))
    }

    fn key_name(&self) -> String {
        self.to_string()
    }
}

/// A path's view box: `{ x, y, width, height }`, unitless (S2.9).
impl FromToml for ViewBox {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let fields = table(item, at, cx)?;
        let mut members = [None; 4];
        let names = ["x", "y", "width", "height"];
        for (key, value) in fields {
            let at = join(at, key);
            match names
                .iter()
                .zip(members.iter_mut())
                .find(|(name, _)| *name == key)
            {
                Some((_, slot)) => *slot = read::number(value, &at, cx),
                None => cx.problem(&at, ProblemKind::UnknownKey),
            }
        }
        let [x, y, width, height] = members;
        let x = read::required(x, &join(at, "x"), cx);
        let y = read::required(y, &join(at, "y"), cx);
        let width = read::required(width, &join(at, "width"), cx);
        let height = read::required(height, &join(at, "height"), cx);
        Some(ViewBox {
            x: x?,
            y: y?,
            width: width?,
            height: height?,
        })
    }
}
