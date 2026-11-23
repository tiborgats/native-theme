//! [`crate::rows`] for the 0.8.0 model: a table row's height under the text
//! scale of the resolved theme's `ThemeContext`. The pickers' rows read no
//! resolved theme (the icon set is the type both models share) and are
//! [`crate::rows`]'s.

/// A table row's height, or its header row's, where the theme states one
/// (`list.row(..).min_size[1]`, `list.header(..).min_size[1]`), under the
/// text scale `text_scale` (`context.text_scale`): the gpui connector's rule
/// for its `DataTable` rows, as [`crate::rows::scaled_height`] gives it. At
/// a scale of 1 or less, or one that is not finite, `stated`, verbatim;
/// above 1 at least the line box of `font_size`, the row's text size as the
/// 0.8.0 theme resolves it (`size_px`, the text scale already applied), at
/// `line_height` (`typography.line_height_multiplier`), rounded up, plus
/// `padding_y`, the row's top and bottom padding together, and `rule`, the
/// width of any line drawn inside the row's height.
#[must_use]
pub fn scaled_height(
    stated: f32,
    text_scale: f32,
    font_size: f32,
    line_height: f32,
    padding_y: f32,
    rule: f32,
) -> f32 {
    if text_scale.is_finite() && text_scale > 1.0 {
        let line = (font_size * line_height).ceil();
        stated.max(line + padding_y + rule)
    } else {
        stated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use native_theme::AccessibilityPreferences;

    /// At a scale of 1, below it, or one that is not finite, a stated
    /// height is kept, even where its text would not fit it.
    #[test]
    fn a_stated_height_is_kept_at_a_scale_of_one_or_less() {
        for scale in [1.0, 0.5, 0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert_eq!(
                scaled_height(20.0, scale, 30.0, 1.2, 8.0, 1.0),
                20.0,
                "scale {scale}"
            );
        }
    }

    /// Above 1 the height is at least the line box of the resolved size,
    /// rounded up, the padding and the rule: 19.5 × 1.2 = 23.4, so 24 + 6 +
    /// 1; a height that already holds it stays.
    #[test]
    fn above_one_a_stated_height_grows_to_hold_its_text() {
        assert_eq!(scaled_height(24.0, 1.5, 19.5, 1.2, 6.0, 1.0), 31.0);
        assert_eq!(scaled_height(24.0, 1.5, 19.5, 1.2, 6.0, 0.0), 30.0);
        assert_eq!(scaled_height(56.0, 1.5, 21.0, 1.2, 0.0, 1.0), 56.0);
    }

    /// The height the 0.7.2 rule gives for an unscaled size under a factor
    /// is the one this gives for the size the 0.8.0 theme resolves under
    /// that scale.
    #[test]
    fn the_height_is_the_old_rules_at_the_resolved_size() {
        for (factor, size) in [(1.0, 13.0), (1.5, 13.0), (2.0, 14.0), (1.25, 10.0)] {
            let mut prefs = AccessibilityPreferences::default();
            prefs.text_scaling_factor = factor;
            assert_eq!(
                scaled_height(24.0, factor, prefs.scaled(size), 1.2, 6.0, 1.0),
                crate::rows::scaled_height(24.0, &prefs, size, 1.2, 6.0, 1.0),
                "factor {factor}, size {size}"
            );
        }
    }
}
