//! The macOS probe's calls that objc2-app-kit 0.3 marks `unsafe`, each in a
//! block of its own; the rest of the probe (`../macos.rs`) has none.

// Objective-C FFI via objc2 -- no safe alternative.
#![allow(unsafe_code)]

use objc2::rc::Retained;
use objc2::{AnyThread, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSBackingStoreType, NSBitmapImageFileType, NSBitmapImageRep, NSDeviceRGBColorSpace, NSFont,
    NSFontTextStyle, NSFontTextStyleBody, NSFontTextStyleCallout, NSFontTextStyleCaption1,
    NSFontTextStyleCaption2, NSFontTextStyleFootnote, NSFontTextStyleHeadline,
    NSFontTextStyleLargeTitle, NSFontTextStyleSubheadline, NSFontTextStyleTitle1,
    NSFontTextStyleTitle2, NSFontTextStyleTitle3, NSFontTraitsAttribute, NSFontWeightRegular,
    NSFontWeightTrait, NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{NSDefaultRunLoopMode, NSDictionary, NSRect, NSRunLoopMode, NSString};

/// `NSFontWeightRegular`.
pub(super) fn font_weight_regular() -> Result<f64, String> {
    // SAFETY: reads an AppKit `extern` CGFloat constant, as
    // native-theme/src/macos.rs does.
    Ok(unsafe { NSFontWeightRegular })
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
