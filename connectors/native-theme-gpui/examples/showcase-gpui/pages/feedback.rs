//! The Feedback page.

use gpui::{Context, IntoElement, Keystroke, ParentElement, Styled, Window, prelude::*, px};
use gpui_component::{IconName, h_flex, kbd::Kbd, v_flex};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{self, CircleKind, MarkerKind, Severity, ShimmerKind, SpinnerKind, TagKind};
use crate::pages::page_root;
use crate::support::with_gap;
use crate::{
    FEEDBACK_ALERT_INFO, FEEDBACK_BADGE_COUNT, FEEDBACK_BADGE_DOT, FEEDBACK_CIRCLE_LOADING,
    FEEDBACK_SPINNER_SMALL, FEEDBACK_TAG_DANGER, FEEDBACK_TAG_PRIMARY, PROBE_NOTIFICATION, probe,
};

/// The Alerts, as `(id, severity, message)`.
const ALERTS: [(&str, Severity, &str); 4] = [
    (
        FEEDBACK_ALERT_INFO,
        Severity::Info,
        "This is an informational message.",
    ),
    (
        "feedback-alert-success",
        Severity::Success,
        "Operation completed successfully.",
    ),
    (
        "feedback-alert-warning",
        Severity::Warning,
        "Please review before proceeding.",
    ),
    (
        "feedback-alert-error",
        Severity::Error,
        "Something went wrong. Please try again.",
    ),
];

/// The Progress bars, as `(id, label id, label, value id, value)`.
const BARS: [(&str, &str, &str, &str, f32); 3] = [
    (
        "feedback-progress-upload",
        "feedback-progress-upload-label",
        "Upload",
        "feedback-progress-upload-value",
        73.0,
    ),
    (
        "feedback-progress-processing",
        "feedback-progress-processing-label",
        "Processing",
        "feedback-progress-processing-value",
        45.0,
    ),
    (
        "feedback-progress-complete",
        "feedback-progress-complete-label",
        "Complete",
        "feedback-progress-complete-value",
        100.0,
    ),
];

/// The Spinners, as `(id, label id, kind)`.
const SPINNERS: [(&str, &str, SpinnerKind); 3] = [
    (
        FEEDBACK_SPINNER_SMALL,
        "feedback-spinner-small-label",
        SpinnerKind::Small,
    ),
    (
        "feedback-spinner-medium",
        "feedback-spinner-medium-label",
        SpinnerKind::Medium,
    ),
    (
        "feedback-spinner-large",
        "feedback-spinner-large-label",
        SpinnerKind::Large,
    ),
];

/// The Tags, as `(id, variant, outlined, label)`.
pub(crate) const TAGS: [(&str, TagKind, bool, &str); 8] = [
    (FEEDBACK_TAG_PRIMARY, TagKind::Primary, false, "Primary"),
    (
        "feedback-tag-secondary",
        TagKind::Secondary,
        false,
        "Secondary",
    ),
    (FEEDBACK_TAG_DANGER, TagKind::Danger, false, "Danger"),
    ("feedback-tag-success", TagKind::Success, false, "Success"),
    ("feedback-tag-warning", TagKind::Warning, false, "Warning"),
    ("feedback-tag-info", TagKind::Info, false, "Info"),
    (
        "feedback-tag-primary-outline",
        TagKind::Primary,
        true,
        "Primary Outline",
    ),
    (
        "feedback-tag-danger-outline",
        TagKind::Danger,
        true,
        "Danger Outline",
    ),
];

/// The Tags section's heading, counting the colours from [`TAGS`]: one
/// filled Tag per colour, then the outlined ones.
pub(crate) fn tags_heading() -> String {
    let colours = TAGS
        .iter()
        .filter(|&&(_, _, outlined, _)| !outlined)
        .count();
    format!("Tags ({colours} colors + outline)")
}

/// The Markers section's heading, counting [`MARKERS`]: its variants, then
/// its loading styles ([`MarkerKind::is_loading`]).
pub(crate) fn markers_heading() -> String {
    let loading = MARKERS
        .iter()
        .filter(|&&(_, kind, _)| kind.is_loading())
        .count();
    let variants = MARKERS.len().saturating_sub(loading);
    format!("Marker ({variants} variants, {loading} loading styles)")
}

/// The keystroke the shortcut tooltip names, a sample and no binding of the
/// showcase's: on gpui's `secondary` modifier, the one the showcase's
/// bindings take (`app::init`), Ctrl off macOS and Cmd on it.
pub(crate) const SAVE_KEYS: &str = "secondary-s";

/// The shortcut tooltip's text: [`SAVE_KEYS`] as a `Kbd` draws it on this
/// platform (gpui-component kbd.rs, Kbd::format), "Ctrl+S" off macOS. The
/// keys as written where gpui cannot parse them.
pub(crate) fn save_tooltip() -> String {
    let keys = Keystroke::parse(SAVE_KEYS)
        .map_or_else(|_| SAVE_KEYS.to_string(), |stroke| Kbd::format(&stroke));
    format!("Save file ({keys})")
}

/// The Badges, as `(id, count or a dot, the badged Button's label)`.
const BADGES: [(&str, Option<usize>, &str); 3] = [
    (FEEDBACK_BADGE_COUNT, Some(5), "Messages"),
    ("feedback-badge-notifications", Some(99), "Notifications"),
    (FEEDBACK_BADGE_DOT, None, "Updates"),
];

/// The Markers, as `(id, kind, text)`.
pub(crate) const MARKERS: [(&str, MarkerKind, &str); 5] = [
    ("feedback-marker-plain", MarkerKind::Plain, "Theme applied"),
    ("feedback-marker-separator", MarkerKind::Separator, "Today"),
    (
        "feedback-marker-border",
        MarkerKind::Border,
        "Unread from here",
    ),
    (
        "feedback-marker-spinner",
        MarkerKind::Spinner,
        "Reading the OS theme…",
    ),
    (
        "feedback-marker-shimmer",
        MarkerKind::Shimmer,
        "Resolving the palette…",
    ),
];

/// The Notification triggers after the first, which carries the probe, as
/// `(id, severity, label, message)`.
const NOTIFICATIONS: [(&str, Severity, &str, &str); 3] = [
    (
        "feedback-notify-success",
        Severity::Success,
        "Success",
        "Operation completed.",
    ),
    (
        "feedback-notify-warning",
        Severity::Warning,
        "Warning",
        "Careful with this action.",
    ),
    (
        "feedback-notify-error",
        Severity::Error,
        "Error",
        "Something went wrong.",
    ),
];

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Feedback
    // -----------------------------------------------------------------------
    pub(crate) fn render_feedback_page(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let ui = &self.info_ui;
        let spinner_icons = self.spinner_icons();
        let marker_icon = self.sample_icon(IconName::CircleCheck);
        let widget_gap = geometry::widget_gap(&self.layout);
        page_root(&self.layout)
            .flex_1()
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-alerts",
                "Alerts (all 4 variants with icons)",
            ))
            .children(
                ALERTS.map(|(id, severity, message)| {
                    demo::severity_alert(ui, cx, id, severity, message)
                }),
            )
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-progress",
                "Progress Bars",
            ))
            .child(
                with_gap(v_flex(), widget_gap)
                    .w(px(360.0))
                    .children(BARS.map(|(id, label_id, label, value_id, value)| {
                        with_gap(v_flex(), widget_gap)
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(demo::label(ui, cx, label_id, label))
                                    .child(demo::label(ui, cx, value_id, format!("{value}%"))),
                            )
                            .child(demo::progress(ui, cx, id, label, Some(value)))
                    })),
            )
            // Indeterminate: the fill sweeps the track.
            .child(
                with_gap(v_flex(), widget_gap)
                    .w(px(360.0))
                    .child(demo::label(
                        ui,
                        cx,
                        "feedback-progress-loading-label",
                        "Loading",
                    ))
                    .child(demo::progress(
                        ui,
                        cx,
                        "feedback-progress-loading",
                        "Loading",
                        None,
                    )),
            )
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-progress-circle",
                "ProgressCircle (determinate and indeterminate)",
            ))
            .child(
                with_gap(h_flex(), widget_gap)
                    .items_center()
                    .child(
                        with_gap(h_flex(), widget_gap)
                            .items_center()
                            .child(demo::progress_circle(
                                ui,
                                cx,
                                "feedback-circle-73",
                                CircleKind::Value,
                            ))
                            .child(demo::label(ui, cx, "feedback-circle-73-label", "73%")),
                    )
                    .child(
                        with_gap(h_flex(), widget_gap)
                            .items_center()
                            .child(demo::progress_circle(
                                ui,
                                cx,
                                "feedback-circle-100",
                                CircleKind::ValueLarge,
                            ))
                            .child(demo::label(ui, cx, "feedback-circle-100-label", "100%")),
                    )
                    .child(demo::progress_circle(
                        ui,
                        cx,
                        FEEDBACK_CIRCLE_LOADING,
                        CircleKind::Indeterminate,
                    ))
                    .child(demo::caption(
                        ui,
                        cx,
                        "feedback-circle-loading-label",
                        "indeterminate",
                    )),
            )
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-spinner",
                "Spinner (3 sizes)",
            ))
            .child(
                with_gap(h_flex(), widget_gap)
                    .items_center()
                    .children(SPINNERS.map(|(id, label_id, kind)| {
                        with_gap(h_flex(), widget_gap)
                            .items_center()
                            .child(demo::spinner(ui, cx, id, kind, &spinner_icons))
                            .child(demo::label(ui, cx, label_id, kind.name()))
                    })),
            )
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-skeleton",
                "Skeleton Placeholders",
            ))
            .child(
                with_gap(v_flex(), widget_gap)
                    .w(px(360.0))
                    // The block keeps the larger of the two roundings this
                    // section always had.
                    .child(demo::skeleton(
                        ui,
                        cx,
                        "feedback-skeleton-title",
                        false,
                        12.0,
                        Some(200.0),
                        false,
                    ))
                    .child(demo::skeleton(
                        ui,
                        cx,
                        "feedback-skeleton-line-1",
                        false,
                        8.0,
                        Some(300.0),
                        false,
                    ))
                    .child(demo::skeleton(
                        ui,
                        cx,
                        "feedback-skeleton-line-2",
                        false,
                        8.0,
                        Some(250.0),
                        false,
                    ))
                    .child(demo::skeleton(
                        ui,
                        cx,
                        "feedback-skeleton-block",
                        true,
                        60.0,
                        None,
                        true,
                    )),
            )
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-shimmer",
                "ShimmerText (a highlight sweeping across the label)",
            ))
            .child(
                with_gap(v_flex(), widget_gap)
                    .w(px(360.0))
                    .child(demo::shimmer_text(
                        ui,
                        cx,
                        "feedback-shimmer-default",
                        ShimmerKind::Default,
                        "Reading the desktop configuration…",
                    ))
                    .child(demo::shimmer_text(
                        ui,
                        cx,
                        "feedback-shimmer-slow",
                        ShimmerKind::Slow,
                        "Resolving the palette…",
                    ))
                    .child(demo::shimmer_text(
                        ui,
                        cx,
                        "feedback-shimmer-reverse",
                        ShimmerKind::Reverse,
                        "Applying to gpui…",
                    )),
            )
            .child(demo::heading(ui, cx, "feedback-heading-empty", "Empty"))
            .child(
                demo::empty(
                    ui,
                    cx,
                    "feedback-empty",
                    "feedback-empty-refresh",
                    "No notifications",
                    "Anything the application reports shows up here.",
                    &self.sample_icon(IconName::Inbox),
                )
                .w(px(360.0)),
            )
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-tags",
                tags_heading(),
            ))
            .child(with_gap(h_flex(), widget_gap).flex_wrap().children(
                TAGS.map(|(id, kind, outline, label)| demo::tag(ui, cx, id, kind, outline, label)),
            ))
            .child(demo::heading(ui, cx, "feedback-heading-badge", "Badge"))
            .child(
                h_flex().children(BADGES.iter().enumerate().map(|(ix, &(id, count, label))| {
                    let next = (ix.saturating_add(1) < BADGES.len()).then_some(widget_gap);
                    demo::badge(ui, cx, id, count, label, next)
                })),
            )
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-marker",
                markers_heading(),
            ))
            .child(with_gap(v_flex(), widget_gap).w(px(360.0)).children(
                MARKERS.map(|(id, kind, text)| demo::marker(ui, cx, id, kind, text, &marker_icon)),
            ))
            .child(demo::heading(ui, cx, "feedback-heading-tooltip", "Tooltip"))
            .child(
                with_gap(h_flex(), widget_gap)
                    .child(demo::tooltip_button(
                        ui,
                        cx,
                        "feedback-tooltip-hover",
                        "Hover me",
                        "This is a tooltip",
                    ))
                    .child(demo::tooltip_button(
                        ui,
                        cx,
                        "feedback-tooltip-shortcut",
                        "With tooltip",
                        save_tooltip(),
                    ))
                    .child(demo::built_tooltip_button(
                        ui,
                        cx,
                        "feedback-tooltip-built",
                        "Built by the application",
                        "This popup carries geometry::tooltip and geometry::tooltip_content: \
                         the platform's padding, radius, text size and text colour, and the \
                         max width the text wraps at.",
                    )),
            )
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-notification",
                "Notification (push via WindowExt)",
            ))
            .child(
                with_gap(h_flex(), widget_gap)
                    .child(probe(
                        PROBE_NOTIFICATION,
                        demo::notification_button(
                            ui,
                            cx,
                            "feedback-notify-info",
                            Severity::Info,
                            "Info",
                            "This is an info notification.",
                        ),
                    ))
                    .children(NOTIFICATIONS.map(|(id, severity, label, message)| {
                        demo::notification_button(ui, cx, id, severity, label, message)
                    })),
            )
    }
}
