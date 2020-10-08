use std::env;
use std::path::Path;
use crate::vector_operations::vector_operations::{euclidean_norm, scalar_product, scalar_vector_multiplication};
use crate::matrix_vector_operations::matrix_vector_operations::matrix_vector_multiplication;
use crate::matrix::Matrix;


#[path = "vector_operations/vector_operations.rs"]
mod vector_operations;
#[path = "matrix_vector_operations/matrix_vector_operations.rs"]
mod matrix_vector_operations;
#[path = "matrix/matrix.rs"]
mod matrix;
#[path = "arnoldi_method.rs"]
mod arnoldi_method;

fn main() {
    arnoldi_method::arnoldi_method::arnoldi_method(Path::new("src/matrix_vector_operations/unit_test_matrix_3x3.txt"), &vec![1.0, 1.0, 1.0]);
}
