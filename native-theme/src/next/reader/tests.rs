use super::*;

const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);

fn accented(accent: Color) -> OsColors {
    OsColors {
        accent: Some(accent),
        ..OsColors::default()
    }
}

#[test]
fn colors_for_finds_the_matching_entry() {
    let one = SystemOverrides {
        color_schemes: vec![((Scheme::Dark, Contrast::Normal), accented(RED))],
        ..SystemOverrides::default()
    };
    assert_eq!(
        one.colors_for(Scheme::Dark, Contrast::Normal),
        Some(&accented(RED))
    );
    // No entry for the pair: the scheme or the contrast differs.
    assert_eq!(one.colors_for(Scheme::Light, Contrast::Normal), None);
    assert_eq!(one.colors_for(Scheme::Dark, Contrast::High), None);
    assert_eq!(
        SystemOverrides::default().colors_for(Scheme::Light, Contrast::Normal),
        None
    );
    // Two entries: each pair finds its own.
    let two = SystemOverrides {
        color_schemes: vec![
            ((Scheme::Light, Contrast::Normal), accented(RED)),
            ((Scheme::Light, Contrast::Increased), accented(GREEN)),
        ],
        ..SystemOverrides::default()
    };
    assert_eq!(
        two.colors_for(Scheme::Light, Contrast::Normal),
        Some(&accented(RED))
    );
    assert_eq!(
        two.colors_for(Scheme::Light, Contrast::Increased),
        Some(&accented(GREEN))
    );
    assert_eq!(two.colors_for(Scheme::Dark, Contrast::Normal), None);
    // Two entries for one pair: the first.
    let doubled = SystemOverrides {
        color_schemes: vec![
            ((Scheme::Dark, Contrast::Normal), accented(RED)),
            ((Scheme::Dark, Contrast::Normal), accented(GREEN)),
        ],
        ..SystemOverrides::default()
    };
    assert_eq!(
        doubled.colors_for(Scheme::Dark, Contrast::Normal),
        Some(&accented(RED))
    );
}

#[test]
fn defaults_state_nothing() {
    let o = SystemOverrides::default();
    assert!(o.color_schemes.is_empty());
    assert_eq!(o.scheme, None);
    assert_eq!(o.ui_font, None);
    assert_eq!(o.icon_sizes, None);
    assert_eq!(o.status_bar_font, None);
    assert_eq!(o.tooltip_font, None);
    assert_eq!(o.scrollbar_width_px, None);
    assert_eq!(o.scrollbar_min_thumb_length_px, None);
    assert_eq!(o.border_width_px, None);
    assert!(o.diagnostics.is_empty());
    assert_eq!(
        OsColors::default(),
        OsColors::new(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None
        )
    );
    assert_eq!(
        OsSurfaceOverrides::default(),
        OsSurfaceOverrides::new(
            None, None, None, None, None, None, None, None, None, None, None, None
        )
    );
    assert_eq!(
        OsSelectionColors::default(),
        OsSelectionColors::new(None, None, None, None)
    );
    assert_eq!(OsFont::default(), OsFont::new(None, None, None, None, None));
    let effects = KdeStateEffects::default();
    assert!(!effects.enabled);
    assert_eq!(effects.intensity, KdeIntensityEffect::None);
    assert_eq!(effects.color, KdeColorEffect::None);
    assert_eq!(effects.contrast, KdeContrastEffect::None);
}

#[test]
fn constructors_take_every_field() {
    let surface = SurfaceColors::new(Some(RED), Some(GREEN), None, None, None);
    let surfaces = OsSurfaceOverrides::new(
        Some(surface),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(surface),
        None,
        Some(surface),
    );
    assert_eq!(surfaces.window, Some(surface));
    assert_eq!(surfaces.title_bar, Some(surface));
    assert_eq!(surfaces.title_bar_inactive, None);
    assert_eq!(surfaces.button, Some(surface));

    let selection = OsSelectionColors::new(Some(RED), None, None, Some(GREEN));
    assert_eq!(selection.background, Some(RED));
    assert_eq!(selection.foreground, None);
    assert_eq!(selection.unfocused_foreground, Some(GREEN));

    let state = KdeStateEffects::new(
        true,
        KdeIntensityEffect::Shade,
        0.25,
        KdeColorEffect::Desaturate,
        0.5,
        GREEN,
        KdeContrastEffect::Fade,
        0.75,
    );
    assert!(state.enabled);
    assert_eq!(state.intensity, KdeIntensityEffect::Shade);
    assert_eq!(state.intensity_amount, 0.25);
    assert_eq!(state.color, KdeColorEffect::Desaturate);
    assert_eq!(state.color_amount, 0.5);
    assert_eq!(state.color_color, GREEN);
    assert_eq!(state.contrast, KdeContrastEffect::Fade);
    assert_eq!(state.contrast_amount, 0.75);

    let effects = KdeColorEffects::new(Some(state), None, false);
    assert_eq!(effects.inactive, Some(state));
    assert_eq!(effects.disabled, None);
    assert!(!effects.change_selection_color);

    let colors = OsColors::new(
        Some(surfaces.clone()),
        Some(selection),
        None,
        Some(RED),
        Some(GREEN),
        None,
        None,
        Some([Some(RED), None, None, Some(GREEN)]),
        Some(RED),
        None,
        Some(GREEN),
        None,
        Some(RED),
        Some(GREEN),
        Some(RED),
        None,
        Some(0.2),
        Some(7),
        Some(effects),
    );
    assert_eq!(colors.surfaces, Some(surfaces));
    assert_eq!(colors.selection, Some(selection));
    assert_eq!(colors.text_selection, None);
    assert_eq!(colors.accent, Some(RED));
    assert_eq!(colors.focus_ring, Some(GREEN));
    assert_eq!(colors.status, Some([Some(RED), None, None, Some(GREEN)]));
    assert_eq!(colors.disabled_text, Some(RED));
    assert_eq!(colors.placeholder, None);
    assert_eq!(colors.caret, Some(GREEN));
    assert_eq!(colors.list_header_foreground, None);
    assert_eq!(colors.grid, Some(RED));
    assert_eq!(colors.alternate_row_background, Some(GREEN));
    assert_eq!(colors.kde_decoration_focus, Some(RED));
    assert_eq!(colors.kde_decoration_hover, None);
    assert_eq!(colors.kde_frame_contrast, Some(0.2));
    assert_eq!(colors.kde_contrast, Some(7));
    assert_eq!(colors.kde_effects, Some(effects));

    let font = OsFont::new(
        Some(Arc::from("Noto Sans")),
        Some(FontSize::Pt(10.0)),
        Some(400),
        Some(FontStyle::Normal),
        None,
    );
    assert_eq!(font.family.as_deref(), Some("Noto Sans"));
    assert_eq!(font.size, Some(FontSize::Pt(10.0)));
    assert_eq!(font.weight, Some(400));
    assert_eq!(font.style, Some(FontStyle::Normal));
    assert_eq!(font.width, None);

    let entry = DiagnosticEntry::new("kde", "no colour scheme file".to_owned());
    assert_eq!(entry.source, "kde");
    assert_eq!(entry.message, "no colour scheme file");

    let overrides = SystemOverrides {
        scheme: Some(Scheme::Dark),
        ..SystemOverrides::default()
    };
    let snapshot = SystemSnapshot::new(Preset::KdeBreeze, ThemeContext::dark(), overrides.clone());
    assert_eq!(snapshot.preset, Preset::KdeBreeze);
    assert_eq!(snapshot.context, ThemeContext::dark());
    assert_eq!(snapshot.overrides, overrides);
}
