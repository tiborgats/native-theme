//! macOS: NSProcessInfo, NSFont, NSColor in both appearances, AppKit control
//! sizes and cell geometry for every NSControlSize, PNG renders of each
//! control at regular size, and, last, a key window's captures: a sidebar
//! material, a table header and a selected segment, sampled per appearance.
//! With `--button-pass`, only the push buttons in a key window per state
//! ([`buttons`]). The calls objc2-app-kit 0.3 marks `unsafe` are in [`ffi`];
//! this file has none.

mod buttons;
mod ffi;

use std::cell::RefCell;
use std::path::Path;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, Sel};
use objc2::{ClassType, MainThreadMarker, MainThreadOnly, Message, sel};
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSApplication, NSApplicationActivationPolicy,
    NSBezelStyle, NSBitmapFormat, NSBitmapImageRep, NSBox, NSBoxType, NSButton, NSButtonType,
    NSColor, NSColorSpace, NSComboBox, NSControl, NSControlSize, NSControlStateValueOff,
    NSControlStateValueOn, NSEventMask, NSFont, NSMenu, NSMenuItem, NSOutlineView, NSPopUpButton,
    NSProgressIndicator, NSProgressIndicatorStyle, NSScreen, NSScrollView, NSScroller,
    NSScrollerStyle, NSSearchField, NSSegmentedControl, NSSlider, NSSliderCell, NSSplitView,
    NSSplitViewDividerStyle, NSStepper, NSSwitch, NSTableColumn, NSTableView, NSTableViewStyle,
    NSTextField, NSTextView, NSTitlePosition, NSView, NSVisualEffectBlendingMode,
    NSVisualEffectMaterial, NSVisualEffectState, NSVisualEffectView, NSWindow, NSWorkspace,
};
use objc2_foundation::{
    NSArray, NSData, NSDate, NSDictionary, NSEdgeInsets, NSNumber, NSOperatingSystemVersion,
    NSPoint, NSProcessInfo, NSRect, NSSize, NSString,
};
use toml::{Table, Value};

use crate::out;

/// Margin around each control in its render, in points, so focus rings and
/// shadows outside the frame are kept.
const RENDER_MARGIN: f64 = 8.0;

/// Width given to the controls that stretch (text fields, sliders, progress
/// bars) when `sizeToFit` leaves them narrower, in points.
const STRETCH_WIDTH: f64 = 160.0;

/// The scroll view the table-header probe puts its NSTableView in, in points.
const TABLE_SCROLL_SIZE: NSSize = NSSize::new(400.0, 300.0);

/// The width of the table-header probe's one column, in points.
const TABLE_COLUMN_WIDTH: f64 = 120.0;

/// The two appearances: the TOML key and the NSAppearanceName.
const APPEARANCES: [(&str, &str); 2] = [
    ("aqua", "NSAppearanceNameAqua"),
    ("darkAqua", "NSAppearanceNameDarkAqua"),
];

const CONTROL_SIZES: [(&str, NSControlSize); 4] = [
    ("mini", NSControlSize::Mini),
    ("small", NSControlSize::Small),
    ("regular", NSControlSize::Regular),
    ("large", NSControlSize::Large),
];

/// Fill `doc` with the macOS results; renders go under `out_dir/renders`.
pub(crate) fn probe(doc: &mut Table, out_dir: &Path, not_key: bool) {
    let (os, version) = os_table();
    out::put_table(doc, "os", os);
    out::put_table(doc, "probe", crate::probe_table());

    let mtm = MainThreadMarker::new();
    // AppKit's views expect NSApp to exist before the first one is made.
    let app = mtm.map(|mtm| {
        let app = NSApplication::sharedApplication(mtm);
        let prohibited = app.setActivationPolicy(NSApplicationActivationPolicy::Prohibited);
        (app, prohibited)
    });

    let mut mac = Table::new();
    out::put_table(&mut mac, "fonts", fonts_table());
    out::put_table(&mut mac, "colors", colors_table(mtm));

    match (mtm, app) {
        (Some(mtm), Some((app, prohibited))) => {
            let mut env = Table::new();
            out::put(
                &mut env,
                "activationPolicy_prohibited_set",
                out::boolean(prohibited),
            );
            out::put(
                &mut env,
                "effectiveAppearance",
                out::text(app.effectiveAppearance().name().to_string()),
            );
            out::put_table(&mut env, "mainScreen", screen_table(mtm));
            out::put_table(&mut env, "accessibility", accessibility_table());
            out::put_table(&mut mac, "environment", env);
            out::put_table(&mut mac, "metrics", metrics_table(mtm));
            out::put_table(&mut mac, "controls", controls_table(mtm, &version, out_dir));
            // Last: it makes the application regular and active, which no
            // earlier section may see.
            out::put_table(
                &mut mac,
                "key_window",
                key_window_table(mtm, &app, out_dir, not_key),
            );
        }
        _ => {
            let reason = out::unavailable("not on the main thread (MainThreadMarker::new is None)");
            out::put(&mut mac, "environment", reason.clone());
            out::put(&mut mac, "metrics", reason.clone());
            out::put(&mut mac, "controls", reason.clone());
            out::put(&mut mac, "key_window", reason);
        }
    }
    out::put_table(doc, "macos", mac);
}

/// `--button-pass`: fill `doc` with `[os]`, `[probe]`, the environment and
/// the button pass only; its captures go under `out_dir/<appearance>`.
pub(crate) fn button_probe(doc: &mut Table, out_dir: &Path) {
    let (os, version) = os_table();
    out::put_table(doc, "os", os);
    out::put_table(doc, "probe", crate::probe_table());
    let mut mac = Table::new();
    match MainThreadMarker::new() {
        Some(mtm) => {
            let app = NSApplication::sharedApplication(mtm);
            let mut env = Table::new();
            out::put(
                &mut env,
                "effectiveAppearance",
                out::text(app.effectiveAppearance().name().to_string()),
            );
            out::put_table(&mut env, "mainScreen", screen_table(mtm));
            out::put_table(&mut env, "accessibility", accessibility_table());
            out::put_table(&mut mac, "environment", env);
            out::put_table(
                &mut mac,
                "button_pass",
                buttons::button_pass_table(mtm, &app, out_dir, &version),
            );
        }
        None => {
            let reason = out::unavailable("not on the main thread (MainThreadMarker::new is None)");
            out::put(&mut mac, "environment", reason.clone());
            out::put(&mut mac, "button_pass", reason);
        }
    }
    out::put_table(doc, "macos", mac);
}

// ── OS ──────────────────────────────────────────────────────────────────────

fn os_table() -> (Table, NSOperatingSystemVersion) {
    let info = NSProcessInfo::processInfo();
    let v = info.operatingSystemVersion();
    let mut t = Table::new();
    out::put(&mut t, "platform", out::text("macOS"));
    out::put(
        &mut t,
        "version",
        out::text(format!(
            "{}.{}.{}",
            v.majorVersion, v.minorVersion, v.patchVersion
        )),
    );
    out::put(&mut t, "major", out::int_isize(v.majorVersion));
    out::put(&mut t, "minor", out::int_isize(v.minorVersion));
    out::put(&mut t, "patch", out::int_isize(v.patchVersion));
    out::put(
        &mut t,
        "operatingSystemVersionString",
        out::text(info.operatingSystemVersionString().to_string()),
    );
    (t, v)
}

// ── Fonts ───────────────────────────────────────────────────────────────────

/// `+[NSFont xxxFontOfSize:]`.
type FontOfSize = fn(f64) -> Retained<NSFont>;
/// `+[NSFont userFontOfSize:]` and its fixed-pitch sibling, which may be nil.
type UserFontOfSize = fn(f64) -> Option<Retained<NSFont>>;
/// `+[NSFont xxxFontOfSize:weight:]`.
type FontOfSizeWeight = fn(f64, f64) -> Retained<NSFont>;

fn fonts_table() -> Table {
    let mut t = Table::new();

    let mut sizes = Table::new();
    out::put(
        &mut sizes,
        "systemFontSize",
        out::float(NSFont::systemFontSize()),
    );
    out::put(
        &mut sizes,
        "smallSystemFontSize",
        out::float(NSFont::smallSystemFontSize()),
    );
    out::put(
        &mut sizes,
        "labelFontSize",
        out::float(NSFont::labelFontSize()),
    );
    let mut per_size = Table::new();
    for (name, size) in CONTROL_SIZES {
        out::put(
            &mut per_size,
            name,
            out::float(NSFont::systemFontSizeForControlSize(size)),
        );
    }
    out::put_table(&mut sizes, "systemFontSizeForControlSize", per_size);
    out::put_table(&mut t, "sizes", sizes);

    let mut named = Table::new();
    out::put(
        &mut named,
        "arguments",
        out::text(
            "size 0.0 (the default size); the monospaced fonts at systemFontSize and NSFontWeightRegular",
        ),
    );
    let class_fonts: [(&str, FontOfSize); 10] = [
        ("systemFontOfSize", NSFont::systemFontOfSize),
        ("boldSystemFontOfSize", NSFont::boldSystemFontOfSize),
        ("labelFontOfSize", NSFont::labelFontOfSize),
        ("messageFontOfSize", NSFont::messageFontOfSize),
        ("menuFontOfSize", NSFont::menuFontOfSize),
        ("menuBarFontOfSize", NSFont::menuBarFontOfSize),
        ("toolTipsFontOfSize", NSFont::toolTipsFontOfSize),
        ("controlContentFontOfSize", NSFont::controlContentFontOfSize),
        ("titleBarFontOfSize", NSFont::titleBarFontOfSize),
        ("paletteFontOfSize", NSFont::paletteFontOfSize),
    ];
    for (name, make) in class_fonts {
        out::put_table(&mut named, name, font_table(&make(0.0)));
    }
    let user_fonts: [(&str, UserFontOfSize); 2] = [
        ("userFontOfSize", NSFont::userFontOfSize),
        ("userFixedPitchFontOfSize", NSFont::userFixedPitchFontOfSize),
    ];
    for (name, make) in user_fonts {
        match make(0.0) {
            Some(font) => out::put_table(&mut named, name, font_table(&font)),
            None => out::put(&mut named, name, out::unavailable("returned nil")),
        }
    }
    let system_size = NSFont::systemFontSize();
    let monospaced: [(&str, FontOfSizeWeight); 2] = [
        (
            "monospacedSystemFontOfSize_weight",
            NSFont::monospacedSystemFontOfSize_weight,
        ),
        (
            "monospacedDigitSystemFontOfSize_weight",
            NSFont::monospacedDigitSystemFontOfSize_weight,
        ),
    ];
    for (name, make) in monospaced {
        match ffi::font_weight_regular() {
            Ok(regular) => {
                out::put_table(&mut named, name, font_table(&make(system_size, regular)))
            }
            Err(reason) => out::put(&mut named, name, out::unavailable(reason)),
        }
    }
    out::put_table(&mut t, "named", named);

    let mut styles = Table::new();
    match ffi::text_styles() {
        Ok(text_styles) => {
            for (name, style) in text_styles {
                match ffi::preferred_font(style) {
                    Ok(font) => out::put_table(&mut styles, name, font_table(&font)),
                    Err(reason) => out::put(&mut styles, name, out::unavailable(reason)),
                }
            }
        }
        Err(reason) => out::put(&mut styles, "error", out::unavailable(reason)),
    }
    out::put_table(&mut t, "preferredFontForTextStyle", styles);
    t
}

fn font_table(font: &NSFont) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "familyName",
        font.familyName()
            .map_or_else(|| out::unavailable("nil"), |n| out::text(n.to_string())),
    );
    out::put(&mut t, "fontName", out::text(font.fontName().to_string()));
    out::put(&mut t, "pointSize", out::float(font.pointSize()));
    out::put(&mut t, "weightTrait", weight_trait(font));
    out::put(&mut t, "ascender", out::float(font.ascender()));
    out::put(&mut t, "descender", out::float(font.descender()));
    out::put(&mut t, "leading", out::float(font.leading()));
    out::put(&mut t, "capHeight", out::float(font.capHeight()));
    out::put(&mut t, "xHeight", out::float(font.xHeight()));
    t
}

/// NSFontWeightTrait from the font descriptor's traits dictionary.
fn weight_trait(font: &NSFont) -> Value {
    let (traits_key, weight_key) = match ffi::font_trait_keys() {
        Ok(keys) => keys,
        Err(reason) => return out::unavailable(reason),
    };
    let descriptor = font.fontDescriptor();
    let Some(traits) = descriptor.objectForKey(traits_key) else {
        return out::unavailable("no NSFontTraitsAttribute");
    };
    let Some(dict) = traits.downcast_ref::<NSDictionary>() else {
        return out::unavailable("NSFontTraitsAttribute is not a dictionary");
    };
    let key: &AnyObject = weight_key.as_ref();
    let Some(weight) = dict.objectForKey(key) else {
        return out::unavailable("no NSFontWeightTrait");
    };
    match weight.downcast_ref::<NSNumber>() {
        Some(n) => out::float(n.doubleValue()),
        None => out::unavailable("NSFontWeightTrait is not a number"),
    }
}

// ── Colours ─────────────────────────────────────────────────────────────────

/// `(name, selector, class method)` for an NSColor class method.
type ColorEntry = (&'static str, Sel, fn() -> Retained<NSColor>);

macro_rules! color_entries {
    ($($name:ident),* $(,)?) => {
        vec![$((stringify!($name), sel!($name), NSColor::$name as fn() -> Retained<NSColor>)),*]
    };
}

/// Every no-argument NSColor class method that returns a UI or system
/// colour (the fixed colours such as `blackColor` excepted), the deprecated
/// ones included.
#[allow(deprecated)]
fn color_entries() -> Vec<ColorEntry> {
    color_entries![
        labelColor,
        secondaryLabelColor,
        tertiaryLabelColor,
        quaternaryLabelColor,
        quinaryLabelColor,
        linkColor,
        placeholderTextColor,
        windowFrameTextColor,
        selectedMenuItemTextColor,
        alternateSelectedControlTextColor,
        headerTextColor,
        separatorColor,
        gridColor,
        windowBackgroundColor,
        underPageBackgroundColor,
        controlBackgroundColor,
        selectedContentBackgroundColor,
        unemphasizedSelectedContentBackgroundColor,
        findHighlightColor,
        textColor,
        textBackgroundColor,
        textInsertionPointColor,
        selectedTextColor,
        selectedTextBackgroundColor,
        unemphasizedSelectedTextBackgroundColor,
        unemphasizedSelectedTextColor,
        controlColor,
        controlTextColor,
        selectedControlColor,
        selectedControlTextColor,
        disabledControlTextColor,
        keyboardFocusIndicatorColor,
        scrubberTexturedBackgroundColor,
        systemRedColor,
        systemGreenColor,
        systemBlueColor,
        systemOrangeColor,
        systemYellowColor,
        systemBrownColor,
        systemPinkColor,
        systemPurpleColor,
        systemGrayColor,
        systemTealColor,
        systemIndigoColor,
        systemMintColor,
        systemCyanColor,
        systemFillColor,
        secondarySystemFillColor,
        tertiarySystemFillColor,
        quaternarySystemFillColor,
        quinarySystemFillColor,
        controlAccentColor,
        highlightColor,
        shadowColor,
        controlHighlightColor,
        controlLightHighlightColor,
        controlShadowColor,
        controlDarkShadowColor,
        scrollBarColor,
        knobColor,
        selectedKnobColor,
        windowFrameColor,
        selectedMenuItemColor,
        headerColor,
        secondarySelectedControlColor,
        alternateSelectedControlColor,
    ]
}

/// `(name, selector, class method)` for an NSColor array class method.
type ColorListEntry = (&'static str, Sel, fn() -> Retained<NSArray<NSColor>>);

#[allow(deprecated)]
fn color_list_entries() -> [ColorListEntry; 2] {
    [
        (
            "alternatingContentBackgroundColors",
            sel!(alternatingContentBackgroundColors),
            NSColor::alternatingContentBackgroundColors,
        ),
        (
            "controlAlternatingRowBackgroundColors",
            sel!(controlAlternatingRowBackgroundColors),
            NSColor::controlAlternatingRowBackgroundColors,
        ),
    ]
}

fn colors_table(mtm: Option<MainThreadMarker>) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "format",
        out::text(
            "#rrggbbaa in sRGB (colorUsingColorSpace: sRGBColorSpace), resolved inside NSAppearance performAsCurrentDrawingAppearance:; each selector checked with respondsToSelector: first",
        ),
    );
    for (key, name) in APPEARANCES {
        let Some(appearance) = NSAppearance::appearanceNamed(&NSString::from_str(name)) else {
            out::put(
                &mut t,
                key,
                out::unavailable(format!("appearanceNamed({name}) is nil")),
            );
            continue;
        };
        let result = RefCell::new(Table::new());
        {
            let block = RcBlock::new(|| {
                *result.borrow_mut() = appearance_colors(mtm);
            });
            appearance.performAsCurrentDrawingAppearance(&block);
        }
        out::put_table(&mut t, key, result.into_inner());
    }
    t
}

fn appearance_colors(mtm: Option<MainThreadMarker>) -> Table {
    let srgb = NSColorSpace::sRGBColorSpace();
    let meta = NSColor::class().metaclass();
    let mut t = Table::new();
    for (name, selector, make) in color_entries() {
        let value = if meta.responds_to(selector) {
            color_value(&make(), &srgb)
        } else {
            out::unavailable(format!("NSColor does not respond to +{name} on this macOS"))
        };
        out::put(&mut t, name, value);
    }
    for (name, selector, make) in color_list_entries() {
        let value = if meta.responds_to(selector) {
            out::array(make().iter().map(|c| color_value(&c, &srgb)).collect())
        } else {
            out::unavailable(format!("NSColor does not respond to +{name} on this macOS"))
        };
        out::put(&mut t, name, value);
    }
    // The per-view caret colour docs/platform-facts.md names beside
    // textInsertionPointColor: a new NSTextView's, which defaults to
    // controlTextColor there.
    let caret = match mtm {
        Some(mtm) => color_value(&NSTextView::new(mtm).insertionPointColor(), &srgb),
        None => out::unavailable("not on the main thread"),
    };
    out::put(&mut t, "NSTextView_insertionPointColor", caret);
    t
}

fn color_value(color: &NSColor, srgb: &NSColorSpace) -> Value {
    match color.colorUsingColorSpace(srgb) {
        Some(c) => out::text(out::rgba_hex(
            c.redComponent(),
            c.greenComponent(),
            c.blueComponent(),
            c.alphaComponent(),
        )),
        None => out::text(format!(
            "not convertible: NSColorType {} ({:?})",
            color.r#type().0,
            color
        )),
    }
}

// ── Environment ─────────────────────────────────────────────────────────────

fn screen_table(mtm: MainThreadMarker) -> Table {
    let mut t = Table::new();
    match NSScreen::mainScreen(mtm) {
        Some(screen) => {
            out::put(
                &mut t,
                "backingScaleFactor",
                out::float(screen.backingScaleFactor()),
            );
            out::put(&mut t, "frame_xywh", rect(screen.frame()));
        }
        None => out::put(&mut t, "screen", out::unavailable("mainScreen is nil")),
    }
    t
}

fn accessibility_table() -> Table {
    let ws = NSWorkspace::sharedWorkspace();
    let mut t = Table::new();
    out::put(
        &mut t,
        "accessibilityDisplayShouldIncreaseContrast",
        out::boolean(ws.accessibilityDisplayShouldIncreaseContrast()),
    );
    out::put(
        &mut t,
        "accessibilityDisplayShouldReduceTransparency",
        out::boolean(ws.accessibilityDisplayShouldReduceTransparency()),
    );
    out::put(
        &mut t,
        "accessibilityDisplayShouldReduceMotion",
        out::boolean(ws.accessibilityDisplayShouldReduceMotion()),
    );
    out::put(
        &mut t,
        "accessibilityDisplayShouldDifferentiateWithoutColor",
        out::boolean(ws.accessibilityDisplayShouldDifferentiateWithoutColor()),
    );
    t
}

// ── Metrics that need no window ─────────────────────────────────────────────

fn metrics_table(mtm: MainThreadMarker) -> Table {
    let mut t = Table::new();

    let mut scroller = Table::new();
    out::put(
        &mut scroller,
        "preferredScrollerStyle",
        out::text(scroller_style_name(NSScroller::preferredScrollerStyle(mtm))),
    );
    let mut widths = Table::new();
    for (name, size) in CONTROL_SIZES {
        let mut per_style = Table::new();
        for (style_name, style) in [
            ("legacy", NSScrollerStyle::Legacy),
            ("overlay", NSScrollerStyle::Overlay),
        ] {
            out::put(
                &mut per_style,
                style_name,
                out::float(NSScroller::scrollerWidthForControlSize_scrollerStyle(
                    size, style, mtm,
                )),
            );
        }
        out::put_table(&mut widths, name, per_style);
    }
    out::put_table(
        &mut scroller,
        "scrollerWidthForControlSize_scrollerStyle",
        widths,
    );
    out::put_table(&mut t, "NSScroller", scroller);

    let mut split = Table::new();
    let split_view = NSSplitView::new(mtm);
    out::put(
        &mut split,
        "default_dividerStyle",
        out::text(divider_style_name(split_view.dividerStyle())),
    );
    out::put(
        &mut split,
        "default_dividerThickness",
        out::float(split_view.dividerThickness()),
    );
    let mut thickness = Table::new();
    for style in [
        NSSplitViewDividerStyle::Thick,
        NSSplitViewDividerStyle::Thin,
        NSSplitViewDividerStyle::PaneSplitter,
    ] {
        split_view.setDividerStyle(style);
        out::put(
            &mut thickness,
            &divider_style_name(style),
            out::float(split_view.dividerThickness()),
        );
    }
    out::put_table(&mut split, "dividerThickness", thickness);
    out::put_table(&mut t, "NSSplitView", split);

    let mut table = Table::new();
    let table_view = NSTableView::new(mtm);
    out::put(&mut table, "rowHeight", out::float(table_view.rowHeight()));
    out::put(
        &mut table,
        "intercellSpacing_wh",
        size(table_view.intercellSpacing()),
    );
    out::put(
        &mut table,
        "style",
        out::text(table_style_name(table_view.style())),
    );
    out::put(
        &mut table,
        "effectiveStyle",
        out::text(table_style_name(table_view.effectiveStyle())),
    );
    out::put_table(&mut table, "header", table_header_table(mtm));
    out::put_table(&mut t, "NSTableView", table);

    let mut outline = Table::new();
    let outline_view = NSOutlineView::new(mtm);
    out::put(
        &mut outline,
        "indentationPerLevel",
        out::float(outline_view.indentationPerLevel()),
    );
    out::put(
        &mut outline,
        "rowHeight",
        out::float(outline_view.rowHeight()),
    );
    out::put_table(&mut t, "NSOutlineView", outline);

    let mut menu_t = Table::new();
    let menu = NSMenu::new(mtm);
    for title in ["First item", "Second item", "Third item"] {
        let item = NSMenuItem::new(mtm);
        item.setTitle(&NSString::from_str(title));
        menu.addItem(&item);
    }
    out::put(
        &mut menu_t,
        "numberOfItems",
        out::int_isize(menu.numberOfItems()),
    );
    out::put(&mut menu_t, "size_wh", size(menu.size()));
    out::put(
        &mut menu_t,
        "menuBarHeight",
        out::float(menu.menuBarHeight()),
    );
    out::put(
        &mut menu_t,
        "item_height",
        out::not_probed(
            "NSMenuItem has no height or frame outside a displayed menu; size_wh is the three-item menu's",
        ),
    );
    out::put_table(&mut t, "NSMenu", menu_t);
    t
}

/// The header of a new NSTableView, per appearance, for its default style
/// and for `NSTableViewStyleInset` and `NSTableViewStyleFullWidth`, each
/// measured without a window and in an offscreen window.
fn table_header_table(mtm: MainThreadMarker) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(format!(
            "a new NSTableView with one {TABLE_COLUMN_WIDTH} pt NSTableColumn, made the document view of a new {} x {} pt NSScrollView whose appearance is set per run, all inside NSAppearance performAsCurrentDrawingAppearance:; setStyle: before setDocumentView: (none for default); windowless: the scroll view in no window; inWindow: the scroll view added to the content view of an offscreen borderless NSWindow (buffered, never ordered front) whose appearance is set per run, removed after the reading; then tile on the scroll view and the table; heights in points",
            TABLE_SCROLL_SIZE.width, TABLE_SCROLL_SIZE.height
        )),
    );
    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(800.0, 600.0));
    let window = ffi::offscreen_window(mtm, frame);
    let content = match &window {
        Ok(window) => window
            .contentView()
            .ok_or_else(|| "the offscreen window has no content view".to_owned()),
        Err(reason) => Err(reason.clone()),
    };
    let styles = [
        ("default", None),
        ("inset", Some(NSTableViewStyle::Inset)),
        ("fullWidth", Some(NSTableViewStyle::FullWidth)),
    ];
    for (key, name) in APPEARANCES {
        let Some(appearance) = NSAppearance::appearanceNamed(&ns(name)) else {
            out::put(
                &mut t,
                key,
                out::unavailable(format!("appearanceNamed({name}) is nil")),
            );
            continue;
        };
        if let Ok(window) = &window {
            window.setAppearance(Some(&appearance));
        }
        let result = RefCell::new(Table::new());
        {
            let block = RcBlock::new(|| {
                let mut per_style = Table::new();
                for (style_key, style) in styles {
                    let mut run = Table::new();
                    out::put_table(
                        &mut run,
                        "windowless",
                        table_header(mtm, &appearance, style, None),
                    );
                    match &content {
                        Ok(content) => out::put_table(
                            &mut run,
                            "inWindow",
                            table_header(mtm, &appearance, style, Some(content)),
                        ),
                        Err(reason) => {
                            out::put(&mut run, "inWindow", out::unavailable(reason));
                        }
                    }
                    out::put_table(&mut per_style, style_key, run);
                }
                *result.borrow_mut() = per_style;
            });
            appearance.performAsCurrentDrawingAppearance(&block);
        }
        out::put_table(&mut t, key, result.into_inner());
    }
    if let Ok(window) = &window {
        window.setAppearance(None);
    }
    t
}

/// One header measurement: `style` is set with `setStyle:` when given; the
/// scroll view is added to `superview` for the reading when given.
fn table_header(
    mtm: MainThreadMarker,
    appearance: &NSAppearance,
    style: Option<NSTableViewStyle>,
    superview: Option<&NSView>,
) -> Table {
    let mut t = Table::new();
    let scroll = NSScrollView::initWithFrame(
        NSScrollView::alloc(mtm),
        NSRect::new(NSPoint::new(0.0, 0.0), TABLE_SCROLL_SIZE),
    );
    scroll.setAppearance(Some(appearance));
    let table_view = NSTableView::new(mtm);
    let column = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), &ns("column"));
    column.setTitle(&ns("Column"));
    column.setWidth(TABLE_COLUMN_WIDTH);
    table_view.addTableColumn(&column);
    if let Some(style) = style {
        table_view.setStyle(style);
    }
    scroll.setDocumentView(Some(&table_view));
    if let Some(superview) = superview {
        superview.addSubview(&scroll);
    }
    scroll.tile();
    table_view.tile();
    out::put(
        &mut t,
        "style",
        out::text(table_style_name(table_view.style())),
    );
    out::put(
        &mut t,
        "effectiveStyle",
        out::text(table_style_name(table_view.effectiveStyle())),
    );
    match table_view.headerView() {
        Some(header) => {
            out::put(
                &mut t,
                "headerView_height",
                out::float(header.frame().size.height),
            );
            out::put(&mut t, "headerView_frame_xywh", rect(header.frame()));
            out::put(
                &mut t,
                "headerRectOfColumn_0_xywh",
                rect(header.headerRectOfColumn(0)),
            );
        }
        None => out::put(
            &mut t,
            "headerView_height",
            out::unavailable("headerView is nil"),
        ),
    }
    if superview.is_some() {
        scroll.removeFromSuperview();
    }
    t
}

fn scroller_style_name(style: NSScrollerStyle) -> String {
    if style == NSScrollerStyle::Legacy {
        "legacy".to_owned()
    } else if style == NSScrollerStyle::Overlay {
        "overlay".to_owned()
    } else {
        format!("unknown ({})", style.0)
    }
}

fn divider_style_name(style: NSSplitViewDividerStyle) -> String {
    if style == NSSplitViewDividerStyle::Thick {
        "thick".to_owned()
    } else if style == NSSplitViewDividerStyle::Thin {
        "thin".to_owned()
    } else if style == NSSplitViewDividerStyle::PaneSplitter {
        "paneSplitter".to_owned()
    } else {
        format!("unknown ({})", style.0)
    }
}

fn table_style_name(style: NSTableViewStyle) -> String {
    if style == NSTableViewStyle::Automatic {
        "automatic".to_owned()
    } else if style == NSTableViewStyle::FullWidth {
        "fullWidth".to_owned()
    } else if style == NSTableViewStyle::Inset {
        "inset".to_owned()
    } else if style == NSTableViewStyle::SourceList {
        "sourceList".to_owned()
    } else if style == NSTableViewStyle::Plain {
        "plain".to_owned()
    } else {
        format!("unknown ({})", style.0)
    }
}

// ── Controls ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    PushButton,
    Checkbox,
    Radio,
    Switch,
    PopUpButton,
    ComboBox,
    TextField,
    SearchField,
    SegmentedControl,
    Slider,
    SliderTicks,
    ProgressBar,
    ProgressSpinning,
    Stepper,
}

impl Kind {
    const ALL: [Kind; 14] = [
        Kind::PushButton,
        Kind::Checkbox,
        Kind::Radio,
        Kind::Switch,
        Kind::PopUpButton,
        Kind::ComboBox,
        Kind::TextField,
        Kind::SearchField,
        Kind::SegmentedControl,
        Kind::Slider,
        Kind::SliderTicks,
        Kind::ProgressBar,
        Kind::ProgressSpinning,
        Kind::Stepper,
    ];

    fn name(self) -> &'static str {
        match self {
            Kind::PushButton => "push_button",
            Kind::Checkbox => "checkbox",
            Kind::Radio => "radio",
            Kind::Switch => "switch",
            Kind::PopUpButton => "pop_up_button",
            Kind::ComboBox => "combo_box",
            Kind::TextField => "text_field",
            Kind::SearchField => "search_field",
            Kind::SegmentedControl => "segmented_control",
            Kind::Slider => "slider",
            Kind::SliderTicks => "slider_ticks",
            Kind::ProgressBar => "progress_bar",
            Kind::ProgressSpinning => "progress_spinning",
            Kind::Stepper => "stepper",
        }
    }

    fn setup(self) -> &'static str {
        match self {
            Kind::PushButton => {
                "NSButton, buttonType momentaryPushIn, bezelStyle push (rounded), title \"Button\""
            }
            Kind::Checkbox => "NSButton, buttonType switch, title \"Checkbox\"",
            Kind::Radio => "NSButton, buttonType radio, title \"Radio\"",
            Kind::Switch => "NSSwitch",
            Kind::PopUpButton => "NSPopUpButton, items \"Option\", \"Other\"",
            Kind::ComboBox => "NSComboBox, stringValue \"Option\"",
            Kind::TextField => "NSTextField, bezeled, editable, stringValue \"Text\"",
            Kind::SearchField => "NSSearchField, empty",
            Kind::SegmentedControl => {
                "NSSegmentedControl, 2 segments \"One\" \"Two\", segment 0 selected"
            }
            Kind::Slider => "NSSlider, horizontal, 0..100 at 50, no tick marks",
            Kind::SliderTicks => "NSSlider, horizontal, 0..100 at 50, 5 tick marks",
            Kind::ProgressBar => "NSProgressIndicator, style bar, determinate, 0..100 at 50",
            Kind::ProgressSpinning => "NSProgressIndicator, style spinning",
            Kind::Stepper => "NSStepper",
        }
    }

    /// Controls whose width is the caller's: given `STRETCH_WIDTH` when
    /// `sizeToFit` leaves them narrower.
    fn stretches(self) -> bool {
        matches!(
            self,
            Kind::ComboBox
                | Kind::TextField
                | Kind::SearchField
                | Kind::Slider
                | Kind::SliderTicks
                | Kind::ProgressBar
        )
    }

    /// Controls rendered both off and on.
    fn has_state(self) -> bool {
        matches!(self, Kind::Checkbox | Kind::Radio | Kind::Switch)
    }

    /// Whether the control section also renders it disabled
    /// (`setEnabled: NO`), beside its enabled renders.
    fn renders_disabled(self) -> bool {
        matches!(
            self,
            Kind::PushButton | Kind::Checkbox | Kind::Radio | Kind::Switch | Kind::Slider
        )
    }
}

fn ns(s: &str) -> Retained<NSString> {
    NSString::from_str(s)
}

/// Give an NSControl the control size and the system font of that size.
fn control_common(control: &NSControl, size: NSControlSize) {
    control.setControlSize(size);
    control.setFont(Some(&NSFont::systemFontOfSize(
        NSFont::systemFontSizeForControlSize(size),
    )));
}

fn state_value(on: bool) -> isize {
    if on {
        NSControlStateValueOn
    } else {
        NSControlStateValueOff
    }
}

fn build(kind: Kind, size: NSControlSize, on: bool, mtm: MainThreadMarker) -> Retained<NSView> {
    match kind {
        Kind::PushButton | Kind::Checkbox | Kind::Radio => {
            let b = NSButton::new(mtm);
            let (button_type, title) = match kind {
                Kind::Checkbox => (NSButtonType::Switch, "Checkbox"),
                Kind::Radio => (NSButtonType::Radio, "Radio"),
                _ => (NSButtonType::MomentaryPushIn, "Button"),
            };
            b.setButtonType(button_type);
            if kind == Kind::PushButton {
                b.setBezelStyle(NSBezelStyle::Push);
            } else {
                b.setState(state_value(on));
            }
            b.setTitle(&ns(title));
            control_common(&b, size);
            view_of(&b)
        }
        Kind::Switch => {
            let s = NSSwitch::new(mtm);
            s.setState(state_value(on));
            control_common(&s, size);
            view_of(&s)
        }
        Kind::PopUpButton => {
            let p = NSPopUpButton::new(mtm);
            p.addItemWithTitle(&ns("Option"));
            p.addItemWithTitle(&ns("Other"));
            control_common(&p, size);
            view_of(&p)
        }
        Kind::ComboBox => {
            let c = NSComboBox::new(mtm);
            c.setStringValue(&ns("Option"));
            control_common(&c, size);
            view_of(&c)
        }
        Kind::TextField => {
            let f = NSTextField::new(mtm);
            f.setBezeled(true);
            f.setEditable(true);
            f.setStringValue(&ns("Text"));
            control_common(&f, size);
            view_of(&f)
        }
        Kind::SearchField => {
            let f = NSSearchField::new(mtm);
            control_common(&f, size);
            view_of(&f)
        }
        Kind::SegmentedControl => {
            let s = NSSegmentedControl::new(mtm);
            s.setSegmentCount(2);
            s.setLabel_forSegment(&ns("One"), 0);
            s.setLabel_forSegment(&ns("Two"), 1);
            s.setSelectedSegment(0);
            control_common(&s, size);
            view_of(&s)
        }
        Kind::Slider | Kind::SliderTicks => {
            let s = NSSlider::new(mtm);
            s.setVertical(false);
            s.setMinValue(0.0);
            s.setMaxValue(100.0);
            s.setDoubleValue(50.0);
            if kind == Kind::SliderTicks {
                s.setNumberOfTickMarks(5);
            }
            control_common(&s, size);
            view_of(&s)
        }
        Kind::ProgressBar | Kind::ProgressSpinning => {
            let p = NSProgressIndicator::new(mtm);
            if kind == Kind::ProgressBar {
                p.setStyle(NSProgressIndicatorStyle::Bar);
                p.setIndeterminate(false);
                p.setMinValue(0.0);
                p.setMaxValue(100.0);
                p.setDoubleValue(50.0);
            } else {
                p.setStyle(NSProgressIndicatorStyle::Spinning);
            }
            p.setControlSize(size);
            view_of(&p)
        }
        Kind::Stepper => {
            let s = NSStepper::new(mtm);
            control_common(&s, size);
            view_of(&s)
        }
    }
}

/// A retained NSView reference to any view subclass.
fn view_of(view: &NSView) -> Retained<NSView> {
    view.retain()
}

fn controls_table(
    mtm: MainThreadMarker,
    version: &NSOperatingSystemVersion,
    out_dir: &Path,
) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(format!(
            "each control alone in an offscreen borderless NSWindow (buffered, never ordered front) whose appearance is set per run; NSControls get setControlSize: and the systemFontOfSize: of systemFontSizeForControlSize:; measured before and after sizeToFit; controls that stretch are widened to {STRETCH_WIDTH} pt after sizeToFit when narrower; cell rects are for the control's bounds after that"
        )),
    );
    out::put(
        &mut t,
        "keys",
        out::text(
            "_wh = [width, height]; _xywh = [x, y, width, height]; _tlbr = [top, left, bottom, right]; all in points",
        ),
    );
    out::put(
        &mut t,
        "method_disabled",
        out::text(
            "at the regular size the push button, checkbox, radio, switch and slider are also rendered disabled (setEnabled: NO), each at its enabled render's size: render_disabled (<kind>-regular-disabled, or -disabled-off for a control with a state) and render_disabled_on (-disabled-on)",
        ),
    );
    let mut setups = Table::new();
    for kind in Kind::ALL {
        out::put(&mut setups, kind.name(), out::text(kind.setup()));
    }
    out::put_table(&mut t, "setup", setups);
    out::put(
        &mut t,
        "extraLarge",
        out::not_probed(
            "NSControlSizeExtraLarge is outside the review's sizes (mini, small, regular, large)",
        ),
    );

    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(800.0, 600.0));
    let window = match ffi::offscreen_window(mtm, frame) {
        Ok(window) => window,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    let Some(content) = window.contentView() else {
        out::put(
            &mut t,
            "error",
            out::unavailable("the offscreen window has no content view"),
        );
        return t;
    };
    let backing_scale = window.backingScaleFactor();
    out::put(
        &mut t,
        "window_backingScaleFactor",
        out::float(backing_scale),
    );

    for (key, name) in APPEARANCES {
        let Some(appearance) = NSAppearance::appearanceNamed(&ns(name)) else {
            out::put(
                &mut t,
                key,
                out::unavailable(format!("appearanceNamed({name}) is nil")),
            );
            continue;
        };
        window.setAppearance(Some(&appearance));
        let run = Run {
            mtm,
            content: &content,
            appearance_key: key,
            out_dir,
            backing_scale,
            // NSControlSizeLarge is macOS 11+.
            large_available: version.majorVersion >= 11,
        };
        let result = RefCell::new(Table::new());
        {
            let block = RcBlock::new(|| {
                *result.borrow_mut() = run.all_controls();
            });
            appearance.performAsCurrentDrawingAppearance(&block);
        }
        out::put_table(&mut t, key, result.into_inner());
    }
    window.setAppearance(None);
    t
}

/// One appearance's control run.
struct Run<'a> {
    mtm: MainThreadMarker,
    content: &'a NSView,
    appearance_key: &'a str,
    out_dir: &'a Path,
    backing_scale: f64,
    large_available: bool,
}

impl Run<'_> {
    fn all_controls(&self) -> Table {
        let mut t = Table::new();
        for (size_name, size) in CONTROL_SIZES {
            if size == NSControlSize::Large && !self.large_available {
                out::put(
                    &mut t,
                    size_name,
                    out::unavailable("NSControlSizeLarge needs macOS 11"),
                );
                continue;
            }
            let mut per_kind = Table::new();
            for kind in Kind::ALL {
                out::put_table(
                    &mut per_kind,
                    kind.name(),
                    self.control(kind, size_name, size),
                );
            }
            out::put_table(&mut t, size_name, per_kind);
        }
        t
    }

    fn control(&self, kind: Kind, size_name: &str, size: NSControlSize) -> Table {
        let view = build(kind, size, false, self.mtm);
        let container = self.mount(&view);
        let mut t = measure(kind, &view);
        fit_container(&container, &view);
        if size == NSControlSize::Regular {
            let state = if kind.has_state() { "-off" } else { "" };
            let stem = format!("{}-{size_name}{state}", kind.name());
            out::put_table(&mut t, "render", self.render(&container, &stem));
            if kind.has_state() {
                let on_view = build(kind, size, true, self.mtm);
                let on_container = self.mount(&on_view);
                on_view.setFrameSize(view.frame().size);
                fit_container(&on_container, &on_view);
                let stem_on = format!("{}-{size_name}-on", kind.name());
                out::put_table(&mut t, "render_on", self.render(&on_container, &stem_on));
                on_container.removeFromSuperview();
            }
            if kind.renders_disabled() {
                for on in [false, true] {
                    if on && !kind.has_state() {
                        continue;
                    }
                    let disabled = build(kind, size, on, self.mtm);
                    let obj: &AnyObject = disabled.as_ref();
                    if let Some(control) = obj.downcast_ref::<NSControl>() {
                        control.setEnabled(false);
                    }
                    let disabled_container = self.mount(&disabled);
                    disabled.setFrameSize(view.frame().size);
                    fit_container(&disabled_container, &disabled);
                    let (key, state) = match (kind.has_state(), on) {
                        (false, _) => ("render_disabled", ""),
                        (true, false) => ("render_disabled", "-off"),
                        (true, true) => ("render_disabled_on", "-on"),
                    };
                    let stem = format!("{}-{size_name}-disabled{state}", kind.name());
                    out::put_table(&mut t, key, self.render(&disabled_container, &stem));
                    disabled_container.removeFromSuperview();
                }
            }
        }
        container.removeFromSuperview();
        t
    }

    /// Put `view` in a fresh container inside the window's content view.
    fn mount(&self, view: &NSView) -> Retained<NSView> {
        let container = NSView::new(self.mtm);
        container.addSubview(view);
        self.content.addSubview(&container);
        container
    }

    /// Render `container` at the window's backing scale and at scale 2.
    fn render(&self, container: &NSView, stem: &str) -> Table {
        let mut t = Table::new();
        let dir = self.out_dir.join("renders").join(self.appearance_key);
        if let Err(e) = std::fs::create_dir_all(&dir) {
            out::put(
                &mut t,
                "error",
                out::unavailable(format!("cannot create {}: {e}", dir.display())),
            );
            return t;
        }
        let bounds = container.bounds();
        out::put(
            &mut t,
            "background",
            out::text("transparent (only the control and its container are drawn)"),
        );
        out::put(&mut t, "margin", out::float(RENDER_MARGIN));
        out::put(&mut t, "points_wh", size(bounds.size));
        out::put(
            &mut t,
            "window_backingScaleFactor",
            out::float(self.backing_scale),
        );

        let backing = match container.bitmapImageRepForCachingDisplayInRect(bounds) {
            Some(rep) => {
                container.cacheDisplayInRect_toBitmapImageRep(bounds, &rep);
                self.write_png(&rep, &dir, &format!("{stem}.png"))
            }
            None => out::unavailable("bitmapImageRepForCachingDisplayInRect: is nil"),
        };
        out::put(&mut t, "backing_scale", backing);

        let scale2 = match bitmap_at_scale(bounds.size, 2.0) {
            Ok(rep) => {
                container.cacheDisplayInRect_toBitmapImageRep(bounds, &rep);
                self.write_png(&rep, &dir, &format!("{stem}@2x.png"))
            }
            Err(reason) => out::unavailable(reason),
        };
        out::put(&mut t, "scale_2", scale2);
        t
    }

    fn write_png(&self, rep: &NSBitmapImageRep, dir: &Path, file: &str) -> Value {
        let data = match ffi::png_data(rep) {
            Ok(data) => data,
            Err(reason) => return out::unavailable(reason),
        };
        let path = dir.join(file);
        if let Err(e) = std::fs::write(&path, data) {
            return out::unavailable(format!("cannot write {}: {e}", path.display()));
        }
        let mut t = Table::new();
        out::put(
            &mut t,
            "file",
            out::text(format!("renders/{}/{file}", self.appearance_key)),
        );
        out::put(
            &mut t,
            "pixels_wh",
            out::array(vec![
                out::int_isize(rep.pixelsWide()),
                out::int_isize(rep.pixelsHigh()),
            ]),
        );
        out::put(&mut t, "rep_size_wh", size(rep.size()));
        Value::Table(t)
    }
}

/// An RGBA bitmap rep of `points` at `scale` pixels per point.
fn bitmap_at_scale(points: NSSize, scale: f64) -> Result<Retained<NSBitmapImageRep>, String> {
    let (Some(width), Some(height)) = (
        out::pixels(points.width, scale),
        out::pixels(points.height, scale),
    ) else {
        return Err(format!(
            "no pixel size for {} x {} pt at scale {scale}",
            points.width, points.height
        ));
    };
    let rep = ffi::rgba_bitmap(width, height)?;
    rep.setSize(points);
    Ok(rep)
}

fn size_to_fit(view: &NSView) {
    let obj: &AnyObject = view.as_ref();
    if let Some(control) = obj.downcast_ref::<NSControl>() {
        control.sizeToFit();
    } else if let Some(progress) = obj.downcast_ref::<NSProgressIndicator>() {
        progress.sizeToFit();
    }
}

/// Measure a mounted control: sizes before and after `sizeToFit`, the
/// alignment insets, and its cell's rects for the final bounds.
fn measure(kind: Kind, view: &NSView) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "class",
        out::text(view.class().name().to_string_lossy()),
    );
    out::put(
        &mut t,
        "intrinsicContentSize_wh",
        size(view.intrinsicContentSize()),
    );
    out::put(&mut t, "fittingSize_wh", size(view.fittingSize()));
    size_to_fit(view);
    let fitted = view.frame();
    out::put(&mut t, "frame_after_sizeToFit_xywh", rect(fitted));
    out::put(
        &mut t,
        "intrinsicContentSize_after_sizeToFit_wh",
        size(view.intrinsicContentSize()),
    );
    if kind.stretches() && fitted.size.width < STRETCH_WIDTH {
        view.setFrameSize(NSSize::new(STRETCH_WIDTH, fitted.size.height));
        out::put(&mut t, "frame_widened_xywh", rect(view.frame()));
    }
    out::put(
        &mut t,
        "alignmentRectInsets_tlbr",
        insets(view.alignmentRectInsets()),
    );
    out::put(
        &mut t,
        "alignmentRectForFrame_xywh",
        rect(view.alignmentRectForFrame(view.frame())),
    );
    out::put(&mut t, "isFlipped", out::boolean(view.isFlipped()));

    let obj: &AnyObject = view.as_ref();
    if let Some(control) = obj.downcast_ref::<NSControl>() {
        out::put(&mut t, "cell", cell_table(control, view));
    }
    if let Some(slider) = obj.downcast_ref::<NSSlider>() {
        out::put(&mut t, "knobThickness", out::float(slider.knobThickness()));
    }
    t
}

/// The cell's sizes and rects for the control's bounds; the slider cell's
/// bar, knob and tick-mark rects.
fn cell_table(control: &NSControl, view: &NSView) -> Value {
    let Some(cell) = control.cell() else {
        return out::unavailable("no cell (not cell-based)");
    };
    let bounds = view.bounds();
    let mut c = Table::new();
    out::put(
        &mut c,
        "class",
        out::text(cell.class().name().to_string_lossy()),
    );
    out::put(&mut c, "bounds_xywh", rect(bounds));
    out::put(&mut c, "cellSize_wh", size(cell.cellSize()));
    out::put(
        &mut c,
        "cellSizeForBounds_wh",
        size(cell.cellSizeForBounds(bounds)),
    );
    out::put(
        &mut c,
        "drawingRectForBounds_xywh",
        rect(cell.drawingRectForBounds(bounds)),
    );
    out::put(
        &mut c,
        "titleRectForBounds_xywh",
        rect(cell.titleRectForBounds(bounds)),
    );
    out::put(
        &mut c,
        "imageRectForBounds_xywh",
        rect(cell.imageRectForBounds(bounds)),
    );
    let cell_obj: &AnyObject = cell.as_ref();
    if let Some(slider_cell) = cell_obj.downcast_ref::<NSSliderCell>() {
        let flipped = view.isFlipped();
        out::put(
            &mut c,
            "knobThickness",
            out::float(slider_cell.knobThickness()),
        );
        out::put(&mut c, "trackRect_xywh", rect(slider_cell.trackRect()));
        out::put(
            &mut c,
            "barRectFlipped_xywh",
            rect(slider_cell.barRectFlipped(flipped)),
        );
        out::put(
            &mut c,
            "knobRectFlipped_xywh",
            rect(slider_cell.knobRectFlipped(flipped)),
        );
        let ticks = slider_cell.numberOfTickMarks();
        out::put(&mut c, "numberOfTickMarks", out::int_isize(ticks));
        // rectOfTickMarkAtIndex: raises NSRangeException past the last tick
        // mark, so it is asked only when one exists.
        let tick0 = if ticks > 0 {
            rect(slider_cell.rectOfTickMarkAtIndex(0))
        } else {
            out::not_probed("no tick marks")
        };
        out::put(&mut c, "rectOfTickMarkAtIndex_0_xywh", tick0);
    }
    Value::Table(c)
}

/// Place `view` at the render margin and size its container around it.
fn fit_container(container: &NSView, view: &NSView) {
    let frame = view.frame();
    view.setFrameOrigin(NSPoint::new(RENDER_MARGIN, RENDER_MARGIN));
    container.setFrame(NSRect::new(
        NSPoint::new(0.0, 0.0),
        NSSize::new(
            frame.size.width + 2.0 * RENDER_MARGIN,
            frame.size.height + 2.0 * RENDER_MARGIN,
        ),
    ));
    container.layoutSubtreeIfNeeded();
}

fn size(s: NSSize) -> Value {
    out::floats(&[s.width, s.height])
}

fn rect(r: NSRect) -> Value {
    out::floats(&[r.origin.x, r.origin.y, r.size.width, r.size.height])
}

fn insets(i: NSEdgeInsets) -> Value {
    out::floats(&[i.top, i.left, i.bottom, i.right])
}

// ── A key window ────────────────────────────────────────────────────────────

/// The key window's content size, in points.
const KEY_WINDOW_SIZE: NSSize = NSSize::new(600.0, 400.0);

/// The sidebar material view's frame in the key window's content view, in
/// points (bottom-left origin).
const KEY_SIDEBAR_FRAME: NSRect = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(180.0, 400.0));

/// The table's scroll view's frame in the key window's content view.
const KEY_TABLE_FRAME: NSRect = NSRect::new(NSPoint::new(200.0, 160.0), NSSize::new(380.0, 220.0));

/// How far right of the table's left edge the header's sample point lies:
/// past the one `TABLE_COLUMN_WIDTH` column and its title, in the header's
/// empty part.
const KEY_HEADER_SAMPLE_INSET: f64 = 260.0;

/// Where the segmented control is put in the key window's content view.
const KEY_SEGMENTED_ORIGIN: NSPoint = NSPoint::new(200.0, 60.0);

/// The width given to each of the segmented control's two segments, so a
/// sample point clears its label.
const KEY_SEGMENT_WIDTH: f64 = 100.0;

/// How far in from a segment's leading edge its sample point lies, clear of
/// the label centred in the segment's width.
const KEY_SEGMENT_SAMPLE_INSET: f64 = 12.0;

/// A point of the content view no view covers: the window's own background.
const KEY_BACKGROUND_POINT: NSPoint = NSPoint::new(390.0, 120.0);

/// The bottom edge of the row of disabled controls in the key window's
/// content view, in the free band between the segmented control and the
/// table, left of `KEY_BACKGROUND_POINT`.
const KEY_DISABLED_ROW_Y: f64 = 96.0;

/// The bottom edge of the row of their enabled twins.
const KEY_ENABLED_ROW_Y: f64 = 128.0;

/// The left edge of each row's first control, the switch that is on; each
/// next control starts `KEY_CONTROL_GAP` right of the one before's fitted
/// frame (an `NSSwitch` fits to 42 pt on macOS 14 and 15 and 54 pt on 26
/// and 27, OS-probe run 40348854007, `switch_on_disabled_frame_xywh`).
const KEY_CONTROLS_X: f64 = 200.0;

/// The space between two controls of a row, in points.
const KEY_CONTROL_GAP: f64 = 8.0;

/// Where each row's slider ends, its width what is left of the row: left of
/// `KEY_BACKGROUND_POINT`, so no control covers it.
const KEY_CONTROLS_END_X: f64 = 380.0;

/// The slider's value, in its 0..1 range: the knob in the middle, the fill
/// left of it.
const KEY_SLIDER_VALUE: f64 = 0.5;

/// Where along a slider's bar its fill is sampled, as a share of the bar's
/// width: well left of the knob.
const KEY_SLIDER_FILL_AT: f64 = 0.25;

/// Where along a slider's bar its unfilled track is sampled: well right of
/// the knob.
const KEY_SLIDER_TRACK_AT: f64 = 0.75;

/// Where along a switch its track is sampled, as a share of its width in
/// from the end away from the knob: the middle of that third.
const KEY_SWITCH_TRACK_AT: f64 = 1.0 / 6.0;

/// The calibration strip's first patch, in the content view: a row below
/// the segmented control, clear of every other sample point.
const KEY_PATCH_ORIGIN: NSPoint = NSPoint::new(200.0, 20.0);

/// The size of a calibration patch, in points.
const KEY_PATCH_SIZE: NSSize = NSSize::new(24.0, 20.0);

/// The distance from one calibration patch's left edge to the next one's.
const KEY_PATCH_STEP: f64 = 30.0;

/// The calibration patches filled with an exact sRGB colour: the key and
/// the colour's `[r, g, b]` bytes.
const SRGB_PATCHES: [(&str, [u8; 3]); 9] = [
    ("srgb_000000", [0x00, 0x00, 0x00]),
    ("srgb_404040", [0x40, 0x40, 0x40]),
    ("srgb_808080", [0x80, 0x80, 0x80]),
    ("srgb_c0c0c0", [0xc0, 0xc0, 0xc0]),
    ("srgb_ececec", [0xec, 0xec, 0xec]),
    ("srgb_ffffff", [0xff, 0xff, 0xff]),
    ("srgb_ff0000", [0xff, 0x00, 0x00]),
    ("srgb_00ff00", [0x00, 0xff, 0x00]),
    ("srgb_0000ff", [0x00, 0x00, 0xff]),
];

/// The calibration patches filled with a dynamic system colour, resolved
/// under the window's appearance as it draws.
type SystemPatch = (&'static str, fn() -> Retained<NSColor>);

/// One step of the run-loop pump.
const PUMP_STEP: std::time::Duration = std::time::Duration::from_millis(50);

/// The most steps the pump waits for the application to become active after
/// each activation call.
const ACTIVATE_WAIT_STEPS: u32 = 20;

/// The most steps the pump waits for the window to become key.
const KEY_WAIT_STEPS: u32 = 100;

/// The steps the pump runs after a change of appearance, so the window
/// server holds the redrawn window before it is captured.
const SETTLE_STEPS: u32 = 10;

/// The second window `--not-key` makes key, in screen points: small, and
/// clear of the probe window's frame.
const NOT_KEY_FRAME: NSRect = NSRect::new(NSPoint::new(900.0, 200.0), NSSize::new(240.0, 120.0));

/// The most steps of `PUMP_STEP` a `screencapture` run is waited for.
const CAPTURE_WAIT_STEPS: u32 = 200;

/// A titled window made key in an activated, regular application: a
/// sidebar material view, a table with a header, a segmented control with
/// one segment selected and a strip of calibration patches, captured per
/// appearance with `screencapture` and sampled in each region, the
/// capture's own bytes read as they are. `cacheDisplayInRect:`, which the
/// control renders use, draws no material and no key-window state. Runs
/// after every other section, so their output is unchanged; each step that
/// fails is recorded as data.
fn key_window_table(
    mtm: MainThreadMarker,
    app: &NSApplication,
    out_dir: &Path,
    not_key: bool,
) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "method",
        out::text(format!(
            "activation policy set to Regular, finishLaunching, activateIgnoringOtherApps: YES, then -activate where the OS has it, each followed by a pump until NSApp isActive (up to {ACTIVATE_WAIT_STEPS} steps); a titled, closable, miniaturizable, resizable NSWindow ({} x {} pt content, empty title) made key with makeKeyAndOrderFront: and makeFirstResponder: nil; the run loop pumped with nextEventMatchingMask:untilDate:inMode:dequeue: and sendEvent: in {} ms steps, up to {KEY_WAIT_STEPS} steps until isKeyWindow, activateIgnoringOtherApps: YES again at each step NSApp is not active; content: an NSVisualEffectView, material sidebar, blending behindWindow, state active, at {}; an NSScrollView with an NSTableView of one {TABLE_COLUMN_WIDTH} pt column titled \"Column\" and its header, at {}; an NSSegmentedControl of two {KEY_SEGMENT_WIDTH} pt segments \"One\" \"Two\", segment 0 selected, sized to fit at {}; a calibration strip of {} x {} pt NSBox patches (custom, no border, no title) from {} every {KEY_PATCH_STEP} pt, filled with colorWithSRGBRed:green:blue:alpha: of the srgb_ patches' bytes and with the dynamic system colours; per appearance the window's appearance is set, the run loop pumped {SETTLE_STEPS} steps, and the window captured with screencapture -x -o -l <windowNumber>; each sample is the capture's pixel at a point given in points from the window frame's top left (sample_points_xy): raw, the PNG's own bytes from bitmapData as #rrggbbaa (alpha ff where the rep has none), and colorAtXY_sRGB, colorAtX:y: converted with colorUsingColorSpace: sRGB",
            KEY_WINDOW_SIZE.width,
            KEY_WINDOW_SIZE.height,
            PUMP_STEP.as_millis(),
            rect_text(KEY_SIDEBAR_FRAME),
            rect_text(KEY_TABLE_FRAME),
            point_text(KEY_SEGMENTED_ORIGIN),
            KEY_PATCH_SIZE.width,
            KEY_PATCH_SIZE.height,
            point_text(KEY_PATCH_ORIGIN),
        )),
    );

    out::put(
        &mut t,
        "activationPolicy_regular_set",
        out::boolean(app.setActivationPolicy(NSApplicationActivationPolicy::Regular)),
    );
    app.finishLaunching();
    out::put_table(&mut t, "activation", activation_table(app));

    let frame = NSRect::new(NSPoint::new(200.0, 200.0), KEY_WINDOW_SIZE);
    let window = match ffi::titled_window(mtm, frame) {
        Ok(window) => window,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    window.setTitle(&ns(""));
    let Some(content) = window.contentView() else {
        out::put(
            &mut t,
            "error",
            out::unavailable("the titled window has no content view"),
        );
        return t;
    };

    let sidebar =
        NSVisualEffectView::initWithFrame(NSVisualEffectView::alloc(mtm), KEY_SIDEBAR_FRAME);
    sidebar.setMaterial(NSVisualEffectMaterial::Sidebar);
    sidebar.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
    sidebar.setState(NSVisualEffectState::Active);
    content.addSubview(&sidebar);

    let scroll = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), KEY_TABLE_FRAME);
    let table_view = NSTableView::new(mtm);
    let column = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), &ns("column"));
    column.setTitle(&ns("Column"));
    column.setWidth(TABLE_COLUMN_WIDTH);
    table_view.addTableColumn(&column);
    scroll.setDocumentView(Some(&table_view));
    content.addSubview(&scroll);
    scroll.tile();
    table_view.tile();

    let segmented = NSSegmentedControl::new(mtm);
    segmented.setSegmentCount(2);
    segmented.setLabel_forSegment(&ns("One"), 0);
    segmented.setLabel_forSegment(&ns("Two"), 1);
    segmented.setWidth_forSegment(KEY_SEGMENT_WIDTH, 0);
    segmented.setWidth_forSegment(KEY_SEGMENT_WIDTH, 1);
    segmented.setSelectedSegment(0);
    segmented.sizeToFit();
    segmented.setFrameOrigin(KEY_SEGMENTED_ORIGIN);
    content.addSubview(&segmented);

    let rows = [
        (KEY_DISABLED_ROW_Y, false, "_disabled"),
        (KEY_ENABLED_ROW_Y, true, ""),
    ];
    let mut switches: Vec<(String, bool, Retained<NSSwitch>)> = Vec::new();
    let mut sliders: Vec<(String, Retained<NSSlider>)> = Vec::new();
    for (y, enabled, suffix) in rows {
        let mut x = KEY_CONTROLS_X;
        for (on, name) in [(true, "switch_on"), (false, "switch_off")] {
            let switch = NSSwitch::new(mtm);
            switch.setState(state_value(on));
            switch.setEnabled(enabled);
            switch.sizeToFit();
            switch.setFrameOrigin(NSPoint::new(x, y));
            content.addSubview(&switch);
            x += switch.frame().size.width + KEY_CONTROL_GAP;
            switches.push((format!("{name}{suffix}"), on, switch));
        }
        let slider = NSSlider::new(mtm);
        slider.setVertical(false);
        slider.setMinValue(0.0);
        slider.setMaxValue(1.0);
        slider.setDoubleValue(KEY_SLIDER_VALUE);
        slider.setEnabled(enabled);
        slider.sizeToFit();
        let height = slider.frame().size.height;
        slider.setFrame(NSRect::new(
            NSPoint::new(x, y),
            NSSize::new((KEY_CONTROLS_END_X - x).max(0.0), height),
        ));
        content.addSubview(&slider);
        sliders.push((format!("slider{suffix}"), slider));
    }

    let system_patches: [SystemPatch; 2] = [
        ("windowBackgroundColor", NSColor::windowBackgroundColor),
        ("controlBackgroundColor", NSColor::controlBackgroundColor),
    ];
    let mut patches: Vec<(&str, NSPoint)> = Vec::new();
    let mut patch_x = KEY_PATCH_ORIGIN.x;
    let fills = SRGB_PATCHES
        .iter()
        .map(|(key, [r, g, b])| {
            let color = NSColor::colorWithSRGBRed_green_blue_alpha(
                f64::from(*r) / 255.0,
                f64::from(*g) / 255.0,
                f64::from(*b) / 255.0,
                1.0,
            );
            (*key, color)
        })
        .chain(system_patches.iter().map(|(key, make)| (*key, make())));
    for (key, color) in fills {
        let patch_frame = NSRect::new(NSPoint::new(patch_x, KEY_PATCH_ORIGIN.y), KEY_PATCH_SIZE);
        let patch = NSBox::initWithFrame(NSBox::alloc(mtm), patch_frame);
        patch.setBoxType(NSBoxType::Custom);
        patch.setTitlePosition(NSTitlePosition::NoTitle);
        patch.setBorderWidth(0.0);
        patch.setFillColor(&color);
        content.addSubview(&patch);
        patches.push((
            key,
            NSPoint::new(
                patch_x + KEY_PATCH_SIZE.width / 2.0,
                KEY_PATCH_ORIGIN.y + KEY_PATCH_SIZE.height / 2.0,
            ),
        ));
        patch_x += KEY_PATCH_STEP;
    }

    window.makeKeyAndOrderFront(None);
    out::put(
        &mut t,
        "makeFirstResponder_nil",
        out::boolean(window.makeFirstResponder(None)),
    );
    let reactivations = std::cell::Cell::new(0_u32);
    let steps = pump(app, KEY_WAIT_STEPS, || {
        if !app.isActive() {
            activate_ignoring(app);
            reactivations.set(reactivations.get().saturating_add(1));
        }
        window.isKeyWindow()
    });
    out::put(&mut t, "isKeyWindow", out::boolean(window.isKeyWindow()));
    out::put(&mut t, "isMainWindow", out::boolean(window.isMainWindow()));
    out::put(&mut t, "NSApp_isActive", out::boolean(app.isActive()));
    out::put(&mut t, "key_wait_steps", out::int(i64::from(steps)));
    out::put(
        &mut t,
        "key_wait_reactivations",
        out::int(i64::from(reactivations.get())),
    );
    let other = not_key.then(|| not_key_window(mtm, app, &window, &mut t));
    let other = other.flatten();
    out::put(
        &mut t,
        "accessibilityDisplayShouldReduceTransparency",
        out::boolean(NSWorkspace::sharedWorkspace().accessibilityDisplayShouldReduceTransparency()),
    );
    out::put(
        &mut t,
        "sidebar_state",
        out::text(effect_state_name(sidebar.state())),
    );
    out::put(
        &mut t,
        "window_backingScaleFactor",
        out::float(window.backingScaleFactor()),
    );
    let window_frame = window.frame();
    out::put(&mut t, "window_frame_xywh", rect(window_frame));
    let content_height = window.contentRectForFrameRect(window_frame).size.height;
    let title_bar = window_frame.size.height - content_height;
    out::put(&mut t, "titleBar_height", out::float(title_bar));

    let header_height = table_view
        .headerView()
        .map(|header| header.frame().size.height);
    match header_height {
        Some(height) => out::put(&mut t, "headerView_height", out::float(height)),
        None => out::put(
            &mut t,
            "headerView_height",
            out::unavailable("headerView is nil"),
        ),
    }
    out::put(&mut t, "column_width", out::float(column.width()));
    let segmented_frame = segmented.frame();
    out::put(&mut t, "segmentedControl_frame_xywh", rect(segmented_frame));
    out::put(
        &mut t,
        "method_controls",
        out::text(format!(
            "in the band between the segmented control and the table: a row of disabled controls (setEnabled: NO) with its bottom edge at y {KEY_DISABLED_ROW_Y} and a row of the same controls enabled at y {KEY_ENABLED_ROW_Y}, from x {KEY_CONTROLS_X}, each control {KEY_CONTROL_GAP} pt right of the one before's fitted frame: an NSSwitch on and an NSSwitch off (both sized to fit) and an NSSlider, horizontal, 0..1 at {KEY_SLIDER_VALUE}, sized to fit in height and as wide as the row leaves up to x {KEY_CONTROLS_END_X}; a switch's track is sampled at {KEY_SWITCH_TRACK_AT} of its width in from the end away from the knob, and its knob at a point inside the knob, half the frame's height in from the end it rests at (the right end when on; the knob's centre only where it is round), both at the frame's vertical centre; a slider's fill at {KEY_SLIDER_FILL_AT} and its track at {KEY_SLIDER_TRACK_AT} of the bar's width (barRectFlipped: of its NSSliderCell, in the content view; the frame where the cell is not one), at the bar's vertical centre, and its knob at the centre of knobRectFlipped: (the bar's centre where the cell is not an NSSliderCell); where a disabled slider draws no value fill (macOS 14, 15 and 26 in run 40348854007), slider_disabled_fill samples the bar"
        )),
    );
    let mut control_points: Vec<(String, NSPoint)> = Vec::new();
    for (name, on, switch) in &switches {
        let f = switch.frame();
        out::put(&mut t, &format!("{name}_frame_xywh"), rect(f));
        let mid = f.origin.y + f.size.height / 2.0;
        let (track_x, knob_x) = if *on {
            (
                f.origin.x + f.size.width * KEY_SWITCH_TRACK_AT,
                f.origin.x + f.size.width - f.size.height / 2.0,
            )
        } else {
            (
                f.origin.x + f.size.width * (1.0 - KEY_SWITCH_TRACK_AT),
                f.origin.x + f.size.height / 2.0,
            )
        };
        control_points.push((name.clone(), NSPoint::new(track_x, mid)));
        control_points.push((format!("{name}_knob"), NSPoint::new(knob_x, mid)));
    }
    for (name, slider) in &sliders {
        let f = slider.frame();
        out::put(&mut t, &format!("{name}_frame_xywh"), rect(f));
        let cell = slider.cell();
        let slider_cell = cell.as_ref().and_then(|cell| {
            let obj: &AnyObject = cell.as_ref();
            obj.downcast_ref::<NSSliderCell>()
        });
        let flipped = slider.isFlipped();
        let (bar, knob) = match slider_cell {
            Some(c) => {
                let bar = slider.convertRect_toView(c.barRectFlipped(flipped), Some(&content));
                let knob = slider.convertRect_toView(c.knobRectFlipped(flipped), Some(&content));
                out::put(&mut t, &format!("{name}_barRect_xywh"), rect(bar));
                out::put(&mut t, &format!("{name}_knobRect_xywh"), rect(knob));
                (bar, Some(knob))
            }
            None => {
                out::put(
                    &mut t,
                    &format!("{name}_barRect_xywh"),
                    out::unavailable("the slider's cell is not an NSSliderCell"),
                );
                (f, None)
            }
        };
        let mid = bar.origin.y + bar.size.height / 2.0;
        control_points.push((
            format!("{name}_fill"),
            NSPoint::new(bar.origin.x + bar.size.width * KEY_SLIDER_FILL_AT, mid),
        ));
        control_points.push((
            format!("{name}_track"),
            NSPoint::new(bar.origin.x + bar.size.width * KEY_SLIDER_TRACK_AT, mid),
        ));
        let knob_centre = match knob {
            Some(k) => NSPoint::new(
                k.origin.x + k.size.width / 2.0,
                k.origin.y + k.size.height / 2.0,
            ),
            None => NSPoint::new(bar.origin.x + bar.size.width / 2.0, mid),
        };
        control_points.push((format!("{name}_knob"), knob_centre));
    }

    // A point of the content view (bottom-left origin) in points from the
    // window frame's top left: the content view is the frame's bottom part.
    let from_content = |p: NSPoint| NSPoint::new(p.x, window_frame.size.height - p.y);
    let segment_y = segmented_frame.origin.y + segmented_frame.size.height / 2.0;
    let mut points: Vec<(&str, NSPoint)> = vec![
        (
            "titleBar",
            NSPoint::new(window_frame.size.width / 2.0, title_bar / 2.0),
        ),
        (
            "sidebar",
            from_content(NSPoint::new(
                KEY_SIDEBAR_FRAME.origin.x + KEY_SIDEBAR_FRAME.size.width / 2.0,
                KEY_SIDEBAR_FRAME.origin.y + KEY_SIDEBAR_FRAME.size.height / 2.0,
            )),
        ),
        (
            "segment_selected",
            from_content(NSPoint::new(
                segmented_frame.origin.x + KEY_SEGMENT_SAMPLE_INSET,
                segment_y,
            )),
        ),
        (
            "segment_unselected",
            from_content(NSPoint::new(
                segmented_frame.origin.x + KEY_SEGMENT_WIDTH + KEY_SEGMENT_SAMPLE_INSET,
                segment_y,
            )),
        ),
        ("windowBackground", from_content(KEY_BACKGROUND_POINT)),
    ];
    if let Some(height) = header_height {
        points.push((
            "listHeader",
            from_content(NSPoint::new(
                KEY_TABLE_FRAME.origin.x + KEY_HEADER_SAMPLE_INSET,
                KEY_TABLE_FRAME.origin.y + KEY_TABLE_FRAME.size.height - height / 2.0,
            )),
        ));
    }
    points.extend(
        control_points
            .iter()
            .map(|(name, p)| (name.as_str(), from_content(*p))),
    );
    let patches: Vec<(&str, NSPoint)> = patches
        .into_iter()
        .map(|(key, p)| (key, from_content(p)))
        .collect();
    let mut sample_points = Table::new();
    for (name, p) in points.iter().chain(patches.iter()) {
        out::put(&mut sample_points, name, out::floats(&[p.x, p.y]));
    }
    out::put_table(&mut t, "sample_points_xy", sample_points);

    let dir = out_dir.join("renders").join("key_window");
    let dir_made =
        std::fs::create_dir_all(&dir).map_err(|e| format!("cannot create {}: {e}", dir.display()));
    for (key, name) in APPEARANCES {
        let Some(appearance) = NSAppearance::appearanceNamed(&ns(name)) else {
            out::put(
                &mut t,
                key,
                out::unavailable(format!("appearanceNamed({name}) is nil")),
            );
            continue;
        };
        window.setAppearance(Some(&appearance));
        content.setNeedsDisplay(true);
        window.displayIfNeeded();
        pump(app, SETTLE_STEPS, || false);
        let mut run = Table::new();
        out::put(&mut run, "isKeyWindow", out::boolean(window.isKeyWindow()));
        out::put(&mut run, "NSApp_isActive", out::boolean(app.isActive()));
        if let Some(other) = &other {
            out::put(
                &mut run,
                "isMainWindow",
                out::boolean(window.isMainWindow()),
            );
            out::put(
                &mut run,
                "other_window_isKeyWindow",
                out::boolean(other.isKeyWindow()),
            );
        }
        let expected = system_expected(&appearance, &system_patches);
        match &dir_made {
            Ok(()) => capture(
                &mut run,
                &window,
                &dir,
                key,
                window_frame.size,
                &points,
                &patches,
                &expected,
            ),
            Err(reason) => out::put(&mut run, "capture", out::unavailable(reason)),
        }
        out::put_table(&mut t, key, run);
    }
    window.setAppearance(None);
    window.orderOut(None);
    window.close();
    if let Some(other) = other {
        other.orderOut(None);
        other.close();
    }
    t
}

/// `--not-key`: a second, small titled window made key and front, and the
/// run loop pumped until the probe's `window` reports it is no longer key
/// (up to `KEY_WAIT_STEPS` steps), recorded under `not_key` in `t`; the
/// second window, which the captures report on, or `None` where it could
/// not be made.
fn not_key_window(
    mtm: MainThreadMarker,
    app: &NSApplication,
    window: &NSWindow,
    t: &mut Table,
) -> Option<Retained<NSWindow>> {
    let mut nk = Table::new();
    out::put(&mut nk, "not_key_requested", out::boolean(true));
    out::put(&mut nk, "other_window_frame_xywh", rect(NOT_KEY_FRAME));
    let other = match ffi::titled_window(mtm, NOT_KEY_FRAME) {
        Ok(other) => other,
        Err(reason) => {
            out::put(&mut nk, "other_window", out::unavailable(reason));
            out::put_table(t, "not_key", nk);
            return None;
        }
    };
    other.setTitle(&ns("Other"));
    other.makeKeyAndOrderFront(None);
    let steps = pump(app, KEY_WAIT_STEPS, || {
        if !app.isActive() {
            activate_ignoring(app);
        }
        !window.isKeyWindow()
    });
    out::put(&mut nk, "wait_steps", out::int(i64::from(steps)));
    out::put(&mut nk, "isKeyWindow", out::boolean(window.isKeyWindow()));
    out::put(&mut nk, "isMainWindow", out::boolean(window.isMainWindow()));
    out::put(&mut nk, "NSApp_isActive", out::boolean(app.isActive()));
    out::put(
        &mut nk,
        "other_window_isKeyWindow",
        out::boolean(other.isKeyWindow()),
    );
    out::put_table(t, "not_key", nk);
    Some(other)
}

/// Activate `app`: `activateIgnoringOtherApps:` first, on every macOS, as
/// winit does (winit 0.30.13 `src/platform_impl/macos/app_state.rs:137`),
/// then `-activate` where the OS has it (macOS 14), each followed by a pump
/// until `NSApp` is active; which call made it active.
fn activation_table(app: &NSApplication) -> Table {
    let mut t = Table::new();
    activate_ignoring(app);
    let ignoring_steps = pump(app, ACTIVATE_WAIT_STEPS, || app.isActive());
    let after_ignoring = app.isActive();
    out::put(
        &mut t,
        "activateIgnoringOtherApps_isActive",
        out::boolean(after_ignoring),
    );
    out::put(
        &mut t,
        "activateIgnoringOtherApps_steps",
        out::int(i64::from(ignoring_steps)),
    );
    let obj: &AnyObject = app.as_ref();
    let has_activate = obj.class().responds_to(sel!(activate));
    out::put(&mut t, "activate_available", out::boolean(has_activate));
    let after_activate = if has_activate {
        app.activate();
        let steps = pump(app, ACTIVATE_WAIT_STEPS, || app.isActive());
        out::put(&mut t, "activate_steps", out::int(i64::from(steps)));
        app.isActive()
    } else {
        after_ignoring
    };
    out::put(&mut t, "activate_isActive", out::boolean(after_activate));
    let by = if after_ignoring {
        "activateIgnoringOtherApps:"
    } else if after_activate {
        "activate"
    } else {
        "none"
    };
    out::put(&mut t, "activated_by", out::text(by));
    t
}

/// `-[NSApplication activateIgnoringOtherApps:]` with YES, deprecated since
/// macOS 14 and still the call that activates an application started from a
/// shell (winit 0.30.13 `src/platform_impl/macos/app_state.rs:137`).
fn activate_ignoring(app: &NSApplication) {
    #[allow(deprecated)]
    app.activateIgnoringOtherApps(true);
}

/// Pump the run loop up to `steps` steps of `PUMP_STEP`, stopping early once
/// `step` returns true; it runs before each step. The steps taken.
fn pump(app: &NSApplication, steps: u32, mut step: impl FnMut() -> bool) -> u32 {
    let mode = ffi::default_run_loop_mode();
    for taken in 0..steps {
        if step() {
            return taken;
        }
        let until = NSDate::dateWithTimeIntervalSinceNow(PUMP_STEP.as_secs_f64());
        while let Some(event) = app.nextEventMatchingMask_untilDate_inMode_dequeue(
            NSEventMask::Any,
            Some(&until),
            mode,
            true,
        ) {
            app.sendEvent(&event);
        }
        app.updateWindows();
    }
    steps
}

fn effect_state_name(state: NSVisualEffectState) -> String {
    if state == NSVisualEffectState::FollowsWindowActiveState {
        "followsWindowActiveState".to_owned()
    } else if state == NSVisualEffectState::Active {
        "active".to_owned()
    } else if state == NSVisualEffectState::Inactive {
        "inactive".to_owned()
    } else {
        format!("unknown ({})", state.0)
    }
}

/// What each calibration patch should read: the `srgb_` patches their
/// bytes, the system patches their colour resolved under `appearance`, in
/// sRGB.
fn system_expected(appearance: &NSAppearance, system: &[SystemPatch]) -> Vec<(String, String)> {
    let mut expected: Vec<(String, String)> = SRGB_PATCHES
        .iter()
        .map(|(key, [r, g, b])| ((*key).to_owned(), format!("#{r:02x}{g:02x}{b:02x}ff")))
        .collect();
    let resolved = RefCell::new(Vec::new());
    {
        let block = RcBlock::new(|| {
            let srgb = NSColorSpace::sRGBColorSpace();
            for (key, make) in system {
                let value = match color_value(&make(), &srgb) {
                    Value::String(text) => text,
                    other => other.to_string(),
                };
                resolved.borrow_mut().push(((*key).to_owned(), value));
            }
        });
        appearance.performAsCurrentDrawingAppearance(&block);
    }
    expected.extend(resolved.into_inner());
    expected
}

/// Capture `window` to `<dir>/<key>.png` with `screencapture` and sample it
/// at `points` and at the calibration `patches`, given in points from the
/// window frame's top left; each patch's sample is checked against what it
/// is filled with (`expected`).
#[allow(clippy::too_many_arguments)]
fn capture(
    run: &mut Table,
    window: &NSWindow,
    dir: &Path,
    key: &str,
    frame_points: NSSize,
    points: &[(&str, NSPoint)],
    patches: &[(&str, NSPoint)],
    expected: &[(String, String)],
) {
    let path = dir.join(format!("{key}.png"));
    if let Err(reason) = screencapture(window.windowNumber(), &path) {
        out::put(run, "capture", out::unavailable(reason));
        return;
    }
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(e) => {
            out::put(
                run,
                "capture",
                out::unavailable(format!("cannot read {}: {e}", path.display())),
            );
            return;
        }
    };
    out::put(
        run,
        "file",
        out::text(format!("renders/key_window/{key}.png")),
    );
    let data = NSData::with_bytes(&bytes);
    let Some(rep) = NSBitmapImageRep::imageRepWithData(&data) else {
        out::put(
            run,
            "capture",
            out::unavailable("the capture is not a bitmap image"),
        );
        return;
    };
    let (wide, high) = (rep.pixelsWide(), rep.pixelsHigh());
    out::put(
        run,
        "pixels_wh",
        out::array(vec![out::int_isize(wide), out::int_isize(high)]),
    );
    out::put_table(run, "pixel_format", pixel_format_table(&rep));
    // Pixels per point, from the capture itself: without its shadow (-o)
    // the window frame is the whole image.
    let ratio = pixel_ratio(wide, frame_points.width);
    let srgb = NSColorSpace::sRGBColorSpace();
    let sample = |p: &NSPoint| -> (Table, Option<String>) {
        let mut s = Table::new();
        let (Some(x), Some(y)) = (pixel_index(p.x, ratio, wide), pixel_index(p.y, ratio, high))
        else {
            out::put(
                &mut s,
                "raw",
                out::unavailable(format!(
                    "{} x {} pt lies outside the {wide} x {high} px capture",
                    p.x, p.y
                )),
            );
            return (s, None);
        };
        out::put(
            &mut s,
            "pixel_xy",
            out::array(vec![out::int_isize(x), out::int_isize(y)]),
        );
        let raw = ffi::pixel_bytes(&rep, x, y).and_then(|b| rgba_text(&b, rep.bitmapFormat()));
        let raw_text = raw.as_ref().ok().cloned();
        out::put(&mut s, "raw", raw.map_or_else(out::unavailable, out::text));
        let converted = match rep.colorAtX_y(x, y) {
            Some(color) => color_value(&color, &srgb),
            None => out::unavailable(format!("colorAtX:{x} y:{y} is nil")),
        };
        out::put(&mut s, "colorAtXY_sRGB", converted);
        (s, raw_text)
    };
    let mut samples = Table::new();
    for (name, p) in points {
        out::put_table(&mut samples, name, sample(p).0);
    }
    out::put_table(run, "samples", samples);
    let mut calibration = Table::new();
    for (name, p) in patches {
        let (mut s, raw) = sample(p);
        let want = expected
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.clone());
        match want {
            Some(want) => {
                out::put(
                    &mut s,
                    "exact",
                    out::boolean(raw.as_deref() == Some(want.as_str())),
                );
                out::put(&mut s, "expected", out::text(want));
            }
            None => out::put(&mut s, "expected", out::unavailable("no expected value")),
        }
        out::put_table(&mut calibration, name, s);
    }
    out::put_table(run, "calibration", calibration);
}

/// Capture the window numbered `window_number`, without its shadow, to
/// `path` with `screencapture`; `Err` says why it failed.
fn screencapture(window_number: isize, path: &Path) -> Result<(), String> {
    let child = std::process::Command::new("screencapture")
        .arg("-x")
        .arg("-o")
        .arg("-l")
        .arg(window_number.to_string())
        .arg(path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    match child {
        Ok(child) => wait_or_kill(child).map_err(|reason| format!("screencapture {reason}")),
        Err(e) => Err(format!("cannot run screencapture: {e}")),
    }
}

/// How the capture's bytes are laid out, which `raw` reads them by, and the
/// colour space they are tagged with.
fn pixel_format_table(rep: &NSBitmapImageRep) -> Table {
    let mut t = Table::new();
    let space = rep.colorSpace();
    out::put(
        &mut t,
        "colorSpace_localizedName",
        space
            .localizedName()
            .map_or_else(|| out::unavailable("nil"), |n| out::text(n.to_string())),
    );
    out::put(
        &mut t,
        "colorSpaceName",
        out::text(rep.colorSpaceName().to_string()),
    );
    let format = rep.bitmapFormat();
    let mut flags = Vec::new();
    for (flag, name) in [
        (NSBitmapFormat::AlphaFirst, "alphaFirst"),
        (
            NSBitmapFormat::AlphaNonpremultiplied,
            "alphaNonpremultiplied",
        ),
        (NSBitmapFormat::FloatingPointSamples, "floatingPointSamples"),
    ] {
        if format.contains(flag) {
            flags.push(out::text(name));
        }
    }
    out::put(&mut t, "bitmapFormat_flags", out::array(flags));
    out::put(&mut t, "bitsPerSample", out::int_isize(rep.bitsPerSample()));
    out::put(
        &mut t,
        "samplesPerPixel",
        out::int_isize(rep.samplesPerPixel()),
    );
    out::put(&mut t, "bitsPerPixel", out::int_isize(rep.bitsPerPixel()));
    out::put(&mut t, "bytesPerRow", out::int_isize(rep.bytesPerRow()));
    out::put(&mut t, "hasAlpha", out::boolean(rep.hasAlpha()));
    out::put(&mut t, "isPlanar", out::boolean(rep.isPlanar()));
    t
}

/// A pixel's 8-bit samples as `#rrggbbaa`, in the order `format` lays them
/// out: alpha first or last, or none (`ff`). Premultiplied samples are given
/// as stored.
fn rgba_text(bytes: &[u8], format: NSBitmapFormat) -> Result<String, String> {
    let alpha_first = format.contains(NSBitmapFormat::AlphaFirst);
    let (r, g, b, a) = match (bytes, alpha_first) {
        ([a, r, g, b], true) => (*r, *g, *b, *a),
        ([r, g, b, a], false) => (*r, *g, *b, *a),
        ([r, g, b], _) => (*r, *g, *b, 0xff),
        _ => return Err(format!("{} samples per pixel", bytes.len())),
    };
    Ok(format!("#{r:02x}{g:02x}{b:02x}{a:02x}"))
}

/// Wait for `child` up to `CAPTURE_WAIT_STEPS` steps of `PUMP_STEP`, then
/// kill it: a capture the OS holds back, for a permission prompt no one
/// answers on a runner, must not hold the probe. `Err` says why it failed.
fn wait_or_kill(mut child: std::process::Child) -> Result<(), String> {
    for _ in 0..CAPTURE_WAIT_STEPS {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => return Ok(()),
            Ok(Some(status)) => return Err(format!("exited with {status}")),
            Ok(None) => std::thread::sleep(PUMP_STEP),
            Err(e) => return Err(format!("could not be waited for: {e}")),
        }
    }
    let killed = child.kill().map_or_else(
        |e| format!(", and could not be killed: {e}"),
        |()| String::new(),
    );
    // Reap it, so no zombie outlives the probe; its status says nothing more.
    let _ = child.wait();
    Err(format!(
        "did not finish within {CAPTURE_WAIT_STEPS} steps of {} ms{killed}",
        PUMP_STEP.as_millis()
    ))
}

/// The capture's pixels per point across `points` of window width, `None`
/// unless finite and positive.
fn pixel_ratio(pixels: isize, points: f64) -> Option<f64> {
    #[allow(clippy::cast_precision_loss)]
    let ratio = pixels as f64 / points;
    (ratio.is_finite() && ratio > 0.0).then_some(ratio)
}

/// The pixel holding the point `points` along an axis of `limit` pixels at
/// `ratio` pixels per point, `None` outside the image.
fn pixel_index(points: f64, ratio: Option<f64>, limit: isize) -> Option<isize> {
    let px = (points * ratio?).floor();
    #[allow(clippy::cast_precision_loss)]
    let inside = px.is_finite() && px >= 0.0 && px < limit as f64;
    if !inside {
        return None;
    }
    // In 0.0..limit, so the cast is exact.
    #[allow(clippy::cast_possible_truncation)]
    let index = px as isize;
    Some(index)
}

fn rect_text(r: NSRect) -> String {
    format!(
        "[{}, {}, {}, {}] (x, y, width, height)",
        r.origin.x, r.origin.y, r.size.width, r.size.height
    )
}

fn point_text(p: NSPoint) -> String {
    format!("[{}, {}] (x, y)", p.x, p.y)
}
