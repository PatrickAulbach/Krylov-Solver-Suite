mod matrix_vector_operations {
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::vector_operations::vector_operations;

    pub fn matrix_vector_multiplication(matrix: &Vec<Vec<f64>>, vector: &Vec<f64>, solution: &mut Vec<f64>) {
        let mut buffer: f64 = 0.0;

        if matrix[0].len() != vector.len() {
            panic!("Dimensions doesn't match. Dimensions are: [{}][{}], [{}]", matrix.len(), matrix[0].len(), vector.len());
        }

        for i in 0..vector.len() {
            for j in 0..matrix[0].len() {
                buffer += matrix[i][j] * vector[j];
            }
            solution.push(buffer);
            buffer = 0.0;
        }
    }

    fn compute_residuum(matrix: &Vec<Vec<f64>>, vector: &Vec<f64>, right_hand_side: &Vec<f64>) -> f64 {
        let mut solution: Vec<f64> = Vec::new();
        let mut negative_right_hand_side: Vec<f64> = Vec::new();

        for i in 0..right_hand_side.len() {
            negative_right_hand_side.push(-right_hand_side[i]);
        }

        matrix_vector_multiplication(matrix, vector, &mut solution);

        vector_operations::vector_addition(&mut solution, &negative_right_hand_side);

        return vector_operations::euclidean_norm(&solution);
    }

    fn read_matrix_from_file(path: &str) -> Vec<Vec<f64>> {
        let mut file = BufReader::new(File::open("matrix.txt").unwrap());

        let matrix: Vec<Vec<f64>> = file.lines()
            .map(|f| f.unwrap().split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect())
            .collect();

        return matrix;
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_matrix_vector_multiplication_with_3_times_3_matrix() {
            let x = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]];
            let y = vec![1.0, 1.0, 1.0];
            let mut solution: Vec<f64> = Vec::new();

            matrix_vector_multiplication(&x, &y, &mut solution);

            assert_eq!(solution, vec![1.0, 1.0, 1.0]);
        }

        #[test]
        #[should_panic]
        fn test_matrix_vector_multiplication_mismatching_vector_and_matrix_should_panic() {
            let x = vec![vec![1.0, 0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0, 0.0], vec![0.0, 0.0, 1.0, 0.0]];
            let y = vec![1.0, 1.0, 1.0];
            let mut solution: Vec<f64> = Vec::new();

            matrix_vector_multiplication(&x, &y, &mut solution);
        }
    }
}