use plotters::prelude::*;

use crate::parser::BarData;
use crate::renderer::{Renderer, ensure_output_dir};

const COLORS: &[RGBColor] = &[
    RGBColor(31, 119, 180),
    RGBColor(255, 127, 14),
    RGBColor(44, 160, 44),
    RGBColor(214, 39, 40),
    RGBColor(148, 103, 189),
];

pub struct BarRenderer {
    pub data: BarData,
}

impl Renderer for BarRenderer {
    fn render(&self, output_path: &str, size: u32) -> anyhow::Result<()> {
        ensure_output_dir(output_path)?;

        let root = BitMapBackend::new(output_path, (size, size)).into_drawing_area();
        root.fill(&WHITE)?;

        let n_cats = self.data.categories.len();
        let n_series = self.data.series.len();
        let y_max = self
            .data
            .series
            .iter()
            .flat_map(|s| s.data.iter())
            .cloned()
            .fold(0.0_f64, f64::max)
            * 1.1
            + f64::EPSILON;

        let title = self.data.title.as_deref().unwrap_or("");
        let x_label = self.data.x_label.as_deref().unwrap_or("").to_string();
        let y_label = self.data.y_label.as_deref().unwrap_or("").to_string();

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 20))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0i32..(n_cats as i32), 0.0..y_max)?;

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc(y_label)
            .x_label_formatter(&|v| {
                let idx = *v as usize;
                if idx < self.data.categories.len() {
                    self.data.categories[idx].clone()
                } else {
                    String::new()
                }
            })
            .draw()?;

        let bar_width = if n_series > 0 {
            1.0 / (n_series as f64 + 0.5)
        } else {
            1.0
        };

        for (si, series) in self.data.series.iter().enumerate() {
            let color = COLORS[si % COLORS.len()];
            let offset = si as f64 * bar_width - (n_series as f64 * bar_width) / 2.0;
            let label = series.label.as_deref().unwrap_or("").to_string();

            chart
                .draw_series(series.data.iter().enumerate().map(|(ci, &val)| {
                    let x0 = ci as f64 + offset + 0.1;
                    let x1 = x0 + bar_width * 0.8;
                    Rectangle::new([(x0 as i32, 0.0), (x1 as i32, val)], color.filled())
                }))?
                .label(label)
                .legend(move |(x, y)| {
                    Rectangle::new([(x, y - 5), (x + 20, y + 5)], color.filled())
                });
        }

        if n_series > 1 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Series;

    fn make_data() -> BarData {
        BarData {
            title: Some("Test".to_string()),
            x_label: Some("Cat".to_string()),
            y_label: Some("Val".to_string()),
            categories: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            series: vec![Series {
                label: Some("s1".to_string()),
                data: vec![10.0, 25.0, 15.0],
            }],
        }
    }

    #[test]
    fn test_render_bar() {
        let renderer = BarRenderer { data: make_data() };
        let path = "output/test_bar.png";
        renderer.render(path, 256).unwrap();
        assert!(std::path::Path::new(path).exists());
    }
}
