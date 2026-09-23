//! What the Charts page's widgets report about themselves (spec §3.4).
//!
//! Every chart paints the same colours in light and in dark: no chart's
//! paint branches on the mode, so each claim below reads its token in both.

use gpui_component::theme::Theme;

use super::{WidgetInfo, claim, px_text};
use crate::demo::{
    PIE_INNER_RADIUS, PIE_OUTER_RADIUS, PIE_PAD_ANGLE, PIE_SLICES, SAMPLE_MONTHS,
    SAMPLE_MONTHS_AREA, SAMPLE_OHLC,
};

/// `months` as the text of a "data" note: each month with its value.
fn months(months: &[(&str, f64)]) -> String {
    let points: Vec<String> = months
        .iter()
        .map(|(month, value)| format!("{month} {value}"))
        .collect();
    format!("{}, one point each", points.join(", "))
}

/// What every chart with axes draws the same way: the plot's own literals.
fn plot_axes(info: WidgetInfo) -> WidgetInfo {
    info.not_themeable(
        "axis text",
        "10px, a literal: the platform's font size does not reach a chart's labels (plot/label.rs, TEXT_SIZE)",
    )
    .not_themeable(
        "line widths",
        "1px, literals: the axis is a 1px stroke (plot/axis.rs, draw_axis) and each grid line a 1px quad (plot/grid.rs, line_bounds)",
    )
}

/// What the connector gives a chart's series colours, which the model does
/// not state.
fn series_colours(info: WidgetInfo) -> WidgetInfo {
    info.not_themeable(
        "series colours",
        "the model states no chart colours: the connector sets chart_1 to the platform's accent and turns that hue 0.2, 0.4, 0.6 and 0.8 of the way round the wheel for chart_2 to chart_5, at no less than 0.3 saturation (native-theme-gpui/colors.rs, assign_charts). Our gap",
    )
}

/// A chart the showcase builds without an id, which upstream draws as a
/// plain plot.
fn no_hover(info: WidgetInfo) -> WidgetInfo {
    info.instance(
        "hover",
        "none: the chart is built without an id, so upstream draws it as a plain plot, with no tooltip and no hover emphasis (plot/mod.rs, Plot::id)",
    )
}

/// The Charts page's BarChart, its bars in `chart_1`.
pub fn bar_chart(t: &Theme) -> WidgetInfo {
    let info = WidgetInfo::new("BarChart")
        .color(claim("bars", "chart_1", t.chart_1, "showcase"))
        .color(claim(
            "axis labels",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/chart/bar_chart.rs:518",
        ))
        .color(claim(
            "axis line",
            "border",
            t.border,
            "gpui-component/chart/bar_chart.rs:490",
        ))
        .color(claim(
            "grid",
            "border",
            t.border,
            "gpui-component/chart/bar_chart.rs:563",
        ))
        .not_themeable(
            "colour source",
            "this demo picks the bars' colour, chart_1, through BarChart::fill; without it every bar is chart_2 (chart/bar_chart.rs, BarChart::paint), so chart_1 reaches a pixel only because an application asks for it. Upstream paints the colour at full strength while nothing is hovered",
        );
    let info = series_colours(plot_axes(info))
        .not_themeable(
            "grid",
            "dashed, 4px on and 2px off: a literal (chart/bar_chart.rs, BarChart::paint)",
        )
        .not_themeable(
            "bar corners",
            "square: Corners::all(px(0.)) unless BarChart::corner_radii sets others, so the theme radius does not reach them (chart/bar_chart.rs, BarChart::new)",
        )
        .instance("data", months(&SAMPLE_MONTHS))
        .instance(
            "bar width",
            "the band's, the bands padded 0.4 between and 0.2 at the ends, so the bars widen with the chart (chart/bar_chart.rs, BarChart::band_scale)",
        )
        .instance(
            "labels",
            "every month under its bar: tick_margin is 1, the default, and BarChart::tick_margin would label every n-th (chart/bar_chart.rs, BarChart::tick_margin)",
        );
    no_hover(info)
}

/// The Charts page's LineChart, its line and dots in `chart_2`.
pub fn line_chart(t: &Theme) -> WidgetInfo {
    let info = WidgetInfo::new("LineChart")
        .color(claim("line", "chart_2", t.chart_2, "showcase"))
        .color(claim("dots", "chart_2", t.chart_2, "showcase"))
        .color(claim(
            "axis labels",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/chart/line_chart.rs:196",
        ))
        .color(claim(
            "axis line",
            "border",
            t.border,
            "gpui-component/chart/line_chart.rs:189",
        ))
        .color(claim(
            "grid",
            "border",
            t.border,
            "gpui-component/chart/line_chart.rs:206",
        ))
        .not_themeable(
            "colour source",
            "this demo picks chart_2 through LineChart::stroke, the colour the chart takes without one (chart/line_chart.rs, LineChart::paint); the dots take the line's colour, filled and edged",
        );
    let info = series_colours(plot_axes(info))
        .not_themeable(
            "grid",
            "dashed, 4px on and 2px off: a literal (chart/line_chart.rs, LineChart::paint)",
        )
        .not_themeable(
            "line width",
            "2px, and each dot 8px across: literals (chart/line_chart.rs, LineChart::paint)",
        )
        .instance("data", months(&SAMPLE_MONTHS))
        .instance(
            "curve",
            "natural, the default: a Catmull-Rom curve through the points (plot/shape/line.rs, build_path); LineChart::linear and step_after pick the others (chart/line_chart.rs, LineChart::natural)",
        )
        .instance(
            "dots",
            "on, through LineChart::dot: one on each point (chart/line_chart.rs, LineChart::dot)",
        );
    no_hover(info)
}

/// The Charts page's AreaChart: one series, its line in `chart_3` over a
/// fill of `chart_3` at 30%.
pub fn area_chart(t: &Theme) -> WidgetInfo {
    let info = WidgetInfo::new("AreaChart")
        .color(claim("line", "chart_3", t.chart_3, "showcase"))
        // The showcase asks for the series colour at 30%, and upstream
        // paints the fill as given (plot/shape/area.rs:223).
        .color(claim(
            "fill, at 30%",
            "chart_3",
            t.chart_3.opacity(0.3),
            "showcase",
        ))
        .color(claim(
            "axis labels",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/chart/area_chart.rs:201",
        ))
        .color(claim(
            "axis line",
            "border",
            t.border,
            "gpui-component/chart/area_chart.rs:194",
        ))
        .color(claim(
            "grid",
            "border",
            t.border,
            "gpui-component/chart/area_chart.rs:211",
        ))
        .not_themeable(
            "colour source",
            "this demo picks chart_3, and chart_3 at 30% for the fill, through AreaChart::stroke and fill; without them a series is chart_2 over chart_2 at 40% (chart/area_chart.rs, AreaChart::paint), so chart_3 reaches a pixel only because an application asks for it",
        );
    let info = series_colours(plot_axes(info))
        .not_themeable(
            "grid",
            "dashed, 4px on and 2px off: a literal (chart/area_chart.rs, AreaChart::paint)",
        )
        .not_themeable(
            "line width",
            "1px, a literal (plot/shape/area.rs, Area::paint)",
        )
        .instance("data", months(&SAMPLE_MONTHS_AREA))
        .instance(
            "series",
            "one; each AreaChart::y adds another, and the n-th stroke and fill are the n-th series' (chart/area_chart.rs, AreaChart::y)",
        );
    no_hover(info)
}

/// The Charts page's PieChart, a donut whose slices are `chart_1`,
/// `chart_2` and `chart_3` in `PIE_SLICES`' order.
pub fn pie_chart(t: &Theme) -> WidgetInfo {
    let slices: Vec<String> = PIE_SLICES
        .iter()
        .map(|(label, value)| format!("{label} {value}"))
        .collect();
    let info = WidgetInfo::new("PieChart")
        .variant("donut")
        .color(claim("first slice", "chart_1", t.chart_1, "showcase"))
        .color(claim("second slice", "chart_2", t.chart_2, "showcase"))
        .color(claim("third slice", "chart_3", t.chart_3, "showcase"))
        .not_themeable(
            "colour source",
            "this demo picks each slice's colour through PieChart::color; without it every slice is chart_2 (chart/pie_chart.rs, PieChart::slice_color), so chart_1 and chart_3 reach a pixel only because an application asks for them. Upstream paints each at full strength while nothing is hovered (chart/pie_chart.rs, PieChart::slice_emphasis)",
        );
    let info = series_colours(info)
        .instance(
            "slices",
            format!(
                "{}, in that order round the ring, each taking its share of the sum (plot/shape/pie.rs, Pie::arcs), and painted with the colour of its place above",
                slices.join(", ")
            ),
        )
        .instance(
            "inner radius",
            format!(
                "{}px, so a donut; 0 would fill the centre (chart/pie_chart.rs, PieChart::inner_radius)",
                px_text(PIE_INNER_RADIUS)
            ),
        )
        .instance(
            "outer radius",
            format!(
                "{}px; without one upstream takes 40% of the chart's height (chart/pie_chart.rs, PieChart::resolve_outer_radius)",
                px_text(PIE_OUTER_RADIUS)
            ),
        )
        .instance(
            "pad angle",
            format!(
                "{PIE_PAD_ANGLE} radians between slices (chart/pie_chart.rs, PieChart::pad_angle)"
            ),
        )
        .instance(
            "labels",
            "none: PieChart::label would draw one outside the ring for each slice (chart/pie_chart.rs, PieChart::label)",
        );
    no_hover(info)
}

/// The Charts page's CandlestickChart, each candle `chart_bullish` or
/// `chart_bearish` by its own open and close.
pub fn candlestick_chart(t: &Theme) -> WidgetInfo {
    // Upstream's rule: a candle is bullish only where it closes above its
    // open (chart/candlestick_chart.rs:273).
    let days = |bullish: bool| -> String {
        let days: Vec<&str> = SAMPLE_OHLC
            .iter()
            .filter(|(_, open, _, _, close)| (close > open) == bullish)
            .map(|(day, ..)| *day)
            .collect();
        days.join(", ")
    };
    let info = WidgetInfo::new("CandlestickChart")
        .color(claim(
            "bullish candles",
            "chart_bullish",
            t.chart_bullish,
            "gpui-component/chart/candlestick_chart.rs:158",
        ))
        .color(claim(
            "bearish candles",
            "chart_bearish",
            t.chart_bearish,
            "gpui-component/chart/candlestick_chart.rs:159",
        ))
        .color(claim(
            "axis labels",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/chart/candlestick_chart.rs:224",
        ))
        .color(claim(
            "axis line",
            "border",
            t.border,
            "gpui-component/chart/candlestick_chart.rs:216",
        ))
        .color(claim(
            "grid",
            "border",
            t.border,
            "gpui-component/chart/candlestick_chart.rs:234",
        ))
        .not_themeable(
            "candle colours",
            "chart_bullish up and chart_bearish down, which the connector writes from its success and danger colours -- green and red only where the platform's are. CandlestickChart::bullish and ::bearish replace them (chart/candlestick_chart.rs, candle_colors)",
        );
    let info = plot_axes(info)
        .not_themeable(
            "grid",
            "dashed, 4px on and 2px off: a literal (chart/candlestick_chart.rs, CandlestickChart::paint)",
        )
        .not_themeable(
            "wick",
            "1px, a literal (chart/candlestick_chart.rs, CandlestickChart::paint)",
        )
        .instance(
            "candles",
            format!(
                "{} close above their open and are bullish; {} do not and are bearish. Upstream calls a candle bullish only where it closes above its open, so one that closes at its open is bearish too, and paints its wick and body in the one colour (chart/candlestick_chart.rs, CandlestickChart::paint)",
                days(true),
                days(false)
            ),
        )
        .instance(
            "body width",
            "0.8 of the band, the default (chart/candlestick_chart.rs, CandlestickChart::body_width_ratio)",
        );
    no_hover(info)
}
