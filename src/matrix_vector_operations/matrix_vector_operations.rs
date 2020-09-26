mod matrix_vector_operations {
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::vector_operations::vector_operations;
    use std::path::Path;
    use crate::matrix;
    use crate::matrix::Matrix;

    pub fn matrix_vector_multiplication(matrix: &Matrix, vector: &Vec<f64>, solution: &mut Vec<f64>) {
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

    fn compute_residuum(matrix: Matrix, vector: &Vec<f64>, right_hand_side: &Vec<f64>) -> f64 {
        let mut solution: Vec<f64> = Vec::new();
        let mut negative_right_hand_side: Vec<f64> = Vec::new();

        for i in 0..right_hand_side.len() {
            negative_right_hand_side.push(-right_hand_side[i]);
        }

        matrix_vector_multiplication(&matrix, vector, &mut solution);

        vector_operations::vector_addition(&mut solution, &negative_right_hand_side);

        return vector_operations::euclidean_norm(&solution);
    }

    fn read_matrix_from_file(path: &Path) -> matrix::Matrix {
        let mut file = BufReader::new(File::open(path).unwrap());

        let mut s = String::new();
        let mut s = String::new();
        f.read_line(&mut s).unwrap();
        f.read_line(&mut s).unwrap();
        let mut matrix = Matrix::new(3, 3);

        matrix = file.lines()
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
            let matrix = read_matrix_from_file(Path::new("src/matrix_vector_operations/unit_test_matrix_3x3.txt"));
            let x = vec![1.0, 1.0, 1.0];
            let mut solution: Vec<f64> = Vec::new();

            matrix_vector_multiplication(&matrix, &x, &mut solution);

            assert_eq!(solution, vec![3.0, 3.0, 3.0]);
        }

        #[test]
        fn test_matrix_vector_multiplication_with_50_times_50_matrix() {
            let matrix = read_matrix_from_file(Path::new("src/matrix_vector_operations/unit_test_matrix_50x50.txt"));
            let mut x: Vec<f64> = Vec::new();
            let mut solution: Vec<f64> = Vec::new();
            let mut vector_to_compare: Vec<f64> = Vec::new();

            for i in 0..matrix[0].len() {
                x.push(1.0);
                vector_to_compare.push(50.0);
            }

            matrix_vector_multiplication(&matrix, &x, &mut solution);

            assert_eq!(solution, vector_to_compare);
        }

        #[test]
        #[should_panic]
        fn test_matrix_vector_multiplication_mismatching_vector_and_matrix_should_panic() {
            let matrix = read_matrix_from_file(Path::new("src/matrix_vector_operations/unit_test_matrix_3x3.txt"));
            let x = vec![1.0, 1.0, 1.0, 1.0];
            let mut solution: Vec<f64> = Vec::new();

            matrix_vector_multiplication(&matrix, &x, &mut solution);
        }
    }
}