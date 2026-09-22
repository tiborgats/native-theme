//! The inspector panel: the Widget Info text a hovered demo writes.

use gpui::{
    Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window, prelude::*,
    px,
};
use gpui_component::{
    StyledExt,
    input::{Textarea, TextareaState},
    label::Label,
    v_flex,
};

use crate::{WIDGET_INFO, WIDGET_INFO_TEXT};

// ---------------------------------------------------------------------------
// Widget Info panel – separate Entity so hover updates only re-render this
// small panel instead of the entire Showcase.
// ---------------------------------------------------------------------------

pub(crate) struct WidgetInfoPanel {
    pub(crate) text: String,
    pub(crate) input_state: Entity<TextareaState>,
    /// True when `text` changed and `input_state` needs syncing on next render.
    pub(crate) needs_sync: bool,
}

impl WidgetInfoPanel {
    pub(crate) fn set_text(&mut self, text: String, cx: &mut Context<Self>) {
        if self.text != text {
            self.text = text;
            self.needs_sync = true;
            cx.notify();
        }
    }
}

impl Render for WidgetInfoPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.needs_sync {
            self.needs_sync = false;
            let val = if self.text.is_empty() {
                SharedString::from("Hover over any widget to see its theme properties.")
            } else {
                SharedString::from(self.text.clone())
            };
            self.input_state.update(cx, |state, cx| {
                state.set_value(val, window, cx);
            });
        }

        // The panel is the sidebar column's only growing child, so it takes
        // every pixel the controls above it leave -- at any window size, and
        // without a height of its own. `min_h_0` is what lets it shrink past
        // the text it holds: the textarea then scrolls its own content instead
        // of pushing the panel out of the column.
        v_flex()
            .p_3()
            .w_full()
            .flex_1()
            .min_h_0()
            // The textarea inside scrolls; without this the sidebar column
            // behind it scrolls by the same delta (see the List demo).
            .occlude()
            .debug_selector(|| WIDGET_INFO.into())
            .child(
                Label::new("Widget Info")
                    .text_size(px(13.0))
                    .font_semibold(),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .debug_selector(|| WIDGET_INFO_TEXT.into())
                    .child(
                        Textarea::new(&self.input_state)
                            .appearance(false)
                            .text_size(px(11.0))
                            .h_full(),
                    ),
            )
    }
}
