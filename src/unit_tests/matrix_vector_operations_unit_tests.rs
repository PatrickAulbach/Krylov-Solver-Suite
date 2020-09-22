//#[path = "../matrix_vector_operations/matrix_vector_operations.rs"] mod matrix_vector_operations;
#[cfg(test)]

mod tests {
    use crate::matrix_vector_operations::matrix_vector_multiplication;

    #[test]
    fn test_matrix_vector_multiplication_with_3_times_3_matrix() {
        let x = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]];
        let y = vec![1.0, 1.0, 1.0];
        let mut solution: Vec<f64> = Vec::new();

        matrix_vector_multiplication(&x, &y, &mut solution);

        assert_eq!(solution[0], 1.0);
        assert_eq!(solution[1], 1.0);
        assert_eq!(solution[2], 1.0);
    }
}