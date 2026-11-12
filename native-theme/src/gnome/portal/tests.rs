//! The portal-only read's tests: the portal first, then 0.7.0's chain for
//! the scheme and the motion (S16's *Which reader*, ADD-A135).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::*;
use crate::color::Color;

fn no_gsettings(_schema: &str, _key: &str) -> Option<String> {
    None
}

fn no_settings_ini() -> Option<String> {
    None
}

/// The reading of a portal giving nothing, `GTK_THEME` `gtk_theme`,
/// gsettings answering `gsetting` and `settings.ini` holding `ini`.
fn read_with(
    gtk_theme: Option<&str>,
    gsetting: impl FnMut(&str, &str) -> Option<String>,
    ini: Option<&str>,
) -> Reading {
    build_reading(
        None,
        None,
        PortalContrast::NoPreference,
        None,
        gtk_theme,
        gsetting,
        || ini.map(str::to_owned),
    )
}

/// gsettings answering `value` for `key` alone.
fn answering(key: &'static str, value: &'static str) -> impl FnMut(&str, &str) -> Option<String> {
    move |_, asked| (asked == key).then(|| value.to_owned())
}

// === the scheme ===

/// The portal's scheme wins over every later step.
#[test]
fn the_portals_scheme_comes_first() {
    for (portal, scheme) in [
        (ColorScheme::PreferDark, Scheme::Dark),
        (ColorScheme::PreferLight, Scheme::Light),
        (ColorScheme::NoPreference, Scheme::Light),
    ] {
        let reading = build_reading(
            Some(portal),
            None,
            PortalContrast::NoPreference,
            None,
            Some("Adwaita:dark"),
            answering("color-scheme", "prefer-dark"),
            || Some("gtk-application-prefer-dark-theme=1".to_owned()),
        );
        assert_eq!(reading.state.scheme, scheme, "{portal:?}");
    }
}

/// Where the portal gives no scheme, 0.7.0's chain decides
/// (`crate::linux_chain`'s tests hold its steps): `GTK_THEME`, gsettings'
/// `color-scheme`, `settings.ini`.
#[test]
fn without_the_portals_scheme_the_chain_decides() {
    let reading = read_with(
        Some("Adwaita:dark"),
        answering("color-scheme", "prefer-light"),
        None,
    );
    assert_eq!(reading.state.scheme, Scheme::Dark);
    let reading = read_with(None, answering("color-scheme", "prefer-dark"), None);
    assert_eq!(reading.state.scheme, Scheme::Dark);
    let reading = read_with(
        None,
        no_gsettings,
        Some("gtk-application-prefer-dark-theme=1"),
    );
    assert_eq!(reading.state.scheme, Scheme::Dark);
    let reading = read_with(None, no_gsettings, None);
    assert_eq!(reading.state.scheme, Scheme::Light);
}

/// `settings.ini` is read only where the chain reaches it.
#[test]
fn settings_ini_is_read_only_when_reached() {
    let reading = build_reading(
        Some(ColorScheme::PreferDark),
        None,
        PortalContrast::NoPreference,
        None,
        None,
        no_gsettings,
        || panic!("settings.ini read"),
    );
    assert_eq!(reading.state.scheme, Scheme::Dark);
}

// === the conditions ===

/// High contrast is the portal's alone; reduced motion the portal's, else
/// gsettings' `reduced-motion`; animations off where `enable-animations`
/// is false, apart from reduced motion (S7.1).
#[test]
fn the_conditions_read() {
    let reading = build_reading(
        None,
        None,
        PortalContrast::High,
        Some(ReducedMotion::NoPreference),
        None,
        |_, key| match key {
            "reduced-motion" => Some("reduce".to_owned()),
            "enable-animations" => Some("false".to_owned()),
            "high-contrast" => Some("true".to_owned()),
            _ => None,
        },
        no_settings_ini,
    );
    assert_eq!(reading.state.contrast, Some(Contrast::High));
    assert!(!reading.state.reduced_motion);
    assert!(reading.state.animations_off);
    let reading = read_with(None, answering("reduced-motion", "reduce"), None);
    assert!(reading.state.reduced_motion && !reading.state.animations_off);
    let reading = read_with(None, answering("high-contrast", "true"), None);
    assert_eq!(reading.state.contrast, None);
}

/// Nothing read: the light scheme, unscaled, every flag false, an empty
/// layer.
#[test]
fn nothing_read_is_the_no_reader_state() {
    let reading = read_with(None, no_gsettings, None);
    assert_eq!(reading.state, ReaderState::new(Scheme::Light, UNSCALED));
    assert_eq!(reading.layer, Layer::default());
    assert_eq!(reading.fallback, None);
}

/// Every gsettings key the read asks is one its `# Sources` list, once.
#[test]
fn one_gsettings_read_per_listed_key() {
    let mut keys = Vec::new();
    let _ = read_with(
        None,
        |schema, key| {
            keys.push(format!("{schema} {key}"));
            None
        },
        None,
    );
    assert_eq!(
        keys,
        [
            "org.gnome.desktop.interface color-scheme",
            "org.gnome.desktop.a11y.interface reduced-motion",
            "org.gnome.desktop.interface enable-animations",
        ]
    );
}

// === the accent ===

/// The portal's accent, classified as the GNOME reader classifies it, at
/// both schemes in the common bases; nothing under `for_preset`.
#[test]
fn the_accent_is_the_gnome_readers() {
    let reading = build_reading(
        None,
        Some(ashpd::desktop::Color::new(0.25, 0.5, 0.75)),
        PortalContrast::NoPreference,
        None,
        None,
        no_gsettings,
        no_settings_ini,
    );
    assert!(reading.layer.for_preset.is_empty());
    let [(name, value)] = reading.layer.bases.inputs.entries.as_slice() else {
        panic!("one input");
    };
    assert_eq!(*name, InputName::accent());
    let blue = Color::parse("#3584e4").map(Value::Stated);
    assert_eq!(value.per_scheme.get(Scheme::Light).cloned(), blue);
    assert_eq!(value.per_scheme.get(Scheme::Dark).cloned(), blue);
}
