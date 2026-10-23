//! The status bar's environment text
//! (`docs/archive/todo_v0.5.9_showcase-app-spec.md` §2.7), one item each:
//! the desktop, the preset and colour mode, the installed theme's
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
/// place ([`DrawnFamily::label`] of [`DrawnFamily::from`] `drawn`).
#[must_use]
pub fn family_label(stated: &str, drawn: Option<&str>) -> String {
    DrawnFamily::from(drawn).label(stated)
}

/// What a toolkit draws text of a theme's stated family in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawnFamily<'a> {
    /// The stated family, under whatever name the font database files it,
    /// or the toolkit does not say.
    Stated,
    /// The family named: the platform's substitute for a family the system
    /// lacks, or gpui's `.SystemUIFont` alias for the system UI font.
    Named(&'a str),
    /// The toolkit's generic family `kind` (`sans-serif`, `monospace`),
    /// which resolves to `family` where the font database names one for it,
    /// drawn because the toolkit draws no face of the family it asked for.
    Generic {
        /// The toolkit, as the label names it (`iced`).
        toolkit: &'a str,
        /// The generic family.
        kind: &'a str,
        /// The family the generic one resolves to, where the database names
        /// one.
        family: Option<&'a str>,
        /// `None` where the system has no face of the stated family;
        /// otherwise the weights of its faces at the normal width, the only
        /// width the toolkit's text is drawn in, none of which the toolkit
        /// can ask for (empty where it has no face at the normal width, so
        /// none the toolkit can match at all).
        held_weights: Option<&'a [u16]>,
    },
    /// `family`, the first family of the text library's common fallback
    /// list the font database holds, drawn because the toolkit draws no face
    /// of the family it asked for and the database holds no family its
    /// generic family names (cosmic-text 0.15 `font/fallback/mod.rs`,
    /// `common_fallback`).
    Fallback {
        /// The toolkit, as the label names it (`iced`).
        toolkit: &'a str,
        /// The text library, as the label names it (`cosmic-text`).
        library: &'a str,
        /// The family drawn.
        family: &'a str,
        /// As [`DrawnFamily::Generic`]'s.
        held_weights: Option<&'a [u16]>,
    },
    /// The face of another of the theme's fonts, `font`, whose family `family` labels: the
    /// toolkit draws every font of a kind in one face (egui: every font but the monospace one
    /// in `defaults.font`'s), so a font whose stated family is another is drawn in that face.
    OtherFont {
        /// The toolkit, as the label names it (`egui`).
        toolkit: &'a str,
        /// The font whose face is drawn, as the label names it before "face" (`the light
        /// variant's defaults.font`).
        font: &'a str,
        /// That font's family, as its own label names what is drawn
        /// ([`DrawnFamily::label`]).
        family: &'a str,
    },
    /// The toolkit's generic family `kind` in the toolkit's own faces, `family` the first
    /// where the toolkit's font list names one, drawn because the build loads no system font,
    /// so the system is not asked for the stated family (the egui showcase without feature
    /// `system-fonts`).
    Unloaded {
        /// The toolkit, as the label names it (`egui`).
        toolkit: &'a str,
        /// The generic family (`proportional`, `monospace`).
        kind: &'a str,
        /// The toolkit's first face of the generic family, where its font list names one.
        family: Option<&'a str>,
    },
    /// `family`, the first family of the toolkit's own fallback stack its text system finds,
    /// drawn because the text system finds no face of the family it was asked for: the stated
    /// one where `asked` is `None`, else `asked`, the platform's substitute for it or gpui's
    /// `.SystemUIFont` alias for the system UI font (gpui-pre 0.3.9 `src/text_system.rs`:
    /// `TextSystem::resolve_font`, lines 370-388, over the stack of lines 255-266).
    FallbackStack {
        /// The toolkit, as the label names it (`gpui`).
        toolkit: &'a str,
        /// The family the toolkit was asked for in the stated one's place, where it was.
        asked: Option<&'a str>,
        /// The family drawn.
        family: &'a str,
    },
}

impl<'a> From<Option<&'a str>> for DrawnFamily<'a> {
    /// `None`, where the toolkit does not say, is [`DrawnFamily::Stated`];
    /// a name is [`DrawnFamily::Named`].
    fn from(drawn: Option<&'a str>) -> Self {
        drawn.map_or(Self::Stated, Self::Named)
    }
}

impl DrawnFamily<'_> {
    /// `stated` as a label that says what is drawn: the stated name alone
    /// where it is drawn; the system UI font's note for `.SystemUIFont`; the
    /// platform's substitute named otherwise; and, for a generic family or
    /// the text library's fallback, whether the system has the stated
    /// family, at which weights where its faces lie only at weights the
    /// toolkit cannot ask for, and the generic family drawn with what it
    /// resolves to, or the fallback family drawn; for a build that loads no
    /// system font, that it does and the generic family drawn in the
    /// toolkit's own face; for a font the toolkit draws in another font's
    /// face, that font and its family's label; for the toolkit's fallback
    /// stack, which family it found no face of and the stack's family drawn.
    #[must_use]
    pub fn label(&self, stated: &str) -> String {
        match *self {
            Self::Stated => stated.to_string(),
            Self::Named(drawn) if drawn == stated => stated.to_string(),
            Self::Named(".SystemUIFont") => {
                format!("{stated} (drawn as the system UI font, .SystemUIFont)")
            }
            Self::Named(drawn) => format!(
                "{stated} (not on this system; drawn in the platform's substitute, {drawn})"
            ),
            Self::Generic {
                toolkit,
                kind,
                family,
                held_weights,
            } => {
                let held = held(toolkit, held_weights);
                match family {
                    Some(family) => {
                        format!("{stated} ({held}; drawn in {toolkit}'s generic {kind}, {family})")
                    }
                    None => format!("{stated} ({held}; drawn in {toolkit}'s generic {kind})"),
                }
            }
            Self::Fallback {
                toolkit,
                library,
                family,
                held_weights,
            } => {
                let held = held(toolkit, held_weights);
                format!("{stated} ({held}; drawn in {library}'s common fallback, {family})")
            }
            Self::OtherFont {
                toolkit,
                font,
                family,
            } => format!("{stated} ({toolkit} draws it in {font} face: {family})"),
            Self::Unloaded {
                toolkit,
                kind,
                family,
            } => match family {
                Some(family) => format!(
                    "{stated} (this build loads no system font; drawn in {toolkit}'s generic \
                     {kind}, {family})"
                ),
                None => format!(
                    "{stated} (this build loads no system font; drawn in {toolkit}'s generic \
                     {kind})"
                ),
            },
            Self::FallbackStack {
                toolkit,
                asked,
                family,
            } => {
                let missing = match asked {
                    None => format!("{toolkit} finds no face of it"),
                    Some(".SystemUIFont") => {
                        format!("{toolkit} finds no face of the system UI font, .SystemUIFont")
                    }
                    Some(asked) => format!(
                        "not on this system; {toolkit} finds no face of the platform's \
                         substitute, {asked}"
                    ),
                };
                format!(
                    "{stated} ({missing}; drawn in the first family of {toolkit}'s fallback \
                     stack it finds, {family})"
                )
            }
        }
    }
}

/// Whether the system has the stated family, as a label says it
/// (`held_weights` as [`DrawnFamily::Generic`]'s).
fn held(toolkit: &str, held_weights: Option<&[u16]>) -> String {
    match held_weights {
        None => "not on this system".to_string(),
        Some([]) => format!("on this system, in no face {toolkit} can draw"),
        Some(weights) => {
            let weights: Vec<String> = weights.iter().map(u16::to_string).collect();
            format!(
                "on this system, its normal-width faces only at weights {toolkit} cannot \
                 ask for ({})",
                weights.join(", ")
            )
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
    environment_drawn(
        preset_label,
        mode,
        font,
        DrawnFamily::from(drawn_family),
        prefs,
    )
}

/// [`environment`], the font named by [`DrawnFamily::label`] of `drawn`.
#[must_use]
pub fn environment_drawn(
    preset_label: &str,
    mode: &str,
    font: &ResolvedFontSpec,
    drawn: DrawnFamily<'_>,
    prefs: Option<&AccessibilityPreferences>,
) -> Vec<String> {
    let mut items = vec![
        desktop(),
        format!("{preset_label} {mode}"),
        format!("{} {}", drawn.label(&font.family), defined_size(font)),
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

    /// A generic family says whether the system has the stated one: not at
    /// all, only at weights the toolkit cannot ask for (listed), or in no
    /// face it can match; and what the generic family resolves to, where
    /// the font database names it, or the text library's fallback drawn; a
    /// build that loads no system font says so; a font drawn in another
    /// font's face names that font and its family's label.
    #[test]
    fn a_generic_family_says_why_it_is_drawn() {
        let generic = |family, held_weights| DrawnFamily::Generic {
            toolkit: "iced",
            kind: "sans-serif",
            family,
            held_weights,
        };
        assert_eq!(
            generic(Some("Open Sans"), None).label("Segoe UI"),
            "Segoe UI (not on this system; drawn in iced's generic sans-serif, Open Sans)"
        );
        assert_eq!(
            generic(None, None).label("Segoe UI"),
            "Segoe UI (not on this system; drawn in iced's generic sans-serif)"
        );
        assert_eq!(
            generic(Some("Open Sans"), Some(&[350, 650])).label("Theme Sans"),
            "Theme Sans (on this system, its normal-width faces only at weights iced cannot \
             ask for (350, 650); drawn in iced's generic sans-serif, Open Sans)"
        );
        assert_eq!(
            generic(Some("Open Sans"), Some(&[])).label("Theme Sans"),
            "Theme Sans (on this system, in no face iced can draw; drawn in iced's generic \
             sans-serif, Open Sans)"
        );
        let fallback = |held_weights| DrawnFamily::Fallback {
            toolkit: "iced",
            library: "cosmic-text",
            family: "Noto Sans",
            held_weights,
        };
        assert_eq!(
            fallback(None).label("Segoe UI"),
            "Segoe UI (not on this system; drawn in cosmic-text's common fallback, Noto Sans)"
        );
        assert_eq!(
            fallback(Some(&[350])).label("Theme Sans"),
            "Theme Sans (on this system, its normal-width faces only at weights iced cannot \
             ask for (350); drawn in cosmic-text's common fallback, Noto Sans)"
        );
        assert_eq!(DrawnFamily::Stated.label("Noto Sans"), "Noto Sans");
        let unloaded = |family| DrawnFamily::Unloaded {
            toolkit: "egui",
            kind: "proportional",
            family,
        };
        assert_eq!(
            unloaded(Some("Ubuntu-Light")).label("Noto Sans"),
            "Noto Sans (this build loads no system font; drawn in egui's generic proportional, \
             Ubuntu-Light)"
        );
        assert_eq!(
            unloaded(None).label("Noto Sans"),
            "Noto Sans (this build loads no system font; drawn in egui's generic proportional)"
        );
        let body = family_label("Segoe UI", Some("Adwaita Sans"));
        assert_eq!(
            DrawnFamily::OtherFont {
                toolkit: "egui",
                font: "the light variant's defaults.font",
                family: &body,
            }
            .label("Noto Serif"),
            "Noto Serif (egui draws it in the light variant's defaults.font face: Segoe UI (not \
             on this system; drawn in the platform's substitute, Adwaita Sans))"
        );
        assert_eq!(
            DrawnFamily::from(Some("Adwaita Sans")).label("Segoe UI"),
            family_label("Segoe UI", Some("Adwaita Sans"))
        );
    }

    /// The toolkit's fallback stack says which family the toolkit found no face of: the stated
    /// one, the platform's substitute for it, or the system UI font's alias; and names the
    /// family drawn.
    #[test]
    fn a_fallback_stack_says_what_was_not_found() {
        let stack = |asked| DrawnFamily::FallbackStack {
            toolkit: "gpui",
            asked,
            family: "DejaVu Sans",
        };
        assert_eq!(
            stack(None).label("Segoe UI"),
            "Segoe UI (gpui finds no face of it; drawn in the first family of gpui's fallback \
             stack it finds, DejaVu Sans)"
        );
        assert_eq!(
            stack(Some("Noto Sans")).label("NoSuchFamily-T10b"),
            "NoSuchFamily-T10b (not on this system; gpui finds no face of the platform's \
             substitute, Noto Sans; drawn in the first family of gpui's fallback stack it finds, \
             DejaVu Sans)"
        );
        assert_eq!(
            stack(Some(".SystemUIFont")).label("SF Pro"),
            "SF Pro (gpui finds no face of the system UI font, .SystemUIFont; drawn in the first \
             family of gpui's fallback stack it finds, DejaVu Sans)"
        );
    }

    /// The environment names the font by the label of what is drawn.
    #[test]
    fn the_environment_names_a_generic_family() {
        let font = font();
        let drawn = DrawnFamily::Generic {
            toolkit: "iced",
            kind: "sans-serif",
            family: None,
            held_weights: None,
        };
        let items = environment_drawn("kde-breeze", "light", &font, drawn, None);
        assert_eq!(
            items.get(2),
            Some(&format!(
                "{} {}",
                drawn.label(&font.family),
                defined_size(&font)
            ))
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
