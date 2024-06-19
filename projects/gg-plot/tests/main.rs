use ggplot::BarChart;
use std::fs::File;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let data = vec![100.0, -200.0, 300.0];
    let config = BarChart::default();
    let doc = config.plot(data);
    let mut out = File::create("bar-chart.svg").unwrap();
    svg::write(&mut out, &doc).unwrap();
}
