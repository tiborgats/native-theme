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
//! The readers linked so far: GNOME's (with the `portal` feature, on GNOME
//! and Budgie), and the portal-only read of the Linux desktops with no
//! reader of their own (S16's *Which reader*, ADD-A135). The KDE, macOS and
//! Windows readers return with their presets (the 0.8.0 plan's M1b); until
//! then `from_system` reports their platforms as
//! [`ThemeError::PlatformUnsupported`].

use crate::error::{ReaderFailure, ThemeError};
use crate::layer::Layer;
use crate::presets::PresetId;
use crate::resolve::ReaderState;

/// What one read gave: the state and the layer, and the source that failed
/// where another stood in for it (S9.1, S9.3: the status's `Fallback`).
#[cfg_attr(
    not(all(target_os = "linux", feature = "portal")),
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
}

impl SystemReader {
    /// Reads the system, the reader's own platform's part under `own`.
    #[cfg_attr(
        not(all(target_os = "linux", feature = "portal")),
        allow(unused_variables, reason = "no reader is linked in this build")
    )]
    pub(crate) async fn read(self, own: PresetId) -> Result<Reading, ReaderFailure> {
        match self {
            #[cfg(all(target_os = "linux", feature = "portal"))]
            SystemReader::Gnome => Ok(crate::gnome::read(own).await),
            #[cfg(all(target_os = "linux", feature = "portal"))]
            SystemReader::Portal => Ok(crate::gnome::portal::read().await),
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
/// [`ThemeError::FeatureDisabled`] without the `portal` feature. Linux has
/// no generation table: the one preset of the desktop is its only
/// generation, so `generation` is accepted where it is that preset and is
/// [`ThemeError::PresetNotForPlatform`] otherwise. KDE, macOS and Windows
/// are [`ThemeError::PlatformUnsupported`] in this build: no reader reads
/// them until their readers and presets return.
pub(crate) async fn select(
    generation: Option<PresetId>,
) -> Result<(SystemReader, PresetId), ThemeError> {
    #[cfg(target_os = "linux")]
    {
        linux_reader(crate::detect::refined_linux_desktop().await, generation)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = generation;
        Err(ThemeError::PlatformUnsupported)
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
        crate::detect::LinuxDesktop::Kde => Err(ThemeError::PlatformUnsupported),
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::detect::LinuxDesktop;

    /// Every Linux desktop but KDE's, with no reader of its own in this
    /// build (S16's *Which reader*).
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
    /// `adwaita`, both with the `portal` feature; KDE has no reader in this
    /// build.
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
        assert_eq!(
            linux_reader(LinuxDesktop::Kde, None),
            Err(ThemeError::PlatformUnsupported)
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
