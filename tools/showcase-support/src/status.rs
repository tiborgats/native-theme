//! The status bar's environment text
//! (`docs/archive/todo_v0.5.9_showcase-app-spec.md` §2.7), one item each: the desktop, the preset and colour mode, the installed theme's
//! `defaults.font` with its stated size, and, once a native theme is
//! installed, its text-scale factor and each accessibility preference that
//! is set.

use native_theme::AccessibilityPreferences;
use native_theme::theme::{FontSize, ResolvedFontSpec};

/// The desktop `native_theme::detect` recognises in `XDG_CURRENT_DESKTOP`,
/// or `Unknown` where it recognises none. `SystemTheme::from_system` then
/// asks the portal, then `kdeglobals` (native-theme `pipeline.rs`,
/// `select_reader`), and the preset names what it settled on.
#[cfg(target_os = "linux")]
#[must_use]
pub fn desktop() -> String {
    format!("{:?}", native_theme::detect::detect_linux_desktop())
}

/// The operating system: `native_theme::detect` names a desktop on Linux
/// only.
#[cfg(not(target_os = "linux"))]
#[must_use]
pub fn desktop() -> String {
    std::env::consts::OS.to_string()
}

/// A font size in the unit its source stated, never converted: `10.5pt`,
/// `14px`, or `(size not stated)`.
///
/// A platform that reports points (KDE, GNOME, Windows) and a preset written
/// in pixels both resolve to logical pixels, and 10.5pt at 96 DPI is exactly
/// 14px, so dividing the resolved size by the DPI would print "10.5pt" for a
/// preset whose author wrote `size_px = 14`. `ResolvedFontSpec::defined_size`
/// carries what the source said; validation records a missing size rather
/// than inventing one, so none is presented as a definition.
#[must_use]
pub fn defined_size(font: &ResolvedFontSpec) -> String {
    match font.defined_size {
        Some(FontSize::Pt(v)) => format!("{v}pt"),
        Some(FontSize::Px(v)) => format!("{v}px"),
        None => "(size not stated)".to_string(),
    }
}

/// The family the theme states, `stated`, as a label that says what the
/// toolkit draws, `drawn` (`None` where the toolkit does not say): the
/// stated name alone where it is drawn; the system UI font's note where gpui
/// draws `.SystemUIFont`; else the substitute the platform draws in its
/// place.
#[must_use]
pub fn family_label(stated: &str, drawn: Option<&str>) -> String {
    match drawn {
        None => stated.to_string(),
        Some(drawn) if drawn == stated => stated.to_string(),
        Some(".SystemUIFont") => {
            format!("{stated} (drawn as the system UI font, .SystemUIFont)")
        }
        Some(drawn) => {
            format!("{stated} (not on this system; drawn in the platform's substitute, {drawn})")
        }
    }
}

/// The status bar's items: the desktop ([`desktop`]), `<preset> <mode>`
/// (`preset_label` as the preset picker names it, `mode` `light` or `dark`),
/// the font as [`family_label`] of its family and `drawn_family` and its
/// [`defined_size`], then, where `prefs` is given (a native theme is
/// installed), `text ×<factor>` and each preference that is set, by its
/// field name.
#[must_use]
pub fn environment(
    preset_label: &str,
    mode: &str,
    font: &ResolvedFontSpec,
    drawn_family: Option<&str>,
    prefs: Option<&AccessibilityPreferences>,
) -> Vec<String> {
    let mut items = vec![
        desktop(),
        format!("{preset_label} {mode}"),
        format!(
            "{} {}",
            family_label(&font.family, drawn_family),
            defined_size(font)
        ),
    ];
    if let Some(prefs) = prefs {
        items.push(format!("text ×{}", prefs.text_scaling_factor));
        let flags = [
            ("reduce_motion", prefs.reduce_motion),
            ("high_contrast", prefs.high_contrast),
            ("reduce_transparency", prefs.reduce_transparency),
        ];
        items.extend(
            flags
                .into_iter()
                .filter(|(_, set)| *set)
                .map(|(name, _)| name.to_string()),
        );
    }
    items
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    use native_theme::ResolutionContext;
    use native_theme::theme::{ColorMode, Theme};

    /// kde-breeze light's `defaults.font`, resolved for tests.
    fn font() -> ResolvedFontSpec {
        Theme::preset("kde-breeze")
            .expect("the preset loads")
            .resolve_with(ColorMode::Light, &ResolutionContext::for_tests())
            .expect("the preset resolves")
            .variant
            .defaults
            .font
    }

    /// The stated name alone where it is drawn or nothing is said; gpui's
    /// system UI font note; the substitute named otherwise.
    #[test]
    fn a_family_label_says_what_is_drawn() {
        assert_eq!(family_label("Noto Sans", None), "Noto Sans");
        assert_eq!(family_label("Noto Sans", Some("Noto Sans")), "Noto Sans");
        assert_eq!(
            family_label("SF Pro", Some(".SystemUIFont")),
            "SF Pro (drawn as the system UI font, .SystemUIFont)"
        );
        assert_eq!(
            family_label("Segoe UI", Some("Adwaita Sans")),
            "Segoe UI (not on this system; drawn in the platform's substitute, Adwaita Sans)"
        );
    }

    /// A size in the unit its source stated, or that none is stated.
    #[test]
    fn a_size_keeps_its_stated_unit() {
        let mut font = font();
        font.defined_size = Some(FontSize::Pt(10.5));
        assert_eq!(defined_size(&font), "10.5pt");
        font.defined_size = Some(FontSize::Px(14.0));
        assert_eq!(defined_size(&font), "14px");
        font.defined_size = None;
        assert_eq!(defined_size(&font), "(size not stated)");
    }

    /// The desktop, the preset and mode, the font; then, with preferences,
    /// the text factor and each set preference by its field name.
    #[test]
    fn the_environment_names_its_items_in_order() {
        let font = font();
        let font_item = format!("{} {}", font.family, defined_size(&font));
        assert_eq!(
            environment("default (kde-breeze)", "light", &font, None, None),
            [
                desktop(),
                "default (kde-breeze) light".to_string(),
                font_item.clone(),
            ]
        );
        let mut prefs = AccessibilityPreferences::default();
        prefs.text_scaling_factor = 1.25;
        prefs.reduce_motion = true;
        prefs.high_contrast = false;
        prefs.reduce_transparency = true;
        assert_eq!(
            environment("adwaita", "dark", &font, Some("Cantarell"), Some(&prefs)),
            [
                desktop(),
                "adwaita dark".to_string(),
                format!(
                    "{} {}",
                    family_label(&font.family, Some("Cantarell")),
                    defined_size(&font)
                ),
                "text ×1.25".to_string(),
                "reduce_motion".to_string(),
                "reduce_transparency".to_string(),
            ]
        );
        let none_set = AccessibilityPreferences::default();
        let items = environment("adwaita", "dark", &font, None, Some(&none_set));
        assert_eq!(
            items.last(),
            Some(&format!("text ×{}", none_set.text_scaling_factor))
        );
    }
}
