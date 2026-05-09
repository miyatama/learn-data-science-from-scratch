use graph_image::parser;
use graph_image::renderer::{
    Renderer, bar::BarRenderer, line::LineRenderer, scatter::ScatterRenderer,
};

#[test]
fn test_line_from_example_json() {
    let json = std::fs::read_to_string("examples/line.json").unwrap();
    let data = parser::parse_line(&json).unwrap();
    let renderer = LineRenderer { data };
    renderer.render("output/integration_line.png", 256).unwrap();
    assert!(std::path::Path::new("output/integration_line.png").exists());
}

#[test]
fn test_bar_from_example_json() {
    let json = std::fs::read_to_string("examples/bar.json").unwrap();
    let data = parser::parse_bar(&json).unwrap();
    let renderer = BarRenderer { data };
    renderer.render("output/integration_bar.png", 256).unwrap();
    assert!(std::path::Path::new("output/integration_bar.png").exists());
}

#[test]
fn test_scatter_from_example_json() {
    let json = std::fs::read_to_string("examples/scatter.json").unwrap();
    let data = parser::parse_scatter(&json).unwrap();
    let renderer = ScatterRenderer { data };
    renderer
        .render("output/integration_scatter.png", 256)
        .unwrap();
    assert!(std::path::Path::new("output/integration_scatter.png").exists());
}
