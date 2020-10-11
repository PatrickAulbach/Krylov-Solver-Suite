pub(crate) mod arnoldi_method {
    use std::path::Path;
    use crate::matrix::Matrix;
    use crate::matrix_vector_operations::matrix_vector_operations;
    use crate::vector_operations::vector_operations;
    use std::borrow::Borrow;
    use crate::vector_operations::vector_operations::euclidean_norm;

    //for now right hand side is an argument. TODO: Matrix object should be a general matrix object
    pub fn arnoldi_method(path: &Path, right_hand_side: &Vec<f64>, krylov_subspace_dimension: usize) -> Vec<Vec<f64>> {
        let mut qk: Vec<f64> = Vec::new();
        let matrix_vector_struct: Matrix = matrix_vector_operations::read_matrix_from_file(path);

        let mut Q = vec![vec![0.0f64; matrix_vector_struct.get_row_len()]; krylov_subspace_dimension];

        let mut H = vec![vec![0.0f64; krylov_subspace_dimension]; krylov_subspace_dimension];

        //compute first vector
        Q[0] = vector_operations::scalar_vector_multiplication((1.0 / vector_operations::euclidean_norm(right_hand_side)), right_hand_side);

        for k in 1..krylov_subspace_dimension {
            matrix_vector_operations::matrix_vector_multiplication(&matrix_vector_struct, &Q[k - 1], &mut qk);

            Q[k] = qk.to_owned();

            vector_operations::gram_schmidt(&mut Q, &mut H, k);

            H[k][k - 1] = euclidean_norm(&Q[k]);
            Q[k] = vector_operations::scalar_vector_multiplication(1.0 / H[k][k - 1], &Q[k]);
        }
        return H;
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use assert_approx_eq::assert_approx_eq;

        #[test]
        fn test_arnoldi_method_with_3_times_3_matrix_should_return_correct_hessenberg_matrix() {
            let eps: f64 = 0.000001;

            let hessenberg_matrix = arnoldi_method(
                Path::new("src/matrix_vector_operations/unit_test_matrix_3x3.txt"),
                &vec![1.0, 1.0, 1.0], 3);

            assert_approx_eq!(hessenberg_matrix[0][0], 3.0, eps);
            assert_approx_eq!(hessenberg_matrix[0][1], -3.0, eps);
            assert_approx_eq!(hessenberg_matrix[0][2], 0.0, eps);

            assert_approx_eq!(hessenberg_matrix[1][0], 0.0, eps);
            assert_approx_eq!(hessenberg_matrix[1][1], 0.0, eps);
            assert_approx_eq!(hessenberg_matrix[1][2], 0.0, eps);

            assert_approx_eq!(hessenberg_matrix[2][0], 0.0, eps);
            assert_approx_eq!(hessenberg_matrix[2][1], 0.0, eps);
            assert_approx_eq!(hessenberg_matrix[2][2], 0.0, eps);
        }
    }
}