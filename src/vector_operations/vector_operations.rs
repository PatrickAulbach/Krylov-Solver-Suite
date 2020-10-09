pub(crate) mod vector_operations {
    pub fn vector_addition_subtraction(first_vector: &mut Vec<f64>, second_vector: &Vec<f64>, is_subtraction: bool) {
        if is_subtraction {
            for i in 0..first_vector.len() {
                first_vector[i] -= second_vector[i];
            }
        } else {
            for i in 0..first_vector.len() {
                first_vector[i] += second_vector[i];
            }
        }
    }

    pub fn euclidean_norm(vector: &Vec<f64>) -> f64 {
        let mut norm: f64 = 0.0;

        for i in 0..vector.len() {
            norm += vector[i] * vector[i];
        }

        return norm.sqrt();
    }

    pub fn scalar_vector_multiplication(scalar: f64, vector: &Vec<f64>) -> Vec<f64> {
        let mut vector_buff: Vec<f64> = Vec::new();

        for i in 0..vector.len() {
            vector_buff.push(scalar * vector[i]);
        }

        return vector_buff;
    }

    pub fn scalar_product(first_vector: &Vec<f64>, second_vector: &Vec<f64>) -> f64 {
        let mut sum_of_products: f64 = 0.0;

        for i in 0..first_vector.len() {
            sum_of_products += first_vector[i] * second_vector[i];
        }

        return sum_of_products;
    }

    pub fn gram_schmidt(base_vector_matrix: &mut Vec<Vec<f64>>, hessenberg_matrix: &mut Vec<Vec<f64>>, iteration_count: usize) {
        for i in 0..iteration_count {
            //hj,k-1 = qj*qk
            hessenberg_matrix[i][iteration_count - 1] = scalar_product(&base_vector_matrix[i], &base_vector_matrix[iteration_count]);
            //qk = qk - hj,k-1*qj
            let second_vector = scalar_vector_multiplication(hessenberg_matrix[i][iteration_count - 1], &mut base_vector_matrix[i]);

            vector_addition_subtraction(&mut base_vector_matrix[iteration_count],
                                        &second_vector,
                                        true);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use assert_approx_eq::assert_approx_eq;

        #[test]
        fn test_vector_addition_with_two_3_times_3_vectors() {
            let mut first_vec = vec![1.0, 1.0, 1.0];
            let second_vec = vec![-1.0, -1.0, -1.0];
            let solution = vec![0.0, 0.0, 0.0];

            vector_addition_subtraction(&mut first_vec, &second_vec, false);

            assert_eq!(first_vec, solution);
        }

        #[test]
        fn test_vector_subtraction_with_two_3_times_3_vectors() {
            let mut first_vec = vec![1.0, 1.0, 1.0];
            let second_vec = vec![1.0, 1.0, 1.0];
            let solution = vec![0.0, 0.0, 0.0];

            vector_addition_subtraction(&mut first_vec, &second_vec, true);

            assert_eq!(first_vec, solution);
        }

        #[test]
        fn test_projection_subtraction_with_2_times_2_vectors() {
            let norm: f64 = 10.0;
            let eps = 0.0001;

            let mut base_vector_matrix = vec![vec![3.0 / norm.sqrt(), 1.0 / norm.sqrt()], vec![2.0, 2.0]];
            let mut hessenberg_matrix = vec![vec![0.0; 2]; 2];

            gram_schmidt(&mut base_vector_matrix, &mut hessenberg_matrix, 1);

            assert_approx_eq!(base_vector_matrix[1][0], -0.4, eps);
            assert_approx_eq!(base_vector_matrix[1][1], 1.2, eps);
        }
    }
}