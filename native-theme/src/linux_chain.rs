//! 0.7.0's scheme and motion chain on Linux (S16's *Which reader*,
//! ADD-A135, ADD-A148): the reads that need no feature.
//!
//! The portal-only read takes from it the scheme and the motion the portal
//! does not give. Where the desktop's reader is not compiled (no `portal`
//! for GNOME, Budgie and the desktops with no reader of their own, no `kde`
//! for KDE), it reads the desktop alone ([`read`]), under the preset that
//! reader would have used, in its conditions only, as 0.7.0's
//! `linux_fallback` gave the desktop's preset in the scheme it detected
//! (`pipeline.rs:1103-1110` at v0.7.0, caeddd6b). The read cannot fail: a
//! setting none of these gives keeps the no-reader state's value.
//!
//! # Sources
//!
//! - `org.gnome.desktop.interface color-scheme`: light or dark, where
//!   `GTK_THEME` names no dark theme.
//! - `org.gnome.desktop.a11y.interface reduced-motion`: reduced motion.
//! - `org.gnome.desktop.interface enable-animations`: animations off.
//!
//! # Sources not watched
//!
//! - `$GTK_THEME`: dark where it names a dark theme, fixed for a process.
//! - `gtk-3.0/settings.ini` in the user's config directory:
//!   `gtk-application-prefer-dark-theme`, the last step of the scheme's
//!   chain, a file no signal reports, as 0.7.0 did not watch it.

use crate::selector::Scheme;

#[cfg(not(all(feature = "portal", feature = "kde")))]
use crate::error::ReaderFailure;
#[cfg(not(all(feature = "portal", feature = "kde")))]
use crate::layer::Layer;
#[cfg(not(all(feature = "portal", feature = "kde")))]
use crate::reader::Reading;
#[cfg(not(all(feature = "portal", feature = "kde")))]
use crate::resolve::ReaderState;

/// The text-scaling factor of unscaled text, S16's no-reader state's: the
/// chain reads none (S9.4).
pub(crate) const UNSCALED: f32 = 1.0;

/// One deadline for all the gsettings reads of one read together, so that
/// an unresponsive D-Bus costs it this once rather than once per key.
/// Ruled equal to the bound [`crate::detect`] gives one subprocess.
pub(crate) const GSETTINGS_DEADLINE: std::time::Duration = crate::detect::SUBPROCESS_TIMEOUT;

/// Read a single gsettings value within `timeout`, returning None if
/// gsettings is unavailable, fails or outlives it, or `timeout` is zero.
///
/// Single-quote wrapping (gsettings string output format) is stripped from
/// the result.
pub(crate) fn read_gsetting(
    schema: &str,
    key: &str,
    timeout: std::time::Duration,
) -> Option<String> {
    if timeout.is_zero() {
        return None;
    }
    let raw = crate::detect::gsettings_get(schema, key, timeout)?;
    let trimmed = raw.trim_matches('\'').to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// The gsettings reader of one read: each key through [`read_gsetting`],
/// all of them within one [`GSETTINGS_DEADLINE`] from now.
pub(crate) fn gsettings() -> impl FnMut(&str, &str) -> Option<String> {
    let start = std::time::Instant::now();
    move |schema, key| {
        read_gsetting(
            schema,
            key,
            GSETTINGS_DEADLINE.saturating_sub(start.elapsed()),
        )
    }
}

/// `$GTK_THEME`, where it is set.
pub(crate) fn gtk_theme() -> Option<String> {
    std::env::var("GTK_THEME").ok()
}

/// The text of `gtk-3.0/settings.ini` in the user's config directory
/// ([`crate::detect::xdg_config_home`]), where it is readable.
pub(crate) fn settings_ini() -> Option<String> {
    crate::detect::xdg_config_home()
        .and_then(|dir| std::fs::read_to_string(dir.join("gtk-3.0/settings.ini")).ok())
}

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

/// The scheme: dark where `gtk_theme` names a dark theme; else gsettings'
/// `color-scheme` where it names one; else dark where `settings_ini`, read
/// only when the chain reaches it, prefers it; else light.
pub(crate) fn scheme(
    gtk_theme: Option<&str>,
    gsetting: impl FnOnce(&str, &str) -> Option<String>,
    settings_ini: impl FnOnce() -> Option<String>,
) -> Scheme {
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

/// gsettings' `org.gnome.desktop.a11y.interface reduced-motion`: `reduce`
/// against `no-preference`; `None` where it gives none.
pub(crate) fn reduced_motion(gsetting: impl FnOnce(&str, &str) -> Option<String>) -> Option<bool> {
    gsetting("org.gnome.desktop.a11y.interface", "reduced-motion").map(|value| value == "reduce")
}

/// Whether gsettings' `org.gnome.desktop.interface enable-animations` is
/// false.
pub(crate) fn animations_off(gsetting: impl FnOnce(&str, &str) -> Option<String>) -> bool {
    gsetting("org.gnome.desktop.interface", "enable-animations")
        .and_then(|value| value.parse::<bool>().ok())
        == Some(false)
}

/// The chain's reading from `GTK_THEME`, the gsettings values `gsetting`
/// reads (one read per key) and `gtk-3.0/settings.ini`'s text, which
/// `settings_ini` reads where the chain reaches it; `uncompiled` is the
/// reader the build lacks, which the chain stands in for.
///
/// - The state: the scheme ([`scheme`]); reduced motion, gsettings'
///   `reduced-motion`; animations off where `enable-animations` is false.
///   Nothing else is read: no contrast, every other flag false, unscaled,
///   no font DPI.
/// - The layer: empty, nothing in the common bases or under `for_preset`.
/// - The fallback: `uncompiled`, so the theme's status tells this reading
///   from the reader's.
#[cfg(not(all(feature = "portal", feature = "kde")))]
pub(crate) fn build_reading(
    gtk_theme: Option<&str>,
    mut gsetting: impl FnMut(&str, &str) -> Option<String>,
    settings_ini: impl FnOnce() -> Option<String>,
    uncompiled: ReaderFailure,
) -> Reading {
    let mut state = ReaderState::new(scheme(gtk_theme, &mut gsetting, settings_ini), UNSCALED);
    state.reduced_motion = reduced_motion(&mut gsetting).unwrap_or(false);
    state.animations_off = animations_off(&mut gsetting);
    Reading {
        state,
        layer: Layer::default(),
        fallback: Some(uncompiled),
    }
}

/// Read the desktop through the chain alone: `$GTK_THEME`, gsettings
/// (sharing one [`GSETTINGS_DEADLINE`]) and `gtk-3.0/settings.ini`, standing
/// in for the reader `uncompiled` names ([`build_reading`]).
#[cfg(not(all(feature = "portal", feature = "kde")))]
pub(crate) fn read(uncompiled: ReaderFailure) -> Reading {
    build_reading(
        gtk_theme().as_deref(),
        gsettings(),
        settings_ini,
        uncompiled,
    )
}

#[cfg(test)]
mod tests;
