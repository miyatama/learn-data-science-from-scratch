use crate::statistics::{mean, std_deviation};

pub fn covariance(x: &[f64], y: &[f64]) -> f64 {
    let mx = mean(x);
    let my = mean(y);
    let n = x.len() as f64;
    x.iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| (xi - mx) * (yi - my))
        .sum::<f64>()
        / (n - 1.0)
}

pub fn correlation(x: &[f64], y: &[f64]) -> f64 {
    let sx = std_deviation(x);
    let sy = std_deviation(y);
    covariance(x, y) / (sx * sy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covariance_positive() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let cov = covariance(&x, &y);
        assert!(cov > 0.0);
    }

    #[test]
    fn test_correlation_perfect_positive() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let r = correlation(&x, &y);
        let rounded = (r * 1000.0).round() / 1000.0;
        assert_eq!(rounded, 1.0);
    }

    #[test]
    fn test_correlation_negative() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![10.0, 8.0, 6.0, 4.0, 2.0];
        let r = correlation(&x, &y);
        assert!(r < 0.0);
    }
}
