//! The Theme Map page.

use gpui::{
    Context, IntoElement, ParentElement, Pixels, StyleRefinement, Styled, div, prelude::*, px,
};
use gpui_component::v_flex;

use crate::app::Showcase;
use crate::demo::{self, ThemeToken};
use crate::support::NativeStyled as _;

/// The space between two swatches of a row, and between two rows: the
/// showcase's own, as the model states no swatch table.
const SWATCH_GAP_X: Pixels = px(16.);
const SWATCH_GAP_Y: Pixels = px(4.);

/// The swatch table, as `(heading id, heading, fields)`: every ThemeColor
/// field once.
pub(crate) const THEME_MAP_GROUPS: [(&str, &str, &[ThemeToken]); 18] = [
    (
        "theme-map-heading-core",
        "Core",
        &[
            ThemeToken::Background,
            ThemeToken::Foreground,
            ThemeToken::Accent,
            ThemeToken::AccentForeground,
            ThemeToken::Border,
            ThemeToken::Muted,
            ThemeToken::MutedForeground,
            ThemeToken::Input,
            ThemeToken::Ring,
            ThemeToken::Selection,
            ThemeToken::Caret,
            ThemeToken::Link,
            ThemeToken::LinkHover,
            ThemeToken::LinkActive,
            ThemeToken::Overlay,
        ],
    ),
    (
        "theme-map-heading-primary",
        "Primary",
        &[
            ThemeToken::Primary,
            ThemeToken::PrimaryForeground,
            ThemeToken::PrimaryHover,
            ThemeToken::PrimaryActive,
        ],
    ),
    (
        "theme-map-heading-secondary",
        "Secondary",
        &[
            ThemeToken::Secondary,
            ThemeToken::SecondaryForeground,
            ThemeToken::SecondaryHover,
            ThemeToken::SecondaryActive,
        ],
    ),
    (
        "theme-map-heading-button",
        "Button (28 fields, from secondary/primary/status)",
        &[
            ThemeToken::Button,
            ThemeToken::ButtonHover,
            ThemeToken::ButtonActive,
            ThemeToken::ButtonForeground,
            ThemeToken::ButtonSecondary,
            ThemeToken::ButtonSecondaryHover,
            ThemeToken::ButtonSecondaryActive,
            ThemeToken::ButtonSecondaryForeground,
            ThemeToken::ButtonPrimary,
            ThemeToken::ButtonPrimaryHover,
            ThemeToken::ButtonPrimaryActive,
            ThemeToken::ButtonPrimaryForeground,
            ThemeToken::ButtonDanger,
            ThemeToken::ButtonDangerHover,
            ThemeToken::ButtonDangerActive,
            ThemeToken::ButtonDangerForeground,
            ThemeToken::ButtonInfo,
            ThemeToken::ButtonInfoHover,
            ThemeToken::ButtonInfoActive,
            ThemeToken::ButtonInfoForeground,
            ThemeToken::ButtonSuccess,
            ThemeToken::ButtonSuccessHover,
            ThemeToken::ButtonSuccessActive,
            ThemeToken::ButtonSuccessForeground,
            ThemeToken::ButtonWarning,
            ThemeToken::ButtonWarningHover,
            ThemeToken::ButtonWarningActive,
            ThemeToken::ButtonWarningForeground,
        ],
    ),
    (
        "theme-map-heading-danger",
        "Danger",
        &[
            ThemeToken::Danger,
            ThemeToken::DangerForeground,
            ThemeToken::DangerHover,
            ThemeToken::DangerActive,
            ThemeToken::Red,
            ThemeToken::RedLight,
        ],
    ),
    (
        "theme-map-heading-success",
        "Success",
        &[
            ThemeToken::Success,
            ThemeToken::SuccessForeground,
            ThemeToken::SuccessHover,
            ThemeToken::SuccessActive,
            ThemeToken::Green,
            ThemeToken::GreenLight,
        ],
    ),
    (
        "theme-map-heading-warning",
        "Warning",
        &[
            ThemeToken::Warning,
            ThemeToken::WarningForeground,
            ThemeToken::WarningHover,
            ThemeToken::WarningActive,
            ThemeToken::Yellow,
            ThemeToken::YellowLight,
        ],
    ),
    (
        "theme-map-heading-info",
        "Info",
        &[
            ThemeToken::Info,
            ThemeToken::InfoForeground,
            ThemeToken::InfoHover,
            ThemeToken::InfoActive,
            ThemeToken::Blue,
            ThemeToken::BlueLight,
        ],
    ),
    (
        "theme-map-heading-list",
        "List",
        &[
            ThemeToken::List,
            ThemeToken::ListActive,
            ThemeToken::ListActiveBorder,
            ThemeToken::ListEven,
            ThemeToken::ListHead,
            ThemeToken::ListHover,
        ],
    ),
    (
        "theme-map-heading-table",
        "Table",
        &[
            ThemeToken::Table,
            ThemeToken::TableActive,
            ThemeToken::TableActiveBorder,
            ThemeToken::TableEven,
            ThemeToken::TableHead,
            ThemeToken::TableHeadForeground,
            ThemeToken::TableFoot,
            ThemeToken::TableFootForeground,
            ThemeToken::TableHover,
            ThemeToken::TableRowBorder,
        ],
    ),
    (
        "theme-map-heading-tab",
        "Tab",
        &[
            ThemeToken::Tab,
            ThemeToken::TabActive,
            ThemeToken::TabActiveForeground,
            ThemeToken::TabBar,
            ThemeToken::TabBarSegmented,
            ThemeToken::TabForeground,
        ],
    ),
    (
        "theme-map-heading-sidebar",
        "Sidebar",
        &[
            ThemeToken::Sidebar,
            ThemeToken::SidebarForeground,
            ThemeToken::SidebarAccent,
            ThemeToken::SidebarAccentForeground,
            ThemeToken::SidebarBorder,
            ThemeToken::SidebarPrimary,
            ThemeToken::SidebarPrimaryForeground,
        ],
    ),
    (
        "theme-map-heading-scrollbar",
        "Scrollbar",
        &[
            ThemeToken::Scrollbar,
            ThemeToken::ScrollbarThumb,
            ThemeToken::ScrollbarThumbHover,
        ],
    ),
    (
        "theme-map-heading-accordion",
        "Accordion",
        &[ThemeToken::Accordion],
    ),
    (
        "theme-map-heading-group-box",
        "GroupBox",
        &[ThemeToken::GroupBox, ThemeToken::GroupBoxForeground],
    ),
    (
        "theme-map-heading-chart",
        "Chart",
        &[
            ThemeToken::Chart1,
            ThemeToken::Chart2,
            ThemeToken::Chart3,
            ThemeToken::Chart4,
            ThemeToken::Chart5,
            ThemeToken::ChartBullish,
            ThemeToken::ChartBearish,
        ],
    ),
    (
        "theme-map-heading-misc",
        "Misc",
        &[
            ThemeToken::DescriptionListLabel,
            ThemeToken::DescriptionListLabelForeground,
            ThemeToken::DragBorder,
            ThemeToken::DropTarget,
            ThemeToken::Popover,
            ThemeToken::PopoverForeground,
            ThemeToken::ProgressBar,
            ThemeToken::Skeleton,
            ThemeToken::SliderBar,
            ThemeToken::SliderThumb,
            ThemeToken::Switch,
            ThemeToken::SwitchThumb,
            ThemeToken::StatusBar,
            ThemeToken::StatusBarBorder,
            ThemeToken::TitleBar,
            ThemeToken::TitleBarBorder,
            ThemeToken::WindowBorder,
        ],
    ),
    (
        "theme-map-heading-base",
        "Base",
        &[
            ThemeToken::Magenta,
            ThemeToken::MagentaLight,
            ThemeToken::Cyan,
            ThemeToken::CyanLight,
        ],
    ),
];

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Theme Map
    // -----------------------------------------------------------------------
    pub(crate) fn render_theme_map_page(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let ui = &self.info_ui;
        // Built once, so every swatch sits in the showcase's one frame
        // without the theme being reached for a hundred and forty times.
        let frame = StyleRefinement::default().demo_frame(cx);
        v_flex()
            .gap_4()
            .p_4()
            .flex_1()
            .child(demo::heading(
                ui,
                cx,
                "theme-map-heading-all",
                "All ThemeColor Fields",
            ))
            .children(THEME_MAP_GROUPS.iter().flat_map(|(id, title, tokens)| {
                [
                    demo::heading(ui, cx, id, *title).into_any_element(),
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_x(SWATCH_GAP_X)
                        .gap_y(SWATCH_GAP_Y)
                        .children(
                            tokens
                                .iter()
                                .map(|token| demo::swatch(ui, cx, &frame, *token)),
                        )
                        .into_any_element(),
                ]
            }))
    }
}
