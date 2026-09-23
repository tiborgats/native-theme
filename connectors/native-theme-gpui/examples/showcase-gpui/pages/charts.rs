//! The Charts page.

use gpui::{Context, Hsla, IntoElement, ParentElement, SharedString, Styled, div, prelude::*, px};
use gpui_component::{
    ActiveTheme,
    chart::{AreaChart, BarChart, CandlestickChart, LineChart, PieChart},
    v_flex,
};

use crate::CHARTS_BAR_CHART;
use crate::app::Showcase;
use crate::support::{format_font_info, section};

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Charts
    // -----------------------------------------------------------------------
    pub(crate) fn render_charts_page(
        &self,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();

        // Sample data structs for charts
        #[derive(Clone)]
        struct MonthData {
            month: SharedString,
            value: f64,
        }

        #[derive(Clone)]
        struct OhlcData {
            date: SharedString,
            open: f64,
            high: f64,
            low: f64,
            close: f64,
        }

        #[derive(Clone)]
        struct PieSlice {
            _label: SharedString,
            amount: f32,
            color: Hsla,
        }

        let months: Vec<MonthData> = vec![
            MonthData {
                month: "Jan".into(),
                value: 40.0,
            },
            MonthData {
                month: "Feb".into(),
                value: 65.0,
            },
            MonthData {
                month: "Mar".into(),
                value: 55.0,
            },
            MonthData {
                month: "Apr".into(),
                value: 80.0,
            },
            MonthData {
                month: "May".into(),
                value: 72.0,
            },
            MonthData {
                month: "Jun".into(),
                value: 90.0,
            },
        ];

        let months2: Vec<MonthData> = vec![
            MonthData {
                month: "Jan".into(),
                value: 30.0,
            },
            MonthData {
                month: "Feb".into(),
                value: 50.0,
            },
            MonthData {
                month: "Mar".into(),
                value: 45.0,
            },
            MonthData {
                month: "Apr".into(),
                value: 70.0,
            },
            MonthData {
                month: "May".into(),
                value: 60.0,
            },
            MonthData {
                month: "Jun".into(),
                value: 85.0,
            },
        ];

        let ohlc_data = vec![
            OhlcData {
                date: "Mon".into(),
                open: 100.0,
                high: 115.0,
                low: 95.0,
                close: 110.0,
            },
            OhlcData {
                date: "Tue".into(),
                open: 110.0,
                high: 120.0,
                low: 105.0,
                close: 108.0,
            },
            OhlcData {
                date: "Wed".into(),
                open: 108.0,
                high: 118.0,
                low: 100.0,
                close: 115.0,
            },
            OhlcData {
                date: "Thu".into(),
                open: 115.0,
                high: 125.0,
                low: 110.0,
                close: 112.0,
            },
            OhlcData {
                date: "Fri".into(),
                open: 112.0,
                high: 122.0,
                low: 108.0,
                close: 120.0,
            },
        ];

        let pie_data = vec![
            PieSlice {
                _label: "Desktop".into(),
                amount: 55.0,
                color: t.chart_1,
            },
            PieSlice {
                _label: "Mobile".into(),
                amount: 30.0,
                color: t.chart_2,
            },
            PieSlice {
                _label: "Tablet".into(),
                amount: 15.0,
                color: t.chart_3,
            },
        ];

        let bar_fill = t.chart_1;
        let line_stroke = t.chart_2;
        let area_stroke = t.chart_3;
        let area_fill = t.chart_3.opacity(0.3);

        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Bar chart
            .child(section("BarChart"))
            .child(
                div()
                    .id("tt-bar-chart")
                    .debug_selector(|| CHARTS_BAR_CHART.into())
                    .h(px(220.0))
                    .w_full()
                    .child(
                        BarChart::new(months.clone())
                            .band(|d: &MonthData| d.month.clone())
                            .value(|d: &MonthData| d.value)
                            .fill(move |_: &MonthData, _, _, _| bar_fill),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "BarChart",
                        &[
                            ("fill", "chart_1", t.chart_1, "showcase"),
                            ("axis", "muted_foreground", t.muted_foreground, "gpui-component/chart/bar_chart.rs:518"),
                            ("grid", "border", t.border, "gpui-component/chart/bar_chart.rs:563"),
                        ],
                        &[],
                        &[
                            ("colour source", "this demo picks the series colours; every chart widget defaults to chart_2 on its own (chart/bar_chart.rs, chart/line_chart.rs, chart/area_chart.rs, chart/pie_chart.rs), so chart_1 and chart_3 reach a pixel only because an application asks for them"),
                            ("bar width", "auto-scaled"),
                            ("tick_margin", "configurable"),
                        ],
                    )),
            )
            // Line chart
            .child(section("LineChart"))
            .child(
                div()
                    .id("tt-line-chart")
                    .h(px(220.0))
                    .w_full()
                    .child(
                        LineChart::new(months.clone())
                            .x(|d: &MonthData| d.month.clone())
                            .y(|d: &MonthData| d.value)
                            .stroke(line_stroke)
                            .dot(),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "LineChart",
                        &[
                            ("stroke", "chart_2", t.chart_2, "showcase"),
                            (
                                "axis",
                                "muted_foreground",
                                t.muted_foreground,
                                "gpui-component/chart/line_chart.rs:196",
                            ),
                            ("grid", "border", t.border, "gpui-component/chart/line_chart.rs:206"),
                        ],
                        &[],
                        &[("style", "natural/linear/step_after"), ("dot", "optional")],
                    )),
            )
            // Area chart
            .child(section("AreaChart"))
            .child(
                div()
                    .id("tt-area-chart")
                    .h(px(220.0))
                    .w_full()
                    .child(
                        AreaChart::new(months2)
                            .x(|d: &MonthData| d.month.clone())
                            .y(|d: &MonthData| d.value)
                            .stroke(area_stroke)
                            .fill(area_fill),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "AreaChart",
                        &[
                            (
                                "stroke",
                                "chart_3",
                                t.chart_3,
                                "showcase",
                            ),
                            (
                                "fill",
                                "chart_3",
                                t.chart_3,
                                "showcase",
                            ),
                            (
                                "axis",
                                "muted_foreground",
                                t.muted_foreground,
                                "gpui-component/chart/area_chart.rs:201",
                            ),
                        ],
                        &[],
                        &[("multiple series", "chain .y()/.stroke()/.fill()")],
                    )),
            )
            // Pie chart
            .child(section("PieChart (donut)"))
            .child(
                div()
                    .id("tt-pie-chart")
                    .h(px(250.0))
                    .w(px(250.0))
                    .child(
                        PieChart::new(pie_data)
                            .value(|d: &PieSlice| d.amount)
                            .color(|d: &PieSlice| d.color)
                            .inner_radius(40.0)
                            .outer_radius(100.0)
                            .pad_angle(0.03),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "PieChart",
                        &[
                            ("slice 1", "chart_1", t.chart_1, "showcase"),
                            ("slice 2", "chart_2", t.chart_2, "showcase"),
                            ("slice 3", "chart_3", t.chart_3, "showcase"),
                        ],
                        &[],
                        &[
                            ("inner_radius", "0=filled, >0=donut"),
                            ("pad_angle", "gap between slices"),
                        ],
                    )),
            )
            // Candlestick chart
            .child(section("CandlestickChart"))
            .child(
                div()
                    .id("tt-candlestick-chart")
                    .h(px(220.0))
                    .w_full()
                    .child(
                        CandlestickChart::new(ohlc_data)
                            .x(|d: &OhlcData| d.date.clone())
                            .open(|d: &OhlcData| d.open)
                            .high(|d: &OhlcData| d.high)
                            .low(|d: &OhlcData| d.low)
                            .close(|d: &OhlcData| d.close),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "CandlestickChart",
                        &[
                            (
                                "bullish",
                                "chart_bullish",
                                t.chart_bullish,
                                "gpui-component/chart/candlestick_chart.rs:158",
                            ),
                            (
                                "bearish",
                                "chart_bearish",
                                t.chart_bearish,
                                "gpui-component/chart/candlestick_chart.rs:159",
                            ),
                            (
                                "axis",
                                "muted_foreground",
                                t.muted_foreground,
                                "gpui-component/chart/candlestick_chart.rs:224",
                            ),
                        ],
                        &[],
                        &[
                            ("body_width_ratio", "default 0.8"),
                            ("candle colours", "chart_bullish up and chart_bearish down, which the connector writes from its success and danger colours -- green and red only where the platform's are. CandlestickChart::bullish and ::bearish replace them (chart/candlestick_chart.rs, candle_colors)"),
                        ],
                    )),
            )
    }
}
