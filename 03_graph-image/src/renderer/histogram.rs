use plotters::prelude::*;
use std::collections::BTreeMap;

use crate::parser::HistogramData;
use crate::renderer::{Renderer, ensure_output_dir};

const COLORS: &[RGBColor] = &[
    RGBColor(31, 119, 180),
    RGBColor(255, 127, 14),
    RGBColor(44, 160, 44),
    RGBColor(214, 39, 40),
    RGBColor(148, 103, 189),
];

pub struct HistogramRenderer {
    pub data: HistogramData,
}

fn bin_data(values: &[f64], bin_size: f64) -> BTreeMap<i64, u64> {
    let mut bins: BTreeMap<i64, u64> = BTreeMap::new();
    for &v in values {
        let idx = (v / bin_size).floor() as i64;
        *bins.entry(idx).or_insert(0) += 1;
    }
    bins
}

impl Renderer for HistogramRenderer {
    fn render(&self, output_path: &str, size: u32) -> anyhow::Result<()> {
        ensure_output_dir(output_path)?;

        let bin_size = self.data.size;
        let x_scale = self.data.x_scale;

        // 全seriesのビン集計
        let all_bins: Vec<BTreeMap<i64, u64>> = self
            .data
            .series
            .iter()
            .map(|s| bin_data(&s.data, bin_size))
            .collect();

        // x/y の範囲を求める
        let x_min_idx = all_bins
            .iter()
            .filter_map(|b| b.keys().next())
            .min()
            .copied()
            .unwrap_or(0);
        let x_max_idx = all_bins
            .iter()
            .filter_map(|b| b.keys().last())
            .max()
            .copied()
            .unwrap_or(0);
        let y_max = all_bins
            .iter()
            .flat_map(|b| b.values())
            .copied()
            .max()
            .unwrap_or(1) as f64
            * 1.1
            + f64::EPSILON;

        let x_min = x_min_idx as f64 * bin_size;
        let x_max = (x_max_idx + 1) as f64 * bin_size;

        let title = self.data.title.as_deref().unwrap_or("");
        let x_label = self.data.x_label.as_deref().unwrap_or("").to_string();
        let y_label = self.data.y_label.as_deref().unwrap_or("").to_string();

        let root = BitMapBackend::new(output_path, (size, size)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 20))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(x_min..x_max, 0.0..y_max)?;

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc(y_label)
            .x_label_formatter(&|v| {
                if (v / x_scale).fract().abs() < 1e-9 {
                    format!("{}", *v as i64)
                } else {
                    String::new()
                }
            })
            .draw()?;

        for (si, bins) in all_bins.iter().enumerate() {
            let color = COLORS[si % COLORS.len()];
            let label = self.data.series[si]
                .label
                .as_deref()
                .unwrap_or("")
                .to_string();

            chart
                .draw_series(bins.iter().map(|(&idx, &count)| {
                    let x0 = idx as f64 * bin_size;
                    let x1 = x0 + bin_size;
                    Rectangle::new([(x0, 0.0), (x1, count as f64)], color.filled())
                }))?
                .label(label)
                .legend(move |(x, y)| {
                    Rectangle::new([(x, y - 5), (x + 20, y + 5)], color.filled())
                });
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Series;

    fn make_data() -> HistogramData {
        HistogramData {
            title: Some("Test Histogram".to_string()),
            x_label: Some("value".to_string()),
            y_label: Some("count".to_string()),
            size: 1.0,
            x_scale: 5.0,
            series: vec![Series {
                label: Some("s1".to_string()),
                data: vec![1.0, 1.0, 2.0, 3.0, 3.0, 3.0, 5.0],
            }],
        }
    }

    #[test]
    fn test_bin_data() {
        let bins = bin_data(&[1.0, 1.0, 2.0, 3.0], 1.0);
        assert_eq!(bins[&1], 2);
        assert_eq!(bins[&2], 1);
        assert_eq!(bins[&3], 1);
    }

    #[test]
    fn test_render_histogram() {
        let renderer = HistogramRenderer { data: make_data() };
        let path = "output/test_histogram.png";
        renderer.render(path, 256).unwrap();
        assert!(std::path::Path::new(path).exists());
    }
}
