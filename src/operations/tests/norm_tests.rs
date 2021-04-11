#[cfg(test)]
mod tests {
    use crate::common::matrix::matrix::Vector;
    use crate::operations::matrix_operations::MatrixOperations;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn euclidian_norm_should_be_correct() {
        let vector: Vector<f64> = Vector::new(
            vec![
                1.0, 2.0, 3.0
            ],
            1,
            3,
        );

        let norm = MatrixOperations::euclidean_norm(vector);

        assert_approx_eq!(3.74165738677, norm, 1e-3f64);
    }
}