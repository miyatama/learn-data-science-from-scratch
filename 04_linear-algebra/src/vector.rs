fn check_same_length(v1: &[f64], v2: &[f64]) -> Result<(), String> {
    if v1.len() != v2.len() {
        Err("vectors must have the same length".to_string())
    } else {
        Ok(())
    }
}

pub fn vector_add(v1: &[f64], v2: &[f64]) -> Result<Vec<f64>, String> {
    check_same_length(v1, v2)?;
    Ok(v1.iter().zip(v2.iter()).map(|(a, b)| a + b).collect())
}

pub fn vector_subtract(v1: &[f64], v2: &[f64]) -> Result<Vec<f64>, String> {
    check_same_length(v1, v2)?;
    Ok(v1.iter().zip(v2.iter()).map(|(a, b)| a - b).collect())
}

pub fn scalar_multiply(scalar: f64, v: &[f64]) -> Vec<f64> {
    v.iter().map(|a| scalar * a).collect()
}

pub fn dot_product(v1: &[f64], v2: &[f64]) -> Result<f64, String> {
    check_same_length(v1, v2)?;
    Ok(v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum())
}

pub fn sum_of_squares(v: &[f64]) -> f64 {
    v.iter().map(|a| a * a).sum()
}

pub fn magnitude(v: &[f64]) -> f64 {
    sum_of_squares(v).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add_normal() {
        assert_eq!(
            vector_add(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).unwrap(),
            vec![5.0, 7.0, 9.0]
        );
    }

    #[test]
    fn test_vector_add_dimension_mismatch() {
        assert!(vector_add(&[1.0, 2.0], &[1.0, 2.0, 3.0]).is_err());
    }

    #[test]
    fn test_vector_subtract_normal() {
        assert_eq!(
            vector_subtract(&[5.0, 7.0, 9.0], &[4.0, 5.0, 6.0]).unwrap(),
            vec![1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn test_vector_subtract_dimension_mismatch() {
        assert!(vector_subtract(&[1.0, 2.0], &[1.0, 2.0, 3.0]).is_err());
    }

    #[test]
    fn test_scalar_multiply() {
        assert_eq!(scalar_multiply(2.0, &[1.0, 2.0, 3.0]), vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_dot_product_normal() {
        assert_eq!(
            dot_product(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).unwrap(),
            32.0
        );
    }

    #[test]
    fn test_dot_product_dimension_mismatch() {
        assert!(dot_product(&[1.0, 2.0], &[1.0, 2.0, 3.0]).is_err());
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(&[1.0, 2.0, 3.0]), 14.0);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(magnitude(&[3.0, 4.0]), 5.0);
    }
}
