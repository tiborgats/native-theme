//! The side panel's pickers' rows: the preset picker's, which the command
//! palette lists too, and the icon-theme picker's.

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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

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
