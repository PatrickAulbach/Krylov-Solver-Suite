#[path = "../vector_operations/vector_operations.rs"] mod vector_operations;

pub fn matrix_vector_multiplication(matrix: &Vec<Vec<f64>>, vector: &Vec<f64>, solution: &mut Vec<f64>) {

    let mut buffer: f64 = 0.0;

    for i in 0..vector.len() {
        for j in 0..matrix[0].len() {
            buffer += matrix[i][j] * vector[j];
        }
        solution.push(buffer);
        buffer = 0.0;
    }
}

pub fn compute_residuum(matrix: &Vec<Vec<f64>>, vector: &Vec<f64>, right_hand_side: &Vec<f64>) -> f64 {
    let mut solution: Vec<f64> = Vec::new();
    let mut negative_right_hand_side: Vec<f64> = Vec::new();

    for i in 0..right_hand_side.len() {
        negative_right_hand_side.push(-right_hand_side[i]);
    }

    matrix_vector_multiplication(matrix, vector, &mut solution);

    vector_operations::vector_addition(&mut solution, &negative_right_hand_side);

    return vector_operations::euclidean_norm(&solution);
}