//! Values (S2.2) and the maps keyed by scheme and by surface set.

use super::expr::Expr;
use super::roles::SurfaceSet;
use super::selector::Scheme;

/// A field's value: stated, derived, sized by content or not drawn.
///
/// A field with no value in any layer is missing, and fails validation.
#[derive(Clone, Debug, PartialEq)]
pub enum Value<T> {
    /// A stated value.
    Stated(T),
    /// A derivation, evaluated where used.
    Expr(Expr),
    /// The platform sizes by content (lengths and line heights). On a minimum
    /// size (`min_width`, `min_height`): no minimum beyond the content; on a
    /// maximum size (`max_width`, `max_height`): no maximum, the content
    /// decides. Never an operand of a call.
    Auto,
    /// The platform does not draw this; resolves to none. Allowed only where
    /// the field is marked `may_be_absent`.
    Absent,
}

/// At most one value per [`Scheme`]; empty by default.
///
/// A palette states only its schemes, and a value no layer states for a
/// scheme is missing.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SchemeMap<T> {
    /// The light scheme's value.
    pub light: Option<T>,
    /// The dark scheme's value.
    pub dark: Option<T>,
}

impl<T> SchemeMap<T> {
    /// A value for each scheme.
    #[must_use]
    pub fn new(light: T, dark: T) -> Self {
        Self {
            light: Some(light),
            dark: Some(dark),
        }
    }

    /// A value for one scheme only.
    #[must_use]
    pub fn one(scheme: Scheme, value: T) -> Self {
        let mut map = Self::default();
        *map.slot(scheme) = Some(value);
        map
    }

    /// The value for `scheme`, if one is stated.
    #[must_use]
    pub fn get(&self, scheme: Scheme) -> Option<&T> {
        match scheme {
            Scheme::Light => self.light.as_ref(),
            Scheme::Dark => self.dark.as_ref(),
        }
    }

    fn slot(&mut self, scheme: Scheme) -> &mut Option<T> {
        match scheme {
            Scheme::Light => &mut self.light,
            Scheme::Dark => &mut self.dark,
        }
    }
}

impl<T> Default for SchemeMap<T> {
    fn default() -> Self {
        Self {
            light: None,
            dark: None,
        }
    }
}

/// At most one value per [`SurfaceSet`], kept in [`SurfaceSet`]'s order, so
/// two maps holding the same values are equal however they were filled; empty
/// by default.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SurfaceMap<T> {
    entries: Vec<(SurfaceSet, T)>,
}

impl<T> SurfaceMap<T> {
    /// The value for `set`, if one is held.
    #[must_use]
    pub fn get(&self, set: SurfaceSet) -> Option<&T> {
        let index = self.entries.binary_search_by_key(&set, |(held, _)| *held);
        index
            .ok()
            .and_then(|index| self.entries.get(index))
            .map(|(_, value)| value)
    }

    /// Holds `value` for `set`, returning the value it replaces.
    pub fn insert(&mut self, set: SurfaceSet, value: T) -> Option<T> {
        match self.entries.binary_search_by_key(&set, |(held, _)| *held) {
            Ok(index) => self
                .entries
                .get_mut(index)
                .map(|(_, held)| std::mem::replace(held, value)),
            Err(index) => {
                self.entries.insert(index, (set, value));
                None
            }
        }
    }

    /// The sets and their values, in [`SurfaceSet`]'s order.
    pub fn iter(&self) -> impl Iterator<Item = (SurfaceSet, &T)> {
        self.entries.iter().map(|(set, value)| (*set, value))
    }
}

impl<T> Default for SurfaceMap<T> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheme_map_holds_one_value_per_scheme() {
        let both = SchemeMap::new(1, 2);
        assert_eq!(
            (both.get(Scheme::Light), both.get(Scheme::Dark)),
            (Some(&1), Some(&2))
        );
        let dark = SchemeMap::one(Scheme::Dark, 3);
        assert_eq!(
            (dark.get(Scheme::Light), dark.get(Scheme::Dark)),
            (None, Some(&3))
        );
        assert_eq!(SchemeMap::<u8>::default().get(Scheme::Light), None);
    }

    #[test]
    fn surface_map_replaces_by_key_in_set_order() {
        let mut map = SurfaceMap::default();
        assert_eq!(map.insert(SurfaceSet::View, 1), None);
        assert_eq!(map.insert(SurfaceSet::Window, 2), None);
        assert_eq!(map.insert(SurfaceSet::View, 3), Some(1));
        assert_eq!(map.get(SurfaceSet::View), Some(&3));
        assert_eq!(map.get(SurfaceSet::Button), None);
        let order: Vec<_> = map.iter().collect();
        assert_eq!(order, [(SurfaceSet::Window, &2), (SurfaceSet::View, &3)]);
        let mut other = SurfaceMap::default();
        other.insert(SurfaceSet::Window, 2);
        other.insert(SurfaceSet::View, 3);
        assert_eq!(map, other);
    }
}
