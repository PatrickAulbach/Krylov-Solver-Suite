use super::data::Data;
use super::dimensions::Dimensions;

pub type Vector<Nrows, T> = Matrix<Nrows, Dimensions::VECTOR, T>;

pub struct Matrix<Nrows, Ncols, T> {
    data: Data<T>
}

impl Matrix<Nrows, Ncols, T> {
    pub fn new(data: Data<T>) -> Self {
        Matrix {
            data
        }
    }

    pub fn add(self, matrix: Matrix<Nrows, Ncols, T>) -> Matrix<Nrows, Ncols, T> {
        unimplemented!()
    }
}