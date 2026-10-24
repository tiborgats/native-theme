//! The side panel's pickers' rows: the preset picker's, which the command
//! palette lists too, and the icon-theme picker's; and a table row's height
//! under the OS text scale, which the iced and egui showcases' tables take
//! as the gpui connector's `DataTable` rows do.

use native_theme::AccessibilityPreferences;
use native_theme::icons::{IconSetChoice, default_icon_choice};
use native_theme::theme::{IconSet, Theme};

/// The name the preset picker, the command palette and the status bar give
/// `default`, the desktop's own theme, built on the platform preset
/// `default_preset`: `default (<preset>)`.
#[must_use]
pub fn default_label(default_preset: &str) -> String {
    format!("default ({default_preset})")
}

/// One row of the preset picker: a theme, by the key `--theme` takes and the
/// name the row shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresetRow {
    pub key: String,
    pub display_name: String,
}

/// The presets a showcase offers, in the order it offers them: the
/// desktop's own theme, keyed `default` and labelled with `default_preset`,
/// the preset it builds on ([`default_label`]), then the presets meant for
/// this platform (`Theme::list_presets_for_platform`), which are what
/// `--theme` takes ([`crate::cli::theme_key`]).
#[must_use]
pub fn preset_rows(default_preset: &str) -> Vec<PresetRow> {
    let default = PresetRow {
        key: "default".to_string(),
        display_name: default_label(default_preset),
    };
    std::iter::once(default)
        .chain(
            Theme::list_presets_for_platform()
                .iter()
                .map(|info| PresetRow {
                    key: info.key.to_string(),
                    display_name: info.display_name.to_string(),
                }),
        )
        .collect()
}

/// The icon-theme picker's rows for a theme of `icon_set` naming
/// `icon_theme`: its `default` row, where the theme names an icon theme that
/// is available (`default_icon_choice` returns `Default`), then `system`,
/// each of the `installed` freedesktop themes, Lucide and Material. A
/// showcase with a row of its own adds it (gpui inserts its built-in icons
/// after the installed themes).
#[must_use]
pub fn icon_rows(
    icon_set: IconSet,
    icon_theme: Option<&str>,
    installed: &[String],
) -> Vec<IconSetChoice> {
    let default = match default_icon_choice(icon_set, icon_theme) {
        choice @ IconSetChoice::Default(_) => Some(choice),
        _ => None,
    };
    default
        .into_iter()
        .chain([IconSetChoice::System])
        .chain(
            installed
                .iter()
                .map(|name| IconSetChoice::Freedesktop(name.clone())),
        )
        .chain([IconSetChoice::Lucide, IconSetChoice::Material])
        .collect()
}

/// Why the system icon theme is unavailable, `error` being why its
/// detection (`native_theme::detect::system_icon_theme`) failed:
/// `System icon theme: unavailable (<reason>)`. The icon-theme picker's
/// `system` row then reads `system (unavailable)` (`IconSetChoice`'s
/// `Display`): a control's label is short, and the reason, which can run to
/// a reader's whole error, is shown where there is room for it: the
/// picker's hover text and a line on the Icons page in each showcase, and
/// the gpui showcase's Widget Info for the picker.
#[must_use]
pub fn icon_theme_unavailable(error: &native_theme::Error) -> String {
    format!("System icon theme: unavailable ({error})")
}

/// A table row's height, or its header row's, where the theme states one
/// (`list.row_height`, `list.header_height`), under the OS text-scaling
/// factor of `prefs`: the gpui connector's rule for its `DataTable` rows
/// (`connectors/native-theme-gpui/src/geometry.rs`, `table_row_height`,
/// which `data_table_size` and `list_header_height` take). At a factor of 1
/// or less `stated`, verbatim; above 1 at least the line box of the row's
/// font at its scaled size (`prefs.scaled(font_size)`) and `line_height`
/// (`defaults.line_height`), rounded up, plus `padding_y`, the row's top and
/// bottom padding together, and `rule`, the width of any line drawn inside
/// the row's height. So a stated height holds the text the OS scales up, as
/// gpui's does. The model states no such growth, and the iced and egui
/// connectors style no table: each showcase passes its own table's padding
/// and rule.
#[must_use]
pub fn scaled_height(
    stated: f32,
    prefs: &AccessibilityPreferences,
    font_size: f32,
    line_height: f32,
    padding_y: f32,
    rule: f32,
) -> f32 {
    if prefs.text_scale_factor() <= 1.0 {
        return stated;
    }
    let line = (prefs.scaled(font_size) * line_height).ceil();
    stated.max(line + padding_y + rule)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn scaled_by(factor: f32) -> AccessibilityPreferences {
        let mut prefs = AccessibilityPreferences::default();
        prefs.text_scaling_factor = factor;
        prefs
    }

    /// At a factor of 1, below it, or one the preferences ignore (not
    /// finite and positive), a stated height is kept, even where its text
    /// would not fit it.
    #[test]
    fn a_stated_height_is_kept_at_a_factor_of_one_or_less() {
        for factor in [1.0, 0.5, 0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert_eq!(
                scaled_height(20.0, &scaled_by(factor), 30.0, 1.2, 8.0, 1.0),
                20.0,
                "factor {factor}"
            );
        }
    }

    /// Above 1 the height is at least the scaled line box, rounded up, the
    /// padding and the rule: 13 × 1.5 × 1.2 = 23.4, so 24 + 6 + 1.
    #[test]
    fn above_one_a_stated_height_grows_to_hold_its_text() {
        let prefs = scaled_by(1.5);
        assert_eq!(scaled_height(24.0, &prefs, 13.0, 1.2, 6.0, 1.0), 31.0);
        assert_eq!(scaled_height(24.0, &prefs, 13.0, 1.2, 6.0, 0.0), 30.0);
    }

    /// Above 1 a stated height that already holds the scaled text stays.
    #[test]
    fn above_one_a_tall_enough_height_stays() {
        assert_eq!(
            scaled_height(56.0, &scaled_by(1.5), 14.0, 1.2, 0.0, 1.0),
            56.0
        );
    }

    /// `default` first, labelled with the preset it builds on, then this
    /// platform's presets in their order, each a key `--theme` takes.
    #[test]
    fn the_preset_rows_start_with_the_desktops_own() {
        let rows = preset_rows("kde-breeze");
        let Some((first, presets)) = rows.split_first() else {
            panic!("no preset row");
        };
        assert_eq!(
            first,
            &PresetRow {
                key: "default".into(),
                display_name: "default (kde-breeze)".into(),
            }
        );
        let offered: Vec<(&str, &str)> = Theme::list_presets_for_platform()
            .iter()
            .map(|info| (info.key, info.display_name))
            .collect();
        let listed: Vec<(&str, &str)> = presets
            .iter()
            .map(|row| (row.key.as_str(), row.display_name.as_str()))
            .collect();
        assert_eq!(listed, offered);
        for row in &rows {
            assert!(crate::cli::theme_key(&row.key).is_ok(), "{}", row.key);
        }
    }

    /// A failed detection's reason, in whole.
    #[test]
    fn a_failed_detection_gives_its_reason_in_whole() {
        let failed = native_theme::Error::PlatformUnsupported {
            platform: "the test's failing detection",
        };
        assert_eq!(
            icon_theme_unavailable(&failed),
            "System icon theme: unavailable (platform not supported: the test's failing detection)"
        );
    }

    /// The bundled sets' rows close the list, after `system` and the
    /// installed themes; a `default` row leads it only where the theme names
    /// an available icon theme.
    #[test]
    fn the_icon_rows_are_in_the_pickers_order() {
        let installed = vec!["breeze".to_string(), "Adwaita".to_string()];
        let rows = icon_rows(IconSet::Material, None, &installed);
        assert_eq!(
            rows,
            [
                IconSetChoice::System,
                IconSetChoice::Freedesktop("breeze".into()),
                IconSetChoice::Freedesktop("Adwaita".into()),
                IconSetChoice::Lucide,
                IconSetChoice::Material,
            ]
        );
        // A bundled set is always available, so its `default` row leads.
        let rows = icon_rows(IconSet::Material, Some("material"), &[]);
        assert_eq!(
            rows.first(),
            Some(&IconSetChoice::Default("material".into()))
        );
        assert_eq!(
            rows.iter().rev().take(2).collect::<Vec<_>>(),
            [&IconSetChoice::Material, &IconSetChoice::Lucide]
        );
    }
}
