#[cfg(test)]
mod tests {
    use crate::common::matrix::matrix::{Matrix, Vector};
    use crate::operations::matrix_operations::MatrixOperations;

    #[test]
    fn test_matrix_matrix_multiplication_with_3x3_matrices() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            3
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0
            ],
            3,
            3
        );

        let multiplicated_matrix: Matrix<f64> = MatrixOperations::mul(first_matrix, second_matrix).unwrap();

        assert_eq!(multiplicated_matrix.data(), &vec![12.0, 15.0, 18.0, 12.0, 15.0, 18.0, 12.0, 15.0, 18.0]);
    }

    #[test]
    fn test_matrix_matrix_multiplication_with_non_symmetric_matrices() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            2
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 2.0,
                4.0, 5.0,
                7.0, 8.0
            ],
            2,
            3
        );

        let multiplicated_matrix: Matrix<f64> = MatrixOperations::mul(first_matrix, second_matrix).unwrap();

        assert_eq!(multiplicated_matrix.data(), &vec![12.0, 15.0, 12.0, 15.0]);
    }

    #[test]
    fn test_matrix_vector_multiplication() {
        let matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                0.0, 2.0, 5.0,
                2.0, 5.0, -1.0
            ],
            3,
            3
        );
        let vec: Vector<f64> = Vector::new(
            vec![
                2.0,
                2.0,
                2.0
            ],
            1,
            3
        );

        let matrix_vector: Matrix<f64> = MatrixOperations::mul(matrix, vec).unwrap();

        assert_eq!(matrix_vector.data(), &vec![6.0, 14.0, 12.0]);
    }
}