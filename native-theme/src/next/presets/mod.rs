//! The platform builders, one per preset (spec §6): each resolves a theme
//! from the context and the resolve inputs: `kde-breeze`'s, `adwaita`'s,
//! `windows-11`'s, `macos-monterey`'s, `macos-tahoe`'s, `material-3`'s and
//! `ios-13`'s, with the scaffold the builders share.

pub(crate) mod adwaita;
pub(crate) mod helpers;
pub(crate) mod ios;
pub(crate) mod kde;
pub(crate) mod macos;
pub(crate) mod material;
pub(crate) mod windows;
