//! The Typography tab.

use gpui::{Context, IntoElement, Keystroke, ParentElement, Styled, div, prelude::*, px, rems};
use gpui_component::{
    ActiveTheme, h_flex, input::Editor, kbd::Kbd, label::Label, link::Link, text::TextView, v_flex,
};

use crate::app::Showcase;
use crate::support::{MARKDOWN_SAMPLE, format_font_info, section};

impl Showcase {
    // -----------------------------------------------------------------------
    // Tab: Typography
    // -----------------------------------------------------------------------
    pub(crate) fn render_typography_tab(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Labels
            .child(section("Label"))
            .child(
                div()
                    .id("tt-label")
                    .child(
                        v_flex()
                            .gap_2()
                            .child(Label::new("Regular label"))
                            .child(Label::new("Label with secondary").secondary("(secondary text)"))
                            .child(Label::new("Masked label: secret123").masked(true)),
                    )
                    .on_hover(self.hover_info(&fi, "Label", &[("text", "foreground", t.foreground, "gpui-component/label.rs:211"), ("secondary", "muted_foreground", t.muted_foreground, "gpui-component/label.rs:171")], &[
                            ("font", format!("font_family: {}", t.font_family)),
                            (
                                "size",
                                format!("font_size: {}px (gpui renders)", t.font_size.as_f32()),
                            ),
                        ], &[("highlights", "blue, on the ranges Label::highlights marks, which this demo does not set (label.rs, Label::highlights)"), ("font weight", "ambient, and the ambient weight is not the platform's: label.rs states no weight, no family and no size, gpui-component's Theme has no font-weight field, and Root::render sets the family, the rem size and the foreground but no weight (root.rs, Root). The geometry:: builders that carry a font spec do carry font.weight with it -- input, list_item, tooltip, status_bar, checkbox and the rest through with_text, and button since v0.5.9 -- so a Label outside one, like these, renders at gpui's default instead. The panel used to call that hardcoded, which is the reverse: nothing sets it")])),
            )
            // Link
            .child(section("Link"))
            .child(
                div()
                    .id("tt-link")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                Link::new("link-1")
                                    .child("Visit Documentation")
                                    .href("https://github.com"),
                            )
                            .child(
                                Link::new("link-2")
                                    .child("Another Link")
                                    .href("https://gpui.rs"),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Link", &[("text+decoration", "link", t.link, "gpui-component/link.rs:76")], &[], &[
                            ("underline", "always on, and out of reach: the decoration is set on the base style before the caller's refinement merges into it (link.rs, Link), and a refinement whose `underline` is None leaves the base's `Some` standing -- `text_decoration_none` sets exactly that None, so it cannot switch one off. link.underline_enabled is modelled and has no receiver: Tier U"),
                            ("hover text", "link at 0.8, computed from the one token (link.rs, Link). Upstream has a link_hover token, the connector writes it from link.hover_text_color, and a Button::link() reads it (button/button.rs, ButtonVariant) -- this widget is the one place that does not. Tier U"),
                            ("active text", "link at 0.6, the same story: link_active is written by the connector and read by the Button variant, not here (link.rs, Link)"),
                        ])),
            )
            // Headings
            .child(section("Headings (H1–H6)"))
            .child(
                div()
                    .id("tt-headings")
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(rems(1.875))
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("H1 — Page Title"),
                            )
                            .child(
                                div()
                                    .text_size(rems(1.5))
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("H2 — Section"),
                            )
                            .child(
                                div()
                                    .text_size(rems(1.25))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("H3 — Subsection"),
                            )
                            .child(
                                div()
                                    .text_size(rems(1.125))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("H4 — Group"),
                            )
                            .child(
                                div()
                                    .text_size(rems(1.0))
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .child("H5 — Detail"),
                            )
                            .child(
                                div()
                                    .text_size(rems(0.875))
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .child("H6 — Fine Print"),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Headings", &[("text", "foreground", t.foreground, "gpui-component/root.rs:596")], &[("font", format!("font_family: {}", t.font_family))], &[("sizes", "a rem ladder of this showcase's own -- 1.875 / 1.5 / 1.25 / 1.125 / 1 / 0.875 -- so it follows the platform: gpui-component sets the rem to Theme::font_size (root.rs, Root::render set_rem_size), which this connector fills from font.size. The px figures this note used to give were the ladder at a 16px rem, which no preset here produces. What is unused is text_scale: the model states four named roles, each with its own size, weight and line height, and this demo states six of its own")])),
            )
            // Font weights
            .child(section("Font Weights (Thin → Black)"))
            .child(
                div()
                    .id("tt-weights")
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::THIN)
                                    .child("Thin (100)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::EXTRA_LIGHT)
                                    .child("Extra Light (200)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::LIGHT)
                                    .child("Light (300)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::NORMAL)
                                    .child("Normal (400)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .child("Medium (500)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("Semibold (600)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("Bold (700)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::EXTRA_BOLD)
                                    .child("Extra Bold (800)"),
                            )
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::BLACK)
                                    .child("Black (900)"),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Font Weights", &[("text", "foreground", t.foreground, "gpui-component/root.rs:596")], &[("font", format!("font_family: {}", t.font_family))], &[("weights", "gpui::FontWeight constants")])),
            )
            // Font sizes
            .child(section("Font Sizes (XS → XL)"))
            .child(
                div()
                    .id("tt-sizes")
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("text_xs — Extra Small").text_xs())
                            .child(Label::new("text_sm — Small").text_sm())
                            .child(Label::new("text_base — Base (default)"))
                            .child(Label::new("text_lg — Large").text_lg())
                            .child(Label::new("text_xl — Extra Large").text_xl()),
                    )
                    .on_hover(self.hover_info(&fi, "Font Sizes", &[("text", "foreground", t.foreground, "gpui-component/label.rs:211")], &[("base", format!("font_size: {}px (gpui renders)", t.font_size.as_f32()))], &[
                            ("xs", "0.75rem"),
                            ("sm", "0.875rem"),
                            ("base", "1rem"),
                            ("lg", "1.125rem"),
                            ("xl", "1.25rem"),
                        ])),
            )
            // Text decorations
            .child(section("Text Decorations"))
            .child(
                div()
                    .id("tt-decorations")
                    .child(
                        v_flex()
                            .gap_1()
                            .child(div().font_weight(gpui::FontWeight::BOLD).child("Bold text"))
                            .child(
                                div()
                                    .underline()
                                    .text_decoration_1()
                                    .child("Underlined text"),
                            )
                            .child(
                                div()
                                    .line_through()
                                    .text_decoration_1()
                                    .child("Strikethrough text"),
                            )
                            .child(div().italic().child("Italic text")),
                    )
                    .on_hover(self.hover_info(&fi, "Text Decorations", &[("text", "foreground", t.foreground, "gpui-component/root.rs:596")], &[], &[("styles", "bold / underline / strikethrough / italic")])),
            )
            // Kbd
            .child(section("Kbd (keyboard shortcuts)"))
            .child(
                div()
                    .id("tt-kbd")
                    .child(
                        h_flex().gap_4().items_center().children(
                            ["cmd-c", "cmd-v", "cmd-shift-p", "ctrl-z"]
                                .iter()
                                .filter_map(|k| Keystroke::parse(k).ok())
                                .map(Kbd::new),
                        ),
                    )
                    .on_hover(self.hover_info(&fi, "Kbd", &[("bg", "muted", t.muted, "gpui-component/kbd.rs:238"), ("text", "muted_foreground", t.muted_foreground, "gpui-component/kbd.rs:237")], &[("border-radius", format!("radius / 2: {}px", t.radius.as_f32() / 2.0))], &[("edge", "none: only an outline Kbd draws one, in border, and these are not outlined (kbd.rs, Kbd)"), ("padding", "py_0p5 and px_1 (kbd.rs, Kbd) -- rems, so already proportional to the platform font"), ("font", "not monospace, and not set at all: kbd.rs states a text size and a colour and no family (kbd.rs, Kbd), so a Kbd inherits the window's -- the platform UI font Root::render applies. The panel claimed a family nothing produces")])),
            )
            // Muted / mono text
            .child(section("Muted & Monospace Text"))
            .child(
                div()
                    .id("tt-muted-mono")
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(t.muted_foreground)
                                    .child("Muted text (secondary content)"),
                            )
                            .child(
                                div()
                                    .font_family(t.mono_font_family.clone())
                                    .child("Monospace text (code / technical content)"),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Muted & Mono", &[("muted", "muted_foreground", t.muted_foreground, "gpui-component/label.rs:171"), ("text", "foreground", t.foreground, "gpui-component/root.rs:596")], &[(
                            "mono font",
                            format!("mono_font_family: {}", t.mono_font_family),
                        )], &[])),
            )
            // Code editor
            .child(section("Code editor (Rust)"))
            .child(
                div()
                    .id("tt-code-editor")
                    .occlude()
                    .child(Editor::new(&self.editor_state).h(px(240.0)))
                    .on_hover(self.hover_info(&fi, "Editor", &[("edge", "input", t.input, "gpui-component/input/input.rs:714"), ("indent guides", "border", t.border, "gpui-base/input/base/element.rs:2319"), ("text", "foreground", t.foreground, "gpui-component/input/input.rs:498"), ("caret", "caret", t.caret, "gpui-component/input/input.rs:503"), ("selection", "selection", t.selection, "gpui-component/input/input.rs:502"), ("line numbers", "muted_foreground", t.muted_foreground, "gpui-component/input/input.rs:499")], &[
                            (
                                "mono font",
                                format!("mono_font_family: {}", t.mono_font_family),
                            ),
                            (
                                "mono size",
                                format!("mono_font_size: {}px (gpui renders)", t.mono_font_size.as_f32()),
                            ),
                        ], &[
                            ("bg", "#0a0a0a in dark and #ffffff in light, whatever the platform: the connector installs upstream's default highlight themes, and their editor.background wins over the input_background() fallback (theme/mod.rs, editor_background; native-theme-gpui/lib.rs, to_theme). The current line is filled from the same theme, #171717 or #F5F5F5. The fields are public, so this one is ours"),
                            ("syntax colors", "highlight_theme: default_light / default_dark per color mode; the grammar comes from the tree-sitter-rust dev feature"),
                            ("style block", "the editor takes one InputEditorStyle built in a single place (input/input.rs, set_editor_style), so its colours cite the same block. Its border paints the indent guides; the edge is the bordered Input's own input token (input/editor.rs, Editor::render)"),
                            ("line height", "1.5 is only what the widget sets first: Editor is Styled and applies the caller's refinement last, on purpose (input/editor.rs, Editor::render -- the comment there says a text style set on the editor refines over them). defaults.line_height is modelled (1.4 on the bundled defaults) and no builder carries it yet"),
                            ("line numbers / search", "on by default (gpui-base/input/base/state.rs, EditorMode)"),
                        ])),
            )
            // Markdown
            .child(section("Markdown"))
            .child(
                div()
                    .id("tt-markdown")
                    .child(TextView::markdown("markdown-sample", MARKDOWN_SAMPLE).selectable(true))
                    .on_hover(self.hover_info(&fi, "Markdown (TextView)", &[("text", "foreground", t.foreground, "gpui-component/text/mod.rs:48"), ("link", "link", t.link, "gpui-component/text/mod.rs:50"), ("code block bg", "muted", t.muted, "gpui-component/text/mod.rs:52"), ("inline code bg", "accent", t.accent, "gpui-component/text/mod.rs:58"), ("table border", "border", t.border, "gpui-component/text/mod.rs:53"), ("table head", "table_head", t.table_head, "gpui-component/text/mod.rs:44"), ("table head text", "table_head_foreground", t.table_head_foreground, "gpui-component/text/mod.rs:45")], &[
                            (
                                "border-radius",
                                format!("radius: {}px", t.radius.as_f32()),
                            ),
                            (
                                "mono font",
                                format!("mono_font_family: {}", t.mono_font_family),
                            ),
                        ], &[("style source", "a TextView reads no theme field itself: gpui-component builds a TextViewStyle from the theme once and the view takes it (text/mod.rs)"),
                            ("inline code", "accent again -- the menu highlight, shared with menu rows, Toggle and a hovered ghost button (Tier U)"),
                            ("heading sizes", "2, 1.5, 1.25, 1.125, 1 and 1 times a fixed 14px, not the platform's font: base_text_view_style never sets heading_base_font_size (text/mod.rs, base_text_view_style), so gpui-base's default stands (gpui-base/text/style.rs, heading_base_font_size; gpui-base/text/node.rs, BlockNode::Heading). The weights are literals too, bold down to medium")])),
            )
    }
}
