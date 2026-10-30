//! Rules' selectors and the conditions they hold under (S7), and the
//! references to widgets, parts and variants they name.

use super::key::Key;
use super::widgets::{TypedPart, Variant, WidgetKind};

/// A widget: a typed one, or a keyed component a preset declares (S5.1).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WidgetRef {
    /// A typed widget.
    Typed(WidgetKind),
    /// A keyed component.
    Component(Key),
}

/// A part of a widget: a typed widget's, or a keyed component's (S1).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PartRef {
    /// A typed widget's part, naming its widget.
    Typed(TypedPart),
    /// A keyed component's part.
    Component {
        /// The component.
        key: Key,
        /// Its part.
        part: Key,
    },
}

/// A variant: the catalogue's typed one, or a keyed one a preset declares.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VariantRef {
    /// A typed variant.
    Typed(Variant),
    /// A keyed variant.
    Keyed(Key),
}

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
