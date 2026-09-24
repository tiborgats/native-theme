//! Gate: the native themes state what their platform documents
//! (spec `docs/archive/todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md` §1.4–§1.6).
//!
//! One row per (platform, widget) gives every padding side, and for the
//! toolbar its `bar_height` and `item_gap`, for the menu and the list their
//! `row_height`, and for the combobox its `arrow_area_width` where the
//! platform gives one number, as `Some(v)` where `docs/platform-facts.md`
//! documents a value for that platform and `None` where it does not, or where
//! a ruling leaves it unstated. Each row cites the platform-facts lines it
//! reads.
//!
//! Presets that platform-facts does not cover (the colour schemes, and the
//! `material` and `ios` platform presets, which no platform-facts column
//! documents) state none of these sizes: nothing cites a source for them, so
//! the toolkit's own stand.
//!
//! A second table, one row per (platform, field), gives the text scale's
//! sizes and weights, the dialog title font's size and weight, the slider's
//! `track_height` and `thumb_diameter`, and the progress bar's
//! `track_height`. A cell that leaves the field to inheritance
//! ("← `defaults.font`", "(none)") is checked as not stated.
//!
//! Every row is checked in both variants against two resolutions:
//!
//! - **static**: the platform's full preset;
//! - **live**: the full preset, the `-live` preset merged over it, then the
//!   reader's size constants merged over that, in `pipeline.rs`'s order.
//!
//! The KDE and Windows readers return one variant (`ReaderOutput::Single`),
//! so the real pipeline applies their constants to the active variant only
//! and takes the inactive variant from the full preset, which the static
//! resolution covers. The live resolution here applies the constants to each
//! variant in turn, as the pipeline does when that variant is the active one.
//! The macOS reader fills both variants. GNOME's reader states no sizes.
//!
//! This lives inside the crate because the reader constants
//! (`kde_metrics::populate_widget_sizing`, `windows::winui3_widget_sizing`,
//! `macos::macos_widget_defaults`) are `pub(crate)`.

use crate::model::border::ResolvedPadding;
use crate::resolve::ResolutionContext;
use crate::{ColorMode, ResolvedTheme, Theme, ThemeMode};

const PLATFORM_FACTS: &str = include_str!("../../../docs/platform-facts.md");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Platform {
    Kde,
    Gnome,
    Macos,
    Windows,
}

impl Platform {
    const ALL: [Platform; 4] = [
        Platform::Kde,
        Platform::Gnome,
        Platform::Macos,
        Platform::Windows,
    ];

    fn preset(self) -> &'static str {
        match self {
            Platform::Kde => "kde-breeze",
            Platform::Gnome => "adwaita",
            Platform::Macos => "macos-sonoma",
            Platform::Windows => "windows-11",
        }
    }

    fn live_preset(self) -> &'static str {
        match self {
            Platform::Kde => "kde-breeze-live",
            Platform::Gnome => "adwaita-live",
            Platform::Macos => "macos-sonoma-live",
            Platform::Windows => "windows-11-live",
        }
    }

    /// The reader's size constants as the sparse variant the reader returns.
    fn reader_constants(self) -> Option<ThemeMode> {
        match self {
            Platform::Kde => {
                let mut v = ThemeMode::default();
                crate::kde_metrics::populate_widget_sizing(&mut v);
                Some(v)
            }
            Platform::Gnome => None,
            Platform::Macos => Some(crate::macos::macos_widget_defaults()),
            Platform::Windows => {
                let mut v = ThemeMode::default();
                crate::windows::winui3_widget_sizing(&mut v);
                Some(v)
            }
        }
    }
}

/// Top, right, bottom, left.
type Sides = [Option<f32>; 4];

const NONE: Sides = [None; 4];

const fn all(v: f32) -> Sides {
    [Some(v); 4]
}

/// Equal top/bottom and equal left/right.
const fn axes(vertical: f32, horizontal: f32) -> Sides {
    [
        Some(vertical),
        Some(horizontal),
        Some(vertical),
        Some(horizontal),
    ]
}

const fn trbl(t: f32, r: f32, b: f32, l: f32) -> Sides {
    [Some(t), Some(r), Some(b), Some(l)]
}

struct Row {
    platform: Platform,
    widget: &'static str,
    padding: Sides,
    /// platform-facts.md lines this row reads: the padding rows, or the
    /// section heading where the section has no padding row.
    lines: &'static [usize],
    /// How the cells were read.
    note: &'static str,
    /// Further fields: (field, expected, platform-facts.md line).
    extra: &'static [(&'static str, Option<f32>, usize)],
}

impl Row {
    const fn with(self, extra: &'static [(&'static str, Option<f32>, usize)]) -> Row {
        Row { extra, ..self }
    }
}

const fn row(
    platform: Platform,
    widget: &'static str,
    padding: Sides,
    lines: &'static [usize],
    note: &'static str,
) -> Row {
    Row {
        platform,
        widget,
        padding,
        lines,
        note,
        extra: &[],
    }
}

use Platform::{Gnome, Kde, Macos, Windows};

#[rustfmt::skip]
const ROWS: &[Row] = &[
    // --- KDE (kde-breeze, kde-breeze-live, kde_metrics) ---
    row(Kde, "window", NONE, &[1157, 1158], "pointer to §2.20 layout margins"),
    row(Kde, "button", all(6.0), &[1171, 1172], "Button_MarginWidth = 6, both axes"),
    row(Kde, "input", axes(3.0, 6.0), &[1196, 1197], "LineEdit_FrameWidth = 6; 3 (measured)"),
    row(Kde, "checkbox", NONE, &[1216, 1217], "(none)"),
    row(Kde, "menu", all(4.0), &[1235, 1236], "MenuItem_MarginWidth = 4; MenuItem_MarginHeight = 4")
        .with(&[("row_height", None, 1234)]),
    row(Kde, "tooltip", all(3.0), &[1257, 1258], "ToolTip_FrameWidth = 3"),
    row(Kde, "progress_bar", NONE, &[1297], "§2.10 has no padding row"),
    row(Kde, "tab", axes(4.0, 8.0), &[1318, 1319], "TabBar_TabMarginWidth = 8; TabBar_TabMarginHeight = 4"),
    row(Kde, "sidebar", NONE, &[1327], "§2.12 has no padding row"),
    Row {
        platform: Kde,
        widget: "toolbar",
        padding: all(6.0),
        lines: &[1353, 1354],
        note: "ToolBar_ItemMargin = 6 on all four sides (qtoolbarlayout.cpp:87-89)",
        extra: &[("bar_height", None, 1351), ("item_gap", Some(0.0), 1352)],
    },
    row(Kde, "status_bar", trbl(3.0, 14.0, 2.0, 2.0), &[1372, 1373], "QStatusBar: 2 left, 3 top / 2 bottom; 14 right, the 1 + 13px size grip Breeze paints as nothing (not maximized)"),
    row(Kde, "list", axes(1.0, 2.0), &[1390, 1391], "2; 1")
        .with(&[("row_height", None, 1389)]),
    row(Kde, "popover", NONE, &[1412, 1413], "(none)"),
    row(Kde, "dialog", all(10.0), &[1491, 1492], "Layout_TopLevelMarginWidth = 10"),
    row(Kde, "combo_box", trbl(6.0, 0.0, 6.0, 6.0), &[1552, 1557], "ComboBox_FrameWidth = 6 left, top and bottom; 0 right, to the 20px arrow column")
        .with(&[("arrow_area_width", Some(20.0), 1554)]),
    row(Kde, "segmented_control", NONE, &[1571, 1576], "tab bar as proxy, not the platform's value"),
    row(Kde, "card", NONE, &[1592, 1593], "(none)"),
    row(Kde, "expander", NONE, &[1607, 1608], "(none), app-defined"),
    // --- GNOME (adwaita, adwaita-live; the reader states no sizes) ---
    row(Gnome, "window", NONE, &[1157, 1158], "pointer to §2.20 layout margins"),
    row(Gnome, "button", axes(5.0, 10.0), &[1171, 1172], "10; 5"),
    row(Gnome, "input", axes(0.0, 9.0), &[1196, 1197], "9; 0"),
    row(Gnome, "checkbox", all(3.0), &[1216, 1217], "check { padding: 3px }"),
    row(Gnome, "menu", axes(0.0, 12.0), &[1235, 1236], "12 ($menu_padding); 0")
        .with(&[("row_height", Some(32.0), 1234)]),
    row(Gnome, "tooltip", axes(6.0, 10.0), &[1257, 1258], "10; 6"),
    row(Gnome, "progress_bar", NONE, &[1297], "§2.10 has no padding row"),
    row(Gnome, "tab", axes(3.0, 12.0), &[1318, 1319], "12; 3"),
    row(Gnome, "sidebar", NONE, &[1327], "§2.12 has no padding row"),
    Row {
        platform: Gnome,
        widget: "toolbar",
        padding: all(6.0),
        lines: &[1353, 1354],
        note: ".toolbar { padding: 6px }",
        extra: &[("bar_height", None, 1351), ("item_gap", Some(6.0), 1352)],
    },
    row(Gnome, "status_bar", axes(6.0, 10.0), &[1372, 1373], "statusbar { padding: 6px 10px }"),
    row(Gnome, "list", all(2.0), &[1390, 1391], "plain list context: 2")
        .with(&[("row_height", None, 1389)]),
    row(Gnome, "popover", all(8.0), &[1412, 1413], "popover > contents { padding: 8px }"),
    row(Gnome, "dialog", trbl(32.0, 24.0, 24.0, 24.0), &[1491, 1492], "24; 32 top / 24 bottom"),
    row(Gnome, "combo_box", axes(5.0, 10.0), &[1552, 1557], "← button padding (10px); ← button (5px)")
        .with(&[("arrow_area_width", None, 1554)]),
    row(Gnome, "segmented_control", NONE, &[1571, 1576], "(none)"),
    row(Gnome, "card", NONE, &[1592, 1593], "(none), app-defined"),
    row(Gnome, "expander", NONE, &[1607, 1608], "row padding, no number"),
    // --- macOS (macos-sonoma, macos-sonoma-live, macos_widget_defaults) ---
    row(Macos, "window", NONE, &[1157, 1158], "pointer to §2.20 layout margins"),
    row(Macos, "button", axes(3.0, 8.0), &[1171, 1172], "~8 (WebKit); 3 (measured)"),
    row(Macos, "input", axes(3.0, 4.0), &[1196, 1197], "4; 3 (measured)"),
    row(Macos, "checkbox", NONE, &[1216, 1217], "(none)"),
    row(Macos, "menu", axes(3.0, 12.0), &[1235, 1236], "12; 3 (measured)")
        .with(&[("row_height", Some(22.0), 1234)]),
    row(Macos, "tooltip", all(4.0), &[1257, 1258], "4; 4"),
    row(Macos, "progress_bar", NONE, &[1297], "§2.10 has no padding row"),
    row(Macos, "tab", axes(4.0, 12.0), &[1318, 1319], "12; 4 (measured)"),
    row(Macos, "sidebar", NONE, &[1327], "§2.12 has no padding row"),
    Row {
        platform: Macos,
        widget: "toolbar",
        padding: axes(0.0, 8.0),
        lines: &[1353, 1354],
        note: "8 (measured); 0",
        extra: &[("bar_height", Some(38.0), 1351), ("item_gap", Some(8.0), 1352)],
    },
    row(Macos, "status_bar", NONE, &[1372, 1373], "(none): no window status bar"),
    row(Macos, "list", all(4.0), &[1390, 1391], "4; 4 (measured)")
        .with(&[("row_height", Some(24.0), 1389)]),
    row(Macos, "popover", NONE, &[1412, 1413], "(none)"),
    row(Macos, "dialog", all(20.0), &[1491, 1492], "~20 (measured)"),
    row(Macos, "combo_box", [Some(3.0), None, Some(3.0), None], &[1552, 1557], "horizontal is a range; ~3 (measured)")
        .with(&[("arrow_area_width", None, 1554)]),
    row(Macos, "segmented_control", [Some(3.0), None, Some(3.0), None], &[1571, 1576], "horizontal is a range; ~3 (measured)"),
    row(Macos, "card", NONE, &[1592, 1593], "(none)"),
    row(Macos, "expander", NONE, &[1607, 1608], "(none), app-defined"),
    // --- Windows (windows-11, windows-11-live, winui3_widget_sizing) ---
    row(Windows, "window", NONE, &[1157, 1158], "pointer to §2.20 layout margins"),
    row(Windows, "button", trbl(5.0, 11.0, 6.0, 11.0), &[1171, 1172], "11; 5 top / 6 bottom"),
    row(Windows, "input", trbl(5.0, 6.0, 6.0, 10.0), &[1196, 1197], "10 left / 6 right; 5 top / 6 bottom"),
    row(Windows, "checkbox", NONE, &[1216, 1217], "(none)"),
    row(Windows, "menu", trbl(4.0, 11.0, 5.0, 11.0), &[1235, 1236], "11; mouse context 4 top / 5 bottom")
        .with(&[("row_height", Some(23.0), 1234)]),
    row(Windows, "tooltip", trbl(6.0, 9.0, 8.0, 9.0), &[1257, 1258], "ToolTipBorderPadding=9,6,9,8"),
    row(Windows, "progress_bar", NONE, &[1297], "§2.10 has no padding row"),
    row(Windows, "tab", axes(3.0, 8.0), &[1318, 1319], "without-close-button context: TabViewItemHeaderPaddingWithoutCloseButton=8,3,8,3"),
    row(Windows, "sidebar", NONE, &[1327], "§2.12 has no padding row"),
    Row {
        platform: Windows,
        widget: "toolbar",
        padding: trbl(0.0, 0.0, 0.0, 4.0),
        lines: &[1353, 1354],
        note: "Padding=\"4,0,0,0\"",
        extra: &[("bar_height", Some(48.0), 1351), ("item_gap", Some(0.0), 1352)],
    },
    row(Windows, "status_bar", NONE, &[1372, 1373], "(none): not specified"),
    row(Windows, "list", axes(0.0, 12.0), &[1390, 1391], "12; 0")
        .with(&[("row_height", Some(40.0), 1389)]),
    row(Windows, "popover", trbl(15.0, 16.0, 17.0, 16.0), &[1412, 1413], "FlyoutContentPadding=16,15,16,17"),
    row(Windows, "dialog", all(24.0), &[1491, 1492], "ContentDialogPadding=24"),
    row(Windows, "combo_box", trbl(5.0, 0.0, 7.0, 12.0), &[1552, 1557], "ComboBoxPadding=12,5,0,7: 0 right, to the 38px arrow column")
        .with(&[("arrow_area_width", Some(38.0), 1554)]),
    row(Windows, "segmented_control", NONE, &[1571, 1576], "(none)"),
    row(Windows, "card", all(12.0), &[1592, 1593], "12 (convention)"),
    row(Windows, "expander", trbl(0.0, 0.0, 0.0, 16.0), &[1607, 1608], "header context: ExpanderHeaderPadding=16,0,0,0"),
];

/// Every widget whose border carries padding.
const WIDGETS: [&str; 18] = [
    "window",
    "button",
    "input",
    "checkbox",
    "menu",
    "tooltip",
    "progress_bar",
    "tab",
    "sidebar",
    "toolbar",
    "status_bar",
    "list",
    "popover",
    "dialog",
    "combo_box",
    "segmented_control",
    "card",
    "expander",
];

/// The platform-facts section each widget's rows live in.
fn section(widget: &str) -> &'static str {
    match widget {
        "window" => "2.2",
        "button" => "2.3",
        "input" => "2.4",
        "checkbox" => "2.5",
        "menu" => "2.6",
        "tooltip" => "2.7",
        "progress_bar" => "2.10",
        "tab" => "2.11",
        "sidebar" => "2.12",
        "toolbar" => "2.13",
        "status_bar" => "2.14",
        "list" => "2.15",
        "popover" => "2.16",
        "dialog" => "2.22",
        "combo_box" => "2.24",
        "segmented_control" => "2.25",
        "card" => "2.26",
        "expander" => "2.27",
        _ => "",
    }
}

fn padding(theme: &ResolvedTheme, widget: &str) -> Option<ResolvedPadding> {
    let border = match widget {
        "window" => &theme.window.border,
        "button" => &theme.button.border,
        "input" => &theme.input.border,
        "checkbox" => &theme.checkbox.border,
        "menu" => &theme.menu.border,
        "tooltip" => &theme.tooltip.border,
        "progress_bar" => &theme.progress_bar.border,
        "tab" => &theme.tab.border,
        "sidebar" => &theme.sidebar.border,
        "toolbar" => &theme.toolbar.border,
        "status_bar" => &theme.status_bar.border,
        "list" => &theme.list.border,
        "popover" => &theme.popover.border,
        "dialog" => &theme.dialog.border,
        "combo_box" => &theme.combo_box.border,
        "segmented_control" => &theme.segmented_control.border,
        "card" => &theme.card.border,
        "expander" => &theme.expander.border,
        _ => return None,
    };
    Some(border.padding)
}

/// The in-scope sizing keys a preset states for one widget, unresolved: the
/// four padding sides, and the widget's further fields (`bar_height` and
/// `item_gap` for the toolbar, `row_height` for the menu and the list,
/// `arrow_area_width` for the combobox).
fn stated_sizes(v: &ThemeMode, widget: &str) -> Option<Vec<(&'static str, Option<f32>)>> {
    let border = match widget {
        "window" => &v.window.border,
        "button" => &v.button.border,
        "input" => &v.input.border,
        "checkbox" => &v.checkbox.border,
        "menu" => &v.menu.border,
        "tooltip" => &v.tooltip.border,
        "progress_bar" => &v.progress_bar.border,
        "tab" => &v.tab.border,
        "sidebar" => &v.sidebar.border,
        "toolbar" => &v.toolbar.border,
        "status_bar" => &v.status_bar.border,
        "list" => &v.list.border,
        "popover" => &v.popover.border,
        "dialog" => &v.dialog.border,
        "combo_box" => &v.combo_box.border,
        "segmented_control" => &v.segmented_control.border,
        "card" => &v.card.border,
        "expander" => &v.expander.border,
        _ => return None,
    };
    let side =
        |f: fn(&crate::model::border::WidgetBorderSpec) -> Option<f32>| border.as_ref().and_then(f);
    let mut sizes = vec![
        ("border.padding_top", side(|b| b.padding_top)),
        ("border.padding_right", side(|b| b.padding_right)),
        ("border.padding_bottom", side(|b| b.padding_bottom)),
        ("border.padding_left", side(|b| b.padding_left)),
    ];
    match widget {
        "toolbar" => {
            sizes.push(("bar_height", v.toolbar.bar_height));
            sizes.push(("item_gap", v.toolbar.item_gap));
        }
        "menu" => sizes.push(("row_height", v.menu.row_height)),
        "list" => sizes.push(("row_height", v.list.row_height)),
        "combo_box" => sizes.push(("arrow_area_width", v.combo_box.arrow_area_width)),
        _ => {}
    }
    Some(sizes)
}

/// A row's further field, resolved.
fn extra_field(theme: &ResolvedTheme, widget: &str, field: &str) -> Option<Option<f32>> {
    match (widget, field) {
        ("toolbar", "bar_height") => Some(theme.toolbar.bar_height),
        ("toolbar", "item_gap") => Some(theme.toolbar.item_gap),
        ("menu", "row_height") => Some(theme.menu.row_height),
        ("list", "row_height") => Some(theme.list.row_height),
        ("combo_box", "arrow_area_width") => Some(theme.combo_box.arrow_area_width),
        _ => None,
    }
}

fn resolve(variant: ThemeMode) -> Result<ResolvedTheme, String> {
    variant
        .into_resolved(&ResolutionContext::for_tests())
        .map_err(|e| format!("resolution failed: {e}"))
}

/// The static resolution's input: the platform's full preset, unresolved.
fn static_variant(platform: Platform, mode: ColorMode) -> Result<ThemeMode, String> {
    let full = Theme::preset(platform.preset()).map_err(|e| e.to_string())?;
    full.into_variant(mode).map_err(|e| e.to_string())
}

/// The live resolution's input, unresolved: the full preset, the `-live`
/// preset merged over it, then the reader's size constants.
fn live_variant(platform: Platform, mode: ColorMode) -> Result<ThemeMode, String> {
    let mut merged = Theme::preset(platform.preset()).map_err(|e| e.to_string())?;
    merged.merge(&Theme::preset(platform.live_preset()).map_err(|e| e.to_string())?);
    let mut variant = merged.into_variant(mode).map_err(|e| e.to_string())?;
    if let Some(reader) = platform.reader_constants() {
        variant.merge(&reader);
    }
    Ok(variant)
}

fn gate_variant(platform: Platform, mode: ColorMode, live: bool) -> Result<ThemeMode, String> {
    if live {
        live_variant(platform, mode)
    } else {
        static_variant(platform, mode)
    }
}

fn source(platform: Platform, live: bool) -> String {
    if live {
        format!(
            "{} + {} + reader constants",
            platform.preset(),
            platform.live_preset()
        )
    } else {
        platform.preset().to_string()
    }
}

/// Line `n` (1-based) of platform-facts.md, or "" past its end.
fn facts_line(n: usize) -> &'static str {
    PLATFORM_FACTS.lines().nth(n.wrapping_sub(1)).unwrap_or("")
}

/// The `### 2.` section heading line `n` of platform-facts.md sits under.
fn facts_heading(n: usize) -> &'static str {
    PLATFORM_FACTS
        .lines()
        .take(n)
        .filter(|l| l.starts_with("### 2."))
        .last()
        .unwrap_or("")
}

fn cite(lines: &[usize]) -> String {
    let lines: Vec<String> = lines.iter().map(|l| format!(":{l}")).collect();
    format!("docs/platform-facts.md{}", lines.join(", "))
}

#[test]
fn native_themes_state_documented_sizes() {
    let mut failures = Vec::new();
    for platform in Platform::ALL {
        for (mode, variant) in [(ColorMode::Light, "light"), (ColorMode::Dark, "dark")] {
            for live in [false, true] {
                let theme = match gate_variant(platform, mode, live).and_then(resolve) {
                    Ok(t) => t,
                    Err(e) => {
                        failures.push(format!("{} {variant}: {e}", source(platform, live)));
                        continue;
                    }
                };
                for row in ROWS.iter().filter(|r| r.platform == platform) {
                    let Some(got) = padding(&theme, row.widget) else {
                        failures.push(format!("unknown widget `{}`", row.widget));
                        continue;
                    };
                    let got = [got.top, got.right, got.bottom, got.left];
                    for (i, side) in ["top", "right", "bottom", "left"].iter().enumerate() {
                        if got[i] != row.padding[i] {
                            failures.push(format!(
                                "{}.border.padding_{side}: {} {variant}: resolved {:?}, \
                                 platform-facts gives {:?} ({}; {})",
                                row.widget,
                                source(platform, live),
                                got[i],
                                row.padding[i],
                                cite(row.lines),
                                row.note,
                            ));
                        }
                    }
                    for &(field, expected, line) in row.extra {
                        let got = extra_field(&theme, row.widget, field);
                        if got != Some(expected) {
                            failures.push(format!(
                                "{}.{field}: {} {variant}: resolved {:?}, \
                                 platform-facts gives {expected:?} ({})",
                                row.widget,
                                source(platform, live),
                                got.flatten(),
                                cite(&[line]),
                            ));
                        }
                    }
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} documented-size mismatches:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn every_platform_and_widget_has_one_row() {
    for platform in Platform::ALL {
        for widget in WIDGETS {
            let n = ROWS
                .iter()
                .filter(|r| r.platform == platform && r.widget == widget)
                .count();
            assert_eq!(n, 1, "{platform:?} {widget}: {n} rows");
        }
    }
    assert_eq!(ROWS.len(), Platform::ALL.len() * WIDGETS.len());
}

/// Each cited line is the row it claims to be, inside the widget's section:
/// a padding row, the section heading where a section has no padding row,
/// or the named toolbar field.
#[test]
fn every_citation_names_its_platform_facts_row() {
    for row in ROWS {
        let title = format!("### {} ", section(row.widget));
        let extra_lines = row.extra.iter().map(|&(_, _, n)| n);
        for n in row.lines.iter().copied().chain(extra_lines) {
            assert!(
                facts_heading(n).starts_with(&title),
                "{:?} {}: platform-facts.md:{n} is outside §{}",
                row.platform,
                row.widget,
                section(row.widget),
            );
        }
        for &n in row.lines {
            let text = facts_line(n);
            assert!(
                text.starts_with("| `border.padding_") || text.starts_with(&title),
                "{:?} {}: platform-facts.md:{n} is neither a padding row nor the \
                 section heading: {text:?}",
                row.platform,
                row.widget,
            );
        }
        for &(field, _, n) in row.extra {
            let text = facts_line(n);
            assert!(
                text.starts_with(&format!("| `{field}`")),
                "{:?} {}.{field}: platform-facts.md:{n} is not the `{field}` row: {text:?}",
                row.platform,
                row.widget,
            );
        }
    }
}

/// The live resolution merges the `-live` preset under the reader constants,
/// so a `-live` value a reader overrides never reaches the gate above. Each
/// full preset and its `-live` twin therefore state the same sizes (the
/// padding sides and the further fields), in both variants.
#[test]
fn full_and_live_presets_state_the_same_sizes() {
    let mut failures = Vec::new();
    for platform in Platform::ALL {
        for mode in [ColorMode::Light, ColorMode::Dark] {
            let variant = |name: &str| {
                Theme::preset(name)
                    .and_then(|t| t.into_variant(mode))
                    .map_err(|e| format!("{name} {mode:?}: {e}"))
            };
            let (full, live) = match (variant(platform.preset()), variant(platform.live_preset())) {
                (Ok(f), Ok(l)) => (f, l),
                (f, l) => {
                    failures.extend(f.err().into_iter().chain(l.err()));
                    continue;
                }
            };
            for widget in WIDGETS {
                let (Some(f), Some(l)) = (stated_sizes(&full, widget), stated_sizes(&live, widget))
                else {
                    failures.push(format!("unknown widget `{widget}`"));
                    continue;
                };
                for ((key, f), (_, l)) in f.iter().zip(&l) {
                    if f != l {
                        failures.push(format!(
                            "{widget}.{key} {mode:?}: {} states {f:?}, {} states {l:?}",
                            platform.preset(),
                            platform.live_preset(),
                        ));
                    }
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} full/live disagreements:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// Presets that platform-facts does not cover: the colour schemes, and the
/// `material` and `ios` platform presets, which no platform-facts column
/// documents.
const UNSOURCED_PRESETS: [&str; 12] = [
    "catppuccin-latte",
    "catppuccin-frappe",
    "catppuccin-macchiato",
    "catppuccin-mocha",
    "dracula",
    "gruvbox",
    "material",
    "nord",
    "one-dark",
    "solarized",
    "tokyo-night",
    "ios",
];

/// A preset platform-facts does not cover cites no source for a size, so it
/// states no padding side, `row_height`, `bar_height`, `item_gap` or
/// `arrow_area_width`, and the toolkit's own sizes stand.
#[test]
fn unsourced_presets_state_no_gated_size() {
    let mut failures = Vec::new();
    for name in UNSOURCED_PRESETS {
        for mode in [ColorMode::Light, ColorMode::Dark] {
            let variant = match Theme::preset(name).and_then(|t| t.into_variant(mode)) {
                Ok(v) => v,
                Err(e) => {
                    failures.push(format!("{name} {mode:?}: {e}"));
                    continue;
                }
            };
            for widget in WIDGETS {
                let Some(sizes) = stated_sizes(&variant, widget) else {
                    failures.push(format!("unknown widget `{widget}`"));
                    continue;
                };
                for (key, value) in sizes {
                    if let Some(value) = value {
                        failures.push(format!(
                            "{name} {mode:?}: states {widget}.{key} = {value}, with no source"
                        ));
                    }
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} unsourced sizes:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// What platform-facts gives for one number.
#[derive(Clone, Copy, Debug)]
enum Want {
    /// A font size in points, resolved to logical pixels at the gate's 96 dpi.
    Pt(f32),
    /// A size in logical pixels (Windows' effective pixels).
    Px(f32),
    /// The resolved body font's size times a factor (Kirigami's heading levels).
    BodyTimes(f32),
    /// A CSS font weight.
    Weight(u16),
    /// The cell leaves it to inheritance ("← `defaults.font`", "(none)"): the
    /// merged variant states nothing and the resolver fills it.
    Unstated,
}

struct FieldRow {
    platform: Platform,
    /// `text_scale.<role>.size` / `.weight`, `dialog.title_font.size` /
    /// `.weight`, `slider.track_height`, `slider.thumb_diameter` or
    /// `progress_bar.track_height`.
    field: &'static str,
    want: Want,
    /// The platform-facts.md line of the cell.
    line: usize,
    /// How the cell was read.
    note: &'static str,
}

const fn field(
    platform: Platform,
    field: &'static str,
    want: Want,
    line: usize,
    note: &'static str,
) -> FieldRow {
    FieldRow {
        platform,
        field,
        want,
        line,
        note,
    }
}

use Want::{BodyTimes, Pt, Px, Unstated, Weight};

#[rustfmt::skip]
const FIELD_ROWS: &[FieldRow] = &[
    // --- KDE ---
    field(Kde, "text_scale.caption.size", Pt(8.0), 1435, "smallestReadableFont; its default is 8pt (:573)"),
    field(Kde, "text_scale.caption.weight", Weight(400), 1435, "smallestReadableFont; its default is 400 (:573)"),
    field(Kde, "text_scale.section_heading.size", BodyTimes(1.20), 1436, "Kirigami Heading level 2: body × 1.20"),
    field(Kde, "text_scale.section_heading.weight", Weight(400), 1436, "Font.Normal unless type: Primary"),
    field(Kde, "text_scale.dialog_title.size", BodyTimes(1.35), 1437, "Kirigami Heading level 1: body × 1.35"),
    field(Kde, "text_scale.dialog_title.weight", Weight(400), 1437, "Font.Normal unless type: Primary"),
    field(Kde, "text_scale.display.size", Unstated, 1438, "(none): resolves from the body font"),
    field(Kde, "text_scale.display.weight", Unstated, 1438, "(none): resolves from the body font"),
    field(Kde, "dialog.title_font.size", Unstated, 1496, "← defaults.font"),
    field(Kde, "dialog.title_font.weight", Unstated, 1497, "← defaults.font"),
    field(Kde, "slider.track_height", Px(6.0), 1292, "Slider_GrooveThickness = 6"),
    field(Kde, "slider.thumb_diameter", Px(20.0), 1293, "Slider_ControlThickness = 20"),
    field(Kde, "progress_bar.track_height", Px(6.0), 1303, "ProgressBar_Thickness = 6"),
    // --- GNOME ---
    field(Gnome, "text_scale.caption.size", Pt(9.0), 1435, ".caption: ≈9pt"),
    field(Gnome, "text_scale.caption.weight", Weight(400), 1435, ".caption: 400"),
    field(Gnome, "text_scale.section_heading.size", Pt(11.0), 1436, ".heading: 11pt"),
    field(Gnome, "text_scale.section_heading.weight", Weight(700), 1436, ".heading: 700"),
    field(Gnome, "text_scale.dialog_title.size", Pt(15.0), 1437, ".title-2: ≈15pt"),
    field(Gnome, "text_scale.dialog_title.weight", Weight(800), 1437, ".title-2: 800"),
    field(Gnome, "text_scale.display.size", Pt(20.0), 1438, ".title-1: ≈20pt"),
    field(Gnome, "text_scale.display.weight", Weight(800), 1438, ".title-1: 800"),
    field(Gnome, "dialog.title_font.size", Pt(15.0), 1496, "136% of base ≈15pt (.title-2)"),
    field(Gnome, "dialog.title_font.weight", Weight(800), 1497, "800 (.title-2)"),
    field(Gnome, "slider.track_height", Px(10.0), 1292, "libadwaita .scale: 10"),
    field(Gnome, "slider.thumb_diameter", Px(20.0), 1293, "libadwaita: 20"),
    field(Gnome, "progress_bar.track_height", Px(8.0), 1303, "libadwaita .progressbar: 8"),
    // --- macOS ---
    field(Macos, "text_scale.caption.size", Pt(10.0), 1435, ".caption1: 10pt"),
    field(Macos, "text_scale.caption.weight", Weight(400), 1435, ".caption1: 400"),
    field(Macos, "text_scale.section_heading.size", Pt(13.0), 1436, ".headline: 13pt"),
    field(Macos, "text_scale.section_heading.weight", Weight(700), 1436, ".headline: 700"),
    field(Macos, "text_scale.dialog_title.size", Pt(22.0), 1437, ".title1: 22pt"),
    field(Macos, "text_scale.dialog_title.weight", Weight(400), 1437, ".title1: 400"),
    field(Macos, "text_scale.display.size", Pt(26.0), 1438, ".largeTitle: 26pt"),
    field(Macos, "text_scale.display.weight", Weight(400), 1438, ".largeTitle: 400"),
    field(Macos, "dialog.title_font.size", Pt(13.0), 1496, "emphasized system font: systemFontSize, 13pt"),
    field(Macos, "dialog.title_font.weight", Weight(700), 1497, "emphasized system font: Bold (700)"),
    field(Macos, "slider.track_height", Px(5.0), 1292, "NSSlider: 5"),
    field(Macos, "slider.thumb_diameter", Px(21.0), 1293, "NSSlider knob: 21"),
    field(Macos, "progress_bar.track_height", Px(6.0), 1303, "NSProgressIndicator: 6"),
    // --- Windows ---
    // The presets state the type ramp and title in points, each epx × 72/96
    // (Caption 9pt, Subtitle 15, Title 21, Display 51, dialog title 15). The
    // Windows reader's font_dpi is 96 (`windows::LOGICAL_DPI`), as is the
    // gate's, so they resolve to the epx below at any display scale.
    field(Windows, "text_scale.caption.size", Px(12.0), 1435, "Caption: 12epx"),
    field(Windows, "text_scale.caption.weight", Weight(400), 1435, "Caption: 400"),
    field(Windows, "text_scale.section_heading.size", Px(20.0), 1436, "Subtitle: 20epx"),
    field(Windows, "text_scale.section_heading.weight", Weight(600), 1436, "Subtitle: 600"),
    field(Windows, "text_scale.dialog_title.size", Px(28.0), 1437, "Title: 28epx"),
    field(Windows, "text_scale.dialog_title.weight", Weight(600), 1437, "Title: 600"),
    field(Windows, "text_scale.display.size", Px(68.0), 1438, "Display: 68epx"),
    field(Windows, "text_scale.display.weight", Weight(600), 1438, "Display: 600"),
    field(Windows, "dialog.title_font.size", Px(20.0), 1496, "20px (ContentDialog template)"),
    field(Windows, "dialog.title_font.weight", Weight(600), 1497, "SemiBold (600)"),
    field(Windows, "slider.track_height", Px(4.0), 1292, "WinUI3: 4"),
    field(Windows, "slider.thumb_diameter", Px(18.0), 1293, "WinUI3: 18"),
    field(Windows, "progress_bar.track_height", Px(1.0), 1303, "the groove, ProgressBarTrackHeight: 1"),
];

/// Every field a [`FieldRow`] may name.
const FIELDS: [&str; 13] = [
    "text_scale.caption.size",
    "text_scale.caption.weight",
    "text_scale.section_heading.size",
    "text_scale.section_heading.weight",
    "text_scale.dialog_title.size",
    "text_scale.dialog_title.weight",
    "text_scale.display.size",
    "text_scale.display.weight",
    "dialog.title_font.size",
    "dialog.title_font.weight",
    "slider.track_height",
    "slider.thumb_diameter",
    "progress_bar.track_height",
];

/// The platform-facts section and row key of a field's cell.
fn field_cell(field: &str) -> Option<(&'static str, &'static str)> {
    Some(match field {
        "text_scale.caption.size" | "text_scale.caption.weight" => ("2.19", "caption"),
        "text_scale.section_heading.size" | "text_scale.section_heading.weight" => {
            ("2.19", "section_heading")
        }
        "text_scale.dialog_title.size" | "text_scale.dialog_title.weight" => {
            ("2.19", "dialog_title")
        }
        "text_scale.display.size" | "text_scale.display.weight" => ("2.19", "display"),
        "dialog.title_font.size" => ("2.22", "title_font.size"),
        "dialog.title_font.weight" => ("2.22", "title_font.weight"),
        "slider.track_height" => ("2.9", "track_height"),
        "slider.thumb_diameter" => ("2.9", "thumb_diameter"),
        "progress_bar.track_height" => ("2.10", "track_height"),
        _ => return None,
    })
}

fn stated_entry<'a>(v: &'a ThemeMode, role: &str) -> Option<&'a crate::TextScaleEntry> {
    match role {
        "caption" => v.text_scale.caption.as_ref(),
        "section_heading" => v.text_scale.section_heading.as_ref(),
        "dialog_title" => v.text_scale.dialog_title.as_ref(),
        "display" => v.text_scale.display.as_ref(),
        _ => None,
    }
}

/// Whether the unresolved variant states the field; `None` for an unknown field.
fn states_field(v: &ThemeMode, field: &str) -> Option<bool> {
    let title = v.dialog.title_font.as_ref();
    Some(match field {
        "dialog.title_font.size" => title.and_then(|f| f.size).is_some(),
        "dialog.title_font.weight" => title.and_then(|f| f.weight).is_some(),
        "slider.track_height" => v.slider.track_height.is_some(),
        "slider.thumb_diameter" => v.slider.thumb_diameter.is_some(),
        "progress_bar.track_height" => v.progress_bar.track_height.is_some(),
        _ => {
            let (role, sub) = field.strip_prefix("text_scale.")?.split_once('.')?;
            let entry = stated_entry(v, role);
            match sub {
                "size" => entry.and_then(|e| e.size).is_some(),
                "weight" => entry.and_then(|e| e.weight).is_some(),
                _ => return None,
            }
        }
    })
}

/// The field resolved: sizes in logical pixels, weights as numbers.
fn resolved_field(theme: &ResolvedTheme, field: &str) -> Option<f32> {
    let ts = &theme.text_scale;
    Some(match field {
        "text_scale.caption.size" => ts.caption.size,
        "text_scale.caption.weight" => f32::from(ts.caption.weight),
        "text_scale.section_heading.size" => ts.section_heading.size,
        "text_scale.section_heading.weight" => f32::from(ts.section_heading.weight),
        "text_scale.dialog_title.size" => ts.dialog_title.size,
        "text_scale.dialog_title.weight" => f32::from(ts.dialog_title.weight),
        "text_scale.display.size" => ts.display.size,
        "text_scale.display.weight" => f32::from(ts.display.weight),
        "dialog.title_font.size" => theme.dialog.title_font.size,
        "dialog.title_font.weight" => f32::from(theme.dialog.title_font.weight),
        "slider.track_height" => theme.slider.track_height,
        "slider.thumb_diameter" => theme.slider.thumb_diameter,
        "progress_bar.track_height" => theme.progress_bar.track_height,
        _ => return None,
    })
}

/// The gate's variants of one platform, unresolved: `(source, variant name, variant)`.
fn gate_variants(platform: Platform) -> Vec<Result<(String, &'static str, ThemeMode), String>> {
    let mut out = Vec::new();
    for (mode, name) in [(ColorMode::Light, "light"), (ColorMode::Dark, "dark")] {
        for live in [false, true] {
            out.push(
                gate_variant(platform, mode, live)
                    .map(|v| (source(platform, live), name, v))
                    .map_err(|e| format!("{} {name}: {e}", source(platform, live))),
            );
        }
    }
    out
}

/// The text-scale, dialog-title, slider and progress-bar numbers of the
/// native themes, static and live, against platform-facts.
#[test]
fn native_themes_state_documented_fields() {
    let mut failures = Vec::new();
    for platform in Platform::ALL {
        for variant in gate_variants(platform) {
            let (source, name, variant) = match variant {
                Ok(v) => v,
                Err(e) => {
                    failures.push(e);
                    continue;
                }
            };
            let theme = match resolve(variant.clone()) {
                Ok(t) => t,
                Err(e) => {
                    failures.push(format!("{source} {name}: {e}"));
                    continue;
                }
            };
            let body = theme.defaults.font.size;
            for row in FIELD_ROWS.iter().filter(|r| r.platform == platform) {
                let at = format!(
                    "{}: {source} {name} ({}; {})",
                    row.field,
                    cite(&[row.line]),
                    row.note
                );
                if let Want::Unstated = row.want {
                    match states_field(&variant, row.field) {
                        Some(false) => {}
                        Some(true) => failures.push(format!(
                            "{at}: stated, but platform-facts leaves it to inheritance"
                        )),
                        None => failures.push(format!("{at}: unknown field")),
                    }
                    continue;
                }
                let expected = match row.want {
                    Want::Pt(v) => v * ResolutionContext::for_tests().font_dpi / 72.0,
                    Want::Px(v) => v,
                    Want::BodyTimes(f) => body * f,
                    Want::Weight(w) => f32::from(w),
                    Want::Unstated => continue,
                };
                match resolved_field(&theme, row.field) {
                    Some(got) if (got - expected).abs() < 1e-3 => {}
                    Some(got) => failures.push(format!(
                        "{at}: resolved {got}, platform-facts gives {:?} = {expected}",
                        row.want
                    )),
                    None => failures.push(format!("{at}: unknown field")),
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} documented-field mismatches:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// The Windows rows state effective pixels, which the presets' points reach
/// only at the Windows reader's `font_dpi`: it is the gate's 96.
#[test]
fn windows_reader_font_dpi_is_the_gates() {
    assert_eq!(
        crate::windows::LOGICAL_DPI as f32,
        ResolutionContext::for_tests().font_dpi
    );
}

#[test]
fn every_platform_and_field_has_one_row() {
    for platform in Platform::ALL {
        for f in FIELDS {
            let n = FIELD_ROWS
                .iter()
                .filter(|r| r.platform == platform && r.field == f)
                .count();
            assert_eq!(n, 1, "{platform:?} {f}: {n} rows");
        }
    }
    for f in FIELDS {
        assert!(field_cell(f).is_some(), "{f}: no platform-facts cell");
    }
    assert_eq!(FIELD_ROWS.len(), Platform::ALL.len() * FIELDS.len());
}

/// Each field row cites its own cell's line, inside the cell's section.
#[test]
fn every_field_citation_names_its_platform_facts_row() {
    for row in FIELD_ROWS {
        let (section, key) = field_cell(row.field).unwrap_or(("?", "?"));
        assert!(
            facts_heading(row.line).starts_with(&format!("### {section} ")),
            "{:?} {}: platform-facts.md:{} is outside §{section}",
            row.platform,
            row.field,
            row.line,
        );
        let text = facts_line(row.line);
        assert!(
            text.starts_with(&format!("| `{key}`")),
            "{:?} {}: platform-facts.md:{} is not the `{key}` row: {text:?}",
            row.platform,
            row.field,
            row.line,
        );
    }
}

/// Platform-facts gives no text-scale line height, so no native preset states
/// one and the resolver computes it from `defaults.line_height` (E5).
#[test]
fn native_presets_state_no_text_scale_line_height() {
    let mut failures = Vec::new();
    for platform in Platform::ALL {
        for name in [platform.preset(), platform.live_preset()] {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let variant = match Theme::preset(name).and_then(|t| t.into_variant(mode)) {
                    Ok(v) => v,
                    Err(e) => {
                        failures.push(format!("{name} {mode:?}: {e}"));
                        continue;
                    }
                };
                for role in ["caption", "section_heading", "dialog_title", "display"] {
                    if let Some(lh) = stated_entry(&variant, role).and_then(|e| e.line_height) {
                        failures.push(format!(
                            "{name} {mode:?}: states text_scale.{role}.line_height = {lh:?}, \
                             with no source"
                        ));
                    }
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} unsourced line heights:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
