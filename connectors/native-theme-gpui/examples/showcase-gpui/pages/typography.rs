//! The Typography page.

use gpui::{Context, IntoElement, ParentElement, Styled, prelude::*, px};
use gpui_component::{h_flex, v_flex};

use crate::app::Showcase;
use crate::demo::{self, DecorationKind, HeadingLevel, LabelKind, TextSize, WeightKind};
use crate::support::MARKDOWN_SAMPLE;
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

/// The Kbds, as `(id, keys)`.
const KBDS: [(&str, &str); 4] = [
    ("typography-kbd-copy", "cmd-c"),
    ("typography-kbd-paste", "cmd-v"),
    ("typography-kbd-palette", "cmd-shift-p"),
    ("typography-kbd-undo", "ctrl-z"),
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
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            .child(demo::heading(ui, cx, "typography-heading-label", "Label"))
            .child(
                v_flex()
                    .gap_2()
                    .children(
                        LABELS.map(|(id, kind, text)| demo::gallery_label(ui, cx, id, kind, text)),
                    )
                    // Only the secret is masked: the words naming it are page
                    // text beside it.
                    .child(
                        h_flex()
                            .gap_2()
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
            .child(
                h_flex()
                    .gap_4()
                    .children(LINKS.map(|(id, text, href)| demo::link(ui, cx, id, text, href))),
            )
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-headings",
                "Headings (H1–H6)",
            ))
            .child(v_flex().gap_1().children(
                HEADINGS.map(|(id, level, text)| demo::heading_level(ui, cx, id, level, text)),
            ))
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-weights",
                "Font Weights (Thin → Black)",
            ))
            .child(
                v_flex()
                    .gap_1()
                    .children(WEIGHTS.map(|(id, weight)| demo::weight_sample(ui, cx, id, weight))),
            )
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-sizes",
                "Font Sizes (XS → XL)",
            ))
            .child(
                v_flex().gap_1().children(
                    SIZES.map(|(id, size, text)| demo::sized_label(ui, cx, id, size, text)),
                ),
            )
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-decorations",
                "Text Decorations",
            ))
            .child(
                v_flex()
                    .gap_1()
                    .children(DECORATIONS.map(|(id, decoration, text)| {
                        demo::decoration_sample(ui, cx, id, decoration, text)
                    })),
            )
            .child(demo::heading(
                ui,
                cx,
                "typography-heading-kbd",
                "Kbd (keyboard shortcuts)",
            ))
            .child(
                h_flex().gap_4().items_center().children(
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
                v_flex()
                    .gap_2()
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
            .child(
                demo::editor(ui, cx, "typography-editor", &self.editor_state, px(240.0)).occlude(),
            )
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
    }
}
