//! The Data tab.

use gpui::{
    Context, IntoElement, ParentElement, SharedString, Styled, Window, div, prelude::*, px,
};
use gpui_component::{
    ActiveTheme, IconName,
    attachment::{
        Attachment, AttachmentContent, AttachmentDescription, AttachmentMedia, AttachmentStatus,
        AttachmentTitle,
    },
    avatar::{Avatar, AvatarGroup},
    bubble::{Bubble, BubbleVariant},
    button::Button,
    description_list::DescriptionList,
    h_flex,
    label::Label,
    list::ListItem,
    message::MessageAlignment,
    message_scroller::MessageScroller,
    pagination::Pagination,
    table::{DataTable, Table, TableBody, TableCell, TableHead, TableHeader, TableRow},
    tree::Tree,
    v_flex,
};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::support::{
    ChatMessage, NativeStyled, PAGE_COUNT, chat_message, format_font_info, native_icon,
    next_attachment_status, section, with_gap,
};
use crate::{LIST_DEMO, PROBE_ATTACHMENT, PROBE_CHAT_SEND, PROBE_PAGINATION, TREE_DEMO, probe};

impl Showcase {
    // -----------------------------------------------------------------------
    // Tab: Data
    // -----------------------------------------------------------------------
    pub(crate) fn render_data_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let widget_gap = geometry::widget_gap(&self.layout);
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Description list
            .child(section("DescriptionList"))
            .child(
                div()
                    .id("tt-desclist")
                    .child(
                        DescriptionList::new()
                            .columns(2)
                            .item("Name", "native-theme", 1)
                            .item("Version", "0.1.0", 1)
                            .item("License", "MIT OR Apache-2.0 OR BSD-0", 1)
                            .item("Platforms", "Linux, macOS, Windows", 1)
                            .item("Description", "Universal theme abstraction layer", 2),
                    )
                    .on_hover(self.hover_info(&fi, "DescriptionList", &[("label bg", "description_list_label", t.description_list_label, "gpui-component/description_list.rs:333"), ("label text", "description_list_label_foreground", t.description_list_label_foreground, "gpui-component/description_list.rs:317"), ("border", "border", t.border, "gpui-component/description_list.rs:288")], &[], &[("layout spacing", "per Size, in literal px (description_list.rs, DescriptionList), and the label column defaults to 120px -- a width DescriptionList::label_width does take"),])),
            )
            // Table
            .child(section("Table (striped, 3 cols × 5 rows)"))
            .child(
                div()
                    .id("tt-table")
                    .h(px(220.0))
                    .occlude()
                    .child(
                        DataTable::new(&self.table_state)
                            .stripe(true)
                            .bordered(true),
                    )
                    .on_hover(self.hover_info(&fi, "Table", &[("frame", "border", t.border, "gpui-component/table/data_table.rs:171"), ("header bg", "table_head", t.table_head, "gpui-component/table/state.rs:1768"), ("header text", "table_head_foreground", t.table_head_foreground, "gpui-component/table/state.rs:1769"), ("row bg", "table", t.table, "gpui-component/table/data_table.rs:167"), ("stripe", "table_even", t.table_even, "gpui-component/table/state.rs:1980"), ("active row", "table_active", t.table_active, "gpui-component/table/state.rs:2196"), ("hover", "table_hover", t.table_hover, "gpui-component/table/state.rs:1986"), ("row border", "table_row_border", t.table_row_border, "gpui-component/table/state.rs:1978")], &[("geometry", "DataTable is not Styled (table/data_table.rs: DataTable impls \
                                 Sizable and RenderOnce, not Styled); \
                                 geometry::table goes to the declarative Table below".to_string())], &[
                            ("row height", "per Size, but Size::Size(px) is an escape hatch that returns the pixel value verbatim (sizing.rs, table_row_height) while table_cell_padding has no Size::Size arm and stays on the Medium edges this demo already uses -- so DataTable::with_size(Size::Size(px(list.row_height))) would carry the platform's row height and change nothing else. list.row_height is modelled; nothing applies it here yet"),
                        ])),
            )
            // The other table: rows written out instead of driven by a
            // delegate. This one is `Styled` (`table/table.rs:84`), so it is
            // the receiver `geometry::table` documents.
            .child(section("Table (declarative)"))
            .child(
                div()
                    .id("tt-table-declarative")
                    .child(
                        Table::new()
                            .native(cx, geometry::table)
                            .accessibility_label("Theme sources")
                            .child(
                                TableHeader::new().child(
                                    TableRow::new()
                                        .child(TableHead::new().child("Field"))
                                        .child(TableHead::new().child("Source")),
                                ),
                            )
                            .child(
                                TableBody::new()
                                    .child(
                                        TableRow::new()
                                            .child(TableCell::new().child("radius"))
                                            .child(TableCell::new().child(
                                                "defaults.border.corner_radius",
                                            )),
                                    )
                                    .child(
                                        TableRow::new()
                                            .child(TableCell::new().child("font"))
                                            .child(
                                                TableCell::new().child("defaults.font.family"),
                                            ),
                                    )
                                    .child(
                                        TableRow::new()
                                            .child(TableCell::new().child("row text"))
                                            .child(TableCell::new().child("list.item_font")),
                                    ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Table (declarative)", &[("bg", "table", t.table, "gpui-component/table/table.rs:113"), ("header bg", "table_head", t.table_head, "gpui-component/table/table.rs:199"), ("header text", "table_head_foreground", t.table_head_foreground, "gpui-component/table/table.rs:200"), ("row border", "table_row_border", t.table_row_border, "gpui-component/table/table.rs:203")], &[("geometry", "geometry::table: list.item_font on the table root (table/table.rs, Table::render: text_sm then refine_style)".to_string())], &[
                            ("cell padding", "inner (Tier U)"),
                        ])),
            )
            // Pagination
            .child(section(format!(
                "Pagination (page {} of {})",
                self.page, PAGE_COUNT
            )))
            .child(
                div()
                    .id("tt-pagination")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .child(probe(
                                PROBE_PAGINATION,
                                with_gap(
                                    Pagination::new("pagination-1")
                                    .current_page(self.page)
                                    .total_pages(PAGE_COUNT)
                                    .on_click(cx.listener(|this, page: &usize, _w, cx| {
                                        this.page = *page;
                                        cx.notify();
                                    })),
                                    widget_gap,
                                ),
                            ))
                            .child(
                                Label::new(SharedString::from(format!(
                                    "Rows {}–{} of {}",
                                    (self.page - 1) * 10 + 1,
                                    self.page * 10,
                                    PAGE_COUNT * 10
                                )))
                                .text_sm()
                                .text_color(t.muted_foreground),
                            )
                            .child(
                                Pagination::new("pagination-compact")
                                    .compact()
                                    .current_page(self.page)
                                    .total_pages(PAGE_COUNT)
                                    .on_click(cx.listener(|this, page: &usize, _w, cx| {
                                        this.page = *page;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Pagination", &[("hover", "accent", t.accent, "gpui-component/button/button.rs:1126"), ("text", "secondary_foreground", t.secondary_foreground, "gpui-component/button/button.rs:964"), ("current page text", "button_foreground", t.button_foreground, "gpui-component/button/button.rs:949"), ("current page edge", "input", t.input, "gpui-component/button/button.rs:1001")], &[("gap", "geometry::widget_gap on the row; upstream's own is gap_1 (pagination.rs, Pagination::render)".to_string())], &[
                            ("buttons", "built by the widget as ghost/outline Button (pagination.rs, Pagination::render page items); no refinement reaches them"),
                            ("other pages", "no fill until hovered: a ghost Button is transparent, and it then hovers with accent -- the menu highlight, halved in dark mode (button/button.rs, ButtonVariant::hovered Ghost arm)"),
                            ("ellipsis", "a ghost Button whose dropdown lists the hidden pages (pagination.rs, PageItem::Ellipsis)"),
                        ])),
            )
            // List
            .child(section("List (selectable)"))
            .child(
                div()
                    .id("tt-list")
                    .h(px(200.0))
                    .w(px(260.0))
                    // Upstream's List paints no frame of its own, so this box
                    // is the list's frame and shows the list theme's border --
                    // the same one the DataTable above draws for itself.
                    .native(cx, geometry::list)
                    // gpui's scroll listeners run in the bubble phase and stop
                    // at no one, so without this the page under the List
                    // scrolls by the same delta (div.rs, paint_scroll_listener;
                    // window.rs, HitboxBehavior::BlockMouse). Every demo box
                    // that holds a scroller of its own carries it.
                    .occlude()
                    .debug_selector(|| LIST_DEMO.into())
                    .child(gpui_component::list::List::new(&self.list_state))
                    .on_hover(self.hover_info(&fi, "List", &[("active", "list_active", t.list_active, "gpui-component/list/list_item.rs:237"), ("hover", "list_hover", t.list_hover, "gpui-component/list/list_item.rs:209")], &[("geometry", "geometry::list on the box around it: list.border line width, colour and corner radius, and a clip to that radius. List paints no frame of its own (list/list.rs, RenderOnce for List), so the frame is the application's".to_string()), ("row geometry", "geometry::list_item on each row: list.row_height (control height), list.border.padding_*, and list.item_font including its colour. Applied in the delegate rather than in the demo block, which is why the builder-coverage test cannot see it here (showcase-gpui/support.rs, SampleListDelegate::render_item)".to_string())], &[("fill", "none from a token: ThemeColor::list has no reader, and a ListItem paints no idle background (list/list_item.rs, ListItem::render). geometry::list lands on the element around the rows and carries list.border but not list.background_color, which it could -- our gap"), 
                            ("even rows", "the list_even token has no reader anywhere in gpui-component or gpui-base: a List paints every row the same, and the connector writes the slot for nothing (Tier U)"),
                        ])),
            )
            // Tree
            .child(section("Tree (file structure)"))
            .child(
                div()
                    .id("tt-tree")
                    .h(px(200.0))
                    .w(px(260.0))
                    // A tree is a list view: the model gives it no theme of
                    // its own, so its frame is the list's (geometry::list).
                    .native(cx, geometry::list)
                    .occlude()
                    .debug_selector(|| TREE_DEMO.into())
                    .child(Tree::new(
                        &self.tree_state,
                        |ix, entry, selected, _w, cx| {
                            ListItem::new(("tree-item", ix))
                                .native(cx, geometry::list_item)
                                .child(Label::new(entry.item().label.clone()).text_sm())
                                .selected(selected)
                        },
                    ))
                    .on_hover(self.hover_info(&fi, "Tree", &[("active", "list_active", t.list_active, "gpui-component/list/list_item.rs:237"), ("hover", "list_hover", t.list_hover, "gpui-component/list/list_item.rs:209")], &[("geometry", "geometry::list on the box around it: a tree is a list view and the model gives it no theme of its own, so its frame is the list's".to_string()), ("row geometry", "geometry::list_item on each row: list.row_height (control height), list.border.padding_*, and list.item_font including its colour -- upstream labels the row with foreground one line before applying it (list/list_item.rs, ListItem::render)".to_string())], &[("fill", "none from a token: ThemeColor::list has no reader, and a ListItem paints no idle background (list/list_item.rs, ListItem::render). geometry::list lands on the element around the rows and carries list.border but not list.background_color, which it could -- our gap"), 
                            ("indent", "none here, and none upstream: Tree::new takes a render_item closure and tree.rs draws no row content of its own (tree.rs, Tree), so a row's indent is whatever the application's closure applies -- this demo's applies none"),
                            ("disclosure icon", "the same: tree.rs contains no icon, so a chevron would be the closure's to draw. The panel used to claim a hardcoded ChevronRight, which is in neither this demo nor upstream"),
                        ])),
            )
            // Avatar & AvatarGroup
            .child(section("Avatar & AvatarGroup"))
            .child(
                div()
                    .id("tt-avatar")
                    .child(
                        h_flex()
                            .gap_6()
                            .items_center()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Avatar::new().name("Alice"))
                                    .child(Avatar::new().name("Bob"))
                                    .child(Avatar::new().name("Carol")),
                            )
                            .child(
                                AvatarGroup::new()
                                    .child(Avatar::new().name("D"))
                                    .child(Avatar::new().name("E"))
                                    .child(Avatar::new().name("F"))
                                    .child(Avatar::new().name("G"))
                                    .limit(3),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Avatar / AvatarGroup", &[], &[], &[
                            ("colours", "none from the theme: a named Avatar takes its fill, initials and edge from three OKLCH literals picked by a hash of its initials -- twelve hues, a light and a dark set -- and only the mode is the theme's (avatar/avatar.rs, IdentityColor::from_hue). secondary, background and border paint an avatar with no name and no image, which this demo does not show"),
                            ("size", "configurable via Size enum"),
                            ("limit", "three shown and the fourth dropped with no marker: the overflow marker is an Avatar named ⋯, not a +N count, and only AvatarGroup::ellipsis adds it, which this demo does not call (avatar/avatar_group.rs, AvatarGroup::ellipsis)"),
                        ])),
            )
            // Bubble
            .child(section("Bubble (all 7 variants, incoming and outgoing)"))
            .child(
                div()
                    .id("tt-bubble")
                    .child(
                        with_gap(v_flex().w(px(420.0)), widget_gap)
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::Start)
                                    .with_variant(BubbleVariant::Muted)
                                    .child("Incoming, Muted"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::End)
                                    .with_variant(BubbleVariant::Filled)
                                    .child("Outgoing, Filled"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::Start)
                                    .with_variant(BubbleVariant::Secondary)
                                    .child("Secondary"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::End)
                                    .with_variant(BubbleVariant::Tinted)
                                    .child("Tinted"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::Start)
                                    .with_variant(BubbleVariant::Outline)
                                    .child("Outline"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::Start)
                                    .with_variant(BubbleVariant::Ghost)
                                    .child("Ghost: no surface, no padding"),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::End)
                                    .with_variant(BubbleVariant::Destructive)
                                    .child("Destructive: this one failed to send"),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Bubble", &[("filled bg", "primary", t.primary, "gpui-component/bubble.rs:211"), ("filled text", "primary_foreground", t.primary_foreground, "gpui-component/bubble.rs:212"), ("muted bg", "muted", t.muted, "gpui-component/bubble.rs:218"), ("muted text", "secondary_foreground", t.secondary_foreground, "gpui-component/bubble.rs:219")], &[(
                            "border-radius",
                            format!("radius_2xl(): {}px", t.radius_2xl().as_f32()),
                        ), ("stack gap", "geometry::widget_gap between the bubbles".to_string())], &[("token path", "a Bubble reads cx.theme().semantic_tokens().colors, not the ThemeColor fields directly (bubble.rs, the content surface). The values are the same; the access path is a third one, beside cx.theme().field and the Theme impl's own self.tokens.field"),
                            ("surface padding", "px_3 / py_2 -- rems, so the platform's font -- and settable: Bubble::content takes a BubbleContent, which applies the caller's refinement last (bubble.rs, BubbleContent). native-theme states no bubble"),
                            ("max width", "80% of the row unless the Bubble's own refinement, applied last, sets another (bubble.rs, Bubble::render)"),
                        ])),
            )
            // Message
            .child(section("Message (avatar, bubble, both alignments)"))
            .child(
                div()
                    .id("tt-message")
                    .child(
                        with_gap(v_flex().w(px(420.0)), widget_gap).children(
                            self.chat_messages.iter().take(2).map(chat_message),
                        ),
                    )
                    .on_hover(self.hover_info(&fi, "Message", &[("incoming bubble", "muted", t.muted, "gpui-component/bubble.rs:218"), ("outgoing bubble", "primary", t.primary, "gpui-component/bubble.rs:211")], &[("row gap", "geometry::widget_gap between the rows".to_string())], &[
                            ("bubbles", "a Message delegates its bubble to Bubble, so the two fills are Bubble's and the citations point there"),
                            ("slot gap", "rems(0.625) -- the platform's font, not a literal -- and settable: Message applies the caller's refinement last, and the stack takes Message::with_stack_style. Only the 0.5rem between avatar and content is out of reach (message.rs, Message::render)"),
                            ("avatar", "a shared size-8 minimum, kept flush with the bubble's bottom edge (message.rs, the avatar slot's RenderOnce: min_w_8, self_end), coloured like the Avatar panel's: OKLCH literals hashed from the sender's initials, not theme colours"),
                        ])),
            )
            // MessageScroller
            .child(section(format!(
                "MessageScroller (virtualised thread, {} messages)",
                self.chat_messages.len()
            )))
            .child(
                div()
                    .id("tt-message-scroller")
                    .occlude()
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .w(px(460.0))
                            .child(
                                div()
                                    .h(px(240.0))
                                    .demo_frame(cx)
                                    .child({
                                        // The data stays with the caller; the
                                        // state owns only the virtual list's
                                        // bookkeeping (message_scroller.rs:22-25).
                                        let messages = self.chat_messages.clone();
                                        MessageScroller::new(
                                            "chat-scroller",
                                            self.chat_scroller.clone(),
                                            move |ix, _w, _cx| match messages.get(ix) {
                                                Some(msg) => {
                                                    chat_message(msg).into_any_element()
                                                }
                                                None => div().into_any_element(),
                                            },
                                        )
                                        .with_bottom_fade(t.background)
                                        .size_full()
                                    }),
                            )
                            .child(probe(
                                PROBE_CHAT_SEND,
                                Button::new("chat-send")
                                    .native(cx, geometry::button)
                                    .label("Send a reply")
                                    .on_click(cx.listener(|this, _ev, _w, cx| {
                                        this.chat_messages.push(ChatMessage {
                                            outgoing: true,
                                            sender: "You".into(),
                                            text: "Dark mode follows the desktop too.".into(),
                                        });
                                        this.chat_scroller.update(cx, |state, cx| {
                                            state.append(1, cx);
                                        });
                                        cx.notify();
                                    })),
                            )),
                    )
                    .on_hover(self.hover_info(&fi, "MessageScroller", &[("bottom fade", "background", t.background, "showcase"), ("scrollbar", "scrollbar_thumb", t.scrollbar_thumb, "gpui-component/theme/mod.rs:312"), ("jump button", "background", t.background, "gpui-component/message_scroller.rs:433"), ("jump button edge", "border", t.border, "gpui-component/message_scroller.rs:432"), ("jump button icon", "foreground", t.foreground, "gpui-component/message_scroller.rs:434")], &[], &[
                            ("rows", "the Message rows above, rendered on demand"),
                            ("follow", "FollowMode::Tail: Send scrolls the thread to the new row (message_scroller.rs, MessageScrollerState::new)"),
                            ("jump button", "appears once the user scrolls away from the tail, fading over a 200ms module const rather than the theme's motion tokens (message_scroller.rs, JUMP_BUTTON_TRANSITION)"),
                        ])),
            )
            // Attachment
            .child(section(format!(
                "Attachment (complete, uploading, and one at {:?} — click it)",
                self.attachment_status
            )))
            .child(
                div()
                    .id("tt-attachment")
                    .child(
                        with_gap(h_flex().flex_wrap(), widget_gap)
                            .child(
                                Attachment::new()
                                    .media(AttachmentMedia::new().child(native_icon(
                                        cx,
                                        IconName::Inbox,
                                        geometry::icon_size_small,
                                    )))
                                    .content(
                                        AttachmentContent::new()
                                            .title(AttachmentTitle::new("platform-facts.md"))
                                            .description(AttachmentDescription::new("48 KB")),
                                    ),
                            )
                            .child(
                                Attachment::new()
                                    .status(AttachmentStatus::Uploading)
                                    .media(AttachmentMedia::new().child(native_icon(
                                        cx,
                                        IconName::Copy,
                                        geometry::icon_size_small,
                                    )))
                                    .content(
                                        AttachmentContent::new()
                                            .title(
                                                AttachmentTitle::new("breeze-palette.png")
                                                    .status(AttachmentStatus::Uploading),
                                            )
                                            .description(
                                                AttachmentDescription::new("uploading…")
                                                    .status(AttachmentStatus::Uploading),
                                            ),
                                    ),
                            )
                            // The whole card is the click target, so the
                            // status it is in is the status a click advances.
                            .child(probe(
                                PROBE_ATTACHMENT,
                                Attachment::new()
                                    .id("attachment-cycle")
                                    .status(self.attachment_status)
                                    .media(AttachmentMedia::new().child(native_icon(
                                        cx,
                                        IconName::Settings,
                                        geometry::icon_size_small,
                                    )))
                                    .content(
                                        AttachmentContent::new()
                                            .title(
                                                AttachmentTitle::new("kdeglobals")
                                                    .status(self.attachment_status),
                                            )
                                            .description(
                                                AttachmentDescription::new(SharedString::from(
                                                    format!("{:?}", self.attachment_status),
                                                ))
                                                .status(self.attachment_status),
                                            ),
                                    )
                                    .on_click(cx.listener(|this, _ev, _w, cx| {
                                        this.attachment_status =
                                            next_attachment_status(this.attachment_status);
                                        cx.notify();
                                    })),
                            )),
                    )
                    .on_hover(self.hover_info(&fi, "Attachment", &[("bg", "background", t.background, "gpui-component/attachment.rs:214"), ("border", "border", t.border, "gpui-component/attachment.rs:211"), ("media bg", "muted", t.muted, "gpui-component/attachment.rs:220"), ("description", "muted_foreground", t.muted_foreground, "gpui-component/attachment.rs:595"), ("failed", "danger", t.danger, "gpui-component/theme/mod.rs:428")], &[(
                            "border-radius",
                            format!("radius_2xl(): {}px", t.radius_2xl().as_f32()),
                        ), ("card gap", "geometry::widget_gap between the cards".to_string()), ("icon size", "geometry::icon_size_small: defaults.icon_sizes.small".to_string())], &[
                            ("failed tint", "the semantic layer renames danger to destructive, and a failed card tints its border with it at 30% (theme/mod.rs, color_tokens; attachment.rs, Attachment::render)"),
                            ("in-progress title", "the ShimmerText highlight, driven by the status (attachment.rs, AttachmentTitle::render)"),
                            ("pending", "a dashed border; failed tints the border with destructive (attachment.rs, Attachment::render: border_dashed, destructive.opacity(0.3))"),
                        ])),
            )
    }
}
