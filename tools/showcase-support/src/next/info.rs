//! [`crate::info`] for the 0.8.0 model: a listed element's Widget Info, a
//! row per home of the element (the list's `homes`), its value read from the
//! 0.8.0 resolved theme ([`crate::next::values::home_value`]) and its swatch
//! in `native_theme::next::Color`. The rows and the Copy text are
//! [`crate::info`]'s types.

use native_theme::next::{Color, ResolvedTheme};

use crate::elements::ShowcaseElement;
use crate::info::{InfoRow, ListedInfo};
use crate::next::values::{StatedFont, home_value};

/// The Widget Info of `element` under the 0.8.0 resolved theme `theme`,
/// whose serialised form is `json` (`next::values::theme_json`): a row per
/// home of the element, `<home> <value>`, every text style's family as
/// `family` labels what the toolkit draws ([`home_value`]), and a home of no
/// known name shown as the error that says so; `route` is the toolkit's
/// "how" line for a home, and `not_themeable` the widget's own lines. The
/// counterpart of [`ListedInfo::new`], which also takes what the 0.7.2
/// resolved theme does not hold: the 0.8.0 one holds its layout and icons.
pub fn listed_info(
    element: &ShowcaseElement,
    theme: &ResolvedTheme,
    json: &serde_json::Value,
    family: impl Fn(&str, &StatedFont) -> String,
    route: impl Fn(&str) -> String,
    not_themeable: Vec<String>,
) -> ListedInfo<Color> {
    let rows = element
        .homes
        .iter()
        .map(|home| {
            let (value, colour) = match home_value(home, theme, json, &family) {
                Ok(value) => (value.text, value.colour),
                Err(error) => (format!("error: {error}"), None),
            };
            InfoRow {
                leaf: home.clone(),
                value,
                colour,
                how: route(home),
            }
        })
        .collect();
    ListedInfo {
        title: element.title(),
        rows,
        not_themeable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::{BTreeMap, BTreeSet};
    use std::str::FromStr as _;

    use native_theme::next::{Preset, Theme, ThemeContext};

    use crate::next::values::{STATED_AT_REST, theme_json};
    use crate::values::NOT_STATED;

    type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// A lookup that names every family as stated.
    fn as_stated(_: &str, font: &StatedFont) -> String {
        font.family.clone()
    }

    fn element(homes: &[&str], states: &[&str]) -> ShowcaseElement {
        ShowcaseElement {
            id: "basic.buttons.default".into(),
            name: "Button".into(),
            parent: "window".into(),
            leaves: Vec::new(),
            homes: homes.iter().map(|home| (*home).to_string()).collect(),
            states: states.iter().map(|state| (*state).to_string()).collect(),
            part: false,
            when: None,
        }
    }

    /// kde-breeze, resolved for the light scheme's default context, and its
    /// JSON.
    fn kde_breeze_light() -> Result<(ResolvedTheme, serde_json::Value)> {
        let theme = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
        let json = theme_json(&theme)?;
        Ok((theme, json))
    }

    /// A row per home in the list's order, its value, swatch and route, a
    /// home of no known name shown as its error; the title with the state;
    /// Copy's text in the list's form.
    #[test]
    fn the_rows_follow_the_lists_homes() -> Result {
        let (theme, json) = kde_breeze_light()?;
        let homes = [
            "button.body(NONE).fill",
            "button.body(NONE).corners",
            "layout.widget_gap",
            "button.no_such_part(NONE).fill",
        ];
        let info = listed_info(
            &element(&homes, &["Normal"]),
            &theme,
            &json,
            as_stated,
            |home| format!("route of {home}"),
            vec!["focus ring: upstream's ring".to_string()],
        );
        assert_eq!(info.title, "Button · Normal");
        let rows: Vec<(&str, &str, &str)> = info
            .rows
            .iter()
            .map(|row| (row.leaf.as_str(), row.value.as_str(), row.how.as_str()))
            .collect();
        let unknown = "error: `button.no_such_part(NONE).fill` is no home of the model";
        assert_eq!(
            rows,
            [
                (
                    "button.body(NONE).fill",
                    "#fcfcfc",
                    "route of button.body(NONE).fill"
                ),
                (
                    "button.body(NONE).corners",
                    "5 px",
                    "route of button.body(NONE).corners"
                ),
                ("layout.widget_gap", "6 px", "route of layout.widget_gap"),
                (
                    "button.no_such_part(NONE).fill",
                    unknown,
                    "route of button.no_such_part(NONE).fill"
                ),
            ]
        );
        let colours: Vec<Option<Color>> = info.rows.iter().map(|row| row.colour).collect();
        assert_eq!(
            colours,
            [Some(Color::from_str("#fcfcfc")?), None, None, None]
        );
        assert_eq!(
            info.copy_text(),
            format!(
                "Button · Normal\n\nTheme\nbutton.body(NONE).fill #fcfcfc\n  route of button.body(NONE).fill\n\
                 button.body(NONE).corners 5 px\n  route of button.body(NONE).corners\n\
                 layout.widget_gap 6 px\n  route of layout.widget_gap\n\
                 button.no_such_part(NONE).fill {unknown}\n  route of button.no_such_part(NONE).fill\n\n\
                 Not themeable\nfocus ring: upstream's ring\n"
            )
        );
        Ok(())
    }

    /// The seven presets the scaffolding compares, by their 0.7.2 name, and
    /// the font DPI 0.7.2 resolves each in, as native-theme's model parity
    /// (`src/next/parity.rs`) does: `ResolutionContext::from_system`'s 72 for
    /// the macOS presets and `ios-13`, `ResolutionContext::for_tests`'s 96
    /// for the rest.
    const PRESETS: [(&str, Preset, f32); 7] = [
        ("kde-breeze", Preset::KdeBreeze, 96.0),
        ("adwaita", Preset::Adwaita, 96.0),
        ("windows-11", Preset::Windows11, 96.0),
        ("macos-tahoe", Preset::MacosTahoe, 72.0),
        ("macos-monterey", Preset::MacosMonterey, 72.0),
        ("material-3", Preset::Material3, 96.0),
        ("ios-13", Preset::Ios13, 72.0),
    ];

    /// Every home of every element of the list evaluates without error on
    /// each preset and scheme the scaffolding compares.
    #[test]
    fn every_listed_home_evaluates() -> Result {
        let list = crate::elements::showcase_elements()?;
        let mut errors = BTreeSet::new();
        for (_, preset, _) in PRESETS {
            for context in [ThemeContext::light(), ThemeContext::dark()] {
                let theme = Theme::preset(preset).resolve(&context);
                let json = theme_json(&theme)?;
                for home in list.iter().flat_map(|element| &element.homes) {
                    if let Err(error) = home_value(home, &theme, &json, &as_stated) {
                        errors.insert(format!("{preset:?}: {error}"));
                    }
                }
            }
        }
        assert!(errors.is_empty(), "homes that do not evaluate: {errors:#?}");
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Scaffolding of the 0.8.0 work, deleted at the cut with
    // native-theme/tests/next_homes.toml (docs/v0.8.0-plan.md, M4, step 2).
    // -----------------------------------------------------------------------

    /// The table of homes, whose rows tie a 0.7.2 leaf to its homes.
    const HOMES_TABLE: &str = include_str!("../../../../native-theme/tests/next_homes.toml");

    /// The table's rows by key.
    fn rows() -> Result<BTreeMap<String, toml::Table>> {
        let table: toml::Table = toml::from_str(HOMES_TABLE)?;
        let rows = table
            .get("row")
            .and_then(toml::Value::as_array)
            .ok_or("next_homes.toml has no rows")?;
        rows.iter()
            .map(|row| {
                let row = row.as_table().ok_or("a row is no table")?;
                let key = row
                    .get("key")
                    .and_then(toml::Value::as_str)
                    .ok_or("a row has no key")?;
                Ok((key.to_string(), row.clone()))
            })
            .collect()
    }

    /// The strings of `row`'s `key`, none where it has none.
    fn strings(row: &toml::Table, key: &str) -> Vec<String> {
        row.get(key)
            .and_then(toml::Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(toml::Value::as_str)
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// The table's key of a 0.7.2 leaf as the list spells it: the leaf
    /// itself, a hand row's key for the two the `Resolved` wrapper holds, or
    /// the leaf without its `_px` and `_ms` units and with a border's
    /// `padding_<side>` as `padding.<side>`.
    fn row_key(leaf: &str, rows: &BTreeMap<String, toml::Table>) -> Option<String> {
        if rows.contains_key(leaf) {
            return Some(leaf.to_string());
        }
        let hand = match leaf {
            "theme_variant.icon_set" => Some("Resolved.icon_set"),
            "defaults.icon_theme" => Some("Resolved.icon_theme"),
            _ => None,
        };
        if let Some(key) = hand {
            return Some(key.to_string());
        }
        let segments: Vec<&str> = leaf.split('.').collect();
        let mut key = Vec::new();
        for (index, segment) in segments.iter().enumerate() {
            let bare = segment
                .strip_suffix("_px")
                .or_else(|| segment.strip_suffix("_ms"))
                .unwrap_or(segment);
            let after_border = index > 0 && segments.get(index - 1) == Some(&"border");
            match bare.strip_prefix("padding_") {
                Some(side) if after_border => key.extend(["padding", side]),
                _ => key.push(bare),
            }
        }
        let key = key.join(".");
        rows.contains_key(&key).then_some(key)
    }

    /// A leaf's rows and homes: the rows it reads, the homes they give (a
    /// whole font's, each text part once, from its property rows), and the
    /// homes they give a preset of its own (`homes_by_preset`).
    struct Candidates {
        keys: Vec<String>,
        homes: Vec<String>,
        by_preset: Vec<String>,
    }

    /// The text part a property home is of (`button.label(NONE).size_px` is
    /// `button.label(NONE)`).
    fn strip_property(home: &str) -> &str {
        home.rsplit_once('.').map_or(home, |(part, _)| part)
    }

    /// `leaf`'s [`Candidates`], `None` where no row reads it.
    fn candidates(leaf: &str, rows: &BTreeMap<String, toml::Table>) -> Option<Candidates> {
        if let Some(key) = row_key(leaf, rows) {
            let row = rows.get(&key)?;
            let by_preset = row
                .get("homes_by_preset")
                .and_then(toml::Value::as_table)
                .map(|presets| {
                    presets
                        .values()
                        .filter_map(toml::Value::as_array)
                        .flatten()
                        .filter_map(toml::Value::as_str)
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default();
            return Some(Candidates {
                homes: strings(row, "homes"),
                keys: vec![key],
                by_preset,
            });
        }
        let properties: &[&str] = if leaf.starts_with("text_scale.") {
            &["size", "weight", "line_height"]
        } else {
            &["family", "size", "defined_size", "weight", "style"]
        };
        let keys: Vec<String> = properties.iter().map(|p| format!("{leaf}.{p}")).collect();
        let mut homes: Vec<String> = Vec::new();
        for key in &keys {
            for home in strings(rows.get(key)?, "homes") {
                let whole = strip_property(&home).to_string();
                if !homes.contains(&whole) {
                    homes.push(whole);
                }
            }
        }
        Some(Candidates {
            keys,
            homes,
            by_preset: Vec::new(),
        })
    }

    /// Whether `row` is plain on `preset`: no listed difference there, and
    /// none of the keys that change how the table's comparator reads a row.
    fn plain(row: &toml::Table, preset: &str) -> bool {
        const SPECIAL: [&str; 6] = [
            "if_null",
            "only_if_null",
            "null_reference",
            "null_means",
            "homes_by_preset",
            "note",
        ];
        let differs = strings(row, "difference_presets")
            .iter()
            .any(|listed| listed == preset);
        let unstated_if_unset = row
            .get("unstated_if_unset")
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        !differs && !unstated_if_unset && SPECIAL.iter().all(|key| !row.contains_key(*key))
    }

    /// Every home of an element is a home of one of its leaves' rows (a
    /// preset's own included), and every leaf has one of its row's homes
    /// among the element's homes.
    #[test]
    fn the_homes_are_the_leaves_rows_homes() -> Result {
        let rows = rows()?;
        let mut faults = Vec::new();
        for element in crate::elements::showcase_elements()? {
            let mut allowed = BTreeSet::new();
            for leaf in &element.leaves {
                let Some(c) = candidates(leaf, &rows) else {
                    faults.push(format!("{}: no row reads `{leaf}`", element.id));
                    continue;
                };
                if !c.homes.iter().any(|home| element.homes.contains(home)) {
                    faults.push(format!(
                        "{}: none of `{leaf}`'s homes {:?} is listed",
                        element.id, c.homes
                    ));
                }
                allowed.extend(c.homes.into_iter().chain(c.by_preset));
            }
            for home in &element.homes {
                if !allowed.contains(home) {
                    faults.push(format!("{}: `{home}` is no home of its leaves", element.id));
                }
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
        Ok(())
    }

    /// On the seven presets, light and dark, each plain row's 0.8.0 text
    /// (its home's) equals the 0.7.2 text (its leaf's, under the same
    /// preset and scheme), a row marked as stated at rest where its text
    /// without the marker does; a home the 0.8.0 model leaves unstated or
    /// states as none where 0.7.2 shows a value, and a marked row where
    /// 0.7.2 shows "not stated" (a change of form), are reported, not
    /// failed. Each (preset, scheme, home, leaf) is counted once.
    #[test]
    fn plain_rows_read_as_in_0_7_2() -> Result {
        use native_theme::ResolutionContext;
        use native_theme::theme::{ColorMode, Theme as OldTheme};

        let rows = rows()?;
        let list = crate::elements::showcase_elements()?;
        let old_stated =
            |_: &str, font: &native_theme::theme::ResolvedFontSpec| font.family.to_string();
        let mut seen = BTreeSet::new();
        let (mut equal, mut not_plain) = (0_usize, 0_usize);
        let mut differ = Vec::new();
        let mut unstated = Vec::new();
        let mut marked = 0_usize;
        let mut new_form = Vec::new();
        for (name, preset, font_dpi) in PRESETS {
            for (mode, context) in [
                (ColorMode::Light, ThemeContext::light()),
                (ColorMode::Dark, ThemeContext::dark()),
            ] {
                let old_theme = OldTheme::preset(name)?;
                let old = old_theme.resolve_with(
                    mode,
                    &ResolutionContext {
                        font_dpi,
                        ..ResolutionContext::for_tests()
                    },
                )?;
                let old_json = crate::values::theme_json(&old.variant)?;
                let theme = Theme::preset(preset).resolve(&context);
                let json = theme_json(&theme)?;
                for element in &list {
                    for home in &element.homes {
                        let leaves: Vec<(&String, Candidates)> = element
                            .leaves
                            .iter()
                            .filter_map(|leaf| Some((leaf, candidates(leaf, &rows)?)))
                            .filter(|(_, c)| c.homes.contains(home) || c.by_preset.contains(home))
                            .collect();
                        let is_plain = leaves.iter().all(|(_, c)| {
                            c.keys
                                .iter()
                                .all(|key| rows.get(key).is_some_and(|row| plain(row, name)))
                        });
                        for (leaf, _) in &leaves {
                            if !seen.insert((
                                name,
                                format!("{mode:?}"),
                                home.clone(),
                                (*leaf).clone(),
                            )) {
                                continue;
                            }
                            if !is_plain {
                                not_plain += 1;
                                continue;
                            }
                            let new = home_value(home, &theme, &json, &as_stated)?.text;
                            let was = crate::values::leaf_value(
                                leaf,
                                &old_json,
                                &old_theme.layout,
                                Some(old.icon_set),
                                old.icon_theme.as_deref(),
                                &old_stated,
                            )
                            .text;
                            let line = format!("{name} {mode:?} {home} `{new}` / {leaf} `{was}`");
                            let unmarked = new.strip_suffix(&format!(" {STATED_AT_REST}"));
                            if unmarked.is_some() {
                                marked += 1;
                            }
                            match unmarked {
                                Some(text) if text == was => equal += 1,
                                Some(_) if was == NOT_STATED => new_form.push(line),
                                Some(_) => differ.push(line),
                                None if new == was => equal += 1,
                                None if new.contains(NOT_STATED) || new == "none" => {
                                    unstated.push(line);
                                }
                                None => differ.push(line),
                            }
                        }
                    }
                }
            }
        }
        println!(
            "plain rows equal: {equal}; differ: {}; marked stated at rest: {marked}; stated at rest where 0.7.2 not stated: {}; new \"not stated\" or \"none\": {}; not plain: {not_plain}",
            differ.len(),
            new_form.len(),
            unstated.len()
        );
        for line in &new_form {
            println!("stated at rest, 0.7.2 not stated: {line}");
        }
        for line in &unstated {
            println!("new not stated: {line}");
        }
        assert!(differ.is_empty(), "plain rows that differ: {differ:#?}");
        Ok(())
    }
}
