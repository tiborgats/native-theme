//! A rule's `when` (S7, S11): the selector's conditions, context, position,
//! variants, sizes, densities, size classes, direction and states.

use ::toml::{Table, Value as Item};

use super::model::{widget_named, widget_or_part};
use super::read::{self, FromToml, Named, Reader, Spelling, join, table, text};
use crate::next::error::ProblemKind;
use crate::next::presets::PresetId;
use crate::next::roles::SurfaceSet;
use crate::next::selector::{
    Condition, ContextMatch, ContextScope, ControlSize, Density, Direction, Idiom, InputKind,
    Parity, PositionMatch, Posture, Scheme, Selector, SharedFlag, SiblingMatch, SiblingScope,
    SizeClass, SizeClassMatch, StateFlag, TiledEdges, VariantRef, WidgetRef,
};

/// A rule's `when` table (S11).
pub(crate) fn read_selector(item: &Item, at: &str, cx: &mut Reader) -> Option<Selector> {
    let fields = table(item, at, cx)?;
    let mut selector = Selector::default();
    for (key, item) in fields {
        let at = join(at, key);
        match key.as_str() {
            "conditions" => selector.conditions.all_of = names(item, &at, cx, condition),
            "not_conditions" => selector.conditions.none_of = names(item, &at, cx, condition),
            "context" => selector.context = contexts(item, &at, cx),
            "not_context" => selector.not_context = contexts(item, &at, cx),
            "position" => selector.position = position(item, &at, cx),
            "variants" => selector.variants.all_of = names(item, &at, cx, variant),
            "not_variants" => selector.variants.none_of = names(item, &at, cx, variant),
            "sizes" => selector.size.sizes.all_of = names(item, &at, cx, named::<ControlSize>),
            "not_sizes" => selector.size.sizes.none_of = names(item, &at, cx, named::<ControlSize>),
            "densities" => selector.size.densities.all_of = names(item, &at, cx, density),
            "not_densities" => selector.size.densities.none_of = names(item, &at, cx, density),
            "size_class" => {
                if let Some(classes) = size_classes(item, &at, cx) {
                    selector.size.size_classes = classes;
                }
            }
            "direction" => selector.direction = read::named::<Direction>(item, &at, cx),
            "states" => selector.states.all_of = names(item, &at, cx, state),
            "not_states" => selector.states.none_of = names(item, &at, cx, state),
            "latest" => selector.latest = latest(item, &at, cx),
            _ => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    Some(selector)
}

/// `latest`: a table, one key per recency set and its flag as the value
/// (S7.2, S11).
fn latest(item: &Item, at: &str, cx: &mut Reader) -> Vec<(crate::next::key::Key, StateFlag)> {
    let Some(sets) = table(item, at, cx) else {
        return Vec::new();
    };
    sets.iter()
        .filter_map(|(name, item)| {
            let at = join(at, name);
            let set = read::key_named(name, &at, cx);
            let flag = text(item, &at, cx).and_then(|flag| state(flag, &at, cx));
            Some((set?, flag?))
        })
        .collect()
}

/// An array of names, each read by `name`; an item that names nothing is
/// recorded and left out.
pub(crate) fn names<T>(
    item: &Item,
    at: &str,
    cx: &mut Reader,
    name: fn(&str, &str, &mut Reader) -> Option<T>,
) -> Vec<T> {
    let Some(items) = read::array(item, at, cx) else {
        return Vec::new();
    };
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            let at = read::item(at, index);
            let text = text(item, &at, cx)?;
            name(text, &at, cx)
        })
        .collect()
}

/// A unit-only enum's variant by its name.
pub(crate) fn named<T: Named>(name: &str, at: &str, cx: &mut Reader) -> Option<T> {
    T::from_name(name).or_else(|| cx.unknown(at, name))
}

/// A condition (S7.1, S11): `"<name>"`, or `"<name>:<value>"`.
pub(crate) fn condition(text: &str, at: &str, cx: &mut Reader) -> Option<Condition> {
    let (name, value) = match text.split_once(':') {
        Some((name, value)) => (name, Some(value)),
        None => (text, None),
    };
    let plain = match name {
        "contrast_increased" => Some(Condition::ContrastIncreased),
        "contrast_high" => Some(Condition::ContrastHigh),
        "reduced_transparency" => Some(Condition::ReducedTransparency),
        "reduced_motion" => Some(Condition::ReducedMotion),
        "animations_off" => Some(Condition::AnimationsOff),
        "bold_text" => Some(Condition::BoldText),
        "button_shapes" => Some(Condition::ButtonShapes),
        "differentiate_without_color" => Some(Condition::DifferentiateWithoutColor),
        "tablet_mode" => Some(Condition::TabletMode),
        "mobile_form_factor" => Some(Condition::MobileFormFactor),
        "battery_saver" => Some(Condition::BatterySaver),
        "overlay_scrollbars" => Some(Condition::OverlayScrollbars),
        "on_off_labels" => Some(Condition::OnOffLabels),
        "full_keyboard_access" => Some(Condition::FullKeyboardAccess),
        "prefer_cross_fade" => Some(Condition::PreferCrossFade),
        "accessibility_text_size" => Some(Condition::AccessibilityTextSize),
        "window_inactive" => Some(Condition::WindowInactive),
        "not_main" => Some(Condition::NotMain),
        "app_inactive" => Some(Condition::AppInactive),
        "maximized" => Some(Condition::Maximized),
        "full_screen" => Some(Condition::FullScreen),
        "elevated" => Some(Condition::Elevated),
        "not_composited" => Some(Condition::NotComposited),
        "shaded" => Some(Condition::Shaded),
        "attention_flash" => Some(Condition::AttentionFlash),
        "tint_dimmed" => Some(Condition::TintDimmed),
        "mnemonics_visible" => Some(Condition::MnemonicsVisible),
        "freeform" => Some(Condition::Freeform),
        "unit_scale" => Some(Condition::UnitScale),
        _ => None,
    };
    match (plain, value) {
        (Some(condition), None) => return Some(condition),
        (Some(_), Some(_)) => return cx.mistyped(at, "a condition with no `:<value>`"),
        (None, _) => {}
    }
    let Some(value) = value else {
        return match name {
            "scheme" | "container_size" | "idiom" | "system_setting" | "device_model" | "tiled"
            | "input" | "posture" | "flavor" | "preset" => {
                cx.mistyped(at, "a condition and its value, `<name>:<value>`")
            }
            _ => cx.unknown(at, text),
        };
    };
    match name {
        "scheme" => named::<Scheme>(value, at, cx).map(Condition::Scheme),
        "idiom" => named::<Idiom>(value, at, cx).map(Condition::Idiom),
        "input" => named::<InputKind>(value, at, cx).map(Condition::Input),
        "posture" => named::<Posture>(value, at, cx).map(Condition::Posture),
        "system_setting" => read::key_named(value, at, cx).map(Condition::SystemSetting),
        "device_model" => read::key_named(value, at, cx).map(Condition::DeviceModel),
        "flavor" => read::key_named(value, at, cx).map(Condition::Flavor),
        "preset" => PresetId::from_key(value)
            .map(Condition::Preset)
            .or_else(|| cx.unknown(at, value)),
        "tiled" => tiled(value, at, cx).map(Condition::Tiled),
        "container_size" => {
            let Some((container, size)) = value.split_once(':') else {
                return cx.mistyped(at, "`container_size:<container>:<size>`");
            };
            let container = widget_named(container, at, cx).map(WidgetRef::Typed);
            let size = read::key_named(size, at, cx);
            Some(Condition::ContainerSize {
                container: container?,
                size: size?,
            })
        }
        _ => cx.unknown(at, text),
    }
}

/// `tiled:<edge>,…`: the edges a window is tiled to.
fn tiled(value: &str, at: &str, cx: &mut Reader) -> Option<TiledEdges> {
    let mut edges = TiledEdges::default();
    for edge in value.split(',') {
        let slot = match edge {
            "top" => &mut edges.top,
            "right" => &mut edges.right,
            "bottom" => &mut edges.bottom,
            "left" => &mut edges.left,
            _ => return cx.unknown(at, edge),
        };
        if *slot {
            return cx.mistyped(at, "each edge once");
        }
        *slot = true;
    }
    Some(edges)
}

/// A variant: a typed variant's name, or `@<key>` (S10).
pub(crate) fn variant(name: &str, at: &str, cx: &mut Reader) -> Option<VariantRef> {
    VariantRef::read(&Item::String(name.to_owned()), Spelling::Bare, at, cx)
}

/// A density: `default`, `compact`, or `@<key>` (S7.3).
pub(crate) fn density(name: &str, at: &str, cx: &mut Reader) -> Option<Density> {
    match name {
        "default" => Some(Density::Default),
        "compact" => Some(Density::Compact),
        _ if name.starts_with('@') => read::keyed(name, at, cx).map(Density::Keyed),
        _ => cx.unknown(at, name),
    }
}

/// A state flag (S7.2): a typed flag's or a shared flag's name, or
/// `@<key>` for a flag the preset declares.
pub(crate) fn state(name: &str, at: &str, cx: &mut Reader) -> Option<StateFlag> {
    if let Some(flag) = StateFlag::typed(name) {
        return Some(flag);
    }
    if name.starts_with('@') {
        return read::keyed(name, at, cx).map(StateFlag::Widget);
    }
    named::<SharedFlag>(name, at, cx).map(StateFlag::Shared)
}

/// A state flag as a value (a transition's `from`): its name, as a selector
/// writes it.
impl FromToml for StateFlag {
    fn read(item: &Item, _: Spelling, at: &str, cx: &mut Reader) -> Option<Self> {
        let name = text(item, at, cx)?;
        state(name, at, cx)
    }
}

/// `context` and `not_context`: arrays of ancestor matches.
fn contexts(item: &Item, at: &str, cx: &mut Reader) -> Vec<ContextMatch> {
    let Some(items) = read::array(item, at, cx) else {
        return Vec::new();
    };
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| context(item, &read::item(at, index), cx))
        .collect()
}

/// One ancestor match: `{ scope, widget, part, surface, sizes, not_sizes,
/// variants, not_variants, states, not_states }`; `scope` left out is any
/// ancestor, as a bare `context w` is `context any w` (the catalogue's
/// selector notation).
fn context(item: &Item, at: &str, cx: &mut Reader) -> Option<ContextMatch> {
    let fields = table(item, at, cx)?;
    let mut scope = Some(ContextScope::AnyAncestor);
    let mut matched = ContextMatch::new(ContextScope::AnyAncestor);
    for (key, item) in fields {
        let at = join(at, key);
        match key.as_str() {
            "scope" => scope = context_scope(item, &at, cx),
            "widget" | "part" => {}
            "surface" => matched.surface = SurfaceSet::read(item, Spelling::Bare, &at, cx),
            "sizes" => matched.sizes.all_of = names(item, &at, cx, named::<ControlSize>),
            "not_sizes" => matched.sizes.none_of = names(item, &at, cx, named::<ControlSize>),
            "variants" => matched.variants.all_of = names(item, &at, cx, variant),
            "not_variants" => matched.variants.none_of = names(item, &at, cx, variant),
            "states" => matched.states.all_of = names(item, &at, cx, state),
            "not_states" => matched.states.none_of = names(item, &at, cx, state),
            _ => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    let target = widget_or_part(fields.get("widget"), fields.get("part"), at, cx);
    matched.scope = scope?;
    matched.target = target?;
    Some(matched)
}

/// `scope = "any" | { nearest = [ … ] }`.
fn context_scope(item: &Item, at: &str, cx: &mut Reader) -> Option<ContextScope> {
    match item {
        Item::String(name) if name == "any" => Some(ContextScope::AnyAncestor),
        Item::Table(fields) if fields.len() == 1 && fields.contains_key("nearest") => {
            let at = join(at, "nearest");
            let widgets = fields.get("nearest")?;
            Vec::<WidgetRef>::read(widgets, Spelling::Bare, &at, cx).map(ContextScope::NearestAmong)
        }
        _ => cx.mistyped(at, "\"any\" or `{ nearest = [ … ] }`"),
    }
}

/// `position = { among, first, last, only, parity, previous, next }`.
fn position(item: &Item, at: &str, cx: &mut Reader) -> Option<PositionMatch> {
    let fields = table(item, at, cx)?;
    let mut position = PositionMatch::default();
    for (key, item) in fields {
        let at = join(at, key);
        match key.as_str() {
            "among" => {
                if let Some(among) = among(item, &at, cx) {
                    position.among = among;
                }
            }
            "first" => position.first = bool::read(item, Spelling::Bare, &at, cx),
            "last" => position.last = bool::read(item, Spelling::Bare, &at, cx),
            "only" => position.only = bool::read(item, Spelling::Bare, &at, cx),
            "parity" => position.parity = read::named::<Parity>(item, &at, cx),
            "previous" => position.previous = sibling(item, &at, cx),
            "next" => position.next = sibling(item, &at, cx),
            _ => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    Some(position)
}

/// `among = "same" | "any" | [ <widget>, … ]`.
fn among(item: &Item, at: &str, cx: &mut Reader) -> Option<SiblingScope> {
    match item {
        Item::String(name) if name == "same" => Some(SiblingScope::SameKind),
        Item::String(name) if name == "any" => Some(SiblingScope::Any),
        Item::Array(_) => {
            Vec::<WidgetRef>::read(item, Spelling::Bare, at, cx).map(SiblingScope::Kinds)
        }
        _ => cx.mistyped(at, "\"same\", \"any\" or an array of widgets"),
    }
}

/// `previous` and `next`: `{ widget, part, variants, not_variants, states,
/// not_states }`.
fn sibling(item: &Item, at: &str, cx: &mut Reader) -> Option<SiblingMatch> {
    let fields = table(item, at, cx)?;
    let mut matched = SiblingMatch::default();
    for (key, item) in fields {
        let at = join(at, key);
        match key.as_str() {
            "widget" | "part" => {}
            "variants" => matched.variants.all_of = names(item, &at, cx, variant),
            "not_variants" => matched.variants.none_of = names(item, &at, cx, variant),
            "states" => matched.states.all_of = names(item, &at, cx, state),
            "not_states" => matched.states.none_of = names(item, &at, cx, state),
            _ => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    matched.target = widget_or_part(fields.get("widget"), fields.get("part"), at, cx)?;
    Some(matched)
}

/// `size_class = { horizontal, not_horizontal, vertical, not_vertical }`.
fn size_classes(item: &Item, at: &str, cx: &mut Reader) -> Option<SizeClassMatch> {
    let fields: &Table = table(item, at, cx)?;
    let mut classes = SizeClassMatch::default();
    for (key, item) in fields {
        let at = join(at, key);
        let list = names(item, &at, cx, named::<SizeClass>);
        match key.as_str() {
            "horizontal" => classes.horizontal.all_of = list,
            "not_horizontal" => classes.horizontal.none_of = list,
            "vertical" => classes.vertical.all_of = list,
            "not_vertical" => classes.vertical.none_of = list,
            _ => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    Some(classes)
}
