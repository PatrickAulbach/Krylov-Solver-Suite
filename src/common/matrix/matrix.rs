use num::Num;
use std::ops::{Mul, Add};
use crate::common::matrix::dimensions::DimensionError;

pub type Vector<T> = Matrix<T>;

#[derive(Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T: Num> Matrix<T> where T: Mul<Output = T> + Add<Output = T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Matrix {
             data
        }
    }

    pub fn check_consistency(&self) -> Result<(), DimensionError> {

        type Error = DimensionError;
        for column in self.data.iter() {
            if column.len() != self.nrows() {
                return Err(DimensionError::InvalidDimension);
            }
        }
        Ok(())
    }

    pub fn ncols(&self) -> usize {
        self.data.len()
    }

    pub fn nrows(&self) -> usize {
        self.data[0].len()
    }

    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

}
