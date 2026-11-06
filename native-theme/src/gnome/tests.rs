//! The GNOME reader's tests: what it reads where, and the state and layer
//! it builds (S9.3, S9.5).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use ashpd::desktop::Color as PortalColor;
use ashpd::desktop::settings::{ColorScheme, Contrast as PortalContrast, ReducedMotion};

use super::*;

/// A text-scaling factor other than the identity (synthetic).
const SCALED: f32 = 1.5;

fn no_gsettings(_schema: &str, _key: &str) -> Option<String> {
    None
}

/// The reading of a portal giving nothing but `scheme`, `contrast` and
/// `motion`, and gsettings answering `gsetting`.
fn read_with(
    portal_scheme: Option<ColorScheme>,
    contrast: PortalContrast,
    portal_motion: Option<ReducedMotion>,
    gsetting: impl FnMut(&str, &str) -> Option<String>,
) -> Reading {
    build_reading(
        portal_scheme,
        None,
        contrast,
        portal_motion,
        None,
        gsetting,
        PresetId::Adwaita,
    )
}

/// A GNOME session's data: light, no accent, nothing else asked for.
fn gnome_data() -> GnomeData {
    GnomeData {
        scheme: Scheme::Light,
        accent_rgb: None,
        high_contrast: false,
        gsettings_high_contrast: None,
        reduced_motion: None,
        animations_enabled: None,
        text_scaling_factor: None,
        session_type: None,
        overlay_scrolling: None,
        icon_theme: None,
    }
}

/// The accent the reading's common bases state, per scheme.
fn stated_accent(reading: &Reading) -> Option<[Option<Color>; 2]> {
    let (_, value) = reading
        .layer
        .bases
        .inputs
        .entries
        .iter()
        .find(|(name, _)| *name == InputName::accent())?;
    let at = |scheme| match value.per_scheme.get(scheme) {
        Some(Value::Stated(color)) => Some(*color),
        _ => None,
    };
    Some([at(Scheme::Light), at(Scheme::Dark)])
}

// === the scheme ===

#[test]
fn no_portal_scheme_reads_gsettings_color_scheme() {
    for (value, scheme) in [
        ("prefer-dark", Scheme::Dark),
        ("default", Scheme::Light),
        ("prefer-light", Scheme::Light),
    ] {
        let reading = read_with(None, PortalContrast::NoPreference, None, |_, key| {
            (key == "color-scheme").then(|| value.to_string())
        });
        assert_eq!(reading.state.scheme, scheme, "{value}");
    }
}

#[test]
fn portal_scheme_wins_over_gsettings() {
    let prefer_dark = |_: &str, key: &str| (key == "color-scheme").then(|| "prefer-dark".into());
    let reading = read_with(
        Some(ColorScheme::PreferLight),
        PortalContrast::NoPreference,
        None,
        prefer_dark,
    );
    assert_eq!(reading.state.scheme, Scheme::Light);
    let reading = read_with(
        Some(ColorScheme::PreferDark),
        PortalContrast::NoPreference,
        None,
        no_gsettings,
    );
    assert_eq!(reading.state.scheme, Scheme::Dark);
}

#[test]
fn nothing_read_is_the_light_scheme_unscaled() {
    let reading = read_with(None, PortalContrast::NoPreference, None, no_gsettings);
    assert_eq!(reading.state, ReaderState::new(Scheme::Light, UNSCALED));
    assert_eq!(reading.layer, Layer::default());
    assert_eq!(reading.fallback, None);
}

// === one read per key ===

/// Each gsettings key is read once at most, and a key the portal answers
/// for is not read: the scheme, the high contrast and the reduced motion.
#[test]
fn one_gsettings_read_per_key() {
    for (scheme, contrast, motion, reads) in [
        (None, PortalContrast::NoPreference, None, 7),
        (
            Some(ColorScheme::PreferDark),
            PortalContrast::NoPreference,
            None,
            6,
        ),
        (
            Some(ColorScheme::PreferDark),
            PortalContrast::High,
            Some(ReducedMotion::NoPreference),
            4,
        ),
    ] {
        let mut keys = Vec::new();
        let reading = read_with(scheme, contrast, motion, |schema, key| {
            keys.push(format!("{schema} {key}"));
            (key == "icon-theme").then(|| "Papirus".to_string())
        });
        assert_eq!(keys.len(), reads, "{keys:?}");
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), reads, "a key read twice");
        assert_eq!(reading.layer.for_preset.len(), 1);
    }
}

#[test]
fn a_spent_deadline_reads_nothing() {
    assert_eq!(
        read_gsetting(
            "org.gnome.desktop.interface",
            "icon-theme",
            std::time::Duration::ZERO
        ),
        None
    );
}

// === the accent ===

/// The portal's accent is the accent input at both schemes: GNOME's accent
/// setting is independent of the scheme (S9.3).
#[test]
fn the_portal_accent_is_the_accent_at_both_schemes() {
    for portal_scheme in [ColorScheme::PreferLight, ColorScheme::PreferDark] {
        let reading = build_reading(
            Some(portal_scheme),
            Some(PortalColor::new(0.25, 0.5, 0.75)),
            PortalContrast::NoPreference,
            None,
            None,
            no_gsettings,
            PresetId::Adwaita,
        );
        let accent = Color {
            r: 0.25,
            g: 0.5,
            b: 0.75,
            a: 1.0,
        };
        assert_eq!(stated_accent(&reading), Some([Some(accent), Some(accent)]));
    }
}

/// A channel outside `0..=1` is the portal's "accent not set": no accent
/// is stated, and the preset's shows.
#[test]
fn an_out_of_range_accent_is_unset() {
    for rgb in [(1.5, 0.0, 0.0), (-0.1, 0.5, 0.5), (0.5, 0.5, f64::NAN)] {
        let mut data = gnome_data();
        data.accent_rgb = Some(rgb);
        let reading = build_gnome_reading(&data, PresetId::Adwaita);
        assert_eq!(stated_accent(&reading), None, "{rgb:?}");
    }
}

#[test]
fn the_accents_range_is_inclusive() {
    for (rgb, channel) in [((0.0, 0.0, 0.0), 0.0), ((1.0, 1.0, 1.0), 1.0)] {
        let color = portal_accent(rgb).unwrap();
        assert_eq!(
            [color.r, color.g, color.b, color.a],
            [channel, channel, channel, 1.0]
        );
    }
}

// === the conditions ===

#[test]
fn high_contrast_is_the_portal_then_gsettings() {
    let gsettings = |value: &'static str| {
        move |_: &str, key: &str| (key == "high-contrast").then(|| value.to_string())
    };
    let contrast = |portal, value| {
        read_with(None, portal, None, gsettings(value))
            .state
            .contrast
    };
    assert_eq!(
        contrast(PortalContrast::High, "false"),
        Some(Contrast::High)
    );
    assert_eq!(
        contrast(PortalContrast::NoPreference, "true"),
        Some(Contrast::High)
    );
    assert_eq!(contrast(PortalContrast::NoPreference, "false"), None);
}

/// Reduced motion is the portal's, else gsettings' `reduced-motion`; a
/// false `enable-animations` turns animations off, a condition of its own
/// (GN-3.25, GN-3.26).
#[test]
fn reduced_motion_and_animations_off_are_apart() {
    let gsettings = |motion: &'static str, animations: &'static str| {
        move |_: &str, key: &str| match key {
            "reduced-motion" => Some(motion.to_string()),
            "enable-animations" => Some(animations.to_string()),
            _ => None,
        }
    };
    let read = |portal, motion, animations| {
        let state = read_with(
            None,
            PortalContrast::NoPreference,
            portal,
            gsettings(motion, animations),
        )
        .state;
        (state.reduced_motion, state.animations_off)
    };
    assert_eq!(read(None, "reduce", "true"), (true, false));
    assert_eq!(read(None, "no-preference", "false"), (false, true));
    assert_eq!(
        read(Some(ReducedMotion::NoPreference), "reduce", "true"),
        (false, false)
    );
    assert_eq!(
        read(Some(ReducedMotion::ReducedMotion), "no-preference", "true"),
        (true, false)
    );
}

#[test]
fn overlay_scrolling_is_the_overlay_scrollbars_condition() {
    for (overlay, expected) in [(Some(true), true), (Some(false), false), (None, false)] {
        let mut data = gnome_data();
        data.overlay_scrolling = overlay;
        let reading = build_gnome_reading(&data, PresetId::Adwaita);
        assert_eq!(reading.state.overlay_scrollbars, expected, "{overlay:?}");
    }
}

// === the text-scaling factor ===

/// On an X11 session the toolkit's scale factor, `Xft.dpi` over 96,
/// carries the text-scaling factor already, so the reader reports none.
#[test]
fn an_x11_session_reports_no_text_scale() {
    let mut data = gnome_data();
    data.text_scaling_factor = Some(SCALED);
    data.session_type = Some("x11".to_string());
    let reading = build_gnome_reading(&data, PresetId::Adwaita);
    assert_eq!(reading.state.text_scaling_factor, UNSCALED);
}

/// Off X11 no toolkit scale factor carries the text-scaling factor, so
/// the reader reports it: on a Wayland session, and where the session
/// type is not set. GNOME's font DPI is the preset's (S9.4).
#[test]
fn a_session_off_x11_reports_the_text_scale() {
    for session_type in [Some("wayland"), None] {
        let mut data = gnome_data();
        data.text_scaling_factor = Some(SCALED);
        data.session_type = session_type.map(str::to_string);
        let reading = build_gnome_reading(&data, PresetId::Adwaita);
        assert_eq!(
            reading.state.text_scaling_factor, SCALED,
            "{session_type:?}"
        );
        assert_eq!(reading.state.font_dpi, None);
    }
}

/// A factor S12 would refuse, not finite or not above zero, is not
/// reported.
#[test]
fn an_invalid_text_scale_is_not_reported() {
    for factor in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        let mut data = gnome_data();
        data.text_scaling_factor = Some(factor);
        let reading = build_gnome_reading(&data, PresetId::Adwaita);
        assert_eq!(reading.state.text_scaling_factor, UNSCALED, "{factor}");
    }
}

/// The reader passes the session type it is given to the X11 rule.
#[test]
fn build_reading_reads_the_session_type_it_is_given() {
    let factor = |session_type: Option<&str>| {
        build_reading(
            None,
            None,
            PortalContrast::NoPreference,
            None,
            session_type.map(str::to_string),
            |_: &str, key: &str| (key == "text-scaling-factor").then(|| SCALED.to_string()),
            PresetId::Adwaita,
        )
        .state
        .text_scaling_factor
    };
    assert_eq!(factor(Some("x11")), UNSCALED);
    assert_eq!(factor(Some("wayland")), SCALED);
}

// === the icon theme ===

/// The icon theme is the reader's own preset's (S9.5): under
/// `for_preset.<own>`, never in the common bases.
#[test]
fn the_icon_theme_is_the_own_presets() {
    let mut data = gnome_data();
    data.icon_theme = Some("Papirus".to_string());
    let reading = build_gnome_reading(&data, PresetId::MacosTahoe);
    assert_eq!(reading.layer.bases.tokens.icons, None);
    let [(own, bases)] = reading.layer.for_preset.as_slice() else {
        panic!("one preset's part");
    };
    assert_eq!(*own, PresetId::MacosTahoe);
    let theme = bases
        .tokens
        .icons
        .as_ref()
        .and_then(|icons| icons.theme.clone());
    assert_eq!(theme, Some(Value::Stated("Papirus".to_string())));
}

// === the portal backend and the deadline ===

const KDE_BACKEND: &str = "org.freedesktop.impl.portal.desktop.kde";
const GNOME_BACKEND: &str = "org.freedesktop.impl.portal.desktop.gnome";

#[test]
fn portal_backend_running_preferred_over_activatable() {
    assert_eq!(
        choose_portal_backend(&[KDE_BACKEND], &[GNOME_BACKEND, KDE_BACKEND]),
        Some(crate::detect::LinuxDesktop::Kde)
    );
}

#[test]
fn portal_backend_both_running_names_no_desktop() {
    assert_eq!(
        choose_portal_backend(&[GNOME_BACKEND, KDE_BACKEND], &[GNOME_BACKEND, KDE_BACKEND]),
        None
    );
}

#[test]
fn portal_backend_only_activatable_gnome() {
    assert_eq!(
        choose_portal_backend(&["org.freedesktop.DBus"], &[GNOME_BACKEND]),
        Some(crate::detect::LinuxDesktop::Gnome)
    );
}

#[test]
fn portal_backend_both_activatable_none_running_names_no_desktop() {
    assert_eq!(
        choose_portal_backend(&[], &[KDE_BACKEND, GNOME_BACKEND]),
        None
    );
    assert_eq!(choose_portal_backend(&[], &[]), None);
}

#[test]
fn deadline_ends_a_future_that_never_completes() {
    let deadline = std::time::Duration::from_millis(100);
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let start = std::time::Instant::now();
        let out = pollster::block_on(with_deadline(std::future::pending::<()>(), deadline));
        let _ = tx.send((out, start.elapsed()));
    });
    let (out, elapsed) = rx
        .recv_timeout(std::time::Duration::from_secs(5))
        .expect("with_deadline did not return within 5 s");
    assert_eq!(out, None);
    assert!(elapsed >= deadline, "returned after {elapsed:?}");
}

#[test]
fn deadline_passes_a_ready_value_through() {
    let out = pollster::block_on(with_deadline(
        std::future::ready(7),
        std::time::Duration::from_secs(5),
    ));
    assert_eq!(out, Some(7));
}
