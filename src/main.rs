use std::path::Path;

#[path = "vector_operations/vector_operations.rs"]
mod vector_operations;
#[path = "matrix_vector_operations/matrix_vector_operations.rs"]
mod matrix_vector_operations;
#[path = "matrix/matrix.rs"]
mod matrix;
#[path = "arnoldi_method.rs"]
mod arnoldi_method;

fn main() {

    let q = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    arnoldi_method::arnoldi_method::arnoldi_method(Path::new("src/matrix_vector_operations/einheitsmatrix.txt"), &q, 10);
}
