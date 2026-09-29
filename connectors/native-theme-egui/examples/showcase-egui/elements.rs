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
