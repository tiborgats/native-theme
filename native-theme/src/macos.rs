//! macOS platform helpers: the running macOS version, for the generation
//! table, and the system UI font Core Text resolves, for
//! [`crate::fonts`].

// Objective-C FFI via objc2 -- no safe alternative
#![allow(unsafe_code)]

#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2_foundation::NSProcessInfo;

/// The running macOS version, from `NSProcessInfo.operatingSystemVersion`
/// (a safe binding in objc2-foundation 0.3.2), for the generation table.
/// AppKit's components are never negative; one that were would read as 0, and
/// a version of 0.0.0 gets `macos-monterey` with `VersionMatch::PresetNewer`.
/// Early macOS 26 betas reported 16 here to a binary built against the macOS
/// 15 SDK; `pipeline::MACOS_GENERATIONS` gives the sources and the mapping.
#[cfg(all(target_os = "macos", feature = "macos"))]
pub(crate) fn os_version() -> crate::pipeline::OsVersion {
    let v = NSProcessInfo::processInfo().operatingSystemVersion();
    let part = |n: isize| u32::try_from(n).unwrap_or_default();
    crate::pipeline::OsVersion::new(
        part(v.majorVersion),
        part(v.minorVersion),
        part(v.patchVersion),
    )
}

/// The macOS system UI font as Core Text resolves it: its family name and
/// the file it lives in (egui connector spec §8.2).
#[cfg(all(target_os = "macos", feature = "system-fonts"))]
pub(crate) struct SystemUiFont {
    /// Core Text's family name for the font (`kCTFontFamilyNameAttribute`).
    pub(crate) family: String,
    /// The font file (`kCTFontURLAttribute` as a file path).
    pub(crate) path: std::path::PathBuf,
}

/// Ask Core Text which font is the system UI font — `CTFontUIFontType::System`
/// at the type's default size, no language — and, for `italic`, its italic
/// variant (the upright font where Core Text has none). `None` where Core
/// Text returns none, an attribute is missing or is not the type the
/// attribute documents, or the URL is not a file path. `CTFont::family_name`
/// is not used: its binding `expect`s a non-null result (objc2-core-text
/// 0.3.2 `src/generated/CTFont.rs` lines 977–983).
#[cfg(all(target_os = "macos", feature = "system-fonts"))]
pub(crate) fn system_ui_font(italic: bool) -> Option<SystemUiFont> {
    use objc2_core_foundation::{CFString, CFURL};
    use objc2_core_text::{
        CTFont, CTFontSymbolicTraits, CTFontUIFontType, kCTFontFamilyNameAttribute,
        kCTFontURLAttribute,
    };
    // SAFETY: `CTFontCreateUIFontForLanguage` takes a UI font type, a size
    // (0.0 = the type's default) and an optional language; a null language
    // is documented as allowed, and the binding maps a NULL result to None.
    let upright = unsafe { CTFont::new_ui_font_for_language(CTFontUIFontType::System, 0.0, None) }?;
    let font = if italic {
        // SAFETY: a null matrix is documented as allowed by the binding's
        // safety note; the value and mask both name the italic trait.
        let slanted = unsafe {
            upright.copy_with_symbolic_traits(
                0.0,
                std::ptr::null(),
                CTFontSymbolicTraits::ItalicTrait,
                CTFontSymbolicTraits::ItalicTrait,
            )
        };
        slanted.unwrap_or(upright)
    } else {
        upright
    };
    // SAFETY: the two keys are Core Text's own statics, declared in `extern`
    // blocks; reading them is an FFI read of a constant.
    let (family_key, url_key) = unsafe { (kCTFontFamilyNameAttribute, kCTFontURLAttribute) };
    // SAFETY: `CTFontCopyAttribute` returns a retained reference or NULL,
    // which the binding maps to None; the downcasts check the type id.
    let family = unsafe { font.attribute(family_key) }?
        .downcast::<CFString>()
        .ok()?
        .to_string();
    // SAFETY: as above: `url_key` is Core Text's own static, `font.attribute`
    // returns a retained reference or NULL (mapped to None), and the downcast
    // checks the type id.
    let path = unsafe { font.attribute(url_key) }?
        .downcast::<CFURL>()
        .ok()?
        .to_file_path()?;
    Some(SystemUiFont { family, path })
}
