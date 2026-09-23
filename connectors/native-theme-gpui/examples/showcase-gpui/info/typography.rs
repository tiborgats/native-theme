//! What the Typography page's widgets and text samples report about
//! themselves (spec §3.4). Its Labels are `info::text`'s.

use gpui_component::theme::Theme;

use super::{WidgetInfo, claim, px_text};
use crate::demo::{DecorationKind, HeadingLevel, TextSize, WeightKind};

/// One level of the showcase's own heading ladder: plain text in a div, no
/// widget, at a rem size and a weight the showcase sets.
pub fn heading_level(t: &Theme, level: HeadingLevel) -> WidgetInfo {
    let rem = t.font_size.as_f32();
    WidgetInfo::new("Text")
        .variant(level.name())
        // A div paints no colour of its own: the text takes the one the
        // showcase sets on its window.
        .color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .config("font", format!("font_family: {}", t.font_family))
        .config(
            "size",
            format!(
                "{}px: {} rem at a font_size of {}px",
                px_text(level.rems() * rem),
                level.rems(),
                px_text(rem)
            ),
        )
        .not_themeable("size", "a rem ladder of this showcase's own -- 1.875 / 1.5 / 1.25 / 1.125 / 1 / 0.875 -- so it follows the platform: gpui-component sets the rem to Theme::font_size (root.rs, Root::render set_rem_size), which this connector fills from font.size times the text-scaling factor (native-theme-gpui/lib.rs, to_theme). What is unused is text_scale: the model states four named roles, each with its own size, weight and line height, and this demo states six of its own")
        .instance(
            "weight",
            format!(
                "FontWeight::{}, a gpui constant the showcase sets",
                level.weight().1
            ),
        )
}

/// One sample of the Font Weights list: plain text in a div at `weight`.
pub fn weight(t: &Theme, weight: WeightKind) -> WidgetInfo {
    WidgetInfo::new("Text")
        .variant(weight.name())
        .color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .config("font", format!("font_family: {}", t.font_family))
        .instance(
            "weight",
            format!(
                "FontWeight::{}, a gpui constant the showcase sets",
                weight.weight().1
            ),
        )
}

/// One sample of the Text Decorations list: plain text in a div, styled
/// `decoration`.
pub fn decoration(t: &Theme, decoration: DecorationKind) -> WidgetInfo {
    let info = WidgetInfo::new("Text")
        .variant(decoration.name())
        .color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .config("font", format!("font_family: {}", t.font_family));
    // A line given no colour is drawn in its text run's colour
    // (gpui-pre text_system/line.rs:662, :682), which is inherited here.
    match decoration {
        DecorationKind::Bold => info.instance("style", "FontWeight::BOLD, a gpui constant the showcase sets"),
        DecorationKind::Underline => info
            .color(claim(
                "underline, the text's colour",
                "foreground",
                t.foreground,
                "showcase",
            ))
            .instance("style", "underline, 1px thick and given no colour, so gpui draws it in the text's colour (gpui-pre/text_system/line.rs, paint_line)"),
        DecorationKind::Strikethrough => info
            .color(claim(
                "strikethrough, the text's colour",
                "foreground",
                t.foreground,
                "showcase",
            ))
            .instance("style", "line_through, 1px thick and given no colour, so gpui draws it in the text's colour (gpui-pre/text_system/line.rs, paint_line)"),
        DecorationKind::Italic => info.instance("style", "FontStyle::Italic, set by the showcase (gpui-pre/styled.rs, italic)"),
    }
}

/// The muted sample: plain text in a div the showcase paints
/// muted_foreground.
pub fn muted_text(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Text")
        .variant("muted")
        .color(claim(
            "text",
            "muted_foreground",
            t.muted_foreground,
            "showcase",
        ))
        .config("font", format!("font_family: {}", t.font_family))
        .instance("style", "muted_foreground, set by the showcase on a plain div: no Label is involved, so neither label.rs's foreground nor its secondary highlight is")
}

/// The monospace sample: plain text in a div given the mono family and
/// nothing else.
pub fn mono_text(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Text")
        .variant("monospace")
        .color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .config(
            "mono font",
            format!("mono_font_family: {}", t.mono_font_family),
        )
        .config(
            "size",
            format!(
                "{}px: font_size, the window's rem. The sample sets the mono family and no size, so mono_font_size ({}px) does not reach it",
                px_text(t.font_size.as_f32()),
                px_text(t.mono_font_size.as_f32())
            ),
        )
}

/// A `Link`, wherever the showcase shows one; the caller adds what it reads
/// and where it goes.
pub fn link(t: &Theme) -> WidgetInfo {
    // link.rs:74-88: the text and its underline, then a hover and an active
    // style over them, all from the one token. The resting underline and the
    // hovered and pressed text are literal opacities of it, so those
    // swatches are the opacities.
    WidgetInfo::new("Link")
        .color(claim(
            "text",
            "link",
            t.link,
            "gpui-component/link.rs:76",
        ))
        .color(claim(
            "underline, at 50%",
            "link",
            t.link.opacity(0.5),
            "gpui-component/link.rs:78",
        ))
        .color(claim(
            "hover text, at 80%",
            "link",
            t.link.opacity(0.8),
            "gpui-component/link.rs:80",
        ))
        .color(claim(
            "hover underline",
            "link",
            t.link,
            "gpui-component/link.rs:82",
        ))
        .color(claim(
            "pressed text, at 60%",
            "link",
            t.link.opacity(0.6),
            "gpui-component/link.rs:85",
        ))
        .color(claim(
            "pressed underline",
            "link",
            t.link,
            "gpui-component/link.rs:87",
        ))
        .not_themeable("underline", "always on, and out of reach: the decoration is set on the base style before the caller's refinement merges into it (link.rs, Link), and a refinement whose `underline` is None leaves the base's `Some` standing -- `text_decoration_none` sets exactly that None, so it cannot switch one off. link.underline_enabled is modelled and has no receiver: Tier U")
        .not_themeable("hover text", "link at 0.8, computed from the one token (link.rs, Link). Upstream has a link_hover token, the connector writes it from link.hover_text_color, and a Button::link() reads it (button/button.rs, ButtonVariant) -- this widget is the one place that does not. Tier U")
        .not_themeable("pressed text", "link at 0.6, the same story: link_active is written by the connector from link.active_text_color and read by the Button variant, not here (link.rs, Link)")
}

/// A `Kbd` for `keys`, which `Kbd::format` draws as `drawn` on this
/// platform.
pub fn kbd(t: &Theme, keys: &str, drawn: &str) -> WidgetInfo {
    let rem = t.font_size.as_f32();
    WidgetInfo::new("Kbd")
        .color(claim(
            "bg",
            "muted",
            t.muted,
            "gpui-component/kbd.rs:238",
        ))
        .color(claim(
            "text",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/kbd.rs:237",
        ))
        .config(
            "border-radius",
            format!("radius / 2: {}px", px_text(t.radius.as_f32() / 2.0)),
        )
        .config(
            "size",
            format!(
                "{}px: text_xs, {} rem at a font_size of {}px",
                px_text(TextSize::Xs.rems() * rem),
                TextSize::Xs.rems(),
                px_text(rem)
            ),
        )
        .not_themeable("edge", "none: only an outline Kbd draws one, in border, and these are not outlined (kbd.rs, Kbd)")
        .not_themeable("padding", "py_0p5 and px_1 (kbd.rs, Kbd) -- rems, so already proportional to the platform font")
        .not_themeable("font", "not monospace, and not set at all: kbd.rs states a text size and a colour and no family (kbd.rs, Kbd), so a Kbd inherits the window's -- the platform UI font Root::render applies. The panel claimed a family nothing produces")
        .instance(
            "keys",
            format!("{keys}, drawn as \"{drawn}\" on this platform (kbd.rs, Kbd::format)"),
        )
}

/// The Rust code `Editor`.
pub fn editor(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Editor")
        .color(claim(
            "edge",
            "input",
            t.input,
            "gpui-component/input/input.rs:714",
        ))
        // Painted at a literal 85% of the style block's border
        // (gpui-base input/base/element.rs:2319).
        .color(claim(
            "indent guides, border at 85%",
            "border",
            t.border.opacity(0.85),
            "gpui-base/input/base/element.rs:2319",
        ))
        // Unhighlighted text is painted in the colour the element inherits
        // (gpui-base input/base/element.rs:1823), not the style block's
        // foreground, and Input sets none (input/input.rs:639): so the
        // colour the showcase sets on its window.
        .color(claim(
            "unhighlighted text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .color(claim(
            "caret",
            "caret",
            t.caret,
            "gpui-component/input/input.rs:503",
        ))
        .color(claim(
            "selection",
            "selection",
            t.selection,
            "gpui-component/input/input.rs:502",
        ))
        .color(claim(
            "line numbers",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/input/input.rs:499",
        ))
        // The current line's number takes the style block's foreground
        // (gpui-base input/base/element.rs:2126).
        .color(claim(
            "current line number",
            "foreground",
            t.foreground,
            "gpui-component/input/input.rs:498",
        ))
        .config(
            "mono font",
            format!("mono_font_family: {}", t.mono_font_family),
        )
        .config(
            "mono size",
            format!("mono_font_size: {}px (gpui renders)", px_text(t.mono_font_size.as_f32())),
        )
        .not_themeable("bg", "#0a0a0a in dark and #ffffff in light, whatever the platform: the connector installs upstream's default highlight themes, and their editor.background wins over the input_background() fallback (theme/mod.rs, editor_background; native-theme-gpui/lib.rs, to_theme). The current line is filled from the same theme, #171717 or #F5F5F5. The fields are public, so this one is ours")
        .not_themeable("syntax colors", "highlight_theme: default_light / default_dark per color mode; the grammar comes from the tree-sitter-rust dev feature")
        .not_themeable("style block", "the editor takes one InputEditorStyle built in a single place (input/input.rs, set_editor_style), so its colours cite the same block. Its border paints the indent guides; the edge is the bordered Input's own input token (input/editor.rs, Editor::render)")
        .not_themeable("text colour", "none of its own: the block's foreground reaches only the current line's number, and unhighlighted text takes the colour it inherits (gpui-base/input/base/element.rs, prepaint), which is the one the showcase sets on its window. A syntax-highlighted run takes its colour from highlight_theme instead")
        .not_themeable("line height", "1.5 is only what the widget sets first: Editor is Styled and applies the caller's refinement last, on purpose (input/editor.rs, Editor::render -- the comment there says a text style set on the editor refines over them). defaults.line_height is modelled (1.4 on the bundled defaults) and no builder carries it yet")
        .not_themeable("line numbers / search", "on by default (gpui-base/input/base/state.rs, EditorMode)")
}

/// The Markdown `TextView`.
pub fn markdown(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("TextView")
        .variant("markdown")
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/text/mod.rs:48",
        ))
        .color(claim(
            "link",
            "link",
            t.link,
            "gpui-component/text/mod.rs:50",
        ))
        .color(claim(
            "code block bg",
            "muted",
            t.muted,
            "gpui-component/text/mod.rs:52",
        ))
        .color(claim(
            "inline code bg",
            "accent",
            t.accent,
            "gpui-component/text/mod.rs:58",
        ))
        .color(claim(
            "table border",
            "border",
            t.border,
            "gpui-component/text/mod.rs:53",
        ))
        .color(claim(
            "table head",
            "table_head",
            t.table_head,
            "gpui-component/text/mod.rs:44",
        ))
        .color(claim(
            "table head text",
            "table_head_foreground",
            t.table_head_foreground,
            "gpui-component/text/mod.rs:45",
        ))
        .config("border-radius", format!("radius: {}px", px_text(t.radius.as_f32())))
        .config(
            "mono font",
            format!("mono_font_family: {}", t.mono_font_family),
        )
        .not_themeable("style source", "a TextView reads no theme field itself: gpui-component builds a TextViewStyle from the theme once and the view takes it (text/mod.rs)")
        .not_themeable("inline code", "accent again -- the menu highlight, shared with menu rows, Toggle and a hovered ghost button (Tier U)")
        .not_themeable("heading sizes", "2, 1.5, 1.25, 1.125, 1 and 1 times a fixed 14px, not the platform's font: base_text_view_style never sets heading_base_font_size (text/mod.rs, base_text_view_style), so gpui-base's default stands (gpui-base/text/style.rs, heading_base_font_size; gpui-base/text/node.rs, BlockNode::Heading). The weights are literals too, bold down to medium")
}
