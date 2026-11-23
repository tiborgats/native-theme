//! Model parity of the 0.8.0 work (plan, section 1, rule 2) on the slice: the globals, 0.7.2's `button`,
//! `checkbox`, `list`, `scrollbar` and `tooltip`, and the widgets the model has added since
//! ([`SLICE_GROUPS`]), on `kde-breeze`.
//!
//! This half checks the table of homes, `tests/next_homes.toml`, against both models: it resolves 0.7.2's
//! `kde-breeze` (light and dark) with 0.7.2's public API, takes the keys from the JSON serialisation of the
//! resolved theme, and asserts that every key of the slice has exactly one row, that no row names a key that
//! does not exist (rows written by hand excepted), and that every home parses and names only what the spec
//! declares (the vocabulary below, written from spec §§2-5). Every problem found is listed before the test
//! fails. The value half, [`compare`], compares each row's old value with the values at its homes.
//!
//! Scaffolding of the 0.8.0 work: deleted at the 0.8.0 cut.

mod compare;

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use crate::resolve::ResolutionContext;
use crate::theme::{ColorMode, Theme};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// The table of homes.
const HOMES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/next_homes.toml"
));

/// The preset of the slice.
const PRESET: &str = "kde-breeze";

/// The first segments of the slice's keys in 0.7.2's resolved theme: its globals, the first slice's five
/// widgets and the widgets the model has added since.
const SLICE_GROUPS: &[&str] = &[
    "defaults",
    "text_scale",
    "button",
    "checkbox",
    "list",
    "scrollbar",
    "tooltip",
    "window",
    "input",
    "text_area",
    "link",
    "slider",
    "progress_bar",
    "splitter",
    "separator",
    "spinner",
    "menu",
    "tab",
    "sidebar",
    "toolbar",
    "status_bar",
    "popover",
    "switch",
    "dialog",
    "combo_box",
    "segmented_control",
    "card",
    "expander",
];

/// The fields of 0.7.2's `Resolved` wrapper beside the resolved theme, which do not serialise
/// (`native-theme/src/model/resolved.rs`, `Resolved`).
const RESOLVED_WRAPPER_KEYS: &[&str] = &[
    "Resolved.mode",
    "Resolved.icon_set",
    "Resolved.icon_theme",
    "Resolved.icon_theme_explicit",
];

/// The fields of 0.7.2's `Theme.layout`, which is not a field of the resolved theme
/// (`native-theme/src/model/widgets/mod.rs`, `LayoutTheme`).
const LAYOUT_KEYS: &[&str] = &[
    "layout.widget_gap",
    "layout.container_margin",
    "layout.window_margin",
    "layout.section_gap",
];

/// The fields of 0.7.2's `SystemTheme.accessibility`, which is not a field of the resolved theme
/// (`native-theme/src/lib.rs`, `AccessibilityPreferences`).
const ACCESSIBILITY_KEYS: &[&str] = &[
    "accessibility.text_scaling_factor",
    "accessibility.reduce_motion",
    "accessibility.high_contrast",
    "accessibility.reduce_transparency",
];

/// Every key a row written by hand must name, and the only ones it may.
fn hand_keys() -> impl Iterator<Item = &'static str> {
    RESOLVED_WRAPPER_KEYS
        .iter()
        .chain(LAYOUT_KEYS)
        .chain(ACCESSIBILITY_KEYS)
        .copied()
}

/// The fields of `ThemeContext` (spec §3), named after `context.` (the resolved theme's) or
/// `snapshot.context.` (the reader's, `SystemSnapshot::context`, spec §8).
const CONTEXT_FIELDS: &[&str] = &[
    "scheme",
    "contrast",
    "motion",
    "reduced_transparency",
    "bold_text",
    "button_shapes",
    "differentiate_without_color",
    "window_active",
    "text_scale",
    "direction",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Table {
    row: Vec<Row>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Row {
    key: String,
    homes: Vec<String>,
    #[serde(default)]
    homes_by_preset: BTreeMap<String, Vec<String>>,
    if_null: Option<String>,
    only_if_null: Option<String>,
    home_fallback: Option<String>,
    null_reference: Option<String>,
    context: Option<RowContext>,
    #[serde(default)]
    unstated_if_unset: bool,
    null_means: Option<NullMeans>,
    #[serde(default)]
    hand: bool,
    difference_presets: Option<Vec<String>>,
    difference_reason: Option<String>,
    difference_citation: Option<String>,
    difference_values: Option<BTreeMap<String, toml::Value>>,
    // Read by people; the tests only require it to be text.
    #[allow(dead_code)]
    note: Option<String>,
}

/// A row's `context`: the `ThemeContext` values its homes are read with in place of the defaults (spec §3).
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct RowContext {
    window_active: bool,
}

impl RowContext {
    /// `ctx` with this row's values in place of its own.
    fn applied_to(
        self,
        ctx: &crate::next::query::ThemeContext,
    ) -> crate::next::query::ThemeContext {
        ctx.clone().with_window_active(self.window_active)
    }
}

/// What a row's null `key` states at its homes.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
enum NullMeans {
    /// `"none"`: the home has no outline, `Stroke::None` with the `STROKE` bit set.
    #[serde(rename = "none")]
    NoOutline,
}

/// The dotted keys of every leaf of `value` under `prefix`. An object whose keys all begin with a capital
/// letter is an externally tagged enum value (`{"Pt": 10.0}`): one leaf.
fn leaves(prefix: &str, value: &serde_json::Value, out: &mut BTreeSet<String>) {
    match value {
        serde_json::Value::Object(map)
            if !map
                .keys()
                .all(|k| k.chars().next().is_some_and(char::is_uppercase)) =>
        {
            for (k, v) in map {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                leaves(&key, v, out);
            }
        }
        _ => {
            out.insert(prefix.to_owned());
        }
    }
}

/// The keys of the slice in 0.7.2's resolved `kde-breeze` of `mode`.
fn slice_keys(mode: ColorMode) -> Result<BTreeSet<String>, Box<dyn std::error::Error>> {
    let resolved = Theme::preset(PRESET)?.resolve_with(mode, &ResolutionContext::for_tests())?;
    let json = serde_json::to_value(&resolved.variant)?;
    let mut all = BTreeSet::new();
    leaves("", &json, &mut all);
    Ok(all
        .into_iter()
        .filter(|k| {
            k.split('.')
                .next()
                .is_some_and(|g| SLICE_GROUPS.contains(&g))
        })
        .collect())
}

// ---------------------------------------------------------------------------------------------------------
// The vocabulary of the 0.8.0 model that a home may name (spec §§2-5).
// ---------------------------------------------------------------------------------------------------------

/// The kind of a part: `PartStyle`, `TextStyle` or `IconStyle` (spec §5's legend: P, T, I).
#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    P,
    T,
    I,
}

/// The query a ruled widget takes (spec §5's table), which decides its variant axes.
#[derive(Clone, Copy, Debug, PartialEq)]
enum QueryKind {
    Style,
    Button,
    Input,
    Tab,
    List,
}

struct Widget {
    name: &'static str,
    /// `None` for a static widget, else the query of a ruled one.
    query: Option<QueryKind>,
    parts: &'static [(&'static str, Kind)],
    metrics: &'static [&'static str],
}

use Kind::{I, P, T};

/// Spec §5, "Complete 27-Widget Contract Table", with its notes: every widget has `origin`, and
/// `disabled_opacity` is a metric of `button`, `input`, `checkbox`, `radio`, `slider`, `switch`, `combo_box`
/// and `segmented_control`, and of `text_area`, an `InputWidget` that takes `input`'s `disabled_opacity`
/// (the notes' homes of `input.*`).
const WIDGETS: &[Widget] = &[
    Widget {
        name: "button",
        query: Some(QueryKind::Button),
        parts: &[("body", P), ("label", T), ("icon", I)],
        metrics: &["origin", "icon_text_gap", "disabled_opacity"],
    },
    Widget {
        name: "input",
        query: Some(QueryKind::Input),
        parts: &[
            ("field", P),
            ("text", T),
            ("placeholder", T),
            ("helper_text", T),
        ],
        metrics: &["origin", "disabled_opacity"],
    },
    Widget {
        name: "text_area",
        query: Some(QueryKind::Input),
        parts: &[
            ("field", P),
            ("text", T),
            ("placeholder", T),
            ("helper_text", T),
        ],
        metrics: &["origin", "disabled_opacity"],
    },
    Widget {
        name: "checkbox",
        query: Some(QueryKind::Style),
        parts: &[("indicator", P), ("checkmark", I), ("label", T)],
        metrics: &["origin", "label_gap", "disabled_opacity"],
    },
    Widget {
        name: "radio",
        query: Some(QueryKind::Style),
        parts: &[("indicator", P), ("dot", P), ("label", T)],
        metrics: &["origin", "label_gap", "disabled_opacity"],
    },
    Widget {
        name: "switch",
        query: Some(QueryKind::Style),
        parts: &[("track", P), ("thumb", P), ("label", T)],
        metrics: &[
            "origin",
            "label_side",
            "label_gap",
            "label_gap_stretches",
            "show_state_glyphs",
            "disabled_opacity",
        ],
    },
    Widget {
        name: "slider",
        query: Some(QueryKind::Style),
        parts: &[("track", P), ("active_track", P), ("thumb", P), ("tick", P)],
        metrics: &["origin", "tick_length", "disabled_opacity"],
    },
    Widget {
        name: "progress_bar",
        query: Some(QueryKind::Style),
        parts: &[("track", P), ("fill", P)],
        metrics: &["origin", "indeterminate_period_ms"],
    },
    Widget {
        name: "scrollbar",
        query: Some(QueryKind::Style),
        parts: &[("track", P), ("thumb", P)],
        metrics: &["origin", "min_thumb_length", "overlay_mode"],
    },
    Widget {
        name: "tab",
        query: Some(QueryKind::Tab),
        parts: &[
            ("bar", P),
            ("tab", P),
            ("label", T),
            ("indicator", P),
            ("close_button", P),
            ("close_icon", I),
        ],
        metrics: &[
            "origin",
            "item_gap",
            "indicator_side",
            "active_outward_corner_radius",
        ],
    },
    Widget {
        name: "menu",
        query: Some(QueryKind::Style),
        parts: &[
            ("panel", P),
            ("item", P),
            ("item_label", T),
            ("shortcut", T),
            ("icon", I),
            ("submenu_arrow", I),
            ("separator", P),
        ],
        metrics: &["origin", "icon_text_gap"],
    },
    Widget {
        name: "menu_bar",
        query: Some(QueryKind::Style),
        parts: &[("bar", P), ("item", P), ("item_label", T)],
        metrics: &["origin", "item_gap"],
    },
    Widget {
        name: "list",
        query: Some(QueryKind::List),
        parts: &[
            ("container", P),
            ("row", P),
            ("highlight", P),
            ("item_label", T),
            ("header", P),
            ("header_label", T),
            ("header_separator", P),
        ],
        metrics: &["origin", "grid_color"],
    },
    Widget {
        name: "sidebar",
        query: Some(QueryKind::Style),
        parts: &[
            ("container", P),
            ("item", P),
            ("item_label", T),
            ("item_icon", I),
            ("section_header", T),
        ],
        metrics: &["origin", "item_gap"],
    },
    Widget {
        name: "toolbar",
        query: Some(QueryKind::Style),
        parts: &[
            ("bar", P),
            ("button", P),
            ("button_label", T),
            ("button_icon", I),
            ("separator", P),
        ],
        metrics: &["origin", "item_gap"],
    },
    Widget {
        name: "status_bar",
        query: None,
        parts: &[("bar", P), ("label", T), ("separator", P)],
        metrics: &["origin"],
    },
    Widget {
        name: "combo_box",
        query: Some(QueryKind::Style),
        parts: &[("field", P), ("label", T), ("chevron", I)],
        metrics: &["origin", "arrow_area_width", "disabled_opacity"],
    },
    Widget {
        name: "segmented_control",
        query: Some(QueryKind::Style),
        parts: &[("track", P), ("segment", P), ("label", T), ("separator", P)],
        metrics: &["origin", "separator_width", "disabled_opacity"],
    },
    Widget {
        name: "expander",
        query: Some(QueryKind::Style),
        parts: &[
            ("container", P),
            ("header", P),
            ("header_label", T),
            ("arrow", I),
        ],
        metrics: &[
            "origin",
            "arrow_side",
            "arrow_gap",
            "content_indent",
            "frame_enabled",
        ],
    },
    Widget {
        name: "card",
        query: Some(QueryKind::Style),
        parts: &[("container", P)],
        metrics: &["origin"],
    },
    Widget {
        name: "popover",
        query: None,
        parts: &[("panel", P), ("arrow", P), ("text", T)],
        metrics: &["origin"],
    },
    Widget {
        name: "dialog",
        query: None,
        parts: &[("panel", P), ("title", T), ("body", T), ("icon", I)],
        metrics: &["origin", "button_gap", "button_order", "entrance"],
    },
    Widget {
        name: "tooltip",
        query: None,
        parts: &[("panel", P), ("text", T)],
        metrics: &["origin"],
    },
    Widget {
        name: "spinner",
        query: None,
        parts: &[],
        metrics: &[
            "origin",
            "color",
            "diameter",
            "min_diameter",
            "stroke_width",
            "period_ms",
        ],
    },
    Widget {
        name: "splitter",
        query: Some(QueryKind::Style),
        parts: &[("handle", P)],
        metrics: &["origin", "divider_width"],
    },
    Widget {
        name: "separator",
        query: None,
        parts: &[],
        metrics: &["origin", "color", "thickness", "inset", "margin"],
    },
    Widget {
        name: "link",
        query: Some(QueryKind::Style),
        parts: &[("container", P), ("label", T)],
        metrics: &["origin"],
    },
];

/// The properties of a `PartStyle` a home may name (spec §2; `stroke` by its colour and width).
const PART_PROPERTIES: &[&str] = &[
    "fill",
    "stroke.color",
    "stroke.width",
    "corners",
    "shadows",
    "opacity",
    "transform",
    "padding.top",
    "padding.right",
    "padding.bottom",
    "padding.left",
    "margin.top",
    "margin.right",
    "margin.bottom",
    "margin.left",
    "min_size[0]",
    "min_size[1]",
    "max_size[0]",
    "max_size[1]",
    "focus",
    "ripple",
    "transition",
    "cursor",
];

/// The properties of a `TextStyle` (spec §2).
const TEXT_PROPERTIES: &[&str] = &[
    "family",
    "color",
    "size_px",
    "defined_size",
    "weight",
    "style",
    "width",
    "line_height",
    "letter_spacing",
    "optical_size",
    "decoration",
    "blend_mode",
];

/// The properties of an `IconStyle` (spec §2).
const ICON_PROPERTIES: &[&str] = &[
    "size",
    "color",
    "opacity",
    "weight",
    "rendering_mode",
    "stroke_width",
    "blend_mode",
];

/// `WidgetState`'s bits (spec §3).
const STATES: &[&str] = &[
    "HOVER",
    "PRESSED",
    "FOCUSED",
    "FOCUS_WITHIN",
    "FOCUS_VISIBLE",
    "UNFOCUSED_VIEW",
    "DISABLED",
    "CHECKED",
    "SELECTED",
    "INDETERMINATE",
    "OPEN",
    "DEFAULT",
    "INVALID",
    "READ_ONLY",
    "VISITED",
    "DRAGGED",
    "DROP_TARGET",
    "SCROLLED",
    "HOVER_WITHIN",
];

const CONTROL_SIZES: &[&str] = &["Mini", "Small", "Regular", "Large", "ExtraLarge"];
const SURFACES: &[&str] = &[
    "Window",
    "View",
    "HeaderBar",
    "Sidebar",
    "Card",
    "Dialog",
    "Popover",
    "Tooltip",
    "Osd",
];
const POSITIONS: &[&str] = &["Single", "First", "Middle", "Last"];
const ORIENTATIONS: &[&str] = &["Horizontal", "Vertical"];
const BUTTON_VARIANTS: &[&str] = &[
    "Standard",
    "Primary",
    "Secondary",
    "Destructive",
    "Outlined",
    "Flat",
    "Glass",
    "Link",
];
const BUTTON_SHAPES: &[&str] = &["Default", "Pill", "Circle", "Square"];
const INPUT_VARIANTS: &[&str] = &["Standard", "Outlined", "Filled", "Search", "Borderless"];
const TAB_VARIANTS: &[&str] = &["Standard", "Document", "Pill", "Segmented"];
const LIST_VARIANTS: &[&str] = &["Plain", "InsetGrouped", "Sidebar", "Table"];
const BOOLS: &[&str] = &["true", "false"];

/// The values an axis of `query` takes, or `None` where the query has no such axis.
fn axis_values(query: QueryKind, axis: &str) -> Option<&'static [&'static str]> {
    match (query, axis) {
        (_, "size") => Some(CONTROL_SIZES),
        (_, "surface") => Some(SURFACES),
        (_, "position") => Some(POSITIONS),
        (_, "orientation") => Some(ORIENTATIONS),
        (QueryKind::Button, "variant") => Some(BUTTON_VARIANTS),
        (QueryKind::Button, "shape") => Some(BUTTON_SHAPES),
        (QueryKind::Input, "variant") => Some(INPUT_VARIANTS),
        (QueryKind::Tab, "variant") => Some(TAB_VARIANTS),
        (QueryKind::List, "variant") => Some(LIST_VARIANTS),
        (QueryKind::List, "alternate") => Some(BOOLS),
        _ => None,
    }
}

/// Every global a home may name: the fields of `ResolvedTheme` outside the widgets (spec §5), to their
/// leaves as spec §§2-4 declare them.
fn globals() -> BTreeSet<String> {
    let mut g = BTreeSet::new();
    let mut add = |prefix: &str, fields: &[&str]| {
        for f in fields {
            g.insert(format!("{prefix}{f}"));
        }
    };
    add("", &["name", "preset"]);
    add("context.", CONTEXT_FIELDS);
    let surface_fields = [
        "background",
        "foreground",
        "foreground_muted",
        "border",
        "divider",
    ];
    for s in [
        "window", "view", "header", "sidebar", "card", "dialog", "popover", "tooltip", "osd",
    ] {
        add(&format!("colors.{s}."), &surface_fields);
    }
    for s in ["selection", "text_selection"] {
        add(
            &format!("colors.{s}."),
            &[
                "background",
                "foreground",
                "unfocused_background",
                "unfocused_foreground",
            ],
        );
    }
    add(
        "colors.",
        &[
            "find_highlight",
            "scrim",
            "focus_ring",
            "caret",
            "link",
            "link_visited",
            "disabled_text",
            "shadow",
            "disabled_opacity",
        ],
    );
    add("colors.accent.", &["fill", "foreground", "standalone"]);
    for s in ["error", "warning", "success", "info"] {
        add(
            &format!("colors.{s}."),
            &["fill", "foreground", "standalone", "origin"],
        );
    }
    for slot in [
        "display",
        "title1",
        "title2",
        "title3",
        "title4",
        "headline",
        "body",
        "callout",
        "subheadline",
        "caption",
        "monospace",
    ] {
        add(&format!("typography.{slot}."), TEXT_PROPERTIES);
    }
    add(
        "typography.",
        &[
            "line_height_multiplier",
            "fallbacks",
            "emoji_family",
            "font_dpi",
            "font_whole_pixels",
        ],
    );
    add(
        "typography.rendering.",
        &["antialiasing", "hinting", "subpixel_order", "smoothing"],
    );
    add("icons.", &["set", "theme", "theme_explicit"]);
    add(
        "icons.sizes.",
        &["toolbar", "small", "large", "dialog", "panel"],
    );
    add(
        "layout.",
        &[
            "widget_gap",
            "container_margin",
            "window_margin",
            "section_gap",
            "corner_radius",
            "corner_radius_lg",
            "border_width",
            "border_opacity",
            "shadow_enabled",
            "cursor_size_px",
            "cursor_theme",
        ],
    );
    add(
        "timings.",
        &[
            "tooltip_show_ms",
            "tooltip_hide_ms",
            "tooltip_reshow_ms",
            "double_click_ms",
            "drag_threshold_px",
            "scrollbar_auto_hide_ms",
            "menu_hover_open_ms",
            "key_repeat_delay_ms",
            "key_repeat_interval_ms",
        ],
    );
    add(
        "motion_tokens.",
        &[
            "fast",
            "standard",
            "emphasized",
            "spring_spatial",
            "spring_effects",
            "theme_change",
        ],
    );
    add(
        "focus_ring.",
        &[
            "kind",
            "stroke.color",
            "stroke.width",
            "inner_stroke.color",
            "inner_stroke.width",
            "offset",
            "corners",
            "trigger",
        ],
    );
    add(
        "caret.",
        &[
            "width",
            "blink_on_ms",
            "blink_off_ms",
            "blink_timeout_ms",
            "shape",
        ],
    );
    g
}

// ---------------------------------------------------------------------------------------------------------
// The home notation.
// ---------------------------------------------------------------------------------------------------------

/// A parsed home.
#[derive(Debug, PartialEq)]
enum Home {
    Global,
    Snapshot,
    Metric,
    Part,
    Window,
}

/// The getters of `WindowChrome` a home may name (spec §4), with their kinds.
const WINDOW_GETTERS: &[(&str, Kind)] = &[("frame", P), ("title_bar", P), ("title", T)];

/// `home` parsed against the vocabulary, or what is wrong with it.
fn parse_home(home: &str, globals: &BTreeSet<String>) -> Result<Home, String> {
    if globals.contains(home) {
        return Ok(Home::Global);
    }
    if let Some(rest) = home.strip_prefix("window.") {
        let (getter, property) = rest.split_once("(normal).").ok_or_else(|| {
            format!("`{home}`: a window home is `window.<getter>(normal).<property>`")
        })?;
        let kind = WINDOW_GETTERS
            .iter()
            .find(|(name, _)| *name == getter)
            .map(|(_, kind)| *kind)
            .ok_or_else(|| format!("`{home}`: `WindowChrome` has no getter `{getter}`"))?;
        check_property(home, kind, property)?;
        return Ok(Home::Window);
    }
    if let Some(field) = home.strip_prefix("snapshot.context.") {
        return if CONTEXT_FIELDS.contains(&field) {
            Ok(Home::Snapshot)
        } else {
            Err(format!("`{home}`: `{field}` is no field of `ThemeContext`"))
        };
    }
    let (widget_name, rest) = home
        .split_once('.')
        .ok_or_else(|| format!("`{home}`: neither a global nor `<widget>.…`"))?;
    let widget = WIDGETS
        .iter()
        .find(|w| w.name == widget_name)
        .ok_or_else(|| format!("`{home}`: no global and no widget `{widget_name}`"))?;
    if widget.metrics.contains(&rest) {
        return Ok(Home::Metric);
    }
    // `<part>(<query>).<property>` or `<part>.<property>`.
    let (part_name, query, property) = match rest.split_once('(') {
        Some((part, after)) => {
            let (query, property) = after
                .split_once(").")
                .ok_or_else(|| format!("`{home}`: a query must close with `).<property>`"))?;
            (part, Some(query), property)
        }
        None => {
            let (part, property) = rest
                .split_once('.')
                .ok_or_else(|| format!("`{home}`: `{rest}` is no metric of `{widget_name}`"))?;
            (part, None, property)
        }
    };
    let kind = widget
        .parts
        .iter()
        .find(|(name, _)| *name == part_name)
        .map(|(_, kind)| *kind)
        .ok_or_else(|| format!("`{home}`: `{widget_name}` has no part `{part_name}`"))?;
    match (widget.query, query) {
        (Some(query_kind), Some(query)) => check_query(home, query_kind, query)?,
        (Some(_), None) => {
            return Err(format!(
                "`{home}`: `{widget_name}` is ruled, its part needs a query"
            ));
        }
        (None, Some(_)) => {
            return Err(format!(
                "`{home}`: `{widget_name}` is static, its part takes no query"
            ));
        }
        (None, None) => {}
    }
    check_property(home, kind, property)?;
    Ok(Home::Part)
}

/// Checks that `property` is a property of a part of `kind`.
fn check_property(home: &str, kind: Kind, property: &str) -> Result<(), String> {
    let properties = match kind {
        Kind::P => PART_PROPERTIES,
        Kind::T => TEXT_PROPERTIES,
        Kind::I => ICON_PROPERTIES,
    };
    if properties.contains(&property) {
        Ok(())
    } else {
        Err(format!(
            "`{home}`: `{property}` is no property of a {kind:?} part"
        ))
    }
}

/// Checks `<states>[; <axis>=<value>, …]` for a widget of `query_kind`.
fn check_query(home: &str, query_kind: QueryKind, query: &str) -> Result<(), String> {
    let (states, axes) = match query.split_once("; ") {
        Some((states, axes)) => (states, Some(axes)),
        None => (query, None),
    };
    if states != "NONE" {
        let mut seen = BTreeSet::new();
        for state in states.split('|') {
            if !STATES.contains(&state) {
                return Err(format!("`{home}`: no state `{state}`"));
            }
            if !seen.insert(state) {
                return Err(format!("`{home}`: the state `{state}` is named twice"));
            }
        }
    }
    let mut seen = BTreeSet::new();
    for axis in axes.into_iter().flat_map(|a| a.split(", ")) {
        let (name, value) = axis
            .split_once('=')
            .ok_or_else(|| format!("`{home}`: an axis is `<name>=<value>`, not `{axis}`"))?;
        if !seen.insert(name) {
            return Err(format!("`{home}`: the axis `{name}` is named twice"));
        }
        let values = axis_values(query_kind, name)
            .ok_or_else(|| format!("`{home}`: the widget's query has no axis `{name}`"))?;
        if !values.contains(&value) {
            return Err(format!("`{home}`: `{value}` is no value of `{name}`"));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------
// The checks.
// ---------------------------------------------------------------------------------------------------------

/// Every problem of the table against the keys of the slice (`keys`) and the vocabulary.
fn problems(table: &Table, keys: &BTreeSet<String>) -> Vec<String> {
    let globals = globals();
    let mut found = Vec::new();

    // Every key of the slice: exactly one row.
    let mut rows_per_key: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &table.row {
        let count = rows_per_key.entry(row.key.as_str()).or_default();
        *count = count.saturating_add(1);
    }
    for key in keys {
        match rows_per_key.get(key.as_str()) {
            None => found.push(format!("`{key}`: no row")),
            Some(1) => {}
            Some(n) => found.push(format!("`{key}`: {n} rows")),
        }
    }
    for (key, n) in &rows_per_key {
        if *n > 1 && !keys.contains(*key) {
            found.push(format!("`{key}`: {n} rows"));
        }
    }
    for key in hand_keys() {
        if !table.row.iter().any(|r| r.key == key && r.hand) {
            found.push(format!("`{key}`: no row written by hand"));
        }
    }

    for row in &table.row {
        let key = &row.key;
        // A row names a key that exists, or is written by hand for a key that does not serialise.
        if row.hand {
            if keys.contains(key) {
                found.push(format!(
                    "`{key}`: marked as written by hand, but it serialises"
                ));
            }
            if !hand_keys().any(|k| k == key) {
                found.push(format!(
                    "`{key}`: written by hand, but no field of 0.7.2's `Resolved` wrapper, \
                     `LayoutTheme` or `AccessibilityPreferences`"
                ));
            }
        } else if !keys.contains(key) {
            found.push(format!("`{key}`: no such key in 0.7.2's serialisation"));
        }

        // Every home parses.
        if row.homes.is_empty() {
            found.push(format!("`{key}`: no home"));
        }
        for (preset, homes) in &row.homes_by_preset {
            if homes.is_empty() {
                found.push(format!("`{key}`: no home on `{preset}`"));
            }
        }
        let by_preset = row.homes_by_preset.values().flatten();
        for home in row.homes.iter().chain(by_preset) {
            if let Err(e) = parse_home(home, &globals) {
                found.push(format!("`{key}`: {e}"));
            }
        }
        for preset in row
            .homes_by_preset
            .keys()
            .chain(row.difference_presets.iter().flatten())
        {
            if Theme::preset(preset).is_err() {
                found.push(format!("`{key}`: no 0.7.2 preset `{preset}`"));
            }
        }

        // The fallbacks name keys and homes that exist.
        for (field, other) in [
            ("if_null", &row.if_null),
            ("only_if_null", &row.only_if_null),
        ] {
            if let Some(other) = other
                && !keys.contains(other)
            {
                found.push(format!("`{key}`: {field} `{other}` is no key of the slice"));
            }
        }
        if let Some(reference) = &row.null_reference {
            let parses = reference.split('|').all(|state| STATES.contains(&state));
            if reference == "NONE" || !parses {
                found.push(format!(
                    "`{key}`: null_reference `{reference}` is no set of states"
                ));
            }
        }
        if row.null_means.is_some() {
            let by_preset = row.homes_by_preset.values().flatten();
            for home in row.homes.iter().chain(by_preset) {
                let stroke = home.ends_with(".stroke.color") || home.ends_with(".stroke.width");
                if !stroke || parse_home(home, &globals) != Ok(Home::Part) {
                    found.push(format!(
                        "`{key}`: null_means needs a part's stroke at every home, not `{home}`"
                    ));
                }
            }
        }
        if let Some(fallback) = &row.home_fallback {
            if row.homes.len() != 1 || !row.homes_by_preset.is_empty() {
                found.push(format!("`{key}`: home_fallback needs exactly one home"));
            }
            match parse_home(fallback, &globals) {
                Ok(Home::Global) => {}
                Ok(_) => found.push(format!(
                    "`{key}`: home_fallback `{fallback}` is not a global"
                )),
                Err(e) => found.push(format!("`{key}`: home_fallback {e}")),
            }
        }

        // A listed difference states its presets, its reason, its citation and, for each of its presets
        // and no other, the value 0.8.0 has at the homes: "unstated" or a single value, or a table of
        // exactly `light` and `dark`, each a single value, where the two schemes differ; where the homes
        // differ, a table of such values by home, naming every home of the row on the preset and no
        // other. "none" (no outline) is a value at a part's stroke only.
        match (
            &row.difference_presets,
            &row.difference_reason,
            &row.difference_citation,
            &row.difference_values,
        ) {
            (None, None, None, None) => {}
            (Some(presets), Some(reason), Some(citation), Some(values))
                if !presets.is_empty() && !reason.is_empty() && !citation.is_empty() =>
            {
                let named: BTreeSet<&str> = presets.iter().map(String::as_str).collect();
                let valued: BTreeSet<&str> = values.keys().map(String::as_str).collect();
                if named != valued {
                    found.push(format!(
                        "`{key}`: difference_values must name exactly the difference_presets"
                    ));
                }
                let single =
                    |v: &toml::Value| !matches!(v, toml::Value::Table(_) | toml::Value::Array(_));
                let per_scheme_or_single = |v: &toml::Value| match v {
                    toml::Value::Table(schemes) => {
                        schemes.len() == 2
                            && ["light", "dark"]
                                .iter()
                                .all(|s| schemes.get(*s).is_some_and(single))
                    }
                    other => single(other),
                };
                let no_outline = |v: &toml::Value| match v {
                    toml::Value::Table(schemes) => {
                        schemes.values().any(|v| v.as_str() == Some("none"))
                    }
                    other => other.as_str() == Some("none"),
                };
                let not_a_stroke = |preset: &str, home: &str| {
                    format!(
                        "`{key}`: difference_values for `{preset}` names \"none\" at `{home}`, \
                         which is no part's stroke"
                    )
                };
                for (preset, value) in values {
                    let homes = row.homes_by_preset.get(preset).unwrap_or(&row.homes);
                    match value {
                        toml::Value::Table(table) if !compare::is_per_scheme(table) => {
                            let named: BTreeSet<&str> = table.keys().map(String::as_str).collect();
                            let expected: BTreeSet<&str> =
                                homes.iter().map(String::as_str).collect();
                            if named != expected {
                                found.push(format!(
                                    "`{key}`: difference_values for `{preset}` by home must name \
                                     exactly the row's homes on it"
                                ));
                            }
                            for (home, value) in table {
                                if !per_scheme_or_single(value) {
                                    found.push(format!(
                                        "`{key}`: difference_values for `{preset}` at `{home}` is \
                                         not a single value or a table of `light` and `dark`"
                                    ));
                                }
                                if no_outline(value) && !compare::is_stroke_home(home) {
                                    found.push(not_a_stroke(preset, home));
                                }
                            }
                        }
                        other => {
                            if !per_scheme_or_single(other) {
                                found.push(format!(
                                    "`{key}`: difference_values for `{preset}` is not a single \
                                     value or a table of `light` and `dark`"
                                ));
                            }
                            if no_outline(other) {
                                for home in homes.iter().filter(|h| !compare::is_stroke_home(h)) {
                                    found.push(not_a_stroke(preset, home));
                                }
                            }
                        }
                    }
                }
            }
            _ => found.push(format!(
                "`{key}`: a listed difference needs non-empty difference_presets, \
                 difference_reason, difference_citation and difference_values"
            )),
        }
    }
    found
}

#[test]
fn every_key_of_the_slice_has_one_row_and_every_home_parses() -> TestResult {
    let table: Table = toml::from_str(HOMES)?;
    let light = slice_keys(ColorMode::Light)?;
    let dark = slice_keys(ColorMode::Dark)?;
    assert_eq!(light, dark, "the two schemes serialise different keys");
    let found = problems(&table, &light);
    assert!(
        found.is_empty(),
        "{} problems in tests/next_homes.toml:\n{}",
        found.len(),
        found.join("\n")
    );
    Ok(())
}

#[test]
fn the_checks_catch_what_they_check() -> TestResult {
    let globals = globals();
    for good in [
        "colors.view.background",
        "typography.headline.size_px",
        "button.icon_text_gap",
        "tooltip.panel.fill",
        "button.body(NONE; variant=Primary).fill",
        "checkbox.indicator(HOVER|CHECKED).stroke.color",
        "scrollbar.track(NONE; orientation=Vertical).min_size[0]",
        "list.row(NONE; alternate=true).fill",
        "snapshot.context.motion",
        "window.title_bar(normal).fill",
        "window.title(normal).color",
    ] {
        assert!(parse_home(good, &globals).is_ok(), "{good}");
    }
    assert_eq!(
        parse_home("window.frame(normal).corners", &globals),
        Ok(Home::Window)
    );
    for bad in [
        "colors.nowhere",
        "nowidget.body(NONE).fill",
        "button.face(NONE).fill",
        "button.body.fill",
        "tooltip.panel(NONE).fill",
        "button.body(HOVERED).fill",
        "button.label(NONE).fill",
        "button.body(NONE; alternate=true).fill",
        "button.body(NONE; variant=Huge).fill",
        "button.body(NONE).min_size[2]",
        "button.body(NONE.fill",
        "snapshot.context.nowhere",
        "snapshot.colors.view.background",
        "button.body(HOVER|HOVER).fill",
        "button.body(NONE; variant=Primary, variant=Flat).fill",
        "window.title_bar.fill",
        "window.title_bar(NONE).fill",
        "window.caption(normal).fill",
        "window.title(normal).fill",
        "window.frame(normal).family",
    ] {
        assert!(parse_home(bad, &globals).is_err(), "{bad}");
    }

    let keys: BTreeSet<String> = ["button.min_width", "button.min_height"]
        .into_iter()
        .map(String::from)
        .collect();
    let table: Table = toml::from_str(
        r##"
        [[row]]
        key = "button.min_width"
        homes = ["button.body(NONE).min_size[0]"]
        null_reference = "HOVERED"

        [[row]]
        key = "button.gone"
        homes = ["button.body(NONE).min_size[1]"]
        homes_by_preset = { "kde-breeze" = [] }

        [[row]]
        key = "layout.gone"
        homes = ["layout.widget_gap"]
        hand = true

        [[row]]
        key = "button.min_width"
        homes = ["button.body(NONE).min_size[0]"]
        difference_presets = ["kde-breeze"]
        difference_reason = "a reason"
        difference_citation = "a citation"
        difference_values = { "adwaita" = "unstated" }

        [[row]]
        key = "button.gone"
        homes = ["button.body(NONE).min_size[1]", "button.body(NONE).stroke.width"]
        null_means = "none"

        [[row]]
        key = "button.gone"
        homes = ["button.body(CHECKED).fill"]
        difference_presets = ["kde-breeze"]
        difference_reason = "a reason"
        difference_citation = "a citation"
        difference_values = { "kde-breeze" = { light = "unstated" } }

        [[row]]
        key = "button.gone"
        homes = ["button.body(HOVER).stroke.color", "button.body(PRESSED).stroke.color"]
        difference_presets = ["adwaita", "windows-11", "material-3"]
        difference_reason = "a reason"
        difference_citation = "a citation"
        difference_values = { "adwaita" = { "button.body(HOVER).stroke.color" = "#ffffff" }, "windows-11" = { "button.body(HOVER).stroke.color" = "none", "button.body(PRESSED).stroke.color" = { light = "#000000" } }, "material-3" = { "button.body(HOVER).stroke.color" = "none", "button.body(PRESSED).stroke.color" = { light = "#000000", dark = "none" } } }

        [[row]]
        key = "button.gone"
        homes = ["button.body(NONE).fill"]
        difference_presets = ["kde-breeze", "ios-13"]
        difference_reason = "a reason"
        difference_citation = "a citation"
        difference_values = { "kde-breeze" = "none", "ios-13" = { "button.body(NONE).fill" = { light = "#000000", dark = "none" } } }
        "##,
    )?;
    for bad in [
        "null_means = \"unset\"",
        "context = { window_active = false, scheme = \"Dark\" }",
        "unstated_if_unset = 1",
    ] {
        let row =
            format!("[[row]]\nkey = \"button.min_width\"\nhomes = [\"layout.widget_gap\"]\n{bad}");
        assert!(toml::from_str::<Table>(&row).is_err(), "{bad}");
    }
    let found = problems(&table, &keys);
    for expected in [
        "`button.min_height`: no row",
        "`button.gone`: no such key",
        "`Resolved.mode`: no row written by hand",
        "`layout.section_gap`: no row written by hand",
        "`accessibility.reduce_motion`: no row written by hand",
        "`layout.gone`: written by hand, but no field",
        "`button.min_width`: difference_values must name exactly the difference_presets",
        "`button.min_width`: null_reference `HOVERED` is no set of states",
        "`button.gone`: no home on `kde-breeze`",
        "`button.gone`: null_means needs a part's stroke at every home, \
         not `button.body(NONE).min_size[1]`",
        "`button.gone`: difference_values for `kde-breeze` is not a single value \
         or a table of `light` and `dark`",
        "`button.gone`: difference_values for `adwaita` by home must name exactly the row's \
         homes on it",
        "`button.gone`: difference_values for `windows-11` at `button.body(PRESSED).stroke.color` \
         is not a single value or a table of `light` and `dark`",
        "`button.gone`: difference_values for `kde-breeze` names \"none\" at \
         `button.body(NONE).fill`, which is no part's stroke",
        "`button.gone`: difference_values for `ios-13` names \"none\" at \
         `button.body(NONE).fill`, which is no part's stroke",
    ] {
        assert!(
            found.iter().any(|p| p.contains(expected)),
            "{expected}: {found:?}"
        );
    }
    // A table by home naming exactly the row's homes, "none" at a stroke, is well formed.
    assert!(
        !found.iter().any(|p| p.contains("`material-3`")),
        "{found:?}"
    );
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------
// The value half, `compare` (`parity/compare.rs`), on the kde-breeze builder. Its test on a reader's fixture
// comes with the builder's system path (the next task).
// ---------------------------------------------------------------------------------------------------------

/// The groups of the table (a row's group is its key's first segment) whose rows the kde-breeze builder
/// ports so far: each of their rows is compared and equal, or skipped for a reason [`expected_skip`] names.
const PORTED_GROUPS: &[&str] = &[
    "defaults",
    "text_scale",
    "Resolved",
    "layout",
    "accessibility",
    "button",
    "checkbox",
    "list",
    "scrollbar",
    "tooltip",
    "window",
    "input",
    "text_area",
    "link",
    "slider",
    "progress_bar",
    "splitter",
    "separator",
    "spinner",
    "tab",
    "menu",
    "toolbar",
    "status_bar",
    "sidebar",
    "popover",
    "switch",
    "dialog",
    "combo_box",
    "segmented_control",
    "card",
    "expander",
];

/// The groups of the table whose widgets the model holds but whose values the kde-breeze builder does not
/// state yet (plan, M2's KDE unit): their rows are compared, counted and reported, not asserted.
const NOT_PORTED_GROUPS: &[&str] = &[];

/// The totals of the ported groups' rows on each scheme, `[rows, compared, equal, mismatching, skipped]`:
/// every row equal but those [`expected_skip`] names, skipped.
const PORTED_TOTALS: [usize; 5] = [563, 558, 558, 0, 5];

/// The reason a row of the table is skipped on 0.7.2's `kde-breeze` without a reader's snapshot: the
/// reader's context rows (`accessibility.*`), and `checkbox.background_color`, compared only where
/// `checkbox.unchecked_background` is unset (the table's listed differences).
fn expected_skip(key: &str) -> Option<compare::Skip> {
    if key.starts_with("accessibility.") {
        Some(compare::Skip::NoSnapshot)
    } else if key == "checkbox.background_color" {
        Some(compare::Skip::OtherKeyStated)
    } else {
        None
    }
}

/// 0.7.2's resolved `kde-breeze` of `mode` as the comparator reads it: the JSON serialisation of the
/// resolved theme, with the rows written by hand added from the `Resolved` wrapper and from `Theme.layout`,
/// each in the form `serde_json` gives its field's type (`Resolved.mode` as the variant's name).
fn old_kde_breeze(mode: ColorMode) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    old_resolved(&Theme::preset(PRESET)?, mode)
}

/// The JSON serialisation of the variant of `theme` that [`old_resolved`] resolves for `mode`, unresolved
/// (the table's `unstated_if_unset`).
fn old_unresolved(
    theme: &Theme,
    mode: ColorMode,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(serde_json::to_value(theme.pick_variant(mode)?)?)
}

/// 0.7.2's `theme` resolved for `mode` as the comparator reads it (see [`old_kde_breeze`]).
fn old_resolved(
    theme: &Theme,
    mode: ColorMode,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    old_resolved_in(theme, mode, &ResolutionContext::for_tests())
}

/// [`old_resolved`] in the resolution context `context`.
fn old_resolved_in(
    theme: &Theme,
    mode: ColorMode,
    context: &ResolutionContext,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let resolved = theme.resolve_with(mode, context)?;
    let mut old = serde_json::to_value(&resolved.variant)?;
    let map = old
        .as_object_mut()
        .ok_or("the resolved theme does not serialise as an object")?;
    map.insert(
        "Resolved".to_owned(),
        serde_json::json!({
            "mode": format!("{:?}", resolved.mode),
            "icon_set": serde_json::to_value(resolved.icon_set)?,
            "icon_theme": resolved.icon_theme.as_deref(),
            "icon_theme_explicit": resolved.icon_theme_explicit,
        }),
    );
    let layout = &theme.layout;
    map.insert(
        "layout".to_owned(),
        serde_json::json!({
            "widget_gap": layout.widget_gap,
            "container_margin": layout.container_margin,
            "window_margin": layout.window_margin,
            "section_gap": layout.section_gap,
        }),
    );
    Ok(old)
}

/// The totals of the rows in the ported groups (`ported`) or in the others: rows, compared, equal,
/// mismatching, skipped.
fn group_totals(report: &compare::Report, ported: bool) -> [usize; 5] {
    totals_in(report, PORTED_GROUPS, ported)
}

/// Whether the group of the row of `key`, its first segment, is one of `groups`.
fn in_groups(key: &str, groups: &[&str]) -> bool {
    groups.contains(&key.split('.').next().unwrap_or(""))
}

/// The totals of the rows in `groups` (`inside`) or in the others: rows, compared, equal, mismatching,
/// skipped.
fn totals_in(report: &compare::Report, groups: &[&str], inside: bool) -> [usize; 5] {
    let rows: Vec<&compare::Outcome> = report
        .rows
        .iter()
        .filter(|row| in_groups(&row.key, groups) == inside)
        .map(|row| &row.outcome)
        .collect();
    let count = |f: fn(&compare::Outcome) -> bool| rows.iter().filter(|o| f(o)).count();
    [
        rows.len(),
        count(|o| !matches!(o, compare::Outcome::Skipped(_))),
        count(|o| matches!(o, compare::Outcome::Equal)),
        count(|o| matches!(o, compare::Outcome::Mismatch(_))),
        count(|o| matches!(o, compare::Outcome::Skipped(_))),
    ]
}

/// Model parity of the kde-breeze builder (plan, section 1, rule 2): 0.7.2's `kde-breeze` and
/// `Theme::preset(Preset::KdeBreeze)`, light and dark, over the table of homes. Every row's group is a
/// ported one or one of [`NOT_PORTED_GROUPS`]; zero mismatches in the ported groups, every skip for its
/// named reason; the rows of the groups not ported yet are counted and reported, and the two counts make
/// the whole table. No dead rule in any rule set of the widgets.
#[test]
fn model_parity_kde_breeze() -> TestResult {
    use crate::next::preset::{Preset, Theme as NextTheme};
    use crate::next::query::ThemeContext;

    let table: Table = toml::from_str(HOMES)?;
    for row in &table.row {
        let group = row.key.split('.').next().unwrap_or("");
        assert!(
            PORTED_GROUPS.contains(&group) != NOT_PORTED_GROUPS.contains(&group),
            "`{}`: its group is not exactly one of the ported and the not ported groups",
            row.key
        );
    }
    for (mode, ctx) in [
        (ColorMode::Light, ThemeContext::light()),
        (ColorMode::Dark, ThemeContext::dark()),
    ] {
        let old = old_kde_breeze(mode)?;
        let unresolved = old_unresolved(&Theme::preset(PRESET)?, mode)?;
        let theme = NextTheme::preset(Preset::KdeBreeze);
        let new = theme.resolve(&ctx);
        let with_context = |context: RowContext| theme.resolve(&context.applied_to(&ctx));
        let extra = compare::Extra {
            unresolved: Some(&unresolved),
            with_context: Some(&with_context),
        };
        let report = compare::compare(&old, &new, None, Preset::KdeBreeze, &extra)?;
        println!("{mode:?}: {report}");
        assert_eq!(report.totals.rows, table.row.len());
        for row in report
            .rows
            .iter()
            .filter(|row| in_groups(&row.key, PORTED_GROUPS))
        {
            let skip = match row.outcome {
                compare::Outcome::Skipped(skip) => Some(skip),
                _ => None,
            };
            assert_eq!(skip, expected_skip(&row.key), "{mode:?}: `{}`", row.key);
        }
        let ported = group_totals(&report, true);
        let not_ported = totals_in(&report, NOT_PORTED_GROUPS, true);
        println!(
            "{mode:?}: [rows, compared, equal, mismatching, skipped] in the ported groups {ported:?}, \
             in the groups not ported yet {not_ported:?}"
        );
        assert_eq!(ported, PORTED_TOTALS, "{mode:?}\n{report}");
        assert_eq!(
            ported[0].saturating_add(not_ported[0]),
            table.row.len(),
            "{mode:?}"
        );
        let dead: Vec<_> = new
            .dead_rules()
            .into_iter()
            .filter(|(_, rules)| !rules.is_empty())
            .collect();
        assert!(dead.is_empty(), "{mode:?}: dead rules {dead:?}");
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------
// The value half on the adwaita builder (plan, M2's GNOME unit), static path: the same comparator and table.
// ---------------------------------------------------------------------------------------------------------

/// The groups of the table whose rows the adwaita builder ports so far: each of their rows is compared and
/// equal, or skipped for a reason [`adwaita_expected_skip`] names. Every group, as for kde-breeze.
const ADWAITA_PORTED_GROUPS: &[&str] = PORTED_GROUPS;

/// The groups of the table whose values the adwaita builder does not state yet: their rows are compared,
/// counted and reported, not asserted.
const ADWAITA_NOT_PORTED_GROUPS: &[&str] = &[];

/// The totals of the ported groups' rows on each scheme, `[rows, compared, equal, mismatching, skipped]`:
/// every row equal but those [`adwaita_expected_skip`] names, skipped.
const ADWAITA_PORTED_TOTALS: [usize; 5] = [563, 557, 557, 0, 6];

/// The reason a row of the table is skipped on 0.7.2's `adwaita` without a reader's snapshot: the reader's
/// context rows (`accessibility.*`); `checkbox.background_color`, compared only where
/// `checkbox.unchecked_background` is unset, and `menu.min_row_height`, compared only where
/// `menu.row_height` is unset, both of which `adwaita.toml` states.
fn adwaita_expected_skip(key: &str) -> Option<compare::Skip> {
    if key.starts_with("accessibility.") {
        Some(compare::Skip::NoSnapshot)
    } else if key == "checkbox.background_color" || key == "menu.min_row_height" {
        Some(compare::Skip::OtherKeyStated)
    } else {
        None
    }
}

/// Model parity of the adwaita builder (plan, section 1, rule 2): 0.7.2's `adwaita` and
/// `Theme::preset(Preset::Adwaita)`, light and dark, over the table of homes. Every row's group is one of
/// [`ADWAITA_PORTED_GROUPS`] or one of [`ADWAITA_NOT_PORTED_GROUPS`]; zero mismatches in the ported groups,
/// every skip for its named reason; the rows of the groups not ported yet are counted and reported, and the
/// two counts make the whole table. No dead rule in any rule set of the widgets.
#[test]
fn model_parity_adwaita() -> TestResult {
    use crate::next::preset::{Preset, Theme as NextTheme};
    use crate::next::query::ThemeContext;

    let table: Table = toml::from_str(HOMES)?;
    for row in &table.row {
        let group = row.key.split('.').next().unwrap_or("");
        assert!(
            ADWAITA_PORTED_GROUPS.contains(&group) != ADWAITA_NOT_PORTED_GROUPS.contains(&group),
            "`{}`: its group is not exactly one of the ported and the not ported groups",
            row.key
        );
    }
    for (mode, ctx) in [
        (ColorMode::Light, ThemeContext::light()),
        (ColorMode::Dark, ThemeContext::dark()),
    ] {
        let theme_072 = Theme::preset("adwaita")?;
        let old = old_resolved(&theme_072, mode)?;
        let unresolved = old_unresolved(&theme_072, mode)?;
        let theme = NextTheme::preset(Preset::Adwaita);
        let new = theme.resolve(&ctx);
        let with_context = |context: RowContext| theme.resolve(&context.applied_to(&ctx));
        let extra = compare::Extra {
            unresolved: Some(&unresolved),
            with_context: Some(&with_context),
        };
        let report = compare::compare(&old, &new, None, Preset::Adwaita, &extra)?;
        println!("{mode:?}: {report}");
        assert_eq!(report.totals.rows, table.row.len());
        for row in report
            .rows
            .iter()
            .filter(|row| in_groups(&row.key, ADWAITA_PORTED_GROUPS))
        {
            let skip = match row.outcome {
                compare::Outcome::Skipped(skip) => Some(skip),
                _ => None,
            };
            assert_eq!(
                skip,
                adwaita_expected_skip(&row.key),
                "{mode:?}: `{}`",
                row.key
            );
        }
        let ported = totals_in(&report, ADWAITA_PORTED_GROUPS, true);
        let not_ported = totals_in(&report, ADWAITA_NOT_PORTED_GROUPS, true);
        println!(
            "{mode:?}: [rows, compared, equal, mismatching, skipped] in the ported groups {ported:?}, \
             in the groups not ported yet {not_ported:?}"
        );
        assert_eq!(ported, ADWAITA_PORTED_TOTALS, "{mode:?}\n{report}");
        assert_eq!(
            ported[0].saturating_add(not_ported[0]),
            table.row.len(),
            "{mode:?}"
        );
        let dead: Vec<_> = new
            .dead_rules()
            .into_iter()
            .filter(|(_, rules)| !rules.is_empty())
            .collect();
        assert!(dead.is_empty(), "{mode:?}: dead rules {dead:?}");
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------
// The value half on the windows-11 builder (plan, M2's Windows unit), static path: the same comparator and
// table.
// ---------------------------------------------------------------------------------------------------------

/// The groups of the table whose rows the windows-11 builder ports so far: each of their rows is compared
/// and equal, or skipped for a reason [`windows_11_expected_skip`] names.
const WINDOWS_11_PORTED_GROUPS: &[&str] = PORTED_GROUPS;

/// The groups of the table whose values the windows-11 builder does not state yet: their rows are
/// compared, counted and reported, not asserted.
const WINDOWS_11_NOT_PORTED_GROUPS: &[&str] = &[];

/// The totals of the ported groups' rows on each scheme, `[rows, compared, equal, mismatching, skipped]`:
/// every row equal but those [`windows_11_expected_skip`] names, skipped.
const WINDOWS_11_PORTED_TOTALS: [usize; 5] = [563, 558, 558, 0, 5];

/// The reason a row of the table is skipped on 0.7.2's `windows-11` without a reader's snapshot: the
/// reader's context rows (`accessibility.*`); `checkbox.background_color`, compared only where
/// `checkbox.unchecked_background` is unset, which `windows-11.toml` states.
fn windows_11_expected_skip(key: &str) -> Option<compare::Skip> {
    if key.starts_with("accessibility.") {
        Some(compare::Skip::NoSnapshot)
    } else if key == "checkbox.background_color" {
        Some(compare::Skip::OtherKeyStated)
    } else {
        None
    }
}

/// Model parity of the windows-11 builder (plan, section 1, rule 2): 0.7.2's `windows-11` and
/// `Theme::preset(Preset::Windows11)`, light and dark, over the table of homes. Every row's group is one
/// of [`WINDOWS_11_PORTED_GROUPS`] or one of [`WINDOWS_11_NOT_PORTED_GROUPS`]; zero mismatches in the
/// ported groups, every skip for its named reason; the rows of the groups not ported yet are counted and
/// reported, and the two counts make the whole table. No dead rule in any rule set of the widgets.
#[test]
fn model_parity_windows_11() -> TestResult {
    use crate::next::preset::{Preset, Theme as NextTheme};
    use crate::next::query::ThemeContext;

    let table: Table = toml::from_str(HOMES)?;
    for row in &table.row {
        let group = row.key.split('.').next().unwrap_or("");
        assert!(
            WINDOWS_11_PORTED_GROUPS.contains(&group)
                != WINDOWS_11_NOT_PORTED_GROUPS.contains(&group),
            "`{}`: its group is not exactly one of the ported and the not ported groups",
            row.key
        );
    }
    for (mode, ctx) in [
        (ColorMode::Light, ThemeContext::light()),
        (ColorMode::Dark, ThemeContext::dark()),
    ] {
        let theme_072 = Theme::preset("windows-11")?;
        let old = old_resolved(&theme_072, mode)?;
        let unresolved = old_unresolved(&theme_072, mode)?;
        let theme = NextTheme::preset(Preset::Windows11);
        let new = theme.resolve(&ctx);
        let with_context = |context: RowContext| theme.resolve(&context.applied_to(&ctx));
        let extra = compare::Extra {
            unresolved: Some(&unresolved),
            with_context: Some(&with_context),
        };
        let report = compare::compare(&old, &new, None, Preset::Windows11, &extra)?;
        println!("{mode:?}: {report}");
        assert_eq!(report.totals.rows, table.row.len());
        for row in report
            .rows
            .iter()
            .filter(|row| in_groups(&row.key, WINDOWS_11_PORTED_GROUPS))
        {
            let skip = match row.outcome {
                compare::Outcome::Skipped(skip) => Some(skip),
                _ => None,
            };
            assert_eq!(
                skip,
                windows_11_expected_skip(&row.key),
                "{mode:?}: `{}`",
                row.key
            );
        }
        let ported = totals_in(&report, WINDOWS_11_PORTED_GROUPS, true);
        let not_ported = totals_in(&report, WINDOWS_11_NOT_PORTED_GROUPS, true);
        println!(
            "{mode:?}: [rows, compared, equal, mismatching, skipped] in the ported groups {ported:?}, \
             in the groups not ported yet {not_ported:?}"
        );
        assert_eq!(ported, WINDOWS_11_PORTED_TOTALS, "{mode:?}\n{report}");
        assert_eq!(
            ported[0].saturating_add(not_ported[0]),
            table.row.len(),
            "{mode:?}"
        );
        let dead: Vec<_> = new
            .dead_rules()
            .into_iter()
            .filter(|(_, rules)| !rules.is_empty())
            .collect();
        assert!(dead.is_empty(), "{mode:?}: dead rules {dead:?}");
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------
// The value half on the macos-monterey and macos-tahoe builders (plan, M2's macOS unit), static path: the
// same comparator and table.
// ---------------------------------------------------------------------------------------------------------

/// The groups of the table whose rows the macOS builders port so far: each of their rows is compared and
/// equal, or skipped for a reason [`macos_expected_skip`] names.
const MACOS_PORTED_GROUPS: &[&str] = &[
    "defaults",
    "text_scale",
    "Resolved",
    "layout",
    "accessibility",
    "button",
    "checkbox",
    "list",
    "scrollbar",
    "tooltip",
    "window",
    "input",
    "text_area",
    "link",
    "slider",
    "progress_bar",
    "splitter",
    "separator",
    "spinner",
    "tab",
    "menu",
    "toolbar",
    "status_bar",
    "sidebar",
    "popover",
    "switch",
    "dialog",
    "combo_box",
    "segmented_control",
    "card",
    "expander",
];

/// The groups of the table whose values the macOS builders do not state yet: their rows are compared,
/// counted and reported, not asserted.
const MACOS_NOT_PORTED_GROUPS: &[&str] = &[];

/// The totals of the ported groups' rows on each scheme of `macos-monterey`, `[rows, compared, equal,
/// mismatching, skipped]`: every row equal but those [`macos_expected_skip`] names, skipped.
const MACOS_MONTEREY_PORTED_TOTALS: [usize; 5] = [563, 557, 557, 0, 6];

/// The totals of the ported groups' rows on each scheme of `macos-tahoe`, as
/// [`MACOS_MONTEREY_PORTED_TOTALS`].
const MACOS_TAHOE_PORTED_TOTALS: [usize; 5] = [563, 558, 558, 0, 5];

/// The resolution context 0.7.2 resolves a theme in on macOS: `ResolutionContext::from_system`'s font DPI
/// there, 72, "where a point is a pixel" (`src/resolve/context.rs`), with the deterministic rest of
/// `for_tests`.
const MACOS_RESOLUTION_CONTEXT: ResolutionContext = ResolutionContext {
    font_dpi: 72.0,
    button_order: crate::model::DialogButtonOrder::PrimaryRight,
    icon_theme: None,
};

/// The reason a row of the table is skipped on 0.7.2's macOS presets without a reader's snapshot: the
/// reader's context rows (`accessibility.*`); `checkbox.background_color`, compared only where
/// `checkbox.unchecked_background` is unset, which both macOS TOMLs state; on `macos-monterey`
/// `menu.min_row_height`, compared only where `menu.row_height` is unset, which `macos-monterey.toml`
/// states.
fn macos_expected_skip(preset: crate::next::preset::Preset, key: &str) -> Option<compare::Skip> {
    if key.starts_with("accessibility.") {
        Some(compare::Skip::NoSnapshot)
    } else if key == "checkbox.background_color"
        || (key == "menu.min_row_height" && preset == crate::next::preset::Preset::MacosMonterey)
    {
        Some(compare::Skip::OtherKeyStated)
    } else {
        None
    }
}

/// Model parity of a macOS builder (plan, section 1, rule 2): 0.7.2's preset `name` and
/// `Theme::preset(preset)`, light and dark, over the table of homes. Every row's group is one of
/// [`MACOS_PORTED_GROUPS`] or one of [`MACOS_NOT_PORTED_GROUPS`]; zero mismatches in the ported groups,
/// every skip for its named reason, the ported groups' totals `totals`; the rows of the groups not ported
/// yet are counted and reported, and the two counts make the whole table. No dead rule in any rule set of
/// the widgets.
fn macos_model_parity(
    name: &str,
    preset: crate::next::preset::Preset,
    totals: [usize; 5],
) -> TestResult {
    use crate::next::preset::Theme as NextTheme;
    use crate::next::query::ThemeContext;

    let table: Table = toml::from_str(HOMES)?;
    for row in &table.row {
        let group = row.key.split('.').next().unwrap_or("");
        assert!(
            MACOS_PORTED_GROUPS.contains(&group) != MACOS_NOT_PORTED_GROUPS.contains(&group),
            "`{}`: its group is not exactly one of the ported and the not ported groups",
            row.key
        );
    }
    for (mode, ctx) in [
        (ColorMode::Light, ThemeContext::light()),
        (ColorMode::Dark, ThemeContext::dark()),
    ] {
        let theme_072 = Theme::preset(name)?;
        let old = old_resolved_in(&theme_072, mode, &MACOS_RESOLUTION_CONTEXT)?;
        let unresolved = old_unresolved(&theme_072, mode)?;
        let theme = NextTheme::preset(preset);
        let new = theme.resolve(&ctx);
        let with_context = |context: RowContext| theme.resolve(&context.applied_to(&ctx));
        let extra = compare::Extra {
            unresolved: Some(&unresolved),
            with_context: Some(&with_context),
        };
        let report = compare::compare(&old, &new, None, preset, &extra)?;
        println!("{name} {mode:?}: {report}");
        assert_eq!(report.totals.rows, table.row.len());
        for row in report
            .rows
            .iter()
            .filter(|row| in_groups(&row.key, MACOS_PORTED_GROUPS))
        {
            let skip = match row.outcome {
                compare::Outcome::Skipped(skip) => Some(skip),
                _ => None,
            };
            assert_eq!(
                skip,
                macos_expected_skip(preset, &row.key),
                "{name} {mode:?}: `{}`",
                row.key
            );
        }
        let ported = totals_in(&report, MACOS_PORTED_GROUPS, true);
        let not_ported = totals_in(&report, MACOS_NOT_PORTED_GROUPS, true);
        println!(
            "{name} {mode:?}: [rows, compared, equal, mismatching, skipped] in the ported groups \
             {ported:?}, in the groups not ported yet {not_ported:?}"
        );
        assert_eq!(ported, totals, "{name} {mode:?}\n{report}");
        assert_eq!(
            ported[0].saturating_add(not_ported[0]),
            table.row.len(),
            "{name} {mode:?}"
        );
        let dead: Vec<_> = new
            .dead_rules()
            .into_iter()
            .filter(|(_, rules)| !rules.is_empty())
            .collect();
        assert!(dead.is_empty(), "{name} {mode:?}: dead rules {dead:?}");
    }
    Ok(())
}

/// Model parity of the macos-monterey builder ([`macos_model_parity`]).
#[test]
fn model_parity_macos_monterey() -> TestResult {
    macos_model_parity(
        "macos-monterey",
        crate::next::preset::Preset::MacosMonterey,
        MACOS_MONTEREY_PORTED_TOTALS,
    )
}

/// Model parity of the macos-tahoe builder ([`macos_model_parity`]).
#[test]
fn model_parity_macos_tahoe() -> TestResult {
    macos_model_parity(
        "macos-tahoe",
        crate::next::preset::Preset::MacosTahoe,
        MACOS_TAHOE_PORTED_TOTALS,
    )
}

// ---------------------------------------------------------------------------------------------------------
// The value half on the material-3 builder (plan, M2's Material unit), static path: the same comparator and
// table.
// ---------------------------------------------------------------------------------------------------------

/// The groups of the table whose rows the material-3 builder ports so far: each of their rows is compared
/// and equal, or skipped for a reason [`material_3_expected_skip`] names.
const MATERIAL_3_PORTED_GROUPS: &[&str] = PORTED_GROUPS;

/// The groups of the table whose values the material-3 builder does not state yet: their rows are
/// compared, counted and reported, not asserted.
const MATERIAL_3_NOT_PORTED_GROUPS: &[&str] = &[];

/// The totals of the ported groups' rows on each scheme, `[rows, compared, equal, mismatching, skipped]`:
/// every row equal but those [`material_3_expected_skip`] names, skipped.
const MATERIAL_3_PORTED_TOTALS: [usize; 5] = [563, 557, 557, 0, 6];

/// The reason a row of the table is skipped on 0.7.2's `material-3`, which has no reader: the reader's
/// context rows (`accessibility.*`); `checkbox.background_color`, compared only where
/// `checkbox.unchecked_background` is unset, and `menu.min_row_height`, compared only where
/// `menu.row_height` is unset, both of which `material-3.toml` states.
fn material_3_expected_skip(key: &str) -> Option<compare::Skip> {
    if key.starts_with("accessibility.") {
        Some(compare::Skip::NoSnapshot)
    } else if key == "checkbox.background_color" || key == "menu.min_row_height" {
        Some(compare::Skip::OtherKeyStated)
    } else {
        None
    }
}

/// Model parity of the material-3 builder (plan, section 1, rule 2): 0.7.2's `material-3` and
/// `Theme::preset(Preset::Material3)`, light and dark, over the table of homes. Every row's group is one
/// of [`MATERIAL_3_PORTED_GROUPS`] or one of [`MATERIAL_3_NOT_PORTED_GROUPS`]; zero mismatches in the
/// ported groups, every skip for its named reason; the rows of the groups not ported yet are counted and
/// reported, and the two counts make the whole table. No dead rule in any rule set of the widgets.
#[test]
fn model_parity_material_3() -> TestResult {
    use crate::next::preset::{Preset, Theme as NextTheme};
    use crate::next::query::ThemeContext;

    let table: Table = toml::from_str(HOMES)?;
    for row in &table.row {
        let group = row.key.split('.').next().unwrap_or("");
        assert!(
            MATERIAL_3_PORTED_GROUPS.contains(&group)
                != MATERIAL_3_NOT_PORTED_GROUPS.contains(&group),
            "`{}`: its group is not exactly one of the ported and the not ported groups",
            row.key
        );
    }
    for (mode, ctx) in [
        (ColorMode::Light, ThemeContext::light()),
        (ColorMode::Dark, ThemeContext::dark()),
    ] {
        let theme_072 = Theme::preset("material-3")?;
        let old = old_resolved(&theme_072, mode)?;
        let unresolved = old_unresolved(&theme_072, mode)?;
        let theme = NextTheme::preset(Preset::Material3);
        let new = theme.resolve(&ctx);
        let with_context = |context: RowContext| theme.resolve(&context.applied_to(&ctx));
        let extra = compare::Extra {
            unresolved: Some(&unresolved),
            with_context: Some(&with_context),
        };
        let report = compare::compare(&old, &new, None, Preset::Material3, &extra)?;
        println!("{mode:?}: {report}");
        assert_eq!(report.totals.rows, table.row.len());
        for row in report
            .rows
            .iter()
            .filter(|row| in_groups(&row.key, MATERIAL_3_PORTED_GROUPS))
        {
            let skip = match row.outcome {
                compare::Outcome::Skipped(skip) => Some(skip),
                _ => None,
            };
            assert_eq!(
                skip,
                material_3_expected_skip(&row.key),
                "{mode:?}: `{}`",
                row.key
            );
        }
        let ported = totals_in(&report, MATERIAL_3_PORTED_GROUPS, true);
        let not_ported = totals_in(&report, MATERIAL_3_NOT_PORTED_GROUPS, true);
        println!(
            "{mode:?}: [rows, compared, equal, mismatching, skipped] in the ported groups {ported:?}, \
             in the groups not ported yet {not_ported:?}"
        );
        assert_eq!(ported, MATERIAL_3_PORTED_TOTALS, "{mode:?}\n{report}");
        assert_eq!(
            ported[0].saturating_add(not_ported[0]),
            table.row.len(),
            "{mode:?}"
        );
        let dead: Vec<_> = new
            .dead_rules()
            .into_iter()
            .filter(|(_, rules)| !rules.is_empty())
            .collect();
        assert!(dead.is_empty(), "{mode:?}: dead rules {dead:?}");
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------
// The value half on the ios-13 builder (plan, M2's iOS unit), static path: the same comparator and table.
// ---------------------------------------------------------------------------------------------------------

/// The groups of the table whose rows the ios-13 builder ports so far: each of their rows is compared and
/// equal, or skipped for a reason [`ios_13_expected_skip`] names.
const IOS_13_PORTED_GROUPS: &[&str] = PORTED_GROUPS;

/// The groups of the table whose values the ios-13 builder does not state yet: their rows are compared,
/// counted and reported, not asserted.
const IOS_13_NOT_PORTED_GROUPS: &[&str] = &[];

/// The totals of the ported groups' rows on each scheme, `[rows, compared, equal, mismatching, skipped]`:
/// every row equal but those [`ios_13_expected_skip`] names, skipped.
const IOS_13_PORTED_TOTALS: [usize; 5] = [563, 558, 558, 0, 5];

/// The resolution context 0.7.2 resolves `ios-13` in: Apple's font DPI, 72, "where a point is a pixel"
/// (spec §4, `Typography::font_dpi`: "72.0 Apple"), as [`MACOS_RESOLUTION_CONTEXT`] states it.
/// 0.7.2's `ResolutionContext::from_system` gives 72 on macOS alone (`src/resolve/context.rs`).
const IOS_RESOLUTION_CONTEXT: ResolutionContext = MACOS_RESOLUTION_CONTEXT;

/// The reason a row of the table is skipped on 0.7.2's `ios-13`, which has no reader: the reader's
/// context rows (`accessibility.*`); `checkbox.background_color`, compared only where
/// `checkbox.unchecked_background` is unset, which `ios-13.toml` states.
fn ios_13_expected_skip(key: &str) -> Option<compare::Skip> {
    if key.starts_with("accessibility.") {
        Some(compare::Skip::NoSnapshot)
    } else if key == "checkbox.background_color" {
        Some(compare::Skip::OtherKeyStated)
    } else {
        None
    }
}

/// Model parity of the ios-13 builder (plan, section 1, rule 2): 0.7.2's `ios-13` at Apple's font DPI
/// ([`IOS_RESOLUTION_CONTEXT`]) and `Theme::preset(Preset::Ios13)`, light and dark, over the table of
/// homes. Every row's group is one of [`IOS_13_PORTED_GROUPS`] or one of [`IOS_13_NOT_PORTED_GROUPS`];
/// zero mismatches in the ported groups, every skip for its named reason; the rows of the groups not
/// ported yet are counted and reported, and the two counts make the whole table. No dead rule in any rule
/// set of the widgets.
#[test]
fn model_parity_ios_13() -> TestResult {
    use crate::next::preset::{Preset, Theme as NextTheme};
    use crate::next::query::ThemeContext;

    let table: Table = toml::from_str(HOMES)?;
    for row in &table.row {
        let group = row.key.split('.').next().unwrap_or("");
        assert!(
            IOS_13_PORTED_GROUPS.contains(&group) != IOS_13_NOT_PORTED_GROUPS.contains(&group),
            "`{}`: its group is not exactly one of the ported and the not ported groups",
            row.key
        );
    }
    for (mode, ctx) in [
        (ColorMode::Light, ThemeContext::light()),
        (ColorMode::Dark, ThemeContext::dark()),
    ] {
        let theme_072 = Theme::preset("ios-13")?;
        let old = old_resolved_in(&theme_072, mode, &IOS_RESOLUTION_CONTEXT)?;
        let unresolved = old_unresolved(&theme_072, mode)?;
        let theme = NextTheme::preset(Preset::Ios13);
        let new = theme.resolve(&ctx);
        let with_context = |context: RowContext| theme.resolve(&context.applied_to(&ctx));
        let extra = compare::Extra {
            unresolved: Some(&unresolved),
            with_context: Some(&with_context),
        };
        let report = compare::compare(&old, &new, None, Preset::Ios13, &extra)?;
        println!("{mode:?}: {report}");
        assert_eq!(report.totals.rows, table.row.len());
        for row in report
            .rows
            .iter()
            .filter(|row| in_groups(&row.key, IOS_13_PORTED_GROUPS))
        {
            let skip = match row.outcome {
                compare::Outcome::Skipped(skip) => Some(skip),
                _ => None,
            };
            assert_eq!(
                skip,
                ios_13_expected_skip(&row.key),
                "{mode:?}: `{}`",
                row.key
            );
        }
        let ported = totals_in(&report, IOS_13_PORTED_GROUPS, true);
        let not_ported = totals_in(&report, IOS_13_NOT_PORTED_GROUPS, true);
        println!(
            "{mode:?}: [rows, compared, equal, mismatching, skipped] in the ported groups {ported:?}, \
             in the groups not ported yet {not_ported:?}"
        );
        assert_eq!(ported, IOS_13_PORTED_TOTALS, "{mode:?}\n{report}");
        assert_eq!(
            ported[0].saturating_add(not_ported[0]),
            table.row.len(),
            "{mode:?}"
        );
        let dead: Vec<_> = new
            .dead_rules()
            .into_iter()
            .filter(|(_, rules)| !rules.is_empty())
            .collect();
        assert!(dead.is_empty(), "{mode:?}: dead rules {dead:?}");
    }
    Ok(())
}

// Reader parity (plan, section 1, rule 2) on the KDE fixtures: the theme the 0.7.2 KDE reader path produces
// from a kdeglobals file, and the kde-breeze builder fed the same file as `SystemOverrides`.
// ---------------------------------------------------------------------------------------------------------

/// The totals of the rows of [`PORTED_GROUPS`] on every fixture, `[rows, compared, equal, mismatching,
/// skipped]`: every row equal but `checkbox.background_color`, skipped where
/// `checkbox.unchecked_background` is stated (the table's listed differences).
#[cfg(all(target_os = "linux", feature = "kde"))]
const READER_TOTALS: [usize; 5] = [563, 562, 562, 0, 1];

/// The number of files in `tests/fixtures/kde/`: breeze-dark.ini, breeze-index.theme, breeze-light.ini,
/// custom-accent.ini, high-dpi.ini, malformed-values.ini, minimal.ini, missing-groups.ini.
#[cfg(all(target_os = "linux", feature = "kde"))]
const KDE_FIXTURES: usize = 8;

/// The 0.7.2 KDE reader path on the kdeglobals `content`, as the comparator reads it, and the name it gives
/// the theme: the reader's pure function (`kde::from_kde_content_pure`) at the font DPI the reader states,
/// its theme merged over `kde-breeze` as the pipeline merges a reader's result (`pipeline.rs`, `replay`),
/// resolved for the reader's scheme with the rows written by hand added as [`old_kde_breeze`] adds them,
/// and the reader's `accessibility` block at its key; also the merged theme's variant unresolved
/// ([`old_unresolved`]).
#[cfg(all(target_os = "linux", feature = "kde"))]
fn old_kde_reader(
    content: &str,
) -> Result<(serde_json::Value, serde_json::Value, String), Box<dyn std::error::Error>> {
    let (reader, _, accessibility) =
        crate::kde::from_kde_content_pure(content, Some(crate::detect::LINUX_FONT_DPI))?;
    let mode = if reader.dark.is_some() {
        ColorMode::Dark
    } else {
        ColorMode::Light
    };
    let (old, unresolved) = old_merged_reader(PRESET, &reader, mode, &accessibility)?;
    Ok((old, unresolved, reader.name.to_string()))
}

/// A 0.7.2 reader's theme `reader` merged over the 0.7.2 preset `preset`, resolved for `mode` with the
/// rows written by hand ([`old_resolved`]) and the reader's `accessibility` block at its key, and
/// unresolved ([`old_unresolved`]).
fn old_merged_reader(
    preset: &str,
    reader: &Theme,
    mode: ColorMode,
    accessibility: &crate::AccessibilityPreferences,
) -> Result<(serde_json::Value, serde_json::Value), Box<dyn std::error::Error>> {
    old_merged_reader_in(
        preset,
        reader,
        mode,
        accessibility,
        &ResolutionContext::for_tests(),
    )
}

/// [`old_merged_reader`] resolved in the resolution context `context`.
fn old_merged_reader_in(
    preset: &str,
    reader: &Theme,
    mode: ColorMode,
    accessibility: &crate::AccessibilityPreferences,
    context: &ResolutionContext,
) -> Result<(serde_json::Value, serde_json::Value), Box<dyn std::error::Error>> {
    let mut merged = Theme::preset(preset)?;
    merged.merge(reader);
    let mut old = old_resolved_in(&merged, mode, context)?;
    let unresolved = old_unresolved(&merged, mode)?;
    let map = old
        .as_object_mut()
        .ok_or("the resolved theme does not serialise as an object")?;
    map.insert(
        "accessibility".to_owned(),
        serde_json::json!({
            "text_scaling_factor": accessibility.text_scaling_factor,
            "reduce_motion": accessibility.reduce_motion,
            "high_contrast": accessibility.high_contrast,
            "reduce_transparency": accessibility.reduce_transparency,
        }),
    );
    Ok((old, unresolved))
}

/// Reader parity of the kde-breeze builder: for every file of `tests/fixtures/kde/`, the 0.7.2 reader
/// path ([`old_kde_reader`]) against `Theme::preset(Preset::KdeBreeze)` given the file's
/// `SystemOverrides` and resolved for the file's context (its motion level and text scale at their
/// defaults, its animation scale unset, as 0.7.2 applies neither), over the table of homes, the
/// snapshot's context compared. Zero mismatches over every row of [`PORTED_GROUPS`] but [`KDE_BUTTON_HOVER`]'s
/// on [`KDE_FIXTURES_OTHER_BUTTON`], the four `snapshot.context.*` rows
/// compared and equal, the one skip for its named reason; the other rows counted and reported, the two
/// counts making the whole table; the resolved theme's name is the reader's.
#[cfg(all(target_os = "linux", feature = "kde"))]
#[test]
fn reader_parity_kde() -> TestResult {
    let table: Table = toml::from_str(HOMES)?;
    let dir = kde_fixtures();
    let mut paths = std::fs::read_dir(&dir)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    assert_eq!(paths.len(), KDE_FIXTURES, "the files of {dir:?}");
    for path in &paths {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let listed = if KDE_FIXTURES_OTHER_BUTTON.contains(&name) {
            KDE_BUTTON_HOVER
        } else {
            &[]
        };
        reader_parity_on(name, &std::fs::read_to_string(path)?, &table, listed)?;
    }
    Ok(())
}

/// The folder of the KDE fixtures.
#[cfg(all(target_os = "linux", feature = "kde"))]
fn kde_fixtures() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/kde")
}

/// Reader parity of the kde-breeze builder on the kdeglobals `content` named `name`, as
/// [`reader_parity_kde`] states it for each file, with the listed differences `listed`; the 0.7.2
/// reader path's theme.
#[cfg(all(target_os = "linux", feature = "kde"))]
fn reader_parity_on(
    name: &str,
    content: &str,
    table: &Table,
    listed: &[&str],
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    use crate::next::reader::kde::snapshot_from_kdeglobals;

    let (old, unresolved, old_name) = old_kde_reader(content)?;
    let mut ini = crate::kde::create_kde_parser();
    crate::kde::read_kconfig(&mut ini, content);
    let snapshot = snapshot_from_kdeglobals(&ini, None, None);
    assert_reader_parity(
        name,
        &old,
        &unresolved,
        &old_name,
        &snapshot,
        table,
        &kde_reader_expected(listed),
    )?;
    Ok(old)
}

/// What reader parity expects of a preset's builder on one input: its totals in the ported groups
/// (`[rows, compared, equal, mismatching, skipped]`), the reason a row is skipped for, and the keys of
/// the rows that differ, each a listed difference with its reason given where the input is stated.
struct ReaderExpected<'a> {
    /// The totals in the ported groups.
    totals: [usize; 5],
    /// The reason the row of a key is skipped for, `None` where it is compared.
    skip: fn(&str) -> Option<compare::Skip>,
    /// The keys of the rows that differ.
    listed: &'a [&'a str],
}

/// The kde-breeze builder's reader parity on a KDE input whose listed differences are `listed`: every
/// other row of the ported groups compared and equal but `checkbox.background_color`, skipped where
/// `checkbox.unchecked_background` is stated (the table's listed differences).
#[cfg(all(target_os = "linux", feature = "kde"))]
fn kde_reader_expected<'a>(listed: &'a [&'a str]) -> ReaderExpected<'a> {
    let [rows, compared, ..] = READER_TOTALS;
    ReaderExpected {
        totals: [
            rows,
            compared,
            compared.saturating_sub(listed.len()),
            listed.len(),
            rows.saturating_sub(compared),
        ],
        skip: |key| (key == "checkbox.background_color").then_some(compare::Skip::OtherKeyStated),
        listed,
    }
}

/// M2.FS4: the rows the kde-breeze builder fills in the colour scheme's Button background,
/// `[Colors:Button] BackgroundNormal`, where 0.7.2's reader leaves the preset's Breeze-default
/// constant: the hovered check box and radio (platform-facts.md (§2.5) `hover_background`, the box
/// keeps palette.button(), breezehelper.cpp:862 at breeze f0b1d75), slider handle (platform-facts.md
/// (§2.9) `thumb_hover_color`, QPalette::Button in every state, breezestyle.cpp:8122) and combo box
/// (platform-facts.md (§2.24) `hover_background`, a button frame whose hover turns only the pen,
/// breezehelper.cpp:710-712). They differ on a fixture whose Button background is not Breeze's.
#[cfg(all(target_os = "linux", feature = "kde"))]
const KDE_BUTTON_HOVER: &[&str] = &[
    "checkbox.hover_background",
    "slider.thumb_hover_color",
    "combo_box.hover_background",
];

/// The KDE fixtures whose `[Colors:Button] BackgroundNormal` (49,54,59) is not the kde-breeze preset's
/// Button background (#292c30 dark, #fcfcfc light): [`KDE_BUTTON_HOVER`] is listed on them.
#[cfg(all(target_os = "linux", feature = "kde"))]
const KDE_FIXTURES_OTHER_BUTTON: &[&str] = &[
    "breeze-dark.ini",
    "custom-accent.ini",
    "high-dpi.ini",
    "missing-groups.ini",
];

/// Reader parity of the builder of `snapshot`'s preset given `snapshot` against the 0.7.2 reader path's
/// theme `old` (`unresolved` before its resolution, `old_name` its name), as [`reader_parity_kde`] states
/// it for each file, with `expected`'s totals, skips and listed differences.
fn assert_reader_parity(
    name: &str,
    old: &serde_json::Value,
    unresolved: &serde_json::Value,
    old_name: &str,
    snapshot: &crate::next::reader::SystemSnapshot,
    table: &Table,
    expected: &ReaderExpected<'_>,
) -> TestResult {
    use crate::next::preset::Theme as NextTheme;

    // 0.7.2 applies neither the motion setting nor the text scale to the theme: the theme is
    // resolved with both at their defaults and the animation scale unset, and the settings are
    // compared through their own rows (spec §8, "Every reader", *Context*).
    let mut overrides = snapshot.overrides.clone();
    overrides.animation_scale = None;
    let defaults = crate::next::query::ThemeContext::default();
    let context = crate::next::query::ThemeContext {
        motion: defaults.motion,
        text_scale: defaults.text_scale,
        ..snapshot.context.clone()
    };
    let theme = NextTheme::preset(snapshot.preset).with_system_overrides(overrides);
    let new = theme.resolve(&context);
    let with_context = |row: RowContext| theme.resolve(&row.applied_to(&context));
    let extra = compare::Extra {
        unresolved: Some(unresolved),
        with_context: Some(&with_context),
    };
    let report = compare::compare(old, &new, Some(&snapshot.context), snapshot.preset, &extra)?;
    println!("{name}: {report}");
    assert_eq!(report.totals.rows, table.row.len(), "{name}");
    assert_eq!(&*new.name, old_name, "{name}");
    let totals = totals_in(&report, PORTED_GROUPS, true);
    let not_ported = totals_in(&report, PORTED_GROUPS, false);
    println!(
        "{name}: [rows, compared, equal, mismatching, skipped] in the ported groups {totals:?}, \
         in the groups not ported yet {not_ported:?}"
    );
    let mismatching: Vec<&str> = report
        .rows
        .iter()
        .filter(|row| {
            in_groups(&row.key, PORTED_GROUPS)
                && matches!(row.outcome, compare::Outcome::Mismatch(_))
        })
        .map(|row| row.key.as_str())
        .collect();
    assert_eq!(mismatching, expected.listed, "{name}\n{report}");
    assert_eq!(totals, expected.totals, "{name}\n{report}");
    assert_eq!(
        totals[0].saturating_add(not_ported[0]),
        table.row.len(),
        "{name}"
    );
    for row in report
        .rows
        .iter()
        .filter(|row| in_groups(&row.key, PORTED_GROUPS))
    {
        let skip = match row.outcome {
            compare::Outcome::Skipped(skip) => Some(skip),
            _ => None,
        };
        assert_eq!(skip, (expected.skip)(&row.key), "{name}: `{}`", row.key);
        if row.key.starts_with("accessibility.") {
            assert!(
                matches!(row.outcome, compare::Outcome::Equal),
                "{name}: `{}`",
                row.key
            );
        }
    }
    Ok(())
}

/// The live check of plan `M3`, gate item 3: the theme from the live KDE desktop through the 0.7.2 path
/// (the reader 0.7.2's `from_system` chooses, its theme merged over `kde-breeze` as its pipeline merges
/// it) and through the 0.8.0 path (`Theme::from_system`), compared as [`reader_parity_kde`] compares a
/// fixture's. It prints the comparator's report and the 0.8.0 reader's diagnostics, and fails where the
/// two differ. Run on the maintainer's KDE desktop:
/// `cargo nextest run -p native-theme --features kde,portal live_system_parity_kde --run-ignored only --no-capture`.
#[cfg(all(target_os = "linux", feature = "kde"))]
#[test]
#[ignore = "reads the live KDE desktop: the live check of plan M3"]
fn live_system_parity_kde() -> TestResult {
    use crate::next::preset::{Preset, Theme as NextTheme};
    use crate::next::reader::SystemSnapshot;

    let table: Table = toml::from_str(HOMES)?;
    let selected = pollster::block_on(crate::pipeline::select_reader())
        .ok_or("0.7.2 chooses no reader on this desktop")?;
    if selected.preset != PRESET {
        return Err(format!("0.7.2 chooses the preset {}, not {PRESET}", selected.preset).into());
    }
    let result = pollster::block_on(selected.reader.read())?;
    let reader = result
        .output
        .to_theme(&result.name, result.icon_set, &result.layout);
    let (old, unresolved) =
        old_merged_reader(PRESET, &reader, result.output.mode(), &result.accessibility)?;
    let theme = NextTheme::from_system()?;
    println!("0.8.0 diagnostics: {:?}", theme.diagnostics());
    assert_eq!(theme.preset_id(), Preset::KdeBreeze);
    let overrides = theme
        .system_overrides()
        .ok_or("the theme from the system has no system settings")?;
    let snapshot = SystemSnapshot::new(
        theme.preset_id(),
        theme.context().clone(),
        overrides.clone(),
    );
    assert_reader_parity(
        "the live desktop",
        &old,
        &unresolved,
        &result.name,
        &snapshot,
        &table,
        &kde_reader_expected(&[]),
    )?;
    println!("the live desktop: equal through the 0.7.2 and the 0.8.0 path");
    Ok(())
}

/// Reader parity on kdeglobals texts that move off their defaults the settings no fixture
/// states: Breeze Light's file with `[KDE] frameContrast=0.5`, `contrast=3` and
/// `[ColorEffects:Inactive] ChangeSelectionColor=false`, and with only `[ColorEffects:Inactive]
/// Enable=false`. Zero mismatches in the ported groups on both, and every row those settings feed differs from the
/// plain file's, so that equality proves the builder used them.
#[cfg(all(target_os = "linux", feature = "kde"))]
#[test]
fn reader_parity_kde_settings() -> TestResult {
    const INACTIVE_SELECTION: [&str; 2] = [
        "defaults.selection_inactive_background",
        "defaults.selection_inactive_text_color",
    ];
    let table: Table = toml::from_str(HOMES)?;
    let plain = std::fs::read_to_string(kde_fixtures().join("breeze-light.ini"))?;
    let base = reader_parity_on("breeze-light.ini", &plain, &table, &[])?;
    let contrasts = plain.replacen("[KDE]\n", "[KDE]\nframeContrast=0.5\ncontrast=3\n", 1);
    assert_ne!(contrasts, plain, "no [KDE] group in breeze-light.ini");
    let settings = format!("{contrasts}\n[ColorEffects:Inactive]\nChangeSelectionColor=false\n");
    let effects_off = format!("{plain}\n[ColorEffects:Inactive]\nEnable=false\n");
    let fed_by_settings = [
        "button.border.color",
        "button.default_button_border_color",
        "checkbox.unchecked_border_color",
        "list.header_separator_color",
        "list.grid_color",
        "progress_bar.border.color",
        "menu.separator_color",
        "toolbar.border.color",
        "popover.border.color",
        "card.border.color",
        "switch.unchecked_track_border_color",
        "switch.thumb_border_color",
    ]
    .into_iter()
    .chain(INACTIVE_SELECTION)
    .collect::<Vec<_>>();
    for (name, content, fed) in [
        (
            "frameContrast, contrast, ChangeSelectionColor",
            settings,
            fed_by_settings,
        ),
        (
            "inactive effects off",
            effects_off,
            INACTIVE_SELECTION.to_vec(),
        ),
    ] {
        let old = reader_parity_on(name, &content, &table, &[])?;
        for key in fed {
            let pointer = format!("/{}", key.replace('.', "/"));
            let got = old.pointer(&pointer);
            assert!(got.is_some(), "{name}: no `{key}`");
            assert_ne!(got, base.pointer(&pointer), "{name}: `{key}`");
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------
// The GNOME reader: the clean-room Pango parser against 0.7.2's parse (plan, M2, "The reader").
// ---------------------------------------------------------------------------------------------------------

/// A font as 0.7.2's `gnome::parse_gnome_font_to_fontspec` returns it: family, size (its unit and value),
/// weight and style.
#[cfg(all(target_os = "linux", feature = "portal"))]
type FontOutputs = Option<(
    Option<String>,
    Option<(&'static str, f32)>,
    Option<u16>,
    Option<String>,
)>;

/// The rows of the clean-room Pango parser's table (`tests/vectors/reader.pango_font_description.tsv`,
/// Pango 1.58.2) whose font, as the GNOME reader reads it (`reader::portal::font`), differs from 0.7.2's
/// parse of the same string: each the input and both outputs, 0.7.2's first.
#[cfg(all(target_os = "linux", feature = "portal"))]
fn pango_rows_differing_from_072() -> Result<Vec<(String, FontOutputs, FontOutputs)>, String> {
    use crate::next::reader::pango_font_description::tests::table_rows;

    let old_size = |size: crate::model::font::FontSize| match size {
        crate::model::font::FontSize::Pt(v) => ("Pt", v),
        crate::model::font::FontSize::Px(v) => ("Px", v),
    };
    let new_size = |size: crate::next::style::FontSize| match size {
        crate::next::style::FontSize::Pt(v) => ("Pt", v),
        crate::next::style::FontSize::Px(v) => ("Px", v),
    };
    Ok(table_rows()?
        .into_iter()
        .filter_map(|(_, input, _)| {
            let old: FontOutputs = crate::gnome::parse_gnome_font_to_fontspec(&input).map(|f| {
                (
                    f.family.map(|family| family.to_string()),
                    f.size.map(old_size),
                    f.weight,
                    f.style.map(|style| format!("{style:?}")),
                )
            });
            let new: FontOutputs = crate::next::reader::portal::font(Some(&input)).map(|f| {
                (
                    f.family.map(|family| family.to_string()),
                    f.size.map(new_size),
                    f.weight,
                    f.style.map(|style| format!("{style:?}")),
                )
            });
            (old != new).then_some((input, old, new))
        })
        .collect())
}

/// The number of the table's 263 rows on which the two parses differ, each where Pango's own output
/// (the table's) differs from 0.7.2's: a size not on Pango's grid of 1/1024 (`Sans 10.3`, `Sans 0.0005`,
/// `Sans 7.00048828125`, `Sans 7.000488281`), or below its half step, which Pango stores as 0, no size
/// (`Sans 0.0001`, `Sans 0.0004`); a hexadecimal size (`Sans 0x10`, `Sans 0x1p3px`); a vertical tab before
/// a number (`Sans \v11`, `Sans \v\v+11px`, `Sans weight=\v700 11`, `Sans weight=\v-0 11`); a quoted
/// family (`'Cantarell' 11`: 0.7.2 strips the string's leading quote, Pango keeps both).
#[cfg(all(target_os = "linux", feature = "portal"))]
const PANGO_ROWS_DIFFERING: usize = 13;

/// The plan's first test of the clean-room Pango parser: its vector table run through the GNOME reader's
/// font parse and through 0.7.2's, compared on the outputs 0.7.2's function returns (family, size, weight,
/// style). The rows that differ are printed and counted ([`PANGO_ROWS_DIFFERING`]); they are the reader
/// parity's listed differences of the parser (plan, Appendix B: the clean-room parser replaces 0.7.2's
/// whatever this test shows, the table its citation).
#[cfg(all(target_os = "linux", feature = "portal"))]
#[test]
fn pango_parser_table_against_072() -> TestResult {
    let differing = pango_rows_differing_from_072()?;
    for (input, old, new) in &differing {
        println!("{input:?}\n  0.7.2 {old:?}\n  0.8.0 {new:?}");
    }
    assert_eq!(differing.len(), PANGO_ROWS_DIFFERING);
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------
// Reader parity (plan, section 1, rule 2) on GNOME: the theme the 0.7.2 GNOME reader path produces from the
// portal's data (`gnome::build_gnome_spec_pure`), and the adwaita builder fed the same data as
// `SystemOverrides` (`reader::portal::snapshot_from_gnome_data`).
// ---------------------------------------------------------------------------------------------------------

/// The 0.7.2 GNOME reader path on the portal's data `data`, as the comparator reads it, and the name it
/// gives the theme: the reader's pure function (`gnome::build_gnome_spec_pure`), its theme merged over
/// `adwaita` as the pipeline merges a reader's result, resolved for the reader's scheme with the rows
/// written by hand and the reader's `accessibility` block ([`old_merged_reader`]); also the merged
/// theme's variant unresolved.
#[cfg(all(target_os = "linux", feature = "portal"))]
fn old_gnome_reader(
    data: &crate::gnome::GnomePortalData,
) -> Result<(serde_json::Value, serde_json::Value, String), Box<dyn std::error::Error>> {
    let result = crate::gnome::build_gnome_spec_pure(data);
    let reader = result
        .output
        .to_theme(&result.name, result.icon_set, &result.layout);
    let (old, unresolved) = old_merged_reader(
        "adwaita",
        &reader,
        result.output.mode(),
        &result.accessibility,
    )?;
    Ok((old, unresolved, result.name.to_string()))
}

/// The portal's data of 0.7.2's GNOME reader tests (`gnome/mod.rs`, `default_gnome_data`): light, no
/// accent, the fonts `Cantarell 11`, `Source Code Pro 10` and `Cantarell Bold 11`, a text-scaling factor
/// of 1.0, overlay scroll bars, the icon theme `Adwaita`.
#[cfg(all(target_os = "linux", feature = "portal"))]
fn default_gnome_data() -> crate::gnome::GnomePortalData {
    crate::gnome::GnomePortalData {
        is_dark: false,
        accent_rgb: None,
        high_contrast: false,
        reduce_motion: None,
        font_name: Some("Cantarell 11".to_owned()),
        monospace_font_name: Some("Source Code Pro 10".to_owned()),
        titlebar_font: Some("Cantarell Bold 11".to_owned()),
        text_scaling_factor: Some(1.0),
        session_type: None,
        overlay_scrolling: Some(true),
        cursor_blink: None,
        cursor_blink_time: None,
        cursor_blink_timeout: None,
        icon_theme: Some("Adwaita".to_owned()),
        gsettings_high_contrast: None,
    }
}

/// An accent of 0.7.2's GNOME reader tests as the portal gives it: each 8-bit channel over 255.
#[cfg(all(target_os = "linux", feature = "portal"))]
fn portal_accent((r, g, b): (u8, u8, u8)) -> Option<(f64, f64, f64)> {
    Some((
        f64::from(r) / 255.0,
        f64::from(g) / 255.0,
        f64::from(b) / 255.0,
    ))
}

/// The cases of GNOME's reader parity: the data of 0.7.2's GNOME reader tests (`gnome/mod.rs`), each
/// named, with the keys of the rows that differ, each a listed difference. The accents are libadwaita's
/// blue and purple (`#3584e4`, `#9141ac`, the tests' documented accents).
#[cfg(all(target_os = "linux", feature = "portal"))]
pub(crate) fn gnome_cases() -> Vec<(
    &'static str,
    crate::gnome::GnomePortalData,
    &'static [&'static str],
)> {
    const BLUE: (u8, u8, u8) = (0x35, 0x84, 0xe4);
    const PURPLE: (u8, u8, u8) = (0x91, 0x41, 0xac);
    let base = default_gnome_data;
    let with = |f: fn(&mut crate::gnome::GnomePortalData)| {
        let mut data = base();
        f(&mut data);
        data
    };
    vec![
        ("the default data", base(), &[]),
        ("dark", with(|d| d.is_dark = true), &[]),
        (
            "blue accent, light",
            with(|d| d.accent_rgb = portal_accent(BLUE)),
            &[],
        ),
        (
            "blue accent, dark",
            with(|d| {
                d.is_dark = true;
                d.accent_rgb = portal_accent(BLUE);
            }),
            &[],
        ),
        (
            "purple accent, light",
            with(|d| d.accent_rgb = portal_accent(PURPLE)),
            &[],
        ),
        (
            "purple accent, dark",
            with(|d| {
                d.is_dark = true;
                d.accent_rgb = portal_accent(PURPLE);
            }),
            &[],
        ),
        (
            "purple accent, high contrast from the portal",
            with(|d| {
                d.accent_rgb = portal_accent(PURPLE);
                d.high_contrast = true;
            }),
            &[],
        ),
        (
            "blue accent, dark, high contrast from gsettings",
            with(|d| {
                d.is_dark = true;
                d.accent_rgb = portal_accent(BLUE);
                d.gsettings_high_contrast = Some(true);
            }),
            &[],
        ),
        // Listed: under `Contrast::High` the adwaita builder draws the preset's focus ring, and the
        // focused text field's outline, at libadwaita's `$focus_border_color_hc` alpha of 0.8 (its
        // module documentation; spec §8, "The GNOME part": `adwaita` defines `High` for that value);
        // 0.7.2's reader changes that alpha only through a portal accent, and without one keeps the
        // preset's ring at 0.5. libadwaita 1.10.0 draws the ring under `prefers-contrast: more` in
        // `$focus_border_color_hc` (`_drawing.scss:24-26`), `var(--accent-color)` at 80 %
        // (`_colors.scss:261`, `:308`), and `--accent-color` is the standalone colour of
        // `--accent-bg-color` whatever gives that accent, the default one too (`_colors.scss:166-170`).
        (
            "high contrast without an accent",
            with(|d| d.high_contrast = true),
            &["defaults.focus_ring_color", "input.focus_border_color"],
        ),
        (
            "an accent out of range",
            with(|d| d.accent_rgb = Some((1.5, 0.0, 0.0))),
            &[],
        ),
        (
            "reduced motion",
            with(|d| d.reduce_motion = Some(true)),
            &[],
        ),
        (
            "a text scale on Wayland",
            with(|d| {
                d.text_scaling_factor = Some(1.5);
                d.session_type = Some("wayland".to_owned());
            }),
            &[],
        ),
        (
            "a text scale on X11",
            with(|d| {
                d.text_scaling_factor = Some(1.5);
                d.session_type = Some("x11".to_owned());
            }),
            &[],
        ),
        (
            "Adwaita Sans 14",
            with(|d| d.font_name = Some("Adwaita Sans 14".to_owned())),
            &[],
        ),
        (
            "Adwaita Sans 14px",
            with(|d| d.font_name = Some("Adwaita Sans 14px".to_owned())),
            &[],
        ),
        (
            "Inter Bold 12",
            with(|d| d.font_name = Some("Inter Bold 12".to_owned())),
            &[],
        ),
        (
            "Cantarell Bold Italic 11",
            with(|d| d.font_name = Some("Cantarell Bold Italic 11".to_owned())),
            &[],
        ),
        // Listed: the Pango parser's difference (`pango_parser_table_against_072`): Pango stores 10.3
        // points as 10547/1024 (10.299805), its table's size for `Sans 10.3`, where 0.7.2 parses 10.3;
        // every size taken from the UI font differs by that much.
        (
            "a size off Pango's grid",
            with(|d| d.font_name = Some("Sans 10.3".to_owned())),
            UI_FONT_SIZE_KEYS,
        ),
        (
            "a font without a size",
            with(|d| d.font_name = Some("Adwaita Sans".to_owned())),
            &[],
        ),
        (
            "no fonts",
            with(|d| {
                d.font_name = None;
                d.monospace_font_name = None;
                d.titlebar_font = None;
            }),
            &[],
        ),
        (
            "no overlay scroll bars",
            with(|d| d.overlay_scrolling = Some(false)),
            &[],
        ),
        (
            "a blink cycle and a time-out",
            with(|d| {
                d.cursor_blink_time = Some(1000);
                d.cursor_blink_timeout = Some(5);
            }),
            &[],
        ),
        (
            "a caret that does not blink",
            with(|d| {
                d.cursor_blink = Some(false);
                d.cursor_blink_time = Some(1200);
            }),
            &[],
        ),
        (
            "another icon theme",
            with(|d| d.icon_theme = Some("Papirus".to_owned())),
            &[],
        ),
    ]
}

/// Reader parity of the adwaita builder: for every case of [`gnome_cases`], the 0.7.2 reader path
/// ([`old_gnome_reader`]) against `Theme::preset(Preset::Adwaita)` given the case's `SystemOverrides` and
/// resolved for its context (its motion level and text scale at their defaults, as 0.7.2 applies
/// neither), over the table of homes, the snapshot's context compared. Over every row of the ported
/// groups: the rows of the case's listed differences differ, every other row is equal or skipped for its
/// named reason (`checkbox.background_color` and `menu.min_row_height`, as in the model parity), the
/// four `accessibility.*` rows compared and equal; the resolved theme's name is the reader's.
#[cfg(all(target_os = "linux", feature = "portal"))]
#[test]
fn reader_parity_gnome() -> TestResult {
    use crate::next::reader::portal::snapshot_from_gnome_data;

    let table: Table = toml::from_str(HOMES)?;
    let skip = |key: &str| {
        (key == "checkbox.background_color" || key == "menu.min_row_height")
            .then_some(compare::Skip::OtherKeyStated)
    };
    for (name, data, listed) in gnome_cases() {
        let (old, unresolved, old_name) = old_gnome_reader(&data)?;
        let snapshot = snapshot_from_gnome_data(&data);
        let [rows, compared, ..] = GNOME_READER_ROWS;
        let expected = ReaderExpected {
            totals: [
                rows,
                compared,
                compared.saturating_sub(listed.len()),
                listed.len(),
                rows.saturating_sub(compared),
            ],
            skip,
            listed,
        };
        assert_reader_parity(
            name,
            &old,
            &unresolved,
            &old_name,
            &snapshot,
            &table,
            &expected,
        )?;
    }
    Ok(())
}

/// The keys of the rows whose values the UI font's size gives on adwaita, in the table's order: the body's
/// and every widget font's size and defined size, the text-scale entries' sizes and line heights, the
/// dialog title's size and defined size.
#[cfg(all(target_os = "linux", feature = "portal"))]
const UI_FONT_SIZE_KEYS: &[&str] = &[
    "defaults.font.size",
    "defaults.font.defined_size",
    "text_scale.caption.size",
    "text_scale.caption.line_height",
    "text_scale.section_heading.size",
    "text_scale.section_heading.line_height",
    "text_scale.dialog_title.size",
    "text_scale.dialog_title.line_height",
    "text_scale.display.size",
    "text_scale.display.line_height",
    "button.font.size",
    "button.font.defined_size",
    "checkbox.font.size",
    "checkbox.font.defined_size",
    "list.item_font.size",
    "list.item_font.defined_size",
    "list.header_font.size",
    "list.header_font.defined_size",
    "tooltip.font.size",
    "tooltip.font.defined_size",
    "input.font.size",
    "input.font.defined_size",
    "link.font.size",
    "link.font.defined_size",
    "menu.font.size",
    "menu.font.defined_size",
    "tab.font.size",
    "tab.font.defined_size",
    "sidebar.font.size",
    "sidebar.font.defined_size",
    "toolbar.font.size",
    "toolbar.font.defined_size",
    "status_bar.font.size",
    "status_bar.font.defined_size",
    "popover.font.size",
    "popover.font.defined_size",
    "dialog.title_font.size",
    "dialog.title_font.defined_size",
    "dialog.body_font.size",
    "dialog.body_font.defined_size",
    "combo_box.font.size",
    "combo_box.font.defined_size",
    "segmented_control.font.size",
    "segmented_control.font.defined_size",
    "expander.font.size",
    "expander.font.defined_size",
];

/// The rows of the ported groups on every GNOME case and the rows of them compared: all but
/// `checkbox.background_color` and `menu.min_row_height`, skipped.
#[cfg(all(target_os = "linux", feature = "portal"))]
const GNOME_READER_ROWS: [usize; 2] = [563, 561];

// ---------------------------------------------------------------------------------------------------------
// Reader parity (plan, section 1, rule 2) on Windows: the theme the 0.7.2 Windows reader path produces from
// the settings it read (`windows::reader_result`), and the windows-11 builder fed the same settings as
// `SystemOverrides` (`reader::windows::snapshot_from_settings`). On every host through the pure entry with
// recorded settings; the live comparison runs on Windows (`live_system_parity_windows`).
// ---------------------------------------------------------------------------------------------------------

/// The rows of the ported groups on every Windows case and the rows of them compared: all but
/// `checkbox.background_color`, skipped.
const WINDOWS_READER_ROWS: [usize; 2] = [563, 562];

/// The 0.7.2 Windows reader path on the settings `settings`, as the comparator reads it, and the name it
/// gives the theme: the reader's pure function (`windows::reader_result`), its theme merged over
/// `windows-11` as the pipeline merges a reader's result, resolved for the reader's scheme with the rows
/// written by hand and the reader's `accessibility` block ([`old_merged_reader`]); also the merged
/// theme's variant unresolved.
fn old_windows_reader(
    settings: &crate::windows::WindowsSettings,
) -> Result<(serde_json::Value, serde_json::Value, String), Box<dyn std::error::Error>> {
    let result = crate::windows::reader_result(settings);
    let reader = result
        .output
        .to_theme(&result.name, result.icon_set, &result.layout);
    let (old, unresolved) = old_merged_reader(
        "windows-11",
        &reader,
        result.output.mode(),
        &result.accessibility,
    )?;
    Ok((old, unresolved, result.name.to_string()))
}

/// The settings of 0.7.2's Windows reader tests (`windows.rs`, `light_reader`): light (a black
/// foreground), the accent `#0078D7`, no shade, no font, no icon size, no accessibility setting, no
/// system colour; the scrollbar and border metrics of the reader's test stub (`read_widget_sizing`) and
/// no caret blink time.
fn default_windows_settings() -> crate::windows::WindowsSettings {
    use crate::windows::{AllFonts, CARET_BLINK_FAILED, WidgetSizing, WindowsSettings};
    WindowsSettings {
        accent: crate::Rgba::rgb(0, 120, 215),
        foreground: crate::Rgba::rgb(0, 0, 0),
        accent_shades: [None; 6],
        fonts: AllFonts {
            msg: crate::model::FontSpec::default(),
            caption: crate::model::FontSpec::default(),
            menu: crate::model::FontSpec::default(),
            status: crate::model::FontSpec::default(),
        },
        sizing: WidgetSizing {
            scrollbar_width: 17.0,
            min_thumb_length: 17.0,
            border_width: 1.0,
        },
        icon_sizes: None,
        accessibility: None,
        sys_colors: None,
        caret_blink_time: CARET_BLINK_FAILED,
    }
}

/// The cases of Windows' reader parity: the data of 0.7.2's Windows reader tests (`windows.rs`: its
/// `SHADES`, `named_fonts`, `distinct_sys_colors`, `reader_with_high_contrast` and caret times), and,
/// where those tests use the preset's own value, another test input so that equality proves the builder
/// reads the setting; each named, with the keys of the rows that differ, each a listed difference.
pub(crate) fn windows_cases() -> Vec<(
    &'static str,
    crate::windows::WindowsSettings,
    &'static [&'static str],
)> {
    use crate::windows::{
        AccessibilityData, AllFonts, CARET_BLINK_NONE, SysColors, WidgetSizing, WindowsSettings,
    };
    const WHITE: crate::Rgba = crate::Rgba::rgb(255, 255, 255);
    // M2.FS4: the hovered on switch's fill is a listed difference of the table (`next_homes.toml`),
    // whose value is the preset's accent fill at 0.9, which every case's accent (and high contrast's
    // button face) replaces.
    const ACCENT: &[&str] = &["switch.hover_checked_background"];
    const HIGH_CONTRAST: &[&str] = &[
        "button.hover_border_color",
        "checkbox.hover_border_color",
        "switch.hover_checked_background",
        "expander.hover_background",
    ];
    let shades = || [1, 2, 3, 4, 5, 6].map(|b| Some(crate::Rgba::rgb(0, 0, b)));
    let named_fonts = || {
        let spec = |size, weight| crate::model::FontSpec {
            family: Some("Segoe UI".into()),
            size: Some(crate::model::font::FontSize::Pt(size)),
            weight: Some(weight),
            ..Default::default()
        };
        AllFonts {
            msg: spec(9.0, 400),
            caption: spec(9.0, 700),
            menu: spec(9.0, 400),
            status: spec(8.0, 400),
        }
    };
    let high_contrast = |foreground| WindowsSettings {
        foreground,
        accent_shades: [Some(crate::Rgba::rgb(0, 90, 170)); 6],
        accessibility: Some(AccessibilityData {
            text_scaling_factor: None,
            high_contrast: Some(true),
            reduce_motion: None,
            advanced_effects_enabled: None,
        }),
        sys_colors: Some(SysColors {
            window_bg: crate::Rgba::rgb(1, 1, 1),
            window_text: crate::Rgba::rgb(2, 2, 2),
            btn_face: crate::Rgba::rgb(3, 3, 3),
            btn_text: crate::Rgba::rgb(4, 4, 4),
            highlight: crate::Rgba::rgb(5, 5, 5),
            highlight_text: crate::Rgba::rgb(6, 6, 6),
            hotlight: crate::Rgba::rgb(8, 8, 8),
            gray_text: crate::Rgba::rgb(7, 7, 7),
        }),
        ..default_windows_settings()
    };
    let base = default_windows_settings;
    vec![
        ("light, no shade", base(), ACCENT),
        (
            "dark, no shade",
            WindowsSettings {
                foreground: WHITE,
                ..base()
            },
            ACCENT,
        ),
        (
            "light, the shades",
            WindowsSettings {
                accent_shades: shades(),
                ..base()
            },
            ACCENT,
        ),
        (
            "dark, the shades",
            WindowsSettings {
                foreground: WHITE,
                accent_shades: shades(),
                ..base()
            },
            ACCENT,
        ),
        (
            "light, AccentDark1 unread",
            WindowsSettings {
                accent_shades: {
                    let mut s = shades();
                    s[0] = None;
                    s
                },
                ..base()
            },
            ACCENT,
        ),
        (
            "dark, AccentLight2 and AccentLight3 unread",
            WindowsSettings {
                foreground: WHITE,
                accent_shades: {
                    let mut s = shades();
                    s[4] = None;
                    s[5] = None;
                    s
                },
                ..base()
            },
            ACCENT,
        ),
        (
            "the named fonts, light",
            WindowsSettings {
                fonts: named_fonts(),
                ..base()
            },
            ACCENT,
        ),
        (
            "the named fonts, dark",
            WindowsSettings {
                foreground: WHITE,
                fonts: named_fonts(),
                ..base()
            },
            ACCENT,
        ),
        (
            "the icon sizes",
            WindowsSettings {
                icon_sizes: Some((16.0, 32.0)),
                ..base()
            },
            ACCENT,
        ),
        (
            "other icon sizes and metrics",
            WindowsSettings {
                icon_sizes: Some((20.0, 40.0)),
                sizing: WidgetSizing {
                    scrollbar_width: 21.0,
                    min_thumb_length: 23.0,
                    border_width: 2.0,
                },
                ..base()
            },
            ACCENT,
        ),
        // M2.FS4: the button's hover and pressed outline, the check box's hover outline and the
        // expander header's fill are listed differences of the table (`next_homes.toml`), whose
        // values are the preset's colours, which high contrast replaces.
        (
            "high contrast, light",
            high_contrast(crate::Rgba::rgb(0, 0, 0)),
            HIGH_CONTRAST,
        ),
        ("high contrast, dark", high_contrast(WHITE), HIGH_CONTRAST),
        (
            "a caret blink time",
            WindowsSettings {
                caret_blink_time: 530,
                ..base()
            },
            ACCENT,
        ),
        (
            "another caret blink time",
            WindowsSettings {
                caret_blink_time: 1200,
                ..base()
            },
            ACCENT,
        ),
        (
            "a caret that does not blink",
            WindowsSettings {
                caret_blink_time: CARET_BLINK_NONE,
                ..base()
            },
            ACCENT,
        ),
        (
            "animations and transparency off, a text scale",
            WindowsSettings {
                accessibility: Some(AccessibilityData {
                    text_scaling_factor: Some(1.5),
                    high_contrast: Some(false),
                    reduce_motion: Some(true),
                    advanced_effects_enabled: Some(false),
                }),
                ..base()
            },
            ACCENT,
        ),
    ]
}

/// Reader parity of the windows-11 builder: for every case of [`windows_cases`], the 0.7.2 reader path
/// ([`old_windows_reader`]) against `Theme::preset(Preset::Windows11)` given the case's `SystemOverrides`
/// and resolved for its context (its motion level and text scale at their defaults, as 0.7.2 applies
/// neither), over the table of homes, the snapshot's context compared. Over every row of the ported
/// groups: the rows of the case's listed differences differ, every other row is equal or skipped for its
/// named reason (`checkbox.background_color`, as in the model parity), the four `accessibility.*` rows
/// compared and equal; the resolved theme's name is the reader's.
#[test]
fn reader_parity_windows() -> TestResult {
    use crate::next::reader::windows::snapshot_from_settings;

    let table: Table = toml::from_str(HOMES)?;
    for (name, settings, listed) in windows_cases() {
        let (old, unresolved, old_name) = old_windows_reader(&settings)?;
        let snapshot = snapshot_from_settings(&settings);
        assert_reader_parity(
            name,
            &old,
            &unresolved,
            &old_name,
            &snapshot,
            &table,
            &windows_reader_expected(listed),
        )?;
    }
    Ok(())
}

/// What Windows' reader parity expects of a case whose listed differences are `listed`: every other row of
/// the ported groups compared and equal but `checkbox.background_color`, skipped where
/// `checkbox.unchecked_background` is stated.
fn windows_reader_expected<'a>(listed: &'a [&'a str]) -> ReaderExpected<'a> {
    let [rows, compared] = WINDOWS_READER_ROWS;
    ReaderExpected {
        totals: [
            rows,
            compared,
            compared.saturating_sub(listed.len()),
            listed.len(),
            rows.saturating_sub(compared),
        ],
        skip: |key| (key == "checkbox.background_color").then_some(compare::Skip::OtherKeyStated),
        listed,
    }
}

/// The live check on Windows: the theme from the live system through the 0.7.2 path (the reader 0.7.2's
/// `from_system` chooses, its theme merged over `windows-11` as its pipeline merges it) and through the
/// 0.8.0 path (`Theme::from_system`), compared as [`reader_parity_windows`] compares a case's, with no
/// listed difference. It prints the comparator's report and the 0.8.0 reader's diagnostics, and fails
/// where the two differ. Runs on every Windows test run with the `windows` feature (the CI runners).
#[cfg(all(target_os = "windows", feature = "windows"))]
#[test]
fn live_system_parity_windows() -> TestResult {
    use crate::next::preset::{Preset, Theme as NextTheme};
    use crate::next::reader::SystemSnapshot;

    let table: Table = toml::from_str(HOMES)?;
    let selected = pollster::block_on(crate::pipeline::select_reader())
        .ok_or("0.7.2 chooses no reader on this system")?;
    if selected.preset != "windows-11" {
        return Err(format!(
            "0.7.2 chooses the preset {}, not windows-11",
            selected.preset
        )
        .into());
    }
    let result = pollster::block_on(selected.reader.read())?;
    let reader = result
        .output
        .to_theme(&result.name, result.icon_set, &result.layout);
    let (old, unresolved) = old_merged_reader(
        "windows-11",
        &reader,
        result.output.mode(),
        &result.accessibility,
    )?;
    let theme = NextTheme::from_system()?;
    println!("0.8.0 diagnostics: {:?}", theme.diagnostics());
    assert_eq!(theme.preset_id(), Preset::Windows11);
    let overrides = theme
        .system_overrides()
        .ok_or("the theme from the system has no system settings")?;
    let snapshot = SystemSnapshot::new(
        theme.preset_id(),
        theme.context().clone(),
        overrides.clone(),
    );
    assert_reader_parity(
        "the live system",
        &old,
        &unresolved,
        &result.name,
        &snapshot,
        &table,
        &windows_reader_expected(&[]),
    )?;
    println!("the live system: equal through the 0.7.2 and the 0.8.0 path");
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------
// Reader parity (plan, section 1, rule 2) on macOS: the theme the 0.7.2 macOS reader path produces from the
// settings it read (`macos::reader_result`), and the macOS builders fed the same settings as
// `SystemOverrides` (`reader::macos::snapshot_from_settings`). On every host through the pure entry with
// the data of 0.7.2's macOS reader tests and inputs built from the reader's list of reads; the live
// comparison runs on macOS (`live_system_parity_macos`).
// ---------------------------------------------------------------------------------------------------------

/// The rows of the ported groups on every case of `macos-monterey` and the rows of them compared: all but
/// `checkbox.background_color` and `menu.min_row_height`, skipped as in the model parity.
const MACOS_MONTEREY_READER_ROWS: [usize; 2] = [563, 561];

/// The rows of the ported groups on every case of `macos-tahoe` and the rows of them compared: all but
/// `checkbox.background_color`, skipped as in the model parity.
const MACOS_TAHOE_READER_ROWS: [usize; 2] = [563, 562];

/// The 0.7.2 macOS reader path on the settings `settings`, as the comparator reads it, and the name it
/// gives the theme: the reader's pure function (`macos::reader_result`), its theme merged over the preset
/// `preset` as the pipeline merges a reader's result (the running version's generation, or the preset
/// `from_system_with_preset` pins, `pipeline::pinned_pipeline`), resolved for the reader's scheme at the
/// reader's font DPI, 72 ([`MACOS_RESOLUTION_CONTEXT`]), with the rows written by hand and the reader's
/// `accessibility` block ([`old_merged_reader_in`]); also the merged theme's variant unresolved.
fn old_macos_reader(
    settings: &crate::macos::MacosSettings,
    preset: crate::next::preset::Preset,
) -> Result<(serde_json::Value, serde_json::Value, String), Box<dyn std::error::Error>> {
    let result = crate::macos::reader_result(settings);
    let reader = result
        .output
        .to_theme(&result.name, result.icon_set, &result.layout);
    let (old, unresolved) = old_merged_reader_in(
        preset.as_str(),
        &reader,
        result.output.mode(),
        &result.accessibility,
        &MACOS_RESOLUTION_CONTEXT,
    )?;
    Ok((old, unresolved, result.name.to_string()))
}

/// A font of 0.7.2's macOS reader tests' shape (`macos.rs`, `sample_widget_fonts`): a family, a size in
/// points and a weight where given.
fn macos_font(family: &str, size: f32, weight: Option<u16>) -> crate::model::FontSpec {
    crate::model::FontSpec {
        family: Some(family.into()),
        size: Some(crate::model::font::FontSize::Pt(size)),
        weight,
        ..Default::default()
    }
}

/// The settings of 0.7.2's macOS reader tests (`macos.rs`: `sample_light_defaults`,
/// `sample_dark_defaults`, `sample_widget_fonts`): under Aqua the accent `#007AFF`, the window `#F6F6F6`,
/// black text, a white view and the separator `#C8C8C8`; under Dark Aqua the accent `#0A84FF`, the window
/// `#1E1E1E`, white text, the view `#2C2C2E` and the separator `#38383A`; no other colour; SF Pro and SF
/// Mono at 13 pt without a weight, the menu's SF Pro 14 pt at 400, the tooltip's 11 pt at 400 and the
/// title bar's 13 pt at 700; light, on macOS 15.7.9 (the version 0.7.2's tests give `macos-monterey`);
/// no scroller and no accessibility flag read.
fn default_macos_settings() -> crate::macos::MacosSettings {
    use crate::macos::{AppearanceColors, MacosSettings, WidgetFontData};
    MacosSettings {
        light: Some(AppearanceColors {
            control_accent: Some(crate::Rgba::rgb(0, 122, 255)),
            window_background: Some(crate::Rgba::rgb(246, 246, 246)),
            label: Some(crate::Rgba::rgb(0, 0, 0)),
            control_background: Some(crate::Rgba::rgb(255, 255, 255)),
            separator: Some(crate::Rgba::rgb(200, 200, 200)),
            ..AppearanceColors::default()
        }),
        dark: Some(AppearanceColors {
            control_accent: Some(crate::Rgba::rgb(10, 132, 255)),
            window_background: Some(crate::Rgba::rgb(30, 30, 30)),
            label: Some(crate::Rgba::rgb(255, 255, 255)),
            control_background: Some(crate::Rgba::rgb(44, 44, 46)),
            separator: Some(crate::Rgba::rgb(56, 56, 58)),
            ..AppearanceColors::default()
        }),
        font: macos_font("SF Pro", 13.0, None),
        mono_font: macos_font("SF Mono", 13.0, None),
        widget_fonts: WidgetFontData {
            menu_font: macos_font("SF Pro", 14.0, Some(400)),
            tooltip_font: macos_font("SF Pro", 11.0, Some(400)),
            title_bar_font: macos_font("SF Pro", 13.0, Some(700)),
        },
        scrollbar: None,
        reduce_motion: None,
        increase_contrast: None,
        reduce_transparency: None,
        is_dark: false,
        version: crate::pipeline::OsVersion::new(15, 7, 9),
    }
}

/// A distinct colour for each of the 24 colours the reader reads (spec §8, "The macOS part",
/// *Colours*), `(n, n + offset, 2n)` for the `n`th in the order of `macos::AppearanceColors`, as 0.7.2's
/// Windows reader tests make a distinct value per system colour; no 0.7.2 test reads these colours, so
/// they are test inputs, which only the live comparison on macOS replaces with AppKit's.
fn every_macos_colour(offset: u8) -> crate::macos::AppearanceColors {
    let c = |n: u8| {
        Some(crate::Rgba::rgb(
            n,
            n.saturating_add(offset),
            n.saturating_mul(2),
        ))
    };
    crate::macos::AppearanceColors {
        label: c(1),
        control_accent: c(2),
        window_background: c(3),
        control_background: c(4),
        separator: c(5),
        secondary_label: c(6),
        alternate_selected_control_text: c(7),
        system_red: c(8),
        system_orange: c(9),
        system_green: c(10),
        system_blue: c(11),
        selected_content_background: c(12),
        unemphasized_selected_content_background: c(13),
        selected_text_background: c(14),
        selected_text: c(15),
        link: c(16),
        keyboard_focus_indicator: c(17),
        disabled_control_text: c(18),
        placeholder_text: c(19),
        text_insertion_point: c(20),
        alternate_row: c(21),
        header_text: c(22),
        grid: c(23),
        window_frame_text: c(24),
    }
}

/// The cases of macOS' reader parity: the data of 0.7.2's macOS reader tests (`macos.rs`: the sample
/// defaults and fonts, the versions of its generation tests, the custom accent `#E0383E`, the scroller
/// widths its scroll bar test cites), and, for the reads no 0.7.2 test gives a value, a distinct colour
/// per read ([`every_macos_colour`]) and other fonts, so that equality proves the builder reads the
/// setting; each named, with the preset it is pinned to (else the running version's generation) and the
/// keys of the rows that differ, each a listed difference.
pub(crate) fn macos_cases() -> Vec<(
    &'static str,
    crate::macos::MacosSettings,
    Option<crate::next::preset::Preset>,
    &'static [&'static str],
)> {
    use crate::macos::{AppearanceColors, MacosSettings, WidgetFontData};
    use crate::next::preset::Preset;
    use crate::pipeline::OsVersion;
    // D-R4 (spec §8, "The macOS part", *Derived by the macOS builders*): 0.7.2 takes the disabled on
    // switch's alpha from the running version, the builder from its own generation.
    const PINNED: &[&str] = &["switch.disabled_checked_background"];
    // M2.FS3 (spec §8, "The macOS part", *Derived by the macOS builders*): `macos-tahoe`'s checked
    // button fill is the accent in force mixed by the measured weight; the table's listed difference
    // gives it under the default accent alone.
    const TAHOE_ACCENT: &[&str] = &["button.checked_background"];
    const PINNED_TAHOE_ACCENT: &[&str] = &[
        "button.checked_background",
        "switch.disabled_checked_background",
    ];
    let base = default_macos_settings;
    let every = || MacosSettings {
        light: Some(every_macos_colour(100)),
        dark: Some(every_macos_colour(200)),
        ..base()
    };
    let scroller = |overlay: bool, width: f32| {
        Some(crate::ScrollbarTheme {
            groove_width: Some(width),
            overlay_mode: Some(overlay),
            ..Default::default()
        })
    };
    let red = Some(crate::Rgba::rgb(224, 56, 62));
    let tahoe = OsVersion::new(26, 6, 2);
    vec![
        ("0.7.2's sample data, light", base(), None, &[]),
        (
            "0.7.2's sample data, dark",
            MacosSettings {
                is_dark: true,
                ..base()
            },
            None,
            &[],
        ),
        ("every colour, light", every(), None, &[]),
        (
            "every colour, dark",
            MacosSettings {
                is_dark: true,
                ..every()
            },
            None,
            &[],
        ),
        (
            "macOS 26, every colour, light",
            MacosSettings {
                version: tahoe,
                ..every()
            },
            None,
            TAHOE_ACCENT,
        ),
        (
            "macOS 26, every colour, dark",
            MacosSettings {
                version: tahoe,
                is_dark: true,
                ..every()
            },
            None,
            TAHOE_ACCENT,
        ),
        (
            "macOS 13, no insertion point colour",
            MacosSettings {
                light: Some(AppearanceColors {
                    text_insertion_point: None,
                    ..every_macos_colour(100)
                }),
                version: OsVersion::new(13, 7, 8),
                ..every()
            },
            None,
            &[],
        ),
        (
            "a custom accent, light",
            MacosSettings {
                light: Some(AppearanceColors {
                    control_accent: red,
                    ..AppearanceColors::default()
                }),
                ..base()
            },
            None,
            &[],
        ),
        (
            "macOS 26, a custom accent, dark",
            MacosSettings {
                dark: Some(AppearanceColors {
                    control_accent: red,
                    ..AppearanceColors::default()
                }),
                is_dark: true,
                version: tahoe,
                ..base()
            },
            None,
            TAHOE_ACCENT,
        ),
        (
            "other fonts",
            MacosSettings {
                font: macos_font("Helvetica Neue", 12.0, Some(300)),
                mono_font: macos_font("Menlo", 11.0, Some(500)),
                widget_fonts: WidgetFontData {
                    menu_font: macos_font("Avenir", 15.0, Some(600)),
                    tooltip_font: macos_font("Optima", 10.0, Some(200)),
                    title_bar_font: macos_font("Futura", 14.0, Some(800)),
                },
                ..base()
            },
            None,
            &[],
        ),
        (
            "fonts without a family or a weight",
            MacosSettings {
                font: crate::model::FontSpec {
                    size: Some(crate::model::font::FontSize::Pt(12.0)),
                    ..Default::default()
                },
                mono_font: crate::model::FontSpec::default(),
                widget_fonts: WidgetFontData {
                    menu_font: crate::model::FontSpec::default(),
                    tooltip_font: crate::model::FontSpec::default(),
                    title_bar_font: crate::model::FontSpec::default(),
                },
                ..base()
            },
            None,
            &[],
        ),
        (
            "the overlay scroller",
            MacosSettings {
                scrollbar: scroller(true, 16.0),
                ..base()
            },
            None,
            &[],
        ),
        (
            "the legacy scroller",
            MacosSettings {
                scrollbar: scroller(false, 15.0),
                ..base()
            },
            None,
            &[],
        ),
        (
            "macOS 26's legacy scroller",
            MacosSettings {
                scrollbar: scroller(false, 17.0),
                version: tahoe,
                ..base()
            },
            None,
            &[],
        ),
        (
            "the accessibility flags",
            MacosSettings {
                reduce_motion: Some(true),
                increase_contrast: Some(true),
                reduce_transparency: Some(true),
                ..every()
            },
            None,
            &[],
        ),
        (
            "Aqua alone",
            MacosSettings {
                dark: None,
                ..every()
            },
            None,
            &[],
        ),
        (
            "Aqua alone, dark",
            MacosSettings {
                dark: None,
                is_dark: true,
                ..base()
            },
            None,
            &[],
        ),
        (
            "macOS 26 pinned to macos-monterey",
            MacosSettings {
                version: tahoe,
                ..every()
            },
            Some(Preset::MacosMonterey),
            PINNED,
        ),
        (
            "macOS 15 pinned to macos-tahoe, dark",
            MacosSettings {
                is_dark: true,
                ..every()
            },
            Some(Preset::MacosTahoe),
            PINNED_TAHOE_ACCENT,
        ),
    ]
}

/// Reader parity of the macOS builders: for every case of [`macos_cases`], the 0.7.2 reader path
/// ([`old_macos_reader`], over the case's preset) against `Theme::preset` of that preset given the case's
/// `SystemOverrides` and resolved for its context (its motion level and text scale at their defaults, as
/// 0.7.2 applies neither), over the table of homes, the snapshot's context compared. Over every row of
/// the ported groups: the rows of the case's listed differences differ, every other row is equal or
/// skipped for its named reason (as in the model parity), the four `accessibility.*` rows compared and
/// equal; the resolved theme's name is the reader's.
#[test]
fn reader_parity_macos() -> TestResult {
    use crate::next::reader::macos::snapshot_from_settings;

    let table: Table = toml::from_str(HOMES)?;
    for (name, settings, pinned, listed) in macos_cases() {
        let mut snapshot = snapshot_from_settings(&settings);
        if let Some(preset) = pinned {
            snapshot.preset = preset;
        }
        let (old, unresolved, old_name) = old_macos_reader(&settings, snapshot.preset)?;
        // M2.FS4: macos-monterey's hovered push button keeps its outline in the separator the
        // reader states, where the table's listed difference gives the preset's own line.
        let listed: Vec<&str> = (snapshot.preset == crate::next::preset::Preset::MacosMonterey
            && !MACOS_CASES_PRESET_BORDER.contains(&name))
        .then_some(MONTEREY_HOVER_BORDER)
        .into_iter()
        .chain(listed.iter().copied())
        .collect();
        assert_reader_parity(
            name,
            &old,
            &unresolved,
            &old_name,
            &snapshot,
            &table,
            &macos_reader_expected(snapshot.preset, &listed),
        )?;
    }
    Ok(())
}

/// M2.FS4: the row of a hovered push button's outline on macos-monterey, a listed difference of the table
/// whose value is the preset's default line; a reader's separator replaces that line.
const MONTEREY_HOVER_BORDER: &str = "button.hover_border_color";

/// The macOS reader cases on macos-monterey whose scheme has no separator of the reader's, so the
/// preset's own line stays: [`MONTEREY_HOVER_BORDER`] is listed on every other one.
const MACOS_CASES_PRESET_BORDER: &[&str] = &["a custom accent, light", "Aqua alone, dark"];

/// What macOS' reader parity expects of a case on `preset` whose listed differences are `listed`: every
/// other row of the ported groups compared and equal but the rows [`macos_expected_skip`] skips with a
/// snapshot (`checkbox.background_color`, and on `macos-monterey` `menu.min_row_height`).
fn macos_reader_expected<'a>(
    preset: crate::next::preset::Preset,
    listed: &'a [&'a str],
) -> ReaderExpected<'a> {
    use crate::next::preset::Preset;
    let monterey = preset == Preset::MacosMonterey;
    let [rows, compared] = if monterey {
        MACOS_MONTEREY_READER_ROWS
    } else {
        MACOS_TAHOE_READER_ROWS
    };
    ReaderExpected {
        totals: [
            rows,
            compared,
            compared.saturating_sub(listed.len()),
            listed.len(),
            rows.saturating_sub(compared),
        ],
        skip: if monterey {
            |key| {
                macos_expected_skip(crate::next::preset::Preset::MacosMonterey, key)
                    .filter(|_| !key.starts_with("accessibility."))
            }
        } else {
            |key| {
                macos_expected_skip(crate::next::preset::Preset::MacosTahoe, key)
                    .filter(|_| !key.starts_with("accessibility."))
            }
        },
        listed,
    }
}

/// The live check on macOS: the theme from the live system through the 0.7.2 path (the reader 0.7.2's
/// `from_system` chooses, its theme merged over the running version's preset as its pipeline merges it,
/// at the reader's font DPI) and through the 0.8.0 path (`Theme::from_system`), compared as
/// [`reader_parity_macos`] compares a case's, with no listed difference. It prints the comparator's
/// report and the 0.8.0 reader's diagnostics, and fails where the two differ. Runs on every macOS test
/// run with the `macos` feature (the CI runners).
#[cfg(all(target_os = "macos", feature = "macos"))]
#[test]
fn live_system_parity_macos() -> TestResult {
    use crate::next::preset::Theme as NextTheme;
    use crate::next::reader::SystemSnapshot;

    let table: Table = toml::from_str(HOMES)?;
    let selected = pollster::block_on(crate::pipeline::select_reader())
        .ok_or("0.7.2 chooses no reader on this system")?;
    let result = pollster::block_on(selected.reader.read())?;
    let reader = result
        .output
        .to_theme(&result.name, result.icon_set, &result.layout);
    let (old, unresolved) = old_merged_reader_in(
        selected.preset,
        &reader,
        result.output.mode(),
        &result.accessibility,
        &MACOS_RESOLUTION_CONTEXT,
    )?;
    let theme = NextTheme::from_system()?;
    println!("0.8.0 diagnostics: {:?}", theme.diagnostics());
    assert_eq!(theme.preset_id().as_str(), selected.preset);
    let overrides = theme
        .system_overrides()
        .ok_or("the theme from the system has no system settings")?;
    let snapshot = SystemSnapshot::new(
        theme.preset_id(),
        theme.context().clone(),
        overrides.clone(),
    );
    assert_reader_parity(
        "the live system",
        &old,
        &unresolved,
        &result.name,
        &snapshot,
        &table,
        &macos_reader_expected(snapshot.preset, &[]),
    )?;
    println!("the live system: equal through the 0.7.2 and the 0.8.0 path");
    Ok(())
}
