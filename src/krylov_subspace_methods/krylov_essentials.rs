use crate::common::matrix::matrix::{Matrix, Vector};
use std::path::Path;

pub trait Krylov<T> {
    fn new_from_path(path: &Path) -> Matrix<T>;

    fn new_from_matrix(matrix: Matrix<T>, rhs: Vector<T>, eps: f64) -> Matrix<T>;
}