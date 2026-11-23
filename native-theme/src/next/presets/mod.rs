//! The platform builders, one per preset (spec §6): each resolves a theme
//! from the context and the resolve inputs. So far `kde-breeze`'s,
//! `adwaita`'s, `windows-11`'s, `macos-monterey`'s, `macos-tahoe`'s and
//! `material-3`'s, with the scaffold the builders share.

pub(crate) mod adwaita;
pub(crate) mod helpers;
pub(crate) mod kde;
pub(crate) mod macos;
pub(crate) mod material;
pub(crate) mod windows;
