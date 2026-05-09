use plotters::prelude::*;

use crate::parser::ScatterData;
use crate::renderer::{Renderer, ensure_output_dir};

const COLORS: &[RGBColor] = &[
    RGBColor(31, 119, 180),
    RGBColor(255, 127, 14),
    RGBColor(44, 160, 44),
    RGBColor(214, 39, 40),
    RGBColor(148, 103, 189),
];

pub struct ScatterRenderer {
    pub data: ScatterData,
}

impl Renderer for ScatterRenderer {
    fn render(&self, output_path: &str, size: u32) -> anyhow::Result<()> {
        ensure_output_dir(output_path)?;

        let root = BitMapBackend::new(output_path, (size, size)).into_drawing_area();
        root.fill(&WHITE)?;

        let (x_min, x_max, y_min, y_max) = data_range(&self.data);
        let x_margin = (x_max - x_min) * 0.05 + f64::EPSILON;
        let y_margin = (y_max - y_min) * 0.05 + f64::EPSILON;

        let title = self.data.title.as_deref().unwrap_or("");
        let x_label = self.data.x_label.as_deref().unwrap_or("").to_string();
        let y_label = self.data.y_label.as_deref().unwrap_or("").to_string();

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 20))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(
                (x_min - x_margin)..(x_max + x_margin),
                (y_min - y_margin)..(y_max + y_margin),
            )?;

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc(y_label)
            .draw()?;

        for (i, series) in self.data.series.iter().enumerate() {
            let color = COLORS[i % COLORS.len()];
            let points: Vec<(f64, f64)> = series.data.iter().map(|p| (p.x, p.y)).collect();
            let label = series.label.as_deref().unwrap_or("").to_string();
            chart
                .draw_series(
                    points
                        .iter()
                        .map(|&(x, y)| Circle::new((x, y), 4, color.filled())),
                )?
                .label(label)
                .legend(move |(x, y)| Circle::new((x + 10, y), 4, color.filled()));
        }

        if self.data.series.len() > 1 {
            chart
                .configure_series_labels()
                .background_style(WHITE.mix(0.8))
                .border_style(BLACK)
                .draw()?;
        }

        root.present()?;
        Ok(())
    }
}

fn data_range(data: &ScatterData) -> (f64, f64, f64, f64) {
    let mut x_min = f64::MAX;
    let mut x_max = f64::MIN;
    let mut y_min = f64::MAX;
    let mut y_max = f64::MIN;
    for series in &data.series {
        for p in &series.data {
            x_min = x_min.min(p.x);
            x_max = x_max.max(p.x);
            y_min = y_min.min(p.y);
            y_max = y_max.max(p.y);
        }
    }
    (x_min, x_max, y_min, y_max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{DataPoint, Series};

    fn make_data() -> ScatterData {
        ScatterData {
            title: Some("Test".to_string()),
            x_label: Some("X".to_string()),
            y_label: Some("Y".to_string()),
            series: vec![Series {
                label: Some("s1".to_string()),
                data: vec![DataPoint { x: 1.0, y: 2.0 }, DataPoint { x: 3.0, y: 5.0 }],
            }],
        }
    }

    #[test]
    fn test_render_scatter() {
        let renderer = ScatterRenderer { data: make_data() };
        let path = "output/test_scatter.png";
        renderer.render(path, 256).unwrap();
        assert!(std::path::Path::new(path).exists());
    }
}
