use super::data::Data;
use std::path::Path;
use crate::common::reader::file_reader::FileReader;

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

    pub fn new_from_file(&self, path: Path) -> Self {

        let data = FileReader::read_matrix_from_file(&path);

        Matrix {
            data
        }
    }

    //compute alpha * A + beta * B
    pub fn add(self, matrix: Matrix<T>, alpha: f64, beta: f64) -> Matrix<T> {
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