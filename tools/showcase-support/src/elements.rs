//! The elements the three showcases share, read from
//! `docs/showcase-elements.toml`: the list's reader, which its header names.
//! The gpui and iced showcases read the list through it; until egui moves
//! onto it (M9c T1), it reads the list with a copy of its own. Widget
//! Info shows an element's name, state and leaves from it, and each
//! showcase's layout dump records where it drew each of them. The list's
//! header also records the order the three list their pages in
//! ([`page_order`]).

use std::sync::OnceLock;

use crate::Kind;

/// One element of `docs/showcase-elements.toml`: every field of the schema
/// but `clip` and the sample points, the comparator's, which a showcase
/// ignores.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShowcaseElement {
    /// Stable, dotted, lower case; the layout dump's key.
    pub id: String,
    /// What Widget Info shows as its title.
    pub name: String,
    /// The element its position is measured against; `window` for the
    /// content area's origin. The comparator's.
    pub parent: String,
    /// The `docs/property-registry.toml` paths it draws, in Widget Info
    /// order.
    pub leaves: Vec<String>,
    /// The state or variant the page shows.
    pub states: Vec<String>,
    /// A part of a control: dumped and compared, never a Widget Info target
    /// of its own.
    pub part: bool,
    /// On screen only while this holds (an open menu, a hover).
    pub when: Option<String>,
}

impl ShowcaseElement {
    /// The Widget Info title: the name, then ` · ` and the state or variant
    /// shown, where the list names one.
    #[must_use]
    pub fn title(&self) -> String {
        match self.states.first() {
            Some(state) => format!("{} · {state}", self.name),
            None => self.name.clone(),
        }
    }
}

/// The list, as the crate was built with it.
const SHOWCASE_ELEMENTS: &str = include_str!("../../../docs/showcase-elements.toml");

/// The list's elements, in its order, or why it does not parse.
pub fn showcase_elements() -> Result<Vec<ShowcaseElement>, String> {
    parse(SHOWCASE_ELEMENTS)
}

/// The elements of `text`, a list in `docs/showcase-elements.toml`'s schema.
fn parse(text: &str) -> Result<Vec<ShowcaseElement>, String> {
    let table: toml::Table = toml::from_str(text).map_err(|e| e.to_string())?;
    let rows = table
        .get("element")
        .and_then(toml::Value::as_array)
        .ok_or("docs/showcase-elements.toml: no [[element]] entries")?;
    rows.iter()
        .map(|row| {
            let text = |key: &str| {
                row.get(key)
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
                    .ok_or_else(|| format!("an element has no `{key}`"))
            };
            let list = |key: &str| {
                row.get(key)
                    .and_then(toml::Value::as_array)
                    .ok_or_else(|| format!("an element has no `{key}`"))?
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .map(str::to_owned)
                            .ok_or_else(|| format!("`{key}` holds a value that is not a string"))
                    })
                    .collect::<Result<Vec<_>, _>>()
            };
            Ok(ShowcaseElement {
                id: text("id")?,
                name: text("name")?,
                parent: text("parent")?,
                leaves: list("leaves")?,
                states: list("states")?,
                part: row
                    .get("part")
                    .and_then(toml::Value::as_bool)
                    .unwrap_or(false),
                when: row
                    .get("when")
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
            })
        })
        .collect()
}

/// The list, parsed once. A list that does not parse is reported once on
/// stderr and taken as empty: Widget Info then falls back to each widget's
/// own info, and the dump holds nothing, which the comparator reports.
/// The test `the_element_list_parses` holds the committed list to parsing.
pub fn elements() -> &'static [ShowcaseElement] {
    static LIST: OnceLock<Vec<ShowcaseElement>> = OnceLock::new();
    LIST.get_or_init(|| {
        showcase_elements().unwrap_or_else(|error| {
            eprintln!("docs/showcase-elements.toml: {error}");
            Vec::new()
        })
    })
}

/// The element of the list with `id`.
pub fn element(id: &str) -> Option<&'static ShowcaseElement> {
    elements().iter().find(|e| e.id == id)
}

/// The tab labels `kind`'s showcase gives its pages, in the order the
/// three list them: its column of the counterpart table in the list's
/// header (*The pages' counterparts*), less the rows it has no page for, or
/// why the table does not read. Each showcase's test holds its page list to
/// it.
pub fn page_order(kind: Kind) -> Result<Vec<&'static str>, String> {
    pages_of(SHOWCASE_ELEMENTS, kind)
}

/// The cells of `line`, a row of a table in a comment (`#   | a | b |`), or
/// `None` where it is no such row.
fn table_cells(line: &str) -> Option<Vec<&str>> {
    let row = line.strip_prefix('#')?.trim();
    let inner = row.strip_prefix('|')?.strip_suffix('|')?;
    Some(inner.split('|').map(str::trim).collect())
}

/// `kind`'s column of the counterpart table in `text`, a list in
/// `docs/showcase-elements.toml`'s schema.
fn pages_of(text: &str, kind: Kind) -> Result<Vec<&str>, String> {
    const HEAD: [&str; 4] = ["Counterpart", "gpui", "iced", "egui"];
    let mut lines = text.lines();
    lines
        .by_ref()
        .find(|line| table_cells(line).is_some_and(|cells| cells == HEAD))
        .ok_or("docs/showcase-elements.toml: no counterpart table in the header")?;
    let column = HEAD
        .iter()
        .position(|name| *name == kind.name())
        .ok_or_else(|| format!("the counterpart table has no {} column", kind.name()))?;
    let rule = lines.next().and_then(table_cells).is_some_and(|cells| {
        cells
            .iter()
            .all(|cell| !cell.is_empty() && cell.trim_matches('-').is_empty())
    });
    if !rule {
        return Err("the counterpart table's head is not followed by its rule".to_string());
    }
    let mut pages = Vec::new();
    for cells in lines.map_while(table_cells) {
        if cells.len() != HEAD.len() {
            return Err(format!(
                "a counterpart row has {} cells, not {}",
                cells.len(),
                HEAD.len()
            ));
        }
        match cells.get(column) {
            Some(&"-") | None => {}
            Some(page) => pages.push(*page),
        }
    }
    if pages.is_empty() {
        return Err(format!(
            "the counterpart table lists no {} page",
            kind.name()
        ));
    }
    Ok(pages)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// The list parses, as the showcases read it, and is read once.
    #[test]
    fn the_element_list_parses() {
        let list = showcase_elements().expect("docs/showcase-elements.toml parses");
        assert!(
            !list.is_empty(),
            "docs/showcase-elements.toml lists nothing"
        );
        assert_eq!(elements(), list.as_slice());
        let first = list.first().unwrap();
        assert_eq!(element(&first.id), Some(elements().first().unwrap()));
        assert_eq!(element("no.such.element"), None);
    }

    /// Every field is read; `part` defaults to false and `when` to none; the
    /// title adds the first state where there is one.
    #[test]
    fn an_element_reads_every_field() {
        let list = parse(
            r#"
            [[element]]
            id = "basic.checkboxes.checked"
            name = "Checkbox"
            parent = "basic.checkboxes.unchecked"
            leaves = ["checkbox.font", "checkbox.checked_background"]
            states = ["Checked", "Hovered"]
            part = true
            when = "a widget is hovered"
            sample = { fill = [0.5, 0.5] }

            [[element]]
            id = "chrome.toolbar"
            name = "Toolbar"
            parent = "window"
            leaves = []
            states = []
            "#,
        )
        .unwrap();
        assert_eq!(
            list,
            [
                ShowcaseElement {
                    id: "basic.checkboxes.checked".into(),
                    name: "Checkbox".into(),
                    parent: "basic.checkboxes.unchecked".into(),
                    leaves: vec!["checkbox.font".into(), "checkbox.checked_background".into()],
                    states: vec!["Checked".into(), "Hovered".into()],
                    part: true,
                    when: Some("a widget is hovered".into()),
                },
                ShowcaseElement {
                    id: "chrome.toolbar".into(),
                    name: "Toolbar".into(),
                    parent: "window".into(),
                    leaves: Vec::new(),
                    states: Vec::new(),
                    part: false,
                    when: None,
                },
            ]
        );
        let [checkbox, toolbar] = list.as_slice() else {
            panic!("two elements")
        };
        assert_eq!(checkbox.title(), "Checkbox · Checked");
        assert_eq!(toolbar.title(), "Toolbar");
    }

    /// A list without elements, an element without a field, and a leaf that
    /// is no string do not parse.
    #[test]
    fn a_broken_list_does_not_parse() {
        assert!(parse("").is_err());
        let missing = parse("[[element]]\nid = \"a\"\nname = \"A\"\nleaves = []\nstates = []\n");
        assert_eq!(missing, Err("an element has no `parent`".to_string()));
        let bad = parse(
            "[[element]]\nid = \"a\"\nname = \"A\"\nparent = \"window\"\nleaves = [1]\nstates = []\n",
        );
        assert_eq!(
            bad,
            Err("`leaves` holds a value that is not a string".to_string())
        );
    }

    /// A showcase's column of the counterpart table, less the rows it has
    /// no page for, in the table's order; the table's end is the first line
    /// that is no row.
    #[test]
    fn a_counterpart_table_reads_each_column() {
        let text = "\
# Before it.
#
#   | Counterpart | gpui    | iced      | egui    |
#   |-------------|---------|-----------|---------|
#   | Basic       | Basic   | Basic     | Basic   |
#   | Typography  | Typo    | Display   | -       |
#   | Theme Map   | -       | Theme Map | Map     |
#
#   | Basic       | Later   | Later     | Later   |
";
        assert_eq!(pages_of(text, Kind::Gpui), Ok(vec!["Basic", "Typo"]));
        assert_eq!(
            pages_of(text, Kind::Iced),
            Ok(vec!["Basic", "Display", "Theme Map"])
        );
        assert_eq!(pages_of(text, Kind::Egui), Ok(vec!["Basic", "Map"]));
    }

    /// A header without the table, a head without its rule, a row of
    /// another width and a column with no page do not read.
    #[test]
    fn a_broken_counterpart_table_does_not_read() {
        assert!(pages_of("# no table\n", Kind::Gpui).is_err());
        let head = "#   | Counterpart | gpui | iced | egui |\n";
        assert!(
            pages_of(
                &format!("{head}#   | Basic | Basic | Basic | Basic |\n"),
                Kind::Gpui
            )
            .is_err()
        );
        let rule = "#   |---|---|---|---|\n";
        assert_eq!(
            pages_of(
                &format!("{head}{rule}#   | Basic | Basic | Basic |\n"),
                Kind::Gpui
            ),
            Err("a counterpart row has 3 cells, not 4".to_string())
        );
        assert!(
            pages_of(
                &format!("{head}{rule}#   | Basic | - | Basic | Basic |\n"),
                Kind::Gpui
            )
            .is_err()
        );
    }

    /// The committed table reads for each showcase, and puts the pages the
    /// comparator matches by position where it looks for them: Basic and
    /// Buttons first and second (`chrome.page_tabs.basic`, `.buttons`).
    #[test]
    fn the_committed_counterpart_table_reads() {
        for kind in [Kind::Gpui, Kind::Iced, Kind::Egui] {
            let pages = page_order(kind).unwrap();
            assert_eq!(pages.get(..2), Some(&["Basic", "Buttons"][..]), "{kind:?}");
            assert_eq!(
                pages.get(pages.len() - 2..),
                Some(&["Icons", "Theme Map"][..]),
                "{kind:?}"
            );
        }
        assert!(element("chrome.page_tabs.basic").is_some());
        assert!(element("chrome.page_tabs.buttons").is_some());
    }
}
