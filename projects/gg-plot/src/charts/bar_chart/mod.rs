use crate::charts::BarChart;
use ggplot_core::{Color, Rectangle, Srgb, ToSvg};
use std::fs::File;
use svg::Document;

pub(crate) enum BarChartInner {
    List(Vec<f32>),
}

pub struct BarChartElement {
    shape: Rectangle,
}

impl BarChartInner {
    pub fn plot(&self, config: &BarChart) -> Document {
        let mut doc = Document::new().set("viewBox", (0, 0, 100, 100));
        for element in self.elements(config) {
            doc = doc.add(element.shape.to_svg());
        }
        doc
    }
    pub fn elements(&self, config: &BarChart) -> Vec<BarChartElement> {
        match self {
            Self::List(s) => {
                let mut elements = Vec::with_capacity(s.len());
                let mut delta_x = 0.0;
                for raw in s {
                    elements.push(BarChartElement {
                        shape: Rectangle { x: delta_x, y: 0.0, width: 0.0, height: 0.0, color: Color::BLACK },
                    });
                    delta_x += config.bar_width;
                    delta_x += config.bar_space;
                }
                elements
            }
        }
    }
}

#[test]
fn test() {
    let data = BarChartInner::List(vec![1.0, 2.0, 3.0]);
    let config = BarChart::default();
    let doc = data.plot(&config);
    let mut out = File::create("bar-chart.svg").unwrap();
    svg::write(&mut out, &doc).unwrap();
}
