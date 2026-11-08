//! The portal-only read (S16's *Which reader*, ADD-A135): a Linux desktop
//! with no reader of its own (Xfce, Cinnamon, MATE, LXQt, the Wayland
//! compositors, COSMIC, one the refinement leaves unnamed) is read under
//! `adwaita`, in its common bases and conditions only, nothing under
//! `for_preset`.
//!
//! The portal's `org.freedesktop.appearance` settings come first, as the
//! GNOME reader reads them; then, for the scheme and the motion the portal
//! does not give, 0.7.0's own chain on these desktops (`detect.rs` at
//! v0.7.0, caeddd6b, `:727-810`, `:952-988`). The read cannot fail: a
//! setting none of them gives keeps the no-reader state's value.
//!
//! # Sources
//!
//! - `org.freedesktop.appearance color-scheme`: light or dark.
//! - `org.freedesktop.appearance accent-color`: the accent colour, which
//!   libadwaita classifies into one of its nine accents.
//! - `org.freedesktop.appearance contrast`: high contrast.
//! - `org.freedesktop.appearance reduced-motion`: reduced motion.
//! - `org.gnome.desktop.interface color-scheme`: light or dark, where the
//!   portal and `GTK_THEME` give none.
//! - `org.gnome.desktop.interface enable-animations`: animations off.
//! - `org.gnome.desktop.a11y.interface reduced-motion`: reduced motion, where
//!   the portal gives none.
//!
//! # Sources not watched
//!
//! - `$GTK_THEME`: dark where it names a dark theme, fixed for a process.
//! - `gtk-3.0/settings.ini` in the user's config directory:
//!   `gtk-application-prefer-dark-theme`, the last step of the scheme's
//!   chain; the watcher is the portal's, whose `SettingChanged` tells no
//!   change of a file, as 0.7.0's did not watch it.

use ashpd::desktop::settings::{ColorScheme, Contrast as PortalContrast, ReducedMotion};

use super::{GSETTINGS_DEADLINE, portal_accent, read_gsetting, read_portal, reduced_motion};
use crate::inputs::{Contrast, InputName, InputValue};
use crate::layer::Layer;
use crate::reader::Reading;
use crate::resolve::ReaderState;
use crate::selector::Scheme;
use crate::value::{SchemeMap, Value};

/// The text-scaling factor of unscaled text, S16's no-reader state's: these
/// desktops' read gives none (S9.4).
const UNSCALED: f32 = 1.0;

/// Whether `GTK_THEME` names a dark theme: lowercased, it ends in `:dark`
/// or holds `-dark` (0.7.0's `detect.rs:768-774`).
fn gtk_theme_is_dark(gtk_theme: &str) -> bool {
    let lower = gtk_theme.to_lowercase();
    lower.ends_with(":dark") || lower.contains("-dark")
}

/// gsettings' `org.gnome.desktop.interface color-scheme` as a scheme:
/// `prefer-dark` dark, `prefer-light` and `default` light, anything else
/// none (0.7.0's `detect.rs:727-742`).
fn gsettings_scheme(value: &str) -> Option<Scheme> {
    if value.contains("prefer-dark") {
        Some(Scheme::Dark)
    } else if value.contains("prefer-light") || value.contains("default") {
        Some(Scheme::Light)
    } else {
        None
    }
}

/// Whether `gtk-3.0/settings.ini`'s text sets
/// `gtk-application-prefer-dark-theme` to `1` or `true` (0.7.0's
/// `detect.rs:797-810`).
fn settings_ini_prefers_dark(content: &str) -> bool {
    content.lines().any(|line| {
        let line = line.trim();
        line.starts_with("gtk-application-prefer-dark-theme")
            && line.split('=').nth(1).is_some_and(|value| {
                let value = value.trim();
                value == "1" || value.eq_ignore_ascii_case("true")
            })
    })
}

/// The scheme: the portal's where it gives one; else dark where
/// `gtk_theme` names a dark theme; else gsettings' `color-scheme` where it
/// names one; else dark where `settings_ini` prefers it; else light.
fn scheme(
    portal: Option<ColorScheme>,
    gtk_theme: Option<&str>,
    gsetting: impl FnOnce(&str, &str) -> Option<String>,
    settings_ini: impl FnOnce() -> Option<String>,
) -> Scheme {
    if let Some(scheme) = portal {
        return if matches!(scheme, ColorScheme::PreferDark) {
            Scheme::Dark
        } else {
            Scheme::Light
        };
    }
    if gtk_theme.is_some_and(gtk_theme_is_dark) {
        return Scheme::Dark;
    }
    if let Some(scheme) = gsetting("org.gnome.desktop.interface", "color-scheme")
        .as_deref()
        .and_then(gsettings_scheme)
    {
        return scheme;
    }
    if settings_ini()
        .as_deref()
        .is_some_and(settings_ini_prefers_dark)
    {
        Scheme::Dark
    } else {
        Scheme::Light
    }
}

/// The portal-only reading from what the portal answered (`None` for a
/// setting it did not give), `GTK_THEME`, the gsettings values `gsetting`
/// reads (one read per key) and `gtk-3.0/settings.ini`'s text, which
/// `settings_ini` reads where the chain reaches it.
///
/// - The state: the scheme ([`scheme`]); `contrast_high` where the portal
///   asks for high contrast; reduced motion, the portal's, else gsettings'
///   `reduced-motion`; animations off where `enable-animations` is false.
///   Nothing else is read: every other flag false, unscaled, no font DPI.
/// - The common bases: `input.accent` at both schemes, as the GNOME reader
///   states it.
pub(crate) fn build_reading(
    portal_scheme: Option<ColorScheme>,
    accent: Option<ashpd::desktop::Color>,
    contrast: PortalContrast,
    portal_motion: Option<ReducedMotion>,
    gtk_theme: Option<&str>,
    mut gsetting: impl FnMut(&str, &str) -> Option<String>,
    settings_ini: impl FnOnce() -> Option<String>,
) -> Reading {
    let mut state = ReaderState::new(
        scheme(portal_scheme, gtk_theme, &mut gsetting, settings_ini),
        UNSCALED,
    );
    state.contrast = matches!(contrast, PortalContrast::High).then_some(Contrast::High);
    state.reduced_motion = reduced_motion(portal_motion, &mut gsetting).unwrap_or(false);
    state.animations_off = gsetting("org.gnome.desktop.interface", "enable-animations")
        .and_then(|value| value.parse::<bool>().ok())
        == Some(false);

    let mut layer = Layer::default();
    if let Some(accent) = accent
        .map(|c| (c.red(), c.green(), c.blue()))
        .and_then(portal_accent)
    {
        let accent = Value::Stated(accent);
        layer.bases.inputs.entries.push((
            InputName::accent(),
            InputValue::new(SchemeMap::new(accent.clone(), accent)),
        ));
    }
    Reading {
        state,
        layer,
        fallback: None,
    }
}

/// Read a desktop with no reader of its own: the portal, then
/// `$GTK_THEME`, gsettings (sharing one [`GSETTINGS_DEADLINE`]) and
/// `gtk-3.0/settings.ini` in the user's config directory
/// ([`crate::detect::xdg_config_home`]).
pub(crate) async fn read() -> Reading {
    let (portal_scheme, accent, contrast, portal_motion) = read_portal().await;
    let gtk_theme = std::env::var("GTK_THEME").ok();
    let start = std::time::Instant::now();
    build_reading(
        portal_scheme,
        accent,
        contrast,
        portal_motion,
        gtk_theme.as_deref(),
        |schema, key| {
            read_gsetting(
                schema,
                key,
                GSETTINGS_DEADLINE.saturating_sub(start.elapsed()),
            )
        },
        || {
            crate::detect::xdg_config_home()
                .and_then(|dir| std::fs::read_to_string(dir.join("gtk-3.0/settings.ini")).ok())
        },
    )
}

#[cfg(test)]
mod tests;
