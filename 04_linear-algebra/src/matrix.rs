pub fn shape(matrix: &[Vec<f64>]) -> Result<(usize, usize), String> {
    if matrix.is_empty() {
        return Ok((0, 0));
    }
    let num_cols = matrix[0].len();
    for row in matrix.iter().skip(1) {
        if row.len() != num_cols {
            return Err("all rows must have the same number of columns".to_string());
        }
    }
    Ok((matrix.len(), num_cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_normal() {
        let m = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        assert_eq!(shape(&m).unwrap(), (2, 3));
    }

    #[test]
    fn test_shape_square() {
        let m = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        assert_eq!(shape(&m).unwrap(), (2, 2));
    }

    #[test]
    fn test_shape_empty() {
        let m: Vec<Vec<f64>> = vec![];
        assert_eq!(shape(&m).unwrap(), (0, 0));
    }

    #[test]
    fn test_shape_column_mismatch() {
        let m = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0]];
        assert!(shape(&m).is_err());
    }
}
