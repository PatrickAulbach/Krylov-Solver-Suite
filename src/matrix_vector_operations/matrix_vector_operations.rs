pub (crate) mod matrix_vector_operations {
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::vector_operations::vector_operations;
    use std::path::Path;
    use crate::matrix;
    use crate::matrix::Matrix;

    pub fn matrix_vector_multiplication(matrix: &Matrix, vector: &Vec<f64>, solution: &mut Vec<f64>) {

        if matrix.get_row_len() != vector.len() {
            panic!("Dimensions doesn't match. Dimensions are: [{}][{}], [{}]", matrix.get_column_len(), matrix.get_row_len(), vector.len());
        }

        for i in 0..matrix.get_column_len() {
            solution.push(vector_operations::scalar_product(&matrix.get_matrix()[i], vector));
        }
    }

    fn compute_residuum(matrix: &Matrix, vector: &Vec<f64>, right_hand_side: &Vec<f64>) -> f64 {
        let mut solution: Vec<f64> = Vec::new();

        matrix_vector_multiplication(&matrix, vector, &mut solution);

        vector_operations::vector_addition_subtraction(&mut solution, right_hand_side, true);

        return vector_operations::euclidean_norm(&solution);
    }

    fn read_matrix_from_file(path: &Path) -> Matrix {
        let mut file = BufReader::new(File::open(path).unwrap());

        let mut columns = String::new();
        let mut rows = String::new();
        file.read_line(&mut columns).unwrap();
        file.read_line(&mut rows).unwrap();

        let data = file.lines()
            .map(|f| f.unwrap().split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect())
            .collect();

        return Matrix::new(columns.trim().parse().unwrap(), rows.trim().parse().unwrap(), data);
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

            for i in 0..matrix.get_row_len() {
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