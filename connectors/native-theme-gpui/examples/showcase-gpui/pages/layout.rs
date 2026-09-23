//! The Layout page.

use gpui::{
    App, Axis, Context, IntoElement, ParentElement, SharedString, Styled, Window, div, prelude::*,
    px,
};
use gpui_component::{
    ActiveTheme, IconName, StyledExt, TitleBar, WindowExt,
    accordion::Accordion,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonVariants},
    carousel::{
        Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPagination,
        CarouselPaginationItem, CarouselPrevious,
    },
    collapsible::Collapsible,
    form::{self, Field},
    group_box::GroupBoxVariants,
    h_flex,
    input::Input,
    label::Label,
    notification::Notification,
    resizable::{h_resizable, resizable_panel, v_resizable},
    scroll::ScrollableElement,
    separator::Separator,
    setting::{SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    sidebar::{Sidebar, SidebarMenu, SidebarMenuItem, SidebarToggleButton},
    status_bar::StatusBar,
    stepper::{Stepper, StepperItem},
    theme::Theme,
    v_flex, window_paddings,
};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::support::{
    CAROUSEL_SLIDES, NativeStyled, RESIZABLE_GROUPS, ResizableGroup, STEPPER_STEPS,
    TITLE_BAR_CONTROLS_NOTE, format_font_info, ghost_hover_fill, layout_value, native_geometry,
    native_group_box, native_icon, section, with_accordion_title_style, with_gap, with_padding,
};
use crate::{
    PROBE_CAROUSEL_LAST, PROBE_SETTINGS_ROW, PROBE_SIDEBAR_TOGGLE, PROBE_STEPPER, Page, probe,
};

impl Showcase {
    /// One resizable group of the Layout page, from its [`RESIZABLE_GROUPS`]
    /// entry. The box carries its own id as a debug selector so
    /// `resizable_groups_have_room_to_drag` can measure what was laid out.
    fn render_resizable_group(
        &self,
        group: &'static ResizableGroup,
        fi: &str,
        t: &Theme,
        cx: &App,
    ) -> impl IntoElement {
        let panels = group.panels.iter().map(|panel| {
            let body = v_flex()
                .p_3()
                .size_full()
                .child(Label::new(panel.title).font_semibold())
                .children(panel.caption.map(|c| Label::new(c).text_sm()));
            match panel.size {
                Some(size) => resizable_panel().size(px(size)).child(body),
                None => resizable_panel().child(body),
            }
        });
        let panel_group = match group.axis {
            Axis::Horizontal => h_resizable(group.group_id),
            Axis::Vertical => v_resizable(group.group_id),
        };
        div()
            .id(group.id)
            .debug_selector(|| group.id.into())
            .h(px(group.height))
            .demo_frame(cx)
            .child(panel_group.children(panels))
            .on_hover(self.hover_info(
                fi,
                "Resizable",
                &[
                    (
                        "dragging handle",
                        "drag_border",
                        t.drag_border,
                        "gpui-component/theme/mod.rs:344",
                    ),
                    (
                        "idle handle",
                        "border",
                        t.border,
                        "gpui-component/theme/mod.rs:343",
                    ),
                ],
                &[],
                &[("min panel size", "PANEL_MIN_SIZE, 100px, unless a panel's own size range replaces it (gpui-base resizable/panel.rs, size_range)")],
            ))
    }

    // -----------------------------------------------------------------------
    // Page: Layout
    // -----------------------------------------------------------------------
    pub(crate) fn render_layout_page(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let accordion_title_style = native_geometry(cx, geometry::accordion_title);
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let collapsible_open = self.collapsible_open;
        // The four layout accessors. `None` where the platform specifies
        // nothing (platform-facts §2.20), and then the showcase's own spacing
        // stands — nothing is invented to fill the gap.
        let widget_gap = geometry::widget_gap(&self.layout);
        let container_margin = geometry::container_margin(&self.layout);
        let window_margin = geometry::window_margin(&self.layout);
        let section_gap = geometry::section_gap(&self.layout);
        let spacing_summary = format!(
            "widget_gap {} · container_margin {} · window_margin {} · section_gap {}",
            layout_value(widget_gap),
            layout_value(container_margin),
            layout_value(window_margin),
            layout_value(section_gap),
        );
        // The frame this very window is inside. `Root::new` sets
        // `bordered: true` (`root.rs:117`) and `Root::render` wraps everything
        // it holds in `window_border()` (`root.rs:605`), so the showcase's
        // `WindowBorder` is the window's own edge; a second, nested one would
        // set the client inset and lay down resize hit zones over the window a
        // second time.
        //
        // # shown-by: read, not constructed. `window_paddings` is
        // gpui-component's own `window_border::window_paddings`, and it returns
        // `window.client_inset()` (`window_border.rs:88-94`); the only writer of
        // that inset is `WindowBorder::render` (`window_border.rs:147-149`). The
        // `Some` the summary below reports is therefore live evidence that the
        // widget rendered around this window -- and under server-side
        // decorations, where upstream draws nothing, there is none.
        let decorations = window.window_decorations();
        let frame_insets = window_paddings(window);
        let client_inset = window.client_inset();
        let window_border_summary = format!(
            "{} · insets: top {}px, right {}px, bottom {}px, left {}px · {}",
            match decorations {
                gpui::Decorations::Server => "server-side decorations: a pass-through".to_string(),
                gpui::Decorations::Client { tiling } =>
                    format!("client-side decorations, tiled {tiling:?}"),
            },
            frame_insets.top.as_f32(),
            frame_insets.right.as_f32(),
            frame_insets.bottom.as_f32(),
            frame_insets.left.as_f32(),
            match client_inset {
                Some(inset) => format!(
                    "client inset {}px: the WindowBorder that Root renders around this window \
                     set it",
                    inset.as_f32()
                ),
                // Under server-side decorations the widget did render; it just
                // rendered as a pass-through, and only its client-side arm
                // calls set_client_inset (`window_border.rs:147-148`).
                None => match decorations {
                    gpui::Decorations::Server =>
                        "no client inset: the WindowBorder around this window rendered as a \
                         pass-through, and only its client-side arm sets one"
                            .to_string(),
                    gpui::Decorations::Client { .. } =>
                        "no client inset: nothing has called set_client_inset".to_string(),
                },
            },
        );
        // The resize band is the client-side arm's alone: the Server arm hands
        // back the bare backdrop div (`window_border.rs:172`) and the
        // compositor owns the edges.
        let window_border_hint = match decorations {
            gpui::Decorations::Server => {
                "The compositor draws this window's frame and resizes it; the widget lays no \
                 resize band of its own here."
            }
            gpui::Decorations::Client { .. } => {
                "Drag an edge of the window: the resize band is this widget's."
            }
        };
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            .child(section("WindowBorder (this window's own frame)"))
            .child(
                div()
                    .id("tt-window-border")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .child(
                                Label::new(SharedString::from(window_border_summary)).text_sm(),
                            )
                            .child(
                                Label::new(window_border_hint)
                                    .text_sm()
                                    .text_color(t.muted_foreground),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "WindowBorder", &[], &[], &[
                            ("receiver", "Root::new sets bordered and Root::render wraps the window in window_border() (root.rs); nesting a second one would call set_client_inset again (window_border.rs, WindowBorder::render)"),
                            ("window fill", "the border draws no window background: the claim is dropped (window_border.rs)"),
                            ("frame colour", "hardcoded grey, l=0.2 dark / l=0.8 light (window_border.rs, WindowBorder::render border_color)"),
                            ("shadow", "hardcoded two-layer box shadow (window_border.rs, WindowBorder::render shadow(vec![..]))"),
                            ("server-side decorations", "nothing is drawn: the compositor owns the frame (window_border.rs, WindowBorder::render Decorations::Server arm)"),
                        ])),
            )
            // Window chrome, stacked the way a window stacks it: the title bar
            // above, a toolbar the application draws itself, the status bar
            // below. Each is bounded in its own section like every other
            // widget here — and the title bar is a real one, so it carries
            // upstream's window handlers with it.
            .child(section("TitleBar (a real one: dragging it moves the window)"))
            .child(
                div()
                    .id("tt-title-bar")
                    .demo_frame(cx)
                    .child(
                        TitleBar::new()
                            .native(cx, geometry::title_bar)
                            // Linux only (`title_bar.rs:99-101` drops the
                            // handler on every other platform), and the reason
                            // this bar can be shown at all there: without a
                            // handler the X calls `window.remove_window()`
                            // (`:237`), which would close the showcase.
                            // Upstream's other handlers — drag to move, double
                            // click to zoom — are the widget, and stay.
                            //
                            // On Windows the handler is discarded and the
                            // controls are hit-tested by the OS instead
                            // (`window_control_area`, `:220-222`), so this
                            // demo bar's minimise, maximise and close act on
                            // the real window. Nothing can intercept them
                            // short of not drawing the widget, so the note
                            // below says so on that platform. On macOS
                            // upstream draws no controls at all (`:254-256`).
                            .on_close_window(cx.listener(|_this, _ev, window, cx| {
                                window.push_notification(
                                    Notification::info(
                                        "A nested title bar for the geometry builder; \
                                         its close button is deliberately inert.",
                                    )
                                    .title("TitleBar")
                                    .autohide(true),
                                    cx,
                                );
                            }))
                            // No `.text_sm()`: `Label::render` applies its own
                            // refinement last (`label.rs:212`), so a size set
                            // here would cancel the one `geometry::title_bar`
                            // just supplied.
                            .child(Label::new("native-theme showcase")),
                    )
                    .child(Label::new(TITLE_BAR_CONTROLS_NOTE).text_sm().text_color(
                        if cfg!(target_os = "windows") {
                            t.danger
                        } else {
                            t.muted_foreground
                        },
                    ))
                    .on_hover(self.hover_info(&fi, "TitleBar", &[("bg", "title_bar", t.title_bar, "gpui-component/title_bar.rs:340"), ("border", "title_bar_border", t.title_bar_border, "gpui-component/title_bar.rs:338"), ("window control text", "foreground", t.foreground, "gpui-component/title_bar.rs:217"), ("control hover", "secondary_hover", t.secondary_hover, "gpui-component/title_bar.rs:181"), ("control hover icon", "secondary_foreground", t.secondary_foreground, "gpui-component/title_bar.rs:172"), ("control pressed", "secondary_active", t.secondary_active, "gpui-component/title_bar.rs:190"), ("close hover", "danger", t.danger, "gpui-component/title_bar.rs:179"), ("close hover icon", "danger_foreground", t.danger_foreground, "gpui-component/title_bar.rs:170"), ("close pressed", "danger_active", t.danger_active, "gpui-component/title_bar.rs:188"), ("label text", "foreground", t.foreground, "gpui-component/label.rs:211")], &[("geometry", "geometry::title_bar: window.title_bar_font size and weight, which the label inherits, and colour, which it does not: a Label sets foreground on its own element (label.rs, Label). Upstream sets no text colour on the bar, so the builder's displaces nothing there. The window controls set foreground on their own elements, out of its reach".to_string())], &[
                            ("height", "TITLE_BAR_HEIGHT, 34px, and settable: TitleBar applies the caller's refinement after it (title_bar.rs, TitleBar). The model states no title-bar height -- our gap. Only the window controls stay 34px wide"),
                            ("fill", "a gradient between title_bar and background (title_bar.rs, default_title_bar_background)"),
                            ("window controls", TITLE_BAR_CONTROLS_NOTE),
                        ])),
            )
            .child(section("Toolbar (no widget upstream: the application draws it)"))
            .child(
                div()
                    .id("tt-toolbar")
                    .child(
                        // gpui-component has no toolbar widget, so the row is
                        // the application's own and `geometry::toolbar` is
                        // the whole of its geometry: nothing after it sets a
                        // height, padding, gap or fill that would override it.
                        h_flex()
                            .native(cx, geometry::toolbar)
                            .demo_frame(cx)
                            .child(native_icon(cx, IconName::Search, geometry::icon_size_toolbar))
                            .child(native_icon(cx, IconName::Copy, geometry::icon_size_toolbar))
                            .child(native_icon(cx, IconName::Settings, geometry::icon_size_toolbar))
                            .child(Separator::vertical())
                            .child(Label::new("Toolbar icons at the platform's toolbar size")),
                    )
                    .on_hover(self.hover_info(&fi, "Toolbar (application-drawn)", &[("border", "border", t.border, "showcase")], &[("geometry", "geometry::toolbar: toolbar.bar_height (minimum height), item_gap, border.padding_*, background_color, font size and weight".to_string()), ("icon size", "geometry::icon_size_toolbar: toolbar.icon_size, which inherits defaults.icon_sizes.toolbar".to_string())], &[
                            ("widget", "gpui-component has no toolbar widget, so this row is the application's own h_flex. Its border is the showcase's demo frame (support.rs, demo_frame), not the toolbar's"),
                            ("edge", "the model inherits toolbar.border.color and line_width from defaults.border, but platform-facts §2.13 states neither, so geometry::toolbar draws no edge -- an application that wants a rule draws a Separator"),
                        ])),
            )
            .child(section("StatusBar"))
            .child(
                div()
                    .id("tt-status-bar")
                    .demo_frame(cx)
                    .child(
                        StatusBar::new()
                            .native(cx, geometry::status_bar)
                            .left(native_icon(cx, IconName::Inbox, geometry::icon_size_small))
                            .left("6 items")
                            .child("native-theme showcase")
                            .right("UTF-8"),
                    )
                    .on_hover(self.hover_info(&fi, "StatusBar", &[("bg", "status_bar", t.status_bar, "gpui-component/status_bar.rs:93"), ("border", "status_bar_border", t.status_bar_border, "gpui-component/status_bar.rs:92"), ("upstream text", "muted_foreground", t.muted_foreground, "gpui-component/status_bar.rs:95")], &[("geometry", "geometry::status_bar: status_bar.border.padding_*, status_bar.font — including its colour, which upstream would otherwise paint with muted_foreground (status_bar.rs, StatusBar::render: text_color then refine_style)".to_string()), ("icon size", "geometry::icon_size_small: defaults.icon_sizes.small".to_string())], &[
                            ("region gap", "gap_2, 0.5rem, on region children the refinement does not reach (status_bar.rs, StatusBar::render region); the model states no status-bar item gap"),
                        ])),
            )
            // The four layout accessors, applied rather than printed: the outer
            // box takes the platform's window margin, the row inside it the
            // container margin and the widget gap, and the two rows are
            // separated by the section gap.
            .child(section("Layout spacing (the four LayoutTheme accessors)"))
            .child(
                div()
                    .id("tt-layout-spacing")
                    .child(
                        with_padding(
                            with_gap(
                                v_flex().demo_frame(cx),
                                section_gap,
                            ),
                            window_margin,
                        )
                        .child(
                            with_padding(
                                with_gap(
                                    h_flex().demo_frame(cx),
                                    widget_gap,
                                ),
                                container_margin,
                            )
                            .child(Button::new("ls-a").native(cx, geometry::button).label("One"))
                            .child(Button::new("ls-b").native(cx, geometry::button).label("Two"))
                            .child(Button::new("ls-c").native(cx, geometry::button).label("Three")),
                        )
                        .child(Label::new(SharedString::from(spacing_summary)).text_sm()),
                    )
                    .on_hover(self.hover_info(&fi, "Layout spacing", &[("border", "border", t.border, "showcase")], &[], &[("receivers", "none a theme can write: Theme::spacing_tokens() returns SpacingTokens::default() with no field behind it (theme/mod.rs, spacing_tokens), and its one reader, a Dialog's viewport margin, gets that default (dialog/dialog.rs, Dialog). So these four are the application's to apply (geometry.rs §9.5)")])),
            )
            // The resizable groups, each from its entry in RESIZABLE_GROUPS.
            .children(RESIZABLE_GROUPS.iter().map(|group| {
                v_flex()
                    .gap_5()
                    .child(section(group.heading))
                    .child(self.render_resizable_group(group, &fi, &t, cx))
            }))
            // Dividers
            .child(section("Separator (solid / dashed / labeled)"))
            .child(
                div()
                    .id("tt-layout-divider")
                    .child(
                        v_flex()
                            .gap_3()
                            .child(Separator::horizontal())
                            .child(Separator::horizontal().label("Section Break"))
                            .child(Separator::horizontal_dashed()),
                    )
                    .on_hover(self.hover_info(&fi, "Separator", &[("line", "border", t.border, "gpui-component/separator.rs:128"), ("label bg", "background", t.background, "gpui-component/separator.rs:149"), ("label text", "muted_foreground", t.muted_foreground, "gpui-component/separator.rs:150")], &[], &[("thickness", "Tier U, not an absence: the platform states separator.line_width and the model carries it. Upstream draws the line on an inner absolutely-positioned div at px(1.) and applies the caller's refinement to the outer container instead, so nothing reaches the line (separator.rs, Separator::render_base)")])),
            )
            // GroupBox as container
            .child(section("GroupBox as Layout Container"))
            .child(
                div()
                    .id("tt-layout-groupbox")
                    .child(
                        native_group_box(cx)
                        .title("Contained Content")
                        .fill()
                        .child(
                            v_flex()
                                .gap_2()
                                .child(
                                    Label::new(
                                        "GroupBox can wrap any content as a visual container.",
                                    )
                                    .text_sm(),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(Button::new("gb-1").native(cx, geometry::button).label("Action A"))
                                        .child(Button::new("gb-2").native(cx, geometry::button).label("Action B").primary()),
                                ),
                        ),
                    )
                    .on_hover(self.hover_info(&fi, "GroupBox (layout)", &[("fill bg", "group_box", t.group_box, "gpui-component/group_box.rs:134"), ("text", "group_box_foreground", t.group_box_foreground, "gpui-component/group_box.rs:157"), ("title", "muted_foreground", t.muted_foreground, "gpui-component/group_box.rs:147")], &[("geometry", "geometry::group_box_content: card.border.padding_*, corner_radius, line_width, color -- refined last, so the radius and the edge drawn are the card's, not upstream's radius (group_box.rs, GroupBox)".to_string())], &[])),
            )
            // Scrollable area demo
            .child(section("Scrollable Area (visible scrollbar)"))
            .child(
                div()
                    .id("tt-scrollbar")
                    .occlude()
                    .child(
                        div()
                            .id("scroll-demo-outer")
                            .h(px(150.0))
                            .w_full()
                            .demo_frame(cx)
                            .overflow_y_scrollbar()
                            .native(cx, geometry::scrollbar_gutter)
                            .child(v_flex().gap_2().p_3().children((0..20).map(|i| {
                                Label::new(SharedString::from(format!(
                                    "Scrollable item #{} - demonstrates scrollbar theming",
                                    i + 1
                                )))
                                .text_sm()
                            }))),
                    )
                    .on_hover(self.hover_info(&fi, "Scrollbar", &[("track", "scrollbar", t.scrollbar, "gpui-component/theme/mod.rs:307"), ("thumb", "scrollbar_thumb", t.scrollbar_thumb, "gpui-component/theme/mod.rs:312"), (
                                "thumb hover",
                                "scrollbar_thumb_hover",
                                t.scrollbar_thumb_hover, "gpui-component/theme/mod.rs:323")], &[
                            ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                            (
                                "show mode",
                                format!("scrollbar_mode: {:?}", t.scrollbar_mode),
                            ),
                        ("geometry", "geometry::scrollbar_gutter on the element a scroll container scrolls: scrollbar.groove_width as right padding where scrollbar.overlay_mode is false, because gpui-component overlays the bar on the scroll area instead of putting it beside the content. The track width and the minimum thumb length are the platform's too, written onto gpui-base by base_layer::scrollbar_styles".to_string())], &[
                        ])),
            )
            // Accordion
            .child(section("Accordion"))
            .child(
                div()
                    .id("tt-accordion")
                    .child(
                        Accordion::new("acc-1")
                            .item(|item| {
                                with_accordion_title_style(item, &accordion_title_style)
                                    .title("What is native-theme?")
                                    .open(true)
                                    .child(
                                        Label::new(
                                            "A cross-platform theme abstraction that reads OS \
                                             settings and maps them to toolkit-specific themes.",
                                        )
                                        .text_sm(),
                                    )
                            })
                            .item(|item| {
                                with_accordion_title_style(item, &accordion_title_style)
                                    .title("Supported toolkits")
                                    .child(
                                        Label::new("gpui-component, iced, egui, and more planned.")
                                            .text_sm(),
                                    )
                            })
                            .item(|item| {
                                with_accordion_title_style(item, &accordion_title_style)
                                    .title("How many presets?")
                                    .child(
                                        // Counted from the list itself, so the
                                        // answer cannot go stale again.
                                        Label::new(SharedString::from(format!(
                                            "{} built-in theme presets covering major OS styles.",
                                            native_theme::theme::Theme::list_presets().len()
                                        )))
                                        .text_sm(),
                                    )
                            }),
                    )
                    .on_hover(self.hover_info(&fi, "Accordion", &[("bg", "accordion", t.accordion, "gpui-component/accordion.rs:371"), ("border", "border", t.border, "showcase"), ("text", "foreground", t.foreground, "gpui-component/accordion.rs:305"), ("secondary text", "muted_foreground", t.muted_foreground, "gpui-component/accordion.rs:329")], &[("border-radius", format!("radius: {}px", t.radius.as_f32())), ("header height", "geometry::accordion_title: expander.header_height".to_string())], &[
                            ("padding", "inner (Tier U)"),
                            ("animation", "reads the theme's spring_control (accordion.rs, Accordion). Theme::motion is a writable field (theme/mod.rs, MotionTokens) the connector leaves at its default, because native-theme models no motion -- our model's gap, not upstream's"),
                        ])),
            )
            // Collapsible
            .child(section("Collapsible"))
            .child(
                div()
                    .id("tt-collapsible")
                    .child(
                        Collapsible::new()
                            .open(collapsible_open)
                            .child(
                                Button::new("coll-toggle")
                                    .label(if collapsible_open {
                                        "Click to collapse"
                                    } else {
                                        "Click to expand"
                                    })
                                    .ghost()
                                    .icon(if collapsible_open {
                                        IconName::ChevronDown
                                    } else {
                                        IconName::ChevronRight
                                    })
                                    .on_click(cx.listener(|this, _ev, _w, _cx| {
                                        this.collapsible_open = !this.collapsible_open;
                                    })),
                            )
                            .content(
                                v_flex().p_3().child(
                                    Label::new("This content is shown when collapsible is open.")
                                        .text_sm(),
                                ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Collapsible", &[("toggle text", "secondary_foreground", t.secondary_foreground, "gpui-component/button/button.rs:964"), ("toggle hover (half alpha in dark mode)", "accent", ghost_hover_fill(&t), "gpui-component/button/button.rs:1125-1131"), ("content text", "foreground", t.foreground, "gpui-component/label.rs:211")], &[], &[("fill and edge", "none: a Collapsible only shows or hides its content and paints nothing of its own (collapsible.rs, Collapsible). The accordion fill and border the panel had claimed are an Accordion's; the toggle is this demo's ghost Button"), ("animation", "reads the theme's spring_control (collapsible.rs, Collapsible), the same writable Theme::motion the Accordion uses; the connector leaves it at its default because native-theme models no motion")])),
            )
            // Carousel
            .child(section("Carousel"))
            .child(
                div()
                    .id("tt-carousel")
                    // The slide controls are positioned outside the frame, so
                    // the section leaves a button's width on either side.
                    .px_16()
                    .child(
                        Carousel::new("carousel", &self.carousel_state)
                            .w(px(360.0))
                            .child(
                                CarouselContent::new(&self.carousel_state)
                                    .h(px(120.0))
                                    .children(CAROUSEL_SLIDES.iter().enumerate().map(
                                        |(ix, (title, body))| {
                                            CarouselItem::new(
                                                ("carousel-slide", ix),
                                                ix,
                                                &self.carousel_state,
                                            )
                                            .child(
                                                v_flex()
                                                    .size_full()
                                                    .justify_center()
                                                    .gap_1()
                                                    .p_4()
                                                    .rounded(t.radius)
                                                    .bg(t.muted)
                                                    .child(Label::new(*title).font_semibold())
                                                    .child(
                                                        Label::new(*body)
                                                            .text_sm()
                                                            .text_color(t.muted_foreground),
                                                    ),
                                            )
                                        },
                                    )),
                            )
                            .child(CarouselPagination::new().children(
                                (0..CAROUSEL_SLIDES.len()).map(|ix| {
                                    let dot = CarouselPaginationItem::new(
                                        ("carousel-page", ix),
                                        ix,
                                        &self.carousel_state,
                                    )
                                    .child(SharedString::from((ix + 1).to_string()));
                                    // The last dot is the self-test's way into
                                    // the carousel: the prev/next controls
                                    // place themselves absolutely outside the
                                    // frame, so a wrapper around one of those
                                    // would take it out of the flow.
                                    match ix == CAROUSEL_SLIDES.len() - 1 {
                                        true => probe(PROBE_CAROUSEL_LAST, dot).into_any_element(),
                                        false => dot.into_any_element(),
                                    }
                                }),
                            ))
                            // The carousel's own slide controls. They take the
                            // same state as the viewport and position
                            // themselves outside the frame
                            // (`carousel/carousel.rs:757-768`), so they belong
                            // to the carousel rather than beside it.
                            .child(CarouselPrevious::new(&self.carousel_state))
                            .child(CarouselNext::new(&self.carousel_state)),
                    )
                    .on_hover(self.hover_info(&fi, "Carousel", &[("slide text", "foreground", t.foreground, "gpui-component/label.rs:211"), ("slide caption", "muted_foreground", t.muted_foreground, "gpui-component/label.rs:171"), ("focus ring", "ring", t.ring, "gpui-component/styled.rs:187")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[
                            ("slide fill", "none: a Carousel paints no slide background. The one theme colour it draws is the focus ring, on an overlay the size of the frame while focus is visible (carousel/carousel.rs, Carousel)"),
                            ("snap motion", "Theme::motion's spring_move (carousel/carousel.rs, spring_move); ResolvedTheme has no motion field"),
                            ("reduced motion", "gpui's App::reduce_motion, forwarded by apply_system_theme -- the spring jumps straight to its target (gpui-base/motion.rs, spring)"),
                            ("slide controls", "outline Buttons the widget builds itself, disabled at the ends (carousel/carousel.rs, carousel_control)"),
                        ])),
            )
            // GroupBox variants
            .child(section("GroupBox (3 variants)"))
            .child(
                div()
                    .id("tt-groupbox")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                native_group_box(cx)
                                    .title("Default")
                                    .w(px(180.0))
                                    .child(Label::new("Default style").text_sm()),
                            )
                            .child(
                                native_group_box(cx)
                                    .title("Filled")
                                    .fill()
                                    .w(px(180.0))
                                    .child(Label::new("Filled background").text_sm()),
                            )
                            .child(
                                native_group_box(cx)
                                    .title("Outline")
                                    .outline()
                                    .w(px(180.0))
                                    .child(Label::new("Outlined border").text_sm()),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "GroupBox", &[("fill bg", "group_box", t.group_box, "gpui-component/group_box.rs:134"), ("text", "group_box_foreground", t.group_box_foreground, "gpui-component/group_box.rs:157"), ("title", "muted_foreground", t.muted_foreground, "gpui-component/group_box.rs:147")], &[("geometry", "geometry::group_box_content: card.border.padding_*, corner_radius, line_width, color -- refined last, so the radius and the edge drawn are the card's, not upstream's radius (group_box.rs, GroupBox)".to_string())], &[])),
            )
            // Breadcrumb (with page navigation)
            .child(section("Breadcrumb (click to navigate pages)"))
            .child(
                div()
                    .id("tt-breadcrumb")
                    .child(
                        Breadcrumb::new()
                            .child(BreadcrumbItem::new("Buttons").on_click(cx.listener(
                                |this, _ev, _w, cx| this.show_page(Page::Buttons, cx),
                            )))
                            .child(BreadcrumbItem::new("Inputs").on_click(cx.listener(
                                |this, _ev, _w, cx| this.show_page(Page::Inputs, cx),
                            )))
                            .child(BreadcrumbItem::new("Data").on_click(cx.listener(
                                |this, _ev, _w, cx| this.show_page(Page::Data, cx),
                            )))
                            .child(BreadcrumbItem::new("Feedback").on_click(cx.listener(
                                |this, _ev, _w, cx| this.show_page(Page::Feedback, cx),
                            )))
                            .child(BreadcrumbItem::new("Layout")),
                    )
                    .on_hover(self.hover_info(&fi, "Breadcrumb", &[("last item", "foreground", t.foreground, "gpui-component/breadcrumb.rs:102"), ("non-last + separators", "muted_foreground", t.muted_foreground, "gpui-component/breadcrumb.rs:101")], &[], &[
                            ("separator icon", "a ChevronRight built inline with no setter to replace it (breadcrumb.rs, Breadcrumb) -- unlike an Alert's icon, which Alert::icon takes"),
                            ("spacing", "gap_1p5 (breadcrumb.rs, Breadcrumb) -- rems again -- and applied before the caller's refinement. native-theme states no breadcrumb widget. Our gap"),
                        ])),
            )
            // Stepper
            .child(section(format!(
                "Stepper (step {} of {}: one completed, one current, one pending)",
                self.step + 1,
                STEPPER_STEPS.len()
            )))
            .child(
                div()
                    .id("tt-stepper")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .w(px(480.0))
                            .child(probe(
                                PROBE_STEPPER,
                                Stepper::new("stepper-1")
                                    .selected_index(self.step)
                                    .items(STEPPER_STEPS.iter().map(|(label, icon)| {
                                        // The indicator is a circle
                                        // (stepper/trigger.rs:118-123) around
                                        // the icon it is given, which keeps its
                                        // size (`:138-139`).
                                        StepperItem::new()
                                            .icon(native_icon(
                                                cx,
                                                icon.clone(),
                                                geometry::icon_size_small,
                                            ))
                                            .child(Label::new(*label).text_sm())
                                    }))
                                    .on_click(cx.listener(|this, step: &usize, _w, cx| {
                                        this.step = *step;
                                        cx.notify();
                                    })),
                            ))
                            .child(
                                Stepper::new("stepper-vertical")
                                    .vertical()
                                    .selected_index(self.step)
                                    .items(STEPPER_STEPS.iter().map(|(label, _)| {
                                        StepperItem::new().child(Label::new(*label).text_sm())
                                    }))
                                    .on_click(cx.listener(|this, step: &usize, _w, cx| {
                                        this.step = *step;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Stepper", &[("completed / current", "primary", t.primary, "gpui-component/stepper/trigger.rs:133"), ("separator", "border", t.border, "gpui-component/stepper/item.rs:263"), ("passed separator", "primary", t.primary, "gpui-component/stepper/item.rs:264"), ("completed text", "primary_foreground", t.primary_foreground, "gpui-component/stepper/trigger.rs:134"), ("pending", "secondary", t.secondary, "gpui-component/stepper/trigger.rs:126"), ("pending text", "secondary_foreground", t.secondary_foreground, "gpui-component/stepper/trigger.rs:131"), ("pending hover", "secondary_hover", t.secondary_hover, "gpui-component/stepper/trigger.rs:128")], &[("icon size", "geometry::icon_size_small: defaults.icon_sizes.small".to_string())], &[
                            ("indicator size", "24px for Size::Medium (stepper/item.rs, StepperItem::render icon_size)"),
                            ("separator", "drawn by the item, absolute (stepper/item.rs: StepperItem::render builds it, StepperSeparator::render positions it)"),
                        ])),
            )
            // Form / Field
            .child(section("Form / Field (horizontal layout)"))
            .child(
                div()
                    .id("tt-form")
                    .child(
                        form::Form::horizontal()
                            .label_width(px(100.0))
                            .child(
                                Field::new().label("Name").required(true).child(
                                    Input::new(&self.form_name_state)
                                        .native(cx, geometry::input),
                                ),
                            )
                            .child(
                                Field::new()
                                    .label("Email")
                                    .description("We will never share your email.")
                                    .child(
                                        Input::new(&self.form_email_state)
                                            .native(cx, geometry::input),
                                    ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Form / Field", &[("label", "foreground", t.foreground, "gpui-component/root.rs:596"), ("description", "muted_foreground", t.muted_foreground, "gpui-component/form/field.rs:338"), ("required marker", "danger", t.danger, "gpui-component/form/field.rs:315")], &[("geometry", "geometry::input on each field's input: input.min_height (control height), border.corner_radius, line_width, input.font. The Field wrapper takes none -- the model has no form theme".to_string())], &[
                            ("layout", "horizontal/vertical"),
                            ("label width", "configurable"),
                        ])),
            )
            // Sidebar
            .child(section(
                "Sidebar (mini navigation; the toggle button collapses it)",
            ))
            .child(
                div()
                    .id("tt-sidebar-toggle")
                    .child(
                        with_gap(h_flex(), widget_gap)
                            .items_center()
                            // `SidebarToggleButton` owns no collapsed state of
                            // its own (`sidebar/mod.rs:302-307`): the flag it
                            // draws and the flag the sidebar reads are the
                            // same one, here.
                            .child(probe(
                                PROBE_SIDEBAR_TOGGLE,
                                SidebarToggleButton::new()
                                    .collapsed(self.sidebar_collapsed)
                                    .on_click(cx.listener(|this, _ev, _w, cx| {
                                        this.sidebar_collapsed = !this.sidebar_collapsed;
                                        cx.notify();
                                    })),
                            ))
                            .child(
                                Label::new(if self.sidebar_collapsed {
                                    "collapsed"
                                } else {
                                    "expanded"
                                })
                                .text_sm()
                                .text_color(t.muted_foreground),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "SidebarToggleButton", &[("hover (half alpha in dark mode)", "accent", ghost_hover_fill(&t), "gpui-component/button/button.rs:1125-1131"), ("icon", "secondary_foreground", t.secondary_foreground, "gpui-component/button/button.rs:964"), ("icon on hover", "accent_foreground", t.accent_foreground, "gpui-component/button/button.rs:1141")], &[], &[
                            ("button", "a ghost, small Button built by the widget (sidebar/mod.rs, SidebarToggleButton::new)"),
                            ("fill", "none until hovered: a ghost Button is transparent, and it hovers with accent (at half alpha in dark mode) rather than the button family -- the panel claimed secondary_hover, which no widget-built ghost reads"),
                            ("icon", "PanelLeftOpen / PanelLeftClose at size_4 -- 1rem, so the platform's font size rather than a literal (sidebar/mod.rs, SidebarToggleButton::render)"),
                        ])),
            )
            .child(
                div()
                    .id("tt-sidebar")
                    .h(px(240.0))
                    .w(px(280.0))
                    .demo_frame(cx)
                    .child(
                        // A sidebar is the panel `defaults.icon_sizes.panel`
                        // names, and `SidebarMenuItem` keeps the icon it is
                        // given (`sidebar/menu.rs:300`), so the size arrives.
                        Sidebar::new("layout-sidebar")
                            .collapsed(self.sidebar_collapsed)
                            .child(
                            SidebarMenu::new()
                                .child(
                                    SidebarMenuItem::new("Dashboard")
                                        .icon(native_icon(
                                            cx,
                                            IconName::LayoutDashboard,
                                            geometry::icon_size_panel,
                                        ))
                                        .active(true),
                                )
                                .child(SidebarMenuItem::new("Settings").icon(native_icon(
                                    cx,
                                    IconName::Settings,
                                    geometry::icon_size_panel,
                                )))
                                .child(SidebarMenuItem::new("Inbox").icon(native_icon(
                                    cx,
                                    IconName::Inbox,
                                    geometry::icon_size_panel,
                                )))
                                .child(SidebarMenuItem::new("Calendar").icon(native_icon(
                                    cx,
                                    IconName::Calendar,
                                    geometry::icon_size_panel,
                                ))),
                        ),
                    )
                    .on_hover(self.hover_info(&fi, "Sidebar", &[("bg", "sidebar", t.sidebar, "gpui-component/sidebar/mod.rs:413"), ("text", "sidebar_foreground", t.sidebar_foreground, "gpui-component/sidebar/mod.rs:414"), ("selected row", "sidebar_accent", t.sidebar_accent, "gpui-component/sidebar/menu.rs:297"), ("border", "sidebar_border", t.sidebar_border, "gpui-component/sidebar/mod.rs:415")], &[("icon size", "geometry::icon_size_panel: defaults.icon_sizes.panel, on each SidebarMenuItem icon".to_string())], &[
                            ("selected row text", "sidebar_accent_foreground, beside the fill, at font_medium; a hovered row takes the same pair with the fill at 80% (sidebar/menu.rs, SidebarMenuItem)"),
                            ("width", "255px is only a fallback: the expanded width is read from the caller's own style and used whenever it is an absolute pixel length (sidebar/mod.rs, sidebar_expanded_width), so .w() carries it. Only the collapsed 48px is fixed. SidebarTheme models no width -- our model's gap"),
                            ("children", "must impl SidebarItem, which asks for Collapsible + Clone (sidebar/mod.rs, SidebarItem)"),
                        ])),
            )
            // Settings
            .child(section("Settings (page with field types)"))
            .child(
                div()
                    .id("tt-settings")
                    .debug_selector(|| "settings-frame".into())
                    .h(px(320.0))
                    .w_full()
                    .occlude()
                    .demo_frame(cx)
                    .overflow_y_scroll()
                    .child(
                        Settings::new("settings-demo")
                            .sidebar_width(px(140.0))
                            .page(
                                SettingPage::new("Appearance")
                                    .description("Customize the look and feel")
                                    .default_open(true)
                                    .group(
                                        SettingGroup::new()
                                            .native(cx, geometry::scrollbar_gutter)
                                            .title("Theme")
                                            .item(
                                                SettingItem::new(
                                                    "Dark Mode",
                                                    SettingField::switch(
                                                        |_cx| false,
                                                        |_val, _cx| {},
                                                    ),
                                                )
                                                .description("Toggle dark appearance"),
                                            )
                                            // The row probe is a whole-row item
                                            // (setting/item.rs, SettingItem::render):
                                            // a field's slot is only as wide as its
                                            // content, so it would not span the row.
                                            .item(SettingItem::render(|_, _, _| {
                                                div()
                                                    .w_full()
                                                    .h(px(1.))
                                                    .debug_selector(|| PROBE_SETTINGS_ROW.into())
                                            }))
                                            .item(SettingItem::new(
                                                "Accent Color",
                                                SettingField::dropdown(
                                                    vec![
                                                        ("blue".into(), "Blue".into()),
                                                        ("green".into(), "Green".into()),
                                                        ("red".into(), "Red".into()),
                                                    ],
                                                    |_cx| "blue".into(),
                                                    |_val, _cx| {},
                                                ),
                                            )),
                                    )
                                    .group(
                                        SettingGroup::new()
                                            .native(cx, geometry::scrollbar_gutter)
                                            .title("Editor")
                                            .item(SettingItem::new(
                                                "Font Size",
                                                SettingField::input(
                                                    |_cx| "14".into(),
                                                    |_val, _cx| {},
                                                ),
                                            ))
                                            .item(SettingItem::new(
                                                "Word Wrap",
                                                SettingField::checkbox(|_cx| true, |_val, _cx| {}),
                                            )),
                                    ),
                            )
                            .page(
                                SettingPage::new("Keyboard")
                                    .description("Keyboard shortcuts and input")
                                    .group(
                                        SettingGroup::new()
                                            .native(cx, geometry::scrollbar_gutter)
                                            .title("Shortcuts")
                                            .item(SettingItem::new(
                                                "Vim Mode",
                                                SettingField::switch(|_cx| false, |_val, _cx| {}),
                                            )),
                                    ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Settings", &[("sidebar", "sidebar", t.sidebar, "gpui-component/sidebar/mod.rs:413"), ("border", "border", t.border, "gpui-component/setting/page.rs:185")], &[("geometry", "geometry::scrollbar_gutter on each SettingGroup: scrollbar.groove_width as right padding where scrollbar.overlay_mode is false, so the page's own scrollbar, laid over the body's right edge, does not cover the rows".to_string())], &[("groups", "no fill and no edge: a Settings page uses GroupBoxVariant::Normal unless Settings::with_group_variant picks Fill or Outline, so group_box paints nothing here (setting/settings.rs, with_group_variant)"), 
                            ("fill", "none of its own: nothing under setting/ sets a background, so a Settings page shows the window's (gpui-component setting/)"),
                            ("descriptions", "muted_foreground, set per item, group and page (setting/item.rs, setting/group.rs, setting/page.rs)"),
                            ("fields", "switch, checkbox, input, number input, dropdown, or an element of the application's own (setting/fields/mod.rs, SettingFieldType)"),
                            ("layout", "sidebar + pages"),
                            ("scrollbar", "gpui-component lays the page's scrollbar over the body's right edge (setting/page.rs, SettingPage) and reserves only 1rem; this demo pads each group by the platform's groove width instead (setting/group.rs, SettingGroup). The page body takes no refinement -- Tier U"),
                        ])),
            )
    }
}
