//! 0.7.0's scheme and motion chain's tests: its steps in order, and the
//! reading it gives where the desktop's reader is not compiled (S16's
//! *Which reader*, ADD-A135, ADD-A148).

use super::*;

fn no_gsettings(_schema: &str, _key: &str) -> Option<String> {
    None
}

/// gsettings answering `value` for `key` alone.
fn answering(key: &'static str, value: &'static str) -> impl FnMut(&str, &str) -> Option<String> {
    move |_, asked| (asked == key).then(|| value.to_owned())
}

/// The chain's scheme from `GTK_THEME` `gtk_theme`, gsettings answering
/// `gsetting` and `settings.ini` holding `ini`.
fn scheme_with(
    gtk_theme: Option<&str>,
    gsetting: impl FnMut(&str, &str) -> Option<String>,
    ini: Option<&str>,
) -> Scheme {
    scheme(gtk_theme, gsetting, || ini.map(str::to_owned))
}

// === the scheme ===

/// `GTK_THEME` names a dark theme by a `:dark` suffix or `-dark`, in any
/// case; it is asked before gsettings.
#[test]
fn gtk_theme_dark_comes_before_gsettings() {
    for theme in ["Adwaita:dark", "Adwaita:DARK", "Arc-Dark", "Yaru-dark-blue"] {
        let scheme = scheme_with(Some(theme), answering("color-scheme", "prefer-light"), None);
        assert_eq!(scheme, Scheme::Dark, "{theme}");
    }
    // A light `GTK_THEME` is no answer: the chain goes on.
    let scheme = scheme_with(
        Some("Adwaita"),
        answering("color-scheme", "prefer-dark"),
        None,
    );
    assert_eq!(scheme, Scheme::Dark);
}

/// gsettings' `color-scheme`: `prefer-dark` dark, `prefer-light` and
/// `default` light, each before `settings.ini`; another value is no answer.
#[test]
fn gsettings_color_scheme_then_settings_ini() {
    let dark_ini = Some("[Settings]\ngtk-application-prefer-dark-theme = true\n");
    for (value, expected) in [
        ("prefer-dark", Scheme::Dark),
        ("prefer-light", Scheme::Light),
        ("default", Scheme::Light),
        ("unknown", Scheme::Dark),
    ] {
        let scheme = scheme_with(None, answering("color-scheme", value), dark_ini);
        assert_eq!(scheme, expected, "{value}");
    }
}

/// `settings.ini` prefers dark with `1` or `true`; nothing read is light.
#[test]
fn settings_ini_is_the_last_step() {
    for (ini, expected) in [
        (Some("gtk-application-prefer-dark-theme=1"), Scheme::Dark),
        (
            Some("gtk-application-prefer-dark-theme = TRUE"),
            Scheme::Dark,
        ),
        (Some("gtk-application-prefer-dark-theme=0"), Scheme::Light),
        (Some("gtk-theme-name=Adwaita-dark"), Scheme::Light),
        (None, Scheme::Light),
    ] {
        assert_eq!(scheme_with(None, no_gsettings, ini), expected, "{ini:?}");
    }
}

/// `settings.ini` is read only where the chain reaches it.
#[test]
fn settings_ini_is_read_only_when_reached() {
    let read = std::cell::Cell::new(false);
    let ini = || {
        read.set(true);
        None
    };
    assert_eq!(
        scheme(None, answering("color-scheme", "prefer-dark"), ini),
        Scheme::Dark
    );
    assert!(!read.get());
}

/// A spent deadline reads nothing, without running gsettings.
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

// === the motion ===

/// Reduced motion is gsettings' `reduced-motion` (`reduce`), animations off
/// `enable-animations` false, the two apart (S7.1).
#[test]
fn reduced_motion_and_animations_off_are_apart() {
    assert_eq!(
        reduced_motion(answering("reduced-motion", "reduce")),
        Some(true)
    );
    assert_eq!(
        reduced_motion(answering("reduced-motion", "no-preference")),
        Some(false)
    );
    assert_eq!(reduced_motion(no_gsettings), None);
    assert!(animations_off(answering("enable-animations", "false")));
    assert!(!animations_off(answering("enable-animations", "true")));
    assert!(!animations_off(answering("enable-animations", "maybe")));
    assert!(!animations_off(no_gsettings));
    assert!(!animations_off(answering("reduced-motion", "reduce")));
}

// === the reading where the desktop's reader is not compiled ===

/// The failure the readings below stand in for (synthetic).
#[cfg(not(all(feature = "portal", feature = "kde")))]
fn uncompiled() -> ReaderFailure {
    ReaderFailure::Platform("a synthetic reader is not compiled".to_owned())
}

/// The chain's reading: the scheme and the motion read, nothing else, an
/// empty layer, the fallback naming the reader it stands in for.
#[cfg(not(all(feature = "portal", feature = "kde")))]
#[test]
fn the_chains_reading_states_the_scheme_and_motion_alone() {
    let reading = build_reading(
        Some("Adwaita:dark"),
        |_, key| match key {
            "reduced-motion" => Some("reduce".to_owned()),
            "enable-animations" => Some("false".to_owned()),
            "high-contrast" => Some("true".to_owned()),
            "text-scaling-factor" => Some("1.5".to_owned()),
            "overlay-scrolling" => Some("true".to_owned()),
            _ => None,
        },
        || None,
        uncompiled(),
    );
    let mut expected = ReaderState::new(Scheme::Dark, UNSCALED);
    expected.reduced_motion = true;
    expected.animations_off = true;
    assert_eq!(reading.state, expected);
    assert_eq!(reading.layer, Layer::default());
    assert_eq!(reading.fallback, Some(uncompiled()));
}

/// Nothing read: the light scheme, unscaled, every flag false, an empty
/// layer; still the fallback, the reader not having read.
#[cfg(not(all(feature = "portal", feature = "kde")))]
#[test]
fn nothing_read_is_the_no_reader_state() {
    let reading = build_reading(None, no_gsettings, || None, uncompiled());
    assert_eq!(reading.state, ReaderState::new(Scheme::Light, UNSCALED));
    assert_eq!(reading.layer, Layer::default());
    assert_eq!(reading.fallback, Some(uncompiled()));
}

/// The gsettings keys the chain asks, each once, in its order: the ones its
/// `# Sources` list.
#[cfg(not(all(feature = "portal", feature = "kde")))]
#[test]
fn one_gsettings_read_per_listed_key() {
    let mut keys = Vec::new();
    let _ = build_reading(
        None,
        |schema, key| {
            keys.push(format!("{schema} {key}"));
            None
        },
        || None,
        uncompiled(),
    );
    assert_eq!(
        keys,
        [
            "org.gnome.desktop.interface color-scheme",
            "org.gnome.desktop.a11y.interface reduced-motion",
            "org.gnome.desktop.interface enable-animations",
        ]
    );
    let sources = include_str!("../linux_chain.rs");
    for key in &keys {
        assert!(sources.contains(&format!("//! - `{key}`")), "{key}");
    }
}
