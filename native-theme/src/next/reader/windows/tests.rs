use super::*;
use crate::windows::{
    AccessibilityData, AllFonts, CARET_BLINK_FAILED, CARET_BLINK_NONE, WidgetSizing,
};

/// A light read as 0.7.2's Windows reader tests make one (`windows.rs`,
/// `light_reader`): the accent `#0078D7`, a black foreground, no shade, no
/// font, no icon size, no accessibility setting, no system colour; the
/// scrollbar and border metrics of the reader's test stub
/// (`read_widget_sizing`) and no caret blink time.
fn light_read() -> WindowsSettings {
    WindowsSettings {
        accent: crate::Rgba::rgb(0, 120, 215),
        foreground: crate::Rgba::rgb(0, 0, 0),
        accent_shades: [None; 6],
        fonts: AllFonts {
            msg: crate::model::FontSpec::default(),
            caption: crate::model::FontSpec::default(),
            menu: crate::model::FontSpec::default(),
            status: crate::model::FontSpec::default(),
        },
        sizing: WidgetSizing {
            scrollbar_width: 17.0,
            min_thumb_length: 17.0,
            border_width: 1.0,
        },
        icon_sizes: None,
        accessibility: None,
        sys_colors: None,
        caret_blink_time: CARET_BLINK_FAILED,
    }
}

/// The accessibility settings of `high_contrast`, nothing else read.
fn accessibility(high_contrast: Option<bool>) -> AccessibilityData {
    AccessibilityData {
        text_scaling_factor: None,
        high_contrast,
        reduce_motion: None,
        advanced_effects_enabled: None,
    }
}

/// A distinct value per system colour, as 0.7.2's tests make them
/// (`windows.rs`, `distinct_sys_colors`).
fn distinct_sys_colors() -> SysColors {
    SysColors {
        window_bg: crate::Rgba::rgb(1, 1, 1),
        window_text: crate::Rgba::rgb(2, 2, 2),
        btn_face: crate::Rgba::rgb(3, 3, 3),
        btn_text: crate::Rgba::rgb(4, 4, 4),
        highlight: crate::Rgba::rgb(5, 5, 5),
        highlight_text: crate::Rgba::rgb(6, 6, 6),
        hotlight: crate::Rgba::rgb(8, 8, 8),
        gray_text: crate::Rgba::rgb(7, 7, 7),
    }
}

/// A light read states the scheme, the name, the font DPI, the accent, the
/// ramp of unread shades and the three metrics, in the default context, on
/// `windows-11`; no font, no caret time and no colour scheme.
#[test]
fn a_light_read_states_what_it_read() {
    let snapshot = snapshot_from_settings(&light_read());
    assert_eq!(snapshot.preset, Preset::Windows11);
    assert_eq!(snapshot.context, ThemeContext::default());
    let expected = SystemOverrides {
        scheme: Some(Scheme::Light),
        theme_name: Some(Arc::from("Windows")),
        accent: Some(Color::from_srgb8(0, 120, 215, 255)),
        windows_accent_ramp: Some(WindowsAccentRamp {
            light: [None; 3],
            dark: [None; 3],
        }),
        font_dpi: Some(96.0),
        scrollbar_width_px: Some(17.0),
        scrollbar_min_thumb_length_px: Some(17.0),
        border_width_px: Some(1.0),
        ..SystemOverrides::default()
    };
    assert_eq!(snapshot.overrides, expected);
}

/// A white foreground is the dark scheme, in the context and the slot.
#[test]
fn a_light_foreground_is_the_dark_scheme() {
    let snapshot = snapshot_from_settings(&WindowsSettings {
        foreground: crate::Rgba::rgb(255, 255, 255),
        ..light_read()
    });
    assert_eq!(snapshot.context.scheme, Scheme::Dark);
    assert_eq!(snapshot.overrides.scheme, Some(Scheme::Dark));
}

/// The shades reach the ramp by their names: `[AccentDark1..3,
/// AccentLight1..3]` as read, `AccentLight1` carried too.
#[test]
fn the_shades_reach_the_ramp() {
    let shade = |b| Some(crate::Rgba::rgb(0, 0, b));
    let snapshot = snapshot_from_settings(&WindowsSettings {
        accent_shades: [shade(1), shade(2), None, shade(4), shade(5), shade(6)],
        ..light_read()
    });
    let c = |b| Some(Color::from_srgb8(0, 0, b, 255));
    assert_eq!(
        snapshot.overrides.windows_accent_ramp,
        Some(WindowsAccentRamp {
            light: [c(4), c(5), c(6)],
            dark: [c(1), c(2), None],
        })
    );
}

/// The four fonts reach their slots with 0.7.2's parse: family, points and
/// weight, no style; the icon sizes reach `small` and `large`.
#[test]
fn the_fonts_and_icon_sizes_reach_their_slots() {
    let spec = |size, weight| crate::model::FontSpec {
        family: Some("Segoe UI".into()),
        size: Some(crate::model::font::FontSize::Pt(size)),
        weight,
        ..Default::default()
    };
    let snapshot = snapshot_from_settings(&WindowsSettings {
        fonts: AllFonts {
            msg: spec(9.0, Some(400)),
            caption: spec(9.0, Some(700)),
            menu: spec(9.0, None),
            status: spec(8.0, Some(400)),
        },
        icon_sizes: Some((16.0, 32.0)),
        ..light_read()
    });
    let font = |size, weight| {
        Some(OsFont::new(
            Some(Arc::from("Segoe UI")),
            Some(FontSize::Pt(size)),
            weight,
            None,
            None,
        ))
    };
    let o = &snapshot.overrides;
    assert_eq!(o.ui_font, font(9.0, Some(400)));
    assert_eq!(o.title_bar_font, font(9.0, Some(700)));
    assert_eq!(o.menu_font, font(9.0, None));
    assert_eq!(o.status_bar_font, font(8.0, Some(400)));
    assert_eq!(o.tooltip_font, None);
    assert_eq!(
        o.icon_sizes,
        Some(IconSizes::new(None, Some(16.0), Some(32.0), None, None))
    );
}

/// The caret's times are 0.7.2's mapping of the blink time: a time states
/// both, `INFINITE` an off time of 0 alone, a failed read neither.
#[test]
fn the_caret_times_are_the_readers_mapping() {
    let times = |time| {
        let o = overrides_from_settings(&WindowsSettings {
            caret_blink_time: time,
            ..light_read()
        });
        (
            o.caret_blink_on_ms,
            o.caret_blink_off_ms,
            o.caret_blink_timeout_ms,
        )
    };
    assert_eq!(times(500), (Some(500), Some(500), None));
    assert_eq!(times(CARET_BLINK_NONE), (None, Some(0), None));
    assert_eq!(times(CARET_BLINK_FAILED), (None, None, None));
}

/// Under high contrast the context is `High` and the entry `(scheme, High)`
/// holds the eight system colours in their slots.
#[test]
fn high_contrast_holds_the_system_colours() -> Result<(), Box<dyn std::error::Error>> {
    let snapshot = snapshot_from_settings(&WindowsSettings {
        foreground: crate::Rgba::rgb(255, 255, 255),
        accessibility: Some(accessibility(Some(true))),
        sys_colors: Some(distinct_sys_colors()),
        ..light_read()
    });
    assert_eq!(snapshot.context.contrast, Contrast::High);
    assert_eq!(snapshot.overrides.color_schemes.len(), 1);
    let colors = snapshot
        .overrides
        .colors_for(Scheme::Dark, Contrast::High)
        .ok_or("no high-contrast entry")?;
    let c = |v| Some(Color::from_srgb8(v, v, v, 255));
    let surfaces = colors.surfaces.as_ref().ok_or("no surfaces")?;
    let window = surfaces.window.ok_or("no window")?;
    let button = surfaces.button.ok_or("no button")?;
    assert_eq!((window.background, window.foreground), (c(1), c(2)));
    assert_eq!((button.background, button.foreground), (c(3), c(4)));
    let selection = colors.selection.ok_or("no selection")?;
    assert_eq!((selection.background, selection.foreground), (c(5), c(6)));
    assert_eq!(colors.link, c(8));
    assert_eq!(colors.disabled_text, c(7));
    Ok(())
}

/// Outside high contrast no entry is made, the colours being unread.
#[test]
fn outside_high_contrast_no_colour_scheme() {
    for high_contrast in [Some(false), None] {
        let snapshot = snapshot_from_settings(&WindowsSettings {
            accessibility: Some(accessibility(high_contrast)),
            ..light_read()
        });
        assert_eq!(snapshot.context.contrast, Contrast::Normal);
        assert!(snapshot.overrides.color_schemes.is_empty());
    }
}

/// Animations off are `MotionPreference::None`, transparency effects off
/// reduced transparency, the text scale factor the text scale; a factor
/// that is not a finite number above 0 is not carried and is diagnosed.
#[test]
fn the_accessibility_settings_reach_the_context() {
    let read = |factor| WindowsSettings {
        accessibility: Some(AccessibilityData {
            text_scaling_factor: Some(factor),
            high_contrast: Some(false),
            reduce_motion: Some(true),
            advanced_effects_enabled: Some(false),
        }),
        ..light_read()
    };
    let snapshot = snapshot_from_settings(&read(1.5));
    assert_eq!(snapshot.context.motion, MotionPreference::None);
    assert!(snapshot.context.reduced_transparency);
    assert_eq!(snapshot.context.text_scale, 1.5);
    assert!(snapshot.overrides.diagnostics.is_empty());
    for factor in [0.0, -1.0, f32::NAN] {
        let snapshot = snapshot_from_settings(&read(factor));
        assert_eq!(snapshot.context.text_scale, 1.0);
        assert_eq!(snapshot.overrides.diagnostics.len(), 1);
    }
}
