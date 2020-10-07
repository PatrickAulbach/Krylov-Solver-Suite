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

    #[cfg(test)]
    mod tests {
        use super::*;

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
    }
}