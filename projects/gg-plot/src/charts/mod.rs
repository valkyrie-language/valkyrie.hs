use std::ops::Range;
use std::slice::Iter;
use std::sync::Arc;
use svg::Document;
use self::data::BarChartInner;

mod data;


/// [ListLinePlot](https://reference.wolfram.com/language/ref/ListLinePlot.html)
pub struct BarChart {
    pub kind: BarChartType,
    pub left_space: f64,
    pub right_space: f64,
    pub bar_width: f64,
    pub bar_space: f64,
    pub group_space: f64,
    pub x_range: Style<Range<f64>>,
    pub y_range: Style<Range<f64>>,
}

#[derive(Default)]
pub enum Style<T> {
    ///
    Initial,
    /// Adopt the same settings as the parent node
    #[default]
    Inherit,
    ///
    Custom(Box<T>),
    ///
    Shared(Arc<T>),
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