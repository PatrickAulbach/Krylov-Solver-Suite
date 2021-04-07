use crate::krylov_subspace_methods::krylov_essentials::Krylov;
use std::path::Path;
use crate::common::matrix::matrix::{Matrix, Vector};

pub struct ArnoldiMethod;

impl<T> Krylov<T> for ArnoldiMethod {
    fn new_from_path(path: &Path) -> Matrix<T> {
        unimplemented!()
    }

    fn new_from_matrix(matrix: Matrix<T>, rhs: Vector<T>, eps: f64) -> Matrix<T> {
        unimplemented!()
    }
}