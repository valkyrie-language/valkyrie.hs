use super::*;
use crate::theme::{RotatePalette, WolframTheme};
use ggplot_types::Palette;

/// [BarChart](https://reference.wolfram.com/language/ref/BarChart.html)
#[derive(Clone, Debug)]
pub struct BarChart {
    pub kind: Style<BarChartType>,
    pub left_space: f32,
    pub right_space: f32,
    pub bar_width: f32,
    pub bar_space: f32,
    pub group_space: f32,
    pub x_range: Style<Range<f64>>,
    pub y_range: Style<Range<f64>>,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BarChartType {
    #[default]
    Grouped,
    Stacked,
    Stepped,
    Percentile,
    Overlapped,
}

pub trait BarChartData {
    fn elements(&self, config: &BarChart) -> impl Iterator<Item = BarChartElement>;
    fn plot(&self, config: &BarChart) -> Document;
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarChartElement {
    shape: Rectangle,
}

impl Default for BarChart {
    fn default() -> Self {
        Self {
            kind: Default::default(),
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
    pub fn plot(&self, data: impl BarChartData) -> Document {
        data.plot(self)
    }
}

// <svg viewBox="0 -400 600 600" xmlns="http://www.w3.org/2000/svg">
// <rect fill="color(srgb-linear 0.0955813 0.209225 0.79187 / 1)" height="100" width="90" x="0" y="-100"/>
// <rect fill="color(srgb-linear 0.935186 0.332231 0.00396887 / 1)" height="200" width="90" x="100" y="0"/>
// <rect fill="color(srgb-linear 0.812627 0.0804155 0.0444973 / 1)" height="300" width="90" x="200" y="-300"/>
// </svg>
impl BarChartData for Vec<f32> {
    fn elements(&self, config: &BarChart) -> impl Iterator<Item = BarChartElement> {
        let mut elements = Vec::with_capacity(self.len());
        let mut delta_x = 0.0;

        let mut theme = RotatePalette::business();
        for raw in self.iter() {
            elements.push(BarChartElement {
                shape: {
                    let y = if *raw <= 0.0 { 0.0 } else { -*raw };
                    Rectangle {
                        x: delta_x,
                        width: config.bar_width,
                        y,
                        height: raw.abs(),
                        color: Color::Pure(theme.next_color()),
                    }
                },
            });
            delta_x += config.bar_width;
            delta_x += config.bar_space;
        }
        elements.into_iter()
    }

    fn plot(&self, config: &BarChart) -> Document {
        let mut doc = Document::new().set("viewBox", (0, -400, 600, 600));
        for element in self.elements(config) {
            doc = doc.add(element.shape.to_svg());
        }
        doc
    }
}
