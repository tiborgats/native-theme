//! The Feedback page.

use gpui::{Context, IntoElement, ParentElement, Styled, Window, prelude::*, px};
use gpui_component::{IconName, h_flex, v_flex};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{self, CircleKind, MarkerKind, Severity, ShimmerKind, SpinnerKind, TagKind};
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
const TAGS: [(&str, TagKind, bool, &str); 8] = [
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

/// The Badges, as `(id, count or a dot, the badged Button's label)`.
const BADGES: [(&str, Option<usize>, &str); 3] = [
    (FEEDBACK_BADGE_COUNT, Some(5), "Messages"),
    ("feedback-badge-notifications", Some(99), "Notifications"),
    (FEEDBACK_BADGE_DOT, None, "Updates"),
];

/// The Markers, as `(id, kind, text)`.
const MARKERS: [(&str, MarkerKind, &str); 5] = [
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
        let marker_icon = self.sample_icon(IconName::CircleCheck);
        let widget_gap = geometry::widget_gap(&self.layout);
        v_flex()
            .gap_5()
            .p_4()
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
            .child(v_flex().gap_3().w(px(360.0)).children(BARS.map(
                |(id, label_id, label, value_id, value)| {
                    v_flex()
                        .gap_3()
                        .child(
                            h_flex()
                                .justify_between()
                                .child(demo::label(ui, cx, label_id, label))
                                .child(demo::label(ui, cx, value_id, format!("{value}%"))),
                        )
                        .child(demo::progress(ui, cx, id, label, value))
                },
            )))
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
            .child(h_flex().gap_6().items_center().children(SPINNERS.map(
                |(id, label_id, kind)| {
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(demo::spinner(ui, cx, id, kind))
                        .child(demo::label(ui, cx, label_id, kind.name()))
                },
            )))
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-skeleton",
                "Skeleton Placeholders",
            ))
            .child(
                v_flex()
                    .gap_2()
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
                "Tags (7 colors + outline)",
            ))
            .child(h_flex().gap_2().flex_wrap().children(
                TAGS.map(|(id, kind, outline, label)| demo::tag(ui, cx, id, kind, outline, label)),
            ))
            .child(demo::heading(ui, cx, "feedback-heading-badge", "Badge"))
            .child(
                h_flex().gap_8().children(
                    BADGES.map(|(id, count, label)| demo::badge(ui, cx, id, count, label)),
                ),
            )
            .child(demo::heading(
                ui,
                cx,
                "feedback-heading-marker",
                "Marker (3 variants, 2 loading styles)",
            ))
            .child(with_gap(v_flex(), widget_gap).w(px(360.0)).children(
                MARKERS.map(|(id, kind, text)| demo::marker(ui, cx, id, kind, text, &marker_icon)),
            ))
            .child(demo::heading(ui, cx, "feedback-heading-tooltip", "Tooltip"))
            .child(
                h_flex()
                    .gap_4()
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
                        "Save file (Cmd+S)",
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
                h_flex()
                    .gap_3()
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
