//! The Charts page.

use gpui::{Context, IntoElement, ParentElement, Styled, prelude::*};
use gpui_component::v_flex;

use crate::app::Showcase;
use crate::demo;
use crate::{
    CHARTS_AREA_CHART, CHARTS_BAR_CHART, CHARTS_CANDLESTICK_CHART, CHARTS_LINE_CHART,
    CHARTS_PIE_CHART,
};

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Charts
    // -----------------------------------------------------------------------
    pub(crate) fn render_charts_page(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let ui = &self.info_ui;
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            .child(demo::heading(
                ui,
                cx,
                "charts-heading-bar-chart",
                "BarChart",
            ))
            .child(demo::bar_chart(ui, cx, CHARTS_BAR_CHART))
            .child(demo::heading(
                ui,
                cx,
                "charts-heading-line-chart",
                "LineChart",
            ))
            .child(demo::line_chart(ui, cx, CHARTS_LINE_CHART))
            .child(demo::heading(
                ui,
                cx,
                "charts-heading-area-chart",
                "AreaChart",
            ))
            .child(demo::area_chart(ui, cx, CHARTS_AREA_CHART))
            .child(demo::heading(
                ui,
                cx,
                "charts-heading-pie-chart",
                "PieChart (donut)",
            ))
            .child(demo::pie_chart(ui, cx, CHARTS_PIE_CHART))
            .child(demo::heading(
                ui,
                cx,
                "charts-heading-candlestick-chart",
                "CandlestickChart",
            ))
            .child(demo::candlestick_chart(ui, cx, CHARTS_CANDLESTICK_CHART))
    }
}
