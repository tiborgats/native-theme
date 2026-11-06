//! The symbols pass (`--symbols`): the SF Symbol configuration and image
//! size AppKit gives each place a symbol image is shown in. Per place (a
//! plain image view, a borderless and a push button per control size and
//! state, a segmented control per control size, a sidebar and a list table
//! per row size style, a toolbar per size and display mode, a status item and
//! an alert's icon) it reads the view's geometry and its symbol
//! configuration where AppKit has a public getter, renders the view with the
//! symbol and with a blank image of the same size, and takes the glyph's
//! coverage from the difference. The glyph is then compared with the same
//! symbol rendered by an image view at every point size, weight and scale of
//! a reference grid ([`reference_grid`]): the closest configurations are
//! recorded, so the configuration the place draws is read from its pixels
//! where no getter states it. A multicolour, layered symbol's colours per
//! place show the rendering mode.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{AnyThread, MainThreadMarker, MainThreadOnly, Message};
use objc2_app_kit::{
    NSAlert, NSAlertStyle, NSAppearance, NSAppearanceCustomization, NSApplication,
    NSApplicationActivationPolicy, NSBezelStyle, NSBitmapFormat, NSBitmapImageRep, NSButton,
    NSButtonType, NSCellImagePosition, NSControlSize, NSImage, NSImageScaling,
    NSImageSymbolConfiguration, NSImageSymbolScale, NSImageView, NSScrollView, NSSegmentedControl,
    NSStatusBar, NSTableCellView, NSTableColumn, NSTableView, NSTableViewRowSizeStyle,
    NSTableViewStyle, NSTextField, NSToolbar, NSToolbarDisplayMode, NSToolbarItem,
    NSVariableStatusItemLength, NSView, NSWindow,
};
use objc2_foundation::{NSOperatingSystemVersion, NSPoint, NSRect, NSSize};
use toml::{Table, Value};

use super::{
    APPEARANCES, KEY_WAIT_STEPS, SETTLE_STEPS, activate_ignoring, bitmap_at_scale, ffi, ns, pump,
    rect, screencapture, size, state_value,
};
use crate::out;

/// The symbol whose size and weight each place is matched by: one layer,
/// one colour, a closed outline whose stroke shows the weight.
const SIZE_SYMBOL: &str = "star";

/// The symbol whose drawn colours show each place's rendering mode: it has
/// several layers and a multicolour annotation (yellow sun, blue rain), so
/// monochrome draws it in one colour, hierarchical in one colour at several
/// opacities, multicolour in several colours.
const MODE_SYMBOL: &str = "cloud.sun.rain.fill";

/// The accessibility description given to the probe's symbol images, by
/// which the views AppKit makes for them are found.
const DESCRIPTION: &str = "NativeThemeOsProbeSymbol";

/// The control sizes, with the macOS major version each needs.
const SIZES: [(&str, NSControlSize, isize); 5] = [
    ("mini", NSControlSize::Mini, 0),
    ("small", NSControlSize::Small, 0),
    ("regular", NSControlSize::Regular, 0),
    ("large", NSControlSize::Large, 11),
    ("extraLarge", NSControlSize::ExtraLarge, 26),
];

/// The reference grid's scales.
const SCALES: [(&str, NSImageSymbolScale); 3] = [
    ("small", NSImageSymbolScale::Small),
    ("medium", NSImageSymbolScale::Medium),
    ("large", NSImageSymbolScale::Large),
];

/// The reference grid's point sizes: from `GRID_FIRST` in steps of
/// `GRID_STEP`, `GRID_COUNT` of them (6 to 40 pt by half points).
const GRID_FIRST: f64 = 6.0;
const GRID_STEP: f64 = 0.5;
const GRID_COUNT: u32 = 69;

/// The backing scales each place and reference is rendered at.
const RENDER_SCALES: [(&str, f64); 2] = [("1x", 1.0), ("2x", 2.0)];

/// The margin round a reference image view's image, in points.
const REFERENCE_MARGIN: f64 = 4.0;

/// A pixel counts in a glyph where its coverage is above this.
const COVERAGE_THRESHOLD: f32 = 1.5 / 255.0;

/// How many of the closest references each glyph records.
const BEST_COUNT: usize = 6;

/// How far, in pixels, a glyph is shifted against a reference to find their
/// best alignment.
const SHIFT: isize = 1;

/// How many pixels a reference's glyph may differ from a place's in width
/// or height to be compared with it.
const SIZE_TOLERANCE: usize = 2;

/// How many colours a glyph's histogram keeps.
const HISTOGRAM_COUNT: usize = 8;

/// The offscreen window's content frame.
const OFFSCREEN_FRAME: NSRect = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(800.0, 600.0));

/// Where the offscreen places' views are put in the offscreen window.
const PLACE_ORIGIN: NSPoint = NSPoint::new(20.0, 20.0);

/// The toolbar window's content frame on the screen.
const TOOLBAR_FRAME: NSRect = NSRect::new(NSPoint::new(200.0, 300.0), NSSize::new(480.0, 120.0));

/// The tables' scroll view in the offscreen window.
const TABLE_FRAME: NSRect = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(300.0, 200.0));

/// The tables' column width.
const TABLE_COLUMN_WIDTH: f64 = 260.0;

/// The table cell view's frame before the table sizes it.
const TABLE_CELL_FRAME: NSRect = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(200.0, 24.0));

/// The image view's leading inset in its cell, and the gap to the text
/// field, in points.
const TABLE_IMAGE_INSET: f64 = 2.0;
const TABLE_TEXT_GAP: f64 = 4.0;

/// How deep a walk of a window's views goes.
const WALK_DEPTH: u32 = 12;

/// The pump steps after a toolbar item's or status item's image changes.
const IMAGE_STEPS: u32 = 2;

/// The glyph's coverage per pixel, cropped to the pixels it covers and
/// scaled so its largest value is 1.
struct Mask {
    w: usize,
    h: usize,
    v: Vec<f32>,
}

impl Mask {
    /// The coverage at (`x`, `y`); none outside the mask.
    fn at(&self, x: isize, y: isize) -> f32 {
        let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) else {
            return 0.0;
        };
        if x >= self.w || y >= self.h {
            return 0.0;
        }
        coverage_at(&self.v, self.w, x, y)
    }
}

/// The value at (`x`, `y`) of a map `w` wide; none outside it.
fn coverage_at(v: &[f32], w: usize, x: usize, y: usize) -> f32 {
    match y
        .checked_mul(w)
        .and_then(|o| o.checked_add(x))
        .and_then(|i| v.get(i))
    {
        Some(c) => *c,
        None => 0.0,
    }
}

/// What a glyph's coverage gives besides its mask: its bounding box in the
/// render, its largest raw coverage and its ink (the scaled coverage's sum).
struct Glyph {
    mask: Mask,
    bbox: [usize; 4],
    max: f32,
    ink: f32,
}

/// A render's pixels: 8-bit RGBA, premultiplied, rows top to bottom.
struct Pixels {
    w: usize,
    h: usize,
    row: usize,
    data: Vec<u8>,
}

impl Pixels {
    fn of(rep: &NSBitmapImageRep) -> Result<Self, String> {
        if rep.samplesPerPixel() != 4 || rep.bitsPerPixel() != 32 {
            return Err(format!(
                "{} samples, {} bits a pixel",
                rep.samplesPerPixel(),
                rep.bitsPerPixel()
            ));
        }
        if rep.bitmapFormat().contains(NSBitmapFormat::AlphaFirst) {
            return Err("alpha first".to_owned());
        }
        let data = ffi::bitmap_buffer(rep)?;
        let w = usize::try_from(rep.pixelsWide()).map_err(|_| "negative width".to_owned())?;
        let h = usize::try_from(rep.pixelsHigh()).map_err(|_| "negative height".to_owned())?;
        let row = usize::try_from(rep.bytesPerRow()).map_err(|_| "negative row".to_owned())?;
        Ok(Self { w, h, row, data })
    }

    /// The pixel at (`x`, `y`); transparent black outside the buffer.
    fn px(&self, x: usize, y: usize) -> [u8; 4] {
        let start = y
            .checked_mul(self.row)
            .and_then(|o| x.checked_mul(4).and_then(|p| o.checked_add(p)));
        let bytes = start.and_then(|s| s.checked_add(4).and_then(|e| self.data.get(s..e)));
        match bytes {
            Some([r, g, b, a]) => [*r, *g, *b, *a],
            _ => [0; 4],
        }
    }
}

/// A reference: the size symbol drawn by an image view at one
/// configuration, per render scale.
struct Reference {
    point_size: f64,
    weight: &'static str,
    scale: &'static str,
    intrinsic: NSSize,
    glyphs: Vec<Option<Glyph>>,
}

/// What every place shares.
struct Ctx<'a> {
    mtm: MainThreadMarker,
    app: &'a NSApplication,
    out_dir: &'a Path,
    star: Retained<NSImage>,
    mode: Option<Retained<NSImage>>,
    blank: Retained<NSImage>,
    refs: Vec<Reference>,
}

/// A count as a TOML integer, or its text where it does not fit.
fn count(v: usize) -> Value {
    isize::try_from(v).map_or_else(|_| out::text(v.to_string()), out::int_isize)
}

/// Fill `[macos.symbols]`.
pub(super) fn symbols_table(
    mtm: MainThreadMarker,
    app: &NSApplication,
    out_dir: &Path,
    version: &NSOperatingSystemVersion,
) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "method", out::text(method_text()));
    out::put(
        &mut t,
        "activationPolicy_regular_set",
        out::boolean(app.setActivationPolicy(NSApplicationActivationPolicy::Regular)),
    );
    app.finishLaunching();
    activate_ignoring(app);

    let star = match symbol(SIZE_SYMBOL) {
        Ok(image) => image,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return t;
        }
    };
    let mode = match symbol(MODE_SYMBOL) {
        Ok(image) => Some(image),
        Err(reason) => {
            out::put(&mut t, "mode_symbol", out::unavailable(reason));
            None
        }
    };
    out::put_table(&mut t, "size_symbol", image_table(&star));
    if let Some(mode) = &mode {
        out::put_table(&mut t, "mode_symbol", image_table(mode));
    }
    let blank = NSImage::initWithSize(NSImage::alloc(), star.size());
    blank.setTemplate(true);

    let window = match ffi::offscreen_window(mtm, OFFSCREEN_FRAME) {
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
    out::put(
        &mut t,
        "offscreen_window_backingScaleFactor",
        out::float(window.backingScaleFactor()),
    );

    let (refs, grid) = reference_grid(mtm, &content, &star);
    out::put_table(&mut t, "reference_grid", grid);
    let ctx = Ctx {
        mtm,
        app,
        out_dir,
        star,
        mode,
        blank,
        refs,
    };

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
        let result = RefCell::new(Table::new());
        {
            let block = RcBlock::new(|| {
                *result.borrow_mut() = ctx.offscreen_places(&content, key, version);
            });
            appearance.performAsCurrentDrawingAppearance(&block);
        }
        out::put_table(&mut t, key, result.into_inner());
    }
    window.setAppearance(None);

    out::put_table(&mut t, "toolbar", ctx.toolbar());
    out::put_table(&mut t, "status_item", ctx.status_item());
    out::put_table(&mut t, "alert", ctx.alert());
    t
}

fn method_text() -> String {
    format!(
        "symbol images from imageWithSystemSymbolName:accessibilityDescription: (\"{SIZE_SYMBOL}\" for the size, weight and scale; \"{MODE_SYMBOL}\" for the rendering mode), accessibility description \"{DESCRIPTION}\"; reference_grid: per point size ({GRID_FIRST} pt in steps of {GRID_STEP}, {GRID_COUNT} sizes), NSFontWeight (the nine constants) and NSImageSymbolScale (small, medium, large), an NSImageView (imageScaling NSImageScaleNone) showing \"{SIZE_SYMBOL}\" with setSymbolConfiguration: configurationWithPointSize:weight:scale:, sized to its intrinsicContentSize rounded up plus {REFERENCE_MARGIN} pt each side, in an offscreen borderless window under the aqua appearance, rendered with cacheDisplayInRect:toBitmapImageRep: into 8-bit RGBA bitmaps at 1 and 2 pixels a point; each place: the view AppKit draws the symbol in, rendered the same way with the symbol and again with a blank template NSImage of the symbol's size in its place (the view's frame kept), the glyph's coverage per pixel the largest change of a premultiplied channel or alpha between the two (alone the render's alpha where the two renders differ in size), cropped to the pixels above {COVERAGE_THRESHOLD} and scaled to a largest value of 1; glyph: bbox_px_xywh (in the render, top-left origin), max_coverage (unscaled, 0..1) and ink (the scaled coverage's sum); best: the {BEST_COUNT} references closest to the glyph at the same render scale among those whose glyph is within {SIZE_TOLERANCE} px of its width and height, each [score, point_size, weight, scale], score = sum |g - r| / sum (g + r) over the two masks aligned at their top-left corners and shifted up to {SHIFT} px either way, the smallest over the shifts (0: the same pixels); colors: the {HISTOGRAM_COUNT} commonest colours of the render with the symbol among its pixels whose alpha is not 0, un-premultiplied, as [#rrggbbaa, pixels], and opaque_colors, how many distinct colours its pixels of alpha 255 have; places in the offscreen window are read per appearance (window appearance set, inside performAsCurrentDrawingAppearance:), the toolbar, status item and alert under the system's"
    )
}

/// The symbol `name`, given the probe's accessibility description.
fn symbol(name: &str) -> Result<Retained<NSImage>, String> {
    NSImage::imageWithSystemSymbolName_accessibilityDescription(&ns(name), Some(&ns(DESCRIPTION)))
        .ok_or_else(|| format!("imageWithSystemSymbolName: \"{name}\" is nil"))
}

/// An image's size, its symbol configuration and alignment.
fn image_table(image: &NSImage) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "size_wh", size(image.size()));
    out::put(&mut t, "isTemplate", out::boolean(image.isTemplate()));
    out::put(&mut t, "alignmentRect_xywh", rect(image.alignmentRect()));
    out::put(
        &mut t,
        "symbolConfiguration_description",
        out::text(format!("{:?}", image.symbolConfiguration())),
    );
    out::put(
        &mut t,
        "accessibilityDescription",
        image
            .accessibilityDescription()
            .map_or_else(|| out::unavailable("nil"), |d| out::text(d.to_string())),
    );
    t
}

/// A symbol configuration's description, or that there is none.
fn configuration_value(config: Option<Retained<NSImageSymbolConfiguration>>) -> Value {
    config.map_or_else(|| out::text("nil"), |c| out::text(format!("{c:?}")))
}

/// The reference grid: the references and their table, one row per
/// configuration.
fn reference_grid(
    mtm: MainThreadMarker,
    content: &NSView,
    star: &NSImage,
) -> (Vec<Reference>, Table) {
    let mut t = Table::new();
    out::put(
        &mut t,
        "columns",
        out::text(
            "[point_size, weight, scale, intrinsic_w, intrinsic_h, then per render scale (1x, 2x) bbox_w, bbox_h, ink, or \"none\" where nothing is drawn]",
        ),
    );
    let weights = match ffi::font_weights() {
        Ok(weights) => weights,
        Err(reason) => {
            out::put(&mut t, "error", out::unavailable(reason));
            return (Vec::new(), t);
        }
    };
    let Some(aqua) = NSAppearance::appearanceNamed(&ns("NSAppearanceNameAqua")) else {
        out::put(
            &mut t,
            "error",
            out::unavailable("appearanceNamed(NSAppearanceNameAqua) is nil"),
        );
        return (Vec::new(), t);
    };
    let refs = RefCell::new(Vec::new());
    {
        let block = RcBlock::new(|| {
            let mut list = Vec::new();
            for i in 0..GRID_COUNT {
                let point_size = GRID_FIRST + GRID_STEP * f64::from(i);
                for (weight_name, weight) in weights {
                    for (scale_name, scale) in SCALES {
                        list.push(reference(
                            mtm,
                            content,
                            star,
                            point_size,
                            (weight_name, weight),
                            (scale_name, scale),
                        ));
                    }
                }
            }
            *refs.borrow_mut() = list;
        });
        aqua.performAsCurrentDrawingAppearance(&block);
    }
    let refs = refs.into_inner();
    let rows = refs
        .iter()
        .map(|r| {
            let mut row = vec![
                out::float(r.point_size),
                out::text(r.weight),
                out::text(r.scale),
                out::float(r.intrinsic.width),
                out::float(r.intrinsic.height),
            ];
            for glyph in &r.glyphs {
                match glyph {
                    Some(g) => {
                        row.push(count(g.mask.w));
                        row.push(count(g.mask.h));
                        row.push(out::float(rounded(g.ink, 100.0)));
                    }
                    None => row.push(out::text("none")),
                }
            }
            out::array(row)
        })
        .collect();
    out::put(&mut t, "rows", out::array(rows));
    (refs, t)
}

/// `v` rounded to `1 / per` (`per` 100: two decimals).
fn rounded(v: f32, per: f64) -> f64 {
    (f64::from(v) * per).round() / per
}

/// One reference configuration's renders.
fn reference(
    mtm: MainThreadMarker,
    content: &NSView,
    star: &NSImage,
    point_size: f64,
    (weight_name, weight): (&'static str, f64),
    (scale_name, scale): (&'static str, NSImageSymbolScale),
) -> Reference {
    let config = NSImageSymbolConfiguration::configurationWithPointSize_weight_scale(
        point_size, weight, scale,
    );
    let view = NSImageView::imageViewWithImage(star, mtm);
    view.setImageScaling(NSImageScaling::ScaleNone);
    view.setSymbolConfiguration(Some(&config));
    let intrinsic = view.intrinsicContentSize();
    view.setFrame(NSRect::new(
        PLACE_ORIGIN,
        NSSize::new(
            intrinsic.width.max(1.0).ceil() + REFERENCE_MARGIN + REFERENCE_MARGIN,
            intrinsic.height.max(1.0).ceil() + REFERENCE_MARGIN + REFERENCE_MARGIN,
        ),
    ));
    content.addSubview(&view);
    let glyphs = RENDER_SCALES
        .iter()
        .map(|(_, s)| {
            render(&view, *s)
                .ok()
                .and_then(|p| glyph(&alpha_coverage(&p), p.w, p.h))
        })
        .collect();
    view.removeFromSuperview();
    Reference {
        point_size,
        weight: weight_name,
        scale: scale_name,
        intrinsic,
        glyphs,
    }
}

/// `view` rendered at `scale` pixels a point.
fn render(view: &NSView, scale: f64) -> Result<Pixels, String> {
    let bounds = view.bounds();
    let rep = bitmap_at_scale(bounds.size, scale)?;
    view.cacheDisplayInRect_toBitmapImageRep(bounds, &rep);
    Pixels::of(&rep)
}

/// `view` rendered at `scale` and written to `path` as PNG; its pixels.
fn render_png(view: &NSView, scale: f64, path: &Path) -> Result<Pixels, String> {
    let bounds = view.bounds();
    let rep = bitmap_at_scale(bounds.size, scale)?;
    view.cacheDisplayInRect_toBitmapImageRep(bounds, &rep);
    let data = ffi::png_data(&rep)?;
    std::fs::write(path, data).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
    Pixels::of(&rep)
}

/// Each pixel's alpha, 0..1.
fn alpha_coverage(p: &Pixels) -> Vec<f32> {
    let mut v = Vec::with_capacity(p.w.saturating_mul(p.h));
    for y in 0..p.h {
        for x in 0..p.w {
            let [_, _, _, a] = p.px(x, y);
            v.push(f32::from(a) / 255.0);
        }
    }
    v
}

/// Each pixel's largest change of a premultiplied channel or alpha from
/// `blank` to `with`, 0..1; `None` where the two differ in size.
fn diff_coverage(with: &Pixels, blank: &Pixels) -> Option<Vec<f32>> {
    if with.w != blank.w || with.h != blank.h {
        return None;
    }
    let mut v = Vec::with_capacity(with.w.saturating_mul(with.h));
    for y in 0..with.h {
        for x in 0..with.w {
            let a = with.px(x, y);
            let b = blank.px(x, y);
            let d = a
                .iter()
                .zip(b.iter())
                .map(|(p, q)| p.abs_diff(*q))
                .fold(u8::MIN, u8::max);
            v.push(f32::from(d) / 255.0);
        }
    }
    Some(v)
}

/// The glyph in a coverage map `w` x `h`: `None` where no pixel is covered.
fn glyph(coverage: &[f32], w: usize, h: usize) -> Option<Glyph> {
    let mut bounds: Option<(usize, usize, usize, usize)> = None;
    let mut max = 0f32;
    for y in 0..h {
        for x in 0..w {
            let c = coverage_at(coverage, w, x, y);
            if c > COVERAGE_THRESHOLD {
                bounds = Some(match bounds {
                    None => (x, y, x, y),
                    Some((x0, y0, x1, y1)) => (x0.min(x), y0.min(y), x1.max(x), y1.max(y)),
                });
                max = max.max(c);
            }
        }
    }
    let (x0, y0, x1, y1) = bounds?;
    if max <= 0.0 {
        return None;
    }
    let mw = x1.checked_sub(x0)?.checked_add(1)?;
    let mh = y1.checked_sub(y0)?.checked_add(1)?;
    let mut v = Vec::with_capacity(mw.saturating_mul(mh));
    let mut ink = 0f32;
    for y in y0..=y1 {
        for x in x0..=x1 {
            let c = coverage_at(coverage, w, x, y);
            let s = if c > COVERAGE_THRESHOLD { c / max } else { 0.0 };
            ink += s;
            v.push(s);
        }
    }
    Some(Glyph {
        mask: Mask { w: mw, h: mh, v },
        bbox: [x0, y0, mw, mh],
        max,
        ink,
    })
}

/// How far two masks are apart: 0 for the same pixels, 1 for no overlap.
fn score(a: &Mask, b: &Mask) -> f32 {
    let (Ok(w), Ok(h)) = (isize::try_from(a.w.max(b.w)), isize::try_from(a.h.max(b.h))) else {
        return 1.0;
    };
    let mut best: Option<f32> = None;
    for dy in -SHIFT..=SHIFT {
        for dx in -SHIFT..=SHIFT {
            let mut diff = 0f32;
            let mut total = 0f32;
            for y in -SHIFT..h.saturating_add(SHIFT) {
                for x in -SHIFT..w.saturating_add(SHIFT) {
                    let va = a.at(x, y);
                    let vb = b.at(x.saturating_add(dx), y.saturating_add(dy));
                    diff += (va - vb).abs();
                    total += va + vb;
                }
            }
            if total > 0.0 {
                let s = diff / total;
                best = Some(best.map_or(s, |b| b.min(s)));
            }
        }
    }
    match best {
        Some(s) => s,
        None => 1.0,
    }
}

/// The colours of the pixels whose alpha is not 0, un-premultiplied, and
/// how many distinct colours the opaque pixels have.
fn colors_value(p: &Pixels) -> (Value, usize) {
    let mut counts: BTreeMap<[u8; 4], usize> = BTreeMap::new();
    let mut opaque: BTreeSet<[u8; 3]> = BTreeSet::new();
    for y in 0..p.h {
        for x in 0..p.w {
            let [r, g, b, a] = p.px(x, y);
            if a == 0 {
                continue;
            }
            let un = |c: u8| -> u8 {
                let v = (f32::from(c) * 255.0 / f32::from(a))
                    .round()
                    .clamp(0.0, 255.0);
                // In 0.0..=255.0, so the cast is exact.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let byte = v as u8;
                byte
            };
            let entry = counts.entry([un(r), un(g), un(b), a]).or_insert(0);
            *entry = entry.saturating_add(1);
            if a == u8::MAX {
                opaque.insert([r, g, b]);
            }
        }
    }
    let mut list: Vec<([u8; 4], usize)> = counts.into_iter().collect();
    list.sort_by(|a, b| b.1.cmp(&a.1));
    let rows = list
        .into_iter()
        .take(HISTOGRAM_COUNT)
        .map(|([r, g, b, a], n)| {
            out::array(vec![
                out::text(format!("#{r:02x}{g:02x}{b:02x}{a:02x}")),
                count(n),
            ])
        })
        .collect();
    (out::array(rows), opaque.len())
}

/// Put `view` at the place origin, its frame its current size rounded up.
fn place(content: &NSView, view: &NSView) {
    let f = view.frame().size;
    view.setFrame(NSRect::new(
        PLACE_ORIGIN,
        NSSize::new(f.width.ceil(), f.height.ceil()),
    ));
    content.addSubview(view);
}

impl Ctx<'_> {
    /// The places in the offscreen window, under one appearance.
    fn offscreen_places(
        &self,
        content: &NSView,
        appearance: &str,
        version: &NSOperatingSystemVersion,
    ) -> Table {
        let mut t = Table::new();
        let dir = self.out_dir.join(appearance);
        if let Err(e) = std::fs::create_dir_all(&dir) {
            out::put(
                &mut t,
                "error",
                out::unavailable(format!("cannot create {}: {e}", dir.display())),
            );
            return t;
        }

        // A plain image view, no configuration, at its intrinsic size.
        {
            let view = NSImageView::imageViewWithImage(&self.star, self.mtm);
            view.setFrameSize(view.intrinsicContentSize());
            place(content, &view);
            let mut p = self.view_reads(&view);
            out::put(
                &mut p,
                "setup",
                out::text(
                    "imageViewWithImage:, its defaults, frame its intrinsicContentSize rounded up",
                ),
            );
            let set = |i: Option<&NSImage>| view.setImage(i);
            self.glyphs(&mut p, &view, &set, &dir, appearance, "image_view");
            self.mode_glyph(&mut p, &view, &set, &dir, appearance, "image_view");
            view.removeFromSuperview();
            out::put_table(&mut t, "image_view", p);
        }

        let mut controls = Table::new();
        for (size_name, size_value, needs) in SIZES {
            if version.majorVersion < needs {
                out::put(
                    &mut controls,
                    size_name,
                    out::unavailable(format!("needs macOS {needs}")),
                );
                continue;
            }
            let mut per = Table::new();
            for kind in [Kind::Borderless, Kind::Push, Kind::PushTitled] {
                out::put_table(
                    &mut per,
                    kind.name(),
                    self.button(content, kind, size_name, size_value, &dir, appearance),
                );
            }
            out::put_table(
                &mut per,
                "segmented",
                self.segmented(content, size_name, size_value, &dir, appearance),
            );
            out::put_table(&mut controls, size_name, per);
        }
        out::put_table(&mut t, "controls", controls);

        let mut tables = Table::new();
        for (style_name, style) in [
            ("sourceList", NSTableViewStyle::SourceList),
            ("inset", NSTableViewStyle::Inset),
            ("plain", NSTableViewStyle::Plain),
        ] {
            let mut per = Table::new();
            for (row_name, row_style) in [
                ("default", NSTableViewRowSizeStyle::Default),
                ("small", NSTableViewRowSizeStyle::Small),
                ("medium", NSTableViewRowSizeStyle::Medium),
                ("large", NSTableViewRowSizeStyle::Large),
            ] {
                let stem = format!("table-{style_name}-{row_name}");
                out::put_table(
                    &mut per,
                    row_name,
                    self.table_row(content, style, row_style, &dir, appearance, &stem),
                );
            }
            out::put_table(&mut tables, style_name, per);
        }
        out::put_table(&mut t, "tables", tables);
        t
    }

    /// A view's geometry and the symbol configuration it holds.
    fn view_reads(&self, view: &NSView) -> Table {
        let mut t = Table::new();
        out::put(
            &mut t,
            "class",
            out::text(view.class().name().to_string_lossy()),
        );
        out::put(&mut t, "frame_xywh", rect(view.frame()));
        out::put(
            &mut t,
            "frame_in_window_xywh",
            rect(view.convertRect_toView(view.bounds(), None)),
        );
        out::put(
            &mut t,
            "intrinsicContentSize_wh",
            size(view.intrinsicContentSize()),
        );
        out::put(
            &mut t,
            "alignmentRectInsets_tlbr",
            super::insets(view.alignmentRectInsets()),
        );
        let obj: &AnyObject = view.as_ref();
        if let Some(image_view) = obj.downcast_ref::<NSImageView>() {
            out::put(
                &mut t,
                "symbolConfiguration",
                configuration_value(image_view.symbolConfiguration()),
            );
            out::put(
                &mut t,
                "imageScaling_raw",
                count(image_view.imageScaling().0),
            );
            if let Some(image) = image_view.image() {
                out::put_table(&mut t, "image", image_table(&image));
            }
        }
        if let Some(button) = obj.downcast_ref::<NSButton>() {
            out::put(
                &mut t,
                "symbolConfiguration",
                configuration_value(button.symbolConfiguration()),
            );
            out::put(&mut t, "controlSize_raw", count(button.controlSize().0));
            if let Some(font) = button.font() {
                out::put(&mut t, "font_pointSize", out::float(font.pointSize()));
                out::put(
                    &mut t,
                    "font_fontName",
                    out::text(font.fontName().to_string()),
                );
            }
            if let Some(cell) = button.cell() {
                let bounds = view.bounds();
                out::put(
                    &mut t,
                    "cell_imageRectForBounds_xywh",
                    rect(cell.imageRectForBounds(bounds)),
                );
                out::put(
                    &mut t,
                    "cell_titleRectForBounds_xywh",
                    rect(cell.titleRectForBounds(bounds)),
                );
            }
            if let Some(image) = button.image() {
                out::put_table(&mut t, "image", image_table(&image));
            }
        }
        t
    }

    /// The size symbol's glyph in `view` at each render scale: the render
    /// with the symbol (written to `<rel>/<stem>-<scale>.png`), the blank
    /// render, the glyph and the closest references.
    fn glyphs(
        &self,
        t: &mut Table,
        view: &NSView,
        set: &dyn Fn(Option<&NSImage>),
        dir: &Path,
        rel: &str,
        stem: &str,
    ) {
        let mut all = Table::new();
        for (index, (scale_name, scale)) in RENDER_SCALES.iter().enumerate() {
            let mut g = Table::new();
            let file = format!("{stem}-{scale_name}.png");
            set(Some(&self.star));
            let with = render_png(view, *scale, &dir.join(&file));
            set(Some(&self.blank));
            let blank = render(view, *scale);
            set(Some(&self.star));
            let with = match with {
                Ok(with) => with,
                Err(reason) => {
                    out::put(&mut g, "error", out::unavailable(reason));
                    out::put_table(&mut all, scale_name, g);
                    continue;
                }
            };
            out::put(&mut g, "file", out::text(format!("{rel}/{file}")));
            out::put(
                &mut g,
                "pixels_wh",
                out::array(vec![count(with.w), count(with.h)]),
            );
            let coverage = match blank.as_ref().ok().and_then(|b| diff_coverage(&with, b)) {
                Some(c) => {
                    out::put(
                        &mut g,
                        "coverage",
                        out::text("difference from the blank render"),
                    );
                    c
                }
                None => {
                    let why = match &blank {
                        Ok(_) => "alpha (the blank render differs in size)".to_owned(),
                        Err(reason) => format!("alpha (no blank render: {reason})"),
                    };
                    out::put(&mut g, "coverage", out::text(why));
                    alpha_coverage(&with)
                }
            };
            let (colors, opaque) = colors_value(&with);
            out::put(&mut g, "colors", colors);
            out::put(&mut g, "opaque_colors", count(opaque));
            match glyph(&coverage, with.w, with.h) {
                Some(found) => {
                    out::put_table(&mut g, "glyph", glyph_table(&found));
                    out::put(&mut g, "best", self.best(&found, index));
                }
                None => out::put(&mut g, "glyph", out::unavailable("no pixel covered")),
            }
            out::put_table(&mut all, scale_name, g);
        }
        out::put_table(t, "size_symbol_renders", all);
    }

    /// The mode symbol in `view`: its renders' colours, at each scale.
    fn mode_glyph(
        &self,
        t: &mut Table,
        view: &NSView,
        set: &dyn Fn(Option<&NSImage>),
        dir: &Path,
        rel: &str,
        stem: &str,
    ) {
        let Some(mode) = &self.mode else {
            return;
        };
        let mut all = Table::new();
        set(Some(mode));
        for (scale_name, scale) in RENDER_SCALES {
            let mut g = Table::new();
            let file = format!("{stem}-mode-{scale_name}.png");
            match render_png(view, scale, &dir.join(&file)) {
                Ok(with) => {
                    out::put(&mut g, "file", out::text(format!("{rel}/{file}")));
                    let (colors, opaque) = colors_value(&with);
                    out::put(&mut g, "colors", colors);
                    out::put(&mut g, "opaque_colors", count(opaque));
                }
                Err(reason) => out::put(&mut g, "error", out::unavailable(reason)),
            }
            out::put_table(&mut all, scale_name, g);
        }
        set(Some(&self.star));
        out::put_table(t, "mode_symbol_renders", all);
    }

    /// The references closest to `g` at render scale `index`.
    fn best(&self, g: &Glyph, index: usize) -> Value {
        let mut scored: Vec<(f32, &Reference)> = self
            .refs
            .iter()
            .filter_map(|r| {
                r.glyphs
                    .get(index)
                    .and_then(Option::as_ref)
                    .filter(|rg| {
                        rg.mask.w.abs_diff(g.mask.w) <= SIZE_TOLERANCE
                            && rg.mask.h.abs_diff(g.mask.h) <= SIZE_TOLERANCE
                    })
                    .map(|rg| (score(&g.mask, &rg.mask), r))
            })
            .collect();
        scored.sort_by(|a, b| a.0.total_cmp(&b.0));
        out::array(
            scored
                .into_iter()
                .take(BEST_COUNT)
                .map(|(s, r)| {
                    out::array(vec![
                        out::float(rounded(s, 10000.0)),
                        out::float(r.point_size),
                        out::text(r.weight),
                        out::text(r.scale),
                    ])
                })
                .collect(),
        )
    }

    /// A button of `kind` at `size`, per state.
    fn button(
        &self,
        content: &NSView,
        kind: Kind,
        size_name: &str,
        size_value: NSControlSize,
        dir: &Path,
        rel: &str,
    ) -> Table {
        let mut t = Table::new();
        out::put(&mut t, "setup", out::text(kind.setup()));
        for state in ["normal", "disabled", "pressed", "on"] {
            let b = NSButton::new(self.mtm);
            b.setButtonType(if state == "on" {
                NSButtonType::PushOnPushOff
            } else {
                NSButtonType::MomentaryPushIn
            });
            b.setImage(Some(&self.star));
            match kind {
                Kind::Borderless => {
                    b.setBordered(false);
                    b.setImagePosition(NSCellImagePosition::ImageOnly);
                }
                Kind::Push => {
                    b.setBezelStyle(NSBezelStyle::Push);
                    b.setImagePosition(NSCellImagePosition::ImageOnly);
                }
                Kind::PushTitled => {
                    b.setBezelStyle(NSBezelStyle::Push);
                    b.setTitle(&ns("Button"));
                    b.setImagePosition(NSCellImagePosition::ImageLeft);
                }
            }
            super::control_common(&b, size_value);
            b.sizeToFit();
            place(content, &b);
            match state {
                "disabled" => b.setEnabled(false),
                "pressed" => b.highlight(true),
                "on" => b.setState(state_value(true)),
                _ => {}
            }
            let view: &NSView = &b;
            let mut p = self.view_reads(view);
            let stem = format!("{}-{size_name}-{state}", kind.name());
            let set = |i: Option<&NSImage>| b.setImage(i);
            self.glyphs(&mut p, view, &set, dir, rel, &stem);
            if state == "normal" {
                self.mode_glyph(&mut p, view, &set, dir, rel, &stem);
            }
            b.removeFromSuperview();
            out::put_table(&mut t, state, p);
        }
        t
    }

    /// A one-segment segmented control showing the symbol, at `size`.
    fn segmented(
        &self,
        content: &NSView,
        size_name: &str,
        size_value: NSControlSize,
        dir: &Path,
        rel: &str,
    ) -> Table {
        let s = NSSegmentedControl::new(self.mtm);
        s.setSegmentCount(1);
        s.setImage_forSegment(Some(&self.star), 0);
        super::control_common(&s, size_value);
        s.sizeToFit();
        place(content, &s);
        let width = s.widthForSegment(0);
        let view: &NSView = &s;
        let mut p = self.view_reads(view);
        out::put(
            &mut p,
            "setup",
            out::text(
                "NSSegmentedControl new, segmentCount 1, setImage:forSegment: the symbol, setControlSize: and the systemFontOfSize: of systemFontSizeForControlSize:, sizeToFit, frame rounded up; the blank render keeps the segment's width (setWidth:forSegment: with widthForSegment: as read)",
            ),
        );
        out::put(&mut p, "widthForSegment_0", out::float(width));
        let set = |i: Option<&NSImage>| {
            s.setImage_forSegment(i, 0);
            s.setWidth_forSegment(width, 0);
        };
        self.glyphs(
            &mut p,
            view,
            &set,
            dir,
            rel,
            &format!("segmented-{size_name}"),
        );
        s.removeFromSuperview();
        p
    }

    /// A one-column view-based table of `style` and `row_style`, one row
    /// whose NSTableCellView holds an image view (the symbol) and a text
    /// field, laid out by constraints: the image view's size is its own.
    fn table_row(
        &self,
        content: &NSView,
        style: NSTableViewStyle,
        row_style: NSTableViewRowSizeStyle,
        dir: &Path,
        rel: &str,
        stem: &str,
    ) -> Table {
        let mut t = Table::new();
        out::put(
            &mut t,
            "setup",
            out::text(format!(
                "an NSScrollView ({} x {} pt) holding a view-based NSTableView (no header view, setStyle:, setRowSizeStyle:, one column {TABLE_COLUMN_WIDTH} pt wide) whose one row is an NSTableCellView holding an imageViewWithImage: (the symbol) and a labelWithString: \"Item\", its imageView and textField; the image view's leading edge {TABLE_IMAGE_INSET} pt in from the cell's, the text field's {TABLE_TEXT_GAP} pt after the image view, both centred vertically, by constraints; no size is given to the image view",
                TABLE_FRAME.size.width, TABLE_FRAME.size.height
            )),
        );
        let scroll = NSScrollView::initWithFrame(NSScrollView::alloc(self.mtm), TABLE_FRAME);
        let table = NSTableView::new(self.mtm);
        table.setHeaderView(None);
        table.setStyle(style);
        table.setRowSizeStyle(row_style);
        let column =
            NSTableColumn::initWithIdentifier(NSTableColumn::alloc(self.mtm), &ns("column"));
        column.setWidth(TABLE_COLUMN_WIDTH);
        table.addTableColumn(&column);
        let cell =
            NSTableCellView::initWithFrame(NSTableCellView::alloc(self.mtm), TABLE_CELL_FRAME);
        let image_view = NSImageView::imageViewWithImage(&self.star, self.mtm);
        let field = NSTextField::labelWithString(&ns("Item"), self.mtm);
        image_view.setTranslatesAutoresizingMaskIntoConstraints(false);
        field.setTranslatesAutoresizingMaskIntoConstraints(false);
        cell.addSubview(&image_view);
        cell.addSubview(&field);
        ffi::set_image_view(&cell, &image_view);
        ffi::set_text_field(&cell, &field);
        let constraints = [
            image_view
                .leadingAnchor()
                .constraintEqualToAnchor_constant(&cell.leadingAnchor(), TABLE_IMAGE_INSET),
            image_view
                .centerYAnchor()
                .constraintEqualToAnchor(&cell.centerYAnchor()),
            field
                .leadingAnchor()
                .constraintEqualToAnchor_constant(&image_view.trailingAnchor(), TABLE_TEXT_GAP),
            field
                .centerYAnchor()
                .constraintEqualToAnchor(&cell.centerYAnchor()),
        ];
        for c in &constraints {
            c.setActive(true);
        }
        let views: Vec<Vec<Retained<NSView>>> = vec![vec![Retained::into_super(cell.clone())]];
        let source = ffi::TableSource::new(self.mtm, vec![ns("column")], views);
        ffi::set_table_source(&table, Some(&source));
        scroll.setDocumentView(Some(&table));
        content.addSubview(&scroll);
        table.reloadData();
        scroll.tile();
        table.tile();
        let made = table.viewAtColumn_row_makeIfNecessary(0, 0, true);
        table.layoutSubtreeIfNeeded();
        cell.layoutSubtreeIfNeeded();
        out::put(&mut t, "made", out::boolean(made.is_some()));
        out::put(
            &mut t,
            "effectiveStyle_raw",
            out::int_isize(table.effectiveStyle().0),
        );
        out::put(
            &mut t,
            "effectiveRowSizeStyle_raw",
            out::int_isize(table.effectiveRowSizeStyle().0),
        );
        out::put(&mut t, "rowHeight", out::float(table.rowHeight()));
        out::put(
            &mut t,
            "cell_rowSizeStyle_raw",
            out::int_isize(cell.rowSizeStyle().0),
        );
        out::put(&mut t, "cell_frame_xywh", rect(cell.frame()));
        if let Some(font) = field.font() {
            out::put(
                &mut t,
                "textField_font_pointSize",
                out::float(font.pointSize()),
            );
        }
        let view: &NSView = &image_view;
        let mut p = self.view_reads(view);
        let set = |i: Option<&NSImage>| {
            image_view.setImage(i);
            cell.layoutSubtreeIfNeeded();
        };
        self.glyphs(&mut p, view, &set, dir, rel, stem);
        self.mode_glyph(&mut p, view, &set, dir, rel, stem);
        out::put_table(&mut t, "image_view", p);
        ffi::set_table_source(&table, None);
        scroll.removeFromSuperview();
        drop(source);
        t
    }

    /// Toolbars with a bordered and a plain image item, per size mode and
    /// display mode, in a key titled window.
    // NSToolbarSizeMode is deprecated (AppKit says it will ignore it), still
    // set and read.
    #[allow(deprecated)]
    fn toolbar(&self) -> Table {
        let mut t = Table::new();
        out::put(
            &mut t,
            "setup",
            out::text(
                "a titled window (empty title) given an NSToolbar whose delegate gives two NSToolbarItems with the symbol as image and label \"Item\": bordered (bordered YES) and plain (bordered NO); per sizeMode (regular, small; deprecated, AppKit may ignore it) and displayMode (iconOnly, iconAndLabel) a fresh window and toolbar, the window made key and the run loop pumped; the window's views walked from its content view's superview for image views and buttons whose image has the probe's accessibility description, each read and rendered as the other places (the blank render after both items' image is set to the blank and the run loop pumped); where none is found, every view walked (walk: [depth, class, frame_in_window_xywh]); the window captured with screencapture -o -l to toolbar/<case>.png",
            ),
        );
        for (size_name, size_mode) in [
            ("regular", objc2_app_kit::NSToolbarSizeMode::Regular),
            ("small", objc2_app_kit::NSToolbarSizeMode::Small),
        ] {
            for (display_name, display) in [
                ("iconOnly", NSToolbarDisplayMode::IconOnly),
                ("iconAndLabel", NSToolbarDisplayMode::IconAndLabel),
            ] {
                let case = format!("{size_name}-{display_name}");
                out::put_table(&mut t, &case, self.toolbar_case(&case, size_mode, display));
            }
        }
        t
    }

    #[allow(deprecated)]
    fn toolbar_case(
        &self,
        case: &str,
        size_mode: objc2_app_kit::NSToolbarSizeMode,
        display: NSToolbarDisplayMode,
    ) -> Table {
        let mut t = Table::new();
        let window = match ffi::titled_window(self.mtm, TOOLBAR_FRAME) {
            Ok(window) => window,
            Err(reason) => {
                out::put(&mut t, "error", out::unavailable(reason));
                return t;
            }
        };
        window.setTitle(&ns(""));
        let item = |id: &str, bordered: bool| {
            let item =
                NSToolbarItem::initWithItemIdentifier(NSToolbarItem::alloc(self.mtm), &ns(id));
            item.setAutovalidates(false);
            item.setEnabled(true);
            item.setLabel(&ns("Item"));
            item.setImage(Some(&self.star));
            item.setBordered(bordered);
            item
        };
        let items = vec![item("bordered", true), item("plain", false)];
        let delegate = ffi::ToolbarDelegate::new(self.mtm, items.clone());
        let toolbar = NSToolbar::initWithIdentifier(
            NSToolbar::alloc(self.mtm),
            &ns(&format!("NativeThemeOsProbeSymbols-{case}")),
        );
        toolbar.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
        toolbar.setDisplayMode(display);
        // Deprecated (AppKit says it will ignore it), still set and read.
        #[allow(deprecated)]
        toolbar.setSizeMode(size_mode);
        window.setToolbar(Some(&toolbar));
        window.makeKeyAndOrderFront(None);
        activate_ignoring(self.app);
        pump(self.app, KEY_WAIT_STEPS, || window.isKeyWindow());
        pump(self.app, SETTLE_STEPS, || false);
        out::put(&mut t, "isKeyWindow", out::boolean(window.isKeyWindow()));
        out::put(
            &mut t,
            "toolbarStyle_raw",
            out::int_isize(window.toolbarStyle().0),
        );
        out::put(&mut t, "displayMode_raw", count(toolbar.displayMode().0));
        #[allow(deprecated)]
        let read_size = toolbar.sizeMode().0;
        out::put(&mut t, "sizeMode_raw", count(read_size));
        let frame = window.frame();
        out::put(&mut t, "window_frame_xywh", rect(frame));
        out::put(
            &mut t,
            "titleBar_and_toolbar_height",
            out::float(frame.size.height - window.contentRectForFrameRect(frame).size.height),
        );
        let dir = self.out_dir.join("toolbar");
        if let Err(e) = std::fs::create_dir_all(&dir) {
            out::put(
                &mut t,
                "error",
                out::unavailable(format!("cannot create {}: {e}", dir.display())),
            );
            window.setToolbar(None);
            window.close();
            return t;
        }
        let path = dir.join(format!("{case}.png"));
        match screencapture(window.windowNumber(), &path) {
            Ok(()) => out::put(&mut t, "capture", out::text(format!("toolbar/{case}.png"))),
            Err(reason) => out::put(&mut t, "capture", out::unavailable(reason)),
        }
        let found = find_symbol_views(&window);
        out::put(&mut t, "symbol_views_found", count(found.len()));
        let mut views = Table::new();
        for (index, view) in found.iter().enumerate() {
            let mut p = self.view_reads(view);
            let n = format!("view_{index}");
            let set = |i: Option<&NSImage>| {
                for it in &items {
                    it.setImage(i);
                }
                pump(self.app, IMAGE_STEPS, || false);
            };
            self.glyphs(&mut p, view, &set, &dir, "toolbar", &format!("{case}-{n}"));
            out::put_table(&mut views, &n, p);
        }
        out::put_table(&mut t, "views", views);
        if found.is_empty() {
            out::put(&mut t, "walk", walk_value(&window));
        }
        window.setToolbar(None);
        window.close();
        drop(delegate);
        t
    }

    /// A status item showing the symbol.
    fn status_item(&self) -> Table {
        let mut t = Table::new();
        out::put(
            &mut t,
            "setup",
            out::text(
                "NSStatusBar systemStatusBar statusItemWithLength: NSVariableStatusItemLength, its button's image the symbol, the run loop pumped; the button read and rendered as the other places; its window captured with screencapture -o -l to status_item/capture.png; the item removed after",
            ),
        );
        let bar = NSStatusBar::systemStatusBar();
        out::put(&mut t, "thickness", out::float(bar.thickness()));
        let item = bar.statusItemWithLength(NSVariableStatusItemLength);
        let Some(button) = item.button(self.mtm) else {
            out::put(&mut t, "button", out::unavailable("nil"));
            bar.removeStatusItem(&item);
            return t;
        };
        button.setImage(Some(&self.star));
        pump(self.app, SETTLE_STEPS, || false);
        out::put(&mut t, "length", out::float(item.length()));
        let dir = self.out_dir.join("status_item");
        if let Err(e) = std::fs::create_dir_all(&dir) {
            out::put(
                &mut t,
                "error",
                out::unavailable(format!("cannot create {}: {e}", dir.display())),
            );
            bar.removeStatusItem(&item);
            return t;
        }
        if let Some(window) = button.window() {
            out::put(&mut t, "window_frame_xywh", rect(window.frame()));
            let path = dir.join("capture.png");
            match screencapture(window.windowNumber(), &path) {
                Ok(()) => out::put(&mut t, "capture", out::text("status_item/capture.png")),
                Err(reason) => out::put(&mut t, "capture", out::unavailable(reason)),
            }
        }
        let view: &NSView = &button;
        let mut p = self.view_reads(view);
        let set = |i: Option<&NSImage>| {
            button.setImage(i);
            pump(self.app, IMAGE_STEPS, || false);
        };
        self.glyphs(&mut p, view, &set, &dir, "status_item", "button");
        self.mode_glyph(&mut p, view, &set, &dir, "status_item", "button");
        out::put_table(&mut t, "button", p);
        bar.removeStatusItem(&item);
        t
    }

    /// An alert's image views, per alert style, after `layout`.
    fn alert(&self) -> Table {
        let mut t = Table::new();
        out::put(
            &mut t,
            "setup",
            out::text(
                "NSAlert new, messageText \"Message\", setAlertStyle:, layout (never run); its window's image views walked from the content view's superview: [depth, class, frame_in_window_xywh, image size_wh]; alert.icon's size",
            ),
        );
        for (name, style) in [
            ("warning", NSAlertStyle::Warning),
            ("informational", NSAlertStyle::Informational),
            ("critical", NSAlertStyle::Critical),
        ] {
            let mut a = Table::new();
            let alert = NSAlert::new(self.mtm);
            alert.setMessageText(&ns("Message"));
            alert.setAlertStyle(style);
            alert.layout();
            match alert.icon() {
                Some(icon) => out::put(&mut a, "icon_size_wh", size(icon.size())),
                None => out::put(&mut a, "icon_size_wh", out::unavailable("nil")),
            }
            let window = alert.window();
            out::put(&mut a, "window_frame_xywh", rect(window.frame()));
            let mut rows = Vec::new();
            if let Some(root) = window.contentView().and_then(|c| ffi::superview(&c)) {
                walk(&root, 0, &mut |view, depth| {
                    let obj: &AnyObject = view.as_ref();
                    if let Some(iv) = obj.downcast_ref::<NSImageView>() {
                        rows.push(out::array(vec![
                            Value::Integer(i64::from(depth)),
                            out::text(view.class().name().to_string_lossy()),
                            rect(view.convertRect_toView(view.bounds(), None)),
                            iv.image()
                                .map_or_else(|| out::text("no image"), |i| size(i.size())),
                        ]));
                    }
                });
            }
            out::put(&mut a, "image_views", out::array(rows));
            out::put_table(&mut t, name, a);
        }
        t
    }
}

/// The button kinds the pass places.
#[derive(Clone, Copy)]
enum Kind {
    Borderless,
    Push,
    PushTitled,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Self::Borderless => "borderless",
            Self::Push => "push",
            Self::PushTitled => "push_titled",
        }
    }

    fn setup(self) -> &'static str {
        match self {
            Self::Borderless => {
                "NSButton new, setButtonType: MomentaryPushIn (PushOnPushOff for on), setImage: the symbol, setBordered: NO, imagePosition NSImageOnly, setControlSize: and the systemFontOfSize: of systemFontSizeForControlSize:, sizeToFit, frame rounded up; then the state: disabled setEnabled: NO, pressed highlight: YES, on setState: NSControlStateValueOn"
            }
            Self::Push => {
                "NSButton new, setButtonType: MomentaryPushIn (PushOnPushOff for on), setImage: the symbol, bezelStyle push, imagePosition NSImageOnly, setControlSize: and the systemFontOfSize: of systemFontSizeForControlSize:, sizeToFit, frame rounded up; then the state: disabled setEnabled: NO, pressed highlight: YES, on setState: NSControlStateValueOn"
            }
            Self::PushTitled => {
                "NSButton new, setButtonType: MomentaryPushIn (PushOnPushOff for on), setImage: the symbol, bezelStyle push, title \"Button\", imagePosition NSImageLeft, setControlSize: and the systemFontOfSize: of systemFontSizeForControlSize:, sizeToFit, frame rounded up; then the state: disabled setEnabled: NO, pressed highlight: YES, on setState: NSControlStateValueOn"
            }
        }
    }
}

/// The views of `window` (from its content view's superview) that are
/// image views or buttons whose image has the probe's description.
fn find_symbol_views(window: &NSWindow) -> Vec<Retained<NSView>> {
    let mut found = Vec::new();
    let Some(root) = window.contentView().and_then(|c| ffi::superview(&c)) else {
        return found;
    };
    walk(&root, 0, &mut |view, _| {
        let obj: &AnyObject = view.as_ref();
        let image = obj
            .downcast_ref::<NSImageView>()
            .and_then(NSImageView::image)
            .or_else(|| obj.downcast_ref::<NSButton>().and_then(NSButton::image));
        let ours = image
            .and_then(|i| i.accessibilityDescription())
            .is_some_and(|d| d.to_string() == DESCRIPTION);
        if ours {
            found.push(view.retain());
        }
    });
    found
}

/// Every view of `window` from its content view's superview: depth, class
/// and frame in the window.
fn walk_value(window: &NSWindow) -> Value {
    let mut rows = Vec::new();
    let Some(root) = window.contentView().and_then(|c| ffi::superview(&c)) else {
        return out::unavailable("the content view has no superview");
    };
    walk(&root, 0, &mut |view, depth| {
        rows.push(out::array(vec![
            Value::Integer(i64::from(depth)),
            out::text(view.class().name().to_string_lossy()),
            rect(view.convertRect_toView(view.bounds(), None)),
        ]));
    });
    out::array(rows)
}

/// `f` on `view` and every view below it, `WALK_DEPTH` deep.
fn walk(view: &NSView, depth: u32, f: &mut dyn FnMut(&NSView, u32)) {
    f(view, depth);
    if depth >= WALK_DEPTH {
        return;
    }
    for sub in view.subviews().iter() {
        walk(&sub, depth.saturating_add(1), f);
    }
}

fn glyph_table(g: &Glyph) -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "bbox_px_xywh",
        out::array(g.bbox.iter().map(|v| count(*v)).collect()),
    );
    out::put(&mut t, "max_coverage", out::float(rounded(g.max, 10000.0)));
    out::put(&mut t, "ink", out::float(rounded(g.ink, 100.0)));
    t
}
