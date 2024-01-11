use std::fs::File;
use svg::Document;
use crate::charts::BarChart;

pub(crate) enum BarChartInner {
    List(Vec<f32>),
}

pub struct Rectangle {
    /// The x-coordinate of the upper left point
    x: f64,
    /// The y-coordinate of the upper left point
    y: f64,
    /// The width of the rectangle
    width: f64,
    /// The height of the rectangle
    height: f64,
    color: Color,
}

pub enum Color {
    RGBA(RGBA)
}

#[repr(C)]
pub struct RGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Rectangle {
    pub fn to_svg(&self) -> svg::node::element::Rectangle {
        let mut out = svg::node::element::Rectangle::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height);

        out
    }
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
                        shape: Rectangle {
                            x: delta_x,
                            y: 0.0,
                            width: config.bar_width,
                            height: 0.0,
                            color: crate::charts::data::Color::RGBA(RGBA {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.0,
                            }),
                        },
                    })
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
    let mut out = File::create("gg-plot.svg").unwrap();
    svg::write(&mut out, &doc).unwrap();
}