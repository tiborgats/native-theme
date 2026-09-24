// Linux freedesktop icon theme lookup
//
// Resolves IconRole variants to SVG bytes from the user's active desktop
// icon theme (Adwaita, Breeze, Papirus, etc.) using the freedesktop-icons
// crate. Returns None when the role has no freedesktop mapping or the
// icon is not found in the active theme or a theme it inherits from;
// hicolor, loose files and pixmaps never stand in for a missing icon.

use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::num::NonZeroU32;
use std::sync::{Mutex, OnceLock};

use crate::IconData;
use crate::model::animated::{AnimatedIcon, TransformAnimation};
use std::path::{Component, Path, PathBuf};

/// Frame duration for freedesktop sprite sheet animations (80ms per frame).
const FREEDESKTOP_FRAME_DURATION_MS: u32 = 80;

/// Spin duration for single-frame icons animated with rotation (1 second).
const FREEDESKTOP_SPIN_DURATION_MS: u32 = 1000;

/// Detect the current freedesktop icon theme name.
///
/// Delegates to [`crate::system_icon_theme()`] which handles DE-specific
/// detection (KDE reads kdeglobals, GNOME uses gsettings, etc.). The result
/// is cached by [`DetectionContext`](crate::detect::DetectionContext).
fn detect_theme() -> String {
    crate::system_icon_theme()
}

/// The icon base directories freedesktop-icons 0.4.0 searches for themes,
/// in its order (`freedesktop-icons-0.4.0/src/theme/paths.rs:13-32`): each
/// `$XDG_DATA_DIRS` entry's `icons`, `$XDG_DATA_HOME/icons`, then
/// `~/.icons`, keeping those that exist. freedesktop-icons also searches
/// the `pixmaps` directory beside each `icons` one; no UI icon may come
/// from there, so it is left out.
///
/// Read once per process, as freedesktop-icons reads its own list once.
fn icon_base_dirs() -> &'static [PathBuf] {
    static DIRS: OnceLock<Vec<PathBuf>> = OnceLock::new();
    DIRS.get_or_init(xdg_icon_base_dirs)
}

/// Build [`icon_base_dirs`] from the environment the way freedesktop-icons'
/// `xdg` 2.5.2 does (`xdg-2.5.2/src/base_directories.rs:268-309`): a
/// relative or unset `$XDG_DATA_HOME` is `~/.local/share`, relative
/// `$XDG_DATA_DIRS` entries are dropped, and none left is
/// `/usr/local/share:/usr/share`. Without a home directory `xdg` gives no
/// data dirs at all, and there is no `~/.icons` either.
fn xdg_icon_base_dirs() -> Vec<PathBuf> {
    let Some(home) = std::env::home_dir() else {
        return Vec::new();
    };
    let data_dirs: Vec<PathBuf> = std::env::var_os("XDG_DATA_DIRS")
        .map(|dirs| {
            std::env::split_paths(&dirs)
                .filter(|dir| dir.is_absolute())
                .collect::<Vec<_>>()
        })
        .filter(|dirs| !dirs.is_empty())
        .unwrap_or_else(|| {
            vec![
                PathBuf::from("/usr/local/share"),
                PathBuf::from("/usr/share"),
            ]
        });
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|dir| dir.is_absolute())
        .unwrap_or_else(|| home.join(".local/share"));
    let mut dirs: Vec<PathBuf> = data_dirs.iter().map(|dir| dir.join("icons")).collect();
    dirs.push(data_home.join("icons"));
    dirs.push(home.join(".icons"));
    dirs.retain(|dir| dir.exists());
    dirs
}

/// Whether `name` can name a theme directory: one plain path component,
/// so joining it to a base dir stays inside that base dir.
fn is_theme_name(name: &str) -> bool {
    let mut components = Path::new(name).components();
    matches!(
        (components.next(), components.next()),
        (Some(Component::Normal(_)), None)
    )
}

/// The themes listed by the `Inherits=` key of an `index.theme`'s
/// `[Icon Theme]` group, in order.
fn inherits(index: &str) -> Vec<&str> {
    let mut in_icon_theme = false;
    for line in index.lines().map(str::trim) {
        if let Some(group) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            in_icon_theme = group == "Icon Theme";
            continue;
        }
        if in_icon_theme
            && let Some((key, value)) = line.split_once('=')
            && key.trim() == "Inherits"
        {
            return value
                .split(',')
                .map(str::trim)
                .filter(|parent| !parent.is_empty())
                .collect();
        }
    }
    Vec::new()
}

/// The themes an icon for `theme` may come from: `theme` and every theme
/// in its declared `Inherits=` chain, transitively, except `hicolor`
/// unless it is `theme` itself. A theme's parents are read from every
/// `index.theme` it has in `bases`, as freedesktop-icons reads each of
/// them (`freedesktop-icons-0.4.0/src/lib.rs:309-327`).
///
/// `None` when `theme` has no `index.theme` in any of `bases`: the theme
/// does not exist, and nothing may stand in for it.
fn theme_chain(theme: &str, bases: &[PathBuf]) -> Option<Vec<String>> {
    if !is_theme_name(theme)
        || !bases
            .iter()
            .any(|base| base.join(theme).join("index.theme").exists())
    {
        return None;
    }
    let mut chain = vec![theme.to_string()];
    let mut next = 0usize;
    while let Some(name) = chain.get(next).cloned() {
        next = next.saturating_add(1);
        for base in bases {
            let Ok(index) = std::fs::read_to_string(base.join(&name).join("index.theme")) else {
                continue;
            };
            for parent in inherits(&index) {
                if parent != "hicolor"
                    && is_theme_name(parent)
                    && !chain.iter().any(|known| known == parent)
                {
                    chain.push(parent.to_string());
                }
            }
        }
    }
    Some(chain)
}

/// [`theme_chain`] over [`icon_base_dirs`], kept per theme name for the
/// life of the process, as freedesktop-icons keeps its theme list.
fn cached_theme_chain(theme: &str) -> Option<Vec<String>> {
    type Chains = Mutex<HashMap<String, Option<Vec<String>>>>;
    static CHAINS: OnceLock<Chains> = OnceLock::new();
    let chains = CHAINS.get_or_init(Chains::default);
    if let Ok(known) = chains.lock()
        && let Some(chain) = known.get(theme)
    {
        return chain.clone();
    }
    let chain = theme_chain(theme, icon_base_dirs());
    if let Ok(mut known) = chains.lock() {
        known.insert(theme.to_string(), chain.clone());
    }
    chain
}

/// Whether `path` lies inside the directory of one of `themes` under one
/// of `bases`, compared by path component, so `breeze` does not take a
/// file of `breeze-dark`. The path is not canonicalized: the theme
/// directory freedesktop-icons searched is what decides, even where it
/// links into another theme. A path with a `..` component is refused, as
/// it could leave the directory it names.
fn is_in_theme_dirs(path: &Path, themes: &[String], bases: &[PathBuf]) -> bool {
    if path
        .components()
        .any(|component| component == Component::ParentDir)
    {
        return false;
    }
    bases.iter().any(|base| {
        let Ok(rest) = path.strip_prefix(base) else {
            return false;
        };
        let mut rest = rest.components();
        let in_theme = matches!(
            rest.next(),
            Some(Component::Normal(dir)) if themes.iter().any(|theme| dir == OsStr::new(theme))
        );
        in_theme && rest.next().is_some()
    })
}

/// Look `name` up in `theme` through freedesktop-icons, keeping the result
/// only when it lies in a directory of `chain` (see [`theme_chain`]).
///
/// freedesktop-icons falls back to `hicolor` when the theme does not
/// exist or it and its parents lack the icon, then to loose files in the
/// icon base dirs and `/usr/share/pixmaps`
/// (`freedesktop-icons-0.4.0/src/lib.rs:297-347`); each would be an icon
/// from another set, so each gives `None` here.
fn lookup_in_theme(name: &str, theme: &str, size: u16, chain: &[String]) -> Option<PathBuf> {
    freedesktop_icons::lookup(name)
        .with_theme(theme)
        .with_size(size)
        .force_svg()
        .find()
        .filter(|path| is_in_theme_dirs(path, chain, icon_base_dirs()))
}

/// Look up an icon by freedesktop name using a two-pass strategy.
///
/// First tries the `-symbolic` suffix (single-frame static icons), then
/// falls back to the plain name. This order avoids animation sprite
/// sheets (e.g. Breeze's `animations/process-working.svg` is a 15-frame
/// vertical strip) which render incorrectly as static images.
///
/// The symbolic-first order also naturally handles Adwaita, which stores
/// most action icons only as `*-symbolic.svg`.
///
/// Each pass finds only an icon of `theme` or of a theme it declares in
/// its `Inherits=` chain, never of `hicolor` unless `theme` is `hicolor`
/// (see [`lookup_in_theme`]). `None` when `theme` is not installed.
fn find_icon(name: &str, theme: &str, size: u16) -> Option<(PathBuf, bool)> {
    let chain = cached_theme_chain(theme)?;
    // First try: symbolic variant (e.g., "edit-copy-symbolic")
    // Symbolic icons are always single-frame, avoiding sprite sheets
    // in themes like Breeze that put animation strips under plain names.
    let symbolic = format!("{name}-symbolic");
    if let Some(path) = lookup_in_theme(&symbolic, theme, size, &chain) {
        return Some((path, true));
    }
    // Second try: plain name (e.g., "edit-copy")
    // If the name itself already ends with "-symbolic" (caller passed it
    // explicitly via load_freedesktop_icon_by_name), mark as symbolic.
    lookup_in_theme(name, theme, size, &chain).map(|path| (path, name.ends_with("-symbolic")))
}

/// Load a freedesktop icon by name from the given theme.
///
/// Looks up the name in the specified theme directory (with `-symbolic`
/// suffix fallback for Adwaita-style themes), then dispatches on file
/// extension:
///
/// - `.svg` → returns [`IconData::Svg`]. For GTK-convention symbolic
///   icons (Adwaita, Yaru, elementary), hardcoded foreground placeholders
///   are replaced with `fg_color` so the SVG renders correctly on both
///   light and dark themes. Pass `None` to fall back to `currentColor`
///   (requires connector colorization).
/// - `.png` → decodes the PNG to RGBA and returns [`IconData::Rgba`]
///   (raster-only legacy themes like `AdwaitaLegacy`).
/// - Any other extension → `None`.
///
/// Returns `None` if the icon is not found in the theme or if the
/// decoded file cannot be parsed.
///
/// **Performance note:** Each call reads the icon file from disk. Callers
/// that load the same icon repeatedly should cache the returned `IconData`.
#[must_use]
pub(crate) fn load_freedesktop_icon_by_name(
    name: &str,
    theme: &str,
    size: u16,
    fg_color: Option<[u8; 3]>,
) -> Option<IconData> {
    let (path, is_symbolic) = find_icon(name, theme, size)?;
    load_icon_file(&path, is_symbolic, fg_color)
}

/// Read and decode an icon file, branching on its extension.
///
/// Split out from [`load_freedesktop_icon_by_name`] so format handling
/// is testable without the freedesktop-icons lookup and is easy to
/// extend (e.g. future XPM support).
fn load_icon_file(path: &Path, is_symbolic: bool, fg_color: Option<[u8; 3]>) -> Option<IconData> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);
    match ext.as_deref() {
        Some("svg") => {
            let bytes = std::fs::read(path).ok()?;
            let bytes = if is_symbolic {
                let replacement = fg_to_replacement(fg_color);
                normalize_gtk_symbolic(bytes, &replacement)
            } else {
                bytes
            };
            Some(IconData::Svg(Cow::Owned(bytes)))
        }
        Some("png") => {
            let bytes = std::fs::read(path).ok()?;
            decode_png_to_rgba(&bytes)
        }
        _ => None,
    }
}

/// Decode a PNG byte slice to an 8-bit RGBA [`IconData`].
///
/// Uses `png` crate transformations to normalise palette, grayscale, and
/// 16-bit inputs to 8-bit. Non-RGBA outputs (RGB, Grayscale, GrayscaleAlpha)
/// are expanded to RGBA in-place so downstream renderers only ever see
/// 4-byte-per-pixel data.
///
/// Returns `None` if the input is not a valid PNG, decoding fails, or
/// the output buffer shape does not match `width * height * 4`.
fn decode_png_to_rgba(bytes: &[u8]) -> Option<IconData> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(
        png::Transformations::EXPAND | png::Transformations::STRIP_16 | png::Transformations::ALPHA,
    );
    let mut reader = decoder.read_info().ok()?;
    let mut buf = vec![0u8; reader.output_buffer_size()?];
    let info = reader.next_frame(&mut buf).ok()?;
    buf.truncate(info.buffer_size());

    let (width, height) = (info.width, info.height);
    let pixel_count = (width as usize).checked_mul(height as usize)?;
    let rgba_len = pixel_count.checked_mul(4)?;

    let rgba = match info.color_type {
        png::ColorType::Rgba => buf,
        png::ColorType::Rgb => expand_to_rgba(&buf, 3, |p, out| {
            // `expand_to_rgba` passes chunks of exactly `sample_stride` bytes (3 here).
            if let [r, g, b] = p {
                out.extend_from_slice(&[*r, *g, *b, 0xff]);
            }
        })?,
        png::ColorType::GrayscaleAlpha => expand_to_rgba(&buf, 2, |p, out| {
            if let [gray, alpha] = p {
                out.extend_from_slice(&[*gray, *gray, *gray, *alpha]);
            }
        })?,
        png::ColorType::Grayscale => expand_to_rgba(&buf, 1, |p, out| {
            if let [gray] = p {
                out.extend_from_slice(&[*gray, *gray, *gray, 0xff]);
            }
        })?,
        // Indexed is converted to RGB/RGBA by EXPAND + ALPHA transformations,
        // so we should never see it here. Treat as unsupported if we do.
        png::ColorType::Indexed => return None,
    };

    if rgba.len() != rgba_len {
        return None;
    }
    Some(IconData::Rgba {
        width,
        height,
        data: rgba,
    })
}

/// Expand a packed pixel buffer to RGBA using the caller-provided per-pixel writer.
///
/// `sample_stride` is the number of bytes per pixel in the input (1 for
/// Grayscale, 2 for GrayscaleAlpha, 3 for Rgb). Returns `None` if `buf`
/// length is not a multiple of `sample_stride`.
fn expand_to_rgba<F>(buf: &[u8], sample_stride: usize, mut write: F) -> Option<Vec<u8>>
where
    F: FnMut(&[u8], &mut Vec<u8>),
{
    if sample_stride == 0 || !buf.len().is_multiple_of(sample_stride) {
        return None;
    }
    // sample_stride > 0 verified above; checked_div cannot return None here.
    let pixel_count = buf.len().checked_div(sample_stride)?;
    let mut out = Vec::with_capacity(pixel_count.saturating_mul(4));
    for chunk in buf.chunks_exact(sample_stride) {
        write(chunk, &mut out);
    }
    Some(out)
}

/// Convert an optional RGB foreground color to a replacement string
/// for GTK symbolic icon normalization.
fn fg_to_replacement(fg_color: Option<[u8; 3]>) -> String {
    match fg_color {
        Some([r, g, b]) => format!("#{r:02x}{g:02x}{b:02x}"),
        None => "currentColor".to_string(),
    }
}

/// Parse a vertical SVG sprite sheet into individual frame SVGs.
///
/// Detection: if the viewBox height > width and is an exact multiple,
/// the SVG is treated as a sprite sheet with `height/width` frames.
/// Each frame's SVG is the original with viewBox rewritten to window
/// into the correct vertical slice.
///
/// Returns `None` if the SVG is not a sprite sheet (single-frame,
/// non-multiple dimensions, or parse error).
fn parse_sprite_sheet(svg_bytes: &[u8]) -> Option<Vec<Vec<u8>>> {
    let svg_str = std::str::from_utf8(svg_bytes).ok()?;

    // Find viewBox attribute (handle both double and single quotes)
    let (vb_attr_start, vb_val_start, quote) = svg_str
        .find("viewBox=\"")
        .map(|i| (i, i.saturating_add(9), '"'))
        .or_else(|| {
            svg_str
                .find("viewBox='")
                .map(|i| (i, i.saturating_add(9), '\''))
        })?;

    let tail = svg_str.get(vb_val_start..)?;
    let vb_val_end = tail.find(quote)?.saturating_add(vb_val_start);
    let vb_value = svg_str.get(vb_val_start..vb_val_end)?;

    // Split on whitespace or commas
    let parts: Vec<f64> = vb_value
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();

    let [_, _, width, height] = parts.as_slice() else {
        return None;
    };
    let (width, height) = (*width, *height);
    if height <= width {
        return None; // Single-frame, not a sprite sheet
    }

    let frame_count = (height / width).round() as usize;
    if frame_count < 2 {
        return None;
    }

    // Verify exact division (within floating-point tolerance)
    if (height - width * frame_count as f64).abs() > 0.01 {
        return None;
    }

    // Build the full original viewBox attribute string for replacement
    let original_vb_attr = svg_str.get(vb_attr_start..vb_val_end.saturating_add(1))?; // includes closing quote

    let frames = (0..frame_count)
        .map(|i| {
            let y_offset = width * i as f64;
            let new_vb = format!("viewBox={quote}0 {y_offset} {width} {width}{quote}");
            svg_str.replacen(original_vb_attr, &new_vb, 1).into_bytes()
        })
        .collect();

    Some(frames)
}

/// Load the freedesktop loading spinner from the active icon theme.
///
/// Strategy:
/// 1. Try "process-working" (plain) at size 22 -- may be a sprite sheet -> Frames
/// 2. If found but single-frame (parse_sprite_sheet returns None) -> Transform::Spin
/// 3. Try "process-working-symbolic" at size 22 -- single frame -> Transform::Spin
/// 4. Return None if neither found (caller falls back to bundled Adwaita)
///
/// Each pass finds only a spinner of the theme or of a theme in its
/// `Inherits=` chain, never of `hicolor` unless the theme is `hicolor`
/// (see [`lookup_in_theme`]).
pub(crate) fn load_freedesktop_spinner(theme: Option<&str>) -> Option<AnimatedIcon> {
    let detected;
    let theme: &str = match theme {
        Some(t) => t,
        None => {
            detected = detect_theme();
            &detected
        }
    };
    let chain = cached_theme_chain(theme)?;

    // First pass: plain name (finds sprite sheets in animations/ dirs)
    if let Some(path) = lookup_in_theme("process-working", theme, 22, &chain) {
        let bytes = std::fs::read(&path).ok()?;
        let frame_dur = NonZeroU32::new(FREEDESKTOP_FRAME_DURATION_MS)?;
        let spin_dur = NonZeroU32::new(FREEDESKTOP_SPIN_DURATION_MS)?;
        if let Some(frames) = parse_sprite_sheet(&bytes) {
            let frame_icons: Vec<IconData> = frames
                .into_iter()
                .map(|b| IconData::Svg(Cow::Owned(b)))
                .collect();
            return AnimatedIcon::frames(frame_icons, frame_dur).ok();
        }
        // Not a sprite sheet -- treat as single frame with spin
        return Some(AnimatedIcon::transform(
            IconData::Svg(Cow::Owned(bytes)),
            TransformAnimation::Spin {
                duration_ms: spin_dur,
            },
        ));
    }

    // Second pass: symbolic name (always single frame)
    if let Some(path) = lookup_in_theme("process-working-symbolic", theme, 22, &chain) {
        let bytes = std::fs::read(&path).ok()?;
        let spin_dur = NonZeroU32::new(FREEDESKTOP_SPIN_DURATION_MS)?;
        return Some(AnimatedIcon::transform(
            IconData::Svg(Cow::Owned(bytes)),
            TransformAnimation::Spin {
                duration_ms: spin_dur,
            },
        ));
    }

    None
}

/// The GTK symbolic icon foreground placeholder colors.
///
/// GTK's icon rendering pipeline replaces these at paint time with the
/// widget's CSS `color` property (the text/foreground color). We replace
/// them with the caller-provided foreground color so the SVG is
/// self-contained and renders correctly without connector colorization.
///
/// Measured from `/usr/share/icons/Adwaita/symbolic/`:
/// - `#2e3436`: 483 fill attrs + 8 CSS style fills + 1 stroke (Tango Aluminium 6)
/// - `#2e3434`: 118 files (68 primary, 50 with fill-opacity)
/// - `#222222`: 27 occurrences (primary + dimmed)
/// - `#474747`: 50 emote/legacy icons (monochrome, never mixed with above)
const GTK_FG_COLORS: &[&str] = &["#2e3436", "#2e3434", "#222222", "#474747"];

/// Recolor a GTK-convention symbolic SVG by replacing foreground placeholders.
///
/// GTK symbolic icons use hardcoded dark fill colors (e.g., `#2e3436`)
/// that GTK replaces at render time with the widget's CSS text color.
/// This function does the same: it replaces those placeholders with
/// `replacement`, which should be either a hex color (e.g., `#ffffff`)
/// or `currentColor` as a fallback.
///
/// Handles three placement patterns found in Adwaita:
/// - XML attributes: `fill="#2e3436"`, `stroke="#2e3436"`
/// - CSS style attributes: `style="fill:#2e3436;..."`
///
/// Only foreground placeholder colors are replaced. Semantic colors
/// (success green `#33d17a`, warning orange `#ff7800`, error red
/// `#e01b24`/`#ed333b`) are preserved.
///
/// Returns the original bytes unchanged if the SVG already uses
/// `currentColor` (Breeze-style) or is not valid UTF-8.
fn normalize_gtk_symbolic(svg_bytes: Vec<u8>, replacement: &str) -> Vec<u8> {
    let Ok(svg_str) = std::str::from_utf8(&svg_bytes) else {
        return svg_bytes;
    };

    // Already uses currentColor (Breeze convention) -- no normalization needed
    if svg_str.contains("currentColor") {
        return svg_bytes;
    }

    // Check if any GTK foreground colors are present
    if !GTK_FG_COLORS.iter().any(|c| svg_str.contains(c)) {
        return svg_bytes;
    }

    let mut result = svg_str.to_string();
    for color in GTK_FG_COLORS {
        // XML attributes: fill="..." and stroke="..."
        result = result.replace(
            &format!("fill=\"{color}\""),
            &format!("fill=\"{replacement}\""),
        );
        result = result.replace(
            &format!("stroke=\"{color}\""),
            &format!("stroke=\"{replacement}\""),
        );
        // CSS style attributes: fill:#2e3436 (8 icons use this form)
        result = result.replace(&format!("fill:{color}"), &format!("fill:{replacement}"));
        result = result.replace(&format!("stroke:{color}"), &format!("stroke:{replacement}"));
    }
    result.into_bytes()
}

#[cfg(test)]
#[cfg(feature = "system-icons")]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::{IconRole, IconSet, icon_name};

    #[test]
    #[ignore = "requires a freedesktop icon theme installed (not available on CI)"]
    fn load_icon_returns_some_for_dialog_error() {
        let theme = detect_theme();
        let name = icon_name(IconRole::DialogError, IconSet::Freedesktop).unwrap();
        let result = load_freedesktop_icon_by_name(name, &theme, 24, None);
        assert!(result.is_some(), "DialogError should resolve to an icon");
        match result.unwrap() {
            IconData::Svg(ref cow) => {
                let s = String::from_utf8_lossy(cow);
                assert!(s.contains("<svg"), "Icon data should contain <svg tag");
            }
            other => panic!("Expected SVG data, got {other:?}"),
        }
    }

    #[test]
    fn load_icon_notification_attempts_native_lookup() {
        // Notification is mapped to "notification-active" (KDE convention).
        // Result depends on whether the active theme ships this icon.
        // This test verifies the loader does not panic and does not fall back to Material.
        let theme = detect_theme();
        if let Some(name) = icon_name(IconRole::Notification, IconSet::Freedesktop) {
            let _result = load_freedesktop_icon_by_name(name, &theme, 24, None);
        }
        // No assertion on Some/None -- theme-dependent
    }

    #[test]
    #[ignore = "requires a freedesktop icon theme installed (not available on CI)"]
    fn load_icon_returns_svg_variant() {
        let theme = detect_theme();
        let name = icon_name(IconRole::ActionCopy, IconSet::Freedesktop).unwrap();
        let result = load_freedesktop_icon_by_name(name, &theme, 24, None);
        assert!(result.is_some(), "ActionCopy should resolve to an icon");
        assert!(
            matches!(result.unwrap(), IconData::Svg(_)),
            "Expected Svg variant"
        );
    }

    #[test]
    fn detect_theme_returns_non_empty() {
        let theme = detect_theme();
        assert!(!theme.is_empty(), "Theme name should not be empty");
    }

    #[test]
    fn find_icon_nonexistent_returns_none() {
        let result = find_icon("totally-nonexistent-icon-xyz", "hicolor", 24);
        assert!(result.is_none(), "Nonexistent icon should return None");
    }

    #[test]
    #[ignore = "requires a freedesktop icon theme installed (not available on CI)"]
    fn load_icon_by_name_finds_edit_copy() {
        let theme = detect_theme();
        let result = load_freedesktop_icon_by_name("edit-copy", &theme, 24, None);
        assert!(
            result.is_some(),
            "edit-copy should be found in system theme"
        );
        assert!(matches!(result.unwrap(), IconData::Svg(_)));
    }

    // === Theme directory filter (fixture base dirs) ===

    /// A throwaway tree of icon base dirs, removed on drop:
    ///
    /// - `share/icons/`: `child` (`Inherits=parent,hicolor`), `parent`,
    ///   `hicolor`, `breeze`, `breeze-dark`, `cycle-a` and `cycle-b`
    ///   (each inheriting the other), and a loose `loose.svg`;
    /// - `home/.icons/`: `child/` again, without an `index.theme`;
    /// - `share/pixmaps/`: `pixmap.svg`, outside every base dir.
    struct Fixture {
        root: PathBuf,
        bases: Vec<PathBuf>,
    }

    impl Fixture {
        fn new(tag: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "native-theme-icon-dirs-{}-{tag}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&root);
            let icons = root.join("share/icons");
            let home_icons = root.join("home/.icons");
            let theme = |base: &Path, name: &str, inherits: Option<&str>| {
                let dir = base.join(name);
                std::fs::create_dir_all(dir.join("actions")).unwrap();
                let inherits = inherits.map_or(String::new(), |i| format!("Inherits={i}\n"));
                std::fs::write(
                    dir.join("index.theme"),
                    format!("[Icon Theme]\nName={name}\n{inherits}Directories=actions\n"),
                )
                .unwrap();
            };
            theme(&icons, "child", Some("parent,hicolor"));
            theme(&icons, "parent", None);
            theme(&icons, "hicolor", None);
            theme(&icons, "breeze", None);
            theme(&icons, "breeze-dark", None);
            theme(&icons, "cycle-a", Some("cycle-b"));
            theme(&icons, "cycle-b", Some("cycle-a"));
            std::fs::create_dir_all(home_icons.join("child/actions")).unwrap();
            std::fs::write(icons.join("loose.svg"), "<svg/>").unwrap();
            std::fs::create_dir_all(root.join("share/pixmaps")).unwrap();
            std::fs::write(root.join("share/pixmaps/pixmap.svg"), "<svg/>").unwrap();
            Self {
                bases: vec![icons, home_icons],
                root,
            }
        }

        fn icons(&self, rest: &str) -> PathBuf {
            self.root.join("share/icons").join(rest)
        }

        /// Whether `path` passes the filter for `theme`.
        fn accepts(&self, theme: &str, path: &Path) -> bool {
            theme_chain(theme, &self.bases)
                .is_some_and(|names| is_in_theme_dirs(path, &names, &self.bases))
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn theme_dirs_accept_the_theme_itself() {
        let fx = Fixture::new("own");
        assert!(fx.accepts("child", &fx.icons("child/actions/edit-copy.svg")));
        assert!(fx.accepts(
            "child",
            &fx.root.join("home/.icons/child/actions/edit-copy.svg")
        ));
    }

    #[test]
    fn theme_dirs_accept_a_declared_parent() {
        let fx = Fixture::new("parent");
        assert!(fx.accepts("child", &fx.icons("parent/actions/edit-copy.svg")));
    }

    #[test]
    fn theme_dirs_reject_hicolor_even_when_declared() {
        let fx = Fixture::new("hicolor");
        assert!(!fx.accepts("child", &fx.icons("hicolor/actions/edit-copy.svg")));
        assert!(!fx.accepts("parent", &fx.icons("hicolor/actions/edit-copy.svg")));
    }

    #[test]
    fn theme_dirs_accept_hicolor_when_it_is_the_theme() {
        let fx = Fixture::new("hicolor-chosen");
        assert!(fx.accepts("hicolor", &fx.icons("hicolor/actions/edit-copy.svg")));
    }

    #[test]
    fn theme_dirs_reject_loose_and_pixmaps_files() {
        let fx = Fixture::new("loose");
        assert!(!fx.accepts("child", &fx.icons("loose.svg")));
        assert!(!fx.accepts("child", &fx.root.join("home/.icons/loose.svg")));
        assert!(!fx.accepts("child", &fx.root.join("share/pixmaps/pixmap.svg")));
        assert!(!fx.accepts(
            "child",
            &fx.root.join("share/pixmaps/child/actions/edit-copy.svg")
        ));
    }

    #[test]
    fn theme_dirs_match_whole_components() {
        let fx = Fixture::new("components");
        assert!(!fx.accepts("breeze", &fx.icons("breeze-dark/actions/edit-copy.svg")));
        assert!(!fx.accepts("breeze-dark", &fx.icons("breeze/actions/edit-copy.svg")));
        assert!(fx.accepts("breeze", &fx.icons("breeze/actions/edit-copy.svg")));
    }

    #[test]
    fn theme_dirs_reject_a_path_climbing_out_of_the_theme() {
        let fx = Fixture::new("climb");
        assert!(!fx.accepts("child", &fx.icons("child/../hicolor/actions/edit-copy.svg")));
    }

    #[test]
    fn theme_chain_terminates_on_a_cycle() {
        let fx = Fixture::new("cycle");
        let mut names = theme_chain("cycle-a", &fx.bases).unwrap();
        names.sort();
        assert_eq!(names, ["cycle-a", "cycle-b"]);
    }

    #[test]
    fn theme_chain_of_a_missing_theme_is_none() {
        let fx = Fixture::new("missing");
        assert_eq!(theme_chain("no-such-theme", &fx.bases), None);
        assert_eq!(theme_chain("../share/icons/child", &fx.bases), None);
    }

    /// A loose file in an icon base directory (`/usr/share/icons/cachyos.svg`
    /// on CachyOS) belongs to no theme, so a theme lookup must not return it.
    #[test]
    fn find_icon_rejects_a_loose_base_dir_file() {
        let loose = Path::new("/usr/share/icons/cachyos.svg");
        if !loose.exists() || !Path::new("/usr/share/icons/breeze/index.theme").exists() {
            eprintln!("skipped: needs {} and the breeze theme", loose.display());
            return;
        }
        assert_eq!(find_icon("cachyos", "breeze", 24), None);
    }

    /// An icon breeze lacks is not taken from `hicolor`.
    #[test]
    fn find_icon_never_returns_hicolor_for_another_theme() {
        let apps = Path::new("/usr/share/icons/hicolor/scalable/apps");
        let (Ok(entries), true) = (
            std::fs::read_dir(apps),
            Path::new("/usr/share/icons/breeze/index.theme").exists(),
        ) else {
            eprintln!("skipped: needs {} and the breeze theme", apps.display());
            return;
        };
        let hicolor = Path::new("/usr/share/icons/hicolor");
        let names: Vec<String> = entries
            .filter_map(Result::ok)
            .filter_map(|e| {
                let path = e.path();
                (path.extension()? == "svg").then_some(())?;
                Some(path.file_stem()?.to_str()?.to_string())
            })
            .take(20)
            .collect();
        for name in &names {
            if let Some((path, _)) = find_icon(name, "breeze", 24) {
                assert!(
                    !path.starts_with(hicolor),
                    "{name} for breeze came from hicolor: {}",
                    path.display()
                );
            }
        }
    }

    /// The fix must not lose an icon the theme has.
    #[test]
    fn find_icon_still_finds_breeze_edit_copy() {
        if !Path::new("/usr/share/icons/breeze/index.theme").exists() {
            eprintln!("skipped: the breeze theme is not installed");
            return;
        }
        let (path, _) = find_icon("edit-copy", "breeze", 24).expect("breeze has edit-copy");
        assert!(
            path.starts_with("/usr/share/icons/breeze"),
            "{}",
            path.display()
        );
    }

    #[test]
    fn load_icon_by_name_returns_none_for_nonexistent() {
        let result = load_freedesktop_icon_by_name("zzz-nonexistent-icon", "hicolor", 24, None);
        assert!(result.is_none());
    }

    // === Sprite sheet parser tests ===

    #[test]
    fn test_parse_sprite_sheet_two_frames() {
        // 10x20 viewBox = 2 frames of 10x10
        let svg = br#"<svg viewBox="0 0 10 20" xmlns="http://www.w3.org/2000/svg">
            <rect x="0" y="0" width="10" height="10" fill="red"/>
            <rect x="0" y="10" width="10" height="10" fill="blue"/>
        </svg>"#;

        let frames = parse_sprite_sheet(svg).expect("should parse 2-frame sprite sheet");
        assert_eq!(frames.len(), 2);

        let frame0 = std::str::from_utf8(&frames[0]).unwrap();
        assert!(
            frame0.contains(r#"viewBox="0 0 10 10""#),
            "frame 0 viewBox: {frame0}"
        );

        let frame1 = std::str::from_utf8(&frames[1]).unwrap();
        assert!(
            frame1.contains(r#"viewBox="0 10 10 10""#),
            "frame 1 viewBox: {frame1}"
        );
    }

    #[test]
    fn test_parse_sprite_sheet_fifteen_frames() {
        // 22x330 viewBox = 15 frames (Breeze-like)
        let svg = br#"<svg viewBox="0 0 22 330" xmlns="http://www.w3.org/2000/svg">
            <path d="M0 0"/>
        </svg>"#;

        let frames = parse_sprite_sheet(svg).expect("should parse 15-frame sprite sheet");
        assert_eq!(frames.len(), 15);

        // Verify first and last frame viewBox values
        let first = std::str::from_utf8(&frames[0]).unwrap();
        assert!(first.contains(r#"viewBox="0 0 22 22""#));

        let last = std::str::from_utf8(&frames[14]).unwrap();
        assert!(last.contains(r#"viewBox="0 308 22 22""#));
    }

    #[test]
    fn test_parse_sprite_sheet_single_frame_returns_none() {
        // 22x22 = single frame, not a sprite sheet
        let svg = br#"<svg viewBox="0 0 22 22" xmlns="http://www.w3.org/2000/svg">
            <circle cx="11" cy="11" r="10"/>
        </svg>"#;
        assert!(parse_sprite_sheet(svg).is_none());
    }

    #[test]
    fn test_parse_sprite_sheet_non_multiple_returns_none() {
        // 22x33: height is not an exact multiple of width
        let svg = br#"<svg viewBox="0 0 22 33" xmlns="http://www.w3.org/2000/svg">
            <path d="M0 0"/>
        </svg>"#;
        assert!(parse_sprite_sheet(svg).is_none());
    }

    #[test]
    fn test_parse_sprite_sheet_invalid_svg_returns_none() {
        assert!(parse_sprite_sheet(b"not svg at all").is_none());
    }

    #[test]
    fn test_parse_sprite_sheet_comma_separated_viewbox() {
        // viewBox with commas instead of spaces
        let svg = br#"<svg viewBox="0,0,10,20" xmlns="http://www.w3.org/2000/svg">
            <rect x="0" y="0" width="10" height="10" fill="red"/>
        </svg>"#;

        let frames = parse_sprite_sheet(svg).expect("should parse comma-separated viewBox");
        assert_eq!(frames.len(), 2);

        let frame0 = std::str::from_utf8(&frames[0]).unwrap();
        assert!(frame0.contains(r#"viewBox="0 0 10 10""#));
    }

    #[test]
    fn test_parse_sprite_sheet_preserves_svg_content() {
        let svg = br#"<svg viewBox="0 0 10 20" xmlns="http://www.w3.org/2000/svg">
            <rect x="0" y="0" width="10" height="10" fill="red" id="unique-marker"/>
            <rect x="0" y="10" width="10" height="10" fill="blue"/>
        </svg>"#;

        let frames = parse_sprite_sheet(svg).unwrap();
        // Both frames should preserve the full SVG content
        for frame in &frames {
            let s = std::str::from_utf8(frame).unwrap();
            assert!(
                s.contains("unique-marker"),
                "SVG content should be preserved in all frames"
            );
            assert!(s.contains("<rect"), "rect elements should be preserved");
            assert!(s.contains("xmlns="), "namespace should be preserved");
        }
    }

    #[test]
    fn test_load_freedesktop_spinner_no_panic() {
        // Just verify the function doesn't panic -- result is theme-dependent
        let _result = load_freedesktop_spinner(None);
    }

    // === GTK symbolic icon normalization tests ===

    #[test]
    fn normalize_gtk_symbolic_replaces_2e3436() {
        let svg = br##"<svg><path fill="#2e3436" d="M0 0"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains(r##"fill="#ffffff""##));
        assert!(!s.contains("#2e3436"));
    }

    #[test]
    fn normalize_gtk_symbolic_replaces_2e3434_preserves_opacity() {
        let svg = br##"<svg><path fill="#2e3434" fill-opacity="0.35" d="M0 0"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains(r##"fill="#ffffff""##));
        assert!(s.contains(r#"fill-opacity="0.35""#));
    }

    #[test]
    fn normalize_gtk_symbolic_replaces_222222() {
        let svg = br##"<svg><path fill="#222222" d="M0 0"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains(r##"fill="#ffffff""##));
        assert!(!s.contains("#222222"));
    }

    #[test]
    fn normalize_gtk_symbolic_replaces_474747() {
        let svg = br##"<svg><path fill="#474747" d="M0 0"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains(r##"fill="#ffffff""##));
        assert!(!s.contains("#474747"));
    }

    #[test]
    fn normalize_gtk_symbolic_replaces_stroke() {
        let svg = br##"<svg><path stroke="#2e3436" fill="none" d="M1 1l14 14"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains(r##"stroke="#ffffff""##));
        assert!(!s.contains("#2e3436"));
    }

    #[test]
    fn normalize_gtk_symbolic_replaces_css_style_fill() {
        let svg = br##"<svg><path style="fill:#2e3436;fill-opacity:1" d="M0 0"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains("fill:#ffffff"));
        assert!(!s.contains("#2e3436"));
    }

    #[test]
    fn normalize_gtk_symbolic_preserves_semantic_colors() {
        let svg = br##"<svg><path fill="#2e3436"/><path fill="#ff7800"/><path fill="#33d17a"/><path fill="#e01b24"/></svg>"##.to_vec();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        let s = std::str::from_utf8(&result).unwrap();
        assert!(s.contains("#ffffff"));
        assert!(s.contains("#ff7800"), "warning color must be preserved");
        assert!(s.contains("#33d17a"), "success color must be preserved");
        assert!(s.contains("#e01b24"), "error color must be preserved");
    }

    #[test]
    fn normalize_gtk_symbolic_skips_currentcolor_svgs() {
        let svg = br##"<svg><defs><style>.ColorScheme-Text{color:#232629}</style></defs><path fill="currentColor"/></svg>"##.to_vec();
        let original = svg.clone();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        assert_eq!(
            result, original,
            "Breeze-style SVGs should pass through unchanged"
        );
    }

    #[test]
    fn normalize_gtk_symbolic_skips_non_gtk_svgs() {
        let svg = br#"<svg><path fill="red"/></svg>"#.to_vec();
        let original = svg.clone();
        let result = normalize_gtk_symbolic(svg, "#ffffff");
        assert_eq!(
            result, original,
            "non-GTK SVGs should pass through unchanged"
        );
    }

    // === PNG decode path (for raster-only legacy themes like AdwaitaLegacy) ===

    /// Encode a minimal in-memory PNG with a known pixel pattern for
    /// round-trip tests. Returns `None` on encode error.
    fn encode_test_png(
        width: u32,
        height: u32,
        color: png::ColorType,
        pixels: &[u8],
    ) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        let mut encoder = png::Encoder::new(&mut out, width, height);
        encoder.set_color(color);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().ok()?;
        writer.write_image_data(pixels).ok()?;
        drop(writer);
        Some(out)
    }

    #[test]
    fn decode_png_to_rgba_roundtrips_2x2_rgba() {
        // Known pixel pattern: 4 distinct RGBA pixels
        let pixels: [u8; 16] = [
            0xff, 0x00, 0x00, 0xff, // red, opaque
            0x00, 0xff, 0x00, 0x80, // green, half-alpha
            0x00, 0x00, 0xff, 0xff, // blue, opaque
            0x11, 0x22, 0x33, 0x44, // arbitrary
        ];
        let bytes = encode_test_png(2, 2, png::ColorType::Rgba, &pixels).expect("test PNG encode");

        let result = decode_png_to_rgba(&bytes);
        match result {
            Some(IconData::Rgba {
                width,
                height,
                data,
            }) => {
                assert_eq!(width, 2);
                assert_eq!(height, 2);
                assert_eq!(data, pixels.to_vec());
            }
            other => panic!("expected Rgba, got {other:?}"),
        }
    }

    #[test]
    fn decode_png_to_rgba_expands_rgb_to_rgba() {
        // RGB (no alpha) input must come out as RGBA with alpha=0xff.
        let pixels: [u8; 6] = [
            0xab, 0xcd, 0xef, // pixel 0
            0x11, 0x22, 0x33, // pixel 1
        ];
        let bytes = encode_test_png(2, 1, png::ColorType::Rgb, &pixels).expect("test PNG encode");

        match decode_png_to_rgba(&bytes) {
            Some(IconData::Rgba {
                width,
                height,
                data,
            }) => {
                assert_eq!((width, height), (2, 1));
                assert_eq!(data, vec![0xab, 0xcd, 0xef, 0xff, 0x11, 0x22, 0x33, 0xff]);
            }
            other => panic!("expected Rgba, got {other:?}"),
        }
    }

    #[test]
    fn decode_png_to_rgba_expands_grayscale_to_rgba() {
        let pixels: [u8; 2] = [0x40, 0xc0];
        let bytes =
            encode_test_png(2, 1, png::ColorType::Grayscale, &pixels).expect("test PNG encode");

        match decode_png_to_rgba(&bytes) {
            Some(IconData::Rgba { data, .. }) => {
                assert_eq!(data, vec![0x40, 0x40, 0x40, 0xff, 0xc0, 0xc0, 0xc0, 0xff]);
            }
            other => panic!("expected Rgba, got {other:?}"),
        }
    }

    #[test]
    fn decode_png_to_rgba_rejects_non_png_bytes() {
        assert!(decode_png_to_rgba(b"not a png at all").is_none());
        assert!(decode_png_to_rgba(&[]).is_none());
    }

    #[test]
    #[ignore = "requires AdwaitaLegacy icon theme installed (GNOME legacy raster theme)"]
    fn load_icon_from_png_only_theme_returns_rgba() {
        // AdwaitaLegacy is a PNG-only legacy theme. The loader must decode
        // PNG to Rgba, not wrap raw PNG bytes inside IconData::Svg.
        let result = load_freedesktop_icon_by_name("edit-copy", "AdwaitaLegacy", 24, None);
        let data = result.expect("edit-copy exists in AdwaitaLegacy 24x24/legacy");
        match data {
            IconData::Rgba {
                width,
                height,
                data,
            } => {
                assert!(width > 0 && height > 0);
                assert_eq!(
                    data.len(),
                    (width as usize) * (height as usize) * 4,
                    "RGBA byte count must match width*height*4"
                );
            }
            IconData::Svg(bytes) => {
                let head: Vec<u8> = bytes.iter().take(8).copied().collect();
                panic!(
                    "expected Rgba, got Svg with {} bytes starting {:x?} \
                     (PNG magic is 89 50 4e 47)",
                    bytes.len(),
                    head
                );
            }
        }
    }
}
