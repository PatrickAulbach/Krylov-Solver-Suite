#[cfg(test)]
mod tests {
    use crate::common::matrix::matrix::Matrix;
    use crate::operations::matrix_operations::MatrixOperations;

    #[test]
    fn add_with_alpha_zero_should_return_b() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            3,
        );

        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            3,
        );

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 0f64, 1f64);

        assert_eq!(added_matrix.data(), &vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn add_with_alpha_zero_and_beta_greater_one_should_return_beta_times_b() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            3,
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0
            ],
            3,
            3,
        );

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 0f64, 2f64);

        assert_eq!(added_matrix.data(), &vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0]);
    }

    #[test]
    fn add_with_beta_negative_should_return_correct_solution() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            3,
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,
                1.0, 1.0, 1.0
            ],
            3,
            3,
        );

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 1f64, -1f64);

        assert_eq!(added_matrix.data(), &vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn add_vector_with_beta_negative_should_return_correct_solution() {
        let first_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0
            ],
            1,
            3,
        );
        let second_matrix: Matrix<f64> = Matrix::new(
            vec![
                1.0, 1.0, 1.0
            ],
            1,
            3,
        );

        let added_matrix: Matrix<f64> = MatrixOperations::add(first_matrix, second_matrix, 1f64, -1f64);

        assert_eq!(added_matrix.data(), &vec![0.0, 0.0, 0.0]);
    }
}