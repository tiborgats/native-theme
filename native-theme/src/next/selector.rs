//! Rules' selectors and the conditions they hold under (S7).

/// A colour scheme.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Scheme {
    /// The light scheme.
    Light,
    /// The dark scheme.
    Dark,
}

impl Scheme {
    /// Every scheme, in declaration order.
    pub const ALL: [Scheme; 2] = [Scheme::Light, Scheme::Dark];

    /// Every scheme's name, in [`Scheme::ALL`]'s order.
    pub const NAMES: [&'static str; 2] = ["light", "dark"];

    /// The scheme's name in TOML and in expressions, its variant in
    /// snake_case.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Scheme::Light => "light",
            Scheme::Dark => "dark",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheme_names_match_variants() {
        assert_eq!(Scheme::ALL.map(Scheme::name), Scheme::NAMES);
    }
}
