use super::data::Data;

pub type Vector<T> = Matrix<T>;

pub struct Matrix<T> {
    data: Data<T>,
}

impl<T> Matrix<T> {
    pub fn new(data: Data<T>) -> Self {
        Matrix {
             data
        }
    }

    //compute alpha * A + beta * B
    pub fn add(self, matrix: Matrix<T>, alpha: T, beta: T) -> Matrix<T> {
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