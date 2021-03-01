use super::data::Data;
use super::dimensions::Dimensions;

pub type Vector<Ncols, T> = Matrix<Ncols, Dimensions::VECTOR, T>;

pub struct Matrix<Ncols, Nrows, T> {
    data: Data<T>
}

impl Matrix<Ncols, Nrows, T> {
    pub fn new(data: Data<T>) -> Self {
        Matrix {
            data
        }
    }

    pub fn add(self, matrix: Matrix<Ncols, Nrows, T>) -> Matrix<Ncols, Nrows, T> {
        unimplemented!()
    }

    pub fn Ncols(&self) -> usize {
        self.data.Ncols()
    }

    pub fn Nrows(&self) -> usize {
        self.data.Nrows()
    }
}