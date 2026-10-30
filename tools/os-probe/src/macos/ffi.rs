//! The macOS probe's calls that objc2-app-kit 0.3 marks `unsafe`, each in a
//! block of its own; the rest of the probe (`../macos.rs`) has none.

// Objective-C FFI via objc2 -- no safe alternative.
#![allow(unsafe_code)]

use std::ptr::NonNull;

use objc2::rc::Retained;
use objc2::runtime::NSObjectProtocol;
use objc2::{AnyThread, DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSBackingStoreType, NSBitmapFormat, NSBitmapImageFileType, NSBitmapImageRep, NSButton, NSCell,
    NSColorSpace, NSDeviceRGBColorSpace, NSFont, NSFontTextStyle, NSFontTextStyleBody,
    NSFontTextStyleCallout, NSFontTextStyleCaption1, NSFontTextStyleCaption2,
    NSFontTextStyleFootnote, NSFontTextStyleHeadline, NSFontTextStyleLargeTitle,
    NSFontTextStyleSubheadline, NSFontTextStyleTitle1, NSFontTextStyleTitle2,
    NSFontTextStyleTitle3, NSFontTraitsAttribute, NSFontWeightBlack, NSFontWeightBold,
    NSFontWeightHeavy, NSFontWeightLight, NSFontWeightMedium, NSFontWeightRegular,
    NSFontWeightSemibold, NSFontWeightThin, NSFontWeightTrait, NSFontWeightUltraLight, NSToolbar,
    NSToolbarDelegate, NSToolbarItem, NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{
    NSArray, NSDefaultRunLoopMode, NSDictionary, NSObject, NSRect, NSRunLoopMode, NSString,
};

/// `NSFontWeightRegular`.
pub(super) fn font_weight_regular() -> Result<f64, String> {
    // SAFETY: reads an AppKit `extern` CGFloat constant, as
    // native-theme/src/macos.rs does.
    Ok(unsafe { NSFontWeightRegular })
}

/// The nine `NSFontWeight` constants, lightest first, keyed by their short
/// names.
pub(super) fn font_weights() -> Result<[(&'static str, f64); 9], String> {
    // SAFETY: each reads an AppKit `extern` CGFloat constant.
    Ok(unsafe {
        [
            ("ultraLight", NSFontWeightUltraLight),
            ("thin", NSFontWeightThin),
            ("light", NSFontWeightLight),
            ("regular", NSFontWeightRegular),
            ("medium", NSFontWeightMedium),
            ("semibold", NSFontWeightSemibold),
            ("bold", NSFontWeightBold),
            ("heavy", NSFontWeightHeavy),
            ("black", NSFontWeightBlack),
        ]
    })
}

/// `-[NSCell getPeriodicDelay:interval:]`: `(delay, interval)` in seconds.
pub(super) fn periodic_delay(cell: &NSCell) -> (f32, f32) {
    let mut delay: f32 = 0.0;
    let mut interval: f32 = 0.0;
    // SAFETY: both pointers come from live, aligned, initialised locals that
    // outlive the call, which only writes one float through each.
    unsafe {
        cell.getPeriodicDelay_interval(NonNull::from(&mut delay), NonNull::from(&mut interval));
    }
    (delay, interval)
}

/// `+[NSButton buttonWithTitle:target:action:]` with no target and no
/// action: the push button AppKit makes by default.
pub(super) fn standard_button(mtm: MainThreadMarker, title: &NSString) -> Retained<NSButton> {
    // SAFETY: a nil target and a NULL action, which the method documents as
    // allowed (the button then sends nothing).
    unsafe { NSButton::buttonWithTitle_target_action(title, None, None, mtm) }
}

/// The eleven `NSFontTextStyle` constants, keyed by their short names.
pub(super) fn text_styles() -> Result<[(&'static str, &'static NSFontTextStyle); 11], String> {
    // SAFETY: each reads an AppKit `extern` NSString constant.
    Ok(unsafe {
        [
            ("largeTitle", NSFontTextStyleLargeTitle),
            ("title1", NSFontTextStyleTitle1),
            ("title2", NSFontTextStyleTitle2),
            ("title3", NSFontTextStyleTitle3),
            ("headline", NSFontTextStyleHeadline),
            ("subheadline", NSFontTextStyleSubheadline),
            ("body", NSFontTextStyleBody),
            ("callout", NSFontTextStyleCallout),
            ("footnote", NSFontTextStyleFootnote),
            ("caption1", NSFontTextStyleCaption1),
            ("caption2", NSFontTextStyleCaption2),
        ]
    })
}

/// `+[NSFont preferredFontForTextStyle:options:]` with no options.
pub(super) fn preferred_font(style: &NSFontTextStyle) -> Result<Retained<NSFont>, String> {
    let options = NSDictionary::new();
    // SAFETY: a text style constant and an empty options dictionary, which
    // the method documents as allowed.
    Ok(unsafe { NSFont::preferredFontForTextStyle_options(style, &options) })
}

/// `(NSFontTraitsAttribute, NSFontWeightTrait)`.
pub(super) fn font_trait_keys() -> Result<(&'static NSString, &'static NSString), String> {
    // SAFETY: reads AppKit `extern` NSString constants, as
    // native-theme/src/macos.rs does.
    Ok(unsafe { (NSFontTraitsAttribute, NSFontWeightTrait) })
}

/// A borderless, buffered, non-deferred NSWindow that is never ordered
/// front and not released on close.
pub(super) fn offscreen_window(
    mtm: MainThreadMarker,
    frame: NSRect,
) -> Result<Retained<NSWindow>, String> {
    // SAFETY: `initWithContentRect:styleMask:backing:defer:` with a plain
    // rect, a valid style mask and backing type; the result is owned by the
    // returned `Retained`.
    let window = unsafe {
        NSWindow::initWithContentRect_styleMask_backing_defer(
            NSWindow::alloc(mtm),
            frame,
            NSWindowStyleMask::Borderless,
            NSBackingStoreType::Buffered,
            false,
        )
    };
    // SAFETY: the window is never closed and `Retained` owns it; not
    // releasing it on close keeps that ownership the only one (objc2's
    // documented pattern for NSWindow).
    unsafe { window.setReleasedWhenClosed(false) };
    Ok(window)
}

/// A titled, closable, miniaturizable, resizable, buffered, non-deferred
/// NSWindow, not released on close, for the probe to order front and make
/// key: the three traffic lights draw their key-window colours.
pub(super) fn titled_window(
    mtm: MainThreadMarker,
    frame: NSRect,
) -> Result<Retained<NSWindow>, String> {
    // SAFETY: `initWithContentRect:styleMask:backing:defer:` with a plain
    // rect, a valid style mask and backing type; the result is owned by the
    // returned `Retained`.
    let window = unsafe {
        NSWindow::initWithContentRect_styleMask_backing_defer(
            NSWindow::alloc(mtm),
            frame,
            NSWindowStyleMask::Titled
                | NSWindowStyleMask::Closable
                | NSWindowStyleMask::Miniaturizable
                | NSWindowStyleMask::Resizable,
            NSBackingStoreType::Buffered,
            false,
        )
    };
    // SAFETY: `Retained` owns the window, and the probe closes it before the
    // `Retained` is dropped; not releasing it on close keeps that ownership
    // the only one (objc2's documented pattern for NSWindow).
    unsafe { window.setReleasedWhenClosed(false) };
    Ok(window)
}

/// `NSDefaultRunLoopMode`, the mode the probe pumps its events in.
pub(super) fn default_run_loop_mode() -> &'static NSRunLoopMode {
    // SAFETY: reads a Foundation `extern` NSString constant.
    unsafe { NSDefaultRunLoopMode }
}

/// An 8-bit RGBA NSBitmapImageRep of `width` x `height` pixels that owns
/// its buffer.
pub(super) fn rgba_bitmap(
    width: isize,
    height: isize,
) -> Result<Retained<NSBitmapImageRep>, String> {
    // SAFETY: reads an AppKit `extern` NSString constant.
    let color_space = unsafe { NSDeviceRGBColorSpace };
    // SAFETY: NULL planes ask the rep to allocate and own its buffer; 8 bits
    // per sample, 4 samples with alpha, meshed; 0 row bytes and pixel bits
    // let AppKit compute them, as the method documents.
    let rep = unsafe {
        NSBitmapImageRep::initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bytesPerRow_bitsPerPixel(
            NSBitmapImageRep::alloc(),
            std::ptr::null_mut(),
            width,
            height,
            8,
            4,
            true,
            false,
            color_space,
            0,
            0,
        )
    };
    rep.ok_or_else(|| format!("initWithBitmapDataPlanes: is nil for {width} x {height}"))
}

/// The samples of the pixel at (`x`, `y`) of `rep`, a meshed 8-bit bitmap:
/// `samplesPerPixel` bytes from its buffer, as stored.
pub(super) fn pixel_bytes(rep: &NSBitmapImageRep, x: isize, y: isize) -> Result<Vec<u8>, String> {
    if rep.isPlanar() {
        return Err("the bitmap is planar".to_owned());
    }
    if rep.bitsPerSample() != 8 {
        return Err(format!("{} bits per sample", rep.bitsPerSample()));
    }
    let (wide, high) = (rep.pixelsWide(), rep.pixelsHigh());
    if !(0..wide).contains(&x) || !(0..high).contains(&y) {
        return Err(format!("({x}, {y}) is outside {wide} x {high} px"));
    }
    let row = rep.bytesPerRow();
    let samples = rep.samplesPerPixel();
    let pixel = rep
        .bitsPerPixel()
        .checked_div(8)
        .ok_or_else(|| "no pixel size".to_owned())?;
    if samples > pixel {
        return Err(format!("{samples} samples in {pixel} bytes a pixel"));
    }
    let to_usize = |v: isize| usize::try_from(v).map_err(|_| format!("{v} is negative"));
    let len = to_usize(
        row.checked_mul(high)
            .ok_or_else(|| "the buffer's size overflows".to_owned())?,
    )?;
    let start = to_usize(
        y.checked_mul(row)
            .and_then(|o| x.checked_mul(pixel).and_then(|p| o.checked_add(p)))
            .ok_or_else(|| "the pixel's offset overflows".to_owned())?,
    )?;
    let end = start
        .checked_add(to_usize(samples)?)
        .ok_or_else(|| "the pixel's end overflows".to_owned())?;
    let data = rep.bitmapData();
    if data.is_null() {
        return Err("bitmapData is NULL".to_owned());
    }
    // SAFETY: a meshed (non-planar) rep keeps its pixels in one buffer of
    // bytesPerRow x pixelsHigh bytes at bitmapData, owned by `rep`, which
    // outlives this borrow; nothing writes to it while the slice lives.
    let buffer = unsafe { std::slice::from_raw_parts(data, len) };
    buffer
        .get(start..end)
        .map(<[u8]>::to_vec)
        .ok_or_else(|| format!("bytes {start}..{end} lie outside the {len} byte buffer"))
}

/// A copy of `rep`'s buffer, a meshed 8-bit bitmap: `bytesPerRow` x
/// `pixelsHigh` bytes, as stored.
pub(super) fn bitmap_buffer(rep: &NSBitmapImageRep) -> Result<Vec<u8>, String> {
    if rep.isPlanar() {
        return Err("the bitmap is planar".to_owned());
    }
    if rep.bitsPerSample() != 8 {
        return Err(format!("{} bits per sample", rep.bitsPerSample()));
    }
    let len = rep
        .bytesPerRow()
        .checked_mul(rep.pixelsHigh())
        .and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| "the buffer's size overflows or is negative".to_owned())?;
    let data = rep.bitmapData();
    if data.is_null() {
        return Err("bitmapData is NULL".to_owned());
    }
    // SAFETY: a meshed (non-planar) rep keeps its pixels in one buffer of
    // bytesPerRow x pixelsHigh bytes at bitmapData, owned by `rep`, which
    // outlives this borrow; nothing writes to it while the slice lives.
    let buffer = unsafe { std::slice::from_raw_parts(data, len) };
    Ok(buffer.to_vec())
}

/// `pixels`, 8-bit RGBA with straight (not premultiplied) alpha, rows top to
/// bottom, `width` x `height`, encoded as PNG, tagged sRGB.
pub(super) fn rgba_png(width: usize, height: usize, pixels: &[u8]) -> Result<Vec<u8>, String> {
    let row = width
        .checked_mul(4)
        .ok_or_else(|| "the row's size overflows".to_owned())?;
    let len = row
        .checked_mul(height)
        .ok_or_else(|| "the image's size overflows".to_owned())?;
    if pixels.len() != len {
        return Err(format!("{} bytes for {width} x {height} px", pixels.len()));
    }
    let to_isize = |v: usize| isize::try_from(v).map_err(|_| format!("{v} does not fit"));
    // SAFETY: reads an AppKit `extern` NSString constant.
    let color_space = unsafe { NSDeviceRGBColorSpace };
    // SAFETY: NULL planes ask the rep to allocate and own its buffer; 8 bits
    // per sample, 4 samples with alpha, meshed, straight alpha, and the row
    // and pixel sizes the buffer is filled by below.
    let rep = unsafe {
        NSBitmapImageRep::initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bitmapFormat_bytesPerRow_bitsPerPixel(
            NSBitmapImageRep::alloc(),
            std::ptr::null_mut(),
            to_isize(width)?,
            to_isize(height)?,
            8,
            4,
            true,
            false,
            color_space,
            NSBitmapFormat::AlphaNonpremultiplied,
            to_isize(row)?,
            32,
        )
    }
    .ok_or_else(|| format!("initWithBitmapDataPlanes: is nil for {width} x {height}"))?;
    if usize::try_from(rep.bytesPerRow()).ok() != Some(row) {
        return Err(format!("bytesPerRow is {}, not {row}", rep.bytesPerRow()));
    }
    let data = rep.bitmapData();
    if data.is_null() {
        return Err("bitmapData is NULL".to_owned());
    }
    // SAFETY: the rep owns a buffer of bytesPerRow (checked to be `row`) x
    // `height` = `len` bytes at bitmapData; `rep` outlives the slice and
    // nothing else reads or writes the buffer while it lives.
    let buffer = unsafe { std::slice::from_raw_parts_mut(data, len) };
    buffer.copy_from_slice(pixels);
    let tagged = rep
        .bitmapImageRepByRetaggingWithColorSpace(&NSColorSpace::sRGBColorSpace())
        .unwrap_or(rep);
    png_data(&tagged)
}

/// What the toolbar delegate hands its toolbar: the items, made beforehand.
pub(super) struct ToolbarItems {
    items: Vec<Retained<NSToolbarItem>>,
}

define_class!(
    // SAFETY: NSObject has no subclassing requirements, and `ToolbarDelegate`
    // does not implement `Drop`.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "NativeThemeOsProbeToolbarDelegate"]
    #[ivars = ToolbarItems]
    pub(super) struct ToolbarDelegate;

    // SAFETY: NSObjectProtocol has no requirements beyond NSObject's.
    unsafe impl NSObjectProtocol for ToolbarDelegate {}

    // SAFETY: each method's selector, argument and return types are
    // NSToolbarDelegate's (objc2-app-kit's `NSToolbarDelegate` trait).
    unsafe impl NSToolbarDelegate for ToolbarDelegate {
        #[unsafe(method_id(toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:))]
        fn item_for_identifier(
            &self,
            _toolbar: &NSToolbar,
            identifier: &NSString,
            _will_be_inserted: bool,
        ) -> Option<Retained<NSToolbarItem>> {
            self.ivars()
                .items
                .iter()
                .find(|item| &*item.itemIdentifier() == identifier)
                .cloned()
        }

        #[unsafe(method_id(toolbarDefaultItemIdentifiers:))]
        fn default_identifiers(&self, _toolbar: &NSToolbar) -> Retained<NSArray<NSString>> {
            self.identifiers()
        }

        #[unsafe(method_id(toolbarAllowedItemIdentifiers:))]
        fn allowed_identifiers(&self, _toolbar: &NSToolbar) -> Retained<NSArray<NSString>> {
            self.identifiers()
        }
    }
);

impl ToolbarDelegate {
    /// A delegate that gives a toolbar `items`, in order.
    pub(super) fn new(
        mtm: MainThreadMarker,
        items: Vec<Retained<NSToolbarItem>>,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(ToolbarItems { items });
        // SAFETY: NSObject's `init` on an allocated instance whose ivars are
        // set, as objc2's `define_class!` documents.
        unsafe { msg_send![super(this), init] }
    }

    fn identifiers(&self) -> Retained<NSArray<NSString>> {
        let ids: Vec<Retained<NSString>> = self
            .ivars()
            .items
            .iter()
            .map(|item| item.itemIdentifier())
            .collect();
        NSArray::from_retained_slice(&ids)
    }
}

/// `rep` encoded as PNG.
pub(super) fn png_data(rep: &NSBitmapImageRep) -> Result<Vec<u8>, String> {
    let properties = NSDictionary::new();
    // SAFETY: the PNG file type and an empty properties dictionary, which
    // the method documents as allowed.
    let data =
        unsafe { rep.representationUsingType_properties(NSBitmapImageFileType::PNG, &properties) };
    data.map(|d| d.to_vec())
        .ok_or_else(|| "representationUsingType:PNG is nil".to_owned())
}
