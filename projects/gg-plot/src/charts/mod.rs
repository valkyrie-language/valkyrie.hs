use self::bar_chart::BarChartInner;
use ggplot_core::Style;
use std::{ops::Range, slice::Iter, sync::Arc};
use svg::Document;

mod bar_chart;

/// [ListLinePlot](https://reference.wolfram.com/language/ref/ListLinePlot.html)
pub struct BarChart {
    pub kind: BarChartType,
    pub left_space: f32,
    pub right_space: f32,
    pub bar_width: f32,
    pub bar_space: f32,
    pub group_space: f32,
    pub x_range: Style<Range<f64>>,
    pub y_range: Style<Range<f64>>,
}

#[derive(Default)]
pub enum BarChartType {
    #[default]
    Grouped,
    Stacked,
    Stepped,
    Percentile,
    Overlapped,
}

impl Default for BarChart {
    fn default() -> Self {
        Self {
            kind: BarChartType::Grouped,
            left_space: 0.0,
            right_space: 0.0,
            bar_width: 90.0,
            bar_space: 10.0,
            group_space: 20.0,
            x_range: Default::default(),
            y_range: Default::default(),
        }
    }
}

impl BarChart {
    pub fn plot(&self, data: &BarChartInner) -> Document {
        data.plot(self)
    }
}
