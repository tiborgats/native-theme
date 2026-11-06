//! The bundled SVGs are SVG documents, and the platform sets load nothing on
//! another OS.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use native_theme::icons::IconRole;

// Bundled icon sets return `IconData::Svg` with `Cow::Borrowed` to static
// bytes, so `cow.as_ref()` is a zero-copy view back to `&'static [u8]`.

/// The first characters of an SVG, for a failure message.
#[cfg(any(feature = "material-icons", feature = "lucide-icons"))]
fn head(content: &str) -> String {
    content.chars().take(40).collect()
}

#[test]
#[cfg(feature = "material-icons")]
fn bundled_material_svg_starts_with_svg_tag() {
    use native_theme::icons::{IconData, MaterialLoader};
    for role in IconRole::ALL.iter().copied() {
        if let Some(data) = MaterialLoader::new(role).load() {
            let IconData::Svg(cow) = data else { continue };
            let svg_bytes: &[u8] = cow.as_ref();
            let content = std::str::from_utf8(svg_bytes)
                .unwrap_or_else(|_| panic!("Material SVG for {role:?} is not valid UTF-8"));
            assert!(
                content.starts_with("<svg") || content.starts_with("<?xml"),
                "Material SVG for {role:?} should start with <svg or <?xml, got: {}...",
                head(content)
            );
        }
    }
}

#[test]
#[cfg(feature = "lucide-icons")]
fn bundled_lucide_svg_starts_with_svg_tag() {
    use native_theme::icons::{IconData, LucideLoader};
    for role in IconRole::ALL.iter().copied() {
        if let Some(data) = LucideLoader::new(role).load() {
            let IconData::Svg(cow) = data else { continue };
            let svg_bytes: &[u8] = cow.as_ref();
            let content = std::str::from_utf8(svg_bytes)
                .unwrap_or_else(|_| panic!("Lucide SVG for {role:?} is not valid UTF-8"));
            assert!(
                content.starts_with("<svg") || content.starts_with("<?xml"),
                "Lucide SVG for {role:?} should start with <svg or <?xml, got: {}...",
                head(content)
            );
        }
    }
}

// The platform-native sets (SfSymbols, SegoeFluent) return None on the wrong
// OS. Freedesktop is not asserted None here: on Linux with `system-icons`
// FreedesktopLoader loads icons from the filesystem.

#[test]
#[cfg(not(target_os = "macos"))]
fn sf_symbols_returns_none_on_non_macos() {
    use native_theme::icons::SfSymbolsLoader;
    for role in IconRole::ALL.iter().copied() {
        assert!(
            SfSymbolsLoader::new(role).load().is_none(),
            "SfSymbols should not load on non-macOS for {role:?}"
        );
    }
}

#[test]
#[cfg(not(target_os = "windows"))]
fn segoe_fluent_returns_none_on_non_windows() {
    use native_theme::icons::SegoeFluentLoader;
    for role in IconRole::ALL.iter().copied() {
        assert!(
            SegoeFluentLoader::new(role).load().is_none(),
            "SegoeFluent should not load on non-Windows for {role:?}"
        );
    }
}
