//! [`crate::info`] for the 0.8.0 model: a listed element's Widget Info, its
//! values read from the 0.8.0 resolved theme's serialised form
//! ([`crate::next::values::leaf_value`]) and its swatches in
//! `native_theme::next::Color`. The rows and the Copy text are
//! [`crate::info`]'s types.

use native_theme::next::Color;

use crate::elements::ShowcaseElement;
use crate::info::{InfoRow, ListedInfo};
use crate::next::values::{StatedFont, leaf_value};

/// The Widget Info of `element` under the 0.8.0 resolved theme's JSON
/// `resolved` (`next::values::theme_json`), each of its leaves a path of
/// that form and every text style's family as `family` labels what the
/// toolkit draws ([`leaf_value`]); `route` is the toolkit's "how" line for a
/// leaf, and `not_themeable` the widget's own lines. The counterpart of
/// [`ListedInfo::new`], which also takes what the 0.7.2 resolved theme does
/// not hold: the 0.8.0 one holds its layout and icons.
pub fn listed_info(
    element: &ShowcaseElement,
    resolved: &serde_json::Value,
    family: impl Fn(&str, &StatedFont) -> String,
    route: impl Fn(&str) -> String,
    not_themeable: Vec<String>,
) -> ListedInfo<Color> {
    let rows = element
        .leaves
        .iter()
        .map(|leaf| {
            let value = leaf_value(leaf, resolved, &family);
            InfoRow {
                leaf: leaf.clone(),
                value: value.text,
                colour: value.colour,
                how: route(leaf),
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

    use std::str::FromStr as _;

    /// A lookup that names every family as stated.
    fn as_stated(_: &str, font: &StatedFont) -> String {
        font.family.clone()
    }

    fn element(leaves: &[&str], states: &[&str]) -> ShowcaseElement {
        ShowcaseElement {
            id: "basic.buttons.default".into(),
            name: "Button".into(),
            parent: "window".into(),
            leaves: leaves.iter().map(|leaf| (*leaf).to_string()).collect(),
            states: states.iter().map(|state| (*state).to_string()).collect(),
            part: false,
            when: None,
        }
    }

    /// A row per leaf in the list's order, its value, swatch and route, a
    /// ruled part's property read from its base and the layout from the
    /// theme; the title with the state; Copy's text in the list's form.
    #[test]
    fn the_rows_follow_the_lists_leaves() -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::json!({
            "button": {"body": {"exclusive": [], "base": {"fill": "#336699", "corners": 4.0}, "rules": []}},
            "layout": {"widget_gap": 8.0, "section_gap": null},
        });
        let leaves = [
            "button.body.fill",
            "button.body.corners_px",
            "layout.widget_gap_px",
            "layout.section_gap_px",
        ];
        let info = listed_info(
            &element(&leaves, &["Normal"]),
            &json,
            as_stated,
            |leaf| format!("route of {leaf}"),
            vec!["focus ring: upstream's ring".to_string()],
        );
        assert_eq!(info.title, "Button · Normal");
        let rows: Vec<(&str, &str, &str)> = info
            .rows
            .iter()
            .map(|row| (row.leaf.as_str(), row.value.as_str(), row.how.as_str()))
            .collect();
        assert_eq!(
            rows,
            [
                ("button.body.fill", "#336699", "route of button.body.fill"),
                (
                    "button.body.corners_px",
                    "4 px",
                    "route of button.body.corners_px"
                ),
                (
                    "layout.widget_gap_px",
                    "8 px",
                    "route of layout.widget_gap_px"
                ),
                (
                    "layout.section_gap_px",
                    crate::values::NOT_STATED,
                    "route of layout.section_gap_px"
                ),
            ]
        );
        let colours: Vec<Option<Color>> = info.rows.iter().map(|row| row.colour).collect();
        assert_eq!(
            colours,
            [Some(Color::from_str("#336699")?), None, None, None]
        );
        assert_eq!(
            info.copy_text(),
            "Button · Normal\n\nTheme\nbutton.body.fill #336699\n  route of button.body.fill\n\
             button.body.corners_px 4 px\n  route of button.body.corners_px\n\
             layout.widget_gap_px 8 px\n  route of layout.widget_gap_px\n\
             layout.section_gap_px not stated\n  route of layout.section_gap_px\n\n\
             Not themeable\nfocus ring: upstream's ring\n"
        );
        Ok(())
    }

    /// Under kde-breeze, each row holds the value and swatch
    /// [`leaf_value`] gives its leaf, the family as the lookup labels it.
    #[test]
    fn the_rows_hold_kde_breezes_values() -> Result<(), String> {
        use native_theme::next::{Preset, Theme, ThemeContext};

        let resolved = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
        let json = crate::next::values::theme_json(&resolved)?;
        let label = |_: &str, font: &StatedFont| format!("{} (drawn)", font.family);
        let leaves = [
            "button.body.fill",
            "button.label",
            "typography.body.family",
            "layout.section_gap_px",
            "icons.theme",
        ];
        let info = listed_info(
            &element(&leaves, &[]),
            &json,
            label,
            str::to_string,
            Vec::new(),
        );
        assert_eq!(info.rows.len(), leaves.len());
        for (row, leaf) in info.rows.iter().zip(leaves) {
            let value = leaf_value(leaf, &json, &label);
            assert_eq!(
                (row.leaf.as_str(), &row.value, row.colour, row.how.as_str()),
                (leaf, &value.text, value.colour, leaf)
            );
        }
        assert_eq!(
            info.rows.get(2).map(|row| row.value.clone()),
            Some(format!("{} (drawn)", resolved.typography.body.family))
        );
        Ok(())
    }
}
