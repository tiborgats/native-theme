//! The layout dump `--dump-layout` writes: where a showcase drew each
//! element of `docs/showcase-elements.toml`, in the object the list's header
//! describes ("The layout dump"), which `scripts/check_showcase_parity.py`
//! compares across the three showcases.

use crate::Kind;

/// An element's rectangle, in logical pixels of the window's content: `(0,
/// 0)` is the top-left of the content area.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    /// The left edge.
    pub x: f32,
    /// The top edge.
    pub y: f32,
    /// The width.
    pub w: f32,
    /// The height.
    pub h: f32,
}

/// The layout dump of a `kind` showcase under `preset` in its `variant`
/// (`light` or `dark`) at `scale` physical pixels per logical one: one
/// rectangle per element id of `elements`, the elements of the list it drew.
/// A later rectangle of an id replaces an earlier one.
pub fn layout_json(
    kind: Kind,
    preset: &str,
    variant: &str,
    scale: f32,
    elements: impl IntoIterator<Item = (String, Rect)>,
) -> serde_json::Value {
    let rects: serde_json::Map<String, serde_json::Value> = elements
        .into_iter()
        .map(|(id, r)| {
            (
                id,
                serde_json::json!({"x": r.x, "y": r.y, "w": r.w, "h": r.h}),
            )
        })
        .collect();
    serde_json::json!({
        "kind": kind.name(),
        "preset": preset,
        "variant": variant,
        "scale": scale,
        "elements": rects,
    })
}

/// `json` pretty-printed into `path`; the error names the flag and the path.
pub fn write_layout(path: &str, json: &serde_json::Value) -> Result<(), String> {
    serde_json::to_string_pretty(json)
        .map_err(|e| e.to_string())
        .and_then(|text| std::fs::write(path, text).map_err(|e| e.to_string()))
        .map_err(|error| crate::cli::Output::Dump.failure(path, &error))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// The dump is the object the list's header describes: the kind, the
    /// preset, the variant, the scale, and a rectangle by element id.
    #[test]
    fn the_layout_dump_is_the_lists_object() {
        let json = layout_json(
            Kind::Iced,
            "kde-breeze",
            "dark",
            1.5,
            [
                (
                    "chrome.toolbar".to_string(),
                    Rect {
                        x: 0.0,
                        y: 28.0,
                        w: 1280.0,
                        h: 40.5,
                    },
                ),
                ("chrome.menu_bar".to_string(), Rect::default()),
            ],
        );
        assert_eq!(
            json,
            serde_json::json!({
                "kind": "iced",
                "preset": "kde-breeze",
                "variant": "dark",
                "scale": 1.5,
                "elements": {
                    "chrome.toolbar": {"x": 0.0, "y": 28.0, "w": 1280.0, "h": 40.5},
                    "chrome.menu_bar": {"x": 0.0, "y": 0.0, "w": 0.0, "h": 0.0},
                },
            })
        );
        let header = include_str!("../../../docs/showcase-elements.toml");
        for key in [
            "\"kind\"",
            "\"preset\"",
            "\"variant\"",
            "\"scale\"",
            "\"elements\"",
        ] {
            assert!(header.contains(key), "the list's header names no {key}");
        }
    }

    /// The dump is written pretty-printed and reads back as itself; a path
    /// that cannot be written is named in the error.
    #[test]
    fn the_layout_dump_is_written_and_a_failure_named() {
        let dir =
            std::env::temp_dir().join(format!("showcase-support-dump-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("dump.json");
        let path = path.to_str().unwrap();
        let json = layout_json(Kind::Gpui, "adwaita", "light", 1.0, []);
        assert_eq!(write_layout(path, &json), Ok(()));
        let text = std::fs::read_to_string(path).unwrap();
        assert!(text.contains('\n'), "the dump is not pretty-printed");
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&text).unwrap(),
            json
        );
        let missing = dir.join("no-such-dir").join("dump.json");
        let missing = missing.to_str().unwrap();
        let error = write_layout(missing, &json).unwrap_err();
        assert!(
            error.starts_with(&format!("--dump-layout {missing}: ")),
            "{error}"
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
