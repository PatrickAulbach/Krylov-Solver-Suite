use super::data::Data;
use num::Num;

pub type Vector<T> = Matrix<T>;

pub struct Matrix<T> {
    data: Data<T>,
}

impl<T: Num> Matrix<T> {
    pub fn new(data: Data<T>) -> Self {
        Matrix {
             data
        }
    }

    //compute alpha * A + beta * B
    pub fn add(&self, matrix: Matrix<T>, alpha: f64, beta: f64) -> Matrix<T>  {
        unimplemented!()
    }

    pub fn ncols(&self) -> usize {
        self.data.ncols()
    }

    pub fn nrows(&self) -> usize {
        self.data.nrows()
    }

    pub fn data(&self) -> &Data<T> {
        &self.data
    }

}