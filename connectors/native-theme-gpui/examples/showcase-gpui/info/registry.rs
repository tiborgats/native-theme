//! Which widget's info the inspector shows (spec §4).

use std::{collections::HashMap, rc::Rc, time::Duration};

use gpui::{
    App, Bounds, Context, Div, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Pixels, Stateful, StatefulInteractiveElement as _, Styled as _, canvas,
    div,
};

use super::WidgetInfo;

/// How long a new choice must stay the choice before it replaces what is
/// shown. The model states no hover delay (docs/todo.md, "A sidebar width
/// and a tooltip delay are missing from the model"); this is the showcase's.
pub const INFO_SETTLE: Duration = Duration::from_millis(250);

#[derive(Default)]
pub struct InfoRegistry {
    epoch: u64,
    bounds: HashMap<ElementId, (Bounds<Pixels>, u64)>,
    /// Hovered targets in the order they were entered.
    hovered: Vec<(ElementId, Rc<WidgetInfo>)>,
    shown: Option<(ElementId, Rc<WidgetInfo>)>,
    /// The choice waiting out `INFO_SETTLE`, and the ticket of its timer.
    pending: Option<(ElementId, u64)>,
    tickets: u64,
}

impl InfoRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn bump_epoch(&mut self) {
        self.epoch = self.epoch.wrapping_add(1);
    }
    pub fn shown(&self) -> Option<&Rc<WidgetInfo>> {
        self.shown.as_ref().map(|(_, info)| info)
    }
    fn record_bounds(&mut self, id: ElementId, bounds: Bounds<Pixels>) {
        self.bounds.insert(id, (bounds, self.epoch));
    }
    /// The hovered target drawn in the latest frame with the smallest area;
    /// the most recently entered wins a tie.
    fn choose(&self) -> Option<(ElementId, Rc<WidgetInfo>)> {
        self.hovered
            .iter()
            .enumerate()
            .filter_map(|(ix, (id, info))| {
                let (b, epoch) = self.bounds.get(id)?;
                (*epoch == self.epoch)
                    .then(|| (b.size.width.as_f32() * b.size.height.as_f32(), ix, id, info))
            })
            .min_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)))
            .map(|(_, _, id, info)| (id.clone(), info.clone()))
    }
    fn set_hovered(
        &mut self,
        id: ElementId,
        info: Rc<WidgetInfo>,
        hovered: bool,
        cx: &mut Context<Self>,
    ) {
        self.hovered.retain(|(h, _)| *h != id);
        if hovered {
            self.hovered.push((id, info));
        }
        self.reconsider(cx);
    }
    /// Every target has recorded its bounds for the frame being drawn.
    ///
    /// A target that was not drawn is no longer hovered. gpui sends no hover
    /// end to an element it stopped drawing, and drops that element's state
    /// with the frame (gpui-pre `window.rs:1137-1141`); drawn again under the
    /// pointer, it starts from not hovered and reports the hover anew
    /// (`elements/div.rs:3148-3157`). Nor does a frame that only takes a
    /// target away send any hover change, so the choice is revisited here.
    fn frame_drawn(&mut self, cx: &mut Context<Self>) {
        let (bounds, epoch) = (&self.bounds, self.epoch);
        self.hovered
            .retain(|(id, _)| bounds.get(id).is_some_and(|(_, e)| *e == epoch));
        self.reconsider(cx);
    }
    /// Start the settle timer for a new choice. A choice already waiting
    /// keeps its timer, so the frames drawn meanwhile do not restart it; a
    /// choice that lapses, to nothing or back to what is shown, drops it.
    fn reconsider(&mut self, cx: &mut Context<Self>) {
        let choice = self.choose().map(|(id, _)| id);
        let Some(choice) = choice.filter(|c| self.shown.as_ref().is_none_or(|(s, _)| s != c))
        else {
            self.pending = None;
            return;
        };
        if self.pending.as_ref().is_some_and(|(p, _)| *p == choice) {
            return;
        }
        self.tickets = self.tickets.wrapping_add(1);
        let ticket = self.tickets;
        self.pending = Some((choice, ticket));
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(INFO_SETTLE).await;
            this.update(cx, |this, cx| {
                let Some((waiting, _)) = this.pending.take_if(|(_, t)| *t == ticket) else {
                    return;
                };
                match this.choose() {
                    Some(choice) if choice.0 == waiting => {
                        this.shown = Some(choice);
                        cx.notify();
                    }
                    _ => this.reconsider(cx),
                }
            })
            .ok();
        })
        .detach();
    }
}

/// The root's first child: its prepaint bumps the epoch before any target
/// records its bounds in the same frame (spec §4.2), and its paint runs
/// after every prepaint of the frame, deferred draws included (gpui-pre
/// `window.rs:3546-3579`).
pub fn epoch_marker(ui: &Entity<InfoRegistry>) -> impl IntoElement {
    let (on_prepaint, on_paint) = (ui.clone(), ui.clone());
    canvas(
        move |_, _, cx: &mut App| on_prepaint.update(cx, |r, _| r.bump_epoch()),
        move |_, _, _, cx: &mut App| on_paint.update(cx, |r, cx| r.frame_drawn(cx)),
    )
    .absolute()
    .size_0()
}

pub trait InfoExt: IntoElement + Sized {
    fn info(
        self,
        ui: &Entity<InfoRegistry>,
        id: impl Into<ElementId>,
        info: WidgetInfo,
    ) -> Stateful<Div> {
        let id: ElementId = id.into();
        let info = Rc::new(info);
        let (on_bounds, on_hover) = (ui.clone(), ui.clone());
        let (bounds_id, hover_id) = (id.clone(), id.clone());
        div()
            .id(id)
            .relative()
            .child(self)
            .child(
                // Pinned to the corner: an absolute box left at its static
                // position would sit below the widget, not over it.
                canvas(
                    move |bounds, _, cx: &mut App| {
                        on_bounds.update(cx, |r, _| r.record_bounds(bounds_id, bounds))
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            )
            .on_hover(move |hovered: &bool, _, cx: &mut App| {
                let (id, info) = (hover_id.clone(), info.clone());
                on_hover.update(cx, |r, cx| r.set_hovered(id, info, *hovered, cx));
            })
    }
}

impl<E: IntoElement> InfoExt for E {}
