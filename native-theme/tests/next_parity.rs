//! Model parity of the 0.8.0 work (plan, section 1, rule 2) on the first slice: the globals and 0.7.2's
//! `button`, `checkbox`, `list`, `scrollbar` and `tooltip`, on `kde-breeze`.
//!
//! This half checks the table of homes, `tests/next_homes.toml`, against both models: it resolves 0.7.2's
//! `kde-breeze` (light and dark) with 0.7.2's public API, takes the keys from the JSON serialisation of the
//! resolved theme, and asserts that every key of the slice has exactly one row, that no row names a key that
//! does not exist (rows written by hand excepted), and that every home parses and names only what the spec
//! declares (the vocabulary below, written from spec §§2-5). Every problem found is listed before the test
//! fails.
//!
//! Scaffolding of the 0.8.0 work: deleted at the 0.8.0 cut.

use std::collections::{BTreeMap, BTreeSet};

use native_theme::resolve::ResolutionContext;
use native_theme::theme::{ColorMode, Theme};
use serde::Deserialize;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// The table of homes.
const HOMES: &str = include_str!("next_homes.toml");

/// The preset of the slice.
const PRESET: &str = "kde-breeze";

/// The first segments of the slice's keys in 0.7.2's resolved theme: its globals and the five widgets.
const SLICE_GROUPS: &[&str] = &[
    "defaults",
    "text_scale",
    "button",
    "checkbox",
    "list",
    "scrollbar",
    "tooltip",
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
    #[serde(default)]
    hand: bool,
    difference_presets: Option<Vec<String>>,
    difference_reason: Option<String>,
    difference_citation: Option<String>,
    difference_values: Option<BTreeMap<String, toml::Value>>,
    // Read by people and by the second half; this half only requires it to be text.
    #[allow(dead_code)]
    note: Option<String>,
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
/// and `segmented_control`.
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
        metrics: &["origin"],
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
            "label_gap",
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
}

/// `home` parsed against the vocabulary, or what is wrong with it.
fn parse_home(home: &str, globals: &BTreeSet<String>) -> Result<Home, String> {
    if globals.contains(home) {
        return Ok(Home::Global);
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
    let properties = match kind {
        Kind::P => PART_PROPERTIES,
        Kind::T => TEXT_PROPERTIES,
        Kind::I => ICON_PROPERTIES,
    };
    if !properties.contains(&property) {
        return Err(format!(
            "`{home}`: `{property}` is no property of a {kind:?} part"
        ));
    }
    Ok(Home::Part)
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
        // and no other, the value 0.8.0 has at the homes: "unstated" or a single value, not a table or array.
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
                for (preset, value) in values {
                    if matches!(value, toml::Value::Table(_) | toml::Value::Array(_)) {
                        found.push(format!(
                            "`{key}`: difference_values for `{preset}` is not a single value"
                        ));
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
    ] {
        assert!(parse_home(good, &globals).is_ok(), "{good}");
    }
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
    ] {
        assert!(parse_home(bad, &globals).is_err(), "{bad}");
    }

    let keys: BTreeSet<String> = ["button.min_width", "button.min_height"]
        .into_iter()
        .map(String::from)
        .collect();
    let table: Table = toml::from_str(
        r#"
        [[row]]
        key = "button.min_width"
        homes = ["button.body(NONE).min_size[0]"]

        [[row]]
        key = "button.gone"
        homes = ["button.body(NONE).min_size[1]"]

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
        "#,
    )?;
    let found = problems(&table, &keys);
    for expected in [
        "`button.min_height`: no row",
        "`button.gone`: no such key",
        "`Resolved.mode`: no row written by hand",
        "`layout.section_gap`: no row written by hand",
        "`accessibility.reduce_motion`: no row written by hand",
        "`layout.gone`: written by hand, but no field",
        "`button.min_width`: difference_values must name exactly the difference_presets",
    ] {
        assert!(
            found.iter().any(|p| p.contains(expected)),
            "{expected}: {found:?}"
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------
// SECOND HALF — not written yet (the next task, after the kde-breeze builder exists): for each scheme, resolve
// the 0.8.0 `kde-breeze` and compare, row by row, the value of each key in 0.7.2's serialisation (or of the
// hand-written field) with the value at each of its homes, applying `homes_by_preset`, `if_null`,
// `only_if_null`, `home_fallback` and the conversions the rows' notes name, and skipping only the listed
// differences of the preset. No test stands here until then.
// ---------------------------------------------------------------------------------------------------------
