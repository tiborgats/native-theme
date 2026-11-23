//! The platform builders, one per preset (spec §6): each resolves a theme
//! from the context and the resolve inputs. So far `kde-breeze`'s, with the
//! scaffold the builders share.

pub(crate) mod helpers;
pub(crate) mod kde;
