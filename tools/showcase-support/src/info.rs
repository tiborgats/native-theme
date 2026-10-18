//! A listed element's Widget Info, in the list's form (the header of
//! `docs/showcase-elements.toml`, "The Widget Info format"): the title, a
//! "Theme" row per leaf with its value and how the toolkit applies it, then
//! "Not themeable" where the toolkit draws what the theme states nothing
//! for; and the text Copy copies. Each showcase draws the panel from it in
//! its own toolkit, in the panel's typography.

use native_theme::color::Rgba;
use native_theme::theme::{IconSet, LayoutTheme};

use crate::elements::ShowcaseElement;
use crate::values::leaf_value;

/// One row of the "Theme" section.
#[derive(Clone, Debug, PartialEq)]
pub struct InfoRow {
    /// The leaf as the list writes it.
    pub leaf: String,
    /// Its value ([`crate::values::leaf_value`]).
    pub value: String,
    /// The value's colour, for the row's swatch, where it is one.
    pub colour: Option<Rgba>,
    /// How this toolkit applies the leaf: the muted line under the row.
    pub how: String,
}

/// A listed element's Widget Info.
#[derive(Clone, Debug, PartialEq)]
pub struct ListedInfo {
    /// The element's name and the state shown ([`ShowcaseElement::title`]).
    pub title: String,
    /// A row per leaf of the element, in the list's order.
    pub rows: Vec<InfoRow>,
    /// The "Not themeable" lines, none where the section is not shown.
    pub not_themeable: Vec<String>,
}

impl ListedInfo {
    /// The Widget Info of `element` under the resolved theme's JSON
    /// `resolved` (`values::theme_json`), with the `layout`, the icon set and
    /// the icon theme the leaves the resolved theme does not hold read from
    /// ([`crate::values::leaf_value`]); `route` is the toolkit's "how" line
    /// for a leaf, and `not_themeable` the widget's own lines.
    pub fn new(
        element: &ShowcaseElement,
        resolved: &serde_json::Value,
        layout: &LayoutTheme,
        icon_set: Option<IconSet>,
        icon_theme: Option<&str>,
        route: impl Fn(&str) -> String,
        not_themeable: Vec<String>,
    ) -> Self {
        let rows = element
            .leaves
            .iter()
            .map(|leaf| {
                let value = leaf_value(leaf, resolved, layout, icon_set, icon_theme);
                InfoRow {
                    leaf: leaf.clone(),
                    value: value.text,
                    colour: value.colour,
                    how: route(leaf),
                }
            })
            .collect();
        Self {
            title: element.title(),
            rows,
            not_themeable,
        }
    }

    /// The text Copy copies, the panel's text: the title, a blank line,
    /// `Theme`, then per row `<leaf> <value>` and under it the route indented
    /// two spaces; then, where there are any, a blank line, `Not themeable`
    /// and its lines.
    #[must_use]
    pub fn copy_text(&self) -> String {
        let mut text = format!("{}\n\nTheme\n", self.title);
        for row in &self.rows {
            text.push_str(&format!("{} {}\n  {}\n", row.leaf, row.value, row.how));
        }
        if !self.not_themeable.is_empty() {
            text.push_str("\nNot themeable\n");
            for line in &self.not_themeable {
                text.push_str(line);
                text.push('\n');
            }
        }
        text
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

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

    /// A row per leaf in the list's order, its value, swatch and route; the
    /// title with the state.
    #[test]
    fn the_rows_follow_the_lists_leaves() {
        let json = serde_json::json!({
            "button": {"background_color": "#336699", "border": {"corner_radius": 4.0}},
        });
        let layout = LayoutTheme {
            widget_gap: Some(8.0),
            ..LayoutTheme::default()
        };
        let info = ListedInfo::new(
            &element(
                &[
                    "button.background_color",
                    "button.border.corner_radius_px",
                    "layout.widget_gap_px",
                ],
                &["Normal"],
            ),
            &json,
            &layout,
            None,
            None,
            |leaf| format!("route of {leaf}"),
            Vec::new(),
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
                (
                    "button.background_color",
                    "#336699",
                    "route of button.background_color"
                ),
                (
                    "button.border.corner_radius_px",
                    "4 px",
                    "route of button.border.corner_radius_px"
                ),
                (
                    "layout.widget_gap_px",
                    "8 px",
                    "route of layout.widget_gap_px"
                ),
            ]
        );
        let colours: Vec<bool> = info.rows.iter().map(|row| row.colour.is_some()).collect();
        assert_eq!(colours, [true, false, false]);
    }

    /// Copy's text: the title, a blank line, `Theme`, each row with its
    /// route indented under it, then `Not themeable` and its lines after a
    /// blank line where there are any.
    #[test]
    fn copy_text_holds_the_panels_lines() {
        let json = serde_json::json!({"button": {"background_color": "#336699"}});
        let layout = LayoutTheme::default();
        let route = |leaf: &str| format!("gpui: {leaf}");
        let leaves = ["button.background_color", "layout.widget_gap_px"];
        let notes = vec!["focus ring: upstream's ring".to_string()];
        let info = ListedInfo::new(
            &element(&leaves, &[]),
            &json,
            &layout,
            None,
            None,
            route,
            notes,
        );
        assert_eq!(
            info.copy_text(),
            "Button\n\nTheme\nbutton.background_color #336699\n  gpui: button.background_color\n\
             layout.widget_gap_px not stated\n  gpui: layout.widget_gap_px\n\nNot themeable\n\
             focus ring: upstream's ring\n"
        );
        let bare = ListedInfo::new(
            &element(&leaves, &["Hovered"]),
            &json,
            &layout,
            None,
            None,
            route,
            Vec::new(),
        );
        assert!(bare.copy_text().starts_with("Button · Hovered\n\nTheme\n"));
        assert!(!bare.copy_text().contains("Not themeable"));
        assert!(bare.copy_text().ends_with("  gpui: layout.widget_gap_px\n"));
    }
}
