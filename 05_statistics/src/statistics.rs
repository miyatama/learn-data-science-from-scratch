use std::collections::HashMap;

pub fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

pub fn median(data: &[f64]) -> f64 {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = sorted.len();
    if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    }
}

pub fn quantile(data: &[f64], p: f64) -> f64 {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let index = (p * sorted.len() as f64).floor() as usize;
    sorted[index.min(sorted.len() - 1)]
}

pub fn mode(data: &[f64]) -> f64 {
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for &v in data {
        *counts.entry(v as i64).or_insert(0) += 1;
    }
    let max_count = counts.values().copied().max().unwrap_or(0);
    let mode_key = counts
        .into_iter()
        .filter(|&(_, c)| c == max_count)
        .map(|(k, _)| k)
        .min()
        .unwrap_or(0);
    mode_key as f64
}

pub fn min(data: &[f64]) -> f64 {
    data.iter().cloned().fold(f64::INFINITY, f64::min)
}

pub fn max(data: &[f64]) -> f64 {
    data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}

pub fn data_range(data: &[f64]) -> f64 {
    max(data) - min(data)
}

pub fn variance(data: &[f64]) -> f64 {
    let m = mean(data);
    let n = data.len() as f64;
    data.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / (n - 1.0)
}

pub fn std_deviation(data: &[f64]) -> f64 {
    variance(data).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_basic() {
        assert_eq!(mean(&[1.0, 2.0, 3.0]), 2.0);
    }

    #[test]
    fn test_median_odd() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
    }

    #[test]
    fn test_median_even() {
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
    }

    #[test]
    fn test_quantile_25() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        assert_eq!(quantile(&data, 0.25), 3.0);
    }

    #[test]
    fn test_quantile_75() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        assert_eq!(quantile(&data, 0.75), 8.0);
    }

    #[test]
    fn test_mode_basic() {
        assert_eq!(mode(&[1.0, 2.0, 2.0, 3.0]), 2.0);
    }

    #[test]
    fn test_min() {
        assert_eq!(min(&[3.0, 1.0, 2.0]), 1.0);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(&[3.0, 1.0, 2.0]), 3.0);
    }

    #[test]
    fn test_data_range() {
        assert_eq!(data_range(&[1.0, 5.0, 3.0]), 4.0);
    }

    #[test]
    fn test_variance_basic() {
        let v = variance(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]);
        let rounded = (v * 1000.0).round() / 1000.0;
        assert_eq!(rounded, 4.571);
    }

    #[test]
    fn test_std_deviation_basic() {
        let s = std_deviation(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]);
        assert!(s > 2.0 && s < 2.2);
    }
}
