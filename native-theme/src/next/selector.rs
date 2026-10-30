//! Rules' selectors and the conditions they hold under (S7).

/// A colour scheme.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Scheme {
    /// The light scheme.
    Light,
    /// The dark scheme.
    Dark,
}
