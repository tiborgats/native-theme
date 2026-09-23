//! Gate: the native themes state what their platform documents
//! (spec `docs/archive/todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md` §1.4–§1.6).
//!
//! One row per (platform, widget) gives every padding side, and for the
//! toolbar its `bar_height` and `item_gap`, as `Some(v)` where
//! `docs/platform-facts.md` documents a value for that platform and `None`
//! where it does not. Each row cites the platform-facts lines it reads.
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
    row(Kde, "menu", all(4.0), &[1235, 1236], "MenuItem_MarginWidth = 4; MenuItem_MarginHeight = 4"),
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
    row(Kde, "status_bar", trbl(3.0, 0.0, 2.0, 2.0), &[1372, 1373], "QStatusBar item layout: 2 left / 0 right, 3 top / 2 bottom"),
    row(Kde, "list", axes(1.0, 2.0), &[1390, 1391], "2; 1"),
    row(Kde, "popover", NONE, &[1412, 1413], "(none)"),
    row(Kde, "dialog", all(10.0), &[1491, 1492], "Layout_TopLevelMarginWidth = 10"),
    row(Kde, "combo_box", all(6.0), &[1552, 1557], "ComboBox_FrameWidth = 6, both axes"),
    row(Kde, "segmented_control", NONE, &[1571, 1576], "tab bar as proxy, not the platform's value"),
    row(Kde, "card", NONE, &[1592, 1593], "(none)"),
    row(Kde, "expander", NONE, &[1607, 1608], "(none), app-defined"),
    // --- GNOME (adwaita, adwaita-live; the reader states no sizes) ---
    row(Gnome, "window", NONE, &[1157, 1158], "pointer to §2.20 layout margins"),
    row(Gnome, "button", axes(5.0, 10.0), &[1171, 1172], "10; 5"),
    row(Gnome, "input", axes(0.0, 9.0), &[1196, 1197], "9; 0"),
    row(Gnome, "checkbox", all(3.0), &[1216, 1217], "check { padding: 3px }"),
    row(Gnome, "menu", axes(0.0, 12.0), &[1235, 1236], "12 ($menu_padding); 0"),
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
    row(Gnome, "list", all(2.0), &[1390, 1391], "plain list context: 2"),
    row(Gnome, "popover", all(8.0), &[1412, 1413], "popover > contents { padding: 8px }"),
    row(Gnome, "dialog", trbl(32.0, 24.0, 24.0, 24.0), &[1491, 1492], "24; 32 top / 24 bottom"),
    row(Gnome, "combo_box", axes(5.0, 10.0), &[1552, 1557], "← button padding (10px); ← button (5px)"),
    row(Gnome, "segmented_control", NONE, &[1571, 1576], "(none)"),
    row(Gnome, "card", NONE, &[1592, 1593], "(none), app-defined"),
    row(Gnome, "expander", NONE, &[1607, 1608], "row padding, no number"),
    // --- macOS (macos-sonoma, macos-sonoma-live, macos_widget_defaults) ---
    row(Macos, "window", NONE, &[1157, 1158], "pointer to §2.20 layout margins"),
    row(Macos, "button", axes(3.0, 8.0), &[1171, 1172], "~8 (WebKit); 3 (measured)"),
    row(Macos, "input", axes(3.0, 4.0), &[1196, 1197], "4; 3 (measured)"),
    row(Macos, "checkbox", NONE, &[1216, 1217], "(none)"),
    row(Macos, "menu", axes(3.0, 12.0), &[1235, 1236], "12; 3 (measured)"),
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
    row(Macos, "list", all(4.0), &[1390, 1391], "4; 4 (measured)"),
    row(Macos, "popover", NONE, &[1412, 1413], "(none)"),
    row(Macos, "dialog", all(20.0), &[1491, 1492], "~20 (measured)"),
    row(Macos, "combo_box", [Some(3.0), None, Some(3.0), None], &[1552, 1557], "horizontal is a range; ~3 (measured)"),
    row(Macos, "segmented_control", [Some(3.0), None, Some(3.0), None], &[1571, 1576], "horizontal is a range; ~3 (measured)"),
    row(Macos, "card", NONE, &[1592, 1593], "(none)"),
    row(Macos, "expander", NONE, &[1607, 1608], "(none), app-defined"),
    // --- Windows (windows-11, windows-11-live, winui3_widget_sizing) ---
    row(Windows, "window", NONE, &[1157, 1158], "pointer to §2.20 layout margins"),
    row(Windows, "button", trbl(5.0, 11.0, 6.0, 11.0), &[1171, 1172], "11; 5 top / 6 bottom"),
    row(Windows, "input", trbl(5.0, 6.0, 6.0, 10.0), &[1196, 1197], "10 left / 6 right; 5 top / 6 bottom"),
    row(Windows, "checkbox", NONE, &[1216, 1217], "(none)"),
    row(Windows, "menu", trbl(4.0, 11.0, 5.0, 11.0), &[1235, 1236], "11; mouse context 4 top / 5 bottom"),
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
    row(Windows, "list", axes(0.0, 12.0), &[1390, 1391], "12; 0"),
    row(Windows, "popover", trbl(15.0, 16.0, 17.0, 16.0), &[1412, 1413], "FlyoutContentPadding=16,15,16,17"),
    row(Windows, "dialog", all(24.0), &[1491, 1492], "ContentDialogPadding=24"),
    row(Windows, "combo_box", [Some(5.0), None, Some(7.0), Some(12.0)], &[1552, 1557], "12 left, right measured to the arrow column; 5 top / 7 bottom"),
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
/// four padding sides, and for the toolbar `bar_height` and `item_gap`.
fn stated_sizes(v: &ThemeMode, widget: &str) -> Option<[Option<f32>; 6]> {
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
    let (bar_height, item_gap) = if widget == "toolbar" {
        (v.toolbar.bar_height, v.toolbar.item_gap)
    } else {
        (None, None)
    };
    Some([
        side(|b| b.padding_top),
        side(|b| b.padding_right),
        side(|b| b.padding_bottom),
        side(|b| b.padding_left),
        bar_height,
        item_gap,
    ])
}

fn toolbar_field(theme: &ResolvedTheme, field: &str) -> Option<Option<f32>> {
    match field {
        "bar_height" => Some(theme.toolbar.bar_height),
        "item_gap" => Some(Some(theme.toolbar.item_gap)),
        _ => None,
    }
}

fn resolve(variant: ThemeMode) -> Result<ResolvedTheme, String> {
    variant
        .into_resolved(&ResolutionContext::for_tests())
        .map_err(|e| format!("resolution failed: {e}"))
}

fn static_theme(platform: Platform, mode: ColorMode) -> Result<ResolvedTheme, String> {
    let full = Theme::preset(platform.preset()).map_err(|e| e.to_string())?;
    resolve(full.into_variant(mode).map_err(|e| e.to_string())?)
}

fn live_theme(platform: Platform, mode: ColorMode) -> Result<ResolvedTheme, String> {
    let mut merged = Theme::preset(platform.preset()).map_err(|e| e.to_string())?;
    merged.merge(&Theme::preset(platform.live_preset()).map_err(|e| e.to_string())?);
    let mut variant = merged.into_variant(mode).map_err(|e| e.to_string())?;
    if let Some(reader) = platform.reader_constants() {
        variant.merge(&reader);
    }
    resolve(variant)
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
                let theme = if live {
                    live_theme(platform, mode)
                } else {
                    static_theme(platform, mode)
                };
                let theme = match theme {
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
                        let got = toolbar_field(&theme, field);
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
    let facts: Vec<&str> = PLATFORM_FACTS.lines().collect();
    let line = |n: usize| facts.get(n.wrapping_sub(1)).copied().unwrap_or("");
    let heading = |n: usize| {
        facts
            .iter()
            .take(n)
            .rev()
            .find(|l| l.starts_with("### 2."))
            .copied()
            .unwrap_or("")
    };
    for row in ROWS {
        let title = format!("### {} ", section(row.widget));
        let extra_lines = row.extra.iter().map(|&(_, _, n)| n);
        for n in row.lines.iter().copied().chain(extra_lines) {
            assert!(
                heading(n).starts_with(&title),
                "{:?} {}: platform-facts.md:{n} is outside §{}",
                row.platform,
                row.widget,
                section(row.widget),
            );
        }
        for &n in row.lines {
            let text = line(n);
            assert!(
                text.starts_with("| `border.padding_") || text.starts_with(&title),
                "{:?} {}: platform-facts.md:{n} is neither a padding row nor the \
                 section heading: {text:?}",
                row.platform,
                row.widget,
            );
        }
        for &(field, _, n) in row.extra {
            let text = line(n);
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
/// full preset and its `-live` twin therefore state the same padding sides,
/// `bar_height` and `item_gap`, in both variants.
#[test]
fn full_and_live_presets_state_the_same_sizes() {
    const KEYS: [&str; 6] = [
        "border.padding_top",
        "border.padding_right",
        "border.padding_bottom",
        "border.padding_left",
        "bar_height",
        "item_gap",
    ];
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
                for (i, key) in KEYS.iter().enumerate() {
                    if f[i] != l[i] {
                        failures.push(format!(
                            "{widget}.{key} {mode:?}: {} states {:?}, {} states {:?}",
                            platform.preset(),
                            f[i],
                            platform.live_preset(),
                            l[i],
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
