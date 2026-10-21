//! Which widget's info the inspector shows (spec §4).
//!
//! # What "drawn" means
//!
//! A frame need not run the code of everything it draws. GPUI Fast draws a
//! view nothing changed for from what it drew in the last frame, copying
//! its prepaint and paint (gpui-fast 0.1.10 src/fast/retained.rs:738-825,
//! `reuse_retained_prepaint`; src/fast/splice.rs:608-757, `splice_prepaint`,
//! for a view drawn around views built again), and gpui-pre does the same
//! for a cached view (src/view.rs:530-546). A target inside such a view
//! records nothing in that frame, yet it is drawn, at the bounds it was
//! drawn at: a view is drawn from the last frame only where it is laid out
//! as it was (retained.rs:630-638, `retained_context_matches`). Inside a
//! scroll area that GPUI Fast composites while it scrolls, it is not: the
//! area's content is drawn moved by the scroll, without its children's
//! prepaint (src/fast/layers/paint.rs:233-235, src/fast/layers/reuse.rs:39),
//! so a record there keeps the bounds of the layer's last repaint, off by
//! the scroll since. gpui moves the hitboxes, so hovers and the choice,
//! which compares areas only, hold; the layout dump does not
//! (`layout_drawn`).
//!
//! So the registry does not ask whether a target recorded in the frame,
//! but whether the frame drew its element, the way gpui itself tells: an
//! element's state lives on while each frame draws the element and goes
//! with the first frame that does not (gpui-fast src/window.rs:1221-1231
//! and :3561-3567, gpui-pre src/window.rs:1195-1202). A frame that copies a
//! view copies the element states it used (gpui-fast window.rs:4079-4084 and
//! :4163-4168, splice.rs:779-784, and for a composited scroll layer
//! src/fast/layers/reuse.rs:101-106 and :204-209; gpui-pre
//! window.rs:4008-4010). Each record holds a weak handle to a [`Presence`]
//! its recorder keeps in its element state ([`begin_record`]): the record
//! stands for a drawn element while the handle is live.
//!
//! # What the registry keeps out of its updates
//!
//! GPUI Fast counts an update of the registry as a change of what every
//! view that records into it read: one made outside drawing has a view
//! inside a notified view built again rather than drawn from the last
//! frame (gpui-fast src/fast/dependencies.rs:757-797, `note_update`, and
//! :513-527). So what only keeps track of the frames, which no view draws
//! from, is kept in cells and changed through reads, and the end of a frame
//! updates the registry only when the frame changed what it decides.
//!
//! The records themselves are updates made while drawing, which GPUI Fast
//! counts as writes: a retained subtree that read the registry and did not
//! write it itself is built again in the next frame (dependencies.rs:757-797
//! and :646-653, `written_since`), as a view that records before a view
//! nested in it records is. That costs reuse, not correctness.

use std::{
    cell::Cell,
    collections::HashMap,
    rc::{Rc, Weak},
    time::Duration,
};

use gpui::{
    App, Bounds, Context, Div, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Pixels, Stateful, StatefulInteractiveElement as _, Styled as _, Window,
    canvas, div, prelude::FluentBuilder as _,
};

use super::WidgetInfo;

/// How long a new choice must stay the choice before it replaces what is
/// shown: the showcase's own. The model's `tooltip.show_delay_ms` is how long
/// a tooltip waits, and Widget Info is not a tooltip.
pub const INFO_SETTLE: Duration = Duration::from_millis(250);

/// What a recorder keeps in its element state while its element is drawn
/// (module docs).
#[derive(Default)]
pub struct Presence;

/// Where a target or an element of the list was laid out, whether its
/// element is still drawn, and, for the tests, in which draw it last
/// recorded that.
struct Record {
    bounds: Bounds<Pixels>,
    presence: Weak<Presence>,
    #[cfg(test)]
    draw: u64,
}

impl Record {
    /// The last frame drawn drew the element, from scratch or from the
    /// frame before (module docs). Meaningful between frames: while one is
    /// drawn, an element it no longer draws keeps its state until the
    /// frame is done.
    fn drawn(&self) -> bool {
        self.presence.strong_count() > 0
    }
    /// The draw it was made in and the presence it holds, for
    /// `InfoRegistry::recorded_twice`.
    #[cfg(test)]
    fn draw_and_presence(&self) -> (u64, Weak<Presence>) {
        (self.draw, self.presence.clone())
    }
}

#[derive(Default)]
pub struct InfoRegistry {
    /// Something recorded since the last frame was done: its first record
    /// set it and had the frame's end scheduled ([`begin_record`]).
    open: Cell<bool>,
    bounds: HashMap<ElementId, Record>,
    /// Hovered targets in the order they were entered.
    hovered: Vec<(ElementId, Rc<WidgetInfo>)>,
    shown: Option<(ElementId, Rc<WidgetInfo>)>,
    /// The choice waiting out `INFO_SETTLE`, and the ticket of its timer.
    pending: Option<(ElementId, u64)>,
    tickets: u64,
    /// What the window shows changed since the last frame was drawn: the
    /// active page, or the installed theme.
    screen_changed: bool,
    /// Where each element of docs/showcase-elements.toml was laid out
    /// (`crate::elements`).
    layout: HashMap<&'static str, Record>,
    /// The draws that recorded anything, counted, for the tests' check of
    /// two records under one id in one draw. A draw prepaints everything
    /// before it paints anything (gpui-fast src/window.rs:3757-3813, gpui-pre
    /// alike), so a record made after a target or a `record` box painted
    /// ([`painted`]) is the next draw's. That holds while every draw that
    /// records paints a target or a `record` box: one whose only records
    /// are `ListedExt::listed` boxes or part observers, which paint nothing,
    /// would count with the next, and a record under the same id there
    /// would be reported as two, a false alarm, never a miss. Several draws
    /// can share one end of a frame: gpui-kit 0.8.0's `click` draws, then
    /// moves the pointer, presses and releases, each drawing again
    /// (src/test.rs:236-257, with :190, :212 and :233), as its `drag` does
    /// (:393-412), and the showcase's `draw` draws once in its update and
    /// once more as the test flushes it (gpui-fast src/app.rs:2092-2106).
    #[cfg(test)]
    draw: u64,
    #[cfg(test)]
    painted: Cell<bool>,
    /// Every id two targets were drawn under in one draw: both recorded in
    /// it, or one recorded while another, drawn from the last frame, stood.
    /// Two targets sharing an id overwrite each other's bounds and hovers,
    /// since the registry keys them by the id alone; the tests read this
    /// after every frame they draw.
    #[cfg(test)]
    pub drawn_twice: std::collections::BTreeSet<String>,
    /// Every element id two elements were drawn under in one draw, as
    /// `drawn_twice` has it for targets; the tests read it.
    #[cfg(test)]
    pub listed_twice: std::collections::BTreeSet<&'static str>,
    /// Records that replaced one of another element made in an earlier
    /// draw, with that element's presence: if the frame still draws it once
    /// it is done, it drew two elements under one id. Taken at the next
    /// update of the registry; one whose element is no longer drawn by then
    /// is dropped.
    #[cfg(test)]
    displaced: Vec<(Twice, Weak<Presence>)>,
    /// The text colour each element of the list was laid out under in the
    /// last frame that recorded it: what a text the element's box holds is
    /// painted in, which no quad of the scene carries. The tests read it.
    #[cfg(test)]
    pub text_colours: HashMap<&'static str, gpui::Hsla>,
    /// The text colour each List and Tree row's label was laid out under in
    /// the last frame that drew it, by the row's info id
    /// (`crate::elements::label_probe`). The tests read it.
    #[cfg(test)]
    pub label_colours: HashMap<String, gpui::Hsla>,
}

/// An id drawn twice, a target's or an element's of the list.
#[cfg(test)]
enum Twice {
    Target(String),
    Element(&'static str),
}

impl InfoRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn shown(&self) -> Option<&Rc<WidgetInfo>> {
        self.shown.as_ref().map(|(_, info)| info)
    }
    /// The element `id` of the list was laid out at `bounds` in the frame
    /// being drawn, by the recorder holding `presence` ([`begin_record`]).
    pub fn record_layout(
        &mut self,
        id: &'static str,
        bounds: Bounds<Pixels>,
        presence: Weak<Presence>,
    ) {
        #[cfg(test)]
        {
            let previous = self.layout.get(id).map(Record::draw_and_presence);
            if self.recorded_twice(previous, &presence, || Twice::Element(id)) {
                self.listed_twice.insert(id);
            }
        }
        self.layout.insert(
            id,
            Record {
                bounds,
                presence,
                #[cfg(test)]
                draw: self.draw,
            },
        );
    }
    /// Every element of the list the last frame drew, and where: where it
    /// last recorded, which inside a scroll area GPUI Fast composites is
    /// off by the scroll since the layer's last repaint (module docs).
    pub fn layout_drawn(&self) -> std::collections::BTreeMap<&'static str, Bounds<Pixels>> {
        self.layout
            .iter()
            .filter(|(_, record)| record.drawn())
            .map(|(id, record)| (*id, record.bounds))
            .collect()
    }
    /// Starts the next draw's count if a target or a `record` box painted
    /// since the last record, then tells whether a record by the recorder
    /// holding `presence` makes two under one id with `previous`, the
    /// record it replaces: true when `previous` was made in this draw too.
    /// When it is another recorder's from an earlier draw, that is two only
    /// if the frame still draws that recorder's element as well, which is
    /// known once the frame is done: `twice` names it for then.
    #[cfg(test)]
    fn recorded_twice(
        &mut self,
        previous: Option<(u64, Weak<Presence>)>,
        presence: &Weak<Presence>,
        twice: impl FnOnce() -> Twice,
    ) -> bool {
        if self.painted.replace(false) {
            self.draw = self.draw.wrapping_add(1);
        }
        let Some((draw, other)) = previous else {
            return false;
        };
        if draw == self.draw {
            return true;
        }
        if !other.ptr_eq(presence) {
            self.displaced.push((twice(), other));
        }
        false
    }
    /// The active page changed, or a theme was installed. Once the next
    /// frame is drawn, what is shown goes back to the hint unless that frame
    /// drew its target too, as it draws the chrome's: no instance of the
    /// previous page stays on show (spec §4.3.4), and no info keeps the
    /// colours of a theme no longer installed -- a target still drawn hands
    /// the registry its info under the new theme (`record_target`), and one
    /// that is not, an overlay closed since, cannot. The pointer leaving a
    /// target still keeps it (§4.3.3).
    pub fn screen_changed(&mut self) {
        self.screen_changed = true;
    }
    /// A target was laid out at `bounds` with `info`, as the frame being
    /// drawn built it, by the recorder holding `presence`
    /// ([`begin_record`]). The info replaces the one hovered or shown under
    /// the same id, so a state change under a still pointer -- a click that
    /// makes an item active, a theme switch -- reaches the inspector; hover
    /// events alone would miss it. True when what is shown changed, and only
    /// then: a new `Rc` of equal content is not a change, or every frame
    /// would ask for the next.
    fn record_target(
        &mut self,
        id: ElementId,
        bounds: Bounds<Pixels>,
        info: &Rc<WidgetInfo>,
        presence: Weak<Presence>,
    ) -> bool {
        // The record it replaces stands for an element no longer drawn: the
        // target is drawn again as a new element after a frame that did not
        // draw it, which starts from not hovered (gpui-pre
        // elements/div.rs:3154-3163), so the hover it had is over. A frame
        // that took it away and recorded nothing did not prune it
        // (`close_frame`).
        if self.bounds.get(&id).is_some_and(|record| !record.drawn()) {
            self.hovered.retain(|(h, _)| *h != id);
        }
        if let Some((_, hovered)) = self.hovered.iter_mut().find(|(h, _)| *h == id)
            && **hovered != **info
        {
            *hovered = info.clone();
        }
        let shown_changed = match &mut self.shown {
            Some((shown_id, shown)) if *shown_id == id && **shown != **info => {
                *shown = info.clone();
                true
            }
            _ => false,
        };
        #[cfg(test)]
        {
            let previous = self.bounds.get(&id).map(Record::draw_and_presence);
            if self.recorded_twice(previous, &presence, || Twice::Target(id.to_string())) {
                self.drawn_twice.insert(id.to_string());
            }
        }
        self.bounds.insert(
            id,
            Record {
                bounds,
                presence,
                #[cfg(test)]
                draw: self.draw,
            },
        );
        shown_changed
    }
    /// The last frame drew the target `id`.
    fn drawn(&self, id: &ElementId) -> bool {
        self.bounds.get(id).is_some_and(Record::drawn)
    }
    /// The hovered target drawn in the last frame with the smallest area;
    /// the most recently entered wins a tie.
    fn choose(&self) -> Option<(ElementId, Rc<WidgetInfo>)> {
        self.hovered
            .iter()
            .enumerate()
            .filter_map(|(ix, (id, info))| {
                let b = self.bounds.get(id).filter(|record| record.drawn())?.bounds;
                Some((b.size.width.as_f32() * b.size.height.as_f32(), ix, id, info))
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
    /// Whether `close_frame` would change anything: a hovered target not
    /// drawn, a screen change to settle, or a choice that changed since its
    /// timer started (`reconsider`).
    fn frame_changes_anything(&self) -> bool {
        self.screen_changed
            || self.hovered.iter().any(|(id, _)| !self.drawn(id))
            || match (self.choice(), &self.pending) {
                (None, None) => false,
                (Some(choice), Some((pending, _))) => choice != *pending,
                _ => true,
            }
    }
    /// The frame is drawn, and what it no longer draws has gone with it
    /// (module docs).
    ///
    /// A target that was not drawn is no longer hovered. gpui sends no hover
    /// end to an element it stopped drawing, and drops that element's state
    /// with the frame (gpui-pre `window.rs:1195-1202`); drawn again under
    /// the pointer, it starts from not hovered and reports the hover anew
    /// (`elements/div.rs:3154-3163`). Nor does a frame that only takes a
    /// target away send any hover change, so the choice is revisited here.
    /// Notifies when it cleared what was shown.
    ///
    /// Only a frame that records has an end ([`begin_record`]). After one
    /// that took a hovered target away and recorded nothing, the target
    /// stays in `hovered` until the next frame that records or a hover
    /// event of its own id: it is never chosen meanwhile, since it is not
    /// drawn, and a record of it drawn again prunes it first
    /// (`record_target`), so the choice is never wrong, only revisited
    /// late: what is shown changes at the next record or hover event.
    fn close_frame(&mut self, cx: &mut Context<Self>) {
        let bounds = &self.bounds;
        self.hovered
            .retain(|(id, _)| bounds.get(id).is_some_and(Record::drawn));
        let cleared = std::mem::take(&mut self.screen_changed)
            && self.shown.as_ref().is_some_and(|(id, _)| !self.drawn(id));
        if cleared {
            self.shown = None;
            cx.notify();
        }
        self.reconsider(cx);
    }
    /// The hovered target to show next, unless it is the one shown.
    fn choice(&self) -> Option<ElementId> {
        self.choose()
            .map(|(id, _)| id)
            .filter(|c| self.shown.as_ref().is_none_or(|(s, _)| s != c))
    }
    /// Start the settle timer for a new choice. A choice already waiting
    /// keeps its timer, so the frames drawn meanwhile do not restart it; a
    /// choice that lapses, to nothing or back to what is shown, drops it.
    fn reconsider(&mut self, cx: &mut Context<Self>) {
        let Some(choice) = self.choice() else {
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

/// Tell the registry's observers, once the frame being drawn is done: a
/// notify raised while a frame is drawn does not ask for the next one
/// (gpui-base/resizable/panel.rs, `ResizablePanelGroup` defers its own for
/// the same reason).
fn notify_after_draw(ui: &Entity<InfoRegistry>, window: &mut Window, cx: &mut App) {
    let ui = ui.clone();
    window.defer(cx, move |_, cx| ui.update(cx, |_, cx| cx.notify()));
}

/// Readies a record of the frame being drawn, from a recorder's prepaint:
/// the presence the recorder keeps in its element state under `key`, for
/// the record to hold (module docs), and, at the first record since the
/// last frame was done, that frame's end scheduled ([`end_frame`]):
/// `Window::defer` runs it as the effects the draw left are flushed, after
/// the frame was swapped in and the states it did not use were dropped
/// (gpui-fast src/window.rs:2645-2650; src/app.rs:1351-1366, an update
/// flushes its effects when it ends, and :2080-2105; the platform's frame
/// callback draws inside one, src/window.rs:1920-1938).
///
/// `key` tells apart the recorders that share their element's id path, as
/// an `elements::record` beside another does. An element state is kept
/// under its type as well as its id (gpui-fast src/window.rs:7420-7433), so
/// the `unwrap` of `with_element_state`'s downcast (:4495) cannot fail; the
/// `expect` (:4497) cannot either, since a box is only put back holding a
/// state (:4501, :4513-4516).
pub fn begin_record(
    ui: &Entity<InfoRegistry>,
    key: ElementId,
    window: &mut Window,
    cx: &mut App,
) -> Weak<Presence> {
    let presence = window.with_global_id(key, |id, window| {
        window.with_element_state(id, |held: Option<Rc<Presence>>, _| {
            let held = held.unwrap_or_default();
            (Rc::downgrade(&held), held)
        })
    });
    if !ui.read(cx).open.replace(true) {
        let ui = ui.clone();
        window.defer(cx, move |_, cx| end_frame(&ui, cx));
    }
    presence
}

/// A target or a `record` box painted: what records next is the next
/// draw's (`InfoRegistry::draw`). Only the tests count draws.
pub fn painted(ui: &Entity<InfoRegistry>, cx: &App) {
    #[cfg(test)]
    ui.read(cx).painted.set(true);
    #[cfg(not(test))]
    let _ = (ui, cx);
}

/// The end of the frames drawn since the first record [`begin_record`]
/// scheduled it for: the registry is updated only when they changed what
/// it decides (module docs).
fn end_frame(ui: &Entity<InfoRegistry>, cx: &mut App) {
    let registry = ui.read(cx);
    registry.open.set(false);
    #[cfg(test)]
    let drew_twice = registry
        .displaced
        .iter()
        .any(|(_, other)| other.strong_count() > 0);
    #[cfg(not(test))]
    let drew_twice = false;
    if !drew_twice && !registry.frame_changes_anything() {
        return;
    }
    ui.update(cx, |registry, cx| {
        #[cfg(test)]
        for (twice, other) in std::mem::take(&mut registry.displaced) {
            if other.strong_count() == 0 {
                continue;
            }
            match twice {
                Twice::Target(id) => {
                    registry.drawn_twice.insert(id);
                }
                Twice::Element(id) => {
                    registry.listed_twice.insert(id);
                }
            }
        }
        registry.close_frame(cx);
        // `frame_changes_anything` restates what `close_frame` decides; once
        // it has run, nothing is left for it to change.
        #[cfg(test)]
        assert!(
            !registry.frame_changes_anything(),
            "frame_changes_anything and close_frame disagree"
        );
    });
}

pub trait InfoExt: IntoElement + Sized {
    fn info(
        self,
        ui: &Entity<InfoRegistry>,
        id: impl Into<ElementId>,
        info: WidgetInfo,
    ) -> Stateful<Div> {
        let id: ElementId = id.into();
        let listed = info.listed.or(match &id {
            ElementId::Name(name) => crate::elements::listed_for(name),
            _ => None,
        });
        let info = Rc::new(WidgetInfo { listed, ..info });
        let (on_bounds, on_paint, on_hover, on_layout) =
            (ui.clone(), ui.clone(), ui.clone(), ui.clone());
        let (bounds_id, hover_id) = (id.clone(), id.clone());
        let bounds_info = info.clone();
        div()
            // The element's own bounds, its first child's, for the layout
            // dump: the wrapper can be wider where its parent stretches it.
            .when_some(listed, |wrapper, listed| {
                wrapper.on_children_prepainted(move |bounds, window, cx| {
                    if let Some(bounds) = bounds.first() {
                        let presence = begin_record(&on_layout, listed.into(), window, cx);
                        on_layout.update(cx, |r, _| r.record_layout(listed, *bounds, presence));
                    }
                })
            })
            .id(id)
            .relative()
            .child(self)
            .child(
                // Pinned to the corner: an absolute box left at its static
                // position would sit below the widget, not over it.
                canvas(
                    move |bounds, window: &mut Window, cx: &mut App| {
                        let presence = begin_record(&on_bounds, bounds_id.clone(), window, cx);
                        let changed = on_bounds.update(cx, |r, _| {
                            r.record_target(bounds_id, bounds, &bounds_info, presence)
                        });
                        if changed {
                            notify_after_draw(&on_bounds, window, cx);
                        }
                    },
                    move |_, _, _, cx: &mut App| painted(&on_paint, cx),
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
