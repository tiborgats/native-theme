use super::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// The settings of a read where nothing answered.
fn nothing_read() -> GnomePortalData {
    GnomePortalData {
        is_dark: false,
        accent_rgb: None,
        high_contrast: false,
        reduce_motion: None,
        font_name: None,
        monospace_font_name: None,
        titlebar_font: None,
        text_scaling_factor: None,
        session_type: None,
        overlay_scrolling: None,
        cursor_blink: None,
        cursor_blink_time: None,
        cursor_blink_timeout: None,
        icon_theme: None,
        gsettings_high_contrast: None,
    }
}

/// A read where nothing answered states the scheme (light), the name and
/// the font DPI alone, in the default context, on `adwaita`.
#[test]
fn nothing_read_states_the_three_slots_the_reader_always_sets() {
    let snapshot = snapshot_from_gnome_data(&nothing_read());
    assert_eq!(snapshot.preset, Preset::Adwaita);
    assert_eq!(snapshot.context, ThemeContext::default());
    let expected = SystemOverrides {
        scheme: Some(Scheme::Light),
        theme_name: Some(Arc::from("GNOME")),
        font_dpi: Some(crate::detect::LINUX_FONT_DPI),
        ..SystemOverrides::default()
    };
    assert_eq!(snapshot.overrides, expected);
}

/// Each setting reaches its slot: the scheme, the accent in 8 bits, the three
/// fonts, the icon theme, the overlay scroll bars and the caret's times as
/// the 0.7.2 reader computes them (the cycle halved, the time-out in
/// milliseconds).
#[test]
fn each_setting_reaches_its_slot() -> TestResult {
    let data = GnomePortalData {
        is_dark: true,
        accent_rgb: Some((
            0x35 as f64 / 255.0,
            0x84 as f64 / 255.0,
            0xe4 as f64 / 255.0,
        )),
        font_name: Some("Cantarell 11".to_owned()),
        monospace_font_name: Some("Source Code Pro 10".to_owned()),
        titlebar_font: Some("Cantarell Bold 11".to_owned()),
        overlay_scrolling: Some(false),
        cursor_blink_time: Some(1000),
        cursor_blink_timeout: Some(5),
        icon_theme: Some("Papirus".to_owned()),
        ..nothing_read()
    };
    let overrides = overrides_from_gnome_data(&data);
    assert_eq!(overrides.scheme, Some(Scheme::Dark));
    assert_eq!(overrides.accent, Some(Color::from_hex("#3584e4")?));
    let font = |family: &str, size: f32, weight: u16| OsFont {
        family: Some(Arc::from(family)),
        size: Some(FontSize::Pt(size)),
        weight: Some(weight),
        style: Some(FontStyle::Normal),
        width: None,
    };
    assert_eq!(overrides.ui_font, Some(font("Cantarell", 11.0, 400)));
    assert_eq!(
        overrides.monospace_font,
        Some(font("Source Code Pro", 10.0, 400))
    );
    assert_eq!(overrides.title_bar_font, Some(font("Cantarell", 11.0, 700)));
    assert_eq!(overrides.icon_theme.as_deref(), Some("Papirus"));
    assert_eq!(overrides.icon_sizes, None);
    assert_eq!(overrides.overlay_scrollbars, Some(false));
    assert_eq!(
        (
            overrides.caret_blink_on_ms,
            overrides.caret_blink_off_ms,
            overrides.caret_blink_timeout_ms
        ),
        (Some(500), Some(500), Some(5000))
    );
    assert!(overrides.color_schemes.is_empty());
    Ok(())
}

/// A caret that does not blink states an off time of 0 beside the half
/// cycle, as the 0.7.2 reader states it.
#[test]
fn a_caret_that_does_not_blink_is_off_for_0_ms() {
    let data = GnomePortalData {
        cursor_blink: Some(false),
        cursor_blink_time: Some(1200),
        ..nothing_read()
    };
    let overrides = overrides_from_gnome_data(&data);
    assert_eq!(overrides.caret_blink_on_ms, Some(600));
    assert_eq!(overrides.caret_blink_off_ms, Some(0));
}

/// An accent with a channel outside 0..=1 is not carried.
#[test]
fn an_accent_out_of_range_is_not_carried() {
    for accent in [(1.5, 0.0, 0.0), (0.0, -0.1, 0.0)] {
        let data = GnomePortalData {
            accent_rgb: Some(accent),
            ..nothing_read()
        };
        assert_eq!(overrides_from_gnome_data(&data).accent, None, "{accent:?}");
    }
}

/// High contrast is the portal's or gsettings'; reduced motion is the merged
/// answer's.
#[test]
fn contrast_and_motion_reach_the_context() {
    let portal = GnomePortalData {
        high_contrast: true,
        ..nothing_read()
    };
    let gsettings = GnomePortalData {
        gsettings_high_contrast: Some(true),
        ..nothing_read()
    };
    for data in [portal, gsettings] {
        assert_eq!(context_from_gnome_data(&data).0.contrast, Contrast::High);
    }
    let reduced = GnomePortalData {
        reduce_motion: Some(true),
        ..nothing_read()
    };
    assert_eq!(
        context_from_gnome_data(&reduced).0.motion,
        MotionPreference::Reduced
    );
    let full = GnomePortalData {
        reduce_motion: Some(false),
        ..nothing_read()
    };
    assert_eq!(
        context_from_gnome_data(&full).0.motion,
        MotionPreference::Full
    );
}

/// The text scale is the factor off X11, 1.0 on an X11 session, and 1.0 with
/// a diagnostic where the factor is not a finite number above 0.
#[test]
fn the_text_scale_off_x11_only_and_only_where_valid() {
    let scaled = |factor: f32, session: Option<&str>| {
        let data = GnomePortalData {
            text_scaling_factor: Some(factor),
            session_type: session.map(str::to_owned),
            ..nothing_read()
        };
        let snapshot = snapshot_from_gnome_data(&data);
        (
            snapshot.context.text_scale,
            snapshot.overrides.diagnostics.len(),
        )
    };
    assert_eq!(scaled(1.5, Some("wayland")), (1.5, 0));
    assert_eq!(scaled(1.5, None), (1.5, 0));
    assert_eq!(scaled(1.5, Some("x11")), (1.0, 0));
    for invalid in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert_eq!(scaled(invalid, Some("wayland")), (1.0, 1), "{invalid}");
    }
    assert_eq!(scaled(f32::NAN, Some("x11")), (1.0, 0));
}

/// A font string as Pango reads it: the style words, a pixel size, the first
/// entry of a family list; no font where the string has no size or no
/// family.
#[test]
fn fonts_as_pango_reads_them() {
    let read = |s: &str| font(Some(s));
    let bold_italic = read("Cantarell Bold Italic 11");
    assert_eq!(
        bold_italic.as_ref().and_then(|f| f.weight),
        Some(700),
        "{bold_italic:?}"
    );
    assert_eq!(bold_italic.and_then(|f| f.style), Some(FontStyle::Italic));
    assert_eq!(
        read("Cantarell 15px").and_then(|f| f.size),
        Some(FontSize::Px(15.0))
    );
    assert_eq!(
        read("Cantarell,Sans 11").and_then(|f| f.family).as_deref(),
        Some("Cantarell")
    );
    assert_eq!(read("Adwaita Sans"), None);
    assert_eq!(read("11"), None);
    assert_eq!(read(""), None);
    assert_eq!(
        read("Sans Ultra-Heavy 11").and_then(|f| f.weight),
        Some(900)
    );
}
