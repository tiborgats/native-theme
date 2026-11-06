//! S9.4's `DynamicType` curve: Apple's published Dynamic Type table, each
//! text style's size and leading per content size category (TS-2, X-DT-1),
//! from the HIG's tables (`gen_dynamic_type`, generated from
//! `hig-typography@2026-02-09` by `scripts/generate_dynamic_type.py`).
//!
//! A row is a text role's: the eleven roles the table has a row for, the
//! others none. The sizes and leadings are in points, converted to logical px
//! by the font DPI (S9.4).

use super::gen_dynamic_type::{DEFAULT, ROLES, TABLES};
use crate::blocks::text::TextRole;
use crate::resolve::ContentSizeCategory;

/// The default category, the one the table names (the HIG's `Large
/// (default)`): the category a preset states its text styles at, and a
/// reader's or stand-in's `None` (S8, S9.4).
pub const DEFAULT_CATEGORY: ContentSizeCategory = DEFAULT;

/// A text role's row of the table at a category, in logical px.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct DynamicTypeSize {
    /// The size (*Size (points)*).
    pub font_size: f32,
    /// The leading, the table's line height (*Leading (points)*).
    pub line_height: f32,
}

/// `role`'s size and leading at `category`, in logical px, the table's
/// points converted by `font_dpi` (S9.4). `None` for a role the table has no
/// row for, and where the DPI is not finite or not above zero.
#[must_use]
pub fn dynamic_type(
    role: TextRole,
    category: ContentSizeCategory,
    font_dpi: f32,
) -> Option<DynamicTypeSize> {
    if !font_dpi.is_finite() || font_dpi <= 0.0 {
        return None;
    }
    let row = ROLES.iter().position(|held| *held == role)?;
    let (_, table) = TABLES.iter().find(|(held, _)| *held == category)?;
    let &(size, leading) = table.get(row)?;
    Some(DynamicTypeSize {
        font_size: size * font_dpi / 72.0,
        line_height: leading * font_dpi / 72.0,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::{DEFAULT_CATEGORY, DynamicTypeSize, ROLES, TABLES, dynamic_type};
    use crate::blocks::text::TextRole;
    use crate::resolve::ContentSizeCategory;

    fn size(font_size: f32, line_height: f32) -> Option<DynamicTypeSize> {
        Some(DynamicTypeSize {
            font_size,
            line_height,
        })
    }

    /// The generated table holds rows copied by hand from the HIG
    /// (`hig-typography@2026-02-09`'s `typography.json`), each by its JSON
    /// pointer below `#/primaryContentSections/0/content/` and its cells as
    /// the page writes them (style, weight, size, leading, emphasized
    /// weight): a check of the generator's parse independent of it.
    #[test]
    fn hand_copied_rows_are_in_the_tables() {
        let rows = [
            // `66/tabs/0/content/1/rows/1`: Large Title Regular 31 38 Bold
            (
                TextRole::LargeTitle,
                ContentSizeCategory::ExtraSmall,
                31.0,
                38.0,
            ),
            // `66/tabs/3/content/1/rows/6`: Body Regular 17 22 Semibold
            (TextRole::Body, ContentSizeCategory::Large, 17.0, 22.0),
            // `66/tabs/3/content/1/rows/10`: Caption 1 Regular 12 16 Semibold
            (TextRole::Caption, ContentSizeCategory::Large, 12.0, 16.0),
            // `66/tabs/6/content/1/rows/8`: Subhead Regular 21 28 Semibold
            (
                TextRole::Subheadline,
                ContentSizeCategory::ExtraExtraExtraLarge,
                21.0,
                28.0,
            ),
            // `68/tabs/0/content/1/rows/6`: Body Regular 28 34 Semibold
            (
                TextRole::Body,
                ContentSizeCategory::AccessibilityMedium,
                28.0,
                34.0,
            ),
            // `68/tabs/4/content/1/rows/11`: Caption 2 Regular 40 48 Semibold
            (
                TextRole::Caption2,
                ContentSizeCategory::AccessibilityExtraExtraExtraLarge,
                40.0,
                48.0,
            ),
        ];
        for (role, category, font_size, line_height) in rows {
            assert_eq!(
                dynamic_type(role, category, 72.0),
                size(font_size, line_height),
                "{role:?} at {category:?}"
            );
        }
    }

    /// The default category is `Large`, the page's `Large (default)`.
    #[test]
    fn the_default_is_large() {
        assert_eq!(DEFAULT_CATEGORY, ContentSizeCategory::Large);
    }

    /// Every category has a table, and every table a row per role.
    #[test]
    fn every_category_has_a_table() {
        assert_eq!(TABLES.len(), 12);
        assert_eq!(ROLES.len(), 11);
    }

    /// The points convert by the DPI: 17 pt at 144 DPI is 34 px.
    #[test]
    fn points_convert_by_the_dpi() {
        assert_eq!(
            dynamic_type(TextRole::Body, ContentSizeCategory::Large, 144.0),
            size(34.0, 44.0)
        );
    }

    /// A role without a row, and a DPI outside the domain, give none.
    #[test]
    fn no_row_no_size() {
        assert_eq!(
            dynamic_type(TextRole::DisplayLarge, ContentSizeCategory::Large, 72.0),
            None
        );
        assert_eq!(
            dynamic_type(TextRole::Body, ContentSizeCategory::Large, 0.0),
            None
        );
    }
}
