//! Apple's published Dynamic Type table: each text style's size and leading
//! per content size category, from the HIG's *iOS, iPadOS Dynamic Type
//! sizes* and *iOS, iPadOS larger accessibility type sizes* (the pages
//! `hig-typography@2026-02-09` and `hig-typography@2025-08-15`, whose tables
//! are equal). Taken from `v0.8.0-rc1` at 78ee0efd: its `dynamic_type.rs`
//! and the table its `gen_dynamic_type.rs` holds (generated there by
//! `scripts/generate_dynamic_type.py`, which is not on this branch), in one
//! file, with its vector table `tests/vectors/dynamic_type.tsv`. Carried with
//! three changes: the text styles and the categories are this module's own
//! enums, the eleven styles the table has a row for and UIKit's twelve
//! categories, as nothing else in `next` names them; the function is
//! `pub(crate)`; the vector test reads the table as a unit test here.
//!
//! The sizes and leadings are in points, converted to logical px by the font
//! DPI.

/// A text style of the table, in its rows' order (the *Style* column).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TextRole {
    /// Large Title.
    LargeTitle,
    /// Title 1.
    Title1,
    /// Title 2.
    Title2,
    /// Title 3.
    Title3,
    /// Headline.
    Headline,
    /// Body.
    Body,
    /// Callout.
    Callout,
    /// Subhead.
    Subheadline,
    /// Footnote.
    Footnote,
    /// Caption 1.
    Caption,
    /// Caption 2.
    Caption2,
}

/// UIKit's `UIContentSizeCategory`, smallest first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContentSizeCategory {
    /// Extra small.
    ExtraSmall,
    /// Small.
    Small,
    /// Medium.
    Medium,
    /// Large, the default.
    Large,
    /// Extra large.
    ExtraLarge,
    /// Extra extra large.
    ExtraExtraLarge,
    /// Extra extra extra large.
    ExtraExtraExtraLarge,
    /// Accessibility medium.
    AccessibilityMedium,
    /// Accessibility large.
    AccessibilityLarge,
    /// Accessibility extra large.
    AccessibilityExtraLarge,
    /// Accessibility extra extra large.
    AccessibilityExtraExtraLarge,
    /// Accessibility extra extra extra large.
    AccessibilityExtraExtraExtraLarge,
}

/// The default category, the tab the pages mark *(default)*: the category a
/// preset states its text styles at.
pub(crate) const DEFAULT_CATEGORY: ContentSizeCategory = ContentSizeCategory::Large;

/// A text style's row of the table at a category, in logical px.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DynamicTypeSize {
    /// The size (*Size (points)*).
    pub(crate) font_size: f32,
    /// The leading, the table's line height (*Leading (points)*).
    pub(crate) line_height: f32,
}

/// `role`'s size and leading at `category`, in logical px, the table's
/// points converted by `font_dpi`. `None` where the DPI is not finite or not
/// above zero.
pub(crate) fn dynamic_type(
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

/// The text styles of the tables' rows, in their order (the *Style* column:
/// Large Title, Title 1, Title 2, Title 3, Headline, Body, Callout,
/// Subhead, Footnote, Caption 1, Caption 2).
const ROLES: [TextRole; 11] = [
    TextRole::LargeTitle,
    TextRole::Title1,
    TextRole::Title2,
    TextRole::Title3,
    TextRole::Headline,
    TextRole::Body,
    TextRole::Callout,
    TextRole::Subheadline,
    TextRole::Footnote,
    TextRole::Caption,
    TextRole::Caption2,
];

/// Every category, smallest first.
pub(crate) const CATEGORIES: [ContentSizeCategory; 12] = [
    ContentSizeCategory::ExtraSmall,
    ContentSizeCategory::Small,
    ContentSizeCategory::Medium,
    ContentSizeCategory::Large,
    ContentSizeCategory::ExtraLarge,
    ContentSizeCategory::ExtraExtraLarge,
    ContentSizeCategory::ExtraExtraExtraLarge,
    ContentSizeCategory::AccessibilityMedium,
    ContentSizeCategory::AccessibilityLarge,
    ContentSizeCategory::AccessibilityExtraLarge,
    ContentSizeCategory::AccessibilityExtraExtraLarge,
    ContentSizeCategory::AccessibilityExtraExtraExtraLarge,
];

/// Each category's table, smallest first: each row `(size, leading)` in
/// points, in [`ROLES`]' order. Each table's pointer is into
/// `hig-typography@2026-02-09`'s `typography.json`; the same table of
/// `hig-typography@2025-08-15` is at `content/65` for the first seven and at
/// `content/67` for the five accessibility ones, at the same tab.
const TABLES: [(ContentSizeCategory, [(f32, f32); 11]); 12] = [
    // `#/primaryContentSections/0/content/66/tabs/0/content/1`
    (
        ContentSizeCategory::ExtraSmall,
        [
            (31.0, 38.0),
            (25.0, 31.0),
            (19.0, 24.0),
            (17.0, 22.0),
            (14.0, 19.0),
            (14.0, 19.0),
            (13.0, 18.0),
            (12.0, 16.0),
            (12.0, 16.0),
            (11.0, 13.0),
            (11.0, 13.0),
        ],
    ),
    // `#/primaryContentSections/0/content/66/tabs/1/content/1`
    (
        ContentSizeCategory::Small,
        [
            (32.0, 39.0),
            (26.0, 32.0),
            (20.0, 25.0),
            (18.0, 23.0),
            (15.0, 20.0),
            (15.0, 20.0),
            (14.0, 19.0),
            (13.0, 18.0),
            (12.0, 16.0),
            (11.0, 13.0),
            (11.0, 13.0),
        ],
    ),
    // `#/primaryContentSections/0/content/66/tabs/2/content/1`
    (
        ContentSizeCategory::Medium,
        [
            (33.0, 40.0),
            (27.0, 33.0),
            (21.0, 26.0),
            (19.0, 24.0),
            (16.0, 21.0),
            (16.0, 21.0),
            (15.0, 20.0),
            (14.0, 19.0),
            (12.0, 16.0),
            (11.0, 13.0),
            (11.0, 13.0),
        ],
    ),
    // `#/primaryContentSections/0/content/66/tabs/3/content/1`
    (
        ContentSizeCategory::Large,
        [
            (34.0, 41.0),
            (28.0, 34.0),
            (22.0, 28.0),
            (20.0, 25.0),
            (17.0, 22.0),
            (17.0, 22.0),
            (16.0, 21.0),
            (15.0, 20.0),
            (13.0, 18.0),
            (12.0, 16.0),
            (11.0, 13.0),
        ],
    ),
    // `#/primaryContentSections/0/content/66/tabs/4/content/1`
    (
        ContentSizeCategory::ExtraLarge,
        [
            (36.0, 43.0),
            (30.0, 37.0),
            (24.0, 30.0),
            (22.0, 28.0),
            (19.0, 24.0),
            (19.0, 24.0),
            (18.0, 23.0),
            (17.0, 22.0),
            (15.0, 20.0),
            (14.0, 19.0),
            (13.0, 18.0),
        ],
    ),
    // `#/primaryContentSections/0/content/66/tabs/5/content/1`
    (
        ContentSizeCategory::ExtraExtraLarge,
        [
            (38.0, 46.0),
            (32.0, 39.0),
            (26.0, 32.0),
            (24.0, 30.0),
            (21.0, 26.0),
            (21.0, 26.0),
            (20.0, 25.0),
            (19.0, 24.0),
            (17.0, 22.0),
            (16.0, 21.0),
            (15.0, 20.0),
        ],
    ),
    // `#/primaryContentSections/0/content/66/tabs/6/content/1`
    (
        ContentSizeCategory::ExtraExtraExtraLarge,
        [
            (40.0, 48.0),
            (34.0, 41.0),
            (28.0, 34.0),
            (26.0, 32.0),
            (23.0, 29.0),
            (23.0, 29.0),
            (22.0, 28.0),
            (21.0, 28.0),
            (19.0, 24.0),
            (18.0, 23.0),
            (17.0, 22.0),
        ],
    ),
    // `#/primaryContentSections/0/content/68/tabs/0/content/1`
    (
        ContentSizeCategory::AccessibilityMedium,
        [
            (44.0, 52.0),
            (38.0, 46.0),
            (34.0, 41.0),
            (31.0, 38.0),
            (28.0, 34.0),
            (28.0, 34.0),
            (26.0, 32.0),
            (25.0, 31.0),
            (23.0, 29.0),
            (22.0, 28.0),
            (20.0, 25.0),
        ],
    ),
    // `#/primaryContentSections/0/content/68/tabs/1/content/1`
    (
        ContentSizeCategory::AccessibilityLarge,
        [
            (48.0, 57.0),
            (43.0, 51.0),
            (39.0, 47.0),
            (37.0, 44.0),
            (33.0, 40.0),
            (33.0, 40.0),
            (32.0, 39.0),
            (30.0, 37.0),
            (27.0, 33.0),
            (26.0, 32.0),
            (24.0, 30.0),
        ],
    ),
    // `#/primaryContentSections/0/content/68/tabs/2/content/1`
    (
        ContentSizeCategory::AccessibilityExtraLarge,
        [
            (52.0, 61.0),
            (48.0, 57.0),
            (44.0, 52.0),
            (43.0, 51.0),
            (40.0, 48.0),
            (40.0, 48.0),
            (38.0, 46.0),
            (36.0, 43.0),
            (33.0, 40.0),
            (32.0, 39.0),
            (29.0, 35.0),
        ],
    ),
    // `#/primaryContentSections/0/content/68/tabs/3/content/1`
    (
        ContentSizeCategory::AccessibilityExtraExtraLarge,
        [
            (56.0, 66.0),
            (53.0, 62.0),
            (50.0, 59.0),
            (49.0, 58.0),
            (47.0, 56.0),
            (47.0, 56.0),
            (44.0, 52.0),
            (42.0, 50.0),
            (38.0, 46.0),
            (37.0, 44.0),
            (34.0, 41.0),
        ],
    ),
    // `#/primaryContentSections/0/content/68/tabs/4/content/1`
    (
        ContentSizeCategory::AccessibilityExtraExtraExtraLarge,
        [
            (60.0, 70.0),
            (58.0, 68.0),
            (56.0, 66.0),
            (55.0, 65.0),
            (53.0, 62.0),
            (53.0, 62.0),
            (51.0, 60.0),
            (49.0, 58.0),
            (44.0, 52.0),
            (43.0, 51.0),
            (40.0, 48.0),
        ],
    ),
];

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::super::vectors::{TestResult, number, rows};
    use super::{
        CATEGORIES, ContentSizeCategory, DEFAULT_CATEGORY, DynamicTypeSize, ROLES, TABLES,
        TextRole, dynamic_type,
    };

    fn size(font_size: f32, line_height: f32) -> Option<DynamicTypeSize> {
        Some(DynamicTypeSize {
            font_size,
            line_height,
        })
    }

    /// The table holds rows copied by hand from the HIG
    /// (`hig-typography@2026-02-09`'s `typography.json`), each by its JSON
    /// pointer below `#/primaryContentSections/0/content/` and its cells as
    /// the page writes them (style, weight, size, leading, emphasized
    /// weight): rc1's check of the generator's parse, independent of it.
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

    /// The default category is `Large`, the page's `Large (default)`, and
    /// the tables are in the categories' order.
    #[test]
    fn the_default_is_large_and_the_tables_are_in_order() {
        assert_eq!(DEFAULT_CATEGORY, ContentSizeCategory::Large);
        assert!(TABLES.iter().map(|(c, _)| *c).eq(CATEGORIES));
        assert_eq!(ROLES.len(), 11);
    }

    /// The points convert by the DPI: 17 pt at 144 DPI is 34 px; a DPI
    /// outside the domain gives none.
    #[test]
    fn points_convert_by_the_dpi() {
        assert_eq!(
            dynamic_type(TextRole::Body, ContentSizeCategory::Large, 144.0),
            size(34.0, 44.0)
        );
        assert_eq!(
            dynamic_type(TextRole::Body, ContentSizeCategory::Large, 0.0),
            None
        );
    }

    fn role(name: &str) -> Result<TextRole, String> {
        let role = match name {
            "large_title" => TextRole::LargeTitle,
            "title1" => TextRole::Title1,
            "title2" => TextRole::Title2,
            "title3" => TextRole::Title3,
            "headline" => TextRole::Headline,
            "body" => TextRole::Body,
            "callout" => TextRole::Callout,
            "subheadline" => TextRole::Subheadline,
            "footnote" => TextRole::Footnote,
            "caption" => TextRole::Caption,
            "caption2" => TextRole::Caption2,
            _ => return Err(format!("no text role `{name}`")),
        };
        Ok(role)
    }

    fn category(name: &str) -> Result<ContentSizeCategory, String> {
        let names = [
            "extra_small",
            "small",
            "medium",
            "large",
            "extra_large",
            "extra_extra_large",
            "extra_extra_extra_large",
            "accessibility_medium",
            "accessibility_large",
            "accessibility_extra_large",
            "accessibility_extra_extra_large",
            "accessibility_extra_extra_extra_large",
        ];
        names
            .iter()
            .zip(CATEGORIES)
            .find_map(|(held, category)| (*held == name).then_some(category))
            .ok_or_else(|| format!("no category `{name}`"))
    }

    /// A length cell of the table, `<number>px`.
    fn px(cell: &str) -> Result<f32, Box<dyn std::error::Error>> {
        let value = cell
            .strip_suffix("px")
            .ok_or_else(|| format!("`{cell}` is not in px"))?;
        Ok(number(value)?)
    }

    /// Every row of `tests/vectors/dynamic_type.tsv`, the twelve tables
    /// written from the pages by rc1's generator, is the function's output,
    /// and the vector table has a row per style and category.
    #[test]
    fn vectors_dynamic_type() -> TestResult {
        let table = include_str!("../../../tests/vectors/dynamic_type.tsv");
        let rows = rows(table)?;
        assert_eq!(rows.len(), ROLES.len() * CATEGORIES.len());
        for row in rows {
            let got = dynamic_type(
                role(row.get("role")?)?,
                category(row.get("category")?)?,
                number(row.get("font_dpi")?)?,
            );
            assert_eq!(
                got,
                size(px(row.get("font_size")?)?, px(row.get("line_height")?)?),
                "{row:?}"
            );
        }
        Ok(())
    }
}
