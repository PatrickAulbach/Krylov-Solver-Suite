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