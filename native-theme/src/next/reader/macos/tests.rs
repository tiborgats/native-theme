use super::*;
use crate::macos::WidgetFontData;

/// A font as 0.7.2's macOS reader tests make one (`macos.rs`,
/// `sample_widget_fonts`, `sample_light_defaults`): a family, a size in
/// points and a weight where given.
fn spec(family: &str, size: f32, weight: Option<u16>) -> crate::model::FontSpec {
    crate::model::FontSpec {
        family: Some(family.into()),
        size: Some(crate::model::font::FontSize::Pt(size)),
        weight,
        ..Default::default()
    }
}

/// A distinct colour per read, `(n, n, n)` for the `n`th of the 24 reads in
/// the order of `AppearanceColors`, plus `offset`.
fn distinct_colors(offset: u8) -> AppearanceColors {
    let c = |n: u8| Some(crate::Rgba::rgb(n + offset, n + offset, n + offset));
    AppearanceColors {
        label: c(1),
        control_accent: c(2),
        window_background: c(3),
        control_background: c(4),
        separator: c(5),
        secondary_label: c(6),
        alternate_selected_control_text: c(7),
        system_red: c(8),
        system_orange: c(9),
        system_green: c(10),
        system_blue: c(11),
        selected_content_background: c(12),
        unemphasized_selected_content_background: c(13),
        selected_text_background: c(14),
        selected_text: c(15),
        link: c(16),
        keyboard_focus_indicator: c(17),
        disabled_control_text: c(18),
        placeholder_text: c(19),
        text_insertion_point: c(20),
        alternate_row: c(21),
        header_text: c(22),
        grid: c(23),
        window_frame_text: c(24),
    }
}

/// A read on macOS 15.7.9 in the light appearance with every colour read
/// under both appearances ([`distinct_colors`], the dark ones 100 higher),
/// 0.7.2's sample fonts, the overlay scroller of width 16 (legacy 15 and
/// overlay 16 on macOS 14.8.9, `docs/platform-facts.md` §2.8), and no
/// accessibility flag set.
fn read() -> MacosSettings {
    MacosSettings {
        light: Some(distinct_colors(0)),
        dark: Some(distinct_colors(100)),
        font: spec("SF Pro", 13.0, None),
        mono_font: spec("SF Mono", 13.0, None),
        widget_fonts: WidgetFontData {
            menu_font: spec("SF Pro", 14.0, Some(400)),
            tooltip_font: spec("SF Pro", 11.0, Some(400)),
            title_bar_font: spec("SF Pro", 13.0, Some(700)),
        },
        scrollbar: Some(crate::ScrollbarTheme {
            groove_width: Some(16.0),
            overlay_mode: Some(true),
            ..Default::default()
        }),
        reduce_motion: Some(false),
        increase_contrast: Some(false),
        reduce_transparency: Some(false),
        is_dark: false,
        version: OsVersion::new(15, 7, 9),
    }
}

/// The colour `(n, n, n)`.
fn grey(n: u8) -> Option<Color> {
    Some(Color::from_srgb8(n, n, n, 255))
}

/// Each of the 24 colours reaches its slot of the appearance's entry, at
/// `Contrast::Normal`, the light one under Aqua and the dark one under Dark
/// Aqua.
#[test]
fn every_colour_reaches_its_slot() {
    let overrides = overrides_from_settings(&read());
    assert_eq!(overrides.color_schemes.len(), 2);
    for (scheme, offset) in [(Scheme::Light, 0u8), (Scheme::Dark, 100)] {
        let c = |n: u8| grey(n + offset);
        let colors = overrides
            .colors_for(scheme, Contrast::Normal)
            .cloned()
            .unwrap_or_default();
        let expected = OsColors {
            surfaces: Some(OsSurfaceOverrides {
                window: Some(SurfaceColors::new(c(3), c(1), c(6), c(5), None)),
                view: Some(SurfaceColors::new(c(4), None, None, None, None)),
                title_bar: Some(SurfaceColors::new(None, c(24), None, None, None)),
                ..OsSurfaceOverrides::default()
            }),
            selection: Some(OsSelectionColors::new(c(12), c(7), c(13), None)),
            text_selection: Some(OsSelectionColors::new(c(14), c(15), None, None)),
            accent: c(2),
            focus_ring: c(17),
            link: c(16),
            status: Some([c(8), c(9), c(10), c(11)]),
            disabled_text: c(18),
            placeholder: c(19),
            caret: c(20),
            list_header_foreground: c(22),
            grid: c(23),
            alternate_row_background: c(21),
            ..OsColors::default()
        };
        assert_eq!(colors, expected, "{scheme:?}");
    }
}

/// The scheme-independent settings: the scheme, the name, the font DPI 72,
/// the five fonts with 0.7.2's parse (family, points, weight, no style) and
/// the scroller's style and width.
#[test]
fn a_read_states_its_scheme_independent_settings() {
    let overrides = overrides_from_settings(&read());
    let os_font = |family: &str, size, weight| {
        Some(OsFont::new(
            Some(Arc::from(family)),
            Some(FontSize::Pt(size)),
            weight,
            None,
            None,
        ))
    };
    let expected = SystemOverrides {
        color_schemes: overrides.color_schemes.clone(),
        scheme: Some(Scheme::Light),
        theme_name: Some(Arc::from("macOS")),
        ui_font: os_font("SF Pro", 13.0, None),
        monospace_font: os_font("SF Mono", 13.0, None),
        menu_font: os_font("SF Pro", 14.0, Some(400)),
        tooltip_font: os_font("SF Pro", 11.0, Some(400)),
        title_bar_font: os_font("SF Pro", 13.0, Some(700)),
        font_dpi: Some(72.0),
        overlay_scrollbars: Some(true),
        scrollbar_width_px: Some(16.0),
        ..SystemOverrides::default()
    };
    assert_eq!(overrides, expected);
}

/// An appearance AppKit does not give has no entry; a scroller not read on
/// the main thread states neither its style nor its width.
#[test]
fn what_was_not_read_is_not_stated() {
    let overrides = overrides_from_settings(&MacosSettings {
        dark: None,
        scrollbar: None,
        ..read()
    });
    assert!(
        overrides
            .colors_for(Scheme::Dark, Contrast::Normal)
            .is_none()
    );
    assert!(
        overrides
            .colors_for(Scheme::Light, Contrast::Normal)
            .is_some()
    );
    assert_eq!(overrides.overlay_scrollbars, None);
    assert_eq!(overrides.scrollbar_width_px, None);
}

/// The context: the scheme of `AppleInterfaceStyle`, `Increased` contrast,
/// `Reduced` motion and reduced transparency where their flags are set, the
/// text scale 1.0; every flag unset or false leaves the default.
#[test]
fn the_flags_reach_the_context() {
    assert_eq!(context_from_settings(&read()), ThemeContext::default());
    let context = context_from_settings(&MacosSettings {
        is_dark: true,
        reduce_motion: Some(true),
        increase_contrast: Some(true),
        reduce_transparency: Some(true),
        ..read()
    });
    let mut expected = ThemeContext::dark();
    expected.contrast = Contrast::Increased;
    expected.motion = MotionPreference::Reduced;
    expected.reduced_transparency = true;
    assert_eq!(context, expected);
    assert_eq!(context.text_scale, 1.0);
    let unread = MacosSettings {
        reduce_motion: None,
        increase_contrast: None,
        reduce_transparency: None,
        ..read()
    };
    assert_eq!(context_from_settings(&unread), ThemeContext::default());
}

/// The snapshot's preset is the running version's generation, as 0.7.2's
/// generation table gives it (`macos.rs`,
/// `a_disabled_on_switch_is_the_accent_at_its_generations_alpha`).
#[test]
fn the_preset_is_the_running_generation() {
    for (version, preset) in [
        (OsVersion::new(12, 0, 0), Preset::MacosMonterey),
        (OsVersion::new(15, 7, 9), Preset::MacosMonterey),
        (OsVersion::new(16, 0, 0), Preset::MacosMonterey),
        (OsVersion::new(26, 6, 2), Preset::MacosTahoe),
        (OsVersion::new(27, 0, 1), Preset::MacosTahoe),
    ] {
        let snapshot = snapshot_from_settings(&MacosSettings { version, ..read() });
        assert_eq!(snapshot.preset, preset, "{version:?}");
    }
}
