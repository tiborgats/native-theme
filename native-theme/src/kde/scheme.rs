//! The scheme the KDE reader reports: Plasma's own test of the palette's
//! window colour, the clean room's [`kde_color_scheme()`] (the 0.8.0 plan's
//! D10), as plasma-integration gives it to every Qt application on Plasma
//! and xdg-desktop-portal-kde publishes it as `org.freedesktop.appearance`
//! `color-scheme` (S9.3's *The KDE reader's readings*, ADD-A140).

use configparser::ini::Ini;

use super::kde_color_scheme::{self, kde_color_scheme};

use crate::selector::Scheme;

/// The scheme of the colours `ini` holds: dark where Qt's grey of the
/// active `Window` set's `BackgroundNormal` is below Plasma's threshold,
/// else light (`plasma-integration@6.8.2
/// qt6/src/platformtheme/khintssettings.cpp:469-485`,
/// `xdg-desktop-portal-kde@6.8.2 src/settings.cpp:259-273`,
/// `qtbase@6.12.0 src/gui/painting/qrgb.h:36-37`); an entry the file leaves
/// unset, or one KConfig reads as no colour, is KColorScheme's `Window`
/// default (`kcolorscheme@6.31.0 src/kcolorscheme.cpp:136`), which the
/// helper holds.
pub(super) fn scheme(ini: &Ini) -> Scheme {
    match kde_color_scheme(ini.get("Colors:Window", "BackgroundNormal").as_deref()) {
        kde_color_scheme::Scheme::Light => Scheme::Light,
        kde_color_scheme::Scheme::Dark => Scheme::Dark,
    }
}
