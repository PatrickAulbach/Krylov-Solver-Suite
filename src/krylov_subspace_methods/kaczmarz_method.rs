use crate::krylov_subspace_methods::krylov_essentials::Krylov;
use std::path::Path;
use crate::common::matrix::matrix::Vector;

pub struct Kaczmarz;

impl<T> Krylov<T> for Kaczmarz {
    fn new(path: &Path) -> Vector<T> {
        unimplemented!()
    }
}