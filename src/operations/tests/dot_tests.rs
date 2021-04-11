#[cfg(test)]
mod tests {
    use crate::common::matrix::matrix::Vector;
    use crate::operations::matrix_operations::MatrixOperations;

    #[test]
    fn test_dot_product_with_vectors() {
        let a: Vector<f64> = Vector::new(
            vec![1f64, 2f64, 3f64, 4f64, 5f64],
            1,
            5
        );
        let b: Vector<f64> = Vector::new(
            vec![1f64, 1f64, 1f64, 1f64, 1f64],
            1,
            5
        );

        let dot: f64 = MatrixOperations::dot(a, b).unwrap();

        assert_eq!(dot, 15f64);
    }
}