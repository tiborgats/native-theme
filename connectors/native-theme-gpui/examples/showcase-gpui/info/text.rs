//! What the pages' own text reports about itself (spec §5.3): section
//! headings, captions and small labels, each a `Label`.

use gpui_component::theme::Theme;

use super::{WidgetInfo, claim};

/// A section heading, built by `demo::heading`.
pub fn heading(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Label")
        .variant("heading")
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/label.rs:211",
        ))
        .not_themeable(
            "line height",
            "rems(1.25), a literal, so it follows the platform's font (label.rs, Label::render)",
        )
        .instance(
            "style",
            "semibold, at a pixel size the showcase sets itself: not a theme value, and not scaled with the platform's font",
        )
}

/// A caption, built by `demo::caption`.
pub fn caption(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Label")
        .variant("caption")
        .color(claim(
            "text",
            "muted_foreground",
            t.muted_foreground,
            "showcase",
        ))
        .not_themeable(
            "line height",
            "rems(1.25), a literal, so it follows the platform's font (label.rs, Label::render)",
        )
        .instance(
            "style",
            "text_sm -- a rem, so it follows the platform's font -- in muted_foreground: the showcase's own caption style, which the Label applies over the foreground it paints (label.rs, Label::render)",
        )
}

/// A small label beside a widget, built by `demo::label`.
pub fn label(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Label")
        .variant("small")
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/label.rs:211",
        ))
        .not_themeable(
            "line height",
            "rems(1.25), a literal, so it follows the platform's font (label.rs, Label::render)",
        )
        .instance(
            "style",
            "text_sm -- a rem, so it follows the platform's font -- in the foreground the Label paints itself (label.rs, Label::render)",
        )
}
