//! The elements the three showcases share, `docs/showcase-elements.toml`, read as its header
//! says: Widget Info's titles and rows, and the ids of `--dump-layout`.

/// One element of `docs/showcase-elements.toml`. The showcase reads its id, name, leaves and
/// states; its tests read the rest.
#[derive(Debug, Clone)]
#[cfg_attr(not(test), allow(dead_code))]
pub struct ShowcaseElement {
    pub id: String,
    pub name: String,
    pub parent: String,
    pub leaves: Vec<String>,
    pub states: Vec<String>,
    pub part: bool,
    pub when: Option<String>,
}

const SHOWCASE_ELEMENTS: &str = include_str!("../../../../docs/showcase-elements.toml");

pub fn showcase_elements() -> Result<Vec<ShowcaseElement>, String> {
    let table: toml::Table = toml::from_str(SHOWCASE_ELEMENTS).map_err(|e| e.to_string())?;
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

/// The tab labels the egui showcase gives its pages, in the order the three showcases list them:
/// the egui column of the counterpart table in the list's header (*The pages' counterparts*),
/// less the rows it has no page for, or why the table does not read. A copy of the reader of
/// `native-theme-showcase-support`'s `elements::page_order`, as this file is of its list reader
/// (until M9c T1); its test holds `Page::ALL` to it.
#[cfg(test)]
pub fn page_order() -> Result<Vec<&'static str>, String> {
    const HEAD: [&str; 4] = ["Counterpart", "gpui", "iced", "egui"];
    const COLUMN: usize = 3;
    let cells = |line: &'static str| -> Option<Vec<&'static str>> {
        let row = line.strip_prefix('#')?.trim();
        let inner = row.strip_prefix('|')?.strip_suffix('|')?;
        Some(inner.split('|').map(str::trim).collect())
    };
    let mut lines = SHOWCASE_ELEMENTS.lines();
    lines
        .by_ref()
        .find(|line| cells(line).is_some_and(|row| row == HEAD))
        .ok_or("docs/showcase-elements.toml: no counterpart table in the header")?;
    let rule = lines.next().and_then(cells).is_some_and(|row| {
        row.iter()
            .all(|cell| !cell.is_empty() && cell.trim_matches('-').is_empty())
    });
    if !rule {
        return Err("the counterpart table's head is not followed by its rule".to_string());
    }
    let mut pages = Vec::new();
    for row in lines.map_while(cells) {
        if row.len() != HEAD.len() {
            return Err(format!(
                "a counterpart row has {} cells, not {}",
                row.len(),
                HEAD.len()
            ));
        }
        match row.get(COLUMN) {
            Some(&"-") | None => {}
            Some(page) => pages.push(*page),
        }
    }
    if pages.is_empty() {
        return Err("the counterpart table lists no egui page".to_string());
    }
    Ok(pages)
}
