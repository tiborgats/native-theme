//! A selected table row (`[macos.selection]`): a key titled window holding a
//! view-based NSTableView whose middle row is selected, each column a text
//! field drawing solid blocks in one label colour, captured per appearance
//! with the table first responder (the row emphasized) and without (not
//! emphasized), and sampled on the glyphs, beside them and across the
//! selected row's bottom edge. NSColor.h says the label colours resolve
//! differently in an emphasized background style; this reads what they draw.

use std::cell::RefCell;
use std::path::Path;

use block2::RcBlock;
use objc2::MainThreadMarker;
use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSApplication, NSBackgroundStyle, NSColor,
    NSColorSpace, NSFont, NSResponder, NSScrollView, NSTableCellView, NSTableColumn, NSTableView,
    NSTableViewGridLineStyle, NSTextField, NSView,
};
use objc2_foundation::{NSIndexSet, NSPoint, NSRect, NSSize, NSString};
use toml::{Table, Value};

use super::{
    APPEARANCES, KEY_WAIT_STEPS, SETTLE_STEPS, activate_ignoring, capture, color_value, ffi, ns,
    point_text, pump, rect, rect_text, table_style_name,
};
use crate::out;

/// The window's content frame on the screen, in points.
const SELECTION_FRAME: NSRect = NSRect::new(NSPoint::new(40.0, 240.0), NSSize::new(940.0, 160.0));

/// The table's scroll view in the content view.
const SELECTION_TABLE_FRAME: NSRect =
    NSRect::new(NSPoint::new(20.0, 20.0), NSSize::new(900.0, 120.0));

/// Each row's height, in points.
const ROW_HEIGHT: f64 = 32.0;

/// Each column's width, in points: wider than the field, so the row's
/// background shows between the field and the cell's right edge.
const COLUMN_WIDTH: f64 = 80.0;

/// The rows; the middle one is selected.
const ROWS: usize = 3;

/// The selected row.
const SELECTED_ROW: usize = 1;

/// The selected row as an NSInteger row index.
const SELECTED_ROW_INDEX: isize = 1;

/// The text field's left edge in its cell, in points.
const FIELD_INSET: f64 = 4.0;

/// The point size of the text fields' font: blocks large enough that their
/// middle is a whole pixel of the text colour.
const GLYPH_SIZE: f64 = 20.0;

/// What each text field shows: three full blocks (U+2588), so the middle of
/// the middle one is covered by the glyph alone.
const GLYPHS: &str = "\u{2588}\u{2588}\u{2588}";

/// The offsets across the selected row's bottom edge (its row rect's lower
/// side in the window) at which `r1_bottom_edge_NN` are sampled, in points
/// up from the edge.
const EDGE_STRIP_PT: [f64; 13] = [
    -3.0, -2.5, -2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0,
];

/// The column the edge strip is sampled in, at its background point.
const EDGE_STRIP_COLUMN: usize = 2;

/// A column: its identifier, the text colour, and whether the field is
/// enabled.
type Column = (&'static str, fn() -> Retained<NSColor>, bool);

/// The columns: the label colours, the link, separator and grid colours,
/// and disabled fields in the first two label colours.
const COLUMNS: [Column; 9] = [
    ("labelColor", NSColor::labelColor, true),
    ("secondaryLabelColor", NSColor::secondaryLabelColor, true),
    ("tertiaryLabelColor", NSColor::tertiaryLabelColor, true),
    ("quaternaryLabelColor", NSColor::quaternaryLabelColor, true),
    ("linkColor", NSColor::linkColor, true),
    ("separatorColor", NSColor::separatorColor, true),
    ("gridColor", NSColor::gridColor, true),
    ("labelColor_disabled", NSColor::labelColor, false),
    (
        "secondaryLabelColor_disabled",
        NSColor::secondaryLabelColor,
        false,
    ),
];

/// A colour class method of NSColor, by name.
type NamedColor = (&'static str, fn() -> Retained<NSColor>);

/// The colours resolved under each appearance for reference, as NSColor
/// reports them outside any background style: the columns' and the ones
/// NSColor.h names for selected content.
const REFERENCE: [NamedColor; 11] = [
    ("labelColor", NSColor::labelColor),
    ("secondaryLabelColor", NSColor::secondaryLabelColor),
    ("tertiaryLabelColor", NSColor::tertiaryLabelColor),
    ("quaternaryLabelColor", NSColor::quaternaryLabelColor),
    ("linkColor", NSColor::linkColor),
    ("separatorColor", NSColor::separatorColor),
    ("gridColor", NSColor::gridColor),
    (
        "alternateSelectedControlTextColor",
        NSColor::alternateSelectedControlTextColor,
    ),
    (
        "selectedContentBackgroundColor",
        NSColor::selectedContentBackgroundColor,
    ),
    (
        "unemphasizedSelectedContentBackgroundColor",
        NSColor::unemphasizedSelectedContentBackgroundColor,
    ),
    (
        "disabledControlTextColor",
        NSColor::disabledControlTextColor,
    ),
];

/// The two conditions each appearance is captured in: the key, and whether
/// the table is made first responder.
const CONDITIONS: [(&str, bool); 2] = [("emphasized", true), ("unemphasized", false)];

/// `[macos.selection]`; runs after the key-window section, with the
/// application regular and active.
pub(super) fn selection_table(mtm: MainThreadMarker, app: &NSApplication, out_dir: &Path) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "method", out::text(method_text()));
    let window = match ffi::titled_window(mtm, SELECTION_FRAME) {
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

    let scroll = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), SELECTION_TABLE_FRAME);
    let table = NSTableView::new(mtm);
    table.setHeaderView(None);
    table.setRowHeight(ROW_HEIGHT);
    table.setGridStyleMask(NSTableViewGridLineStyle::SolidHorizontalGridLineMask);
    let mut ids: Vec<Retained<NSString>> = Vec::new();
    for (id, _, _) in COLUMNS {
        let column = NSTableColumn::initWithIdentifier(NSTableColumn::alloc(mtm), &ns(id));
        column.setWidth(COLUMN_WIDTH);
        table.addTableColumn(&column);
        ids.push(ns(id));
    }
    let font = NSFont::systemFontOfSize(GLYPH_SIZE);
    let mut fields: Vec<Vec<Retained<NSTextField>>> = Vec::new();
    let mut cells: Vec<Vec<Retained<NSTableCellView>>> = Vec::new();
    for _ in 0..ROWS {
        let mut row_fields = Vec::new();
        let mut row_cells = Vec::new();
        for (_, color, enabled) in COLUMNS {
            let cell = NSTableCellView::initWithFrame(
                NSTableCellView::alloc(mtm),
                NSRect::new(
                    NSPoint::new(0.0, 0.0),
                    NSSize::new(COLUMN_WIDTH, ROW_HEIGHT),
                ),
            );
            let field = NSTextField::labelWithString(&ns(GLYPHS), mtm);
            field.setFont(Some(&font));
            field.setTextColor(Some(&color()));
            field.setEnabled(enabled);
            field.sizeToFit();
            let height = field.frame().size.height;
            field.setFrameOrigin(NSPoint::new(FIELD_INSET, (ROW_HEIGHT - height) / 2.0));
            cell.addSubview(&field);
            ffi::set_text_field(&cell, &field);
            row_fields.push(field);
            row_cells.push(cell);
        }
        fields.push(row_fields);
        cells.push(row_cells);
    }
    let views: Vec<Vec<Retained<NSView>>> = cells
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| Retained::into_super(c.clone()))
                .collect()
        })
        .collect();
    let source = ffi::TableSource::new(mtm, ids, views);
    ffi::set_table_source(&table, Some(&source));
    scroll.setDocumentView(Some(&table));
    content.addSubview(&scroll);
    table.reloadData();
    table
        .selectRowIndexes_byExtendingSelection(&NSIndexSet::indexSetWithIndex(SELECTED_ROW), false);
    scroll.tile();
    table.tile();

    window.makeKeyAndOrderFront(None);
    let steps = pump(app, KEY_WAIT_STEPS, || {
        if !app.isActive() {
            activate_ignoring(app);
        }
        window.isKeyWindow()
    });
    out::put(&mut t, "key_wait_steps", out::int(i64::from(steps)));
    out::put(&mut t, "isKeyWindow", out::boolean(window.isKeyWindow()));
    out::put(&mut t, "NSApp_isActive", out::boolean(app.isActive()));
    window.displayIfNeeded();
    pump(app, SETTLE_STEPS, || false);

    out::put(
        &mut t,
        "effectiveStyle",
        out::text(table_style_name(table.effectiveStyle())),
    );
    let spacing = table.intercellSpacing();
    out::put(
        &mut t,
        "intercellSpacing",
        out::floats(&[spacing.width, spacing.height]),
    );
    out::put(&mut t, "rowHeight", out::float(table.rowHeight()));
    out::put(&mut t, "selectedRow", out::int_isize(table.selectedRow()));
    let window_frame = window.frame();
    out::put(&mut t, "window_frame_xywh", rect(window_frame));
    let from_window = |p: NSPoint| NSPoint::new(p.x, window_frame.size.height - p.y);

    let mut points: Vec<(String, NSPoint)> = Vec::new();
    let mut geometry = Table::new();
    for (r, (row_fields, row_cells)) in fields.iter().zip(&cells).enumerate() {
        for ((field, cell), (name, _, _)) in row_fields.iter().zip(row_cells).zip(COLUMNS) {
            if field.window().is_none() {
                out::put(
                    &mut geometry,
                    &format!("r{r}_{name}"),
                    out::unavailable("the table did not place the field in the window"),
                );
                continue;
            }
            let f = field.convertRect_toView(field.bounds(), None);
            out::put(
                &mut geometry,
                &format!("r{r}_{name}_field_window_xywh"),
                rect(f),
            );
            points.push((
                format!("r{r}_{name}_glyph"),
                from_window(NSPoint::new(
                    f.origin.x + f.size.width / 2.0,
                    f.origin.y + f.size.height / 2.0,
                )),
            ));
            let cf = cell.convertRect_toView(cell.bounds(), None);
            out::put(
                &mut geometry,
                &format!("r{r}_{name}_cell_window_xywh"),
                rect(cf),
            );
            points.push((
                format!("r{r}_{name}_background"),
                from_window(NSPoint::new(
                    background_x(f, cf),
                    cf.origin.y + cf.size.height / 2.0,
                )),
            ));
        }
    }
    let row_rect = table.convertRect_toView(table.rectOfRow(SELECTED_ROW_INDEX), None);
    out::put(&mut geometry, "selected_row_window_xywh", rect(row_rect));
    let strip = cells
        .get(SELECTED_ROW)
        .and_then(|row| row.get(EDGE_STRIP_COLUMN))
        .zip(
            fields
                .get(SELECTED_ROW)
                .and_then(|row| row.get(EDGE_STRIP_COLUMN)),
        );
    if let Some((cell, field)) = strip {
        let cf = cell.convertRect_toView(cell.bounds(), None);
        let x = background_x(field.convertRect_toView(field.bounds(), None), cf);
        for (i, dy) in EDGE_STRIP_PT.iter().enumerate() {
            points.push((
                format!("r{SELECTED_ROW}_bottom_edge_{i:02}"),
                from_window(NSPoint::new(x, row_rect.origin.y + dy)),
            ));
        }
    }
    out::put_table(&mut t, "geometry", geometry);
    let mut sample_points = Table::new();
    for (name, p) in &points {
        out::put(&mut sample_points, name, out::floats(&[p.x, p.y]));
    }
    out::put_table(&mut t, "sample_points_xy", sample_points);
    let named: Vec<(&str, NSPoint)> = points.iter().map(|(n, p)| (n.as_str(), *p)).collect();

    let rel = "renders/selection";
    let dir = out_dir.join(rel);
    let dir_made =
        std::fs::create_dir_all(&dir).map_err(|e| format!("cannot create {}: {e}", dir.display()));
    let table_responder: &NSResponder = table.as_ref();
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
        let mut per = Table::new();
        out::put_table(&mut per, "reference", reference_colors(&appearance));
        for (condition, first) in CONDITIONS {
            let mut run = Table::new();
            out::put(
                &mut run,
                "makeFirstResponder",
                out::boolean(window.makeFirstResponder(first.then_some(table_responder))),
            );
            content.setNeedsDisplay(true);
            window.displayIfNeeded();
            pump(app, SETTLE_STEPS, || false);
            out::put(&mut run, "isKeyWindow", out::boolean(window.isKeyWindow()));
            out::put(&mut run, "NSApp_isActive", out::boolean(app.isActive()));
            let table_first = window
                .firstResponder()
                .is_some_and(|r| std::ptr::eq(&*r, table_responder));
            out::put(
                &mut run,
                "table_is_firstResponder",
                out::boolean(table_first),
            );
            out::put_table(&mut run, "row_view", row_view_table(&table, &cells));
            match &dir_made {
                Ok(()) => capture(
                    &mut run,
                    &window,
                    &dir,
                    rel,
                    &format!("{key}-{condition}"),
                    window_frame.size,
                    &named,
                    &[],
                    &[],
                ),
                Err(reason) => out::put(&mut run, "capture", out::unavailable(reason)),
            }
            out::put_table(&mut per, condition, run);
        }
        out::put_table(&mut t, key, per);
    }
    window.makeFirstResponder(None);
    window.setAppearance(None);
    window.orderOut(None);
    window.close();
    ffi::set_table_source(&table, None);
    drop(source);
    t
}

fn method_text() -> String {
    format!(
        "after the key-window section (the application regular and active): a titled, closable, miniaturizable, resizable NSWindow (empty title) at {}, made key with makeKeyAndOrderFront: and the run loop pumped until isKeyWindow (up to {KEY_WAIT_STEPS} steps); in it an NSScrollView at {} holding a view-based NSTableView with no header view, rowHeight {ROW_HEIGHT}, gridStyleMask NSTableViewSolidHorizontalGridLineMask and the default style and intercell spacing, {} columns {COLUMN_WIDTH} pt wide (the column keys), {ROWS} rows, row {SELECTED_ROW} selected (selectRowIndexes:byExtendingSelection:); each cell an NSTableCellView whose textField (and only subview) is an NSTextField labelWithString: of three U+2588 FULL BLOCK in systemFontOfSize: {GLYPH_SIZE}, sized to fit, {FIELD_INSET} pt in from the cell's left edge and centred in the row, with setTextColor: the column's NSColor and setEnabled: NO in the _disabled columns; per appearance the window's appearance is set and, per condition, makeFirstResponder: with the table (emphasized) or nil (unemphasized), the window displayed and the run loop pumped {SETTLE_STEPS} steps, the row view and cell states read (row_view: rowViewAtRow:makeIfNecessary: NO, isSelected, isEmphasized, interiorBackgroundStyle; each cell's backgroundStyle) and the window captured with screencapture -x -o -l <windowNumber> to renders/selection/<appearance>-<condition>.png and sampled as the key-window section samples (raw, colorAtXY_sRGB) at sample_points_xy, in points from the window frame's top left: r<row>_<column>_glyph the centre of the field's frame (inside the middle block), r<row>_<column>_background midway between the field's right edge and the cell's, at the row's middle, r{SELECTED_ROW}_bottom_edge_NN in column {EDGE_STRIP_COLUMN}'s background x at the offsets {EDGE_STRIP_PT:?} pt up from the selected row rect's lower side (rectOfRow: in the window); reference: each colour as NSColor resolves it under the appearance (performAsCurrentDrawingAppearance:), in sRGB",
        point_text(SELECTION_FRAME.origin),
        rect_text(SELECTION_TABLE_FRAME),
        COLUMNS.len(),
    )
}

/// Where a cell's background is sampled across: midway between its field's
/// right edge and its own, both in the window.
fn background_x(field: NSRect, cell: NSRect) -> f64 {
    let field_right = field.origin.x + field.size.width;
    let cell_right = cell.origin.x + cell.size.width;
    (field_right + cell_right) / 2.0
}

/// The selected row's view and each cell's background style.
fn row_view_table(table: &NSTableView, cells: &[Vec<Retained<NSTableCellView>>]) -> Table {
    let mut t = Table::new();
    match table.rowViewAtRow_makeIfNecessary(SELECTED_ROW_INDEX, false) {
        Some(row) => {
            out::put(&mut t, "isSelected", out::boolean(row.isSelected()));
            out::put(&mut t, "isEmphasized", out::boolean(row.isEmphasized()));
            out::put(
                &mut t,
                "interiorBackgroundStyle",
                out::text(style_name(row.interiorBackgroundStyle())),
            );
        }
        None => out::put(
            &mut t,
            "error",
            out::unavailable("rowViewAtRow:makeIfNecessary: NO is nil"),
        ),
    }
    let mut styles = Table::new();
    for (r, row) in cells.iter().enumerate() {
        for (cell, (name, _, _)) in row.iter().zip(COLUMNS) {
            out::put(
                &mut styles,
                &format!("r{r}_{name}"),
                out::text(style_name(cell.backgroundStyle())),
            );
        }
    }
    out::put_table(&mut t, "cell_backgroundStyle", styles);
    t
}

fn style_name(style: NSBackgroundStyle) -> String {
    if style == NSBackgroundStyle::Normal {
        "normal".to_owned()
    } else if style == NSBackgroundStyle::Emphasized {
        "emphasized".to_owned()
    } else if style == NSBackgroundStyle::Raised {
        "raised".to_owned()
    } else if style == NSBackgroundStyle::Lowered {
        "lowered".to_owned()
    } else {
        format!("unknown ({})", style.0)
    }
}

/// The reference colours resolved under `appearance`, in sRGB.
fn reference_colors(appearance: &NSAppearance) -> Table {
    let resolved = RefCell::new(Table::new());
    {
        let block = RcBlock::new(|| {
            let srgb = NSColorSpace::sRGBColorSpace();
            for (key, make) in REFERENCE {
                let value: Value = color_value(&make(), &srgb);
                out::put(&mut resolved.borrow_mut(), key, value);
            }
        });
        appearance.performAsCurrentDrawingAppearance(&block);
    }
    resolved.into_inner()
}
