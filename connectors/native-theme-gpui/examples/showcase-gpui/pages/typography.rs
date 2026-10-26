//! The Typography page.

use gpui::{Context, IntoElement, ParentElement, Styled, prelude::*, px};
use gpui_component::{h_flex, v_flex};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{self, DecorationKind, HeadingLevel, LabelKind, TextSize, WeightKind};
use crate::pages::page_root;
use crate::support::{MARKDOWN_SAMPLE, with_gap};
use crate::{TYPOGRAPHY_H1, TYPOGRAPHY_H2, TYPOGRAPHY_LABEL_PLAIN, TYPOGRAPHY_LABEL_SECONDARY};

/// The Labels above the masked one, as `(id, kind, text)`.
const LABELS: [(&str, LabelKind, &str); 2] = [
    (TYPOGRAPHY_LABEL_PLAIN, LabelKind::Plain, "Regular label"),
    (
        TYPOGRAPHY_LABEL_SECONDARY,
        LabelKind::Secondary("(secondary text)"),
        "Label with secondary",
    ),
];

/// The Links, as `(id, text, target)`.
const LINKS: [(&str, &str, &str); 2] = [
    (
        "typography-link-docs",
        "Visit Documentation",
        "https://github.com",
    ),
    ("typography-link-other", "Another Link", "https://gpui.rs"),
];

/// The heading ladder, as `(id, level, text)`.
const HEADINGS: [(&str, HeadingLevel, &str); 6] = [
    (TYPOGRAPHY_H1, HeadingLevel::H1, "H1 — Page Title"),
    (TYPOGRAPHY_H2, HeadingLevel::H2, "H2 — Section"),
    ("typography-h3", HeadingLevel::H3, "H3 — Subsection"),
    ("typography-h4", HeadingLevel::H4, "H4 — Group"),
    ("typography-h5", HeadingLevel::H5, "H5 — Detail"),
    ("typography-h6", HeadingLevel::H6, "H6 — Fine Print"),
];

/// The weights, as `(id, weight)`.
const WEIGHTS: [(&str, WeightKind); 9] = [
    ("typography-weight-thin", WeightKind::Thin),
    ("typography-weight-extra-light", WeightKind::ExtraLight),
    ("typography-weight-light", WeightKind::Light),
    ("typography-weight-normal", WeightKind::Normal),
    ("typography-weight-medium", WeightKind::Medium),
    ("typography-weight-semibold", WeightKind::Semibold),
    ("typography-weight-bold", WeightKind::Bold),
    ("typography-weight-extra-bold", WeightKind::ExtraBold),
    ("typography-weight-black", WeightKind::Black),
];

/// The sized Labels, as `(id, size, text)`.
const SIZES: [(&str, TextSize, &str); 5] = [
    ("typography-size-xs", TextSize::Xs, "text_xs — Extra Small"),
    ("typography-size-sm", TextSize::Sm, "text_sm — Small"),
    (
        "typography-size-base",
        TextSize::Base,
        "text_base — Base (default)",
    ),
    ("typography-size-lg", TextSize::Lg, "text_lg — Large"),
    ("typography-size-xl", TextSize::Xl, "text_xl — Extra Large"),
];

/// The decorated samples, as `(id, decoration, text)`.
const DECORATIONS: [(&str, DecorationKind, &str); 4] = [
    (
        "typography-decoration-bold",
        DecorationKind::Bold,
        "Bold text",
    ),
    (
        "typography-decoration-underline",
        DecorationKind::Underline,
        "Underlined text",
    ),
    (
        "typography-decoration-strikethrough",
        DecorationKind::Strikethrough,
        "Strikethrough text",
    ),
    (
        "typography-decoration-italic",
        DecorationKind::Italic,
        "Italic text",
    ),
];

/// The Kbds, as `(id, keys)`: samples a Kbd draws, not the showcase's key
/// bindings, each on gpui's `secondary` modifier, the one those bindings
/// take (`app::init`), Ctrl off macOS and Cmd on it.
///
/// `secondary` is resolved with gpui's runtime OS (`demo::kbd`), and
/// `Kbd::format` draws the keystroke by the compile target (gpui-component
/// kbd.rs:105-223); the two agree on every native target.
pub(crate) const KBDS: [(&str, &str); 4] = [
    ("typography-kbd-copy", "secondary-c"),
    ("typography-kbd-paste", "secondary-v"),
    ("typography-kbd-palette", "secondary-shift-p"),
    ("typography-kbd-undo", "secondary-z"),
];

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Typography
    // -----------------------------------------------------------------------
    pub(crate) fn render_typography_page(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let ui = &self.info_ui;
        let widget_gap = geometry::widget_gap(&self.layout);
        // Samples one over another, and side by side, `layout.widget_gap`
        // apart, as the Basic page's typography lines are.
        let column = || with_gap(v_flex(), widget_gap);
        let row = || with_gap(h_flex(), widget_gap);
        page_root(&self.layout)
            .flex_1()
            .child(demo::heading(ui, cx, "typography-heading-label", "Label"))
            .child(
                column()
                    .children(
                        LABELS.map(|(id, kind, text)| demo::gallery_label(ui, cx, id, kind, text)),
                    )
                    // Only the secret is masked: the words naming it are page
                    // text beside it.
                    .child(
                        row()
                            .items_center()
                            .child(demo::label(
                                ui,
                                cx,
                                "typography-label-masked-caption",
                                "Masked label:",
                            ))
                            .child(demo::gallery_label(
                                ui,
                                cx,
                                "typography-label-masked",
                                LabelKind::Masked,
                                "secret123",
                            )),
                    ),
            )
            .child(demo::heading(ui, cx, "typography-heading-link", "Link"))
            .child(row().children(LINKS.map(|(id, text, href)| demo::link(ui, cx, id, text, href))))
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-headings",
                "Headings (H1–H6)",
            ))
            .child(column().children(
                HEADINGS.map(|(id, level, text)| demo::heading_level(ui, cx, id, level, text)),
            ))
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-weights",
                "Font Weights (Thin → Black)",
            ))
            .child(
                column()
                    .children(WEIGHTS.map(|(id, weight)| demo::weight_sample(ui, cx, id, weight))),
            )
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-sizes",
                "Font Sizes (XS → XL)",
            ))
            .child(
                column().children(
                    SIZES.map(|(id, size, text)| demo::sized_label(ui, cx, id, size, text)),
                ),
            )
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-decorations",
                "Text Decorations",
            ))
            .child(column().children(DECORATIONS.map(|(id, decoration, text)| {
                demo::decoration_sample(ui, cx, id, decoration, text)
            })))
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-kbd",
                "Kbd (keyboard shortcuts)",
            ))
            .child(
                row().items_center().children(
                    KBDS.iter()
                        .filter_map(|&(id, keys)| demo::kbd(ui, cx, id, keys)),
                ),
            )
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-muted-mono",
                "Muted & Monospace Text",
            ))
            .child(
                column()
                    .child(demo::muted_text(
                        ui,
                        cx,
                        "typography-muted",
                        "Muted text (secondary content)",
                    ))
                    .child(demo::mono_text(
                        ui,
                        cx,
                        "typography-mono",
                        "Monospace text (code / technical content)",
                    )),
            )
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-editor",
                "Code editor (Rust)",
            ))
            .child(demo::editor(
                ui,
                cx,
                "typography-editor",
                &self.editor_state,
                px(240.0),
            ))
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-markdown",
                "Markdown",
            ))
            .child(demo::markdown(
                ui,
                cx,
                "typography-markdown",
                MARKDOWN_SAMPLE,
            ))
            .child(demo::heading(ui, cx, "typography-heading-diff", "Diff"))
            .child(demo::diff(
                ui,
                cx,
                "typography-diff",
                &self.diff_state,
                px(240.0),
            ))
    }
}
