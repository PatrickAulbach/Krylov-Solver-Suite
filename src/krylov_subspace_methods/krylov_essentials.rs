use crate::common::matrix::matrix::Matrix;
use std::path::Path;

pub trait Krylov<T> {
    fn new(path: &Path) -> Matrix<T>;
}