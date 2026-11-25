//! A colour stated at a home of the 0.8.0 model by a `ThemePatch`: the TOML
//! the iced showcase's route tests and the iced connector's colour-home walk
//! (`connectors/native-theme-iced/src/contract/homes.rs`) both patch a
//! resolved theme with, written in one place so the two state a home alike.

/// Every state flag of the 0.8.0 model by its name in a theme file
/// (`WidgetState`'s serde form, `native-theme/src/next/serial.rs`).
pub const STATE_NAMES: [&str; 19] = [
    "hover",
    "pressed",
    "focused",
    "focus_within",
    "focus_visible",
    "unfocused_view",
    "disabled",
    "checked",
    "selected",
    "indeterminate",
    "open",
    "default",
    "invalid",
    "read_only",
    "visited",
    "dragged",
    "drop_target",
    "scrolled",
    "hover_within",
];

/// The colours a ruled widget holds outside its parts, `<widget>.<field>`,
/// which a patch states in the widget's table (`native-theme/src/next/patch.rs`,
/// the widget's `set` fields).
pub const WIDGET_COLOURS: &[&str] = &["list.grid_color"];

/// The TOML of a patch that states `colour` at `home`, a colour home of the
/// 0.8.0 model, the property `fill`, `stroke.color` or, of a text part,
/// `color`: a colour role, a widget's own colour ([`WIDGET_COLOURS`]) in
/// the widget's table, a static widget's part, or a rule of the home's
/// states appended to a ruled widget's (for `NONE`, a rule that forbids
/// every state, so it answers the query at rest alone and leaves the other
/// states' rules as the preset orders them; spec §9, step 3, `not_states`),
/// of the variant or the rows the matcher after them names, `;variant=` or
/// `;alternate=`, with or without a space after the `;`
/// (`button.body(NONE; variant=Primary).fill`,
/// `list.row(NONE;alternate=true).fill`). A stroke is stated 1 px wide in
/// the colour, so a home whose stroke the theme leaves unstated draws it.
/// `None` for any other home.
#[must_use]
pub fn home_patch_toml(home: &str, colour: &str) -> Option<String> {
    if let Some(role) = home.strip_prefix("colors.") {
        return Some(match role.split_once('.') {
            Some((surface, slot)) => format!("[colors.{surface}]\n{slot} = \"{colour}\"\n"),
            None => format!("[colors]\n{role} = \"{colour}\"\n"),
        });
    }
    if WIDGET_COLOURS.contains(&home) {
        let (widget, field) = home.split_once('.')?;
        return Some(format!("[widgets.{widget}]\n{field} = \"{colour}\"\n"));
    }
    let (widget, rest) = home.split_once('.')?;
    let (part, states, property) = match rest.split_once('(') {
        Some((part, rest)) => {
            let (states, property) = rest.split_once(").")?;
            (part, Some(states), property)
        }
        None => match rest.split_once('.') {
            Some((part, property)) => (part, None, property),
            // A static widget's own colour (`separator.color`).
            None => return Some(format!("[widgets.{widget}]\n{rest} = \"{colour}\"\n")),
        },
    };
    let value = match property {
        "fill" | "color" => format!("{part}.{property} = \"{colour}\""),
        "stroke.color" => format!("{part}.stroke = {{ width = 1.0, color = \"{colour}\" }}"),
        _ => return None,
    };
    Some(match states {
        None => format!("[widgets.{widget}]\n{value}\n"),
        Some(query) => {
            let (states, matcher) = match query.split_once(';') {
                Some((states, matcher)) => match matcher.trim().split_once('=')? {
                    ("variant", variant) => (
                        states,
                        format!("variant = \"{}\"\n", variant.to_lowercase()),
                    ),
                    ("alternate", alternate @ ("true" | "false")) => {
                        (states, format!("alternate = {alternate}\n"))
                    }
                    _ => return None,
                },
                None => (query, String::new()),
            };
            let names: Vec<String> = states
                .split('|')
                .filter(|state| *state != "NONE")
                .map(|state| format!("\"{}\"", state.to_lowercase()))
                .collect();
            let states = if names.is_empty() {
                let every: Vec<String> = STATE_NAMES
                    .iter()
                    .map(|name| format!("\"{name}\""))
                    .collect();
                format!("not_states = [{}]\n", every.join(", "))
            } else {
                format!("states = [{}]\n", names.join(", "))
            };
            format!("[[widgets.{widget}.rules]]\n{matcher}{states}{value}\n")
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use native_theme::next::{ButtonQuery, Color, Paint, Preset, Theme, ThemeContext, ThemePatch};

    /// A variant written with a space after the `;`, as the element list
    /// and the export write `button.body(NONE; variant=Primary).fill`, is
    /// the same patch as one written without it, and states the primary
    /// button's fill on kde-breeze light.
    #[test]
    fn a_spaced_matcher_is_the_same_patch() -> Result<(), String> {
        let spaced = home_patch_toml("button.body(NONE; variant=Primary).fill", "#13579b")
            .ok_or("the spaced variant has no patch")?;
        let tight = home_patch_toml("button.body(NONE;variant=Primary).fill", "#13579b")
            .ok_or("the variant has no patch")?;
        assert_eq!(spaced, tight);
        let patch = ThemePatch::from_toml(&spaced).map_err(|e| format!("{e:?}"))?;
        let mut r = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
        r.apply_patch(&patch);
        let want: Color = "#13579b".parse().map_err(|e| format!("{e:?}"))?;
        assert_eq!(
            r.button.body(ButtonQuery::primary()).fill,
            Paint::Solid(want)
        );
        Ok(())
    }

    /// A home of no colour property, and a matcher of no variant or rows,
    /// have no patch.
    #[test]
    fn a_home_with_no_patch_is_refused() {
        assert!(home_patch_toml("button.body(NONE).corners", "#13579b").is_none());
        assert!(home_patch_toml("button.body(NONE; shape=Pill).fill", "#13579b").is_none());
    }
}
