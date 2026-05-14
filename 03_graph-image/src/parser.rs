use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize)]
pub struct Series<T> {
    pub label: Option<String>,
    pub data: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct LineData {
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub series: Vec<Series<DataPoint>>,
}

#[derive(Debug, Deserialize)]
pub struct BarData {
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub categories: Vec<String>,
    pub series: Vec<Series<f64>>,
}

#[derive(Debug, Deserialize)]
pub struct ScatterData {
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub series: Vec<Series<DataPoint>>,
}

#[derive(Debug, Deserialize)]
pub struct HistogramData {
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub size: f64,
    pub x_scale: f64,
    pub series: Vec<Series<f64>>,
}

pub fn parse_histogram(json: &str) -> anyhow::Result<HistogramData> {
    serde_json::from_str(json).map_err(|e| anyhow::anyhow!("Failed to parse histogram JSON: {}", e))
}

pub fn parse_line(json: &str) -> anyhow::Result<LineData> {
    serde_json::from_str(json).map_err(|e| anyhow::anyhow!("Failed to parse line JSON: {}", e))
}

pub fn parse_bar(json: &str) -> anyhow::Result<BarData> {
    serde_json::from_str(json).map_err(|e| anyhow::anyhow!("Failed to parse bar JSON: {}", e))
}

pub fn parse_scatter(json: &str) -> anyhow::Result<ScatterData> {
    serde_json::from_str(json).map_err(|e| anyhow::anyhow!("Failed to parse scatter JSON: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let json = r#"{"series":[{"label":"s1","data":[{"x":1.0,"y":2.0}]}]}"#;
        let data = parse_line(json).unwrap();
        assert_eq!(data.series.len(), 1);
        assert_eq!(data.series[0].data[0].x, 1.0);
        assert_eq!(data.series[0].data[0].y, 2.0);
    }

    #[test]
    fn test_parse_bar() {
        let json = r#"{"categories":["A","B"],"series":[{"data":[1.0,2.0]}]}"#;
        let data = parse_bar(json).unwrap();
        assert_eq!(data.categories.len(), 2);
        assert_eq!(data.series[0].data[0], 1.0);
    }

    #[test]
    fn test_parse_scatter() {
        let json = r#"{"series":[{"data":[{"x":3.0,"y":4.0}]}]}"#;
        let data = parse_scatter(json).unwrap();
        assert_eq!(data.series[0].data[0].x, 3.0);
    }

    #[test]
    fn test_parse_line_invalid_json() {
        let result = parse_line("not json");
        assert!(result.is_err());
    }
}
