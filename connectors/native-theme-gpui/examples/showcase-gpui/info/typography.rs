//! What the Typography page's widgets and text samples report about
//! themselves (spec §3.4). Its Labels are `info::text`'s.

use gpui_component::theme::Theme;
use native_theme_gpui::ResolvedTheme;

use super::{
    WidgetInfo,
    chrome::{GhostContent, ghost_colours},
    claim, drawn_font, px_text,
};
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
        .not_themeable("size", "a rem ladder of this showcase's own -- 1.875 / 1.5 / 1.25 / 1.125 / 1 / 0.875 -- so it follows the platform: gpui-component's WindowState root plugin sets the rem to Theme::font_size (gpui-component/root.rs, WindowState::prepare set_rem_size), which this connector fills from font.size times the text-scaling factor (native-theme-gpui/lib.rs, to_theme). What is unused is text_scale: the model states four named roles, each with its own size, weight and line height, and this demo states six of its own")
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
    // (gpui-pre/text_system/line.rs:663, :683), which is inherited here.
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

/// The monospace sample: plain text in a div given `native`'s
/// `defaults.mono_font` where a native theme is installed
/// (`demo::text_in`), the mono family and nothing else without one.
pub fn mono_text(t: &Theme, native: Option<&ResolvedTheme>) -> WidgetInfo {
    let info = WidgetInfo::new("Text").variant("monospace").color(claim(
        "text, inherited",
        "foreground",
        t.foreground,
        "showcase",
    ));
    match native {
        Some(r) => info
            .config(
                "mono font",
                format!("defaults.mono_font: {}", drawn_font(&r.defaults.mono_font)),
            )
            .config(
                "size",
                format!(
                    "{}px: defaults.mono_font.size scaled by the text-scaling factor, the size the sample sets, which is Theme::mono_font_size (native-theme-gpui lib.rs, to_theme)",
                    px_text(t.mono_font_size.as_f32())
                ),
            ),
        None => info
            .config(
                "mono font",
                format!("mono_font_family: {}", t.mono_font_family),
            )
            .config(
            "size",
            format!(
                "{}px: font_size, the window's rem. The sample sets the mono family and no size, so mono_font_size ({}px) does not reach it: no native theme is installed",
                px_text(t.font_size.as_f32()),
                px_text(t.mono_font_size.as_f32())
            ),
        ),
    }
}

/// A `Link`, wherever the showcase shows one; the caller adds what it reads
/// and where it goes. `underline` is the `link.underline_enabled` that
/// `geometry::link` refined it by, `None` where no native theme is installed
/// and none refines it.
pub fn link(t: &Theme, underline: Option<bool>) -> WidgetInfo {
    // link.rs:74-88: the text and its underline, then a hover and an active
    // style over them, all from the one token. The resting underline and the
    // hovered and pressed text are literal opacities of it, so those
    // swatches are the opacities.
    let info =
        WidgetInfo::new("Link").color(claim("text", "link", t.link, "gpui-component/link.rs:76"));
    // geometry::link removes the resting underline where the theme says so.
    let info = if underline == Some(false) {
        info
    } else {
        info.color(claim(
            "resting underline, at 50%",
            "link",
            t.link.opacity(0.5),
            "gpui-component/link.rs:78",
        ))
    };
    let info = info
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
        ));
    let info = match underline {
        Some(true) => info.config("underline", "at rest, as upstream draws it: link.underline_enabled is true, so geometry::link leaves the underline gpui-component sets on the base style before the caller's refinement (gpui-component/link.rs, RenderOnce for Link)"),
        Some(false) => info.config("underline", "none at rest: link.underline_enabled is false, so geometry::link removes the resting underline; hover and press still underline. The refinement sets the underline's thickness to 0, which paints none, over the text_decoration_1 upstream sets on the base style first, and upstream's hover and active styles set it again over the refinement (gpui-component/link.rs, RenderOnce for Link)"),
        None => info.not_themeable("underline", "always on: gpui-component sets it on the base style before the caller's refinement (gpui-component/link.rs, RenderOnce for Link), and no native theme is installed, so no geometry::link refines it, which removes it at rest where link.underline_enabled is false"),
    };
    info.not_themeable("hover text", "link at 0.8, computed from the one token (gpui-component/link.rs, Link). Upstream has a link_hover token, the connector writes it from link.hover_text_color, and a Button::link() reads it (button/button.rs, ButtonVariant) -- this widget is the one place that does not. Tier U")
        .not_themeable("pressed text", "link at 0.6, the same story: link_active is written by the connector from link.active_text_color and read by the Button variant, not here (gpui-component/link.rs, Link)")
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
            "gpui-component/kbd.rs:243",
        ))
        .color(claim(
            "text",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/kbd.rs:242",
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
        .not_themeable("font", "not monospace, and not set at all: kbd.rs states a text size and a colour and no family (kbd.rs, Kbd), so a Kbd inherits the window's -- the platform UI font gpui-component's WindowState root plugin applies (gpui-component/root.rs, WindowState::style). The panel claimed a family nothing produces")
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
            "gpui-component/input/input.rs:798",
        ))
        // Painted at a literal 85% of the style block's border
        // (gpui-base/input/base/element.rs:3121).
        .color(claim(
            "indent guides, border at 85%",
            "border",
            t.border.opacity(0.85),
            "gpui-base/input/base/element.rs:3121",
        ))
        // Unhighlighted text is painted in the colour the element inherits
        // (gpui-base/input/base/element.rs:2542), not the style block's
        // foreground, and Input sets none (input/input.rs:707): so the
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
            "gpui-component/input/input.rs:563",
        ))
        .color(claim(
            "selection",
            "selection",
            t.selection,
            "gpui-component/input/input.rs:562",
        ))
        .color(claim(
            "line numbers",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/input/input.rs:559",
        ))
        // The current line's number takes the style block's foreground
        // (gpui-base/input/base/element.rs:2920).
        .color(claim(
            "current line number",
            "foreground",
            t.foreground,
            "gpui-component/input/input.rs:558",
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
        .not_themeable("own icons", super::own_icons("ChevronDown and ChevronRight on the fold buttons in its gutter (input/input.rs, fold_icon_renderer), and the icons of the search panel it opens (input/search.rs, SearchPanel)"))
}

/// The Markdown `TextView`.
pub fn markdown(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("TextView")
        .variant("markdown")
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/text/mod.rs:49",
        ))
        .color(claim(
            "link",
            "link",
            t.link,
            "gpui-component/text/mod.rs:51",
        ))
        .color(claim(
            "code block bg",
            "muted",
            t.muted,
            "gpui-component/text/mod.rs:53",
        ))
        .color(claim(
            "inline code bg",
            "accent",
            t.accent,
            "gpui-component/text/mod.rs:59",
        ))
        .color(claim(
            "table border",
            "border",
            t.border,
            "gpui-component/text/mod.rs:54",
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
            "gpui-component/text/mod.rs:46",
        ))
        .config("border-radius", format!("radius: {}px", px_text(t.radius.as_f32())))
        .config(
            "mono font",
            format!("mono_font_family: {}", t.mono_font_family),
        )
        .not_themeable("style source", "a TextView reads no theme field itself: gpui-component builds a TextViewStyle from the theme once and the view takes it (text/mod.rs)")
        .not_themeable("inline code", "accent again -- the menu highlight, shared with menu rows, Toggle and a hovered ghost button (Tier U)")
        .not_themeable("heading sizes", "1.8, 4/3, 17/15, 1, 14/15 and 13/15 times the font size the view inherits, which follows the platform's: gpui-base scales a heading from literals (gpui-base/text/node.rs, heading_metrics, lines 2867-2876; body_font_size), and base_text_view_style installs no heading refinement over them (text/mod.rs, base_text_view_style; gpui-base/text/style.rs, with_heading). Every level is semibold, a literal too, and the sixth takes muted_foreground (gpui-base/text/node.rs, BlockNode::Heading, lines 3903-3906)")
        .not_themeable("link underline", "always drawn, a 1px hairline in the link colour at 0.4 alpha (gpui-base/text/node.rs, link_underline, lines 2841-2847), so link.underline_enabled = false cannot be honoured")
        .not_themeable("selection text", "the glyphs' own colour: the selection, in the selection token, paints under the glyphs (gpui-base/text/inline.rs, paint_selection, lines 1007 and 1018), so the selection's text colour cannot be honoured")
}

/// The `Diff` over the showcase's one-file patch, its hovered line and line
/// number highlighted.
pub fn diff(t: &Theme) -> WidgetInfo {
    let rem = t.font_size.as_f32();
    let mono = t.mono_font_size.as_f32();
    WidgetInfo::new("Diff")
        .variant("unified")
        .color(claim(
            "bg",
            "background",
            t.background,
            "gpui-component/diff/mod.rs:718",
        ))
        // The file name, the hunk's code where no syntax colour lands, and
        // the header's text inherit the frame's colour.
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/diff/mod.rs:719",
        ))
        .color(claim(
            "edge",
            "border",
            t.border,
            "gpui-component/diff/mod.rs:722",
        ))
        .color(claim(
            "file header rule",
            "border",
            t.border,
            "gpui-component/diff/mod.rs:1117",
        ))
        .color(claim(
            "file header counts",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/diff/mod.rs:1116",
        ))
        .colors(ghost_colours(t, GhostContent::Icon))
        .color(claim(
            "hunk separator, at 35%",
            "muted",
            t.muted.opacity(0.35),
            "gpui-component/diff/mod.rs:933",
        ))
        .color(claim(
            "hunk separator text",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/diff/mod.rs:935",
        ))
        .color(claim(
            "line numbers",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/diff/mod.rs:1301",
        ))
        .color(claim(
            "hovered line number",
            "foreground",
            t.foreground,
            "gpui-component/diff/mod.rs:1302",
        ))
        // A changed line's colour is read once, as `status`, and then
        // painted as its sign, its fill at 12% (diff/mod.rs:1681-1683) and
        // its changed words at 30% (:1481).
        .color(claim(
            "removed sign",
            "danger",
            t.danger,
            "gpui-component/diff/mod.rs:1576",
        ))
        .color(claim(
            "removed line, at 12%",
            "danger",
            t.danger.opacity(0.12),
            "gpui-component/diff/mod.rs:1576",
        ))
        .color(claim(
            "removed words, at 30%",
            "danger",
            t.danger.opacity(0.3),
            "gpui-component/diff/mod.rs:1576",
        ))
        .color(claim(
            "added sign",
            "success",
            t.success,
            "gpui-component/diff/mod.rs:1578",
        ))
        .color(claim(
            "added line, at 12%",
            "success",
            t.success.opacity(0.12),
            "gpui-component/diff/mod.rs:1578",
        ))
        .color(claim(
            "added words, at 30%",
            "success",
            t.success.opacity(0.3),
            "gpui-component/diff/mod.rs:1578",
        ))
        .color(claim(
            "hovered line and line number",
            "list_hover",
            t.list_hover,
            "gpui-component/diff/mod.rs:1686",
        ))
        .color(claim(
            "selected lines",
            "selection",
            t.selection,
            "gpui-component/diff/mod.rs:1684",
        ))
        .color(claim(
            "selected text",
            "selection",
            t.selection,
            "gpui-component/diff/selection.rs:283",
        ))
        .config(
            "mono font",
            format!("mono_font_family: {}", t.mono_font_family),
        )
        .config(
            "code size",
            format!(
                "{}px: rem × mono_font_size / 16, at a rem of {}px and a mono_font_size of {}px",
                px_text(rem * mono / 16.0),
                px_text(rem),
                px_text(mono)
            ),
        )
        .not_themeable("code size", "the code is sized rem × mono_font_size / 16 and its rows 1.6 times that (diff/mod.rs, render_diff), so it is mono_font_size only where the rem is 16px: a smaller platform font makes the code smaller than the platform's monospace size. No builder reaches it without changing every other monospace text; an upstream item")
        .not_themeable("alphas", "the 12% of a changed line's fill (diff/mod.rs, code_line: lines 1681-1683), the 30% of its changed words (diff/mod.rs, line_highlights: line 1481) and the 35% of the hunk separator (diff/mod.rs, render_hunk_separator: line 933) are upstream literals over the theme's colours")
        .not_themeable("syntax colors", "highlight_theme: default_light / default_dark per color mode (diff/mod.rs, line_highlights)")
        .not_themeable("collapse button", "the icon, hover and pressed swatches are the file header's ghost collapse Button, which the widget builds and no refinement reaches (diff/mod.rs, render_file_header)")
        .not_themeable("own icons", super::own_icons("ChevronDown and ChevronRight on the file header's collapse button (diff/mod.rs, render_file_header)"))
        .instance("hover", "DiffHoverHighlight::Both, set by the showcase: upstream highlights nothing by default (diff/mod.rs, DiffHoverHighlight)")
        .instance("add-annotation button", "none: it is shown only when on_add_annotation is set (diff/mod.rs, add_annotation_button: line 1508), and this one is not. It would paint primary, primary_foreground and radius (lines 1537-1539)")
        .instance("conflict sides", "none: only a file read by DiffFile::parse_conflicts is conflicted (diff/conflict.rs, parse_conflicts: lines 264-265), and this patch is read by DiffFile::parse. Its current side would be tinted success and its incoming side info (diff/mod.rs, conflict_tint: lines 1098-1099)")
}
