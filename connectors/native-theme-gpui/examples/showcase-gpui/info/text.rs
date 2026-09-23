//! What the pages' own text reports about itself (spec §5.3): section
//! headings, captions and small labels, each a `Label` -- and the Labels the
//! Typography page shows, which are the same widget asked for more.

use gpui_component::theme::Theme;

use super::{WidgetInfo, claim, px_text};
use crate::demo::{LabelKind, TextSize};

/// A `Label` of `variant`, with what every Label sets whatever it is asked
/// for: the line height Label::render gives its text.
pub(super) fn label_of(variant: impl Into<String>) -> WidgetInfo {
    WidgetInfo::new("Label").variant(variant).not_themeable(
        "line height",
        "rems(1.25), a literal, so it follows the platform's font (label.rs, Label::render)",
    )
}

/// A section heading, built by `demo::heading`.
pub fn heading(t: &Theme) -> WidgetInfo {
    label_of("heading")
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/label.rs:211",
        ))
        .instance(
            "style",
            "semibold at text_base -- one rem, so it follows the platform's font, a step above the text_sm of the captions and labels -- in the foreground the Label paints itself (label.rs, Label::render): the showcase's own heading style",
        )
}

/// A caption, built by `demo::caption`.
pub fn caption(t: &Theme) -> WidgetInfo {
    label_of("caption")
        .color(claim(
            "text",
            "muted_foreground",
            t.muted_foreground,
            "showcase",
        ))
        .instance(
            "style",
            "text_sm -- a rem, so it follows the platform's font -- in muted_foreground: the showcase's own caption style, which the Label applies over the foreground it paints (label.rs, Label::render)",
        )
}

/// A small label beside a widget, built by `demo::label`.
pub fn label(t: &Theme) -> WidgetInfo {
    label_of("small")
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/label.rs:211",
        ))
        .instance(
            "style",
            "text_sm -- a rem, so it follows the platform's font -- in the foreground the Label paints itself (label.rs, Label::render)",
        )
}

/// What a Label the showcase leaves at its default size says about its
/// font: the family and the size it inherits from the window.
fn inherited_font(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    info.config("font", format!("font_family: {}", t.font_family))
        .not_themeable("highlights", "blue, on the ranges Label::highlights marks, which this demo does not set (label.rs, Label::highlights)")
        .not_themeable("font weight", "ambient, and the ambient weight is not the platform's: label.rs states no weight, no family and no size, gpui-component's Theme has no font-weight field, and Root::render sets the family, the rem size and the foreground but no weight (root.rs, Root). The geometry:: builders that carry a font spec do carry font.weight with it -- input, list_item, tooltip, status_bar, checkbox and the rest through with_text, and button since v0.5.9 -- so a Label outside one, like these, renders at gpui's default instead. The panel used to call that hardcoded, which is the reverse: nothing sets it")
}

/// A Label of the Typography page's Label gallery, of `kind`, reading
/// `text`.
pub fn gallery_label(t: &Theme, kind: LabelKind, text: &str) -> WidgetInfo {
    let info = label_of(kind.name()).color(claim(
        "text",
        "foreground",
        t.foreground,
        "gpui-component/label.rs:211",
    ));
    let info = match kind {
        LabelKind::Plain => info.instance("text", text.to_string()),
        LabelKind::Secondary(secondary) => info
            .color(claim(
                "secondary",
                "muted_foreground",
                t.muted_foreground,
                "gpui-component/label.rs:171",
            ))
            .instance("text", text.to_string())
            .instance(
                "secondary",
                format!("\"{secondary}\", after a space, the two drawn as one text with the second part highlighted in muted_foreground (label.rs, Label::full_text)"),
            ),
        // Label::render repeats the mask once per character of the whole
        // text (label.rs:201-205), and measure_highlights gives up on a
        // masked Label before it colours anything (:154-156).
        LabelKind::Masked => info.instance(
            "masked",
            format!(
                "every character of \"{text}\" is drawn as a •, {} of them, so the text itself never reaches the screen. A masked Label paints no secondary colour and no highlight either (label.rs, Label::measure_highlights)",
                text.chars().count()
            ),
        ),
    };
    inherited_font(info, t).config(
        "size",
        format!(
            "{}px: font_size, the window's rem, which a Label with no size of its own inherits",
            px_text(t.font_size.as_f32())
        ),
    )
}

/// A Label of the Typography page's Font Sizes gallery, at `size`.
pub fn sized_label(t: &Theme, size: TextSize) -> WidgetInfo {
    let info = label_of(size.name()).color(claim(
        "text",
        "foreground",
        t.foreground,
        "gpui-component/label.rs:211",
    ));
    let info = match size {
        TextSize::Xs => info.instance("size", "text_xs: rems(0.75) (gpui-pre/styled.rs, text_xs)"),
        TextSize::Sm => info.instance("size", "text_sm: rems(0.875) (gpui-pre/styled.rs, text_sm)"),
        TextSize::Base => info.instance(
            "size",
            "text_base: rems(1.) (gpui-pre/styled.rs, text_base) -- the same as gpui's default text size (gpui-pre/style.rs, TextStyle)",
        ),
        TextSize::Lg => info.instance("size", "text_lg: rems(1.125) (gpui-pre/styled.rs, text_lg)"),
        TextSize::Xl => info.instance("size", "text_xl: rems(1.25) (gpui-pre/styled.rs, text_xl)"),
    };
    let rem = t.font_size.as_f32();
    inherited_font(info, t)
        .config(
            "size",
            format!(
                "{}px: {} rem at a font_size of {}px",
                px_text(size.rems() * rem),
                size.rems(),
                px_text(rem)
            ),
        )
        .not_themeable("rem", "Theme::font_size, which gpui-component makes the window's rem (root.rs, Root::render set_rem_size) and the connector fills from the platform's font.size times the text-scaling factor (native-theme-gpui/lib.rs, to_theme). So all five sizes follow the platform's font and none is a size of its own: the model's text_scale roles, each with its own size, reach none of them")
}
