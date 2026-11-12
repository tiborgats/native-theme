//! The platform readers (S9.3, S9.5): which one reads the running system,
//! with which preset as its own, and what one read gives.
//!
//! A reader reads the system into a [`ReaderState`] and a reader layer: its
//! common bases, which any preset takes, and its own platform's part under
//! `for_preset.<own>`, `own` being the preset the theme was built with
//! (S9.5). [`Theme::from_system`](crate::Theme::from_system) chooses the
//! reader and the preset ([`select`]); the theme keeps both, and
//! [`Theme::reload_reader`](crate::Theme::reload_reader) reads again with
//! the same.
//!
//! The readers: GNOME's (with the `portal` feature, on GNOME and Budgie),
//! the portal-only read of the Linux desktops with no reader of their own
//! (S16's *Which reader*, ADD-A135), KDE's (with the `kde` feature, on KDE
//! Plasma), macOS's (with the `macos` feature) and Windows' (with the
//! `windows` feature).

use crate::error::{ReaderFailure, ThemeError};
use crate::layer::Layer;
use crate::presets::PresetId;
use crate::resolve::ReaderState;

/// What one read gave: the state and the layer, and the source that failed
/// where another stood in for it (S9.1, S9.3: the status's `Fallback`).
#[cfg_attr(
    not(any(
        all(target_os = "linux", any(feature = "portal", feature = "kde")),
        all(target_os = "macos", feature = "macos"),
        all(target_os = "windows", feature = "windows")
    )),
    allow(dead_code, reason = "a reader's: no reader is linked in this build")
)]
pub(crate) struct Reading {
    /// The conditions, text-scaling factor and font DPI read.
    pub(crate) state: ReaderState,
    /// The reader layer.
    pub(crate) layer: Layer,
    /// The source that failed, where another stood in for it.
    pub(crate) fallback: Option<ReaderFailure>,
}

/// A platform reader: one variant per reader this build links.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SystemReader {
    /// The GNOME reader: the portal and gsettings.
    #[cfg(all(target_os = "linux", feature = "portal"))]
    Gnome,
    /// The portal-only read of a Linux desktop with no reader of its own:
    /// the portal, then 0.7.0's scheme and motion chain.
    #[cfg(all(target_os = "linux", feature = "portal"))]
    Portal,
    /// The KDE reader: kdeglobals and the scheme file it names.
    #[cfg(all(target_os = "linux", feature = "kde"))]
    Kde,
    /// The macOS reader: AppKit's appearances, colours and settings.
    #[cfg(all(target_os = "macos", feature = "macos"))]
    Macos,
    /// The Windows reader: `UISettings`, high contrast and its system
    /// colours.
    #[cfg(all(target_os = "windows", feature = "windows"))]
    Windows,
}

impl SystemReader {
    /// Reads the system, the reader's own platform's part under `own`.
    #[cfg_attr(
        not(any(
            all(target_os = "linux", any(feature = "portal", feature = "kde")),
            all(target_os = "macos", feature = "macos"),
            all(target_os = "windows", feature = "windows")
        )),
        allow(unused_variables, reason = "no reader is linked in this build")
    )]
    pub(crate) async fn read(self, own: PresetId) -> Result<Reading, ReaderFailure> {
        match self {
            #[cfg(all(target_os = "linux", feature = "portal"))]
            SystemReader::Gnome => Ok(crate::gnome::read(own).await),
            #[cfg(all(target_os = "linux", feature = "portal"))]
            SystemReader::Portal => Ok(crate::gnome::portal::read().await),
            #[cfg(all(target_os = "linux", feature = "kde"))]
            SystemReader::Kde => crate::kde::read(own),
            #[cfg(all(target_os = "macos", feature = "macos"))]
            SystemReader::Macos => crate::macos::read(own),
            #[cfg(all(target_os = "windows", feature = "windows"))]
            SystemReader::Windows => crate::windows::read(own),
        }
    }
}

/// The reader of the running system and its own preset: the preset
/// `generation` names where it is one of this platform's (R1.5), else the
/// platform's own.
///
/// On Linux the desktop decides, refined through the portal's backend where
/// `XDG_CURRENT_DESKTOP` names none ([`crate::detect::refined_linux_desktop`]):
/// GNOME and Budgie are read by the GNOME reader under `adwaita`; every
/// desktop with no reader of its own (Xfce, Cinnamon, MATE, LXQt, the
/// Wayland compositors, COSMIC, one the refinement leaves unnamed) by the
/// portal-only read, under `adwaita` too (S16's *Which reader*); either
/// [`ThemeError::FeatureDisabled`] without the `portal` feature. KDE is read
/// by the KDE reader under `kde-breeze`, [`ThemeError::FeatureDisabled`]
/// without the `kde` feature. Linux has
/// no generation table: the one preset of the desktop is its only
/// generation, so `generation` is accepted where it is that preset and is
/// [`ThemeError::PresetNotForPlatform`] otherwise.
///
/// On macOS the macOS reader reads, with the `macos` feature
/// ([`ThemeError::FeatureDisabled`] without it), under the running
/// version's generation ([`macos_preset`]).
///
/// On Windows the Windows reader reads, with the `windows` feature
/// ([`ThemeError::FeatureDisabled`] without it), under the running build's
/// generation ([`windows_preset`]). Another platform is
/// [`ThemeError::PlatformUnsupported`].
pub(crate) async fn select(
    generation: Option<PresetId>,
) -> Result<(SystemReader, PresetId), ThemeError> {
    #[cfg(target_os = "linux")]
    {
        linux_reader(crate::detect::refined_linux_desktop().await, generation)
    }
    #[cfg(all(target_os = "macos", feature = "macos"))]
    {
        macos_preset(crate::macos::os_version(), generation).map(|id| (SystemReader::Macos, id))
    }
    #[cfg(all(target_os = "macos", not(feature = "macos")))]
    {
        let _ = generation;
        Err(ThemeError::FeatureDisabled)
    }
    #[cfg(all(target_os = "windows", feature = "windows"))]
    {
        windows_preset(crate::windows::os_version(), generation)
            .map(|id| (SystemReader::Windows, id))
    }
    #[cfg(all(target_os = "windows", not(feature = "windows")))]
    {
        let _ = generation;
        Err(ThemeError::FeatureDisabled)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        let _ = generation;
        Err(ThemeError::PlatformUnsupported)
    }
}

/// The macOS preset of `running`: `generation` where it names one of the
/// generation table's presets (R1.5, `pipeline::MACOS_GENERATIONS`),
/// [`ThemeError::PresetNotForPlatform`] where it names another, else the
/// table's preset for `running`.
#[cfg_attr(
    not(all(target_os = "macos", feature = "macos")),
    allow(dead_code, reason = "the macOS reader's: not linked in this build")
)]
fn macos_preset(
    running: crate::pipeline::OsVersion,
    generation: Option<PresetId>,
) -> Result<PresetId, ThemeError> {
    table_preset(
        &crate::pipeline::MACOS_GENERATIONS,
        crate::pipeline::macos_major(running),
        generation,
    )
}

/// The Windows preset of `running`, as [`macos_preset`] over
/// `pipeline::WINDOWS_GENERATIONS`, by the running build.
#[cfg_attr(
    not(all(target_os = "windows", feature = "windows")),
    allow(dead_code, reason = "the Windows reader's: not linked in this build")
)]
fn windows_preset(
    running: crate::pipeline::OsVersion,
    generation: Option<PresetId>,
) -> Result<PresetId, ThemeError> {
    table_preset(&crate::pipeline::WINDOWS_GENERATIONS, running, generation)
}

/// The preset of `running` in `table`: `generation` where it names one of
/// the table's presets (R1.5), [`ThemeError::PresetNotForPlatform`] where it
/// names another, else the table's preset for `running`
/// (`pipeline::preset_for_version`).
#[cfg_attr(
    not(any(
        all(target_os = "macos", feature = "macos"),
        all(target_os = "windows", feature = "windows")
    )),
    allow(
        dead_code,
        reason = "a generation table's reader: not linked in this build"
    )
)]
fn table_preset(
    table: &crate::pipeline::GenerationTable,
    running: crate::pipeline::OsVersion,
    generation: Option<PresetId>,
) -> Result<PresetId, ThemeError> {
    match generation {
        Some(id)
            if std::iter::once(&table.oldest)
                .chain(table.newer)
                .any(|row| row.preset == id.key()) =>
        {
            Ok(id)
        }
        Some(id) => Err(ThemeError::PresetNotForPlatform(id)),
        None => PresetId::from_key(crate::pipeline::preset_for_version(table, running).0)
            .ok_or(ThemeError::PlatformUnsupported),
    }
}

/// The reader of the Linux desktop `desktop` and its preset, which
/// `generation` may name ([`select`]).
#[cfg(target_os = "linux")]
fn linux_reader(
    desktop: crate::detect::LinuxDesktop,
    generation: Option<PresetId>,
) -> Result<(SystemReader, PresetId), ThemeError> {
    let (reader, preset) = desktop_reader(desktop)?;
    match generation {
        Some(id) if id != preset => Err(ThemeError::PresetNotForPlatform(id)),
        _ => Ok((reader, preset)),
    }
}

/// The reader of the Linux desktop `desktop` and its preset.
#[cfg(target_os = "linux")]
fn desktop_reader(
    desktop: crate::detect::LinuxDesktop,
) -> Result<(SystemReader, PresetId), ThemeError> {
    match desktop {
        crate::detect::LinuxDesktop::Gnome | crate::detect::LinuxDesktop::Budgie => {
            #[cfg(feature = "portal")]
            {
                Ok((SystemReader::Gnome, PresetId::Adwaita))
            }
            #[cfg(not(feature = "portal"))]
            {
                Err(ThemeError::FeatureDisabled)
            }
        }
        crate::detect::LinuxDesktop::Xfce
        | crate::detect::LinuxDesktop::Cinnamon
        | crate::detect::LinuxDesktop::Mate
        | crate::detect::LinuxDesktop::LxQt
        | crate::detect::LinuxDesktop::Hyprland
        | crate::detect::LinuxDesktop::Sway
        | crate::detect::LinuxDesktop::River
        | crate::detect::LinuxDesktop::Niri
        | crate::detect::LinuxDesktop::Wayfire
        | crate::detect::LinuxDesktop::CosmicDe
        | crate::detect::LinuxDesktop::Unknown => {
            #[cfg(feature = "portal")]
            {
                Ok((SystemReader::Portal, PresetId::Adwaita))
            }
            #[cfg(not(feature = "portal"))]
            {
                Err(ThemeError::FeatureDisabled)
            }
        }
        crate::detect::LinuxDesktop::Kde => {
            #[cfg(feature = "kde")]
            {
                Ok((SystemReader::Kde, PresetId::KdeBreeze))
            }
            #[cfg(not(feature = "kde"))]
            {
                Err(ThemeError::FeatureDisabled)
            }
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::detect::LinuxDesktop;

    /// Every Linux desktop with no reader of its own (S16's *Which
    /// reader*).
    const PORTAL_ONLY: [LinuxDesktop; 11] = [
        LinuxDesktop::Xfce,
        LinuxDesktop::Cinnamon,
        LinuxDesktop::Mate,
        LinuxDesktop::LxQt,
        LinuxDesktop::Hyprland,
        LinuxDesktop::Sway,
        LinuxDesktop::River,
        LinuxDesktop::Niri,
        LinuxDesktop::Wayfire,
        LinuxDesktop::CosmicDe,
        LinuxDesktop::Unknown,
    ];

    /// GNOME and Budgie are read by the GNOME reader under `adwaita`, every
    /// desktop with no reader of its own by the portal-only read under
    /// `adwaita`, both with the `portal` feature; KDE by the KDE reader under
    /// `kde-breeze`, with the `kde` feature.
    #[test]
    fn the_desktop_chooses_the_reader_and_its_preset() {
        for desktop in [LinuxDesktop::Gnome, LinuxDesktop::Budgie] {
            #[cfg(feature = "portal")]
            assert_eq!(
                linux_reader(desktop, None),
                Ok((SystemReader::Gnome, PresetId::Adwaita))
            );
            #[cfg(not(feature = "portal"))]
            assert_eq!(
                linux_reader(desktop, None),
                Err(ThemeError::FeatureDisabled)
            );
        }
        for desktop in PORTAL_ONLY {
            #[cfg(feature = "portal")]
            assert_eq!(
                linux_reader(desktop, None),
                Ok((SystemReader::Portal, PresetId::Adwaita)),
                "{desktop:?}"
            );
            #[cfg(not(feature = "portal"))]
            assert_eq!(
                linux_reader(desktop, None),
                Err(ThemeError::FeatureDisabled),
                "{desktop:?}"
            );
        }
        #[cfg(feature = "kde")]
        assert_eq!(
            linux_reader(LinuxDesktop::Kde, None),
            Ok((SystemReader::Kde, PresetId::KdeBreeze))
        );
        #[cfg(not(feature = "kde"))]
        assert_eq!(
            linux_reader(LinuxDesktop::Kde, None),
            Err(ThemeError::FeatureDisabled)
        );
    }

    /// Linux has no generation table: the desktop's preset is its one
    /// generation, and another is not the platform's (R1.5).
    #[cfg(feature = "portal")]
    #[test]
    fn a_generation_is_the_desktops_preset_or_refused() {
        assert_eq!(
            linux_reader(LinuxDesktop::Gnome, Some(PresetId::Adwaita)),
            Ok((SystemReader::Gnome, PresetId::Adwaita))
        );
        assert_eq!(
            linux_reader(LinuxDesktop::Gnome, Some(PresetId::MacosTahoe)),
            Err(ThemeError::PresetNotForPlatform(PresetId::MacosTahoe))
        );
        for desktop in PORTAL_ONLY {
            assert_eq!(
                linux_reader(desktop, Some(PresetId::Adwaita)),
                Ok((SystemReader::Portal, PresetId::Adwaita))
            );
            assert_eq!(
                linux_reader(desktop, Some(PresetId::Material3)),
                Err(ThemeError::PresetNotForPlatform(PresetId::Material3))
            );
        }
    }
}

#[cfg(test)]
mod macos_tests {
    use super::*;
    use crate::pipeline::OsVersion;

    /// With no generation named, the running version's row of the
    /// generation table decides (`pipeline::macos_generation`); every row's
    /// preset is a bundled one.
    #[test]
    fn the_macos_version_chooses_the_preset() {
        assert_eq!(
            macos_preset(OsVersion::new(26, 6, 2), None),
            Ok(PresetId::MacosTahoe)
        );
        assert_eq!(
            macos_preset(OsVersion::new(15, 7, 9), None),
            Ok(PresetId::MacosMonterey)
        );
        let table = &crate::pipeline::MACOS_GENERATIONS;
        for row in std::iter::once(&table.oldest).chain(table.newer) {
            assert!(PresetId::from_key(row.preset).is_some(), "{}", row.preset);
        }
    }

    /// A named generation is taken where it is one of macOS's, whatever
    /// the running version, and refused otherwise (R1.5).
    #[test]
    fn a_generation_is_a_macos_preset_or_refused() {
        let running = OsVersion::new(26, 0, 0);
        assert_eq!(
            macos_preset(running, Some(PresetId::MacosMonterey)),
            Ok(PresetId::MacosMonterey)
        );
        assert_eq!(
            macos_preset(running, Some(PresetId::MacosTahoe)),
            Ok(PresetId::MacosTahoe)
        );
        for id in [PresetId::Adwaita, PresetId::Ios26, PresetId::Material3] {
            assert_eq!(
                macos_preset(running, Some(id)),
                Err(ThemeError::PresetNotForPlatform(id))
            );
        }
    }
}

#[cfg(test)]
mod windows_tests {
    use super::*;
    use crate::pipeline::OsVersion;

    /// With no generation named, every Windows build gets `windows-11`, the
    /// table's one row, Windows 10's included (M2b Step 7).
    #[test]
    fn the_windows_build_chooses_the_preset() {
        for running in [OsVersion::new(10, 0, 26200), OsVersion::new(10, 0, 19045)] {
            assert_eq!(windows_preset(running, None), Ok(PresetId::Windows11));
        }
    }

    /// A named generation is taken where it is Windows', and refused
    /// otherwise (R1.5).
    #[test]
    fn a_generation_is_a_windows_preset_or_refused() {
        let running = OsVersion::new(10, 0, 26200);
        assert_eq!(
            windows_preset(running, Some(PresetId::Windows11)),
            Ok(PresetId::Windows11)
        );
        for id in [PresetId::Adwaita, PresetId::MacosTahoe, PresetId::KdeBreeze] {
            assert_eq!(
                windows_preset(running, Some(id)),
                Err(ThemeError::PresetNotForPlatform(id))
            );
        }
    }
}
