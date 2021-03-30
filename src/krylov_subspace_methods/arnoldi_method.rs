use crate::krylov_subspace_methods::krylov_essentials::Krylov;
use std::path::Path;
use crate::common::matrix::matrix::Matrix;

pub struct ArnoldiMethod;

impl<T> Krylov<T> for ArnoldiMethod {
    fn new(path: &Path) -> Matrix<T> {
        unimplemented!()
    }
}