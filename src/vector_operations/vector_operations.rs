pub(crate) mod vector_operations {

    pub fn vector_addition(first_vector: &mut Vec<f64>, second_vector: &Vec<f64>) {
        for i in 0..first_vector.len() {
            first_vector[i] += second_vector[i];
        }
    }

    pub fn euclidean_norm(vector: &Vec<f64>) -> f64 {
        let mut norm: f64 = 0.0;

        for i in 0..vector.len() {
            norm += vector[i] * vector[i];
        }

        return norm.sqrt();
    }

    pub fn scalar_product(first_vector: &Vec<f64>, second_vector: &Vec<f64>) -> f64 {
        let mut sum_of_products: f64 = 0.0;

        for i in 0..first_vector.len() {
            sum_of_products += first_vector[i] * second_vector[i];
        }

        return sum_of_products;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_vector_addition_with_two_3_times_3_vectors() {
            let mut first_vec = vec![1.0, 1.0, 1.0];
            let second_vec = vec![-1.0, -1.0, -1.0];
            let solution = vec![0.0, 0.0, 0.0];

            vector_addition(&mut first_vec, &second_vec);

            assert_eq!(first_vec, solution);
        }
    }
}