//! Layers read from TOML (S11): `Layer::from_toml`, a preset's file, and the
//! `when` and `declare` readers, walking every key against the model so each
//! unknown key is refused with its TOML path and every problem collected.

mod declare;
pub(crate) mod layer;
mod model;
pub(crate) mod read;
#[cfg(test)]
mod tests;
mod when;
