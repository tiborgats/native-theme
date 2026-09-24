//! The Data page.

use gpui::{Context, IntoElement, ParentElement, SharedString, Styled, Window, prelude::*, px};
use gpui_component::{IconName, attachment::AttachmentStatus, h_flex, v_flex};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{self, BubbleKind, DemoAttachment, DemoPagination};
use crate::support::{ChatMessage, PAGE_COUNT, next_attachment_status, with_gap};
use crate::{
    DATA_PAGINATION, DATA_PAGINATION_COMPACT, PROBE_ATTACHMENT, PROBE_CHAT_SEND, PROBE_PAGINATION,
    probe,
};

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Data
    // -----------------------------------------------------------------------
    pub(crate) fn render_data_page(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let ui = &self.info_ui;
        let widget_gap = geometry::widget_gap(&self.layout);
        let on_page = cx.listener(|this, page: &usize, _w, cx| {
            this.page = *page;
            cx.notify();
        });
        let on_page_compact = cx.listener(|this, page: &usize, _w, cx| {
            this.page = *page;
            cx.notify();
        });
        let on_send = cx.listener(|this, _ev, _w, cx| {
            this.chat_messages.push(ChatMessage {
                outgoing: true,
                sender: "You".into(),
                text: "Dark mode follows the desktop too.".into(),
            });
            this.chat_scroller.update(cx, |state, cx| {
                state.append(1, cx);
            });
            cx.notify();
        });
        let on_attachment = cx.listener(|this, _ev, _w, cx| {
            this.attachment_status = next_attachment_status(this.attachment_status);
            cx.notify();
        });

        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            .child(demo::heading(
                ui,
                cx,
                "data-heading-description-list",
                "DescriptionList",
            ))
            .child(demo::description_list(
                ui,
                cx,
                "data-description-list",
                2,
                &[
                    ("Name", "native-theme", 1),
                    ("Version", env!("CARGO_PKG_VERSION"), 1),
                    ("License", env!("CARGO_PKG_LICENSE"), 1),
                    ("Platforms", "Linux, macOS, Windows", 1),
                    ("Description", "Universal theme abstraction layer", 2),
                ],
            ))
            .child(demo::heading(
                ui,
                cx,
                "data-heading-table",
                "Table (striped, 3 cols × 5 rows)",
            ))
            .child(demo::data_table(
                ui,
                cx,
                "data-table",
                &self.table_state,
                px(220.0),
            ))
            // The other table: rows written out instead of driven by a
            // delegate. This one is `Styled` (`table/table.rs:84`), so it is
            // the receiver `geometry::table` documents.
            .child(demo::heading(
                ui,
                cx,
                "data-heading-table-declarative",
                "Table (declarative)",
            ))
            .child(demo::table(
                ui,
                cx,
                "data-table-declarative",
                "Theme sources",
                ["Field", "Source"],
                &[
                    ["radius", "defaults.border.corner_radius"],
                    ["font", "defaults.font.family"],
                    ["row text", "list.item_font"],
                ],
            ))
            .child(demo::heading(
                ui,
                cx,
                "data-heading-pagination",
                format!("Pagination (page {} of {})", self.page, PAGE_COUNT),
            ))
            .child(
                with_gap(v_flex(), widget_gap)
                    .child(
                        probe(
                            PROBE_PAGINATION,
                            demo::pagination(
                                ui,
                                cx,
                                DemoPagination {
                                    id: DATA_PAGINATION,
                                    page: self.page,
                                    pages: PAGE_COUNT,
                                    compact: false,
                                    gap: widget_gap,
                                },
                                on_page,
                            ),
                        )
                        .self_start(),
                    )
                    .child(demo::caption(
                        ui,
                        cx,
                        "data-pagination-rows",
                        SharedString::from(format!(
                            "Rows {}–{} of {}",
                            (self.page - 1) * 10 + 1,
                            self.page * 10,
                            PAGE_COUNT * 10
                        )),
                    ))
                    .child(
                        demo::pagination(
                            ui,
                            cx,
                            DemoPagination {
                                id: DATA_PAGINATION_COMPACT,
                                page: self.page,
                                pages: PAGE_COUNT,
                                compact: true,
                                gap: None,
                            },
                            on_page_compact,
                        )
                        .self_start(),
                    ),
            )
            .child(demo::heading(
                ui,
                cx,
                "data-heading-list",
                "List (selectable)",
            ))
            .child(demo::list(
                ui,
                cx,
                "data-list",
                &self.list_state,
                px(260.0),
                px(200.0),
            ))
            .child(demo::heading(
                ui,
                cx,
                "data-heading-tree",
                "Tree (file structure)",
            ))
            .child(demo::tree(
                ui,
                cx,
                "data-tree",
                &self.tree_state,
                px(260.0),
                px(200.0),
            ))
            .child(demo::heading(
                ui,
                cx,
                "data-heading-avatar",
                "Avatar & AvatarGroup",
            ))
            .child(
                h_flex()
                    .gap_6()
                    .items_center()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(demo::avatar(ui, "data-avatar-alice", "Alice"))
                            .child(demo::avatar(ui, "data-avatar-bob", "Bob"))
                            .child(demo::avatar(ui, "data-avatar-carol", "Carol")),
                    )
                    .child(demo::avatar_group(
                        ui,
                        "data-avatar-group",
                        &["D", "E", "F", "G"],
                        3,
                    )),
            )
            .child(demo::heading(
                ui,
                cx,
                "data-heading-bubble",
                "Bubble (all 7 variants, incoming and outgoing)",
            ))
            .child(
                with_gap(v_flex().w(px(420.0)), widget_gap)
                    .child(demo::bubble(
                        ui,
                        cx,
                        "data-bubble-muted",
                        BubbleKind::Muted,
                        false,
                        "Incoming, Muted",
                    ))
                    .child(demo::bubble(
                        ui,
                        cx,
                        "data-bubble-filled",
                        BubbleKind::Filled,
                        true,
                        "Outgoing, Filled",
                    ))
                    .child(demo::bubble(
                        ui,
                        cx,
                        "data-bubble-secondary",
                        BubbleKind::Secondary,
                        false,
                        "Secondary",
                    ))
                    .child(demo::bubble(
                        ui,
                        cx,
                        "data-bubble-tinted",
                        BubbleKind::Tinted,
                        true,
                        "Tinted",
                    ))
                    .child(demo::bubble(
                        ui,
                        cx,
                        "data-bubble-outline",
                        BubbleKind::Outline,
                        false,
                        "Outline",
                    ))
                    .child(demo::bubble(
                        ui,
                        cx,
                        "data-bubble-ghost",
                        BubbleKind::Ghost,
                        false,
                        "Ghost: no surface, no padding",
                    ))
                    .child(demo::bubble(
                        ui,
                        cx,
                        "data-bubble-destructive",
                        BubbleKind::Destructive,
                        true,
                        "Destructive: this one failed to send",
                    )),
            )
            .child(demo::heading(
                ui,
                cx,
                "data-heading-message",
                "Message (avatar, bubble, both alignments)",
            ))
            .child(
                with_gap(v_flex().w(px(420.0)), widget_gap).children(
                    self.chat_messages
                        .iter()
                        .take(2)
                        .enumerate()
                        .map(|(ix, msg)| demo::message(ui, cx, format!("data-message-row-{ix}"), msg)),
                ),
            )
            .child(demo::heading(
                ui,
                cx,
                "data-heading-message-scroller",
                format!(
                    "MessageScroller (virtualised thread, {} messages)",
                    self.chat_messages.len()
                ),
            ))
            .child(
                with_gap(v_flex(), widget_gap)
                    .w(px(460.0))
                    .child(demo::message_scroller(
                        ui,
                        cx,
                        "data-message-scroller",
                        &self.chat_scroller,
                        &self.chat_messages,
                        px(240.0),
                    ))
                    .child(
                        probe(
                            PROBE_CHAT_SEND,
                            demo::action_button(
                                ui,
                                cx,
                                "data-chat-send",
                                "Send a reply",
                                "appends a reply to the thread, which the scroller follows to its end",
                                on_send,
                            ),
                        )
                        .self_start(),
                    ),
            )
            .child(demo::heading(
                ui,
                cx,
                "data-heading-attachment",
                format!(
                    "Attachment (complete, uploading, and one at {:?} — click it)",
                    self.attachment_status
                ),
            ))
            .child(
                with_gap(h_flex().flex_wrap(), widget_gap)
                    .child(demo::attachment(
                        ui,
                        cx,
                        DemoAttachment {
                            id: "data-attachment-complete",
                            status: AttachmentStatus::Complete,
                            icon: self.sample_icon(IconName::Inbox),
                            file: "platform-facts.md",
                            description: "48 KB".into(),
                        },
                        None::<fn(&gpui::ClickEvent, &mut Window, &mut gpui::App)>,
                    ))
                    .child(demo::attachment(
                        ui,
                        cx,
                        DemoAttachment {
                            id: "data-attachment-uploading",
                            status: AttachmentStatus::Uploading,
                            icon: self.sample_icon(IconName::Copy),
                            file: "breeze-palette.png",
                            description: "uploading…".into(),
                        },
                        None::<fn(&gpui::ClickEvent, &mut Window, &mut gpui::App)>,
                    ))
                    .child(probe(
                        PROBE_ATTACHMENT,
                        demo::attachment(
                            ui,
                            cx,
                            DemoAttachment {
                                id: "data-attachment-cycle",
                                status: self.attachment_status,
                                icon: self.sample_icon(IconName::Settings),
                                file: "kdeglobals",
                                description: SharedString::from(format!("{:?}", self.attachment_status)),
                            },
                            Some(on_attachment),
                        ),
                    )),
            )
    }
}
