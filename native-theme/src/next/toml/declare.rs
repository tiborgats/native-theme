//! A preset's `[declare]` table (S10).

use ::toml::{Table, Value as Item};

use super::model::{part_named, widget_key};
use super::read::{self, FromToml, MapKey, Reader, Spelling, join, read_map, table, text};
use super::when::{density, named, names, state, variant};
use crate::next::declare::{
    DataName, DataOwner, Declarations, DefaultSizeByVariant, FlavorDecl, FontMetric, ParamGroup,
    ScalarUnit, SettingDecl, SizesByContext, SizesByVariant, WidgetDecl,
};
use crate::next::error::ProblemKind;
use crate::next::expr::ElementDatum;
use crate::next::inputs::InputName;
use crate::next::key::Key;
use crate::next::registry;
use crate::next::selector::{
    Breakpoint, Combine, Comparison, ControlSize, Measured, PartRef, Scheme, VariantRef, WidgetRef,
};
use crate::next::units::Px;
use crate::next::value::Value;
use crate::next::widgets::WidgetKind;

/// `[declare]` (S10).
pub(crate) fn read_declarations(item: &Item, at: &str, cx: &mut Reader) -> Option<Declarations> {
    let fields = table(item, at, cx)?;
    let mut out = Declarations::default();
    for (key, item) in fields {
        let at = join(at, key);
        let keys = |cx: &mut Reader| keys(item, &at, cx);
        match key.as_str() {
            "flavors" => out.flavors = keys(cx),
            "materials" => out.materials = keys(cx),
            "color_roles" => out.color_roles = keys(cx),
            "text_styles" => out.text_styles = keys(cx),
            "elevation" => out.elevation = keys(cx),
            "blend_modes" => out.blend_modes = keys(cx),
            "timings" => out.timings = keys(cx),
            "presentation_events" => out.presentation_events = keys(cx),
            "symbol_variants" => out.symbol_variants = keys(cx),
            "symbol_scales" => out.symbol_scales = keys(cx),
            "icon_state_effects" => out.icon_state_effects = keys(cx),
            "focus_kinds" => out.focus_kinds = keys(cx),
            "device_models" => out.device_models = keys(cx),
            "durations" => out.durations = keys(cx),
            "icon_contexts" => out.icon_contexts = keys(cx),
            "status_colors" => out.status_colors = keys(cx),
            "accent_ramp" => out.accent_ramp = keys(cx),
            "rendering_engines" => out.rendering_engines = keys(cx),
            "lcd_filters" => out.lcd_filters = keys(cx),
            "surface_sets" => {
                out.surface_sets = Vec::read(item, Spelling::Bare, &at, cx).unwrap_or_default();
            }
            "schemes" => out.schemes = names(item, &at, cx, named::<Scheme>),
            "conditions" => out.conditions = settings(item, &at, cx),
            "params" => out.params = params(item, &at, cx),
            "inputs" => out.inputs = names(item, &at, cx, input),
            "accent_inputs" => out.accent_inputs = names(item, &at, cx, input),
            "transition_kinds" => out.transition_kinds = kinds(item, &at, cx),
            "presentation_kinds" => out.presentation_kinds = kinds(item, &at, cx),
            "container_sizes" => out.container_sizes = container_sizes(item, &at, cx),
            "material_params" => {
                out.material_params =
                    read_map::<Key, ScalarUnit>(item, &at, cx).unwrap_or_default();
            }
            "widget" => out.widgets = widgets(item, &at, cx),
            "flavor" => out.flavor = flavors(item, &at, cx),
            _ => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    Some(out)
}

/// A list of keys, each its own name (`["mica", "mica_alt"]`).
fn keys(item: &Item, at: &str, cx: &mut Reader) -> Vec<Key> {
    names(item, at, cx, read::key_named)
}

/// An input's path.
fn input(name: &str, at: &str, cx: &mut Reader) -> Option<InputName> {
    InputName::new(name)
        .map_err(|error| cx.problem(at, ProblemKind::NotAKey(error)))
        .ok()
}

/// `conditions = ["@<key>", { key = "@<key>", for_applications = true }]`.
fn settings(item: &Item, at: &str, cx: &mut Reader) -> Vec<SettingDecl> {
    let Some(items) = read::array(item, at, cx) else {
        return Vec::new();
    };
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            let at = read::item(at, index);
            match item {
                Item::String(name) => setting_key(name, &at, cx).map(|key| SettingDecl {
                    key,
                    for_applications: false,
                }),
                Item::Table(fields) => {
                    let mut key = None;
                    let mut for_applications = None;
                    for (name, item) in fields {
                        let at = join(&at, name);
                        match name.as_str() {
                            "key" => {
                                key =
                                    text(item, &at, cx).and_then(|name| setting_key(name, &at, cx));
                            }
                            "for_applications" => {
                                for_applications = bool::read(item, Spelling::Bare, &at, cx);
                            }
                            _ => cx.problem(&at, ProblemKind::UnknownKey),
                        }
                    }
                    let key = read::required(key, &join(&at, "key"), cx);
                    let for_applications =
                        read::required(for_applications, &join(&at, "for_applications"), cx);
                    Some(SettingDecl {
                        key: key?,
                        for_applications: for_applications?,
                    })
                }
                _ => cx.mistyped(&at, "\"@<key>\" or `{ key, for_applications }`"),
            }
        })
        .collect()
}

/// A system setting's key, `@<key>`.
fn setting_key(name: &str, at: &str, cx: &mut Reader) -> Option<Key> {
    if name.starts_with('@') {
        read::keyed(name, at, cx)
    } else {
        cx.mistyped(at, "a keyed name, `@<key>`")
    }
}

/// `params = { <group> = { unit = "<unit>", keys = [ … ] } }`.
fn params(item: &Item, at: &str, cx: &mut Reader) -> Vec<(Key, ParamGroup)> {
    let Some(groups) = table(item, at, cx) else {
        return Vec::new();
    };
    groups
        .iter()
        .filter_map(|(name, item)| {
            let at = join(at, name);
            let group = read::key_named(name, &at, cx);
            let fields = table(item, &at, cx)?;
            let mut unit = None;
            let mut keys_read = None;
            for (key, item) in fields {
                let at = join(&at, key);
                match key.as_str() {
                    "unit" => unit = ScalarUnit::read(item, Spelling::Bare, &at, cx),
                    "keys" => keys_read = Some(keys(item, &at, cx)),
                    _ => cx.problem(&at, ProblemKind::UnknownKey),
                }
            }
            let unit = read::required(unit, &join(&at, "unit"), cx);
            let keys = read::required(keys_read, &join(&at, "keys"), cx);
            Some((
                group?,
                ParamGroup {
                    unit: unit?,
                    keys: keys?,
                },
            ))
        })
        .collect()
}

/// `transition_kinds` and `presentation_kinds`: `{ <kind> = { <param> =
/// "<unit>" } }`.
fn kinds(item: &Item, at: &str, cx: &mut Reader) -> Vec<(Key, Vec<(Key, ScalarUnit)>)> {
    let Some(kinds) = table(item, at, cx) else {
        return Vec::new();
    };
    kinds
        .iter()
        .filter_map(|(name, item)| {
            let at = join(at, name);
            let kind = read::key_named(name, &at, cx);
            let params = read_map::<Key, ScalarUnit>(item, &at, cx);
            Some((kind?, params?))
        })
        .collect()
}

/// `container_sizes = { <container> = [ <size>, … ] }`, the first the
/// default.
fn container_sizes(item: &Item, at: &str, cx: &mut Reader) -> Vec<(WidgetRef, Vec<Key>)> {
    let Some(containers) = table(item, at, cx) else {
        return Vec::new();
    };
    containers
        .iter()
        .filter_map(|(name, item)| {
            let at = join(at, name);
            let container = widget_key(name, &at, cx).map(WidgetRef::Typed);
            let sizes = keys(item, &at, cx);
            Some((container?, sizes))
        })
        .collect()
}

/// `[declare.widget.<widget>]` sections.
fn widgets(item: &Item, at: &str, cx: &mut Reader) -> Vec<(WidgetKind, WidgetDecl)> {
    let Some(widgets) = table(item, at, cx) else {
        return Vec::new();
    };
    widgets
        .iter()
        .filter_map(|(name, item)| {
            let at = join(at, name);
            let widget = widget_key(name, &at, cx)?;
            let fields = table(item, &at, cx)?;
            Some((widget, widget_decl(widget, fields, &at, cx)))
        })
        .collect()
}

/// `[declare.flavor.<flavor>.widget.<widget>]` sections.
fn flavors(item: &Item, at: &str, cx: &mut Reader) -> Vec<(Key, FlavorDecl)> {
    let Some(flavors) = table(item, at, cx) else {
        return Vec::new();
    };
    flavors
        .iter()
        .filter_map(|(name, item)| {
            let at = join(at, name);
            let flavor = read::key_named(name, &at, cx);
            let fields = table(item, &at, cx)?;
            let mut decl = FlavorDecl::default();
            for (key, item) in fields {
                let at = join(&at, key);
                match key.as_str() {
                    "widget" => decl.widgets = widgets(item, &at, cx),
                    _ => cx.problem(&at, ProblemKind::UnknownKey),
                }
            }
            Some((flavor?, decl))
        })
        .collect()
}

/// One widget's declaration (S10).
fn widget_decl(widget: WidgetKind, fields: &Table, at: &str, cx: &mut Reader) -> WidgetDecl {
    let outer = cx.widget.replace(widget);
    let mut decl = WidgetDecl::default();
    for (key, item) in fields {
        let at = join(at, key);
        match key.as_str() {
            "variants" => decl.variants = Some(names(item, &at, cx, variant)),
            "sizes" => decl.sizes = Some(names(item, &at, cx, named::<ControlSize>)),
            "densities" => decl.densities = Some(names(item, &at, cx, density)),
            "flags" => decl.flags = Some(names(item, &at, cx, state)),
            "exclusive" => decl.exclusive = Some(groups(item, &at, cx, state)),
            "exclusive_variants" => decl.exclusive_variants = Some(groups(item, &at, cx, variant)),
            "recency" => decl.recency = Some(recency(item, &at, cx)),
            "sizes_by_context" => decl.sizes_by_context = Some(sizes_by_context(item, &at, cx)),
            "sizes_by_variant" => decl.sizes_by_variant = Some(sizes_by_variant(item, &at, cx)),
            "default_size" => decl.default_size = read::named::<ControlSize>(item, &at, cx),
            "default_size_by_variant" => {
                decl.default_size_by_variant = Some(default_size_by_variant(item, &at, cx));
            }
            "available" => decl.available = bool::read(item, Spelling::Bare, &at, cx),
            "requires_variants" => decl.requires_variants = Some(names(item, &at, cx, variant)),
            "data" => decl.data = Some(data(widget, item, &at, cx)),
            "keys" => decl.keys = Some(value_keys(widget, item, &at, cx)),
            "breakpoints" => decl.breakpoints = Some(breakpoints(item, &at, cx)),
            _ => cx.problem(&at, ProblemKind::UnknownKey),
        }
    }
    cx.widget = outer;
    decl
}

/// An array of arrays of names.
fn groups<T>(
    item: &Item,
    at: &str,
    cx: &mut Reader,
    name: fn(&str, &str, &mut Reader) -> Option<T>,
) -> Vec<Vec<T>> {
    let Some(items) = read::array(item, at, cx) else {
        return Vec::new();
    };
    items
        .iter()
        .enumerate()
        .map(|(index, item)| names(item, &read::item(at, index), cx, name))
        .collect()
}

/// `recency`: a table of the recency sets, each a key and its flags in the
/// stated order (S10).
fn recency(
    item: &Item,
    at: &str,
    cx: &mut Reader,
) -> Vec<(Key, Vec<crate::next::selector::StateFlag>)> {
    let Some(sets) = table(item, at, cx) else {
        return Vec::new();
    };
    sets.iter()
        .filter_map(|(name, item)| {
            let at = join(at, name);
            let key = read::key_named(name, &at, cx);
            let flags = names(item, &at, cx, state);
            Some((key?, flags))
        })
        .collect()
}

/// The tables of an array of tables, each read by `read` with its fields
/// and path.
fn tables<T>(
    item: &Item,
    at: &str,
    cx: &mut Reader,
    read: impl Fn(&Table, &str, &mut Reader) -> Option<T>,
) -> Vec<T> {
    let Some(items) = read::array(item, at, cx) else {
        return Vec::new();
    };
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            let at = read::item(at, index);
            let fields = table(item, &at, cx)?;
            read(fields, &at, cx)
        })
        .collect()
}

/// A table's keys `wanted`, each item found, every other key unknown.
fn take<'t, const N: usize>(
    fields: &'t Table,
    wanted: [&str; N],
    at: &str,
    cx: &mut Reader,
) -> [Option<&'t Item>; N] {
    for key in fields.keys().filter(|key| !wanted.contains(&key.as_str())) {
        cx.problem(&join(at, key), ProblemKind::UnknownKey);
    }
    wanted.map(|key| fields.get(key))
}

/// `sizes_by_context = [{ context = "<widget>", sizes = [ … ] }]`.
fn sizes_by_context(item: &Item, at: &str, cx: &mut Reader) -> Vec<SizesByContext> {
    tables(item, at, cx, |fields, at, cx| {
        let [context, sizes] = take(fields, ["context", "sizes"], at, cx);
        let context = read::required(context, &join(at, "context"), cx)
            .and_then(|item| WidgetRef::read(item, Spelling::Bare, &join(at, "context"), cx));
        let sizes = read::required(sizes, &join(at, "sizes"), cx)
            .map(|item| names(item, &join(at, "sizes"), cx, named::<ControlSize>));
        Some(SizesByContext {
            context: context?,
            sizes: sizes?,
        })
    })
}

/// `sizes_by_variant = [{ variant = "<v>", sizes = [ … ] }]`.
fn sizes_by_variant(item: &Item, at: &str, cx: &mut Reader) -> Vec<SizesByVariant> {
    tables(item, at, cx, |fields, at, cx| {
        let [variant_item, sizes] = take(fields, ["variant", "sizes"], at, cx);
        let variant = read::required(variant_item, &join(at, "variant"), cx)
            .and_then(|item| VariantRef::read(item, Spelling::Bare, &join(at, "variant"), cx));
        let sizes = read::required(sizes, &join(at, "sizes"), cx)
            .map(|item| names(item, &join(at, "sizes"), cx, named::<ControlSize>));
        Some(SizesByVariant {
            variant: variant?,
            sizes: sizes?,
        })
    })
}

/// `default_size_by_variant = [{ variant = "<v>", size = "<size>" }]`.
fn default_size_by_variant(item: &Item, at: &str, cx: &mut Reader) -> Vec<DefaultSizeByVariant> {
    tables(item, at, cx, |fields, at, cx| {
        let [variant_item, size] = take(fields, ["variant", "size"], at, cx);
        let variant = read::required(variant_item, &join(at, "variant"), cx)
            .and_then(|item| VariantRef::read(item, Spelling::Bare, &join(at, "variant"), cx));
        let size = read::required(size, &join(at, "size"), cx)
            .and_then(|item| read::named::<ControlSize>(item, &join(at, "size"), cx));
        Some(DefaultSizeByVariant {
            variant: variant?,
            size: size?,
        })
    })
}

/// `data = { <part> = [ … ], values = [ … ] }` (S8, S10).
fn data(
    widget: WidgetKind,
    item: &Item,
    at: &str,
    cx: &mut Reader,
) -> Vec<(DataOwner, Vec<DataName>)> {
    let Some(owners) = table(item, at, cx) else {
        return Vec::new();
    };
    owners
        .iter()
        .filter_map(|(name, item)| {
            let at = join(at, name);
            let owner = if name == "values" {
                Some(DataOwner::Values)
            } else {
                part_named(widget, name, &at, cx).map(DataOwner::Part)
            };
            let operands = names(item, &at, cx, data_name);
            Some((owner?, operands))
        })
        .collect()
}

/// One operand a part's values read: an `element.` operand's name,
/// `line_spacing`, `body_font_height`, or `widget.<part>.<metric>` (S10).
fn data_name(name: &str, at: &str, cx: &mut Reader) -> Option<DataName> {
    match name {
        "line_spacing" => return Some(DataName::LineSpacing),
        "body_font_height" => return Some(DataName::BodyFontHeight),
        _ => {}
    }
    if let Some(rest) = name.strip_prefix("widget.") {
        let Some((part, metric)) = rest.split_once('.') else {
            return cx.mistyped(at, "`widget.<part>.<metric>`");
        };
        let Some(widget) = cx.widget else {
            return cx.mistyped(at, "a widget's own part");
        };
        let part = part_named(widget, part, at, cx);
        let metric = named::<FontMetric>(metric, at, cx);
        return Some(DataName::PartMetric {
            part: part?,
            metric: metric?,
        });
    }
    ElementDatum::from_name(name)
        .map(DataName::Element)
        .or_else(|| cx.unknown(at, name))
}

/// `keys = { <value> = [ <key>, … ] }`: each a value of the widget's.
fn value_keys(
    widget: WidgetKind,
    item: &Item,
    at: &str,
    cx: &mut Reader,
) -> Vec<(&'static str, Vec<Key>)> {
    let Some(values) = table(item, at, cx) else {
        return Vec::new();
    };
    let held = registry::widgets()
        .iter()
        .find(|info| info.kind == widget)
        .map_or(&[][..], |info| info.values);
    values
        .iter()
        .filter_map(|(name, item)| {
            let at = join(at, name);
            let value = held
                .iter()
                .find(|field| field.path == name)
                .map(|field| field.path)
                .or_else(|| cx.unknown(&at, name));
            let keys = keys(item, &at, cx);
            Some((value?, keys))
        })
        .collect()
}

/// `breakpoints = [{ measured, width, height, combine, variant }]` (S7.3):
/// `width` and `height` each `{ <comparison> = <n> }`, `combine` left out
/// only where fewer than two sizes are tested.
fn breakpoints(item: &Item, at: &str, cx: &mut Reader) -> Vec<Breakpoint> {
    tables(item, at, cx, |fields, at, cx| {
        let [measured, width, height, combine, variant_item] = take(
            fields,
            ["measured", "width", "height", "combine", "variant"],
            at,
            cx,
        );
        let measured = read::required(measured, &join(at, "measured"), cx)
            .and_then(|item| self::measured(item, &join(at, "measured"), cx));
        let width = width.map(|item| comparison(item, &join(at, "width"), cx));
        let height = height.map(|item| comparison(item, &join(at, "height"), cx));
        let combine = match combine {
            Some(item) => read::named::<Combine>(item, &join(at, "combine"), cx),
            None if width.is_none() || height.is_none() => Some(Combine::All),
            None => read::required(None, &join(at, "combine"), cx),
        };
        let variant = read::required(variant_item, &join(at, "variant"), cx)
            .and_then(|item| VariantRef::read(item, Spelling::Bare, &join(at, "variant"), cx));
        let mut breakpoint = Breakpoint::new(measured?, combine?, variant?);
        breakpoint.width = width.map_or(Some(None), |width| width.map(Some))?;
        breakpoint.height = height.map_or(Some(None), |height| height.map(Some))?;
        Some(breakpoint)
    })
}

/// `{ <comparison> = <length> }`.
fn comparison(item: &Item, at: &str, cx: &mut Reader) -> Option<(Comparison, Value<Px>)> {
    let (name, payload) = read::variant(item, at, cx)?;
    let Some(payload) = payload else {
        return cx.mistyped(at, "`{ <comparison> = <length> }`");
    };
    let at = join(at, name);
    let comparison = <Comparison as MapKey>::map_key(name, &at, cx);
    let length = Value::<Px>::read(payload, Spelling::Px, &at, cx);
    Some((comparison?, length?))
}

/// `"allocation"`, `"window"`, or `{ part = "<part>" }` (S11's enum rule),
/// the part one of the declared widget's own, by its bare name (S7.3).
fn measured(item: &Item, at: &str, cx: &mut Reader) -> Option<Measured> {
    match read::variant(item, at, cx)? {
        ("allocation", None) => Some(Measured::Allocation),
        ("window", None) => Some(Measured::Window),
        ("part", Some(part)) => {
            let at = join(at, "part");
            let name = text(part, &at, cx)?;
            let widget = cx.widget?;
            part_named(widget, name, &at, cx).map(|part| Measured::Part(PartRef::Typed(part)))
        }
        (name, None) => cx.unknown(at, name),
        (name, Some(_)) => {
            cx.problem(&join(at, name), ProblemKind::UnknownKey);
            None
        }
    }
}
